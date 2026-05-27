//! Heading subcommands for crane-cli.
#![allow(clippy::missing_docs_in_private_items)]

use crate::domain::heading_checker::{check_headings, infer_depth_from_numbering};
use std::io::Write;

/// Runs the `crane heading infer` command, writing `{"depth": N, "confidence": "..."}` to `writer`.
///
/// Returns 0 always (inference always succeeds).
pub fn run_infer_inner(text: &str, writer: &mut dyn Write) -> i32 {
    let json = if let Some((depth, confidence)) = infer_depth_from_numbering(text) {
        serde_json::to_string(&serde_json::json!({
            "depth": depth,
            "confidence": confidence,
        }))
        .unwrap_or_else(|_| r#"{"depth":null,"confidence":"NONE"}"#.to_string())
    } else {
        serde_json::to_string(&serde_json::json!({
            "depth": serde_json::Value::Null,
            "confidence": "NONE",
        }))
        .unwrap_or_else(|_| r#"{"depth":null,"confidence":"NONE"}"#.to_string())
    };
    let _ = writeln!(writer, "{json}");
    0
}

/// Runs the `crane heading infer` command, writing to stdout.
///
/// Returns 0 always.
pub fn run_infer(text: &str) -> i32 {
    run_infer_inner(text, &mut std::io::stdout())
}

/// Runs the `crane heading check` command, writing JSON findings to `writer`.
///
/// Returns 0 if no findings, 1 if findings exist.
pub fn run_check_inner(pdf_text: &str, md_text: &str, writer: &mut dyn Write) -> i32 {
    let findings = check_headings(pdf_text, md_text);
    let json = serde_json::to_string(&findings).unwrap_or_else(|_| "[]".to_string());
    let _ = writeln!(writer, "{json}");
    i32::from(!findings.is_empty())
}

/// Runs the `crane heading check` command, writing to stdout.
///
/// Returns 0 if no findings, 1 if findings exist.
pub fn run_check(pdf_text: &str, md_text: &str) -> i32 {
    run_check_inner(pdf_text, md_text, &mut std::io::stdout())
}
