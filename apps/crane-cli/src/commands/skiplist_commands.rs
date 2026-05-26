//! Skip list management subcommands for crane-cli.
#![allow(clippy::missing_docs_in_private_items)]

use crate::domain::skiplist_manager::{add, check, list};
use std::io::Write;

/// Runs the `crane skiplist add` command, writing `{"added": bool}` JSON to `writer`.
///
/// Returns 0 on success, 1 on error (fixes the F# bug where error returned 0).
pub fn run_add_inner(
    md_basename: &str,
    category: &str,
    description: &str,
    writer: &mut dyn Write,
) -> i32 {
    match add(md_basename, category, description) {
        Ok(added) => {
            let json = serde_json::to_string(&serde_json::json!({"added": added}))
                .unwrap_or_else(|_| r#"{"added":false}"#.to_string());
            let _ = writeln!(writer, "{json}");
            0
        }
        Err(e) => {
            eprintln!("Error: {e}");
            1
        }
    }
}

/// Runs the `crane skiplist add` command, writing to stdout.
///
/// Returns 0 on success, 1 on error.
pub fn run_add(md_basename: &str, category: &str, description: &str) -> i32 {
    run_add_inner(md_basename, category, description, &mut std::io::stdout())
}

/// Runs the `crane skiplist check` command, writing `{"match": bool}` JSON to `writer`.
///
/// Returns 0 if match found, 1 if not found or on error.
pub fn run_check_inner(
    md_basename: &str,
    category: &str,
    description: &str,
    writer: &mut dyn Write,
) -> i32 {
    match check(md_basename, category, description) {
        Ok(found) => {
            let json = serde_json::to_string(&serde_json::json!({"match": found}))
                .unwrap_or_else(|_| r#"{"match":false}"#.to_string());
            let _ = writeln!(writer, "{json}");
            i32::from(!found)
        }
        Err(e) => {
            eprintln!("Error: {e}");
            1
        }
    }
}

/// Runs the `crane skiplist check` command, writing to stdout.
///
/// Returns 0 if match found, 1 if not found or on error.
pub fn run_check(md_basename: &str, category: &str, description: &str) -> i32 {
    run_check_inner(md_basename, category, description, &mut std::io::stdout())
}

/// Runs the `crane skiplist list` command, writing JSON array of entries to `writer`.
///
/// Returns 0 always.
pub fn run_list_inner(md_basename: &str, writer: &mut dyn Write) -> i32 {
    match list(md_basename) {
        Ok(entries) => {
            let json = serde_json::to_string(&entries).unwrap_or_else(|_| "[]".to_string());
            let _ = writeln!(writer, "{json}");
            0
        }
        Err(e) => {
            eprintln!("Error: {e}");
            1
        }
    }
}

/// Runs the `crane skiplist list` command, writing to stdout.
///
/// Returns 0 always.
pub fn run_list(md_basename: &str) -> i32 {
    run_list_inner(md_basename, &mut std::io::stdout())
}
