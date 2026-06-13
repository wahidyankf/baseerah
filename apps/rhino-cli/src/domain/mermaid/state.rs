//! State-diagram parser stub — behavior lands in Phase 8.

use super::types::{Direction, MermaidBlock, ParsedDiagram};

/// Parses a `stateDiagram-v2` / `stateDiagram` block.
///
/// Returns an empty [`ParsedDiagram`] — full parsing is implemented in Phase 8.
pub fn parse_state(block: MermaidBlock) -> ParsedDiagram {
    ParsedDiagram {
        block,
        direction: Direction::TB,
        nodes: Vec::new(),
        edges: Vec::new(),
        subgraphs: Vec::new(),
    }
}
