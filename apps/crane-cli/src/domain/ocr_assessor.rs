//! OCR quality assessor — estimates OCR error rate in Markdown sections.
#![allow(clippy::missing_docs_in_private_items)]

use crate::domain::Finding;
use regex::Regex;
use std::sync::OnceLock;

/// Returns the compiled OCR error patterns.
fn ocr_error_patterns() -> &'static [Regex; 4] {
    static PATTERNS: OnceLock<[Regex; 4]> = OnceLock::new();
    PATTERNS.get_or_init(|| {
        [
            Regex::new(r"[^\x00-\x7F]{3,}").expect("static regex"),
            Regex::new(r"\b[lI1]{5,}\b").expect("static regex"),
            Regex::new(r"\b[0Oo]{5,}\b").expect("static regex"),
            Regex::new(r"[a-zA-Z]{30,}").expect("static regex"),
        ]
    })
}

/// Compiled OCR section tag pattern.
fn ocr_tag_pattern() -> &'static Regex {
    static PAT: OnceLock<Regex> = OnceLock::new();
    PAT.get_or_init(|| Regex::new(r"(?s)<!--\s*OCR:\s*(.*?)\s*-->").expect("static regex"))
}

/// An OCR-tagged section extracted from Markdown.
#[derive(Debug, Clone, PartialEq)]
pub struct OcrSection {
    /// Tag type (always "ocr-comment").
    pub tag: String,
    /// Content of the OCR section.
    pub content: String,
}

/// Estimates the OCR error rate for a text string.
///
/// Uses four heuristic patterns: non-ASCII runs, `lI1` runs, `0Oo` runs, and
/// long alpha runs. Returns a rate in `[0.0, 1.0]`.
pub fn estimate_ocr_error_rate(text: &str) -> f64 {
    let clean: String = text.chars().filter(|&c| c != ' ' && c != '\n').collect();
    let total = clean.len();
    if total == 0 {
        return 0.0;
    }

    let error_chars: usize = ocr_error_patterns()
        .iter()
        .map(|pat| pat.find_iter(text).map(|m| m.as_str().len()).sum::<usize>())
        .sum();

    (error_chars as f64 / total as f64).min(1.0)
}

/// Extracts OCR-tagged sections from Markdown (`<!-- OCR: ... -->`).
pub fn extract_ocr_sections(md_text: &str) -> Vec<OcrSection> {
    ocr_tag_pattern()
        .captures_iter(md_text)
        .map(|cap| OcrSection {
            tag: "ocr-comment".to_string(),
            content: cap[1].to_string(),
        })
        .collect()
}

/// Checks OCR quality in Markdown OCR-tagged sections.
///
/// Returns findings for sections exceeding error rate thresholds:
/// - `>10%` → CRITICAL
/// - `>5%` → HIGH
/// - `>2%` → MEDIUM
pub fn check_ocr_quality(md_text: &str) -> Vec<Finding> {
    let sections = extract_ocr_sections(md_text);
    if sections.is_empty() {
        return vec![];
    }

    sections
        .iter()
        .filter_map(|section| {
            let rate = estimate_ocr_error_rate(&section.content);

            if rate > 0.10 {
                Some(Finding {
                    category: "ocr-quality".to_string(),
                    criticality: "CRITICAL".to_string(),
                    confidence: "HIGH".to_string(),
                    location_pdf: None,
                    location_md: Some(section.tag.clone()),
                    description: format!(
                        "OCR error rate {:.1}% exceeds 10% threshold",
                        rate * 100.0
                    ),
                    pdf_text: None,
                    fix_suggestion: Some("Manual review of OCR section required".to_string()),
                    auto_fixable: false,
                })
            } else if rate > 0.05 {
                Some(Finding {
                    category: "ocr-quality".to_string(),
                    criticality: "HIGH".to_string(),
                    confidence: "HIGH".to_string(),
                    location_pdf: None,
                    location_md: Some(section.tag.clone()),
                    description: format!(
                        "OCR error rate {:.1}% exceeds 5% threshold",
                        rate * 100.0
                    ),
                    pdf_text: None,
                    fix_suggestion: Some("Review OCR section for errors".to_string()),
                    auto_fixable: false,
                })
            } else if rate > 0.02 {
                Some(Finding {
                    category: "ocr-quality".to_string(),
                    criticality: "MEDIUM".to_string(),
                    confidence: "MEDIUM".to_string(),
                    location_pdf: None,
                    location_md: Some(section.tag.clone()),
                    description: format!(
                        "OCR error rate {:.1}% exceeds 2% threshold",
                        rate * 100.0
                    ),
                    pdf_text: None,
                    fix_suggestion: Some("Minor OCR cleanup may be needed".to_string()),
                    auto_fixable: false,
                })
            } else {
                None
            }
        })
        .collect()
}
