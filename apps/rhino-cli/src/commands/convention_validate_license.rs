//! `convention license` — checks that all apps/libs carry required license files.
//!
//! Port of `apps/rhino-cli/cmd/governance_license_audit.go`.

use std::fmt::Write as _;

use anyhow::{Context, Error, anyhow};
use clap::Args;
use serde::Serialize;

use crate::domain::cliout::OutputFormat;
use crate::internal::git;
use crate::internal::repo_governance::license_audit::{LicenseFinding, audit_license};

/// JSON output schema identifier for this command.
const SCHEMA: &str = "rhino-cli/license-audit/v1";

/// CLI arguments for `repo-governance license-audit` (none required).
#[derive(Args, Debug)]
pub struct LicenseAuditArgs {}

/// Inner result summary in JSON output.
#[derive(Serialize)]
struct InnerResult<'a> {
    /// Total number of license findings.
    total_findings: usize,
    /// Individual findings.
    findings: &'a [LicenseFinding],
}

/// JSON envelope wrapping the license audit result.
#[derive(Serialize)]
struct Envelope<'a> {
    /// Output schema identifier.
    schema: &'a str,
    /// `"passed"` or `"failed"`.
    status: &'a str,
    /// Detailed result.
    result: InnerResult<'a>,
}

/// Run the `repo-governance license-audit` command.
///
/// # Errors
///
/// Returns an error if the git root cannot be found, the audit fails, or
/// license findings are detected.
pub fn run(
    _args: &LicenseAuditArgs,
    output_format: OutputFormat,
) -> std::result::Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    let findings = audit_license(&repo_root).context("license audit failed")?;

    match output_format {
        OutputFormat::Text => print!("{}", format_text(&findings)),
        OutputFormat::Json => print!("{}", format_json(&findings)?),
        OutputFormat::Markdown => print!("{}", format_markdown(&findings)),
    }

    if !findings.is_empty() {
        return Err(anyhow!("{} license finding(s) found", findings.len()));
    }
    Ok(())
}

/// Format license findings as human-readable text.
fn format_text(findings: &[LicenseFinding]) -> String {
    if findings.is_empty() {
        return "LICENSE AUDIT PASSED: no findings\n".to_string();
    }
    let mut sb = String::new();
    let _ = writeln!(sb, "LICENSE AUDIT FAILED: {} finding(s)", findings.len());
    for f in findings {
        let _ = writeln!(sb, "  [{}] {} — {}", f.kind, f.path, f.message);
    }
    sb
}

/// Serialize license findings as a JSON envelope string.
///
/// # Errors
///
/// Returns an error if JSON serialization fails.
fn format_json(findings: &[LicenseFinding]) -> std::result::Result<String, Error> {
    let status = if findings.is_empty() {
        "passed"
    } else {
        "failed"
    };
    let env = Envelope {
        schema: SCHEMA,
        status,
        result: InnerResult {
            total_findings: findings.len(),
            findings,
        },
    };
    let mut s = serde_json::to_string_pretty(&env)?;
    s.push('\n');
    Ok(s)
}

/// Format license findings as a Markdown table.
fn format_markdown(findings: &[LicenseFinding]) -> String {
    let mut sb = String::new();
    sb.push_str("## License Audit\n\n");
    if findings.is_empty() {
        sb.push_str("**PASSED**: no findings\n");
        return sb;
    }
    let _ = writeln!(sb, "**FAILED**: {} finding(s)\n", findings.len());
    sb.push_str("| Kind | Path | Message |\n");
    sb.push_str("| --- | --- | --- |\n");
    for f in findings {
        let _ = writeln!(sb, "| {} | `{}` | {} |", f.kind, f.path, f.message);
    }
    sb
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    fn sample() -> LicenseFinding {
        LicenseFinding {
            path: "apps/foo".to_string(),
            kind: "missing-license".to_string(),
            message: "msg".to_string(),
        }
    }

    #[test]
    fn format_text_passed() {
        assert!(format_text(&[]).starts_with("LICENSE AUDIT PASSED"));
    }

    #[test]
    fn format_text_failed() {
        let s = format_text(&[sample()]);
        assert!(s.contains("LICENSE AUDIT FAILED: 1"));
        assert!(s.contains("[missing-license] apps/foo — msg"));
    }

    #[test]
    fn format_json_passed() {
        let s = format_json(&[]).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "passed");
        assert_eq!(v["result"]["total_findings"], 0);
        assert_eq!(v["schema"], SCHEMA);
    }

    #[test]
    fn format_json_failed() {
        let s = format_json(&[sample()]).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "failed");
        assert_eq!(v["result"]["total_findings"], 1);
        assert_eq!(v["result"]["findings"][0]["kind"], "missing-license");
    }

    #[test]
    fn format_markdown_passed() {
        assert!(format_markdown(&[]).contains("**PASSED**: no findings"));
    }

    #[test]
    fn format_markdown_failed() {
        let s = format_markdown(&[sample()]);
        assert!(s.contains("**FAILED**: 1"));
        assert!(s.contains("| missing-license | `apps/foo` | msg |"));
    }
}
