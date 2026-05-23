// Port of `apps/rhino-cli/cmd/env_restore.go`.

use anyhow::{anyhow, Error};
use clap::Args;

use crate::internal::cliout::OutputFormat;
use crate::internal::envbackup::{
    detect_worktree, expand_tilde, format_json, format_markdown, format_text, restore, Options,
    DEFAULT_BACKUP_DIR, DEFAULT_MAX_SIZE,
};
use crate::internal::gitutil;

#[derive(Args, Debug)]
pub struct EnvRestoreArgs {
    #[arg(long = "dir", default_value = "")]
    pub dir: String,
    #[arg(long = "worktree-aware")]
    pub worktree_aware: bool,
    #[arg(long = "force", short = 'f')]
    pub force: bool,
    #[arg(long = "include-config")]
    pub include_config: bool,
    #[arg(long, short = 'v')]
    pub verbose: bool,
    #[arg(long, short = 'q')]
    pub quiet: bool,
}

pub fn run(args: &EnvRestoreArgs, output: OutputFormat) -> std::result::Result<(), Error> {
    let repo_root =
        gitutil::find_git_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
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
            dir: "".into(),
            worktree_aware: false,
            force: true,
            include_config: false,
            verbose: false,
            quiet: false,
        };
    }
}
