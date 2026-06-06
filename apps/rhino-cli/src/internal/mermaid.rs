//! Byte-for-byte port of `apps/rhino-cli/internal/mermaid/`.
//!
//! Combined into a single file: types, extractor, parser, graph utilities,
//! validator, and reporter for Mermaid flowchart diagrams embedded in Markdown.
//!
//! Primary entry points: [`extract_blocks`], [`validate_blocks`],
//! [`format_text`], [`format_json`], [`format_markdown`].

use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;
use std::sync::OnceLock;

use anyhow::Error;
use regex::Regex;
use serde::Serialize;

// ── Types ─────────────────────────────────────────────────────────────────

/// Flow direction of a Mermaid flowchart diagram.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// Top-to-bottom (default).
    TB,
    /// Top-down (alias for [`Direction::TB`]).
    TD,
    /// Bottom-to-top.
    BT,
    /// Left-to-right.
    LR,
    /// Right-to-left.
    RL,
}

impl Direction {
    /// Parses a direction string from a Mermaid `flowchart` / `graph` header.
    ///
    /// Unknown strings default to [`Direction::TB`].
    pub fn parse(s: &str) -> Self {
        match s {
            "TD" => Direction::TD,
            "BT" => Direction::BT,
            "LR" => Direction::LR,
            "RL" => Direction::RL,
            _ => Direction::TB,
        }
    }
}

/// Category of a validation violation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViolationKind {
    /// A node label exceeds the configured maximum character count.
    LabelTooLong,
    /// The diagram width (nodes in the widest rank) exceeds the configured maximum.
    WidthExceeded,
    /// A single code block contains more than one `flowchart` / `graph` header.
    MultipleDiagrams,
}

impl ViolationKind {
    /// Returns the stable string code for this kind
    /// (`"label_too_long"`, `"width_exceeded"`, or `"multiple_diagrams"`).
    pub fn code(&self) -> &'static str {
        match self {
            ViolationKind::LabelTooLong => "label_too_long",
            ViolationKind::WidthExceeded => "width_exceeded",
            ViolationKind::MultipleDiagrams => "multiple_diagrams",
        }
    }
}

/// Category of a validation warning (non-blocking advisory).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WarningKind {
    /// Both the width and depth limits are exceeded simultaneously.
    ComplexDiagram,
    /// A subgraph contains more children than the configured maximum.
    SubgraphDense,
}

impl WarningKind {
    /// Returns the stable string code for this kind
    /// (`"complex_diagram"` or `"subgraph_density"`).
    pub fn code(&self) -> &'static str {
        match self {
            WarningKind::ComplexDiagram => "complex_diagram",
            WarningKind::SubgraphDense => "subgraph_density",
        }
    }
}

/// A raw Mermaid code block extracted from a Markdown file.
#[derive(Debug, Clone)]
pub struct MermaidBlock {
    /// Path to the Markdown file containing this block.
    pub file_path: String,
    /// Zero-based index of this block within the file.
    pub block_index: usize,
    /// Raw source of the block (content between the fence markers).
    pub source: String,
    /// 1-based line number of the first line inside the fence.
    pub start_line: usize,
}

/// A node in a parsed Mermaid flowchart.
#[derive(Debug, Clone)]
pub struct Node {
    /// Node identifier as it appears in the source.
    pub id: String,
    /// Display label (may be empty when the node has no explicit label).
    pub label: String,
}

/// A directed edge between two nodes.
#[derive(Debug, Clone)]
pub struct Edge {
    /// Source node identifier.
    pub from: String,
    /// Target node identifier.
    pub to: String,
}

/// A `subgraph` block parsed from a flowchart.
#[derive(Debug, Clone)]
pub struct Subgraph {
    /// Subgraph identifier (may be empty when unnamed).
    pub id: String,
    /// Display label from the `subgraph … [Label]` syntax.
    pub label: String,
    /// Identifiers of nodes that appear inside this subgraph.
    pub node_ids: Vec<String>,
    /// 1-based line number of the `subgraph` keyword within the block.
    pub start_line: usize,
}

/// A fully parsed Mermaid diagram with its structural metadata.
pub struct ParsedDiagram {
    /// The source block this diagram was parsed from.
    pub block: MermaidBlock,
    /// Declared flow direction.
    pub direction: Direction,
    /// All nodes in source order.
    pub nodes: Vec<Node>,
    /// All directed edges.
    pub edges: Vec<Edge>,
    /// All subgraph blocks.
    pub subgraphs: Vec<Subgraph>,
}

/// A single validation violation that blocks the check.
#[derive(Debug, Clone)]
pub struct Violation {
    /// Category of the violation.
    pub kind: ViolationKind,
    /// File path where the violation occurred.
    pub file_path: String,
    /// Zero-based index of the block within the file.
    pub block_index: usize,
    /// 1-based line number of the block's first line.
    pub start_line: usize,
    /// Node identifier (set for `LabelTooLong`; empty otherwise).
    pub node_id: String,
    /// Raw label text (set for `LabelTooLong`; empty otherwise).
    pub label_text: String,
    /// Effective character count of the label.
    pub label_len: usize,
    /// Configured maximum label length.
    pub max_label_len: usize,
    /// Computed diagram width (set for `WidthExceeded`; zero otherwise).
    pub actual_width: usize,
    /// Configured maximum width (set for `WidthExceeded`; zero otherwise).
    pub max_width: usize,
}

/// A non-blocking advisory about a diagram's complexity.
#[derive(Debug, Clone)]
pub struct Warning {
    /// Category of the warning.
    pub kind: WarningKind,
    /// File path where the warning occurred.
    pub file_path: String,
    /// Zero-based index of the block within the file.
    pub block_index: usize,
    /// 1-based line number of the block (or subgraph) start.
    pub start_line: usize,
    /// Computed diagram width.
    pub actual_width: usize,
    /// Computed diagram depth.
    pub actual_depth: usize,
    /// Configured maximum width.
    pub max_width: usize,
    /// Configured maximum depth.
    pub max_depth: usize,
    /// Label of the dense subgraph (set for `SubgraphDense`; empty otherwise).
    pub subgraph_label: String,
    /// Number of direct children in the dense subgraph.
    pub subgraph_node_count: usize,
    /// Configured maximum subgraph child count.
    pub max_subgraph_nodes: usize,
}

/// Aggregated result of a [`validate_blocks`] call.
pub struct ValidationResult {
    /// Number of unique files that contained at least one Mermaid block.
    pub files_scanned: usize,
    /// Total number of Mermaid blocks processed.
    pub blocks_scanned: usize,
    /// All violations found across all blocks.
    pub violations: Vec<Violation>,
    /// All non-blocking warnings found across all blocks.
    pub warnings: Vec<Warning>,
}

/// Tunable thresholds for Mermaid diagram validation.
#[derive(Debug, Clone, Copy)]
pub struct ValidateOptions {
    /// Maximum allowed character count for a single node label line.
    pub max_label_len: usize,
    /// Maximum allowed diagram width (nodes in the widest rank).
    pub max_width: usize,
    /// Maximum allowed diagram depth (number of distinct ranks).
    pub max_depth: usize,
    /// Maximum allowed number of direct children in any subgraph.
    pub max_subgraph_nodes: usize,
}

