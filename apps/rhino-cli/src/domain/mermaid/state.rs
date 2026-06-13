//! State-diagram parser for `stateDiagram-v2` / `stateDiagram` blocks.
//!
//! Implements the grammar facts from
//! `plans/in-progress/standardize-repo-toolchain-parity/tech-docs.md
//! § Pinned grammar facts for state.rs`.
//!
//! Phase 8 feature schedule:
//! - P8a GREEN (this file): header, direction, `-->` edge + node extraction.
//! - P8b GREEN: stereotype nodes (`<<choice>>`/`<<fork>>`/`<<join>>`/`[[...]]`),
//!   composite `state X { }` as Subgraph, note/comment/`--` skipping, label rules.

use std::collections::HashMap;

use super::types::{Direction, Edge, MermaidBlock, Node, ParsedDiagram, Subgraph};

/// Parses a `stateDiagram-v2` / `stateDiagram` block into a [`ParsedDiagram`].
///
/// Handles: header skip, direction, `-->` edges with optional labels,
/// `state "label" as id` aliases, `id : desc` descriptions, composite
/// `state X { }` blocks as Subgraphs, and `%%`/`#` comments.
/// `--` concurrent-region separators and notes are skipped (handled in P8b).
/// Direction defaults to `TB`; `TD` is not valid in state diagrams (falls back to `TB`).
pub fn parse_state(block: MermaidBlock) -> ParsedDiagram {
    let mut direction = Direction::TB;
    let mut node_order: Vec<String> = Vec::new();
    let mut node_labels: HashMap<String, String> = HashMap::new();
    let mut edges: Vec<Edge> = Vec::new();
    let mut subgraphs: Vec<Subgraph> = Vec::new();
    // Stack of open composite states: (composite_id, child_node_ids, start_line).
    let mut composite_stack: Vec<(String, Vec<String>, usize)> = Vec::new();

    for (line_idx, raw) in block.source.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        // Skip diagram headers.
        if line.starts_with("stateDiagram-v2") || line.starts_with("stateDiagram") {
            continue;
        }
        // Skip `%%` and `#` comments.
        if line.starts_with("%%") || line.starts_with('#') {
            continue;
        }
        // Skip `--` concurrent-region separator (not a transition, not a node).
        if line == "--" {
            continue;
        }
        // Close composite: `}` pops the stack.
        if line == "}" {
            if let Some((id, child_ids, start_line)) = composite_stack.pop() {
                ensure_node(&id, &id, &mut node_order, &mut node_labels);
                subgraphs.push(Subgraph {
                    id,
                    label: String::new(),
                    node_ids: child_ids,
                    start_line,
                });
            }
            continue;
        }
        // Open composite: `state X {` or `state "label" as X {`.
        if let Some(composite_id) = parse_composite_open(line) {
            composite_stack.push((composite_id, Vec::new(), line_idx + 1));
            continue;
        }
        // Parse `direction` keyword (TB|BT|LR|RL; TD invalid → TB).
        if let Some(rest) = line.strip_prefix("direction ") {
            direction = match rest.trim() {
                "LR" => Direction::LR,
                "RL" => Direction::RL,
                "BT" => Direction::BT,
                _ => Direction::TB,
            };
            continue;
        }
        // Arrow line: `FROM --> TO` or `FROM --> TO : label`.
        // Match `-->` before `--` (a `--` alone is a concurrent-region separator).
        if line.contains("-->") {
            if let Some((from, to, edge_label)) = parse_arrow(line) {
                ensure_node(&from, &from, &mut node_order, &mut node_labels);
                ensure_node(&to, &to, &mut node_order, &mut node_labels);
                // Track nodes belonging to the innermost open composite.
                if let Some(top) = composite_stack.last_mut() {
                    if !top.1.contains(&from) {
                        top.1.push(from.clone());
                    }
                    if !top.1.contains(&to) {
                        top.1.push(to.clone());
                    }
                }
                edges.push(Edge {
                    from,
                    to,
                    label: edge_label,
                });
            }
            continue;
        }
        // `state "label" as ID` — named state alias (non-composite form).
        if let Some((id, label)) = parse_state_as(line) {
            ensure_node(&id, &label, &mut node_order, &mut node_labels);
            continue;
        }
        // `ID : description` — bare state with inline description.
        if let Some((id, label)) = parse_colon_desc(line) {
            ensure_node(&id, &label, &mut node_order, &mut node_labels);
        }
        // Other lines (notes, `end`, bare ids) are handled in P8b.
    }

    let nodes: Vec<Node> = node_order
        .into_iter()
        .map(|id| {
            let label = node_labels.get(&id).cloned().unwrap_or_default();
            Node { id, label }
        })
        .collect();

    ParsedDiagram {
        block,
        direction,
        nodes,
        edges,
        subgraphs,
    }
}

