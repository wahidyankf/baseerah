//! Pure Mermaid diagram domain model — types, graph metrics, parsers, validator.

/// Flowchart block extractor and parser.
pub mod flowchart;
/// Graph metric utilities: rank assignment, width, depth.
pub mod graph;
/// State-diagram parser stub (behavior lands in Phase 8).
pub mod state;
/// Core domain types for Mermaid diagram validation.
pub mod types;
/// Diagram validation rules.
pub mod validator;

pub use flowchart::{extract_blocks, parse_diagram};
pub use graph::{depth, effective_label_len, max_width};
pub use types::{
    Direction, Edge, MermaidBlock, Node, ParsedDiagram, Subgraph, ValidateOptions,
    ValidationResult, Violation, ViolationKind, Warning, WarningKind,
};
pub use validator::{default_validate_options, validate_blocks};

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