// ── Extractor ────────────────────────────────────────────────────────────

/// Extracts all ` ```mermaid ` / `~~~mermaid` code blocks from `content`.
///
/// Returns one [`MermaidBlock`] per fenced block, in document order.
/// Unclosed blocks at the end of the file are silently ignored.
pub fn extract_blocks(file_path: &str, content: &str) -> Vec<MermaidBlock> {
    let mut blocks = Vec::new();
    let mut in_block = false;
    let mut source_lines: Vec<String> = Vec::new();
    let mut block_index = 0;
    let mut start_line = 0;
    for (i, line) in content.split('\n').enumerate() {
        let trimmed = line.trim();
        if !in_block {
            if line.starts_with("```mermaid") || line.starts_with("~~~mermaid") {
                in_block = true;
                source_lines.clear();
                start_line = i + 1;
            }
        } else if trimmed == "```" || trimmed == "~~~" {
            blocks.push(MermaidBlock {
                file_path: file_path.to_string(),
                block_index,
                source: source_lines.join("\n"),
                start_line,
            });
            block_index += 1;
            in_block = false;
        } else {
            source_lines.push(line.to_string());
        }
    }
    blocks
}

// ── Parser ───────────────────────────────────────────────────────────────

/// Returns the compiled regex that matches a `flowchart` or `graph` header line.
fn flowchart_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(?m)^\s*(flowchart|graph)(\s+(TB|TD|BT|LR|RL))?\s*$")
            .expect("valid hardcoded regex")
    })
}

/// Returns the compiled regex that matches a `subgraph` header line.
fn subgraph_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"^subgraph(?:\s+([^\s\["]+))?(?:\s*\[\s*"?([^"\]]*)"?\s*\])?\s*$"#)
            .expect("valid hardcoded regex")
    })
}

/// Returns the compiled regex that matches Mermaid arrow / edge connectors.
fn arrow_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"-->|---|-\.->|==>|--o|--x|<-->").expect("valid hardcoded regex"))
}

/// Returns the compiled regex that matches edge labels (`-- text -->`).
fn link_text_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"--[^->\n]+?-->").expect("valid hardcoded regex"))
}

/// Returns the compiled regex that matches a pipe-delimited edge label
/// immediately following an arrow (`-->|text|`).
fn pipe_label_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(-->|---|-\.->|==>|--o|--x|<-->)\s*\|[^|\n]*\|")
            .expect("valid hardcoded regex")
    })
}

/// Returns the compiled regex that matches a bare node identifier (word characters only).
fn node_id_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^(\w+)$").expect("valid hardcoded regex"))
}

/// Returns compiled regexes for all Mermaid node shape syntaxes, in match-priority order.
///
/// Each regex captures `(id, label)` in groups 1 and 2.
fn node_shape_patterns() -> &'static Vec<Regex> {
    static PATTERNS: OnceLock<Vec<Regex>> = OnceLock::new();
    PATTERNS.get_or_init(|| {
        vec![
            Regex::new(r"^(\w+)\(\(\(([^)]*)\)\)\)").expect("valid hardcoded regex"),
            Regex::new(r"^(\w+)\(\[([^\]]*)\]\)").expect("valid hardcoded regex"),
            Regex::new(r"^(\w+)\(\(([^)]*)\)\)").expect("valid hardcoded regex"),
            Regex::new(r"^(\w+)\[\[([^\]]*)\]\]").expect("valid hardcoded regex"),
            Regex::new(r"^(\w+)\[\(([^)]*)\)\]").expect("valid hardcoded regex"),
            Regex::new(r"^(\w+)\(([^)]*)\)").expect("valid hardcoded regex"),
            Regex::new(r"^(\w+)\{\{([^}]*)\}\}").expect("valid hardcoded regex"),
            Regex::new(r"^(\w+)\{([^}]*)\}").expect("valid hardcoded regex"),
            Regex::new(r"^(\w+)>([^\]]*)\]").expect("valid hardcoded regex"),
            Regex::new(r"^(\w+)\[/([^/]*)/\]").expect("valid hardcoded regex"),
            Regex::new(r"^(\w+)\[\\([^\\]*)\\]").expect("valid hardcoded regex"),
            Regex::new(r"^(\w+)\[([^\]]*)\]").expect("valid hardcoded regex"),
            Regex::new(r#"^(\w+)@\{\s*[^}]*label:\s*"([^"]*)"\s*[^}]*\}"#)
                .expect("valid hardcoded regex"),
        ]
    })
}

/// Parses a [`MermaidBlock`] into a [`ParsedDiagram`] and the number of
/// `flowchart` / `graph` headers found in the block.
///
/// A count of `0` means the block is not a flowchart (e.g. a sequence diagram).
/// A count `> 1` indicates multiple diagrams packed into one block, which is a violation.
#[allow(clippy::collapsible_if)]
pub fn parse_diagram(block: MermaidBlock) -> (ParsedDiagram, usize) {
    let matches: Vec<_> = flowchart_re().captures_iter(&block.source).collect();
    let count = matches.len();
    if count == 0 {
        return (
            ParsedDiagram {
                block,
                direction: Direction::TB,
                nodes: Vec::new(),
                edges: Vec::new(),
                subgraphs: Vec::new(),
            },
            0,
        );
    }
    let first = &matches[0];
    let dir = match first.get(3) {
        Some(m) if !m.as_str().trim().is_empty() => Direction::parse(m.as_str().trim()),
        _ => Direction::TB,
    };
    let mut node_map: Vec<(String, String)> = Vec::new();
    let mut node_index: HashMap<String, usize> = HashMap::new();
    let mut edges: Vec<Edge> = Vec::new();
    let mut subgraphs: Vec<Subgraph> = Vec::new();
    let mut stack: Vec<Subgraph> = Vec::new();
    let lines: Vec<&str> = block.source.split('\n').collect();
    for (line_idx, raw) in lines.iter().enumerate() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with("subgraph") {
            let (id, label) = parse_subgraph_header(line);
            stack.push(Subgraph {
                id,
                label,
                node_ids: Vec::new(),
                start_line: line_idx + 1,
            });
            continue;
        }
        if line == "end" {
            if let Some(s) = stack.pop() {
                subgraphs.push(s);
            }
            continue;
        }
        if flowchart_re().is_match(line) {
            continue;
        }
        let before: HashSet<String> = node_index.keys().cloned().collect();
        if arrow_re().is_match(line) {
            extract_edge_line(line, &mut node_map, &mut node_index, &mut edges);
        } else {
            extract_standalone_node(line, &mut node_map, &mut node_index);
        }
        let new_ids: Vec<String> = node_index
            .keys()
            .filter(|k| !before.contains(*k))
            .cloned()
            .collect();
        if !new_ids.is_empty() {
            if let Some(top) = stack.last_mut() {
                for id in dedup_order(&new_ids) {
                    if !top.node_ids.contains(&id) {
                        top.node_ids.push(id);
                    }
                }
            }
        }
    }
    while let Some(s) = stack.pop() {
        subgraphs.push(s);
    }
    let seen_order = collect_node_order(&block.source, &node_index);
    let nodes: Vec<Node> = seen_order
        .into_iter()
        .map(|id| {
            let label = node_map
                .iter()
                .find(|(k, _)| *k == id)
                .map(|(_, v)| v.clone())
                .unwrap_or_default();
            Node { id, label }
        })
        .collect();
    (
        ParsedDiagram {
            block,
            direction: dir,
            nodes,
            edges,
            subgraphs,
        },
        count,
    )
}

