// Port of `apps/rhino-cli/cmd/git_pre_commit.go`.

use anyhow::{anyhow, Error};
use clap::Args;

use crate::internal::cliout::OutputFormat;
use crate::internal::git::{run, Deps};
use crate::internal::gitutil;

#[derive(Args, Debug)]
pub struct PreCommitArgs {}

pub fn run_cmd(_args: &PreCommitArgs, _output: OutputFormat) -> std::result::Result<(), Error> {
    let git_root =
        gitutil::find_git_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    let mut deps = Deps::default_for(git_root);
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
