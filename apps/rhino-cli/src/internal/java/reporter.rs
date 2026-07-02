//! Output formatting for Java annotation validation results.
//!
//! JSON timestamps use the same RFC3339-with-offset shape as Go's `timeutil.Timestamp()`
//! and existing Phase-4 reporters.

use std::fmt::Write as _;

use anyhow::Error;
use serde::Serialize;

use super::types::{ValidationResult, ViolationType};

/// Returns the current timestamp in Go's `time.RFC3339` format with a numeric
/// offset, matching `timeutil.Timestamp()`.
fn timestamp() -> String {
    chrono::Local::now()
        .format("%Y-%m-%dT%H:%M:%S%:z")
        .to_string()
}

/// Formats the validation result as human-readable text. Each package is shown
/// with a `PASS` or `FAIL` prefix and its status, followed by a blank line and the
/// violation count summary.
pub fn format_text(result: &ValidationResult, _verbose: bool, quiet: bool) -> String {
    let mut out = String::new();

    for pkg in &result.all_packages {
        if pkg.valid {
            let _ = writeln!(
                out,
                "PASS {}\tpackage-info.java present, @{} found",
                pkg.package_dir, result.annotation
            );
        } else {
            match pkg.violation_type {
                Some(ViolationType::MissingPackageInfo) => {
                    let _ = writeln!(out, "FAIL {}\tpackage-info.java missing", pkg.package_dir);
                }
                Some(ViolationType::MissingAnnotation) => {
                    let _ = writeln!(
                        out,
                        "FAIL {}\tpackage-info.java present, @{} missing",
                        pkg.package_dir, result.annotation
                    );
                }
                None => {}
            }
        }
    }

    let num_violations = result.total_packages - result.valid_packages;
    if num_violations == 0 {
        if !quiet {
            let _ = write!(out, "\n0 violations found.\n");
        }
    } else {
        let _ = write!(out, "\n{num_violations} violation(s) found.\n");
    }

    out
}

/// A single null-safety violation entry in the JSON output envelope.
#[derive(Serialize)]
struct JsonViolation<'a> {
    /// Repo-relative directory of the offending package.
    package_dir: &'a str,
    /// Kind of violation (missing `package-info.java` or missing annotation).
    violation_type: &'a str,
}

/// JSON output envelope for the null-safety validation report.
#[derive(Serialize)]
struct JsonOutput<'a> {
    /// `"success"` when there are zero violations, `"failure"` otherwise.
    status: &'a str,
    /// RFC 3339 timestamp of when the report was generated.
    timestamp: String,
    /// Total number of packages scanned.
    total_packages: usize,
    /// Number of packages that passed validation.
    valid_packages: usize,
    /// Annotation name that was required.
    annotation: &'a str,
    /// The individual violations found.
    violations: Vec<JsonViolation<'a>>,
}

/// Formats the validation result as JSON (two-space indent, HTML escaping, no
/// trailing newline).
///
/// # Errors
///
/// Returns an error if JSON serialization fails.
pub fn format_json(result: &ValidationResult) -> Result<String, Error> {
    let num_violations = result.total_packages - result.valid_packages;
    let status = if num_violations > 0 {
        "failure"
    } else {
        "success"
    };

    let violations: Vec<JsonViolation> = result
        .all_packages
        .iter()
        .filter(|p| !p.valid)
        .map(|p| JsonViolation {
            package_dir: &p.package_dir,
            violation_type: p.violation_type.map_or("", ViolationType::code),
        })
        .collect();

    let out = JsonOutput {
        status,
        timestamp: timestamp(),
        total_packages: result.total_packages,
        valid_packages: result.valid_packages,
        annotation: &result.annotation,
        violations,
    };

    Ok(serde_json::to_string_pretty(&out)?)
}

