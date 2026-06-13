//! `harness sync` — syncs `.claude/agents/` to `.opencode/agents/`.
//!
//! Port of `apps/rhino-cli/cmd/agents_sync.go`.

use anyhow::{Error, anyhow};
use clap::Args;

use crate::domain::cliout::OutputFormat;
use crate::internal::agents::reporter::{format_sync_json, format_sync_markdown, format_sync_text};
use crate::internal::agents::sync::{SyncOptions, sync_all};
use crate::internal::git;

/// CLI arguments for `agents sync`.
#[derive(Args, Debug)]
pub struct SyncArgs {
    /// Preview changes without modifying files.
    #[arg(long = "dry-run")]
    pub dry_run: bool,
    /// Sync only agents (skip skills — no-op as skills are not mirrored).
    #[arg(long = "agents-only")]
    pub agents_only: bool,
    /// Sync only skills (skip agents).
    #[arg(long = "skills-only")]
    pub skills_only: bool,
    /// Verbose output.
    #[arg(long, short = 'v')]
    pub verbose: bool,
    /// Quiet output.
    #[arg(long, short = 'q')]
    pub quiet: bool,
}

/// Run the `agents sync` command.
///
/// # Errors
///
/// Returns an error if `--agents-only` and `--skills-only` are both set, if
/// the git root cannot be found, if the sync operation fails, or if any files
/// failed to sync.
pub fn run(args: &SyncArgs, output_format: OutputFormat) -> std::result::Result<(), Error> {
    if args.agents_only && args.skills_only {
        return Err(anyhow!("cannot use both --agents-only and --skills-only"));
    }

    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    let opts = SyncOptions {
        repo_root,
        dry_run: args.dry_run,
        agents_only: args.agents_only,
        skills_only: args.skills_only,
        verbose: args.verbose,
        quiet: args.quiet,
    };
    let result = sync_all(&opts).map_err(|e| anyhow!("sync failed: {e}"))?;

    match output_format {
        OutputFormat::Text => print!("{}", format_sync_text(&result, args.verbose, args.quiet)),
        OutputFormat::Json => println!("{}", format_sync_json(&result)?),
        OutputFormat::Markdown => print!("{}", format_sync_markdown(&result)),
    }

    if !result.failed_files.is_empty() {
        return Err(anyhow!(
            "sync completed with {} failures",
            result.failed_files.len()
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flag_defaults() {
        let a = SyncArgs {
            dry_run: false,
            agents_only: false,
            skills_only: false,
            verbose: false,
            quiet: false,
        };
        assert!(!a.dry_run);
        assert!(!a.agents_only);
    }
}
