//! `repo-governance traceability-audit` — checks that governance docs reference required vision sections.
//!
//! Port of `apps/rhino-cli/cmd/governance_traceability_audit.go`.

use std::fmt::Write as _;

use anyhow::{Context, Error, anyhow};
use clap::Args;
use serde::Serialize;

use crate::domain::cliout::OutputFormat;
use crate::infrastructure::fs::real::RealFs;
use crate::internal::git;
use crate::internal::repo_governance::traceability_audit::{
    TraceabilityFinding, audit_traceability,
};

/// JSON output schema identifier for this command.
const SCHEMA: &str = "rhino-cli/traceability-audit/v1";

/// CLI arguments for `repo-governance traceability-audit` (none required).
#[derive(Args, Debug)]
pub struct TraceabilityAuditArgs {}

/// Single traceability finding in JSON output.
#[derive(Serialize)]
struct JsonFinding<'a> {
    /// Path of the file containing the finding.
    path: &'a str,
    /// Line number of the offending reference.
    line: usize,
    /// Finding category.
    kind: &'a str,
    /// Human-readable description.
    message: &'a str,
}

/// Inner result summary in JSON output.
#[derive(Serialize)]
struct InnerResult<'a> {
    /// `"passed"` or `"failed"`.
    status: &'a str,
    /// Total number of findings.
    count: usize,
    /// Individual findings.
    findings: Vec<JsonFinding<'a>>,
}

/// JSON envelope wrapping the traceability audit result.
#[derive(Serialize)]
struct Envelope<'a> {
    /// Output schema identifier.
    schema: &'a str,
    /// `"passed"` or `"failed"`.
    status: &'a str,
    /// Detailed result.
    result: InnerResult<'a>,
}

/// Run the `repo-governance traceability-audit` command.
///
/// # Errors
///
/// Returns an error if the git root cannot be found, the audit fails, or
/// traceability findings are detected.
pub fn run(
    _args: &TraceabilityAuditArgs,
    output_format: OutputFormat,
) -> std::result::Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    let findings = audit_traceability(&RealFs, &repo_root).context("traceability audit failed")?;

    match output_format {
        OutputFormat::Text => print!("{}", format_text(&findings)),
        OutputFormat::Json => print!("{}", format_json(&findings)?),
        OutputFormat::Markdown => print!("{}", format_markdown(&findings)),
    }

    if !findings.is_empty() {
        return Err(anyhow!(
            "{} traceability finding(s) reported",
            findings.len()
        ));
    }
    Ok(())
}

/// Format traceability findings as human-readable text.
fn format_text(findings: &[TraceabilityFinding]) -> String {
    if findings.is_empty() {
        return "TRACEABILITY AUDIT PASSED: zero findings\n".to_string();
    }
    let mut sb = String::new();
    let _ = writeln!(
        sb,
        "TRACEABILITY AUDIT FAILED: {} finding(s) reported",
        findings.len()
    );
    for f in findings {
        let _ = writeln!(sb, "  {}:{}  {}  {}", f.path, f.line, f.kind, f.message);
    }
    sb
}

/// Serialize traceability findings as a JSON envelope string.
///
/// # Errors
///
/// Returns an error if JSON serialization fails.
fn format_json(findings: &[TraceabilityFinding]) -> std::result::Result<String, Error> {
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
            kind: &f.kind,
            message: &f.message,
        })
        .collect();
    let env = Envelope {
        schema: SCHEMA,
        status,
        result: InnerResult {
            status,
            count: findings.len(),
            findings: jf,
        },
    };
    let mut s = serde_json::to_string_pretty(&env)?;
    s.push('\n');
    Ok(s)
}

/// Format traceability findings as a Markdown table.
fn format_markdown(findings: &[TraceabilityFinding]) -> String {
    if findings.is_empty() {
        return "## Traceability Audit\n\n**PASSED**: zero findings\n".to_string();
    }
    let mut sb = String::new();
    let _ = writeln!(
        sb,
        "## Traceability Audit\n\n**FAILED**: {} finding(s) reported\n",
        findings.len()
    );
    sb.push_str("| File | Line | Kind | Message |\n");
    sb.push_str("|------|------|------|---------|\n");
    for f in findings {
        let _ = writeln!(
            sb,
            "| {} | {} | {} | {} |",
            f.path, f.line, f.kind, f.message
        );
    }
    sb
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    fn sample() -> TraceabilityFinding {
        TraceabilityFinding {
            path: "p.md".to_string(),
            line: 1,
            kind: "missing-vision-supported".to_string(),
            message: "msg".to_string(),
        }
    }

    #[test]
    fn format_text_passed() {
        assert!(format_text(&[]).starts_with("TRACEABILITY AUDIT PASSED"));
    }

    #[test]
    fn format_text_failed() {
        let s = format_text(&[sample()]);
        assert!(s.contains("TRACEABILITY AUDIT FAILED: 1"));
        assert!(s.contains("p.md:1"));
    }

    #[test]
    fn format_json_passed() {
        let s = format_json(&[]).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "passed");
        assert_eq!(v["result"]["count"], 0);
    }

    #[test]
    fn format_json_failed() {
        let s = format_json(&[sample()]).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "failed");
        assert_eq!(
            v["result"]["findings"][0]["kind"],
            "missing-vision-supported"
        );
    }

    #[test]
    fn format_markdown_passed() {
        assert!(format_markdown(&[]).contains("**PASSED**"));
    }

    #[test]
    fn format_markdown_failed() {
        let s = format_markdown(&[sample()]);
        assert!(s.contains("**FAILED**: 1"));
        assert!(s.contains("| p.md | 1 | missing-vision-supported | msg |"));
    }
}
