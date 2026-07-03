//! `specs gherkin-cardinality` — flags scenarios that use a
//! primary `Given`/`When`/`Then` keyword more than once.
//!
//! Sibling pattern: `governance_emoji_audit.rs`.

use std::fmt::Write as _;
use std::path::Path;

use anyhow::{Context, Error, anyhow};
use clap::Args;
use serde::Serialize;

use crate::domain::cliout::OutputFormat;
use crate::infrastructure::fs::real::RealFs;
use crate::internal::git;
use crate::internal::repo_governance::gherkin_keyword_cardinality_audit::{
    GherkinCardinalityFinding, audit_gherkin_keyword_cardinality,
};

/// JSON output schema identifier for this command.
const SCHEMA: &str = "rhino-cli/gherkin-keyword-cardinality/v1";

/// CLI arguments for `repo-governance gherkin-keyword-cardinality`.
#[derive(Args, Debug)]
pub struct GherkinKeywordCardinalityArgs {
    /// Paths to scan (repeatable; relative to git root).
    #[arg(short = 'p', long = "path", value_name = "PATH")]
    pub path: Vec<String>,
    /// Positional path overrides — same effect as --path.
    pub positional: Vec<String>,
}

/// Single cardinality finding in JSON output.
#[derive(Serialize)]
struct FindingJson<'a> {
    /// Path of the `.feature` file containing the violation.
    file: &'a str,
    /// Line number of the scenario declaration.
    line: usize,
    /// Name of the offending scenario.
    scenario: &'a str,
    /// Primary keyword that appears more than once.
    keyword: &'a str,
    /// Number of primary occurrences of the keyword.
    count: usize,
    /// Severity label.
    severity: &'a str,
}

/// JSON envelope wrapping the cardinality audit result.
#[derive(Serialize)]
struct Envelope<'a> {
    /// Output schema identifier.
    schema: &'a str,
    /// `"passed"` or `"failed"`.
    status: &'a str,
    /// Individual findings.
    result: Vec<FindingJson<'a>>,
}

/// Run the `repo-governance gherkin-keyword-cardinality` command.
///
/// # Errors
///
/// Returns an error if the git root cannot be found, the audit fails, or
/// cardinality findings are detected.
pub fn run(
    args: &GherkinKeywordCardinalityArgs,
    output_format: OutputFormat,
) -> std::result::Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;

    let full_paths = resolve_scan_paths(&args.positional, &args.path, &repo_root);

    let findings = audit_gherkin_keyword_cardinality(&RealFs, &full_paths)
        .context("gherkin keyword cardinality audit failed")?;

    match output_format {
        OutputFormat::Text => print!("{}", format_text(&findings)),
        OutputFormat::Json => print!("{}", format_json(&findings)?),
        OutputFormat::Markdown => print!("{}", format_markdown(&findings)),
    }

    if !findings.is_empty() {
        return Err(anyhow!(
            "{} gherkin keyword cardinality finding(s) found",
            findings.len()
        ));
    }
    Ok(())
}

/// Resolves the scan roots from CLI arguments.
///
/// Positional paths win over `--path` flags; when both are empty the whole
/// repository (`.`) is scanned. Relative paths are joined to `repo_root`;
/// absolute paths are kept as-is.
fn resolve_scan_paths(positional: &[String], flags: &[String], repo_root: &Path) -> Vec<String> {
    let default_paths = [".".to_string()];
    let rel_paths: &[String] = if !positional.is_empty() {
        positional
    } else if !flags.is_empty() {
        flags
    } else {
        &default_paths
    };
    rel_paths
        .iter()
        .map(|p| {
            if Path::new(p).is_absolute() {
                p.clone()
            } else {
                repo_root.join(p).to_string_lossy().to_string()
            }
        })
        .collect()
}

/// Format cardinality findings as human-readable text.
fn format_text(findings: &[GherkinCardinalityFinding]) -> String {
    if findings.is_empty() {
        return "GHERKIN KEYWORD CARDINALITY AUDIT PASSED: every scenario uses each primary keyword at most once\n"
            .to_string();
    }
    let mut sb = String::new();
    let _ = writeln!(
        sb,
        "GHERKIN KEYWORD CARDINALITY AUDIT FAILED: {} violation(s) found",
        findings.len()
    );
    for f in findings {
        let _ = writeln!(
            sb,
            "  {}:{}  [{}]  scenario '{}' uses primary '{}' {} times (chain extras with And/But)",
            f.file, f.line, f.severity, f.scenario, f.keyword, f.count
        );
    }
    sb
}

