//! `env backup` — backs up `.env*` files to an external directory.
//!
//! Port of `apps/rhino-cli/cmd/env_backup.go`.

use anyhow::{Error, anyhow};
use clap::Args;

use crate::domain::cliout::OutputFormat;
use crate::internal::envbackup::{
    DEFAULT_BACKUP_DIR, DEFAULT_MAX_SIZE, Options, backup, canonicalize_best_effort,
    default_skip_dirs, detect_worktree, expand_tilde, format_json, format_markdown, format_text,
};
use crate::internal::git;

/// CLI arguments for `env backup`.
#[derive(Args, Debug)]
pub struct EnvBackupArgs {
    /// Backup directory (default: `~/ose-public-env-backup`).
    #[arg(long = "dir", default_value = "")]
    pub dir: String,
    /// Namespace backup by worktree/repo directory name.
    #[arg(long = "worktree-aware")]
    pub worktree_aware: bool,
    /// Skip overwrite confirmation.
    #[arg(long = "force", short = 'f')]
    pub force: bool,
    /// Also back up known uncommitted config files.
    #[arg(long = "include-config")]
    pub include_config: bool,
    /// Preview what would be backed up without writing any files.
    #[arg(long = "dry-run")]
    pub dry_run: bool,
    /// Verbose output.
    #[arg(long, short = 'v')]
    pub verbose: bool,
    /// Quiet output.
    #[arg(long, short = 'q')]
    pub quiet: bool,
}

/// Run the `env backup` command.
///
/// # Errors
///
/// Returns an error if the git root cannot be found, worktree detection fails,
/// or the backup operation fails.
pub fn run(args: &EnvBackupArgs, output: OutputFormat) -> std::result::Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    let backup_dir = if args.dir.is_empty() {
        let home = expand_tilde("~")?;
        let default_dir = home.join(DEFAULT_BACKUP_DIR);
        canonicalize_best_effort(&default_dir).unwrap_or(default_dir)
    } else {
        let expanded = expand_tilde(&args.dir)?;
        // Best-effort canonicalize (resolving symlinks even when `expanded`
        // does not exist yet) so the inside-repo check below compares two
        // paths in the same (physical) namespace as `repo_root`, which
        // `find_root()` always returns via `git rev-parse --show-toplevel`.
        canonicalize_best_effort(&expanded).unwrap_or(expanded)
    };

    // Force when explicit, non-text output, or unhandled stdin.
    let force = args.force || !matches!(output, OutputFormat::Text);

    let mut opts = Options {
        repo_root,
        backup_dir,
        skip_dirs: default_skip_dirs()
            .iter()
            .map(std::string::ToString::to_string)
            .collect(),
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

    let result = backup(&mut opts).map_err(|e| anyhow!("env backup failed: {e}"))?;

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
    fn args_constructible() {
        let _ = EnvBackupArgs {
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
