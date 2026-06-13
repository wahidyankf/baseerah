//! Human-readable, JSON, and Markdown coverage report formatters.
//!
//! Byte-for-byte port of `apps/rhino-cli/internal/testcoverage/reporter.go`.
//! Format strings match exactly so shadow-diff against the Go binary passes.

use std::fmt::Write as _;

use anyhow::Error;
use serde::{Serialize, Serializer};

use super::types::{FileResult, Result as CoverageResult};

/// Serialises an `f64` the way Go's `encoding/json` does.
///
/// Whole-number floats render without a trailing `.0`
/// (e.g. `90.0` → `"90"`, but `86.08` → `"86.08"`).
#[allow(clippy::trivially_copy_pass_by_ref, clippy::cast_possible_truncation)]
fn serialize_f64_gostyle<S: Serializer>(value: &f64, s: S) -> Result<S::Ok, S::Error> {
    if value.fract() == 0.0 && value.is_finite() && value.abs() < 1e15 {
        s.serialize_i64(*value as i64)
    } else {
        s.serialize_f64(*value)
    }
}

/// JSON-serialisable view of a `FileResult` using Go-style `f64` serialisation.
#[derive(Serialize)]
struct FileResultJson {
    /// Source-file path as it appears in the coverage report.
    path: String,
    /// Number of fully covered lines.
    covered: usize,
    /// Number of partially covered lines.
    partial: usize,
    /// Number of uncovered lines.
    missed: usize,
    /// Total executable lines.
    total: usize,
    /// Coverage percentage serialised without trailing `.0` for whole numbers.
    #[serde(serialize_with = "serialize_f64_gostyle")]
    pct: f64,
}

impl From<&FileResult> for FileResultJson {
    fn from(f: &FileResult) -> Self {
        FileResultJson {
            path: f.path.clone(),
            covered: f.covered,
            partial: f.partial,
            missed: f.missed,
            total: f.total,
            pct: f.pct,
        }
    }
}

/// Human-readable coverage report. Mirrors Go's `FormatText`.
/// Output:
///   "Line coverage: 86.08% (2411 covered, 141 partial, 249 missed, 2801 total)"
///   "PASS: 86.08% >= 85% threshold"   (or FAIL: ... < ... threshold)
pub fn format_text(r: &CoverageResult, _verbose: bool, _quiet: bool) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "Line coverage: {:.2}% ({} covered, {} partial, {} missed, {} total)",
        r.pct, r.covered, r.partial, r.missed, r.total
    );
    if r.passed {
        let _ = writeln!(out, "PASS: {:.2}% >= {:.0}% threshold", r.pct, r.threshold);
    } else {
        let _ = writeln!(out, "FAIL: {:.2}% < {:.0}% threshold", r.pct, r.threshold);
    }
    out
}

/// Formats a per-file coverage table as plain text.
///
/// Only includes files whose coverage percentage is below `below_threshold`
/// (all files are included when `below_threshold` is `0.0`). Files are sorted
/// by ascending coverage percentage.
pub fn format_text_per_file(r: &CoverageResult, below_threshold: f64) -> String {
    let files = filter_and_sort_files(&r.files, below_threshold);
    if files.is_empty() {
        return "No files to report.\n".to_string();
    }
    let mut out = String::new();
    let _ = writeln!(out, "\nPer-file coverage ({} files):", files.len());
    for f in &files {
        let _ = writeln!(
            out,
            "  {:>6.2}%  {} ({} covered, {} partial, {} missed)",
            f.pct, f.path, f.covered, f.partial, f.missed
        );
    }
    out
}