/// Extracts `(id, label)` from a `subgraph` header line.
///
/// Falls back to an empty id and the trimmed remainder as label when the regex
/// does not match.
fn parse_subgraph_header(line: &str) -> (String, String) {
    if let Some(m) = subgraph_re().captures(line) {
        let id = m.get(1).map(|s| s.as_str().to_string()).unwrap_or_default();
        let label = m.get(2).map(|s| s.as_str().to_string()).unwrap_or_default();
        return (id, label);
    }
    let rest = line.trim_start_matches("subgraph").trim();
    let rest = rest.trim_matches('"');
    (String::new(), rest.to_string())
}

/// Returns `ids` with duplicates removed, preserving first-occurrence order.
fn dedup_order(ids: &[String]) -> Vec<String> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut out = Vec::new();
    for id in ids {
        if seen.insert(id.clone()) {
            out.push(id.clone());
        }
    }
    out
}

/// Collects node identifiers from `source` in the order they first appear,
/// filtered to only those present in `node_map`.
fn collect_node_order(source: &str, node_map: &HashMap<String, usize>) -> Vec<String> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut order = Vec::new();
    for raw in source.split('\n') {
        let line = raw.trim();
        if line.is_empty() || line.starts_with("subgraph") || line == "end" {
            continue;
        }
        if flowchart_re().is_match(line) {
            continue;
        }
        for id in extract_all_node_ids(line) {
            if node_map.contains_key(&id) && seen.insert(id.clone()) {
                order.push(id);
            }
        }
    }
    for k in node_map.keys() {
        if seen.insert(k.clone()) {
            order.push(k.clone());
        }
    }
    order
}

/// Extracts all node identifiers mentioned on `line`, handling both edge lines
/// (splitting on arrows) and standalone node lines.
fn extract_all_node_ids(line: &str) -> Vec<String> {
    let mut ids = Vec::new();
    if arrow_re().is_match(line) {
        for seg in arrow_re().split(line) {
            ids.extend(extract_node_ids_from_segment(seg));
        }
    } else {
        ids.extend(extract_node_ids_from_segment(line));
    }
    ids
}

/// Extracts node identifiers from a segment that may contain `&`-separated groups.
fn extract_node_ids_from_segment(seg: &str) -> Vec<String> {
    seg.split('&')
        .filter_map(|sub| {
            let id = extract_node_id_from_segment(sub);
            if id.is_empty() { None } else { Some(id) }
        })
        .collect()
}

/// Extracts the node identifier from a single (non-`&`) segment.
///
/// Returns an empty string when no known shape pattern or bare identifier is recognised.
fn extract_node_id_from_segment(seg: &str) -> String {
    let seg = seg.trim();
    if seg.is_empty() {
        return String::new();
    }
    for re in node_shape_patterns() {
        if let Some(m) = re.captures(seg) {
            return m[1].to_string();
        }
    }
    if let Some(m) = node_id_re().captures(seg) {
        return m[1].to_string();
    }
    String::new()
}

/// Parses a standalone node declaration line (no arrow) and upserts it into `node_map`.
fn extract_standalone_node(
    line: &str,
    node_map: &mut Vec<(String, String)>,
    node_index: &mut HashMap<String, usize>,
) {
    let line = line.trim();
    for re in node_shape_patterns() {
        if let Some(m) = re.captures(line) {
            upsert_node(node_map, node_index, &m[1], normalize_label(&m[2]));
            return;
        }
    }
    if let Some(m) = node_id_re().captures(line) {
        let not_seen = !node_index.contains_key(&m[1]);
        if not_seen {
            upsert_node(node_map, node_index, &m[1], String::new());
        }
    }
}

/// Inserts a new node or updates an existing node's label in `node_map`.
///
/// `node_index` maps identifiers to their position in `node_map`.
fn upsert_node(
    node_map: &mut Vec<(String, String)>,
    node_index: &mut HashMap<String, usize>,
    id: &str,
    label: String,
) {
    if let Some(&idx) = node_index.get(id) {
        node_map[idx].1 = label;
    } else {
        node_index.insert(id.to_string(), node_map.len());
        node_map.push((id.to_string(), label));
    }
}

/// Parses an edge line (containing at least one arrow), upserts all referenced
/// nodes, and appends cartesian-product edges for each `&`-group pair.
fn extract_edge_line(
    line: &str,
    node_map: &mut Vec<(String, String)>,
    node_index: &mut HashMap<String, usize>,
    edges: &mut Vec<Edge>,
) {
    let line = link_text_re().replace_all(line, "-->");
    let line = pipe_label_re().replace_all(&line, "$1");
    let parts: Vec<&str> = arrow_re().split(&line).collect();
    if parts.len() < 2 {
        return;
    }
    let groups: Vec<Vec<String>> = parts
        .iter()
        .filter_map(|p| {
            let ids = extract_node_group(p, node_map, node_index);
            if ids.is_empty() { None } else { Some(ids) }
        })
        .collect();
    for i in 0..groups.len().saturating_sub(1) {
        for from in &groups[i] {
            for to in &groups[i + 1] {
                edges.push(Edge {
                    from: from.clone(),
                    to: to.clone(),
                });
            }
        }
    }
}

/// Parses one arrow-separated segment (`part`) which may contain `&`-separated
/// node references, upserts each node, and returns the list of identifiers.
fn extract_node_group(
    part: &str,
    node_map: &mut Vec<(String, String)>,
    node_index: &mut HashMap<String, usize>,
) -> Vec<String> {
    part.split('&')
        .filter_map(|seg| {
            let seg = seg.trim();
            if seg.is_empty() {
                return None;
            }
            let id = extract_node_id_and_label(seg, node_map, node_index);
            if id.is_empty() { None } else { Some(id) }
        })
        .collect()
}

/// Extracts a node identifier (and optional label) from `seg`, upserts it, and
/// returns the identifier string.  Returns an empty string when unrecognised.
fn extract_node_id_and_label(
    seg: &str,
    node_map: &mut Vec<(String, String)>,
    node_index: &mut HashMap<String, usize>,
) -> String {
    for re in node_shape_patterns() {
        if let Some(m) = re.captures(seg) {
            upsert_node(node_map, node_index, &m[1], normalize_label(&m[2]));
            return m[1].to_string();
        }
    }
    if let Some(m) = node_id_re().captures(seg) {
        if !node_index.contains_key(&m[1]) {
            upsert_node(node_map, node_index, &m[1], String::new());
        }
        return m[1].to_string();
    }
    String::new()
}

