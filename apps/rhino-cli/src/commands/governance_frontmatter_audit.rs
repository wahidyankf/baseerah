// Port of `apps/rhino-cli/cmd/governance_frontmatter_audit.go`.

use std::fmt::Write as _;
use std::path::Path;

use anyhow::{Context, Error, anyhow};
use clap::Args;
use serde::Serialize;

use crate::internal::cliout::OutputFormat;
use crate::internal::gitutil;
use crate::internal::repo_governance::frontmatter_audit::{FrontmatterFinding, audit_frontmatter};

const SCHEMA: &str = "rhino-cli/frontmatter-audit/v1";

const DEFAULT_PATHS: &[&str] = &[
    "repo-governance/",
    "docs/explanation/software-engineering/",
    ".claude/agents/",
    ".claude/skills/",
    "plans/",
];

#[derive(Args, Debug)]
pub struct FrontmatterAuditArgs {
    /// Paths to scan (repeatable; relative to git root).
    #[arg(short = 'p', long = "path", value_name = "PATH")]
    pub path: Vec<String>,
    /// Positional path overrides — same effect as --path.
    pub positional: Vec<String>,
}

#[derive(Serialize)]
struct FindingJson<'a> {
    file: &'a str,
    line: usize,
    severity: &'a str,
    message: &'a str,
}

#[derive(Serialize)]
struct Envelope<'a> {
    schema: &'a str,
    status: &'a str,
    result: Vec<FindingJson<'a>>,
}

pub fn run(
    args: &FrontmatterAuditArgs,
    output_format: OutputFormat,
) -> std::result::Result<(), Error> {
    let repo_root =
        gitutil::find_git_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;

    let rel_paths: Vec<String> = if !args.positional.is_empty() {
        args.positional.clone()
    } else if !args.path.is_empty() {
        args.path.clone()
    } else {
        DEFAULT_PATHS
            .iter()
            .map(std::string::ToString::to_string)
            .collect()
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

    let findings = audit_frontmatter(&full_paths).context("frontmatter audit failed")?;

    match output_format {
        OutputFormat::Text => print!("{}", format_text(&findings)),
        OutputFormat::Json => print!("{}", format_json(&findings)?),
        OutputFormat::Markdown => print!("{}", format_markdown(&findings)),
    }

    if !findings.is_empty() {
        return Err(anyhow!("{} frontmatter finding(s) found", findings.len()));
    }
    Ok(())
}

fn format_text(findings: &[FrontmatterFinding]) -> String {
    if findings.is_empty() {
        return "FRONTMATTER AUDIT PASSED: no date-metadata violations found\n".to_string();
    }
    let mut sb = String::new();
    let _ = writeln!(
        sb,
        "FRONTMATTER AUDIT FAILED: {} violation(s) found",
        findings.len()
    );
    for f in findings {
        let _ = writeln!(
            sb,
            "  {}:{}  [{}]  {}",
            f.file, f.line, f.severity, f.message
        );
    }
    sb
}

fn format_json(findings: &[FrontmatterFinding]) -> std::result::Result<String, Error> {
    let jf: Vec<FindingJson> = findings
        .iter()
        .map(|f| FindingJson {
            file: &f.file,
            line: f.line,
            severity: &f.severity,
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
        result: jf,
    };
    let mut s = serde_json::to_string_pretty(&env)?;
    s.push('\n');
    Ok(s)
}

fn format_markdown(findings: &[FrontmatterFinding]) -> String {
    if findings.is_empty() {
        return "## Governance Frontmatter Audit\n\n**PASSED**: no date-metadata violations found\n"
            .to_string();
    }
    let mut sb = String::new();
    let _ = writeln!(
        sb,
        "## Governance Frontmatter Audit\n\n**FAILED**: {} violation(s) found\n",
        findings.len()
    );
    sb.push_str("| File | Line | Severity | Message |\n");
    sb.push_str("|------|------|----------|---------|\n");
    for f in findings {
        let _ = writeln!(
            sb,
            "| {} | {} | {} | {} |",
            f.file, f.line, f.severity, f.message
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
        assert!(s.starts_with("FRONTMATTER AUDIT PASSED"));
    }

    #[test]
    fn format_text_fails_with_findings() {
        let f = FrontmatterFinding {
            file: "doc.md".to_string(),
            line: 5,
            severity: "high".to_string(),
            message: "x".to_string(),
        };
        let s = format_text(&[f]);
        assert!(s.contains("FRONTMATTER AUDIT FAILED: 1"));
        assert!(s.contains("doc.md:5"));
    }

    #[test]
    fn format_json_status_passed_on_empty() {
        let s = format_json(&[]).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "passed");
        assert_eq!(v["schema"], SCHEMA);
    }

    fn sample() -> FrontmatterFinding {
        FrontmatterFinding {
            file: "x.md".to_string(),
            line: 3,
            severity: "high".to_string(),
            message: "msg".to_string(),
        }
    }

    #[test]
    fn format_json_status_failed_on_findings() {
        let s = format_json(&[sample()]).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "failed");
        assert_eq!(v["result"][0]["file"], "x.md");
        assert_eq!(v["result"][0]["line"], 3);
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
        assert!(s.contains("| File | Line | Severity | Message |"));
        assert!(s.contains("| x.md | 3 | high | msg |"));
    }
}
