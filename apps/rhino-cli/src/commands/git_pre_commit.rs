//! `git pre-commit` — REMOVED in §2a-names. The pre-commit hook now calls individual commands directly.
//!
//! Port of `apps/rhino-cli/cmd/git_pre_commit.go`.

use anyhow::{Error, anyhow};
use clap::Args;

use crate::application::git::pre_commit::run;
use crate::domain::cliout::OutputFormat;
use crate::infrastructure::git::{make_default_deps, root::find_root};

/// CLI arguments for the removed `git pre-commit` command (kept for reference) (none required).
#[derive(Args, Debug)]
pub struct PreCommitArgs {}

/// Run the removed `git pre-commit` command.
///
/// # Errors
///
/// Returns an error if the git root cannot be found or if any pre-commit
/// check fails.
pub fn run_cmd(_args: &PreCommitArgs, _output: OutputFormat) -> std::result::Result<(), Error> {
    let git_root = find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    let mut deps = make_default_deps(git_root);
    run(&mut deps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn args_constructible() {
        let _ = PreCommitArgs {};
    }
}
