//! Core domain types for Mermaid diagram validation.

/// Category of a Mermaid diagram block.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagramKind {
    /// A `flowchart` or `graph` block.
    Flowchart,
    /// A `stateDiagram-v2` or `stateDiagram` block.
    State,
    /// Any other diagram type (sequence, pie, class, etc.).
    Other,
}

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
    /// Optional edge label (state transition text after ` : `; empty for flowcharts).
    pub label: String,
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

/// Aggregated result of a [`super::validate_blocks`] call.
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