/// Strips surrounding quote characters (`"`, `'`, or `` ` ``) from a label string.
fn normalize_label(s: &str) -> String {
    let s = s.trim();
    if s.len() >= 2 {
        let bytes = s.as_bytes();
        let first = bytes[0];
        let last = bytes[s.len() - 1];
        if (first == b'"' && last == b'"')
            || (first == b'\'' && last == b'\'')
            || (first == b'`' && last == b'`')
        {
            return s[1..s.len() - 1].to_string();
        }
    }
    s.to_string()
}

/// Returns the effective display length of `label` after normalising line-break
/// tokens (`<br/>`, `<BR/>`, `<br>`, `<BR>`, `\n`) to actual newlines.
///
/// The length is the maximum character count across all resulting lines.
pub fn effective_label_len(label: &str) -> usize {
    if label.is_empty() {
        return 0;
    }
    let normalized = label
        .replace("<br/>", "\n")
        .replace("<BR/>", "\n")
        .replace("<br>", "\n")
        .replace("<BR>", "\n")
        .replace("\\n", "\n");
    normalized
        .split('\n')
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0)
}

// ── Graph ────────────────────────────────────────────────────────────────

/// Assigns a rank (depth level) to each node using a topological-sort-based
/// longest-path algorithm.
///
/// Cycles are handled by first removing back edges (detected via an iterative
/// DFS in node-declaration order), then ranking the remaining DAG — mirroring
/// how Mermaid itself lays out cyclic flowcharts. Disconnected nodes are
/// assigned rank `0`. Returns an empty map when `nodes` is empty.
fn rank_assign(nodes: &[Node], edges: &[Edge]) -> HashMap<String, i64> {
    if nodes.is_empty() {
        return HashMap::new();
    }
    let node_set: HashSet<&str> = nodes.iter().map(|n| n.id.as_str()).collect();
    let mut adj: HashMap<String, Vec<String>> = HashMap::new();
    for n in nodes {
        adj.insert(n.id.clone(), Vec::new());
    }
    for e in edges {
        if node_set.contains(e.from.as_str()) && node_set.contains(e.to.as_str()) {
            adj.entry(e.from.clone()).or_default().push(e.to.clone());
        }
    }

    // Pass 1: detect back edges via iterative DFS (gray = on stack, black = done),
    // visiting unvisited nodes in declaration order so the result is deterministic.
    let mut color: HashMap<String, u8> = HashMap::new(); // 0/absent=white, 1=gray, 2=black
    let mut back_edges: HashSet<(String, String)> = HashSet::new();
    for start in nodes {
        if color.get(&start.id).copied().unwrap_or(0) != 0 {
            continue;
        }
        // Stack of (node, next-neighbor-index).
        let mut stack: Vec<(String, usize)> = vec![(start.id.clone(), 0)];
        color.insert(start.id.clone(), 1);
        while let Some((cur, idx)) = stack.pop() {
            let neighbors = adj.get(&cur).cloned().unwrap_or_default();
            if idx < neighbors.len() {
                let next = neighbors[idx].clone();
                stack.push((cur.clone(), idx + 1));
                match color.get(&next).copied().unwrap_or(0) {
                    1 => {
                        back_edges.insert((cur, next));
                    }
                    0 => {
                        color.insert(next.clone(), 1);
                        stack.push((next, 0));
                    }
                    _ => {}
                }
            } else {
                color.insert(cur, 2);
            }
        }
    }

    // Pass 2: Kahn's longest-path ranking on the DAG that remains after
    // dropping the back edges.
    let mut in_degree: HashMap<String, usize> = HashMap::new();
    for n in nodes {
        in_degree.insert(n.id.clone(), 0);
    }
    for (from, tos) in &adj {
        for to in tos {
            if !back_edges.contains(&(from.clone(), to.clone())) {
                *in_degree.entry(to.clone()).or_insert(0) += 1;
            }
        }
    }
    let mut rank: HashMap<String, i64> = HashMap::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue: Vec<String> = Vec::new();
    for n in nodes {
        if in_degree.get(&n.id).copied().unwrap_or(0) == 0 {
            queue.push(n.id.clone());
            rank.insert(n.id.clone(), 0);
        }
    }
    while !queue.is_empty() {
        let cur = queue.remove(0);
        visited.insert(cur.clone());
        let cur_rank = *rank.get(&cur).unwrap_or(&0);
        let neighbors = adj.get(&cur).cloned().unwrap_or_default();
        for next in neighbors {
            if back_edges.contains(&(cur.clone(), next.clone())) {
                continue;
            }
            let existing = *rank.get(&next).unwrap_or(&0);
            if cur_rank + 1 > existing {
                rank.insert(next.clone(), cur_rank + 1);
            }
            if let Some(d) = in_degree.get_mut(&next) {
                *d = d.saturating_sub(1);
                if *d == 0 {
                    queue.push(next);
                }
            }
        }
    }
    for n in nodes {
        if !visited.contains(&n.id) {
            rank.entry(n.id.clone()).or_insert(0);
        }
    }
    rank
}

/// Returns the maximum number of nodes sharing the same rank (diagram width).
///
/// Returns `0` when there are no nodes.
pub fn max_width(nodes: &[Node], edges: &[Edge]) -> usize {
    if nodes.is_empty() {
        return 0;
    }
    let ranks = rank_assign(nodes, edges);
    let mut rank_count: HashMap<i64, usize> = HashMap::new();
    for r in ranks.values() {
        *rank_count.entry(*r).or_insert(0) += 1;
    }
    rank_count.values().copied().max().unwrap_or(0)
}

/// Returns the number of distinct rank levels in the diagram (diagram depth).
///
/// Returns `0` when there are no nodes.
pub fn depth(nodes: &[Node], edges: &[Edge]) -> usize {
    if nodes.is_empty() {
        return 0;
    }
    let ranks = rank_assign(nodes, edges);
    ranks.values().collect::<HashSet<_>>().len()
}

// ── Validator ────────────────────────────────────────────────────────────

/// Returns the default validation options used by the CLI when no flags are specified.
///
/// Defaults: `max_label_len = 30`, `max_width = 4`,
/// `max_depth = usize::MAX`, `max_subgraph_nodes = 6`.
pub fn default_validate_options() -> ValidateOptions {
    ValidateOptions {
        max_label_len: 30,
        max_width: 4,
        max_depth: usize::MAX,
        max_subgraph_nodes: 6,
    }
}

/// Validates all `blocks` against `opts` and returns an aggregated
/// [`ValidationResult`].
///
/// Blocks from the same file are counted once in `files_scanned`.
pub fn validate_blocks(blocks: Vec<MermaidBlock>, opts: ValidateOptions) -> ValidationResult {
    let mut files_seen: HashSet<String> = HashSet::new();
    let mut violations = Vec::new();
    let mut warnings = Vec::new();
    let total = blocks.len();
    for block in blocks {
        files_seen.insert(block.file_path.clone());
        validate_one_block(block, &opts, &mut violations, &mut warnings);
    }
    ValidationResult {
        files_scanned: files_seen.len(),
        blocks_scanned: total,
        violations,
        warnings,
    }
}

