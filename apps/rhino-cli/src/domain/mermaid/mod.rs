//! Pure Mermaid diagram domain model — types, graph metrics, parsers, validator.

/// Diagram kind detection utilities.
pub mod diagram;
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

pub use diagram::detect_kind;
pub use flowchart::{extract_blocks, parse_diagram};
pub use graph::{depth, effective_label_len, max_width};
pub use types::{
    DiagramKind, Direction, Edge, MermaidBlock, Node, ParsedDiagram, Subgraph, ValidateOptions,
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
    fn validate_state_11_node_lr_chain_width_exceeded() {
        // 11-state LR chain: depth = 11 ranks, which maps to horizontal on LR.
        // width_exceeded expected with actual_width == 11 (> max_width 4).
        // RED: validate_blocks currently skips state diagrams → no violations.
        let source = "stateDiagram-v2\n  direction LR\n  A --> B\n  B --> C\n  C --> D\n  D --> E\n  E --> F\n  F --> G\n  G --> H\n  H --> I\n  I --> J\n  J --> K\n";
        let block = MermaidBlock {
            file_path: "test.md".to_string(),
            block_index: 0,
            source: source.to_string(),
            start_line: 1,
        };
        let result = validate_blocks(vec![block], default_validate_options());
        let w_exceeded = result
            .violations
            .iter()
            .find(|v| v.kind == ViolationKind::WidthExceeded);
        assert!(
            w_exceeded.is_some(),
            "expected WidthExceeded violation, got none"
        );
        assert_eq!(
            w_exceeded.unwrap().actual_width,
            11,
            "expected width 11, got {}",
            w_exceeded.map_or(0, |v| v.actual_width)
        );
    }

    #[test]
    fn validate_state_long_display_label_emits_label_too_long() {
        // state "desc" as id — desc > 30 chars must yield label_too_long.
        // RED: state display labels not yet checked by validator.
        let source = "stateDiagram-v2\n  state \"ThisLabelIsLongerThan30CharsAndFails\" as S1\n  S1 --> S2\n";
        let block = MermaidBlock {
            file_path: "test.md".to_string(),
            block_index: 0,
            source: source.to_string(),
            start_line: 1,
        };
        let result = validate_blocks(vec![block], default_validate_options());
        assert!(
            result
                .violations
                .iter()
                .any(|v| v.kind == ViolationKind::LabelTooLong),
            "expected LabelTooLong for long state display label, got {:?}",
            result.violations
        );
    }

    #[test]
    fn validate_state_long_transition_label_emits_label_too_long() {
        // Transition labels `A --> B : long-text` checked against max_label_len.
        // RED: transition labels not yet stored or checked.
        let source =
            "stateDiagram-v2\n  A --> B : ThisTransitionLabelIsLongerThan30CharsAndFails\n";
        let block = MermaidBlock {
            file_path: "test.md".to_string(),
            block_index: 0,
            source: source.to_string(),
            start_line: 1,
        };
        let result = validate_blocks(vec![block], default_validate_options());
        assert!(
            result
                .violations
                .iter()
                .any(|v| v.kind == ViolationKind::LabelTooLong),
            "expected LabelTooLong for long transition label, got {:?}",
            result.violations
        );
    }

    #[test]
    fn validate_state_short_colon_label_no_violation() {
        // Short transition label must not emit label_too_long.
        let source = "stateDiagram-v2\n  A --> B : ok\n";
        let block = MermaidBlock {
            file_path: "test.md".to_string(),
            block_index: 0,
            source: source.to_string(),
            start_line: 1,
        };
        let result = validate_blocks(vec![block], default_validate_options());
        assert!(
            !result
                .violations
                .iter()
                .any(|v| v.kind == ViolationKind::LabelTooLong),
            "expected no LabelTooLong for short transition label, got {:?}",
            result.violations
        );
    }

    #[test]
    fn validate_state_pseudostates_count_toward_width() {
        // A single rank with [*], <<choice>>, <<fork>>, <<join>>, plus one more = 5 nodes.
        // With max_width=4, this must yield width_exceeded (actual_width >= 5).
        // RED: parse_state doesn't yet parse standalone stereotype node declarations.
        let source = "stateDiagram-v2\n  direction LR\n  [*] --> <<choice>>\n  <<choice>> --> <<fork>>\n  <<fork>> --> <<join>>\n  <<join>> --> Extra\n  Extra --> [*]\n";
        let block = MermaidBlock {
            file_path: "test.md".to_string(),
            block_index: 0,
            source: source.to_string(),
            start_line: 1,
        };
        let result = validate_blocks(vec![block], default_validate_options());
        let w = result
            .violations
            .iter()
            .find(|v| v.kind == ViolationKind::WidthExceeded);
        assert!(
            w.is_some(),
            "expected WidthExceeded for 5-node pseudostate rank, got none"
        );
        assert!(
            w.unwrap().actual_width >= 5,
            "expected actual_width >= 5, got {}",
            w.map_or(0, |v| v.actual_width)
        );
    }

    #[test]
    fn parse_state_composite_recorded_as_subgraph() {
        // `state Outer { Inner1 --> Inner2 }` — Outer is a Subgraph.
        // RED: parse_state doesn't yet handle composite blocks.
        use crate::domain::mermaid::state::parse_state;
        let source = "stateDiagram-v2\n  state Outer {\n    Inner1 --> Inner2\n  }\n";
        let block = MermaidBlock {
            file_path: "test.md".to_string(),
            block_index: 0,
            source: source.to_string(),
            start_line: 1,
        };
        let d = parse_state(block);
        assert_eq!(
            d.subgraphs.len(),
            1,
            "expected 1 subgraph for Outer composite"
        );
        assert_eq!(d.subgraphs[0].id, "Outer");
        assert!(
            d.subgraphs[0].node_ids.contains(&"Inner1".to_string()),
            "Outer subgraph must contain Inner1"
        );
        assert!(
            d.subgraphs[0].node_ids.contains(&"Inner2".to_string()),
            "Outer subgraph must contain Inner2"
        );
    }

    #[test]
    fn parse_state_skips_notes_comments_and_separator() {
        // Multiline note, %% comment, and -- separator must not produce spurious nodes.
        // RED: parse_state doesn't yet skip multiline notes.
        use crate::domain::mermaid::state::parse_state;
        let source = concat!(
            "stateDiagram-v2\n",
            "  A --> B\n",
            "  note right of A\n",
            "    Some free text that should not parse as a node or edge\n",
            "  end note\n",
            "  %% this is a comment\n",
            "  --\n",
        );
        let make_block = || MermaidBlock {
            file_path: "test.md".to_string(),
            block_index: 0,
            source: source.to_string(),
            start_line: 1,
        };
        let d = parse_state(make_block());
        // Only A and B should be nodes; note text must not become nodes.
        assert_eq!(
            d.nodes.len(),
            2,
            "expected 2 nodes (A, B), got {:?}",
            d.nodes
        );
        assert_eq!(d.edges.len(), 1, "expected 1 edge");
        // Also verify via validate_blocks: 2-node TB chain (depth 2, width 1) = no violations.
        let result = validate_blocks(vec![make_block()], default_validate_options());
        assert!(
            result.violations.is_empty(),
            "expected no violations, got {:?}",
            result.violations
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
                label: String::new(),
            },
            Edge {
                from: "A".into(),
                to: "C".into(),
                label: String::new(),
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
