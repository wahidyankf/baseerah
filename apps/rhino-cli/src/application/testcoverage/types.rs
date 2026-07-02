//! Core types for the test-coverage subsystem.
//!
//! Mirrors `apps/rhino-cli/internal/testcoverage/types.go`.

use serde::Serialize;

/// Identifies the coverage-report format used by a particular file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    /// Go `cover.out` (produced by `go test -coverprofile`).
    Go,
    /// LCOV info format (`.info` files, produced by lcov/geninfo).
    Lcov,
    /// `JaCoCo` XML format (produced by the `JaCoCo` Maven/Gradle plugin).
    Jacoco,
    /// Cobertura XML format (produced by coverage.py and many other tools).
    Cobertura,
    /// Synthetic format used when computing diff-based coverage.
    Diff,
}

impl Format {
    /// Returns the lowercase string code for this format.
    ///
    /// The code is used in JSON output and log messages to identify the
    /// format in a human-readable, machine-parseable way.
    pub fn code(self) -> &'static str {
        match self {
            Format::Go => "go",
            Format::Lcov => "lcov",
            Format::Jacoco => "jacoco",
            Format::Cobertura => "cobertura",
            Format::Diff => "diff",
        }
    }
}

/// Coverage statistics for a single source file.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct FileResult {
    /// Path to the source file as it appears in the coverage report.
    pub path: String,
    /// Number of lines with at least one execution and all branches covered.
    pub covered: usize,
    /// Number of lines executed but with at least one uncovered branch.
    pub partial: usize,
    /// Number of lines with zero executions.
    pub missed: usize,
    /// Total executable lines (`covered + partial + missed`).
    pub total: usize,
    /// Percentage of covered lines: `100.0 * covered / total`.
    pub pct: f64,
}

/// Aggregated coverage result for an entire coverage report.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Result {
    /// Path to the coverage report file that was parsed.
    pub file: String,
    /// Format of the coverage report (e.g. `Format::Go`, `Format::Lcov`).
    pub format: Format,
    /// Total number of fully covered lines across all files.
    pub covered: usize,
    /// Total number of partially covered lines (executed, but branches missed).
    pub partial: usize,
    /// Total number of uncovered lines across all files.
    pub missed: usize,
    /// Total executable lines (`covered + partial + missed`).
    pub total: usize,
    /// Overall coverage percentage: `100.0 * covered / total`.
    pub pct: f64,
    /// Minimum coverage percentage required for the result to pass.
    pub threshold: f64,
    /// `true` when `pct >= threshold`.
    pub passed: bool,
    /// Per-file breakdown; omitted from JSON when empty.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<FileResult>,
}
