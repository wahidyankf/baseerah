//! Data types for the per-level @covers behavior coverage engine.

use std::collections::HashSet;
use std::fmt;

/// Test level: unit, integration, or e2e.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TestLevel {
    /// Isolated unit tests (cargo test / jest).
    Unit,
    /// Integration tests requiring external services.
    Integration,
    /// End-to-end tests (Playwright / Cypress).
    E2e,
}

impl fmt::Display for TestLevel {
    /// Renders the lowercase level name used in diagnostic output
    /// (`"unit"`, `"integration"`, `"e2e"`) and in `--<level>-report` flags.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TestLevel::Unit => "unit",
            TestLevel::Integration => "integration",
            TestLevel::E2e => "e2e",
        };
        f.write_str(s)
    }
}

/// A Gherkin scenario extracted from a feature file.
#[derive(Debug, Clone)]
pub struct ScenarioSpec {
    /// Repo-relative path to the .feature file.
    pub feature_path: String,
    /// Scenario title (without the "Scenario:" keyword).
    pub title: String,
    /// Level tags declared on this scenario (@unit, @integration, @e2e).
    ///
    /// Empty means untagged (a lint error).
    pub level_tags: HashSet<TestLevel>,
    /// True if the scenario is tagged @wip (exempt from coverage).
    pub is_wip: bool,
}

/// An @covers marker found in a test source file.
#[derive(Debug, Clone)]
pub struct CoversMarker {
    /// Repo-relative path to the test source file containing this marker.
    pub source_file: String,
    /// Test level derived from the owning test target (unit/integration/e2e).
    pub level: TestLevel,
    /// Repo-relative path to the feature file (from the marker text).
    pub feature_path: String,
    /// Scenario title (from the marker text).
    pub scenario_title: String,
}

/// The set of test levels a project supports (its level envelope P).
#[derive(Debug, Clone)]
pub struct ProjectEnvelope {
    /// Levels declared in the coverage.projects registry for this project.
    pub levels: HashSet<TestLevel>,
}

/// A violation found by the behavior coverage engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BehaviorCoverageViolation {
    /// A scenario has no @unit/@integration/@e2e level tags.
    UntaggedScenario {
        /// Repo-relative path to the feature file containing the scenario.
        feature_path: String,
        /// Title of the untagged scenario.
        title: String,
    },
    /// A scenario's tag names a level not in the project envelope P.
    LevelOutsideEnvelope {
        /// Repo-relative path to the feature file containing the scenario.
        feature_path: String,
        /// Title of the offending scenario.
        title: String,
        /// The level that is outside the project envelope.
        required_level: TestLevel,
    },
    /// A scenario requires a level (from S) but has no @covers marker at that level.
    MissingCoverage {
        /// Repo-relative path to the feature file containing the scenario.
        feature_path: String,
        /// Title of the scenario that lacks coverage.
        title: String,
        /// The level at which coverage is absent.
        missing_level: TestLevel,
    },
    /// A @covers marker targets a level not in S (over-coverage).
    CoverageAtUndeclaredLevel {
        /// Repo-relative path to the test source file that carries the marker.
        source_file: String,
        /// Repo-relative path to the feature file referenced by the marker.
        feature_path: String,
        /// Title of the scenario referenced by the marker.
        title: String,
        /// The level that is not declared for the scenario.
        extra_level: TestLevel,
    },
    /// A @covers marker references a scenario title that no feature file contains.
    OrphanMarker {
        /// Repo-relative path to the test source file that carries the orphan marker.
        source_file: String,
        /// Repo-relative path to the feature file named in the marker.
        feature_path: String,
        /// Scenario title from the marker text that could not be resolved.
        scenario_title: String,
    },
}

/// Execution status of a single scenario recorded in a tier's machine-readable
/// run report (see [`RunReportEntry`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RunStatus {
    /// The test executed and its assertions succeeded.
    Passed,
    /// The test executed but at least one assertion failed.
    Failed,
    /// The test was skipped, `.only`'d away by another test, marked
    /// `.todo`/pending, or otherwise never ran.
    Skipped,
}

/// A single scenario execution result recorded in a tier's run report.
///
/// Each test-runner ecosystem is expected to normalise its own native report
/// (Jest/Vitest JSON, Playwright JSON, cucumber-rs output, .NET TRX/JSON,
/// etc.) into this flat shape before the runtime cross-check reads it —
/// that per-ecosystem adapter is a per-project rollout concern, not part of
/// the engine itself.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct RunReportEntry {
    /// Repo-relative path to the `.feature` file, matching a
    /// [`CoversMarker::feature_path`].
    pub feature_path: String,
    /// Scenario title, matching a [`CoversMarker::scenario_title`].
    pub scenario_title: String,
    /// Whether the scenario's covering test executed and passed.
    pub status: RunStatus,
}

/// A runtime cross-check violation: a `@covers` marker whose covering test
/// did not execute-and-pass at its declared level, per the tier's run report.
///
/// This is the gap neither the legacy step-text traceability scan
/// (`crate::application::speccoverage::checker::check_all`) nor the
/// marker-existence engine ([`super::validator::validate`]) can see: both
/// only prove a matching implementation *exists*, never that it *ran*.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeCoverageViolation {
    /// No run-report entry names this marker's scenario — the covering test
    /// never executed (skipped/`.only`'d-away/`.todo`/undefined at runtime).
    NotExecuted {
        /// Repo-relative path to the test source file carrying the marker.
        source_file: String,
        /// Repo-relative path to the feature file named in the marker.
        feature_path: String,
        /// Scenario title named in the marker.
        scenario_title: String,
        /// Test level the marker declares.
        level: TestLevel,
    },
    /// A run-report entry exists for this marker's scenario but its status
    /// is not [`RunStatus::Passed`].
    Failed {
        /// Repo-relative path to the test source file carrying the marker.
        source_file: String,
        /// Repo-relative path to the feature file named in the marker.
        feature_path: String,
        /// Scenario title named in the marker.
        scenario_title: String,
        /// Test level the marker declares.
        level: TestLevel,
    },
}
