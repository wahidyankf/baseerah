//! `harness emit-bindings` — writes the Amazon Q Developer bridge files
// deterministically (idempotent overwrite). See
// `crate::internal::agents::bindings` for the canonical content source.

use std::fmt::Write as _;

use anyhow::{Error, anyhow};
use clap::Args;

use crate::domain::cliout::OutputFormat;
use crate::internal::agents::bindings::{EmitResult, emit_bindings};
use crate::internal::git;

/// CLI arguments for `agents emit-bindings`.
#[derive(Args, Debug)]
pub struct EmitBindingsArgs {
    /// Suppress the per-file listing in text output.
    #[arg(long, short = 'q')]
    pub quiet: bool,
}

/// Run the `agents emit-bindings` command.
///
/// # Errors
///
/// Returns an error if the git repository root cannot be found or if the
/// binding emission fails.
pub fn run(args: &EmitBindingsArgs, output_format: OutputFormat) -> std::result::Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    let result = emit_bindings(&repo_root).map_err(|e| anyhow!("failed to emit bindings: {e}"))?;

    match output_format {
        OutputFormat::Text => print!("{}", format_text(&result, args.quiet)),
        OutputFormat::Json => println!("{}", format_json(&result)?),
        OutputFormat::Markdown => print!("{}", format_markdown(&result)),
    }
    Ok(())
}

/// Format the emit result as human-readable text.
fn format_text(result: &EmitResult, quiet: bool) -> String {
    let mut s = String::new();
    if !quiet {
        for path in &result.written {
            let _ = writeln!(s, "wrote {path}");
        }
    }
    let _ = writeln!(
        s,
        "\u{2713} emit-bindings wrote {} file(s)",
        result.written.len()
    );
    s
}

/// Format the emit result as Markdown.
fn format_markdown(result: &EmitResult) -> String {
    let mut s = String::from("# Amazon Q Bindings Emit\n\n");
    for path in &result.written {
        let _ = writeln!(s, "- `{path}`");
    }
    let _ = writeln!(s, "\nWrote {} file(s).", result.written.len());
    s
}

/// Serialize the emit result as a JSON string.
///
/// # Errors
///
/// Returns an error if JSON serialization fails.
fn format_json(result: &EmitResult) -> std::result::Result<String, Error> {
    #[derive(serde::Serialize)]
    struct Out<'a> {
        /// `"success"` always.
        status: &'a str,
        /// Paths of files written during the emit.
        written: &'a [String],
        /// Number of files written.
        count: usize,
    }
    let out = Out {
        status: "success",
        written: &result.written,
        count: result.written.len(),
    };
    Ok(serde_json::to_string_pretty(&out)?)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn args_default() {
        let a = EmitBindingsArgs { quiet: false };
        assert!(!a.quiet);
    }

    #[test]
    fn format_text_lists_files() {
        let r = EmitResult {
            written: vec![".amazonq/rules/00-agents-md.md".to_string()],
            ..Default::default()
        };
        let s = format_text(&r, false);
        assert!(s.contains("wrote .amazonq/rules/00-agents-md.md"));
        assert!(s.contains("wrote 1 file(s)"));
    }

    #[test]
    fn format_text_quiet_hides_listing() {
        let r = EmitResult {
            written: vec![".amazonq/rules/00-agents-md.md".to_string()],
            ..Default::default()
        };
        let s = format_text(&r, true);
        assert!(!s.contains("wrote .amazonq"));
        assert!(s.contains("wrote 1 file(s)"));
    }

    #[test]
    fn format_json_is_valid() {
        let r = EmitResult {
            written: vec!["a".to_string(), "b".to_string()],
            ..Default::default()
        };
        let json = format_json(&r).unwrap();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(v["count"], 2);
        assert_eq!(v["status"], "success");
    }
}
