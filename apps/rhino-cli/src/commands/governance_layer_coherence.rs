//! `repo-governance layer-coherence` — checks that governance docs exist for each defined layer.
//!
//! Port of `apps/rhino-cli/cmd/governance_layer_coherence.go`.

use std::fmt::Write as _;

use anyhow::{Context, Error, anyhow};
use clap::Args;
use serde::Serialize;

use crate::domain::cliout::OutputFormat;
use crate::internal::git;
use crate::internal::repo_governance::layer_coherence::{
    LayerCoherenceFinding, audit_layer_coherence,
};

/// JSON output schema identifier for this command.
const SCHEMA: &str = "rhino-cli/layer-coherence/v1";

/// CLI arguments for `repo-governance layer-coherence` (none required).
#[derive(Args, Debug)]
pub struct LayerCoherenceArgs {}

/// Single layer coherence finding in JSON output.
#[derive(Serialize)]
struct JsonFinding<'a> {
    /// Path of the file containing the finding.
    file: &'a str,
    /// Severity label.
    severity: &'a str,
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

/// JSON envelope wrapping the layer coherence audit result.
#[derive(Serialize)]
struct Envelope<'a> {
    /// Output schema identifier.
    schema: &'a str,
    /// `"passed"` or `"failed"`.
    status: &'a str,
    /// Detailed result.
    result: InnerResult<'a>,
}

/// Run the `repo-governance layer-coherence` command.
///
/// # Errors
///
/// Returns an error if the git root cannot be found, the audit fails, or
/// layer coherence findings are detected.
pub fn run(
    _args: &LayerCoherenceArgs,
    output_format: OutputFormat,
) -> std::result::Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    let findings = audit_layer_coherence(&repo_root).context("layer-coherence audit failed")?;

    match output_format {
        OutputFormat::Text => print!("{}", format_text(&findings)),
        OutputFormat::Json => print!("{}", format_json(&findings)?),
        OutputFormat::Markdown => print!("{}", format_markdown(&findings)),
    }

    if !findings.is_empty() {
        return Err(anyhow!(
            "{} layer-coherence finding(s) reported",
            findings.len()
        ));
    }
    Ok(())
}

/// Format layer coherence findings as human-readable text.
fn format_text(findings: &[LayerCoherenceFinding]) -> String {
    if findings.is_empty() {
        return "LAYER COHERENCE AUDIT PASSED: zero findings\n".to_string();
    }
    let mut sb = String::new();
    let _ = writeln!(
        sb,
        "LAYER COHERENCE AUDIT FAILED: {} finding(s) reported",
        findings.len()
    );
    for f in findings {
        let _ = writeln!(
            sb,
            "  {}  [{}]  {} — {}",
            f.file, f.severity, f.kind, f.message
        );
    }
    sb
}

/// Serialize layer coherence findings as a JSON envelope string.
///
/// # Errors
///
/// Returns an error if JSON serialization fails.
fn format_json(findings: &[LayerCoherenceFinding]) -> std::result::Result<String, Error> {
    let status = if findings.is_empty() {
        "passed"
    } else {
        "failed"
    };
    let jf: Vec<JsonFinding> = findings
        .iter()
        .map(|f| JsonFinding {
            file: &f.file,
            severity: &f.severity,
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

/// Format layer coherence findings as a Markdown table.
fn format_markdown(findings: &[LayerCoherenceFinding]) -> String {
    if findings.is_empty() {
        return "## Layer Coherence Audit\n\n**PASSED**: zero findings\n".to_string();
    }
    let mut sb = String::new();
    let _ = writeln!(
        sb,
        "## Layer Coherence Audit\n\n**FAILED**: {} finding(s) reported\n",
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

    fn sample() -> LayerCoherenceFinding {
        LayerCoherenceFinding {
            file: "a.md".to_string(),
            severity: "fail".to_string(),
            kind: "missing-doc".to_string(),
            message: "x".to_string(),
        }
    }

    #[test]
    fn format_text_passed() {
        assert!(format_text(&[]).starts_with("LAYER COHERENCE AUDIT PASSED"));
    }

    #[test]
    fn format_text_failed() {
        let s = format_text(&[sample()]);
        assert!(s.contains("LAYER COHERENCE AUDIT FAILED: 1"));
        assert!(s.contains("a.md"));
    }

    #[test]
    fn format_json_passed() {
        let s = format_json(&[]).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "passed");
        assert_eq!(v["result"]["count"], 0);
        assert_eq!(v["schema"], SCHEMA);
    }

    #[test]
    fn format_json_failed() {
        let s = format_json(&[sample()]).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "failed");
        assert_eq!(v["result"]["count"], 1);
        assert_eq!(v["result"]["findings"][0]["kind"], "missing-doc");
    }

    #[test]
    fn format_markdown_passed() {
        assert!(format_markdown(&[]).contains("**PASSED**"));
    }

    #[test]
    fn format_markdown_failed() {
        let s = format_markdown(&[sample()]);
        assert!(s.contains("**FAILED**: 1"));
        assert!(s.contains("| a.md | fail | missing-doc | x |"));
    }
}
