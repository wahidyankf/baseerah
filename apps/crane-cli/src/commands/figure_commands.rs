//! Figure subcommands for crane-cli.
#![allow(clippy::missing_docs_in_private_items)]

use crate::core::figure_checker::{check_figures, detect_figures};
use std::io::Write;

/// Runs the `crane figure detect` command, writing JSON figure refs to `writer`.
///
/// Returns 0 always.
pub fn run_detect_inner(text: &str, writer: &mut dyn Write) -> i32 {
    let figures = detect_figures(text);
    let serializable: Vec<serde_json::Value> = figures
        .iter()
        .map(|f| {
            serde_json::json!({
                "label": f.label,
                "number": f.number,
            })
        })
        .collect();
    let json = serde_json::to_string(&serializable).unwrap_or_else(|_| "[]".to_string());
    let _ = writeln!(writer, "{json}");
    0
}

/// Runs the `crane figure detect` command, writing to stdout.
///
/// Returns 0 always.
pub fn run_detect(text: &str) -> i32 {
    run_detect_inner(text, &mut std::io::stdout())
}

/// Runs the `crane figure check` command, writing JSON findings to `writer`.
///
/// Returns 0 if no findings, 1 if findings exist.
pub fn run_check_inner(pdf_text: &str, md_text: &str, writer: &mut dyn Write) -> i32 {
    let findings = check_figures(pdf_text, md_text);
    let json = serde_json::to_string(&findings).unwrap_or_else(|_| "[]".to_string());
    let _ = writeln!(writer, "{json}");
    i32::from(!findings.is_empty())
}

/// Runs the `crane figure check` command, writing to stdout.
///
/// Returns 0 if no findings, 1 if findings exist.
pub fn run_check(pdf_text: &str, md_text: &str) -> i32 {
    run_check_inner(pdf_text, md_text, &mut std::io::stdout())
}
