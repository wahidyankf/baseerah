//! `repo-governance vendor-audit` — checks that governance docs use vendor-neutral terminology.
//!
//! Port of `apps/rhino-cli/cmd/governance_vendor_audit.go`.

use std::fmt::Write as _;
use std::path::Path;

use anyhow::{Context, Error, anyhow};
use clap::Args;
use serde::Serialize;

use crate::domain::cliout::OutputFormat;
use crate::internal::git;
use crate::internal::repo_governance::vendor_audit::{Finding, walk};

/// CLI arguments for `repo-governance vendor-audit`.
#[derive(Args, Debug)]
pub struct VendorAuditArgs {
    /// Optional scan path (defaults to repo-governance/).
    pub path: Option<String>,
}

/// Single vendor term finding in JSON output.
#[derive(Serialize)]
struct JsonFinding<'a> {
    /// Path of the file containing the term.
    path: &'a str,
    /// Line number.
    line: usize,
    /// Matched vendor term.
    r#match: &'a str,
    /// Vendor-neutral replacement.
    replacement: &'a str,
}

/// JSON result wrapping the vendor audit findings (no outer schema envelope).
#[derive(Serialize)]
struct JsonResult<'a> {
    /// `"passed"` or `"failed"`.
    status: &'a str,
    /// Total number of findings.
    count: usize,
    /// Individual findings.
    findings: Vec<JsonFinding<'a>>,
}

/// Run the `repo-governance vendor-audit` command.
///
/// # Errors
///
/// Returns an error if the git root cannot be found, the audit fails, or
/// vendor term violations are detected.
pub fn run(args: &VendorAuditArgs, output_format: OutputFormat) -> std::result::Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    let scan_path = args.path.as_deref().unwrap_or("repo-governance");
    let full_path = if Path::new(scan_path).is_absolute() {
        scan_path.into()
    } else {
        repo_root.join(scan_path)
    };
    let findings = walk(&full_path).context("vendor audit failed")?;

    match output_format {
        OutputFormat::Text => print!("{}", format_text(&findings)),
        OutputFormat::Json => print!("{}", format_json(&findings)?),
        OutputFormat::Markdown => print!("{}", format_markdown(&findings)),
    }

    if !findings.is_empty() {
        return Err(anyhow!("{} violation(s) found", findings.len()));
    }
    Ok(())
}

/// Format vendor audit findings as human-readable text.
fn format_text(findings: &[Finding]) -> String {
    if findings.is_empty() {
        return "GOVERNANCE VENDOR AUDIT PASSED: no violations found\n".to_string();
    }
    let mut sb = String::new();
    let _ = writeln!(
        sb,
        "GOVERNANCE VENDOR AUDIT FAILED: {} violation(s) found",
        findings.len()
    );
    for f in findings {
        let _ = writeln!(
            sb,
            "  {}:{}  {}  →  {}",
            f.path, f.line, f.r#match, f.replacement
        );
    }
    sb
}

/// Serialize vendor audit findings as a JSON string.
///
/// # Errors
///
/// Returns an error if JSON serialization fails.
fn format_json(findings: &[Finding]) -> std::result::Result<String, Error> {
    let status = if findings.is_empty() {
        "passed"
    } else {
        "failed"
    };
    let jf: Vec<JsonFinding> = findings
        .iter()
        .map(|f| JsonFinding {
            path: &f.path,
            line: f.line,
            r#match: &f.r#match,
            replacement: &f.replacement,
        })
        .collect();
    let result = JsonResult {
        status,
        count: findings.len(),
        findings: jf,
    };
    let mut s = serde_json::to_string_pretty(&result)?;
    s.push('\n');
    Ok(s)
}

/// Format vendor audit findings as a Markdown table.
fn format_markdown(findings: &[Finding]) -> String {
    if findings.is_empty() {
        return "## Governance Vendor Audit\n\n**PASSED**: no violations found\n".to_string();
    }
    let mut sb = String::new();
    let _ = writeln!(
        sb,
        "## Governance Vendor Audit\n\n**FAILED**: {} violation(s) found\n",
        findings.len()
    );
    sb.push_str("| File | Line | Term | Replacement |\n");
    sb.push_str("|------|------|------|-------------|\n");
    for f in findings {
        let _ = writeln!(
            sb,
            "| {} | {} | `{}` | {} |",
            f.path, f.line, f.r#match, f.replacement
        );
    }
    sb
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    fn sample() -> Finding {
        Finding {
            path: "x.md".to_string(),
            line: 3,
            r#match: "Claude Code".to_string(),
            replacement: "the coding agent".to_string(),
        }
    }

    #[test]
    fn format_text_passed() {
        assert!(format_text(&[]).starts_with("GOVERNANCE VENDOR AUDIT PASSED"));
    }

    #[test]
    fn format_text_failed() {
        let s = format_text(&[sample()]);
        assert!(s.contains("GOVERNANCE VENDOR AUDIT FAILED: 1"));
        assert!(s.contains("x.md:3"));
    }

    #[test]
    fn format_json_passed() {
        let s = format_json(&[]).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "passed");
        assert_eq!(v["count"], 0);
    }

    #[test]
    fn format_json_failed() {
        let s = format_json(&[sample()]).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "failed");
        assert_eq!(v["findings"][0]["match"], "Claude Code");
    }

    #[test]
    fn format_markdown_passed() {
        assert!(format_markdown(&[]).contains("**PASSED**"));
    }

    #[test]
    fn format_markdown_failed() {
        let s = format_markdown(&[sample()]);
        assert!(s.contains("**FAILED**: 1"));
        assert!(s.contains("x.md"));
    }
}
