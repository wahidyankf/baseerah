//! `md validate-frontmatter` — validates YAML frontmatter in docs and governance files.
//!
//! Port of `apps/rhino-cli/cmd/docs_validate_frontmatter.go`.

use std::fmt::Write as _;
use std::path::Path;

use anyhow::{Context, Error, anyhow};
use clap::Args;
use serde::Serialize;

use crate::domain::cliout::OutputFormat;
use crate::internal::docs::frontmatter::{
    DocsFrontmatterFinding, count_severity, has_fail_findings, validate_docs_frontmatter,
};
use crate::internal::git;

/// JSON output schema identifier for this command.
const SCHEMA: &str = "rhino-cli/docs-validate-frontmatter/v1";

/// Default paths scanned when no positional arguments are supplied.
const DEFAULT_PATHS: &[&str] = &["docs/", "repo-governance/"];

/// CLI arguments for `docs validate-frontmatter`.
#[derive(Args, Debug)]
pub struct ValidateFrontmatterArgs {
    /// Optional positional paths (override defaults).
    pub positional: Vec<String>,
}

/// Single frontmatter finding in JSON output.
#[derive(Serialize)]
struct JsonFinding<'a> {
    /// Path of the file containing the finding.
    file: &'a str,
    /// Severity label (e.g. `"fail"` or `"warn"`).
    severity: &'a str,
    /// Finding category (e.g. `"missing-title"`).
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
    /// Number of fail-level findings.
    fail_count: usize,
    /// Number of warn-level findings.
    warn_count: usize,
    /// Individual findings.
    findings: Vec<JsonFinding<'a>>,
}

/// JSON envelope wrapping the frontmatter scan result.
#[derive(Serialize)]
struct Envelope<'a> {
    /// Output schema identifier.
    schema: &'a str,
    /// `"passed"` or `"failed"`.
    status: &'a str,
    /// Detailed result.
    result: InnerResult<'a>,
}

/// Run the `docs validate-frontmatter` command.
///
/// # Errors
///
/// Returns an error if the git root cannot be found, the scan fails, or
/// fail-level findings are detected.
pub fn run(
    args: &ValidateFrontmatterArgs,
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

    let findings =
        validate_docs_frontmatter(&full_paths).context("docs validate-frontmatter failed")?;

    match output_format {
        OutputFormat::Text => print!("{}", format_text(&findings)),
        OutputFormat::Json => print!("{}", format_json(&findings)?),
        OutputFormat::Markdown => print!("{}", format_markdown(&findings)),
    }

    if has_fail_findings(&findings) {
        let n = count_severity(&findings, "fail");
        return Err(anyhow!("{n} docs frontmatter fail-level finding(s) found"));
    }
    Ok(())
}

/// Format frontmatter findings as human-readable text.
fn format_text(findings: &[DocsFrontmatterFinding]) -> String {
    if !has_fail_findings(findings) && findings.is_empty() {
        return "DOCS FRONTMATTER VALIDATION PASSED: no findings\n".to_string();
    }
    let fail_n = count_severity(findings, "fail");
    let warn_n = count_severity(findings, "warn");
    let mut sb = String::new();
    if fail_n > 0 {
        let _ = write!(
            sb,
            "DOCS FRONTMATTER VALIDATION FAILED: {fail_n} fail finding(s)"
        );
        if warn_n > 0 {
            let _ = write!(sb, ", {warn_n} warn finding(s)");
        }
        sb.push('\n');
    } else {
        let _ = writeln!(
            sb,
            "DOCS FRONTMATTER VALIDATION PASSED with {warn_n} warn finding(s)"
        );
    }
    for f in findings {
        let _ = writeln!(
            sb,
            "  {}  [{}]  {} — {}",
            f.file, f.severity, f.kind, f.message
        );
    }
    sb
}

/// Serialize frontmatter findings as a JSON envelope string.
///
/// # Errors
///
/// Returns an error if JSON serialization fails.
fn format_json(findings: &[DocsFrontmatterFinding]) -> std::result::Result<String, Error> {
    let status = if has_fail_findings(findings) {
        "failed"
    } else {
        "passed"
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
            fail_count: count_severity(findings, "fail"),
            warn_count: count_severity(findings, "warn"),
            findings: jf,
        },
    };
    let mut s = serde_json::to_string_pretty(&env)?;
    s.push('\n');
    Ok(s)
}

/// Format frontmatter findings as a Markdown table.
fn format_markdown(findings: &[DocsFrontmatterFinding]) -> String {
    if findings.is_empty() {
        return "## Docs Frontmatter Validation\n\n**PASSED**: no findings\n".to_string();
    }
    let fail_n = count_severity(findings, "fail");
    let warn_n = count_severity(findings, "warn");
    let mut sb = String::new();
    if fail_n > 0 {
        let _ = writeln!(
            sb,
            "## Docs Frontmatter Validation\n\n**FAILED**: {fail_n} fail finding(s), {warn_n} warn finding(s)\n"
        );
    } else {
        let _ = writeln!(
            sb,
            "## Docs Frontmatter Validation\n\n**PASSED** with {warn_n} warn finding(s)\n"
        );
    }
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

    fn fail() -> DocsFrontmatterFinding {
        DocsFrontmatterFinding {
            file: "x.md".to_string(),
            severity: "fail".to_string(),
            kind: "missing-title".to_string(),
            message: "m".to_string(),
        }
    }

    fn warn() -> DocsFrontmatterFinding {
        DocsFrontmatterFinding {
            file: "y.md".to_string(),
            severity: "warn".to_string(),
            kind: "missing-description".to_string(),
            message: "m".to_string(),
        }
    }

    #[test]
    fn format_text_passed_empty() {
        assert!(format_text(&[]).starts_with("DOCS FRONTMATTER VALIDATION PASSED"));
    }

    #[test]
    fn format_text_failed() {
        let s = format_text(&[fail()]);
        assert!(s.contains("FAILED: 1 fail finding(s)"));
    }

    #[test]
    fn format_text_warn_only_passes() {
        let s = format_text(&[warn()]);
        assert!(s.contains("PASSED with 1 warn finding(s)"));
    }

    #[test]
    fn format_json_passed() {
        let s = format_json(&[]).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "passed");
        assert_eq!(v["result"]["fail_count"], 0);
    }

    #[test]
    fn format_json_failed() {
        let s = format_json(&[fail(), warn()]).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "failed");
        assert_eq!(v["result"]["fail_count"], 1);
        assert_eq!(v["result"]["warn_count"], 1);
    }

    #[test]
    fn format_markdown_passed_empty() {
        assert!(format_markdown(&[]).contains("**PASSED**: no findings"));
    }

    #[test]
    fn format_markdown_warn_only() {
        let s = format_markdown(&[warn()]);
        assert!(s.contains("**PASSED** with 1 warn finding(s)"));
    }

    #[test]
    fn format_markdown_failed() {
        let s = format_markdown(&[fail()]);
        assert!(s.contains("**FAILED**: 1 fail finding(s)"));
    }
}