/// Serialize cardinality findings as a JSON envelope string.
///
/// # Errors
///
/// Returns an error if JSON serialization fails.
fn format_json(findings: &[GherkinCardinalityFinding]) -> std::result::Result<String, Error> {
    let jf: Vec<FindingJson> = findings
        .iter()
        .map(|f| FindingJson {
            file: &f.file,
            line: f.line,
            scenario: &f.scenario,
            keyword: &f.keyword,
            count: f.count,
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

/// Format cardinality findings as a Markdown table.
fn format_markdown(findings: &[GherkinCardinalityFinding]) -> String {
    if findings.is_empty() {
        return "## Gherkin Keyword Cardinality Audit\n\n**PASSED**: every scenario uses each primary keyword at most once\n"
            .to_string();
    }
    let mut sb = String::new();
    let _ = writeln!(
        sb,
        "## Gherkin Keyword Cardinality Audit\n\n**FAILED**: {} violation(s) found\n",
        findings.len()
    );
    sb.push_str("| File | Line | Scenario | Keyword | Count | Severity |\n");
    sb.push_str("|------|------|----------|---------|------:|----------|\n");
    for f in findings {
        let _ = writeln!(
            sb,
            "| {} | {} | {} | {} | {} | {} |",
            f.file, f.line, f.scenario, f.keyword, f.count, f.severity
        );
    }
    sb
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    fn sample() -> GherkinCardinalityFinding {
        GherkinCardinalityFinding {
            file: "specs/x.feature".to_string(),
            line: 3,
            scenario: "Double when".to_string(),
            keyword: "When".to_string(),
            count: 2,
            severity: "high".to_string(),
        }
    }

    #[test]
    fn format_text_passes_when_no_findings() {
        let s = format_text(&[]);
        assert!(s.starts_with("GHERKIN KEYWORD CARDINALITY AUDIT PASSED"));
    }

    #[test]
    fn format_text_fails_with_findings() {
        let s = format_text(&[sample()]);
        assert!(s.contains("GHERKIN KEYWORD CARDINALITY AUDIT FAILED: 1"));
        assert!(s.contains("specs/x.feature:3"));
        assert!(s.contains("'Double when'"));
        assert!(s.contains("'When' 2 times"));
    }

    #[test]
    fn format_json_status_passed_on_empty() {
        let s = format_json(&[]).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "passed");
        assert_eq!(v["schema"], SCHEMA);
    }

    #[test]
    fn format_json_status_failed_on_findings() {
        let s = format_json(&[sample()]).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "failed");
        assert_eq!(v["result"][0]["keyword"], "When");
        assert_eq!(v["result"][0]["count"], 2);
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
        assert!(s.contains("| File | Line | Scenario | Keyword | Count | Severity |"));
        assert!(s.contains("| specs/x.feature | 3 | Double when | When | 2 | high |"));
    }

    #[test]
    fn resolve_scan_paths_positional_wins_over_flags() {
        let root = Path::new("/repo");
        let r = resolve_scan_paths(&["specs/apps".to_string()], &["ignored".to_string()], root);
        assert_eq!(r, vec!["/repo/specs/apps".to_string()]);
    }

    #[test]
    fn resolve_scan_paths_flags_when_no_positional() {
        let root = Path::new("/repo");
        let r = resolve_scan_paths(&[], &["specs/libs".to_string()], root);
        assert_eq!(r, vec!["/repo/specs/libs".to_string()]);
    }

    #[test]
    fn resolve_scan_paths_defaults_to_repo_root() {
        let root = Path::new("/repo");
        let r = resolve_scan_paths(&[], &[], root);
        assert_eq!(r, vec!["/repo/.".to_string()]);
    }

    #[test]
    fn resolve_scan_paths_keeps_absolute() {
        let root = Path::new("/repo");
        let r = resolve_scan_paths(&["/abs/specs".to_string()], &[], root);
        assert_eq!(r, vec!["/abs/specs".to_string()]);
    }
}
