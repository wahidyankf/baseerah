//! Heading depth accuracy checker — verifies heading levels match PDF numbering.
#![allow(clippy::missing_docs_in_private_items)]

use crate::models::Finding;
use regex::Regex;
use std::sync::OnceLock;

/// Compiled section-number pattern (e.g. "1.", "2.3", "3.1.2 ").
fn section_num_pattern() -> &'static Regex {
    static PAT: OnceLock<Regex> = OnceLock::new();
    PAT.get_or_init(|| Regex::new(r"^(\d+|\w)(\.\d+|\.\w)*\.?\s").expect("static regex"))
}

/// A heading entry extracted from Markdown.
#[derive(Debug, Clone, PartialEq)]
pub struct HeadingEntry {
    /// Heading depth (1–6, corresponding to # through ######).
    pub depth: usize,
    /// Heading text without the `#` prefix.
    pub text: String,
}

/// Infers heading depth from a section number prefix.
///
/// Returns `Some((depth, confidence))` where `confidence` is always `"HIGH"`,
/// or `None` if the line does not start with a recognizable section number.
pub fn infer_depth_from_numbering(heading: &str) -> Option<(usize, &'static str)> {
    let heading = heading.trim();
    let m = section_num_pattern().find(heading)?;
    let num_part = m.as_str().trim_end_matches([' ', '\t']);
    let dots = num_part.chars().filter(|&c| c == '.').count();
    let depth = if num_part.ends_with('.') {
        dots + 1
    } else {
        dots + 2
    };
    Some((depth.min(5), "HIGH"))
}

/// Extracts `#`-style headings from Markdown text.
pub fn extract_md_headings(md_text: &str) -> Vec<HeadingEntry> {
    md_text
        .split('\n')
        .filter_map(|line| {
            let trimmed = line.trim_start();
            if trimmed.starts_with('#') {
                let depth = trimmed.chars().take_while(|&c| c == '#').count();
                let text = trimmed.trim_start_matches('#').trim().to_string();
                Some(HeadingEntry { depth, text })
            } else {
                None
            }
        })
        .collect()
}

/// Checks heading depths between PDF layout text and Markdown.
///
/// Returns findings for headings that have a mismatched depth. Lines that don't
/// match a section number pattern are ignored.
pub fn check_headings(pdf_layout_text: &str, md_text: &str) -> Vec<Finding> {
    let md_headings = extract_md_headings(md_text);

    pdf_layout_text
        .split('\n')
        .filter_map(|line| {
            let (expected_depth, _) = infer_depth_from_numbering(line)?;
            let heading_text = section_num_pattern()
                .replace(line.trim(), "")
                .trim()
                .to_string();

            let md_match = md_headings.iter().find(|h| {
                h.text.to_lowercase().contains(&heading_text.to_lowercase())
                    || heading_text.to_lowercase().contains(&h.text.to_lowercase())
            });

            if let Some(md_h) = md_match.filter(|h| h.depth != expected_depth) {
                let diff = md_h.depth.abs_diff(expected_depth);
                let criticality = if diff >= 1 { "HIGH" } else { "MEDIUM" };
                Some(Finding {
                    category: "heading-depth".to_string(),
                    criticality: criticality.to_string(),
                    confidence: "HIGH".to_string(),
                    location_pdf: Some(line.to_string()),
                    location_md: Some(format!("H{}: {}", md_h.depth, md_h.text)),
                    description: format!(
                        "Expected H{expected_depth}, found H{} for '{heading_text}'",
                        md_h.depth
                    ),
                    pdf_text: Some(line.to_string()),
                    fix_suggestion: Some(format!("Change heading to H{expected_depth}")),
                    auto_fixable: false,
                })
            } else {
                None
            }
        })
        .collect()
}
