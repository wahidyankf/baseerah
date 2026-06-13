//! Mermaid validate use case: applies domain rules to extracted blocks.

use crate::domain::mermaid::{MermaidBlock, ValidateOptions, ValidationResult, validate_blocks};

/// Validates extracted Mermaid blocks against the given options.
pub fn validate(blocks: Vec<MermaidBlock>, opts: ValidateOptions) -> ValidationResult {
    validate_blocks(blocks, opts)
}
