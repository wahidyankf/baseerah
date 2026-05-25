//! Test coverage analysis for the `rhino-cli` tool.
//!
//! This module provides a byte-for-byte port of the Go `testcoverage` package.
//! It supports parsing and aggregating coverage data from multiple formats
//! (`Go`, `LCOV`, `JaCoCo`, `Cobertura`) and computing diff-based coverage.
//!
//! Sub-modules follow the same file layout as the original Go source under
//! `apps/rhino-cli/internal/testcoverage/`.

/// Cobertura XML coverage format parser and result computer.
pub mod cobertura;
/// Automatic coverage-format detector by filename and content heuristics.
pub mod detect;
/// Diff-based coverage: measures coverage of lines changed in a git diff.
pub mod diff;
/// File-exclusion helpers using Go `filepath.Match` glob semantics.
pub mod exclude;
/// Go `cover.out` format parser and result computer.
pub mod go_coverage;
/// JaCoCo XML coverage format parser and result computer.
pub mod jacoco;
/// LCOV format parser and result computer.
pub mod lcov;
/// `CoverageMap` merge, LCOV serialisation, and format-dispatch helpers.
pub mod merge;
/// Human-readable, JSON, and Markdown coverage report formatters.
pub mod reporter;
/// Core types: `Format`, `FileResult`, and `Result`.
pub mod types;
