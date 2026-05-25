//! Nesting subcommands for crane-cli.
#![allow(clippy::missing_docs_in_private_items)]

use crate::core::nesting_checker::{check_nesting, extract_nesting_levels};
use std::io::Write;

/// Runs the `crane nesting infer` command, writing JSON nesting items to `writer`.
///
/// Returns 0 always.
pub fn run_infer_inner(text: &str, writer: &mut dyn Write) -> i32 {
    let items = extract_nesting_levels(text);
    let serializable: Vec<serde_json::Value> = items
        .iter()
        .map(|i| {
            serde_json::json!({
                "level": i.level,
                "text": i.text,
            })
        })
        .collect();
    let json = serde_json::to_string(&serializable).unwrap_or_else(|_| "[]".to_string());
    let _ = writeln!(writer, "{json}");
    0
}

/// Runs the `crane nesting infer` command, writing to stdout.
///
/// Returns 0 always.
pub fn run_infer(text: &str) -> i32 {
    run_infer_inner(text, &mut std::io::stdout())
}

/// Runs the `crane nesting check` command, writing JSON findings to `writer`.
///
/// Returns 0 if no findings, 1 if findings exist.
pub fn run_check_inner(pdf_text: &str, md_text: &str, writer: &mut dyn Write) -> i32 {
    let findings = check_nesting(pdf_text, md_text);
    let json = serde_json::to_string(&findings).unwrap_or_else(|_| "[]".to_string());
    let _ = writeln!(writer, "{json}");
    i32::from(!findings.is_empty())
}

/// Runs the `crane nesting check` command, writing to stdout.
///
/// Returns 0 if no findings, 1 if findings exist.
pub fn run_check(pdf_text: &str, md_text: &str) -> i32 {
    run_check_inner(pdf_text, md_text, &mut std::io::stdout())
}
