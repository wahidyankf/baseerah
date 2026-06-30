//! `harness instruction-size validate` — checks instruction-file sizes against budgets
//! derived from the harness registry (`repo-config.yml`) and `instruction-size-budget.yaml`.
//!
//! Thin wrapper delegating to `convention_validate_instruction_size::run_for_root`, which
//! merges registry-derived surfaces with yaml-defined surfaces and thresholds.

use anyhow::{Error, anyhow};
use clap::Args;

use crate::commands::convention_validate_instruction_size;
use crate::domain::cliout::OutputFormat;
use crate::internal::git;

/// CLI arguments for `harness instruction-size validate` (none required).
#[derive(Args, Debug)]
pub struct ValidateInstructionSizeArgs {}

/// Run the `harness instruction-size validate` command.
///
/// Discovers the git repository root and delegates to the shared instruction-size
/// logic that merges `repo-config.yml` harness `instruction:` surfaces with
/// `instruction-size-budget.yaml` per-surface thresholds.
///
/// # Errors
///
/// Returns an error when the git root cannot be found or any instruction file
/// exceeds its fail budget.
pub fn run(
    _args: &ValidateInstructionSizeArgs,
    output_format: OutputFormat,
) -> std::result::Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    convention_validate_instruction_size::run_for_root(&repo_root, output_format)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn run_for_root_passes_when_within_budget() {
        let tmp = TempDir::new().unwrap();
        let yaml = "surfaces:\n  - glob: \"AGENTS.md\"\n    target: 24000\n    warn: 27000\n    fail: 30000\nresolved_tree:\n  root: \"CLAUDE.md\"\n  target: 30000\n  warn: 34000\n  fail: 38000\n";
        fs::write(tmp.path().join("instruction-size-budget.yaml"), yaml).unwrap();
        fs::write(tmp.path().join("AGENTS.md"), "x".repeat(10_000)).unwrap();
        fs::write(tmp.path().join("CLAUDE.md"), "small").unwrap();
        let result =
            convention_validate_instruction_size::run_for_root(tmp.path(), OutputFormat::Text);
        assert!(result.is_ok());
    }

    #[test]
    fn run_for_root_fails_when_registry_surface_oversized() {
        let tmp = TempDir::new().unwrap();
        let repo_config = concat!(
            "harness:\n",
            "  - { name: cursor, tier: native, shadow: .cursor/rules,",
            " instruction: [.cursor/rules] }\n",
            "coverage:\n  projects: []\n",
            "specs:\n  ddd-areas: []\n  domain-areas: []\n",
        );
        fs::write(tmp.path().join("repo-config.yml"), repo_config).unwrap();
        fs::create_dir_all(tmp.path().join(".cursor")).unwrap();
        fs::write(tmp.path().join(".cursor/rules"), "x".repeat(50_000)).unwrap();
        let result =
            convention_validate_instruction_size::run_for_root(tmp.path(), OutputFormat::Text);
        assert!(
            result.is_err(),
            "registry-derived surface must be flagged when oversized"
        );
    }
}
