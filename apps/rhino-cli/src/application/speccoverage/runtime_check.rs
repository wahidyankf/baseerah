//! Runtime cross-check for `specs behavior-coverage validate`.
//!
//! Reads each tier's machine-readable run report and asserts every
//! `@covers`-marked scenario at that tier actually executed *and passed* —
//! closing the gap neither the legacy step-text traceability check in
//! [`super::checker`] nor the marker-existence engine in
//! [`crate::application::behavior_coverage::validator`] can see: both only
//! prove a matching implementation *exists* somewhere, never that it *ran*.
//! A `@covers` marker can sit next to a test that is `.skip`'d, `.only`'d
//! away, `.todo`'d, or undefined at runtime, and both of those checks would
//! still report the scenario as covered.
//!
//! Reuses the marker-extraction types and logic already built in
//! [`crate::application::behavior_coverage`] rather than duplicating them —
//! that engine is correct, just previously unreachable from the live
//! `specs behavior-coverage validate` command.

use std::fs;
use std::path::Path;

use anyhow::{Context as _, Error};

use crate::application::behavior_coverage::extract::extract_covers_markers;
use crate::application::behavior_coverage::types::{
    CoversMarker, RunReportEntry, RunStatus, RuntimeCoverageViolation, TestLevel,
};

/// One tier's marker source directory paired with its optional run-report
/// path.
///
/// A tier with `run_report: None` is skipped entirely by [`check_runtime`] —
/// the cross-check is opt-in per invocation via the `--<level>-report`
/// flags, so every existing invocation of `specs behavior-coverage validate`
/// that predates this cross-check (none of which supply a report path) sees
/// no behaviour change.
pub struct TierInput<'a> {
    /// The test level this tier represents.
    pub level: TestLevel,
    /// Directory containing this tier's test/marker source files.
    pub source_dir: &'a Path,
    /// Path to this tier's machine-readable run-report JSON, if supplied.
    pub run_report: Option<&'a Path>,
    /// Repository root, used to compute repo-relative marker source paths.
    pub repo_root: &'a Path,
}

/// Parses a tier's machine-readable run-report file into [`RunReportEntry`]
/// values.
///
/// Every test-runner ecosystem normalises its own native report (Jest/Vitest
/// JSON, Playwright JSON, cucumber-rs output, .NET TRX/JSON, etc.) to the
/// flat [`RunReportEntry`] shape before this trait ever sees it — factoring
/// parsing behind this trait, rather than hard-coding one format inside
/// [`check_runtime`], is what lets a later per-project rollout phase add a
/// non-JSON parser (e.g. a `.NET` TRX reader) without touching the
/// cross-check logic itself.
pub trait RunReportParser {
    /// Parses the run-report file at `path`.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or its contents do not
    /// match the expected shape.
    fn parse(&self, path: &Path) -> Result<Vec<RunReportEntry>, Error>;
}

/// Parses the flat JSON run-report shape described on [`RunReportEntry`] — a
/// bare JSON array of `{feature_path, scenario_title, status}` objects.
///
/// The only [`RunReportParser`] the engine itself ships with: every fixture
/// and consumer built so far normalises directly to this shape.
pub struct JsonRunReportParser;

impl RunReportParser for JsonRunReportParser {
    fn parse(&self, path: &Path) -> Result<Vec<RunReportEntry>, Error> {
        let content = fs::read_to_string(path)?;
        let entries: Vec<RunReportEntry> = serde_json::from_str(&content)?;
        Ok(entries)
    }
}

