//! `env restore` — restores `.env*` files from an external backup directory.
//!
//! Port of `apps/rhino-cli/cmd/env_restore.go`.

use anyhow::{Error, anyhow};
use clap::Args;

use crate::domain::cliout::OutputFormat;
use crate::internal::envbackup::{
    DEFAULT_BACKUP_DIR, DEFAULT_MAX_SIZE, Options, detect_worktree, expand_tilde, format_json,
    format_markdown, format_text, restore,
};
use crate::internal::git;

/// CLI arguments for `env restore`.
#[derive(Args, Debug)]
pub struct EnvRestoreArgs {
    /// Backup directory (default: `~/ose-public-env-backup`).
    #[arg(long = "dir", default_value = "")]
    pub dir: String,
    /// Namespace restore by worktree/repo directory name.
    #[arg(long = "worktree-aware")]
    pub worktree_aware: bool,
    /// Skip overwrite confirmation.
    #[arg(long = "force", short = 'f')]
    pub force: bool,
    /// Also restore known uncommitted config files.
    #[arg(long = "include-config")]
    pub include_config: bool,
    /// Preview what would be restored without writing any files.
    #[arg(long = "dry-run")]
    pub dry_run: bool,
    /// Verbose output.
    #[arg(long, short = 'v')]
    pub verbose: bool,
    /// Quiet output.
    #[arg(long, short = 'q')]
    pub quiet: bool,
}

/// Run the `env restore` command.
///
/// # Errors
///
/// Returns an error if the git root cannot be found, worktree detection fails,
/// or the restore operation fails.
pub fn run(args: &EnvRestoreArgs, output: OutputFormat) -> std::result::Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    let backup_dir = if args.dir.is_empty() {
        let home = expand_tilde("~")?;
        home.join(DEFAULT_BACKUP_DIR)
    } else {
        let expanded = expand_tilde(&args.dir)?;
        std::fs::canonicalize(&expanded).unwrap_or(expanded)
    };

    let force = args.force || !matches!(output, OutputFormat::Text);

    let mut opts = Options {
        repo_root,
        backup_dir,
        max_size: DEFAULT_MAX_SIZE,
        worktree_aware: args.worktree_aware,
        force,
        include_config: args.include_config,
        dry_run: args.dry_run,
        ..Default::default()
    };
    if args.worktree_aware {
        let info = detect_worktree(&opts.repo_root)
            .map_err(|e| anyhow!("worktree detection failed: {e}"))?;
        opts.worktree_name = info.worktree_name;
    }

    let result = restore(&mut opts).map_err(|e| anyhow!("env restore failed: {e}"))?;

    match output {
        OutputFormat::Text => print!("{}", format_text(&result, args.verbose, args.quiet)),
        OutputFormat::Json => println!("{}", format_json(&result)?),
        OutputFormat::Markdown => print!("{}", format_markdown(&result)),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn args_default() {
        let _ = EnvRestoreArgs {
            dir: String::new(),
            worktree_aware: false,
            force: true,
            include_config: false,
            dry_run: false,
            verbose: false,
            quiet: false,
        };
    }
}