/// Filters `files` to those whose `pct` is below `below_threshold`, then sorts
/// them by ascending `pct`. When `below_threshold` is `0.0` no filtering is applied.
pub(crate) fn filter_and_sort_files(files: &[FileResult], below_threshold: f64) -> Vec<FileResult> {
    let mut result: Vec<FileResult> = files
        .iter()
        .filter(|f| !(below_threshold > 0.0 && f.pct >= below_threshold))
        .cloned()
        .collect();
    result.sort_by(|a, b| {
        a.pct
            .partial_cmp(&b.pct)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    result
}

/// Top-level JSON output structure mirroring Go's `reporter.go` JSON layout.
#[derive(Serialize)]
struct JsonOutput {
    /// `"success"` or `"failure"`.
    status: String,
    /// RFC 3339 timestamp with local timezone offset (second precision).
    timestamp: String,
    /// Path to the coverage report file.
    file: String,
    /// Coverage format code (e.g. `"go"`, `"lcov"`).
    format: String,
    /// Total fully covered lines.
    covered: usize,
    /// Total partially covered lines.
    partial: usize,
    /// Total uncovered lines.
    missed: usize,
    /// Total executable lines.
    total: usize,
    /// Overall coverage percentage (Go-style serialisation).
    #[serde(serialize_with = "serialize_f64_gostyle")]
    pct: f64,
    /// Coverage threshold (Go-style serialisation).
    #[serde(serialize_with = "serialize_f64_gostyle")]
    threshold: f64,
    /// `true` when `pct >= threshold`.
    passed: bool,
    /// Per-file breakdown; omitted from JSON when empty.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    files: Vec<FileResultJson>,
}

/// Formats `r` as a pretty-printed JSON string.
///
/// When `per_file` is `true` and `r.files` is non-empty, the output includes a
/// per-file breakdown filtered to files below `below_threshold` (all files when
/// `below_threshold` is `0.0`).
///
/// # Errors
///
/// Returns an error when JSON serialisation fails (should not happen in practice).
pub fn format_json(
    r: &CoverageResult,
    per_file: bool,
    below_threshold: f64,
) -> std::result::Result<String, Error> {
    let status = if r.passed { "success" } else { "failure" };
    let files: Vec<FileResultJson> = if per_file && !r.files.is_empty() {
        filter_and_sort_files(&r.files, below_threshold)
            .iter()
            .map(FileResultJson::from)
            .collect()
    } else {
        Vec::new()
    };
    // Go's time.RFC3339 with time.Now() → local timezone with offset, second precision.
    let timestamp = chrono::Local::now()
        .format("%Y-%m-%dT%H:%M:%S%:z")
        .to_string();
    let out = JsonOutput {
        status: status.to_string(),
        timestamp,
        file: r.file.clone(),
        format: r.format.code().to_string(),
        covered: r.covered,
        partial: r.partial,
        missed: r.missed,
        total: r.total,
        pct: r.pct,
        threshold: r.threshold,
        passed: r.passed,
        files,
    };
    Ok(serde_json::to_string_pretty(&out)?)
}

/// Formats `r` as a Markdown coverage report.
///
/// Always includes a summary metric table. When `per_file` is `true` and
/// `r.files` is non-empty, appends a per-file breakdown table filtered to
/// files below `below_threshold` (all files when `below_threshold` is `0.0`).
pub fn format_markdown(r: &CoverageResult, per_file: bool, below_threshold: f64) -> String {
    let status = if r.passed { "PASS" } else { "FAIL" };
    let mut out = format!(
        "## Coverage Report\n\n\
         | Metric | Value |\n\
         | --- | --- |\n\
         | File | {} |\n\
         | Format | {} |\n\
         | Line Coverage | {:.2}% |\n\
         | Threshold | {:.0}% |\n\
         | Covered | {} |\n\
         | Partial | {} |\n\
         | Missed | {} |\n\
         | Total | {} |\n\
         | Status | **{}** |\n",
        r.file,
        r.format.code(),
        r.pct,
        r.threshold,
        r.covered,
        r.partial,
        r.missed,
        r.total,
        status
    );

    if per_file && !r.files.is_empty() {
        let files = filter_and_sort_files(&r.files, below_threshold);
        if !files.is_empty() {
            out.push_str("\n### Per-File Breakdown\n\n");
            out.push_str("| Coverage | File | Covered | Partial | Missed |\n");
            out.push_str("| --- | --- | --- | --- | --- |\n");
            for f in &files {
                let _ = writeln!(
                    out,
                    "| {:.2}% | {} | {} | {} | {} |",
                    f.pct, f.path, f.covered, f.partial, f.missed
                );
            }
        }
    }

    out
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::internal::testcoverage::types::Format;

    fn sample_result(passed: bool) -> CoverageResult {
        CoverageResult {
            file: "apps/rhino-cli/cover.out".to_string(),
            format: Format::Go,
            covered: 2411,
            partial: 141,
            missed: 249,
            total: 2801,
            pct: 86.08,
            threshold: 85.0,
            passed,
            files: vec![],
        }
    }

    #[test]
    fn format_text_pass_matches_go_exact_string() {
        let r = sample_result(true);
        let s = format_text(&r, false, false);
        assert_eq!(
            s,
            "Line coverage: 86.08% (2411 covered, 141 partial, 249 missed, 2801 total)\n\
             PASS: 86.08% >= 85% threshold\n"
        );
    }

    #[test]
    fn format_text_fail_matches_go_exact_string() {
        let mut r = sample_result(false);
        r.pct = 80.0;
        let s = format_text(&r, false, false);
        assert!(s.contains("FAIL: 80.00% < 85% threshold"));
    }

    #[test]
    fn format_json_parseable_and_has_status() {
        let r = sample_result(true);
        let s = format_json(&r, false, 0.0).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["status"], "success");
        assert_eq!(v["passed"], true);
        assert_eq!(v["format"], "go");
        assert!(v["files"].is_null() || v["files"].as_array().unwrap().is_empty());
    }

    #[test]
    fn format_markdown_has_metric_table() {
        let r = sample_result(true);
        let md = format_markdown(&r, false, 0.0);
        assert!(md.contains("## Coverage Report"));
        assert!(md.contains("| Line Coverage | 86.08% |"));
        assert!(md.contains("| Status | **PASS** |"));
    }

    #[test]
    fn filter_and_sort_files_sorts_ascending() {
        let files = vec![
            FileResult {
                path: "a.rs".into(),
                covered: 0,
                partial: 0,
                missed: 0,
                total: 0,
                pct: 80.0,
            },
            FileResult {
                path: "b.rs".into(),
                covered: 0,
                partial: 0,
                missed: 0,
                total: 0,
                pct: 50.0,
            },
            FileResult {
                path: "c.rs".into(),
                covered: 0,
                partial: 0,
                missed: 0,
                total: 0,
                pct: 95.0,
            },
        ];
        let sorted = filter_and_sort_files(&files, 0.0);
        assert_eq!(sorted[0].path, "b.rs");
        assert_eq!(sorted[1].path, "a.rs");
        assert_eq!(sorted[2].path, "c.rs");
    }

    #[test]
    fn filter_and_sort_files_below_threshold_excludes_above() {
        let files = vec![
            FileResult {
                path: "low.rs".into(),
                covered: 0,
                partial: 0,
                missed: 0,
                total: 0,
                pct: 70.0,
            },
            FileResult {
                path: "high.rs".into(),
                covered: 0,
                partial: 0,
                missed: 0,
                total: 0,
                pct: 95.0,
            },
        ];
        let filtered = filter_and_sort_files(&files, 80.0);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].path, "low.rs");
    }
}
