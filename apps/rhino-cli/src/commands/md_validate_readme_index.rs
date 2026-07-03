//! `md readme-index` — checks that every file is indexed in its README.
//!
//! Port of `apps/rhino-cli/cmd/governance_readme_index_audit.go`.

use std::fmt::Write as _;
use std::path::Path;

use anyhow::{Context, Error, anyhow};
use clap::Args;
use serde::Serialize;

use crate::domain::cliout::OutputFormat;
use crate::infrastructure::fs::real::RealFs;
use crate::internal::git;
use crate::internal::repo_governance::readme_index_audit::{
    ReadmeIndexFinding, audit_readme_index,
};

/// JSON output schema identifier for this command.
const SCHEMA: &str = "rhino-cli/readme-index-audit/v1";

/// Default paths scanned when no arguments are supplied.
const DEFAULT_PATHS: &[&str] = &[
    "repo-governance/",
    ".claude/agents/",
    ".claude/skills/",
    "docs/explanation/software-engineering/",
];

/// CLI arguments for `repo-governance readme-index-audit`.
#[derive(Args, Debug)]
pub struct ReadmeIndexAuditArgs {
    /// Glob to exclude from audit (repeatable).
    #[arg(long = "exclude")]
    pub exclude: Vec<String>,
    /// Positional paths (override defaults).
    pub positional: Vec<String>,
}

/// Single README index finding in JSON output.
#[derive(Serialize)]
struct JsonFinding<'a> {
    /// Path of the file containing the finding.
    file: &'a str,
    /// Severity label.
    severity: &'a str,
    /// Finding category (e.g. `"orphan"`, `"ghost"`).
    kind: &'a str,
    /// Human-readable description.
    message: &'a str,
}

/// Inner result payload in JSON output.
#[derive(Serialize)]
struct InnerResult<'a> {
    /// Individual findings.
    findings: Vec<JsonFinding<'a>>,
}

/// JSON envelope wrapping the README index audit result.
#[derive(Serialize)]
struct Envelope<'a> {
    /// Output schema identifier.
    schema: &'a str,
    /// `"passed"` or `"failed"`.
    status: &'a str,
    /// Detailed result.
    result: InnerResult<'a>,
}

/// Run the `repo-governance readme-index-audit` command.
///
/// # Errors
///
/// Returns an error if the git root cannot be found, the audit fails, or
/// README index findings are detected.
pub fn run(
    args: &ReadmeIndexAuditArgs,
    output_format: OutputFormat,
) -> std::result::Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    let rel_paths: Vec<String> = if args.positional.is_empty() {
        DEFAULT_PATHS
            .iter()
            .map(std::string::ToString::to_string)
            .collect()
    } else {
        args.positional.clone()
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

    let findings = audit_readme_index(&RealFs, &full_paths, &args.exclude)
        .context("readme-index audit failed")?;

    match output_format {
        OutputFormat::Text => print!("{}", format_text(&findings)),
        OutputFormat::Json => print!("{}", format_json(&findings)?),
        OutputFormat::Markdown => print!("{}", format_markdown(&findings)),
    }

    if !findings.is_empty() {
        return Err(anyhow!("{} readme-index finding(s) found", findings.len()));
    }
    Ok(())
}

/// Format README index findings as human-readable text.
fn format_text(findings: &[ReadmeIndexFinding]) -> String {
    if findings.is_empty() {
        return "README INDEX AUDIT PASSED: no orphan or ghost references found\n".to_string();
    }
    let mut sb = String::new();
    let _ = writeln!(
        sb,
        "README INDEX AUDIT FAILED: {} finding(s)",
        findings.len()
    );
    for f in findings {
        let _ = writeln!(
            sb,
            "  {}  [{}/{}]  {}",
            f.file, f.severity, f.kind, f.message
        );
    }
    sb
}

/// Serialize README index findings as a JSON envelope string.
///
/// # Errors
///
/// Returns an error if JSON serialization fails.
fn format_json(findings: &[ReadmeIndexFinding]) -> std::result::Result<String, Error> {
    let jf: Vec<JsonFinding> = findings
        .iter()
        .map(|f| JsonFinding {
            file: &f.file,
            severity: &f.severity,
            kind: &f.kind,
            message: &f.message,
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
        result: InnerResult { findings: jf },
    };
    let mut s = serde_json::to_string_pretty(&env)?;
    s.push('\n');
    Ok(s)
}

/// Format README index findings as a Markdown table.
fn format_markdown(findings: &[ReadmeIndexFinding]) -> String {
    if findings.is_empty() {
        return "## README Index Audit\n\n**PASSED**: no orphan or ghost references found\n"
            .to_string();
    }
    let mut sb = String::new();
    let _ = writeln!(
        sb,
        "## README Index Audit\n\n**FAILED**: {} finding(s)\n",
        findings.len()
    );
    sb.push_str("| File | Severity | Kind | Message |\n");
    sb.push_str("|------|----------|------|---------|\n");
    for f in findings {
        let _ = writeln!(
            sb,
            "| {} | {} | {} | {} |",
            f.file, f.severity, f.kind, f.message
        );
    }
    sb
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    fn sample() -> ReadmeIndexFinding {
        ReadmeIndexFinding {
            file: "a.md".to_string(),
            severity: "high".to_string(),
            kind: "orphan".to_string(),
            message: "msg".to_string(),
        }
    }

    #[test]
    fn format_text_passed() {
        assert!(format_text(&[]).starts_with("README INDEX AUDIT PASSED"));
    }

    #[test]
    fn format_text_failed() {
        let s = format_text(&[sample()]);
        assert!(s.contains("README INDEX AUDIT FAILED: 1"));
        assert!(s.contains("a.md  [high/orphan]"));
    }

    #[test]
    fn format_json_passed() {
        let s = format_json(&[]).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "passed");
        assert_eq!(v["schema"], SCHEMA);
    }

    #[test]
    fn format_json_failed() {
        let s = format_json(&[sample()]).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "failed");
        assert_eq!(v["result"]["findings"][0]["kind"], "orphan");
    }

    #[test]
    fn format_markdown_passed() {
        assert!(format_markdown(&[]).contains("**PASSED**"));
    }

    #[test]
    fn format_markdown_failed() {
        let s = format_markdown(&[sample()]);
        assert!(s.contains("**FAILED**: 1"));
        assert!(s.contains("| a.md | high | orphan | msg |"));
    }
}
