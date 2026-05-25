//! Mermaid validation subcommands for crane-cli.
#![allow(clippy::missing_docs_in_private_items)]

use crate::core::mermaid_validator::validate_md;
use std::io::Write;

/// Runs the `crane mermaid validate` command, writing JSON findings to `writer`.
///
/// Returns 0 if no findings, 1 if findings exist.
pub fn run_validate_inner(md_text: &str, writer: &mut dyn Write) -> i32 {
    let findings = validate_md(md_text);
    let json = serde_json::to_string(&findings).unwrap_or_else(|_| "[]".to_string());
    let _ = writeln!(writer, "{json}");
    i32::from(!findings.is_empty())
}

/// Runs the `crane mermaid validate` command, writing to stdout.
///
/// Returns 0 if no findings, 1 if findings exist.
pub fn run_validate(md_text: &str) -> i32 {
    run_validate_inner(md_text, &mut std::io::stdout())
}
