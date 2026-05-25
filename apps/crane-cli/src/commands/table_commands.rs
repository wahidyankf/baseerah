//! Table subcommands for crane-cli.
#![allow(clippy::missing_docs_in_private_items)]

use crate::core::table_checker::{check_tables, detect_tables};
use std::io::Write;

/// Runs the `crane table detect` command, writing JSON table specs to `writer`.
///
/// Returns 0 always.
pub fn run_detect_inner(text: &str, writer: &mut dyn Write) -> i32 {
    let tables = detect_tables(text);
    let serializable: Vec<serde_json::Value> = tables
        .iter()
        .map(|t| {
            serde_json::json!({
                "row_count": t.row_count,
                "col_count": t.col_count,
                "header_row": t.header_row,
            })
        })
        .collect();
    let json = serde_json::to_string(&serializable).unwrap_or_else(|_| "[]".to_string());
    let _ = writeln!(writer, "{json}");
    0
}

/// Runs the `crane table detect` command, writing to stdout.
///
/// Returns 0 always.
pub fn run_detect(text: &str) -> i32 {
    run_detect_inner(text, &mut std::io::stdout())
}

/// Runs the `crane table check` command, writing JSON findings to `writer`.
///
/// Returns 0 if no findings, 1 if findings exist.
pub fn run_check_inner(pdf_text: &str, md_text: &str, writer: &mut dyn Write) -> i32 {
    let findings = check_tables(pdf_text, md_text);
    let json = serde_json::to_string(&findings).unwrap_or_else(|_| "[]".to_string());
    let _ = writeln!(writer, "{json}");
    i32::from(!findings.is_empty())
}

/// Runs the `crane table check` command, writing to stdout.
///
/// Returns 0 if no findings, 1 if findings exist.
pub fn run_check(pdf_text: &str, md_text: &str) -> i32 {
    run_check_inner(pdf_text, md_text, &mut std::io::stdout())
}
