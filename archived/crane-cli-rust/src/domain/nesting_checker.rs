//! List nesting depth checker — verifies nesting levels match PDF layout.
#![allow(clippy::missing_docs_in_private_items)]

use crate::domain::Finding;

/// A list nesting item extracted from text.
#[derive(Debug, Clone, PartialEq)]
pub struct NestingItem {
    /// Nesting level (1-indexed, 1 = top level).
    pub level: usize,
    /// Text of the list item.
    pub text: String,
}

/// Extracts nesting levels from layout text (lines starting with `-`, `*`, `•`).
///
/// Level is computed as `indent / 2 + 1` where indent is the number of leading
/// spaces before the bullet character.
pub fn extract_nesting_levels(layout_text: &str) -> Vec<NestingItem> {
    layout_text
        .split('\n')
        .filter_map(|line| {
            let trimmed = line.trim_start();
            if trimmed.starts_with('-') || trimmed.starts_with('*') || trimmed.starts_with('•') {
                let indent = line.len() - trimmed.len();
                let level = indent / 2 + 1;
                let text = trimmed
                    .trim_start_matches(['-', '*', '•', ' '])
                    .trim()
                    .to_string();
                Some(NestingItem { level, text })
            } else {
                None
            }
        })
        .collect()
}

/// Checks nesting depth consistency between PDF layout text and Markdown.
///
/// Returns findings for items where the nesting level differs between PDF and
/// Markdown. Returns `None` for items not found in Markdown.
pub fn check_nesting(pdf_layout_text: &str, md_text: &str) -> Vec<Finding> {
    let pdf_items = extract_nesting_levels(pdf_layout_text);
    let md_items = extract_nesting_levels(md_text);

    pdf_items
        .iter()
        .filter_map(|pdf_item| {
            let md_match = md_items.iter().find(|m| {
                m.text
                    .to_lowercase()
                    .contains(&pdf_item.text.to_lowercase())
                    || pdf_item
                        .text
                        .to_lowercase()
                        .contains(&m.text.to_lowercase())
            });

            if let Some(md_item) = md_match.filter(|m| m.level != pdf_item.level) {
                let is_inverted = md_item.level < pdf_item.level;
                let criticality = if is_inverted { "HIGH" } else { "MEDIUM" };
                Some(Finding {
                    category: "content-nesting".to_string(),
                    criticality: criticality.to_string(),
                    confidence: "MEDIUM".to_string(),
                    location_pdf: None,
                    location_md: None,
                    description: format!(
                        "Nesting mismatch: PDF level {}, MD level {} for '{}'",
                        pdf_item.level, md_item.level, pdf_item.text
                    ),
                    pdf_text: Some(pdf_item.text.clone()),
                    fix_suggestion: Some(format!("Adjust nesting to level {}", pdf_item.level)),
                    auto_fixable: false,
                })
            } else {
                None
            }
        })
        .collect()
}
