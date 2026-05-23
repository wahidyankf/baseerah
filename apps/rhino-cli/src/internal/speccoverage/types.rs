// Byte-for-byte port of `apps/rhino-cli/internal/speccoverage/types.go`.

use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Clone, Default)]
pub struct ScanOptions {
    pub repo_root: PathBuf,
    /// Legacy single-spec-tree input. When `specs_dirs` is non-empty it takes precedence.
    pub specs_dir: PathBuf,
    /// Absolute paths to one or more spec trees walked together.
    pub specs_dirs: Vec<PathBuf>,
    pub app_dir: PathBuf,
    pub verbose: bool,
    pub quiet: bool,
    pub shared_steps: bool,
    pub exclude_dirs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoverageGap {
    pub spec_file: String,
    pub stem: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioGap {
    pub spec_file: String,
    pub scenario_title: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepGap {
    pub spec_file: String,
    pub scenario_title: String,
    pub step_keyword: String,
    pub step_text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrphanStepImpl {
    pub file: String,
    pub matcher_kind: String,
    pub matcher_text: String,
}

#[derive(Debug, Clone, Default)]
pub struct CheckResult {
    pub total_specs: usize,
    pub total_scenarios: usize,
    pub total_steps: usize,
    pub gaps: Vec<CoverageGap>,
    pub scenario_gaps: Vec<ScenarioGap>,
    pub step_gaps: Vec<StepGap>,
    pub orphan_step_impls: Vec<OrphanStepImpl>,
    pub duration: Duration,
}