/// Validates a single [`MermaidBlock`] and appends any findings to the
/// `violations` and `warnings` vectors.
fn validate_one_block(
    block: MermaidBlock,
    opts: &ValidateOptions,
    violations: &mut Vec<Violation>,
    warnings: &mut Vec<Warning>,
) {
    let fp = block.file_path.clone();
    let bi = block.block_index;
    let sl = block.start_line;
    let (diagram, count) = parse_diagram(block);
    if count > 1 {
        violations.push(Violation {
            kind: ViolationKind::MultipleDiagrams,
            file_path: fp.clone(),
            block_index: bi,
            start_line: sl,
            node_id: String::new(),
            label_text: String::new(),
            label_len: 0,
            max_label_len: 0,
            actual_width: 0,
            max_width: 0,
        });
    }
    if count == 0 {
        return;
    }
    for node in &diagram.nodes {
        let label_len = effective_label_len(&node.label);
        if label_len > opts.max_label_len {
            violations.push(Violation {
                kind: ViolationKind::LabelTooLong,
                file_path: fp.clone(),
                block_index: bi,
                start_line: sl,
                node_id: node.id.clone(),
                label_text: node.label.clone(),
                label_len,
                max_label_len: opts.max_label_len,
                actual_width: 0,
                max_width: 0,
            });
        }
    }
    let span = max_width(&diagram.nodes, &diagram.edges);
    let dep = depth(&diagram.nodes, &diagram.edges);
    let (horizontal, vertical) = match diagram.direction {
        Direction::LR | Direction::RL => (dep, span),
        _ => (span, dep),
    };
    if horizontal > opts.max_width && vertical > opts.max_depth {
        warnings.push(Warning {
            kind: WarningKind::ComplexDiagram,
            file_path: fp.clone(),
            block_index: bi,
            start_line: sl,
            actual_width: horizontal,
            actual_depth: vertical,
            max_width: opts.max_width,
            max_depth: opts.max_depth,
            subgraph_label: String::new(),
            subgraph_node_count: 0,
            max_subgraph_nodes: 0,
        });
    } else if horizontal > opts.max_width {
        violations.push(Violation {
            kind: ViolationKind::WidthExceeded,
            file_path: fp.clone(),
            block_index: bi,
            start_line: sl,
            node_id: String::new(),
            label_text: String::new(),
            label_len: 0,
            max_label_len: 0,
            actual_width: horizontal,
            max_width: opts.max_width,
        });
    }
    if opts.max_subgraph_nodes > 0 {
        for sg in &diagram.subgraphs {
            if sg.node_ids.len() > opts.max_subgraph_nodes {
                warnings.push(Warning {
                    kind: WarningKind::SubgraphDense,
                    file_path: fp.clone(),
                    block_index: bi,
                    start_line: sl + sg.start_line,
                    actual_width: 0,
                    actual_depth: 0,
                    max_width: 0,
                    max_depth: 0,
                    subgraph_label: sg.label.clone(),
                    subgraph_node_count: sg.node_ids.len(),
                    max_subgraph_nodes: opts.max_subgraph_nodes,
                });
            }
        }
    }
}

// ── Reporter ─────────────────────────────────────────────────────────────

/// Returns a human-readable description of a single [`Violation`].
fn violation_detail(v: &Violation) -> String {
    match v.kind {
        ViolationKind::LabelTooLong => format!(
            "[{}] node \"{}\" label \"{}\" is {} chars (max {})",
            v.kind.code(),
            v.node_id,
            v.label_text,
            v.label_len,
            v.max_label_len
        ),
        ViolationKind::WidthExceeded => format!(
            "[{}] span {} exceeds max-width {}",
            v.kind.code(),
            v.actual_width,
            v.max_width
        ),
        ViolationKind::MultipleDiagrams => format!(
            "[{}] block contains multiple flowchart/graph headers",
            v.kind.code()
        ),
    }
}

/// Returns a human-readable description of a single [`Warning`].
fn warning_detail(w: &Warning) -> String {
    match w.kind {
        WarningKind::SubgraphDense => {
            let label = if w.subgraph_label.is_empty() {
                "(unnamed)".to_string()
            } else {
                w.subgraph_label.clone()
            };
            format!(
                "[{}] subgraph \"{label}\" has {} children; recommend ≤ {} for mobile rendering",
                w.kind.code(),
                w.subgraph_node_count,
                w.max_subgraph_nodes
            )
        }
        WarningKind::ComplexDiagram => format!(
            "[{}] span {} (max {}) and depth {} (max {}) both exceeded",
            w.kind.code(),
            w.actual_width,
            w.max_width,
            w.actual_depth,
            w.max_depth
        ),
    }
}

/// Formats a [`ValidationResult`] as human-readable text.
///
/// When `quiet` is `true` and there are no findings, returns an empty string.
/// When `verbose` is `true` or there are findings, per-file details are included.
pub fn format_text(result: &ValidationResult, verbose: bool, quiet: bool) -> String {
    let has_findings = !result.violations.is_empty() || !result.warnings.is_empty();
    if quiet && !has_findings {
        return String::new();
    }
    let mut sb = String::new();
    if verbose || has_findings {
        let mut file_violations: HashMap<String, Vec<&Violation>> = HashMap::new();
        let mut file_warnings: HashMap<String, Vec<&Warning>> = HashMap::new();
        for v in &result.violations {
            file_violations
                .entry(v.file_path.clone())
                .or_default()
                .push(v);
        }
        for w in &result.warnings {
            file_warnings
                .entry(w.file_path.clone())
                .or_default()
                .push(w);
        }
        let mut file_set: HashSet<String> = HashSet::new();
        for k in file_violations.keys() {
            file_set.insert(k.clone());
        }
        for k in file_warnings.keys() {
            file_set.insert(k.clone());
        }
        for fp in file_set {
            let vs = file_violations.get(&fp);
            let ws = file_warnings.get(&fp);
            if vs.is_some_and(|v| !v.is_empty()) {
                let _ = writeln!(sb, "✗ {fp}");
            } else if ws.is_some_and(|w| !w.is_empty()) {
                let _ = writeln!(sb, "⚠ {fp}");
            } else {
                let _ = writeln!(sb, "✓ {fp}");
            }
            if let Some(vs) = vs {
                for v in vs {
                    let _ = writeln!(
                        sb,
                        "  block {} (line {}): {}",
                        v.block_index,
                        v.start_line,
                        violation_detail(v)
                    );
                }
            }
            if let Some(ws) = ws {
                for w in ws {
                    let _ = writeln!(
                        sb,
                        "  block {} (line {}): {}",
                        w.block_index,
                        w.start_line,
                        warning_detail(w)
                    );
                }
            }
        }
    }
    let _ = writeln!(
        sb,
        "Found {} violation(s) and {} warning(s) in {} file(s) ({} block(s) scanned).",
        result.violations.len(),
        result.warnings.len(),
        result.files_scanned,
        result.blocks_scanned
    );
    sb
}

