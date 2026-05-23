// Byte-for-byte port of `apps/rhino-cli/internal/mermaid/`.
// Combined into a single file: types + extractor + parser + graph + validator + reporter.

use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;
use std::sync::OnceLock;

use anyhow::Error;
use regex::Regex;
use serde::Serialize;

// ── Types ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    TB,
    TD,
    BT,
    LR,
    RL,
}

impl Direction {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViolationKind {
    LabelTooLong,
    WidthExceeded,
    MultipleDiagrams,
}

impl ViolationKind {
    pub fn code(&self) -> &'static str {
        match self {
            ViolationKind::LabelTooLong => "label_too_long",
            ViolationKind::WidthExceeded => "width_exceeded",
            ViolationKind::MultipleDiagrams => "multiple_diagrams",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WarningKind {
    ComplexDiagram,
    SubgraphDense,
}

impl WarningKind {
    pub fn code(&self) -> &'static str {
        match self {
            WarningKind::ComplexDiagram => "complex_diagram",
            WarningKind::SubgraphDense => "subgraph_density",
        }
    }
}

#[derive(Debug, Clone)]
pub struct MermaidBlock {
    pub file_path: String,
    pub block_index: usize,
    pub source: String,
    pub start_line: usize,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: String,
    pub label: String,
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone)]
pub struct Subgraph {
    pub id: String,
    pub label: String,
    pub node_ids: Vec<String>,
    pub start_line: usize,
}

pub struct ParsedDiagram {
    pub block: MermaidBlock,
    pub direction: Direction,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub subgraphs: Vec<Subgraph>,
}

#[derive(Debug, Clone)]
pub struct Violation {
    pub kind: ViolationKind,
    pub file_path: String,
    pub block_index: usize,
    pub start_line: usize,
    pub node_id: String,
    pub label_text: String,
    pub label_len: usize,
    pub max_label_len: usize,
    pub actual_width: usize,
    pub max_width: usize,
}

#[derive(Debug, Clone)]
pub struct Warning {
    pub kind: WarningKind,
    pub file_path: String,
    pub block_index: usize,
    pub start_line: usize,
    pub actual_width: usize,
    pub actual_depth: usize,
    pub max_width: usize,
    pub max_depth: usize,
    pub subgraph_label: String,
    pub subgraph_node_count: usize,
    pub max_subgraph_nodes: usize,
}

pub struct ValidationResult {
    pub files_scanned: usize,
    pub blocks_scanned: usize,
    pub violations: Vec<Violation>,
    pub warnings: Vec<Warning>,
}

#[derive(Debug, Clone, Copy)]
pub struct ValidateOptions {
    pub max_label_len: usize,
    pub max_width: usize,
    pub max_depth: usize,
    pub max_subgraph_nodes: usize,
}

// ── Extractor ────────────────────────────────────────────────────────────

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

fn flowchart_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?m)^\s*(flowchart|graph)(\s+(TB|TD|BT|LR|RL))?\s*$").unwrap())
}

fn subgraph_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"^subgraph(?:\s+([^\s\["]+))?(?:\s*\[\s*"?([^"\]]*)"?\s*\])?\s*$"#).unwrap()
    })
}

fn arrow_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"-->|---|-\.->|==>|--o|--x|<-->").unwrap())
}

fn link_text_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"--[^->\n]+?-->").unwrap())
}

fn node_id_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^(\w+)$").unwrap())
}

fn node_shape_patterns() -> &'static Vec<Regex> {
    static PATTERNS: OnceLock<Vec<Regex>> = OnceLock::new();
    PATTERNS.get_or_init(|| {
        vec![
            Regex::new(r"^(\w+)\(\(\(([^)]*)\)\)\)").unwrap(),
            Regex::new(r"^(\w+)\(\[([^\]]*)\]\)").unwrap(),
            Regex::new(r"^(\w+)\(\(([^)]*)\)\)").unwrap(),
            Regex::new(r"^(\w+)\[\[([^\]]*)\]\]").unwrap(),
            Regex::new(r"^(\w+)\[\(([^)]*)\)\]").unwrap(),
            Regex::new(r"^(\w+)\(([^)]*)\)").unwrap(),
            Regex::new(r"^(\w+)\{\{([^}]*)\}\}").unwrap(),
            Regex::new(r"^(\w+)\{([^}]*)\}").unwrap(),
            Regex::new(r"^(\w+)>([^\]]*)\]").unwrap(),
            Regex::new(r"^(\w+)\[/([^/]*)/\]").unwrap(),
            Regex::new(r"^(\w+)\[\\([^\\]*)\\]").unwrap(),
            Regex::new(r"^(\w+)\[([^\]]*)\]").unwrap(),
            Regex::new(r#"^(\w+)@\{\s*[^}]*label:\s*"([^"]*)"\s*[^}]*\}"#).unwrap(),
        ]
    })
}

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

fn extract_node_ids_from_segment(seg: &str) -> Vec<String> {
    seg.split('&')
        .filter_map(|sub| {
            let id = extract_node_id_from_segment(sub);
            if id.is_empty() {
                None
            } else {
                Some(id)
            }
        })
        .collect()
}

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