/// Inserts `id` into the node list if not already present.
fn ensure_node(
    id: &str,
    label: &str,
    order: &mut Vec<String>,
    labels: &mut HashMap<String, String>,
) {
    if !labels.contains_key(id) {
        order.push(id.to_string());
        labels.insert(id.to_string(), label.to_string());
    }
}

/// Parses `state X {` or `state "label" as X {`, returning the composite ID.
///
/// Returns `None` for non-composite `state` lines (e.g. `state "desc" as id` without `{`).
fn parse_composite_open(line: &str) -> Option<String> {
    if !line.starts_with("state ") || !line.ends_with('{') {
        return None;
    }
    let inner = line["state ".len()..line.len() - 1].trim();
    // `state "label" as ID {`
    if let Some(rest) = inner.strip_prefix('"') {
        let quote_end = rest.find('"')?;
        let after = rest[quote_end + 1..].trim();
        let id = after.strip_prefix("as ")?.trim();
        if id.is_empty() {
            return None;
        }
        return Some(id.to_string());
    }
    // `state ID {`
    if inner.is_empty() {
        return None;
    }
    Some(inner.to_string())
}

/// Parses `FROM --> TO` or `FROM --> TO : label`, returning `(from_id, to_id, label)`.
///
/// Returns `None` when either side is empty after trimming.
fn parse_arrow(line: &str) -> Option<(String, String, String)> {
    let mut parts = line.splitn(2, "-->");
    let from = parts.next()?.trim();
    let rhs = parts.next()?.trim();
    // Split off optional label: prefer ` : ` (space-colon-space), then `: ` (colon-space).
    let (to, label) = if let Some(colon) = rhs.find(" : ") {
        (rhs[..colon].trim(), rhs[colon + 3..].trim())
    } else if let Some(colon) = rhs.find(": ") {
        (rhs[..colon].trim(), rhs[colon + 2..].trim())
    } else {
        (rhs.trim(), "")
    };
    if from.is_empty() || to.is_empty() {
        return None;
    }
    Some((from.to_string(), to.to_string(), label.to_string()))
}

/// Parses `state "label" as ID` lines.
fn parse_state_as(line: &str) -> Option<(String, String)> {
    let rest = line.strip_prefix("state \"")?;
    let quote_end = rest.find('"')?;
    let label = &rest[..quote_end];
    let after = rest[quote_end + 1..].trim();
    let id = after.strip_prefix("as ")?.trim();
    if id.is_empty() {
        return None;
    }
    Some((id.to_string(), label.to_string()))
}

/// Parses `ID : description` lines where `ID` is a valid state identifier.
fn parse_colon_desc(line: &str) -> Option<(String, String)> {
    let colon = line.find(" : ")?;
    let id = line[..colon].trim();
    let label = line[colon + 3..].trim();
    if id.is_empty() || !is_state_id(id) {
        return None;
    }
    Some((id.to_string(), label.to_string()))
}

/// Returns `true` when `s` looks like a valid Mermaid state identifier:
/// alphanumeric/underscore, `[*]`, or a stereotype like `<<choice>>`.
fn is_state_id(s: &str) -> bool {
    if s == "[*]" {
        return true;
    }
    if s.starts_with("<<") && s.ends_with(">>") {
        return true;
    }
    s.chars().all(|c| c.is_alphanumeric() || c == '_')
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    /// 11-state `direction LR` chain: A → B → … → K (11 nodes, 10 edges).
    ///
    /// RED: `parse_state` stub returns empty nodes — test FAILS until Phase 8 GREEN.
    #[test]
    fn parse_state_11_node_lr_chain_detects_nodes_and_edges() {
        let source = "stateDiagram-v2\n  direction LR\n  A --> B\n  B --> C\n  C --> D\n  D --> E\n  E --> F\n  F --> G\n  G --> H\n  H --> I\n  I --> J\n  J --> K\n";
        let block = MermaidBlock {
            file_path: "test.md".to_string(),
            block_index: 0,
            source: source.to_string(),
            start_line: 1,
        };
        let d = parse_state(block);
        assert_eq!(
            d.nodes.len(),
            11,
            "expected 11 nodes A–K, got {:?}",
            d.nodes
        );
        assert_eq!(d.edges.len(), 10, "expected 10 edges");
        assert!(
            matches!(d.direction, Direction::LR),
            "expected LR direction"
        );
    }
}
