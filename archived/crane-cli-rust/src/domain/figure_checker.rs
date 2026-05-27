//! Figure coverage checker — verifies figure references are represented in Markdown.
#![allow(clippy::missing_docs_in_private_items)]

use crate::domain::Finding;
use regex::Regex;
use std::sync::OnceLock;

/// Compiled figure reference pattern (case-insensitive).
fn figure_pattern() -> &'static Regex {
    static PAT: OnceLock<Regex> = OnceLock::new();
    PAT.get_or_init(|| Regex::new(r"(?i)(figure|fig\.?)\s*(\d+)").expect("static regex"))
}

/// Compiled Mermaid block pattern.
fn mermaid_pattern() -> &'static Regex {
    static PAT: OnceLock<Regex> = OnceLock::new();
    PAT.get_or_init(|| Regex::new(r"```mermaid").expect("static regex"))
}

/// Compiled figure placeholder pattern.
fn placeholder_pattern() -> &'static Regex {
    static PAT: OnceLock<Regex> = OnceLock::new();
    PAT.get_or_init(|| Regex::new(r"(?i)\[FIGURE\s*\d+").expect("static regex"))
}

/// A detected figure reference in PDF text.
#[derive(Debug, Clone, PartialEq)]
pub struct FigureRef {
    /// Full label text (e.g. "Figure 1").
    pub label: String,
    /// Figure number as a string (e.g. "1").
    pub number: String,
}

/// Detects figure references in text using regex.
pub fn detect_figures(text: &str) -> Vec<FigureRef> {
    figure_pattern()
        .captures_iter(text)
        .map(|cap| FigureRef {
            label: cap[0].to_string(),
            number: cap[2].to_string(),
        })
        .collect()
}

fn figure_is_covered(figure_num: &str, md_text: &str) -> bool {
    let has_mermaid = mermaid_pattern().is_match(md_text);
    let has_placeholder = placeholder_pattern().is_match(md_text)
        && md_text.to_lowercase().contains(&figure_num.to_lowercase());
    let has_fig_label = figure_pattern().is_match(md_text)
        && md_text.to_lowercase().contains(&figure_num.to_lowercase());
    has_mermaid || has_placeholder || has_fig_label
}

/// Checks that all PDF figure references have a representation in Markdown.
///
/// A figure is considered covered if the Markdown contains a Mermaid block, a
/// `[FIGURE N: ...]` placeholder, or a direct figure label reference.
pub fn check_figures(pdf_text: &str, md_text: &str) -> Vec<Finding> {
    detect_figures(pdf_text)
        .into_iter()
        .filter_map(|fig| {
            if figure_is_covered(&fig.number, md_text) {
                None
            } else {
                Some(Finding {
                    category: "figure-coverage".to_string(),
                    criticality: "HIGH".to_string(),
                    confidence: "HIGH".to_string(),
                    location_pdf: Some(fig.label.clone()),
                    location_md: None,
                    description: format!("Figure {} has no representation in Markdown", fig.number),
                    pdf_text: Some(fig.label.clone()),
                    fix_suggestion: Some(format!(
                        "Add Mermaid block or [FIGURE {}: description] placeholder",
                        fig.number
                    )),
                    auto_fixable: false,
                })
            }
        })
        .collect()
}