fn extract_edge_line(
    line: &str,
    node_map: &mut Vec<(String, String)>,
    node_index: &mut HashMap<String, usize>,
    edges: &mut Vec<Edge>,
) {
    let line = link_text_re().replace_all(line, "-->");
    let parts: Vec<&str> = arrow_re().split(&line).collect();
    if parts.len() < 2 {
        return;
    }
    let groups: Vec<Vec<String>> = parts
        .iter()
        .filter_map(|p| {
            let ids = extract_node_group(p, node_map, node_index);
            if ids.is_empty() {
                None
            } else {
                Some(ids)
            }
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
            if id.is_empty() {
                None
            } else {
                Some(id)
            }
        })
        .collect()
}

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

fn rank_assign(nodes: &[Node], edges: &[Edge]) -> HashMap<String, i64> {
    if nodes.is_empty() {
        return HashMap::new();
    }
    let node_set: HashSet<&str> = nodes.iter().map(|n| n.id.as_str()).collect();
    let mut adj: HashMap<String, Vec<String>> = HashMap::new();
    let mut in_degree: HashMap<String, usize> = HashMap::new();
    for n in nodes {
        adj.insert(n.id.clone(), Vec::new());
        in_degree.insert(n.id.clone(), 0);
    }
    for e in edges {
        if node_set.contains(e.from.as_str()) && node_set.contains(e.to.as_str()) {
            adj.entry(e.from.clone()).or_default().push(e.to.clone());
            *in_degree.entry(e.to.clone()).or_insert(0) += 1;
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
            rank.insert(n.id.clone(), 0);
        }
    }
    rank
}

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

pub fn depth(nodes: &[Node], edges: &[Edge]) -> usize {
    if nodes.is_empty() {
        return 0;
    }
    let ranks = rank_assign(nodes, edges);
    ranks.values().collect::<HashSet<_>>().len()
}

// ── Validator ────────────────────────────────────────────────────────────

pub fn default_validate_options() -> ValidateOptions {
    ValidateOptions {
        max_label_len: 30,
        max_width: 4,
        max_depth: usize::MAX,
        max_subgraph_nodes: 6,
    }
}

pub fn validate_blocks(blocks: Vec<MermaidBlock>, opts: ValidateOptions) -> ValidationResult {
    let mut files_seen: HashSet<String> = HashSet::new();
    let mut violations = Vec::new();
    let mut warnings = Vec::new();
    let total = blocks.len();
    for block in blocks {
        files_seen.insert(block.file_path.clone());
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
            continue;
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
    ValidationResult {
        files_scanned: files_seen.len(),
        blocks_scanned: total,
        violations,
        warnings,
    }
}

// ── Reporter ─────────────────────────────────────────────────────────────

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

#[derive(Serialize)]
struct JsonViolation<'a> {
    kind: &'a str,
    #[serde(rename = "filePath")]
    file_path: &'a str,
    #[serde(rename = "blockIndex")]
    block_index: usize,
    #[serde(rename = "startLine")]
    start_line: usize,
    #[serde(rename = "nodeId", skip_serializing_if = "str::is_empty")]
    node_id: &'a str,
    #[serde(rename = "labelText", skip_serializing_if = "str::is_empty")]
    label_text: &'a str,
    #[serde(rename = "labelLen", skip_serializing_if = "is_zero_usize")]
    label_len: usize,
    #[serde(rename = "maxLabelLen", skip_serializing_if = "is_zero_usize")]
    max_label_len: usize,
    #[serde(rename = "actualWidth", skip_serializing_if = "is_zero_usize")]
    actual_width: usize,
    #[serde(rename = "maxWidth", skip_serializing_if = "is_zero_usize")]
    max_width: usize,
}

#[derive(Serialize)]
struct JsonWarning<'a> {
    kind: &'a str,
    #[serde(rename = "filePath")]
    file_path: &'a str,
    #[serde(rename = "blockIndex")]
    block_index: usize,
    #[serde(rename = "startLine")]
    start_line: usize,
    #[serde(rename = "actualWidth", skip_serializing_if = "is_zero_usize")]
    actual_width: usize,
    #[serde(rename = "actualDepth", skip_serializing_if = "is_zero_usize")]
    actual_depth: usize,
    #[serde(rename = "maxWidth", skip_serializing_if = "is_zero_usize")]
    max_width: usize,
    #[serde(rename = "maxDepth", skip_serializing_if = "is_zero_usize")]
    max_depth: usize,
    #[serde(rename = "subgraphLabel", skip_serializing_if = "str::is_empty")]
    subgraph_label: &'a str,
    #[serde(rename = "subgraphNodeCount", skip_serializing_if = "is_zero_usize")]
    subgraph_node_count: usize,
    #[serde(rename = "maxSubgraphNodes", skip_serializing_if = "is_zero_usize")]
    max_subgraph_nodes: usize,
}

fn is_zero_usize(n: &usize) -> bool {
    *n == 0
}

#[derive(Serialize)]
struct JsonResult<'a> {
    #[serde(rename = "filesScanned")]
    files_scanned: usize,
    #[serde(rename = "blocksScanned")]
    blocks_scanned: usize,
    violations: Vec<JsonViolation<'a>>,
    warnings: Vec<JsonWarning<'a>>,
}

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
        assert!(result
            .violations
            .iter()
            .any(|v| v.kind == ViolationKind::LabelTooLong));
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
        assert!(result
            .violations
            .iter()
            .any(|v| v.kind == ViolationKind::MultipleDiagrams));
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
        assert!(result
            .warnings
            .iter()
            .any(|w| w.kind == WarningKind::SubgraphDense));
    }
}
