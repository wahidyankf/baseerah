//! Test-coverage subsystem.
//!
//! Parses coverage reports in LCOV, `JaCoCo` XML, Cobertura XML, and Go
//! `cover.out` formats, computes line-coverage results, supports diff-based and
//! merged coverage, and renders human-readable, JSON, and Markdown reports.
//! Backs the `test-coverage` command family.

/// Cobertura XML coverage format parser and result computer.
pub mod cobertura;
/// Coverage-format detection by filename and file content.
pub mod detect;
/// Diff-based coverage restricted to lines changed in a git diff.
pub mod diff;
/// File-exclusion helpers using Go `filepath.Match` glob semantics.
pub mod exclude;
/// Go `cover.out` format parser and result computer.
pub mod go_coverage;
/// `JaCoCo` XML coverage format parser and result computer.
pub mod jacoco;
/// LCOV format parser and result computer.
pub mod lcov;
/// `CoverageMap` merge utilities, LCOV serialisation, and format dispatch.
pub mod merge;
/// Human-readable, JSON, and Markdown coverage report formatters.
pub mod reporter;
/// Core types for the test-coverage subsystem.
pub mod types;
