//! Report management subcommands for crane-cli.
#![allow(clippy::missing_docs_in_private_items)]

use crate::core::report_manager::{finalize_report, init_report};
use std::io::Write;

/// Runs the `crane report init` command, writing `{"path": "..."}` JSON to `writer`.
///
/// Returns 0 on success, 1 on error.
pub fn run_init_inner(scope: &str, pdf: &str, md: &str, writer: &mut dyn Write) -> i32 {
    match init_report(scope, pdf, md) {
        Ok(path) => {
            let json = serde_json::to_string(&serde_json::json!({"path": path}))
                .unwrap_or_else(|_| r#"{"path":""}"#.to_string());
            let _ = writeln!(writer, "{json}");
            0
        }
        Err(e) => {
            eprintln!("Error: {e}");
            1
        }
    }
}

/// Runs the `crane report init` command, writing to stdout.
///
/// Returns 0 on success, 1 on error.
pub fn run_init(scope: &str, pdf: &str, md: &str) -> i32 {
    run_init_inner(scope, pdf, md, &mut std::io::stdout())
}

/// Runs the `crane report finalize` command, writing `{"status": "...", "path": "..."}` to `writer`.
///
/// Returns 0 on success, 1 on error.
pub fn run_finalize_inner(report_path: &str, status: &str, writer: &mut dyn Write) -> i32 {
    match finalize_report(report_path, status) {
        Ok(()) => {
            let json = serde_json::to_string(&serde_json::json!({
                "status": status,
                "path": report_path,
            }))
            .unwrap_or_else(|_| r#"{"status":"","path":""}"#.to_string());
            let _ = writeln!(writer, "{json}");
            0
        }
        Err(e) => {
            eprintln!("Error: {e}");
            1
        }
    }
}

/// Runs the `crane report finalize` command, writing to stdout.
///
/// Returns 0 on success, 1 on error.
pub fn run_finalize(report_path: &str, status: &str) -> i32 {
    run_finalize_inner(report_path, status, &mut std::io::stdout())
}
