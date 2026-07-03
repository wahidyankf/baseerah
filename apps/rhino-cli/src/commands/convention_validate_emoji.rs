//! `convention emoji` — detects emoji codepoints in forbidden file types.
//!
//! Port of `apps/rhino-cli/cmd/governance_emoji_audit.go`.

use std::fmt::Write as _;
use std::path::Path;

use anyhow::{Context, Error, anyhow};
use clap::Args;
use serde::Serialize;

use crate::domain::cliout::OutputFormat;
use crate::infrastructure::fs::real::RealFs;
use crate::internal::git;
use crate::internal::repo_governance::emoji_audit::{EmojiFinding, audit_emoji};

/// JSON output schema identifier for this command.
const SCHEMA: &str = "rhino-cli/emoji-audit/v1";

/// CLI arguments for `repo-governance emoji-audit`.
#[derive(Args, Debug)]
pub struct EmojiAuditArgs {
    /// Paths to scan (repeatable; relative to git root).
    #[arg(short = 'p', long = "path", value_name = "PATH")]
    pub path: Vec<String>,
    /// Positional path overrides — same effect as --path.
    pub positional: Vec<String>,
}

/// Single emoji finding in JSON output.
#[derive(Serialize)]
struct FindingJson<'a> {
    /// Path of the file containing the emoji.
    file: &'a str,
    /// Line number.
    line: usize,
    /// Column number.
    column: usize,
    /// Unicode codepoint string (e.g. `"U+2713"`).
    codepoint: &'a str,
    /// Severity label.
    severity: &'a str,
}

/// JSON envelope wrapping the emoji audit result.
#[derive(Serialize)]
struct Envelope<'a> {
    /// Output schema identifier.
    schema: &'a str,
    /// `"passed"` or `"failed"`.
    status: &'a str,
    /// Individual findings.
    result: Vec<FindingJson<'a>>,
}

/// Run the `repo-governance emoji-audit` command.
///
/// # Errors
///
/// Returns an error if the git root cannot be found, the audit fails, or
/// emoji findings are detected.
pub fn run(args: &EmojiAuditArgs, output_format: OutputFormat) -> std::result::Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;

    let rel_paths: Vec<String> = if !args.positional.is_empty() {
        args.positional.clone()
    } else if !args.path.is_empty() {
        args.path.clone()
    } else {
        vec![".".to_string()]
    };

    let full_paths: Vec<String> = rel_paths
        .iter()
        .map(|p| {
            if Path::new(p).is_absolute() {
                p.clone()
            } else {
                repo_root.join(p).to_string_lossy().to_string()
            }
        })
        .collect();

    let findings = audit_emoji(&RealFs, &full_paths).context("emoji audit failed")?;

    match output_format {
        OutputFormat::Text => print!("{}", format_text(&findings)),
        OutputFormat::Json => print!("{}", format_json(&findings)?),
        OutputFormat::Markdown => print!("{}", format_markdown(&findings)),
    }

    if !findings.is_empty() {
        return Err(anyhow!("{} emoji finding(s) found", findings.len()));
    }
    Ok(())
}

/// Format emoji findings as human-readable text.
fn format_text(findings: &[EmojiFinding]) -> String {
    if findings.is_empty() {
        return "EMOJI AUDIT PASSED: no emoji codepoints found in forbidden file types\n"
            .to_string();
    }
    let mut sb = String::new();
    let _ = writeln!(
        sb,
        "EMOJI AUDIT FAILED: {} emoji codepoint(s) found",
        findings.len()
    );
    for f in findings {
        let _ = writeln!(
            sb,
            "  {}:{}:{}  [{}]  {}",
            f.file, f.line, f.column, f.severity, f.codepoint
        );
    }
    sb
}

/// Serialize emoji findings as a JSON envelope string.
///
/// # Errors
///
/// Returns an error if JSON serialization fails.
fn format_json(findings: &[EmojiFinding]) -> std::result::Result<String, Error> {
    let jf: Vec<FindingJson> = findings
        .iter()
        .map(|f| FindingJson {
            file: &f.file,
            line: f.line,
            column: f.column,
            codepoint: &f.codepoint,
            severity: &f.severity,
        })
        .collect();
    let status = if findings.is_empty() {
        "passed"
    } else {
        "failed"
    };
    let env = Envelope {
        schema: SCHEMA,
        status,
        result: jf,
    };
    let mut s = serde_json::to_string_pretty(&env)?;
    s.push('\n');
    Ok(s)
}

/// Format emoji findings as a Markdown table.
fn format_markdown(findings: &[EmojiFinding]) -> String {
    if findings.is_empty() {
        return "## Governance Emoji Audit\n\n**PASSED**: no emoji codepoints found in forbidden file types\n"
            .to_string();
    }
    let mut sb = String::new();
    let _ = writeln!(
        sb,
        "## Governance Emoji Audit\n\n**FAILED**: {} emoji codepoint(s) found\n",
        findings.len()
    );
    sb.push_str("| File | Line | Column | Codepoint | Severity |\n");
    sb.push_str("|------|------|--------|-----------|----------|\n");
    for f in findings {
        let _ = writeln!(
            sb,
            "| {} | {} | {} | {} | {} |",
            f.file, f.line, f.column, f.codepoint, f.severity
        );
    }
    sb
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn format_text_passes_when_no_findings() {
        let s = format_text(&[]);
        assert!(s.starts_with("EMOJI AUDIT PASSED"));
    }

    #[test]
    fn format_text_fails_with_findings() {
        let f = EmojiFinding {
            file: "x.json".to_string(),
            line: 1,
            column: 2,
            codepoint: "U+2713".to_string(),
            severity: "high".to_string(),
        };
        let s = format_text(&[f]);
        assert!(s.contains("EMOJI AUDIT FAILED: 1"));
        assert!(s.contains("x.json:1:2"));
    }

    #[test]
    fn format_json_status_passed_on_empty() {
        let s = format_json(&[]).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "passed");
        assert_eq!(v["schema"], SCHEMA);
    }

    fn sample() -> EmojiFinding {
        EmojiFinding {
            file: "x.json".to_string(),
            line: 1,
            column: 2,
            codepoint: "U+2713".to_string(),
            severity: "high".to_string(),
        }
    }

    #[test]
    fn format_json_status_failed_on_findings() {
        let s = format_json(&[sample()]).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "failed");
        assert_eq!(v["result"][0]["codepoint"], "U+2713");
    }

    #[test]
    fn format_markdown_passed_when_empty() {
        let s = format_markdown(&[]);
        assert!(s.contains("**PASSED**"));
    }

    #[test]
    fn format_markdown_table_with_findings() {
        let s = format_markdown(&[sample()]);
        assert!(s.contains("**FAILED**: 1"));
        assert!(s.contains("| File | Line | Column | Codepoint | Severity |"));
        assert!(s.contains("| x.json | 1 | 2 | U+2713 | high |"));
    }
}
