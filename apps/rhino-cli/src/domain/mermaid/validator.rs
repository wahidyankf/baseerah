//! Mermaid diagram validation rules.

use std::collections::HashSet;

use super::flowchart::parse_diagram;
use super::graph::{depth, effective_label_len, max_width};
use super::types::{
    Direction, MermaidBlock, ValidateOptions, ValidationResult, Violation, ViolationKind, Warning,
    WarningKind,
};

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