/// JSON representation of a single violation.
#[derive(Serialize)]
struct JsonViolation<'a> {
    /// Violation kind code string.
    kind: &'a str,
    /// Path to the file containing the violation.
    #[serde(rename = "filePath")]
    file_path: &'a str,
    /// Zero-based block index within the file.
    #[serde(rename = "blockIndex")]
    block_index: usize,
    /// 1-based start line of the block.
    #[serde(rename = "startLine")]
    start_line: usize,
    /// Node identifier (omitted when empty).
    #[serde(rename = "nodeId", skip_serializing_if = "str::is_empty")]
    node_id: &'a str,
    /// Label text (omitted when empty).
    #[serde(rename = "labelText", skip_serializing_if = "str::is_empty")]
    label_text: &'a str,
    /// Effective label character count (omitted when zero).
    #[serde(rename = "labelLen", skip_serializing_if = "is_zero_usize")]
    label_len: usize,
    /// Configured maximum label length (omitted when zero).
    #[serde(rename = "maxLabelLen", skip_serializing_if = "is_zero_usize")]
    max_label_len: usize,
    /// Computed diagram width (omitted when zero).
    #[serde(rename = "actualWidth", skip_serializing_if = "is_zero_usize")]
    actual_width: usize,
    /// Configured maximum width (omitted when zero).
    #[serde(rename = "maxWidth", skip_serializing_if = "is_zero_usize")]
    max_width: usize,
}

/// JSON representation of a single warning.
#[derive(Serialize)]
struct JsonWarning<'a> {
    /// Warning kind code string.
    kind: &'a str,
    /// Path to the file containing the warning.
    #[serde(rename = "filePath")]
    file_path: &'a str,
    /// Zero-based block index within the file.
    #[serde(rename = "blockIndex")]
    block_index: usize,
    /// 1-based start line of the block or subgraph.
    #[serde(rename = "startLine")]
    start_line: usize,
    /// Computed diagram width (omitted when zero).
    #[serde(rename = "actualWidth", skip_serializing_if = "is_zero_usize")]
    actual_width: usize,
    /// Computed diagram depth (omitted when zero).
    #[serde(rename = "actualDepth", skip_serializing_if = "is_zero_usize")]
    actual_depth: usize,
    /// Configured maximum width (omitted when zero).
    #[serde(rename = "maxWidth", skip_serializing_if = "is_zero_usize")]
    max_width: usize,
    /// Configured maximum depth (omitted when zero).
    #[serde(rename = "maxDepth", skip_serializing_if = "is_zero_usize")]
    max_depth: usize,
    /// Dense subgraph label (omitted when empty).
    #[serde(rename = "subgraphLabel", skip_serializing_if = "str::is_empty")]
    subgraph_label: &'a str,
    /// Number of children in the dense subgraph (omitted when zero).
    #[serde(rename = "subgraphNodeCount", skip_serializing_if = "is_zero_usize")]
    subgraph_node_count: usize,
    /// Configured maximum subgraph child count (omitted when zero).
    #[serde(rename = "maxSubgraphNodes", skip_serializing_if = "is_zero_usize")]
    max_subgraph_nodes: usize,
}

/// Returns `true` when `n` is zero; used to omit zero-valued fields from JSON output.
#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_zero_usize(n: &usize) -> bool {
    *n == 0
}

/// Top-level JSON document for the mermaid validation result.
#[derive(Serialize)]
struct JsonResult<'a> {
    /// Number of unique files scanned.
    #[serde(rename = "filesScanned")]
    files_scanned: usize,
    /// Total number of Mermaid blocks processed.
    #[serde(rename = "blocksScanned")]
    blocks_scanned: usize,
    /// All violations found.
    violations: Vec<JsonViolation<'a>>,
    /// All non-blocking warnings found.
    warnings: Vec<JsonWarning<'a>>,
}

/// Serialises the validation result to a pretty-printed JSON string.
///
/// # Errors
///
/// Returns an error when `serde_json` serialisation fails.
pub fn format_json(result: &ValidationResult) -> std::result::Result<String, Error> {
    let violations: Vec<JsonViolation> = result
        .violations
        .iter()
        .map(|v| JsonViolation {
            kind: v.kind.code(),
            file_path: &v.file_path,
            block_index: v.block_index,
            start_line: v.start_line,
            node_id: &v.node_id,
            label_text: &v.label_text,
            label_len: v.label_len,
            max_label_len: v.max_label_len,
            actual_width: v.actual_width,
            max_width: v.max_width,
        })
        .collect();
    let warnings: Vec<JsonWarning> = result
        .warnings
        .iter()
        .map(|w| JsonWarning {
            kind: w.kind.code(),
            file_path: &w.file_path,
            block_index: w.block_index,
            start_line: w.start_line,
            actual_width: w.actual_width,
            actual_depth: w.actual_depth,
            max_width: w.max_width,
            max_depth: w.max_depth,
            subgraph_label: &w.subgraph_label,
            subgraph_node_count: w.subgraph_node_count,
            max_subgraph_nodes: w.max_subgraph_nodes,
        })
        .collect();
    let out = JsonResult {
        files_scanned: result.files_scanned,
        blocks_scanned: result.blocks_scanned,
        violations,
        warnings,
    };
    Ok(serde_json::to_string_pretty(&out)?)
}

