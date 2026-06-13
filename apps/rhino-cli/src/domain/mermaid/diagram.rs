//! Diagram kind detection — identifies the type of a Mermaid block from its header.

use super::types::DiagramKind;

/// Detects the kind of a Mermaid diagram from its raw source.
///
/// Returns [`DiagramKind::Flowchart`] for `flowchart`/`graph` headers,
/// [`DiagramKind::State`] for `stateDiagram-v2`/`stateDiagram` headers,
/// and [`DiagramKind::Other`] for all other types (sequence, pie, class, etc.).
///
/// Only the first non-empty header line is inspected.
pub fn detect_kind(source: &str) -> DiagramKind {
    for line in source.lines() {
        let t = line.trim();
        if t.is_empty() {
            continue;
        }
        if t.starts_with("flowchart") || t.starts_with("graph ") || t == "graph" {
            return DiagramKind::Flowchart;
        }
        if t.starts_with("stateDiagram-v2") || t.starts_with("stateDiagram") {
            return DiagramKind::State;
        }
        // Any other non-empty first line is an unrecognised diagram type.
        return DiagramKind::Other;
    }
    DiagramKind::Other
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn detect_kind_returns_state_for_state_diagram_v2() {
        assert_eq!(
            detect_kind("stateDiagram-v2\nA --> B"),
            DiagramKind::State,
            "stateDiagram-v2 header must be detected as State"
        );
    }

    #[test]
    fn detect_kind_returns_state_for_state_diagram_v1() {
        assert_eq!(
            detect_kind("stateDiagram\nA --> B"),
            DiagramKind::State,
            "stateDiagram header must be detected as State"
        );
    }
}
