//! `md validate-naming` — checks that markdown file names follow kebab-case conventions.
//!
//! Port of `apps/rhino-cli/cmd/docs_validate_naming.go`.

use std::fmt::Write as _;
use std::path::Path;

use anyhow::{Context, Error, anyhow};
use clap::Args;
use serde::Serialize;

use crate::domain::cliout::OutputFormat;
use crate::internal::docs::naming::{DocsNamingFinding, validate_docs_naming};
use crate::internal::git;

/// JSON output schema identifier for this command.
const SCHEMA: &str = "rhino-cli/docs-validate-naming/v1";

/// Default paths scanned when no positional arguments are supplied.
const DEFAULT_PATHS: &[&str] = &["docs/", "repo-governance/"];

/// CLI arguments for `docs validate-naming`.
#[derive(Args, Debug)]
pub struct ValidateNamingArgs {
    /// Basename glob to exempt from the kebab-case rule (repeatable).
    #[arg(long = "exempt")]
    pub exempt: Vec<String>,
    /// Optional positional paths (override defaults).
    pub positional: Vec<String>,
}

/// Single naming finding in JSON output.
#[derive(Serialize)]
struct JsonFinding<'a> {
    /// Path of the offending file.
    file: &'a str,
    /// Severity label.
    severity: &'a str,
    /// Human-readable description.
    message: &'a str,
}

/// JSON envelope wrapping the naming scan result.
#[derive(Serialize)]
struct Envelope<'a> {
    /// Output schema identifier.
    schema: &'a str,
    /// `"passed"` or `"failed"`.
    status: &'a str,
    /// Individual findings.
    result: Vec<JsonFinding<'a>>,
}

/// Run the `docs validate-naming` command.
///
/// # Errors
///
/// Returns an error if the git root cannot be found, the scan fails, or
/// naming violations are found.
pub fn run(
    args: &ValidateNamingArgs,
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
        validate_docs_naming(&full_paths, &args.exempt).context("docs validate-naming failed")?;

    match output_format {
        OutputFormat::Text => print!("{}", format_text(&findings)),
        OutputFormat::Json => print!("{}", format_json(&findings)?),
        OutputFormat::Markdown => print!("{}", format_markdown(&findings)),
    }

    if !findings.is_empty() {
        return Err(anyhow!("{} docs naming finding(s) found", findings.len()));
    }
    Ok(())
}

/// Format naming findings as human-readable text.
fn format_text(findings: &[DocsNamingFinding]) -> String {
    if findings.is_empty() {
        return "DOCS NAMING VALIDATION PASSED: no naming violations found\n".to_string();
    }
    let mut sb = String::new();
    let _ = writeln!(
        sb,
        "DOCS NAMING VALIDATION FAILED: {} violation(s) found",
        findings.len()
    );
    for f in findings {
        let _ = writeln!(sb, "  {}  [{}]  {}", f.file, f.severity, f.message);
    }
    sb
}

/// Serialize naming findings as a JSON envelope string.
///
/// # Errors
///
/// Returns an error if JSON serialization fails.
fn format_json(findings: &[DocsNamingFinding]) -> std::result::Result<String, Error> {
    let jf: Vec<JsonFinding> = findings
        .iter()
        .map(|f| JsonFinding {
            file: &f.file,
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

/// Format naming findings as a Markdown table.
fn format_markdown(findings: &[DocsNamingFinding]) -> String {
    if findings.is_empty() {
        return "## Docs Filename Naming Validation\n\n**PASSED**: no naming violations found\n"
            .to_string();
    }
    let mut sb = String::new();
    let _ = writeln!(
        sb,
        "## Docs Filename Naming Validation\n\n**FAILED**: {} violation(s) found\n",
        findings.len()
    );
    sb.push_str("| File | Severity | Message |\n");
    sb.push_str("|------|----------|---------|\n");
    for f in findings {
        let _ = writeln!(sb, "| {} | {} | {} |", f.file, f.severity, f.message);
    }
    sb
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    fn sample() -> DocsNamingFinding {
        DocsNamingFinding {
            file: "x.md".to_string(),
            severity: "high".to_string(),
            message: "msg".to_string(),
        }
    }

    #[test]
    fn format_text_passed() {
        assert!(format_text(&[]).starts_with("DOCS NAMING VALIDATION PASSED"));
    }

    #[test]
    fn format_text_failed() {
        let s = format_text(&[sample()]);
        assert!(s.contains("DOCS NAMING VALIDATION FAILED: 1"));
        assert!(s.contains("x.md  [high]"));
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
        assert_eq!(v["result"][0]["file"], "x.md");
    }

    #[test]
    fn format_markdown_passed() {
        assert!(format_markdown(&[]).contains("**PASSED**"));
    }

    #[test]
    fn format_markdown_failed() {
        let s = format_markdown(&[sample()]);
        assert!(s.contains("**FAILED**: 1"));
        assert!(s.contains("| x.md | high | msg |"));
    }
}
