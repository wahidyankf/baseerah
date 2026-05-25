//! Text completeness subcommands for crane-cli.
#![allow(clippy::missing_docs_in_private_items)]

use crate::adapters::PdfAdapter;
use crate::core::text_checker::{check_text, compute_similarity, segment_is_present};
use std::io::Write;

/// Runs the `crane text check` command, writing JSON findings to `writer`.
///
/// Returns 0 if no findings, 1 if findings exist or on error.
pub fn run_check_inner(
    adapter: &dyn PdfAdapter,
    pdf: &str,
    md_text: &str,
    writer: &mut dyn Write,
) -> i32 {
    match adapter.sample_text(pdf, 999) {
        Ok(pdf_text) => {
            let chunks: Vec<&str> = pdf_text
                .split('\n')
                .filter(|s| s.trim().len() > 10)
                .collect();
            let findings = check_text(&chunks, md_text);
            let json = serde_json::to_string(&findings).unwrap_or_else(|_| "[]".to_string());
            let _ = writeln!(writer, "{json}");
            i32::from(!findings.is_empty())
        }
        Err(msg) => {
            eprintln!("Error: {msg}");
            1
        }
    }
}

/// Runs the `crane text check` command, writing JSON findings to stdout.
///
/// Returns 0 if no findings, 1 if findings exist or on error.
pub fn run_check(adapter: &dyn PdfAdapter, pdf: &str, md_text: &str) -> i32 {
    run_check_inner(adapter, pdf, md_text, &mut std::io::stdout())
}

/// Runs the `crane text search` command, writing `{"found": bool, "similarity": f64}` to `writer`.
///
/// Returns 0 if found, 1 if not found.
pub fn run_search_inner(md_text: &str, segment: &str, writer: &mut dyn Write) -> i32 {
    let found = segment_is_present(segment, md_text);
    let similarity = compute_similarity(segment, md_text);
    let json = serde_json::to_string(&serde_json::json!({
        "found": found,
        "similarity": similarity,
    }))
    .unwrap_or_else(|_| r#"{"found":false,"similarity":0.0}"#.to_string());
    let _ = writeln!(writer, "{json}");
    i32::from(!found)
}

/// Runs the `crane text search` command, writing to stdout.
///
/// Returns 0 if found, 1 if not found.
pub fn run_search(md_text: &str, segment: &str) -> i32 {
    run_search_inner(md_text, segment, &mut std::io::stdout())
}
