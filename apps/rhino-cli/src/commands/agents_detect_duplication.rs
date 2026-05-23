// Port of `apps/rhino-cli/cmd/agents_detect_duplication.go`.

use std::fmt::Write as _;

use anyhow::{anyhow, Context, Error};
use clap::Args;
use serde::Serialize;

use crate::internal::agents::detect_duplication::{detect_duplication, DuplicationFinding};
use crate::internal::cliout::OutputFormat;
use crate::internal::gitutil;

const SCHEMA: &str = "rhino-cli/agents-detect-duplication/v1";

#[derive(Args, Debug)]
pub struct DetectDuplicationArgs {}

#[derive(Serialize)]
struct JsonFinding<'a> {
    files: &'a [String],
    start_lines: &'a [usize],
    window_size: usize,
    severity: &'a str,
    message: &'a str,
}

#[derive(Serialize)]
struct Envelope<'a> {
    schema: &'a str,
    status: &'a str,
    result: Vec<JsonFinding<'a>>,
}

pub fn run(
    _args: &DetectDuplicationArgs,
    output_format: OutputFormat,
) -> std::result::Result<(), Error> {
    let repo_root =
        gitutil::find_git_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    let findings = detect_duplication(&repo_root).context("agents detect-duplication failed")?;

    match output_format {
        OutputFormat::Text => print!("{}", format_text(&findings)),
        OutputFormat::Json => print!("{}", format_json(&findings)?),
        OutputFormat::Markdown => print!("{}", format_markdown(&findings)),
    }

    if !findings.is_empty() {
        return Err(anyhow!(
            "{} duplication cluster(s) detected",
            findings.len()
        ));
    }
    Ok(())
}

fn format_text(findings: &[DuplicationFinding]) -> String {
    if findings.is_empty() {
        return "AGENTS DUPLICATION VALIDATION PASSED: 0 clusters\n".to_string();
    }
    let mut sb = String::new();
    let _ = writeln!(
        sb,
        "AGENTS DUPLICATION VALIDATION FAILED: {} cluster(s)",
        findings.len()
    );
    for f in findings {
        let _ = writeln!(
            sb,
            "  [{}] {} (window={})",
            f.severity, f.message, f.window_size
        );
        for (i, p) in f.files.iter().enumerate() {
            let _ = writeln!(sb, "    - {}:{}", p, f.start_lines[i]);
        }
    }
    sb
}

fn format_json(findings: &[DuplicationFinding]) -> std::result::Result<String, Error> {
    let jf: Vec<JsonFinding> = findings
        .iter()
        .map(|f| JsonFinding {
            files: &f.files,
            start_lines: &f.start_lines,
            window_size: f.window_size,
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

fn format_markdown(findings: &[DuplicationFinding]) -> String {
    let mut sb = String::new();
    sb.push_str("## Agents Duplication Detection\n\n");
    if findings.is_empty() {
        sb.push_str("**PASSED**: 0 duplication clusters detected\n");
        return sb;
    }
    let _ = writeln!(
        sb,
        "**FAILED**: {} duplication cluster(s) detected\n",
        findings.len()
    );
    sb.push_str("| Severity | Window | Files | Start Lines | Message |\n");
    sb.push_str("|----------|--------|-------|-------------|---------|\n");
    for f in findings {
        let files = f.files.join("<br>");
        let starts: Vec<String> = f.start_lines.iter().map(|s| s.to_string()).collect();
        let _ = writeln!(
            sb,
            "| {} | {} | {} | {} | {} |",
            f.severity,
            f.window_size,
            files,
            starts.join("<br>"),
            f.message
        );
    }
    sb
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> DuplicationFinding {
        DuplicationFinding {
            files: vec!["a.md".to_string(), "b.md".to_string()],
            start_lines: vec![5, 10],
            window_size: 10,
            severity: "high".to_string(),
            message: "10-line verbatim duplication across 2 files".to_string(),
        }
    }

    #[test]
    fn format_text_passed() {
        let s = format_text(&[]);
        assert!(s.contains("AGENTS DUPLICATION VALIDATION PASSED"));
    }

    #[test]
    fn format_text_failed() {
        let s = format_text(&[sample()]);
        assert!(s.contains("FAILED: 1 cluster"));
        assert!(s.contains("a.md:5"));
        assert!(s.contains("b.md:10"));
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
        assert_eq!(v["result"][0]["window_size"], 10);
    }

    #[test]
    fn format_markdown_passed() {
        assert!(format_markdown(&[]).contains("**PASSED**"));
    }

    #[test]
    fn format_markdown_failed() {
        let s = format_markdown(&[sample()]);
        assert!(s.contains("**FAILED**: 1"));
        assert!(s.contains("| Severity | Window"));
        assert!(s.contains("a.md<br>b.md"));
    }
}
