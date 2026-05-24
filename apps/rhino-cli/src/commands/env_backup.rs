// Port of `apps/rhino-cli/cmd/env_backup.go`.

use anyhow::{Error, anyhow};
use clap::Args;

use crate::internal::cliout::OutputFormat;
use crate::internal::envbackup::{
    DEFAULT_BACKUP_DIR, DEFAULT_MAX_SIZE, Options, backup, default_skip_dirs, detect_worktree,
    expand_tilde, format_json, format_markdown, format_text,
};
use crate::internal::gitutil;

#[derive(Args, Debug)]
pub struct EnvBackupArgs {
    /// Backup directory (default: ~/ose-open-env-backup).
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
    /// Verbose output.
    #[arg(long, short = 'v')]
    pub verbose: bool,
    /// Quiet output.
    #[arg(long, short = 'q')]
    pub quiet: bool,
}

pub fn run(args: &EnvBackupArgs, output: OutputFormat) -> std::result::Result<(), Error> {
    let repo_root =
        gitutil::find_git_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    let backup_dir = if args.dir.is_empty() {
        let home = expand_tilde("~")?;
        home.join(DEFAULT_BACKUP_DIR)
    } else {
        let expanded = expand_tilde(&args.dir)?;
        std::fs::canonicalize(&expanded).unwrap_or(expanded)
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
            verbose: false,
            quiet: false,
        };
    }
}