/// Formats the validation result as a markdown report.
pub fn format_markdown(result: &ValidationResult) -> String {
    let mut out = String::new();

    let num_violations = result.total_packages - result.valid_packages;

    out.push_str("# Java Null Safety Validation Report\n\n");
    let _ = writeln!(out, "- **Annotation required**: `@{}`", result.annotation);
    let _ = writeln!(out, "- **Total packages**: {}", result.total_packages);
    let _ = writeln!(out, "- **Valid packages**: {}", result.valid_packages);
    let _ = writeln!(out, "- **Violations**: {num_violations}\n");

    if num_violations == 0 {
        out.push_str("PASS All packages have the required annotation.\n");
        return out;
    }

    out.push_str("## Violations\n\n");
    for pkg in &result.all_packages {
        if pkg.valid {
            continue;
        }
        match pkg.violation_type {
            Some(ViolationType::MissingPackageInfo) => {
                let _ = writeln!(out, "- `{}`: `package-info.java` missing", pkg.package_dir);
            }
            Some(ViolationType::MissingAnnotation) => {
                let _ = writeln!(
                    out,
                    "- `{}`: `package-info.java` present, `@{}` missing",
                    pkg.package_dir, result.annotation
                );
            }
            None => {}
        }
    }

    out
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::super::types::PackageEntry;
    use super::*;

    fn valid_entry(dir: &str) -> PackageEntry {
        PackageEntry {
            package_dir: dir.to_string(),
            valid: true,
            violation_type: None,
        }
    }

    fn invalid_entry(dir: &str, vt: ViolationType) -> PackageEntry {
        PackageEntry {
            package_dir: dir.to_string(),
            valid: false,
            violation_type: Some(vt),
        }
    }

    fn result_all_valid() -> ValidationResult {
        ValidationResult {
            total_packages: 1,
            valid_packages: 1,
            all_packages: vec![valid_entry("com/foo")],
            annotation: "NullMarked".to_string(),
        }
    }

    fn result_with_violations() -> ValidationResult {
        ValidationResult {
            total_packages: 2,
            valid_packages: 0,
            all_packages: vec![
                invalid_entry("com/bar", ViolationType::MissingPackageInfo),
                invalid_entry("com/foo", ViolationType::MissingAnnotation),
            ],
            annotation: "NullMarked".to_string(),
        }
    }

    #[test]
    fn text_valid_uses_tab_and_zero_summary() {
        let s = format_text(&result_all_valid(), false, false);
        assert_eq!(
            s,
            "PASS com/foo\tpackage-info.java present, @NullMarked found\n\n0 violations found.\n"
        );
    }

    #[test]
    fn text_quiet_suppresses_zero_summary() {
        let s = format_text(&result_all_valid(), false, true);
        assert_eq!(
            s,
            "PASS com/foo\tpackage-info.java present, @NullMarked found\n"
        );
    }

    #[test]
    fn text_violations_always_print_summary() {
        let s = format_text(&result_with_violations(), false, true);
        assert!(s.contains("FAIL com/bar\tpackage-info.java missing\n"));
        assert!(s.contains("FAIL com/foo\tpackage-info.java present, @NullMarked missing\n"));
        assert!(s.ends_with("\n2 violation(s) found.\n"));
    }

    #[test]
    fn json_success_no_violations() {
        let s = format_json(&result_all_valid()).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "success");
        assert_eq!(v["total_packages"], 1);
        assert_eq!(v["valid_packages"], 1);
        assert_eq!(v["annotation"], "NullMarked");
        assert!(v["violations"].as_array().unwrap().is_empty());
        assert!(!s.ends_with('\n'));
    }

    #[test]
    fn json_failure_lists_violations() {
        let s = format_json(&result_with_violations()).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "failure");
        assert_eq!(v["violations"][0]["package_dir"], "com/bar");
        assert_eq!(v["violations"][0]["violation_type"], "missing_package_info");
        assert_eq!(v["violations"][1]["violation_type"], "missing_annotation");
    }

    #[test]
    fn markdown_no_violations() {
        let s = format_markdown(&result_all_valid());
        assert!(s.starts_with("# Java Null Safety Validation Report\n\n"));
        assert!(s.contains("- **Annotation required**: `@NullMarked`\n"));
        assert!(s.ends_with("PASS All packages have the required annotation.\n"));
    }

    #[test]
    fn markdown_with_violations() {
        let s = format_markdown(&result_with_violations());
        assert!(s.contains("- **Violations**: 2\n"));
        assert!(s.contains("- `com/bar`: `package-info.java` missing\n"));
        assert!(s.contains("- `com/foo`: `package-info.java` present, `@NullMarked` missing\n"));
    }
}
