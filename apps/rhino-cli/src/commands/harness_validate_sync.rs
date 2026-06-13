//! `harness validate-sync` — checks that `.opencode/agents/` mirrors `.claude/agents/` exactly.
//!
//! Port of `apps/rhino-cli/cmd/agents_validate_sync.go`.

use anyhow::{Error, anyhow};
use clap::Args;

use crate::domain::cliout::OutputFormat;
use crate::internal::agents::reporter::{
    format_validation_json, format_validation_markdown, format_validation_text,
};
use crate::internal::agents::sync_validator::validate_sync;
use crate::internal::git;

/// CLI arguments for `agents validate-sync`.
#[derive(Args, Debug)]
pub struct ValidateSyncArgs {
    /// Verbose output (show all checks).
    #[arg(long, short = 'v')]
    pub verbose: bool,
    /// Quiet output (summary only).
    #[arg(long, short = 'q')]
    pub quiet: bool,
}

/// Run the `agents validate-sync` command.
///
/// # Errors
///
/// Returns an error if the git root cannot be found or if any sync validation
/// checks fail.
pub fn run(args: &ValidateSyncArgs, output_format: OutputFormat) -> std::result::Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    let result = validate_sync(&repo_root);

    match output_format {
        OutputFormat::Text => print!(
            "{}",
            format_validation_text(&result, args.verbose, args.quiet)
        ),
        OutputFormat::Json => println!("{}", format_validation_json(&result)?),
        OutputFormat::Markdown => print!("{}", format_validation_markdown(&result, args.verbose)),
    }

    if result.failed_checks > 0 {
        return Err(anyhow!(
            "validation failed: {} checks failed",
            result.failed_checks
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn args_default() {
        let a = ValidateSyncArgs {
            verbose: false,
            quiet: false,
        };
        assert!(!a.verbose);
        assert!(!a.quiet);
    }
}
