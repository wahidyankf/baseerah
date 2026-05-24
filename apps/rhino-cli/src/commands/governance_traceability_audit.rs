// Port of `apps/rhino-cli/cmd/governance_traceability_audit.go`.

use std::fmt::Write as _;

use anyhow::{Context, Error, anyhow};
use clap::Args;
use serde::Serialize;

use crate::internal::cliout::OutputFormat;
use crate::internal::git;
use crate::internal::repo_governance::traceability_audit::{
    TraceabilityFinding, audit_traceability,
};

const SCHEMA: &str = "rhino-cli/traceability-audit/v1";

#[derive(Args, Debug)]
pub struct TraceabilityAuditArgs {}

#[derive(Serialize)]
struct JsonFinding<'a> {
    path: &'a str,
    line: usize,
    kind: &'a str,
    message: &'a str,
}

#[derive(Serialize)]
struct InnerResult<'a> {
    status: &'a str,
    count: usize,
    findings: Vec<JsonFinding<'a>>,
}

#[derive(Serialize)]
struct Envelope<'a> {
    schema: &'a str,
    status: &'a str,
    result: InnerResult<'a>,
}

pub fn run(
    _args: &TraceabilityAuditArgs,
    output_format: OutputFormat,
) -> std::result::Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    let findings = audit_traceability(&repo_root).context("traceability audit failed")?;

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
