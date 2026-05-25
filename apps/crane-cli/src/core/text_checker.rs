//! Text completeness checker — verifies PDF text exists in Markdown.
#![allow(clippy::missing_docs_in_private_items)]

use crate::models::{Criticality, Finding};
use regex::Regex;
use std::sync::OnceLock;

/// Fuzzy similarity threshold for single-word matching.
const FUZZY_THRESHOLD: f64 = 0.85;

/// Compiled whitespace-collapsing regex.
fn ws_pattern() -> &'static Regex {
    static WS: OnceLock<Regex> = OnceLock::new();
    WS.get_or_init(|| Regex::new(r"\s+").expect("static regex"))
}

/// Normalizes text by collapsing whitespace and trimming.
pub fn normalize(text: &str) -> String {
    ws_pattern().replace_all(text.trim(), " ").into_owned()
}

/// Computes normalized Levenshtein similarity between two strings.
///
/// Normalizes and lowercases both inputs before comparison. Returns 1.0 for
/// identical strings.
pub fn compute_similarity(a: &str, b: &str) -> f64 {
    let na = normalize(a).to_lowercase();
    let nb = normalize(b).to_lowercase();
    if na == nb {
        1.0
    } else {
        strsim::normalized_levenshtein(&na, &nb)
    }
}

/// Returns `true` if the segment is present in `md_text`.
///
/// Uses exact substring match as the fast path. For single-word segments, falls
/// back to per-word fuzzy similarity (≥ 0.85) to handle minor OCR variations.
pub fn segment_is_present(segment: &str, md_text: &str) -> bool {
    let norm_seg = normalize(segment).to_lowercase();
    let norm_md = normalize(md_text).to_lowercase();
    match_normalized(&norm_seg, &norm_md)
}

fn match_normalized(norm_seg_lower: &str, norm_md_lower: &str) -> bool {
    if norm_md_lower.contains(norm_seg_lower) {
        return true;
    }
    let seg_words: Vec<&str> = norm_seg_lower.split(' ').collect();
    if seg_words.len() == 1 {
        norm_md_lower
            .split(' ')
            .any(|w| strsim::normalized_levenshtein(norm_seg_lower, w) >= FUZZY_THRESHOLD)
    } else {
        false
    }
}

fn classify_missing(segment: &str) -> Criticality {
    let norm = normalize(segment);
    // Longer missing text (≥ 50 chars) means a substantial section is absent → CRITICAL.
    // Shorter fragments are still concerning but may be headers or labels → HIGH.
    if norm.len() >= 50 {
        Criticality::Critical
    } else {
        Criticality::High
    }
}

/// Checks that all PDF text chunks are present in the Markdown.
///
/// Returns a `Vec<Finding>` for each chunk missing from `md_text`. Empty
/// chunks are skipped.
pub fn check_text(pdf_chunks: &[&str], md_text: &str) -> Vec<Finding> {
    let norm_md = normalize(md_text).to_lowercase();

    pdf_chunks
        .iter()
        .filter(|chunk| !chunk.trim().is_empty())
        .filter_map(|chunk| {
            let norm_seg = normalize(chunk).to_lowercase();
            if match_normalized(&norm_seg, &norm_md) {
                None
            } else {
                let criticality = classify_missing(chunk);
                let truncated = &chunk[..chunk.len().min(50)];
                Some(Finding {
                    category: "text-completeness".to_string(),
                    criticality: criticality.to_string(),
                    confidence: "HIGH".to_string(),
                    location_pdf: None,
                    location_md: None,
                    description: format!("Missing text: {truncated}"),
                    pdf_text: Some((*chunk).to_string()),
                    fix_suggestion: Some("Add the missing section to the Markdown".to_string()),
                    auto_fixable: false,
                })
            }
        })
        .collect()
}
