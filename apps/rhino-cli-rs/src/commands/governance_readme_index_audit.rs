// Port of `apps/rhino-cli/cmd/governance_readme_index_audit.go`.

use std::fmt::Write as _;
use std::path::Path;

use anyhow::{anyhow, Context, Error};
use clap::Args;
use serde::Serialize;

use crate::internal::cliout::OutputFormat;
use crate::internal::gitutil;
use crate::internal::repo_governance::readme_index_audit::{
    audit_readme_index, ReadmeIndexFinding,
};

const SCHEMA: &str = "rhino-cli/readme-index-audit/v1";

const DEFAULT_PATHS: &[&str] = &[
    "repo-governance/",
    ".claude/agents/",
    ".claude/skills/",
    "docs/explanation/software-engineering/",
];

#[derive(Args, Debug)]
pub struct ReadmeIndexAuditArgs {
    /// Glob to exclude from audit (repeatable).
    #[arg(long = "exclude")]
    pub exclude: Vec<String>,
    /// Positional paths (override defaults).
    pub positional: Vec<String>,
}

#[derive(Serialize)]
struct JsonFinding<'a> {
    file: &'a str,
    severity: &'a str,
    kind: &'a str,
    message: &'a str,
}

#[derive(Serialize)]
struct InnerResult<'a> {
    findings: Vec<JsonFinding<'a>>,
}

#[derive(Serialize)]
struct Envelope<'a> {
    schema: &'a str,
    status: &'a str,
    result: InnerResult<'a>,
}

pub fn run(
    args: &ReadmeIndexAuditArgs,
    output_format: OutputFormat,
) -> std::result::Result<(), Error> {
    let repo_root =
        gitutil::find_git_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    let rel_paths: Vec<String> = if !args.positional.is_empty() {
        args.positional.clone()
    } else {
        DEFAULT_PATHS.iter().map(|s| s.to_string()).collect()
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

    let findings =
        audit_readme_index(&full_paths, &args.exclude).context("readme-index audit failed")?;

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
