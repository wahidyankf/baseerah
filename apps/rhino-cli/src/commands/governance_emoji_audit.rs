// Port of `apps/rhino-cli/cmd/governance_emoji_audit.go`.

use std::fmt::Write as _;
use std::path::Path;

use anyhow::{anyhow, Context, Error};
use clap::Args;
use serde::Serialize;

use crate::internal::cliout::OutputFormat;
use crate::internal::gitutil;
use crate::internal::repo_governance::emoji_audit::{audit_emoji, EmojiFinding};

const SCHEMA: &str = "rhino-cli/emoji-audit/v1";

#[derive(Args, Debug)]
pub struct EmojiAuditArgs {
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
    column: usize,
    codepoint: &'a str,
    severity: &'a str,
}

#[derive(Serialize)]
struct Envelope<'a> {
    schema: &'a str,
    status: &'a str,
    result: Vec<FindingJson<'a>>,
}

pub fn run(args: &EmojiAuditArgs, output_format: OutputFormat) -> std::result::Result<(), Error> {
    let repo_root =
        gitutil::find_git_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;

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

    let findings = audit_emoji(&full_paths).context("emoji audit failed")?;

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
