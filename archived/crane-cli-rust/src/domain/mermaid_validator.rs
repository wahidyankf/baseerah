//! Mermaid diagram syntax validator — validates Mermaid blocks in Markdown.
#![allow(clippy::missing_docs_in_private_items)]

use crate::domain::Finding;
use std::collections::HashSet;
use std::sync::OnceLock;

/// Returns the set of valid Mermaid diagram type keywords.
fn valid_types() -> &'static HashSet<&'static str> {
    static TYPES: OnceLock<HashSet<&'static str>> = OnceLock::new();
    TYPES.get_or_init(|| {
        [
            "graph",
            "flowchart",
            "sequenceDiagram",
            "stateDiagram",
            "stateDiagram-v2",
            "classDiagram",
            "gantt",
            "pie",
            "erDiagram",
            "journey",
            "gitGraph",
            "mindmap",
            "timeline",
            "quadrantChart",
            "xychart-beta",
            "sankey-beta",
            "block-beta",
            "architecture-beta",
        ]
        .iter()
        .copied()
        .collect()
    })
}

/// A Mermaid block extracted from Markdown.
#[derive(Debug, Clone, PartialEq)]
pub struct MermaidBlock {
    /// Line number where the block content starts (1-indexed).
    pub line_number: usize,
    /// Raw content of the block (without the ` ```mermaid ` fences).
    pub content: String,
}

/// Validates a Mermaid block's content.
///
/// Checks that the diagram type is known and that brackets and parentheses are
/// balanced.
///
/// # Errors
///
/// Returns `Err(String)` with a description of the validation error.
pub fn validate_block(content: &str) -> Result<(), String> {
    let trimmed = content.trim();
    let first_line = trimmed.lines().next().unwrap_or("").trim();

    if first_line.is_empty() {
        return Err("empty Mermaid block".to_string());
    }

    let diagram_type = first_line.split_whitespace().next().unwrap_or("");

    if !valid_types().contains(diagram_type) {
        return Err(format!("unknown diagram type: {diagram_type}"));
    }

    let open_brackets = content.chars().filter(|&c| c == '[').count();
    let close_brackets = content.chars().filter(|&c| c == ']').count();
    if open_brackets != close_brackets {
        return Err("unmatched brackets".to_string());
    }

    let open_parens = content.chars().filter(|&c| c == '(').count();
    let close_parens = content.chars().filter(|&c| c == ')').count();
    if open_parens != close_parens {
        return Err("unmatched parentheses".to_string());
    }

    Ok(())
}

/// Extracts all Mermaid blocks from Markdown text.
pub fn extract_blocks(md_text: &str) -> Vec<MermaidBlock> {
    let lines: Vec<&str> = md_text.split('\n').collect();
    let mut blocks = Vec::new();
    let mut in_block = false;
    let mut block_start = 0;
    let mut block_content = String::new();

    for (i, line) in lines.iter().enumerate() {
        if !in_block && line.trim() == "```mermaid" {
            in_block = true;
            block_start = i + 1;
            block_content = String::new();
        } else if in_block && line.trim() == "```" {
            blocks.push(MermaidBlock {
                line_number: block_start,
                content: block_content.clone(),
            });
            in_block = false;
        } else if in_block {
            block_content.push_str(line);
            block_content.push('\n');
        }
    }

    blocks
}

/// Validates all Mermaid blocks in a Markdown document.
///
/// Returns findings for blocks that fail validation.
pub fn validate_md(md_text: &str) -> Vec<Finding> {
    extract_blocks(md_text)
        .into_iter()
        .filter_map(|block| match validate_block(&block.content) {
            Ok(()) => None,
            Err(msg) => Some(Finding {
                category: "mermaid-syntax".to_string(),
                criticality: "HIGH".to_string(),
                confidence: "HIGH".to_string(),
                location_pdf: None,
                location_md: Some(format!("line {}", block.line_number)),
                description: msg,
                pdf_text: None,
                fix_suggestion: Some("Fix Mermaid diagram syntax".to_string()),
                auto_fixable: false,
            }),
        })
        .collect()
}
