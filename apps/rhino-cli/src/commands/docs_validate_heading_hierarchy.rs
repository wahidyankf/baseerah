// Port of `apps/rhino-cli/cmd/docs_validate_heading_hierarchy.go`.

use std::fmt::Write as _;
use std::path::Path;

use anyhow::{Context, Error, anyhow};
use clap::Args;
use serde::Serialize;

use crate::internal::cliout::OutputFormat;
use crate::internal::docs::heading_hierarchy::{
    DocsHeadingFinding, validate_docs_heading_hierarchy,
};
use crate::internal::gitutil;

const SCHEMA: &str = "rhino-cli/docs-validate-heading-hierarchy/v1";

const DEFAULT_PATHS: &[&str] = &["docs/", "repo-governance/"];

#[derive(Args, Debug)]
pub struct ValidateHeadingHierarchyArgs {
    /// Optional positional paths (override defaults).
    pub positional: Vec<String>,
}

#[derive(Serialize)]
struct JsonFinding<'a> {
    file: &'a str,
    line: usize,
    severity: &'a str,
    kind: &'a str,
    message: &'a str,
}

#[derive(Serialize)]
struct Envelope<'a> {
    schema: &'a str,
    status: &'a str,
    result: Vec<JsonFinding<'a>>,
}

pub fn run(
    args: &ValidateHeadingHierarchyArgs,
    output_format: OutputFormat,
) -> std::result::Result<(), Error> {
    let repo_root =
        gitutil::find_git_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
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

    let findings = validate_docs_heading_hierarchy(&full_paths)
        .context("docs validate-heading-hierarchy failed")?;

    match output_format {
        OutputFormat::Text => print!("{}", format_text(&findings)),
        OutputFormat::Json => print!("{}", format_json(&findings)?),
        OutputFormat::Markdown => print!("{}", format_markdown(&findings)),
    }

    if !findings.is_empty() {
        return Err(anyhow!(
            "{} docs heading hierarchy finding(s) found",
            findings.len()
        ));
    }
    Ok(())
}

fn format_text(findings: &[DocsHeadingFinding]) -> String {
    if findings.is_empty() {
        return "DOCS HEADING HIERARCHY VALIDATION PASSED: no heading hierarchy violations found\n"
            .to_string();
    }
    let mut sb = String::new();
    let _ = writeln!(
        sb,
        "DOCS HEADING HIERARCHY VALIDATION FAILED: {} violation(s) found",
        findings.len()
    );
    for f in findings {
        let _ = writeln!(
            sb,
            "  {}:{}  [{}]  [{}]  {}",
            f.file, f.line, f.severity, f.kind, f.message
        );
    }
    sb
}

fn format_json(findings: &[DocsHeadingFinding]) -> std::result::Result<String, Error> {
    let jf: Vec<JsonFinding> = findings
        .iter()
        .map(|f| JsonFinding {
            file: &f.file,
            line: f.line,
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
        result: jf,
    };
    let mut s = serde_json::to_string_pretty(&env)?;
    s.push('\n');
    Ok(s)
}

fn format_markdown(findings: &[DocsHeadingFinding]) -> String {
    if findings.is_empty() {
        return "## Docs Heading Hierarchy Validation\n\n**PASSED**: no heading hierarchy violations found\n"
            .to_string();
    }
    let mut sb = String::new();
    let _ = writeln!(
        sb,
        "## Docs Heading Hierarchy Validation\n\n**FAILED**: {} violation(s) found\n",
        findings.len()
    );
    sb.push_str("| File | Line | Severity | Kind | Message |\n");
    sb.push_str("|------|------|----------|------|---------|\n");
    for f in findings {
        let _ = writeln!(
            sb,
            "| {} | {} | {} | {} | {} |",
            f.file, f.line, f.severity, f.kind, f.message
        );
    }
    sb
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    fn sample() -> DocsHeadingFinding {
        DocsHeadingFinding {
            file: "a.md".to_string(),
            line: 2,
            severity: "high".to_string(),
            kind: "missing-h1".to_string(),
            message: "msg".to_string(),
        }
    }

    #[test]
    fn format_text_passed() {
        assert!(format_text(&[]).starts_with("DOCS HEADING HIERARCHY VALIDATION PASSED"));
    }

    #[test]
    fn format_text_failed() {
        let s = format_text(&[sample()]);
        assert!(s.contains("FAILED: 1 violation"));
        assert!(s.contains("a.md:2"));
    }

    #[test]
    fn format_json_passed() {
        let s = format_json(&[]).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "passed");
    }

    #[test]
    fn format_json_failed() {
        let s = format_json(&[sample()]).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "failed");
        assert_eq!(v["result"][0]["kind"], "missing-h1");
    }

    #[test]
    fn format_markdown_passed() {
        assert!(format_markdown(&[]).contains("**PASSED**"));
    }

    #[test]
    fn format_markdown_failed() {
        let s = format_markdown(&[sample()]);
        assert!(s.contains("**FAILED**: 1"));
        assert!(s.contains("| a.md | 2 | high | missing-h1 | msg |"));
    }
}