/// Runs the runtime cross-check across `tiers`, returning every scenario
/// whose `@covers` marker names a test that did not execute-and-pass at its
/// declared level.
///
/// Uses [`JsonRunReportParser`] for every tier. Call [`check_runtime_with`]
/// directly to supply a different [`RunReportParser`] per tier.
///
/// # Errors
///
/// Returns an error if a marker directory cannot be walked, or a supplied
/// run-report file cannot be read or parsed as JSON.
pub fn check_runtime(tiers: &[TierInput<'_>]) -> Result<Vec<RuntimeCoverageViolation>, Error> {
    check_runtime_with(tiers, &JsonRunReportParser)
}

/// Runs the runtime cross-check across `tiers` using `parser` to read every
/// tier's run-report file.
///
/// # Errors
///
/// Returns an error if a marker directory cannot be walked, or a supplied
/// run-report file cannot be read or parsed.
pub fn check_runtime_with(
    tiers: &[TierInput<'_>],
    parser: &dyn RunReportParser,
) -> Result<Vec<RuntimeCoverageViolation>, Error> {
    let mut violations = Vec::new();

    for tier in tiers {
        let Some(report_path) = tier.run_report else {
            continue;
        };
        let markers = extract_covers_markers(tier.source_dir, tier.level, tier.repo_root)
            .with_context(|| {
                format!("scanning {} for @covers markers", tier.source_dir.display())
            })?;
        if markers.is_empty() {
            continue;
        }
        let report = parser
            .parse(report_path)
            .with_context(|| format!("reading run report {}", report_path.display()))?;

        for marker in &markers {
            if let Some(v) = check_marker(marker, &report) {
                violations.push(v);
            }
        }
    }

    Ok(violations)
}

/// Cross-checks a single marker against the entries in `report`, returning a
/// violation when the marker's scenario is missing from the report (never
/// executed) or present with a non-passing status.
fn check_marker(
    marker: &CoversMarker,
    report: &[RunReportEntry],
) -> Option<RuntimeCoverageViolation> {
    let entry = report.iter().find(|e| {
        e.feature_path == marker.feature_path && e.scenario_title == marker.scenario_title
    });

    match entry {
        None => Some(RuntimeCoverageViolation::NotExecuted {
            source_file: marker.source_file.clone(),
            feature_path: marker.feature_path.clone(),
            scenario_title: marker.scenario_title.clone(),
            level: marker.level,
        }),
        Some(e) if e.status != RunStatus::Passed => Some(RuntimeCoverageViolation::Failed {
            source_file: marker.source_file.clone(),
            feature_path: marker.feature_path.clone(),
            scenario_title: marker.scenario_title.clone(),
            level: marker.level,
        }),
        Some(_) => None,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn write(dir: &Path, rel: &str, content: &str) {
        let p = dir.join(rel);
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(p, content).unwrap();
    }

    #[test]
    fn marker_with_no_report_entry_is_not_executed() {
        let tmp = TempDir::new().unwrap();
        write(
            tmp.path(),
            "unit/test.rs",
            "// @covers specs/x.feature:Logs in\n",
        );
        write(tmp.path(), "report.json", "[]");
        let tiers = [TierInput {
            level: TestLevel::Unit,
            source_dir: &tmp.path().join("unit"),
            run_report: Some(&tmp.path().join("report.json")),
            repo_root: tmp.path(),
        }];
        let violations = check_runtime(&tiers).unwrap();
        assert_eq!(violations.len(), 1);
        assert!(matches!(
            violations[0],
            RuntimeCoverageViolation::NotExecuted { .. }
        ));
    }

    #[test]
    fn marker_with_failed_report_entry_is_failed() {
        let tmp = TempDir::new().unwrap();
        write(
            tmp.path(),
            "unit/test.rs",
            "// @covers specs/x.feature:Logs in\n",
        );
        write(
            tmp.path(),
            "report.json",
            r#"[{"feature_path":"specs/x.feature","scenario_title":"Logs in","status":"failed"}]"#,
        );
        let tiers = [TierInput {
            level: TestLevel::Unit,
            source_dir: &tmp.path().join("unit"),
            run_report: Some(&tmp.path().join("report.json")),
            repo_root: tmp.path(),
        }];
        let violations = check_runtime(&tiers).unwrap();
        assert_eq!(violations.len(), 1);
        assert!(matches!(
            violations[0],
            RuntimeCoverageViolation::Failed { .. }
        ));
    }

    #[test]
    fn marker_with_passed_report_entry_has_no_violation() {
        let tmp = TempDir::new().unwrap();
        write(
            tmp.path(),
            "unit/test.rs",
            "// @covers specs/x.feature:Logs in\n",
        );
        write(
            tmp.path(),
            "report.json",
            r#"[{"feature_path":"specs/x.feature","scenario_title":"Logs in","status":"passed"}]"#,
        );
        let tiers = [TierInput {
            level: TestLevel::Unit,
            source_dir: &tmp.path().join("unit"),
            run_report: Some(&tmp.path().join("report.json")),
            repo_root: tmp.path(),
        }];
        let violations = check_runtime(&tiers).unwrap();
        assert!(violations.is_empty());
    }

    #[test]
    fn tier_with_no_report_is_skipped_entirely() {
        let tmp = TempDir::new().unwrap();
        write(
            tmp.path(),
            "unit/test.rs",
            "// @covers specs/x.feature:Logs in\n",
        );
        let tiers = [TierInput {
            level: TestLevel::Unit,
            source_dir: &tmp.path().join("unit"),
            run_report: None,
            repo_root: tmp.path(),
        }];
        let violations = check_runtime(&tiers).unwrap();
        assert!(violations.is_empty());
    }

    #[test]
    fn tier_with_no_markers_skips_report_loading() {
        // No @covers markers in source_dir → report is never even read, so an
        // invalid/missing report path at this tier is harmless.
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("unit")).unwrap();
        let tiers = [TierInput {
            level: TestLevel::Unit,
            source_dir: &tmp.path().join("unit"),
            run_report: Some(&tmp.path().join("does-not-exist.json")),
            repo_root: tmp.path(),
        }];
        let violations = check_runtime(&tiers).unwrap();
        assert!(violations.is_empty());
    }

    /// A non-JSON stand-in [`RunReportParser`] — proves [`check_runtime_with`]
    /// is genuinely pluggable, not hard-wired to [`JsonRunReportParser`].
    struct AlwaysPassedParser;

    impl RunReportParser for AlwaysPassedParser {
        fn parse(&self, _path: &Path) -> Result<Vec<RunReportEntry>, Error> {
            Ok(vec![RunReportEntry {
                feature_path: "specs/x.feature".to_string(),
                scenario_title: "Logs in".to_string(),
                status: RunStatus::Passed,
            }])
        }
    }

    #[test]
    fn check_runtime_with_accepts_a_custom_parser() {
        let tmp = TempDir::new().unwrap();
        write(
            tmp.path(),
            "unit/test.rs",
            "// @covers specs/x.feature:Logs in\n",
        );
        let tiers = [TierInput {
            level: TestLevel::Unit,
            source_dir: &tmp.path().join("unit"),
            // The path itself is irrelevant to `AlwaysPassedParser`, but a
            // report path must still be `Some` for the tier to activate.
            run_report: Some(&tmp.path().join("ignored.trx")),
            repo_root: tmp.path(),
        }];
        let violations = check_runtime_with(&tiers, &AlwaysPassedParser).unwrap();
        assert!(violations.is_empty());
    }
}