/// Formats the validation result as a Markdown table.
///
/// Returns a single-line "all passed" message when there are no findings.
pub fn format_markdown(result: &ValidationResult) -> String {
    if result.violations.is_empty() && result.warnings.is_empty() {
        return format!(
            "All {} block(s) in {} file(s) passed mermaid validation.\n",
            result.blocks_scanned, result.files_scanned
        );
    }
    let mut sb = String::new();
    sb.push_str("| File | Block | Line | Severity | Kind | Detail |\n");
    sb.push_str("|------|-------|------|----------|------|--------|\n");
    for v in &result.violations {
        let _ = writeln!(
            sb,
            "| {} | {} | {} | error | {} | {} |",
            v.file_path,
            v.block_index,
            v.start_line,
            v.kind.code(),
            violation_detail(v)
        );
    }
    for w in &result.warnings {
        let _ = writeln!(
            sb,
            "| {} | {} | {} | warning | {} | {} |",
            w.file_path,
            w.block_index,
            w.start_line,
            w.kind.code(),
            warning_detail(w)
        );
    }
    sb
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn extract_blocks_finds_mermaid_fences() {
        let content = "# T\n\n```mermaid\nflowchart TB\nA --> B\n```\n";
        let blocks = extract_blocks("a.md", content);
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].start_line, 3);
        assert!(blocks[0].source.contains("flowchart TB"));
    }

    #[test]
    fn extract_blocks_supports_tilde_fences() {
        let content = "~~~mermaid\ngraph LR\nX --> Y\n~~~\n";
        let blocks = extract_blocks("a.md", content);
        assert_eq!(blocks.len(), 1);
    }

    #[test]
    fn parse_flowchart_detects_nodes_and_edges() {
        let block = MermaidBlock {
            file_path: "a.md".to_string(),
            block_index: 0,
            source: "flowchart TB\nA --> B\nA --> C\n".to_string(),
            start_line: 1,
        };
        let (d, count) = parse_diagram(block);
        assert_eq!(count, 1);
        assert_eq!(d.nodes.len(), 3);
        assert_eq!(d.edges.len(), 2);
    }

    #[test]
    fn parse_flowchart_handles_pipe_labeled_edges() {
        // Standard Mermaid pipe-label syntax `A -->|text| B` must parse as an
        // edge — previously the `|text|` segment broke target-node extraction,
        // leaving all nodes at rank 0 and inflating the measured span.
        let block = MermaidBlock {
            file_path: "a.md".to_string(),
            block_index: 0,
            source: "graph TD\nA -->|\"browse and search\"| B\nB -->|deploy| C\n".to_string(),
            start_line: 1,
        };
        let (d, count) = parse_diagram(block);
        assert_eq!(count, 1);
        assert_eq!(d.nodes.len(), 3, "nodes: {:?}", d.nodes);
        assert_eq!(d.edges.len(), 2, "edges: {:?}", d.edges);
        // A → B → C chain: span 1, depth 3.
        assert_eq!(max_width(&d.nodes, &d.edges), 1);
        assert_eq!(depth(&d.nodes, &d.edges), 3);
    }

    #[test]
    fn rank_assign_handles_cycles_via_back_edge_removal() {
        // A → B → C → A is a cycle. Previously NO node had in-degree 0, Kahn's
        // queue started empty, and every node fell back to rank 0 — inflating
        // the measured span to the full node count. With back-edge removal the
        // chain ranks 0,1,2: span 1, depth 3.
        let block = MermaidBlock {
            file_path: "a.md".to_string(),
            block_index: 0,
            source: "graph TD\nA --> B\nB --> C\nC --> A\n".to_string(),
            start_line: 1,
        };
        let (d, _) = parse_diagram(block);
        assert_eq!(max_width(&d.nodes, &d.edges), 1, "cycle must rank as chain");
        assert_eq!(depth(&d.nodes, &d.edges), 3);
    }

    #[test]
    fn rank_assign_handles_back_edge_into_rooted_chain() {
        // Rooted chain with a feedback edge (the forms.md shape):
        // A → B → C → D plus D → B. Back edge must not zero out the ranking.
        let block = MermaidBlock {
            file_path: "a.md".to_string(),
            block_index: 0,
            source: "graph TD\nA --> B\nB --> C\nC --> D\nD --> B\n".to_string(),
            start_line: 1,
        };
        let (d, _) = parse_diagram(block);
        assert_eq!(max_width(&d.nodes, &d.edges), 1);
        assert_eq!(depth(&d.nodes, &d.edges), 4);
    }

    #[test]
    fn parse_non_flowchart_returns_zero_count() {
        let block = MermaidBlock {
            file_path: "a.md".to_string(),
            block_index: 0,
            source: "sequenceDiagram\nA -> B: Hi\n".to_string(),
            start_line: 1,
        };
        let (_, count) = parse_diagram(block);
        assert_eq!(count, 0);
    }

    #[test]
    fn effective_label_len_handles_breaks() {
        assert_eq!(effective_label_len("hello"), 5);
        assert_eq!(effective_label_len("a<br/>longer"), 6);
        assert_eq!(effective_label_len("a\\nbb"), 2);
        assert_eq!(effective_label_len(""), 0);
    }

    #[test]
    fn validate_label_too_long_emits_violation() {
        let block = MermaidBlock {
            file_path: "a.md".to_string(),
            block_index: 0,
            source: "flowchart TB\nA[ThisLabelIsLongerThan30CharsAndShouldFail] --> B\n"
                .to_string(),
            start_line: 1,
        };
        let result = validate_blocks(vec![block], default_validate_options());
        assert!(
            result
                .violations
                .iter()
                .any(|v| v.kind == ViolationKind::LabelTooLong)
        );
    }

    #[test]
    fn validate_multiple_diagrams() {
        let block = MermaidBlock {
            file_path: "a.md".to_string(),
            block_index: 0,
            source: "flowchart TB\nA --> B\nflowchart LR\nC --> D\n".to_string(),
            start_line: 1,
        };
        let result = validate_blocks(vec![block], default_validate_options());
        assert!(
            result
                .violations
                .iter()
                .any(|v| v.kind == ViolationKind::MultipleDiagrams)
        );
    }

    #[test]
    fn validate_clean_diagram_no_findings() {
        let block = MermaidBlock {
            file_path: "a.md".to_string(),
            block_index: 0,
            source: "flowchart TB\nA --> B\n".to_string(),
            start_line: 1,
        };
        let result = validate_blocks(vec![block], default_validate_options());
        assert!(result.violations.is_empty());
        assert!(result.warnings.is_empty());
    }

    #[test]
    fn max_width_simple_tree() {
        let nodes = vec![
            Node {
                id: "A".into(),
                label: String::new(),
            },
            Node {
                id: "B".into(),
                label: String::new(),
            },
            Node {
                id: "C".into(),
                label: String::new(),
            },
        ];
        let edges = vec![
            Edge {
                from: "A".into(),
                to: "B".into(),
            },
            Edge {
                from: "A".into(),
                to: "C".into(),
            },
        ];
        assert_eq!(max_width(&nodes, &edges), 2);
        assert_eq!(depth(&nodes, &edges), 2);
    }

    #[test]
    fn format_text_quiet_with_no_findings_empty() {
        let result = ValidationResult {
            files_scanned: 1,
            blocks_scanned: 1,
            violations: Vec::new(),
            warnings: Vec::new(),
        };
        assert!(format_text(&result, false, true).is_empty());
    }

    #[test]
    fn format_text_non_quiet_with_no_findings_shows_summary() {
        let result = ValidationResult {
            files_scanned: 1,
            blocks_scanned: 2,
            violations: Vec::new(),
            warnings: Vec::new(),
        };
        let s = format_text(&result, false, false);
        assert!(s.contains("Found 0 violation(s) and 0 warning(s)"));
    }

    #[test]
    fn format_markdown_clean_yields_passed_message() {
        let result = ValidationResult {
            files_scanned: 3,
            blocks_scanned: 5,
            violations: Vec::new(),
            warnings: Vec::new(),
        };
        let s = format_markdown(&result);
        assert!(s.contains("passed mermaid validation"));
    }

    #[test]
    fn format_json_serializes_clean_result() {
        let result = ValidationResult {
            files_scanned: 2,
            blocks_scanned: 3,
            violations: Vec::new(),
            warnings: Vec::new(),
        };
        let s = format_json(&result).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["filesScanned"], 2);
        assert_eq!(v["blocksScanned"], 3);
    }

    /// Builds a sample `LabelTooLong` violation for tests.
    fn label_violation() -> Violation {
        Violation {
            kind: ViolationKind::LabelTooLong,
            file_path: "a.md".to_string(),
            block_index: 0,
            start_line: 5,
            node_id: "A".to_string(),
            label_text: "too long".to_string(),
            label_len: 35,
            max_label_len: 30,
            actual_width: 0,
            max_width: 0,
        }
    }

    /// Builds a sample `WidthExceeded` violation for tests.
    fn width_violation() -> Violation {
        Violation {
            kind: ViolationKind::WidthExceeded,
            file_path: "a.md".to_string(),
            block_index: 0,
            start_line: 10,
            node_id: String::new(),
            label_text: String::new(),
            label_len: 0,
            max_label_len: 0,
            actual_width: 5,
            max_width: 4,
        }
    }

    /// Builds a sample `MultipleDiagrams` violation for tests.
    fn multi_violation() -> Violation {
        Violation {
            kind: ViolationKind::MultipleDiagrams,
            file_path: "a.md".to_string(),
            block_index: 0,
            start_line: 1,
            node_id: String::new(),
            label_text: String::new(),
            label_len: 0,
            max_label_len: 0,
            actual_width: 0,
            max_width: 0,
        }
    }

    /// Builds a sample `SubgraphDense` warning for tests.
    fn dense_warning() -> Warning {
        Warning {
            kind: WarningKind::SubgraphDense,
            file_path: "a.md".to_string(),
            block_index: 0,
            start_line: 7,
            actual_width: 0,
            actual_depth: 0,
            max_width: 0,
            max_depth: 0,
            subgraph_label: "Foo".to_string(),
            subgraph_node_count: 8,
            max_subgraph_nodes: 6,
        }
    }

    /// Builds a sample `ComplexDiagram` warning for tests.
    fn complex_warning() -> Warning {
        Warning {
            kind: WarningKind::ComplexDiagram,
            file_path: "a.md".to_string(),
            block_index: 0,
            start_line: 3,
            actual_width: 6,
            actual_depth: 5,
            max_width: 4,
            max_depth: 4,
            subgraph_label: String::new(),
            subgraph_node_count: 0,
            max_subgraph_nodes: 0,
        }
    }

    #[test]
    fn format_text_with_violations_renders() {
        let result = ValidationResult {
            files_scanned: 1,
            blocks_scanned: 1,
            violations: vec![label_violation(), width_violation(), multi_violation()],
            warnings: Vec::new(),
        };
        let s = format_text(&result, false, false);
        assert!(s.contains("✗ a.md"));
        assert!(s.contains("label_too_long"));
        assert!(s.contains("width_exceeded"));
        assert!(s.contains("multiple_diagrams"));
    }

    #[test]
    fn format_text_with_warnings_only_uses_warn_marker() {
        let result = ValidationResult {
            files_scanned: 1,
            blocks_scanned: 1,
            violations: Vec::new(),
            warnings: vec![dense_warning(), complex_warning()],
        };
        let s = format_text(&result, false, false);
        assert!(s.contains("⚠ a.md"));
        assert!(s.contains("subgraph_density"));
        assert!(s.contains("complex_diagram"));
    }

    #[test]
    fn format_text_verbose_with_no_findings_shows_summary() {
        let result = ValidationResult {
            files_scanned: 2,
            blocks_scanned: 3,
            violations: Vec::new(),
            warnings: Vec::new(),
        };
        let s = format_text(&result, true, false);
        assert!(s.contains("Found 0 violation"));
    }

    #[test]
    fn format_markdown_with_findings_renders_table() {
        let result = ValidationResult {
            files_scanned: 1,
            blocks_scanned: 1,
            violations: vec![label_violation()],
            warnings: vec![dense_warning()],
        };
        let s = format_markdown(&result);
        assert!(s.contains("| File | Block | Line | Severity | Kind | Detail |"));
        assert!(s.contains("error"));
        assert!(s.contains("warning"));
    }

    #[test]
    fn format_json_with_findings_serializes() {
        let result = ValidationResult {
            files_scanned: 1,
            blocks_scanned: 1,
            violations: vec![label_violation(), width_violation()],
            warnings: vec![complex_warning()],
        };
        let s = format_json(&result).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["violations"].as_array().unwrap().len(), 2);
        assert_eq!(v["warnings"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn parse_subgraph_with_id_and_label() {
        let block = MermaidBlock {
            file_path: "a.md".to_string(),
            block_index: 0,
            source: "flowchart TB\nsubgraph WF1 [Workflow 1]\nA --> B\nend\nC --> D\n".to_string(),
            start_line: 1,
        };
        let (d, count) = parse_diagram(block);
        assert_eq!(count, 1);
        assert_eq!(d.subgraphs.len(), 1);
        assert_eq!(d.subgraphs[0].id, "WF1");
        assert_eq!(d.subgraphs[0].label, "Workflow 1");
        assert!(d.subgraphs[0].node_ids.contains(&"A".to_string()));
        assert!(d.subgraphs[0].node_ids.contains(&"B".to_string()));
    }

    #[test]
    fn parse_direction_lr_recognised() {
        let block = MermaidBlock {
            file_path: "a.md".to_string(),
            block_index: 0,
            source: "flowchart LR\nA --> B\n".to_string(),
            start_line: 1,
        };
        let (d, _) = parse_diagram(block);
        assert!(matches!(d.direction, Direction::LR));
    }

    #[test]
    fn parse_node_shapes_extract_labels() {
        let block = MermaidBlock {
            file_path: "a.md".to_string(),
            block_index: 0,
            source: "flowchart TB\nA[Rectangle]\nB((Circle))\nC{Diamond}\n".to_string(),
            start_line: 1,
        };
        let (d, _) = parse_diagram(block);
        let labels: Vec<&str> = d.nodes.iter().map(|n| n.label.as_str()).collect();
        assert!(labels.contains(&"Rectangle"));
        assert!(labels.contains(&"Circle"));
        assert!(labels.contains(&"Diamond"));
    }

    #[test]
    fn parse_edge_with_label_text() {
        let block = MermaidBlock {
            file_path: "a.md".to_string(),
            block_index: 0,
            source: "flowchart TB\nA -- text --> B\n".to_string(),
            start_line: 1,
        };
        let (d, _) = parse_diagram(block);
        assert_eq!(d.edges.len(), 1);
        assert_eq!(d.edges[0].from, "A");
        assert_eq!(d.edges[0].to, "B");
    }

    #[test]
    fn parse_cartesian_product_edges() {
        let block = MermaidBlock {
            file_path: "a.md".to_string(),
            block_index: 0,
            source: "flowchart TB\nA & B --> C & D\n".to_string(),
            start_line: 1,
        };
        let (d, _) = parse_diagram(block);
        // A&B → C&D = 4 edges
        assert_eq!(d.edges.len(), 4);
    }

    #[test]
    fn direction_parse_handles_unknowns() {
        assert!(matches!(Direction::parse("XX"), Direction::TB));
        assert!(matches!(Direction::parse("BT"), Direction::BT));
    }

    #[test]
    fn violation_kind_codes_match_go() {
        assert_eq!(ViolationKind::LabelTooLong.code(), "label_too_long");
        assert_eq!(ViolationKind::WidthExceeded.code(), "width_exceeded");
        assert_eq!(ViolationKind::MultipleDiagrams.code(), "multiple_diagrams");
    }

    #[test]
    fn warning_kind_codes_match_go() {
        assert_eq!(WarningKind::ComplexDiagram.code(), "complex_diagram");
        assert_eq!(WarningKind::SubgraphDense.code(), "subgraph_density");
    }

    #[test]
    fn validate_subgraph_density_warns() {
        let source = "flowchart TB\nsubgraph WF1 [F]\nA & B & C & D & E & F & G --> Z\nend\n";
        let block = MermaidBlock {
            file_path: "a.md".to_string(),
            block_index: 0,
            source: source.to_string(),
            start_line: 1,
        };
        let result = validate_blocks(vec![block], default_validate_options());
        assert!(
            result
                .warnings
                .iter()
                .any(|w| w.kind == WarningKind::SubgraphDense)
        );
    }
}
