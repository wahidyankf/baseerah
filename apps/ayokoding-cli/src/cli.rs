//! CLI argument parsing and dispatch for `ayokoding-cli`.
//!
//! Defines the top-level [`Cli`] struct, the [`Commands`] enum, and
//! the [`run`] function that orchestrates parsing and subcommand dispatch.
use clap::{Parser, Subcommand};

use crate::commands;

/// Top-level CLI configuration for `ayokoding-cli`.
#[derive(Debug, Parser)]
#[command(
    name = "ayokoding-cli",
    about = "CLI tools for ayokoding-web site maintenance",
    version
)]
pub struct Cli {
    /// Enable verbose output with additional details.
    #[arg(short = 'v', long, global = true)]
    pub verbose: bool,

    /// Suppress all output except errors.
    #[arg(short = 'q', long, global = true)]
    pub quiet: bool,

    /// Output format: text, json, or markdown.
    #[arg(short = 'o', long, default_value = "text", global = true)]
    pub output: String,

    /// Disable colored output.
    #[arg(long, global = true)]
    pub no_color: bool,

    /// Subcommand to execute.
    #[command(subcommand)]
    pub command: Commands,
}

/// Top-level subcommands for `ayokoding-cli`.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Link management commands for `ayokoding-web` content.
    Links(LinksCmd),
}

/// Container for the `links` subcommand group.
#[derive(Debug, clap::Args)]
pub struct LinksCmd {
    /// Links subcommand to execute.
    #[command(subcommand)]
    pub subcommand: LinksSubcommand,
}

/// Subcommands available under `links`.
#[derive(Debug, Subcommand)]
pub enum LinksSubcommand {
    /// Validate internal links in `ayokoding-web` content.
    Check(commands::links::LinksCheckArgs),
}

/// Dispatch a fully-parsed [`Cli`] value to the appropriate subcommand
/// handler and return an exit code.
///
/// Separated from [`run`] so tests can inject a constructed [`Cli`] without
/// touching `std::env::args`.
///
/// Returns an exit code:
/// - `0` on success
/// - `1` on runtime error (broken links, I/O failure, etc.)
/// - `2` on invalid arguments (unknown output format)
pub fn dispatch(cli: &Cli) -> i32 {
    match cli.output.as_str() {
        "text" | "json" | "markdown" => {}
        other => {
            eprintln!("Error: unknown output format {other:?}: must be text, json, or markdown");
            return 2;
        }
    }

    let result = match &cli.command {
        Commands::Links(links_cmd) => match &links_cmd.subcommand {
            LinksSubcommand::Check(args) => {
                commands::links::run_links_check(args, &cli.output, cli.quiet, cli.verbose)
            }
        },
    };

    match result {
        Ok(()) => 0,
        Err(e) => {
            if !cli.quiet {
                eprintln!("Error: {e}");
            }
            1
        }
    }
}

/// Parse CLI arguments, validate them, and dispatch to the appropriate
/// subcommand handler.
///
/// Returns an exit code:
/// - `0` on success
/// - `1` on runtime error (broken links, I/O failure, etc.)
/// - `2` on invalid arguments (unknown output format)
pub fn run() -> i32 {
    let cli = Cli::parse();
    dispatch(&cli)
}

#[cfg(test)]
mod tests {
    use std::io::Write as _;

    use tempfile::TempDir;

    use super::{Cli, Commands, LinksCmd, LinksSubcommand, dispatch};
    use crate::commands::links::LinksCheckArgs;

    fn make_cli(dir: &TempDir, output: &str, quiet: bool, verbose: bool) -> Cli {
        Cli {
            verbose,
            quiet,
            output: output.to_owned(),
            no_color: false,
            command: Commands::Links(LinksCmd {
                subcommand: LinksSubcommand::Check(LinksCheckArgs {
                    content: dir.path().to_string_lossy().into_owned(),
                }),
            }),
        }
    }

    #[test]
    fn test_dispatch_text_empty_dir_returns_zero() {
        let dir = TempDir::new().expect("create tempdir");
        let cli = make_cli(&dir, "text", false, false);
        assert_eq!(dispatch(&cli), 0);
    }

    #[test]
    fn test_dispatch_json_empty_dir_returns_zero() {
        let dir = TempDir::new().expect("create tempdir");
        let cli = make_cli(&dir, "json", false, false);
        assert_eq!(dispatch(&cli), 0);
    }

    #[test]
    fn test_dispatch_markdown_empty_dir_returns_zero() {
        let dir = TempDir::new().expect("create tempdir");
        let cli = make_cli(&dir, "markdown", false, false);
        assert_eq!(dispatch(&cli), 0);
    }

    #[test]
    fn test_dispatch_unknown_format_returns_two() {
        let dir = TempDir::new().expect("create tempdir");
        let cli = make_cli(&dir, "xml", false, false);
        assert_eq!(dispatch(&cli), 2);
    }

    #[test]
    fn test_dispatch_broken_link_returns_one() {
        let dir = TempDir::new().expect("create tempdir");
        let md_path = dir.path().join("page.md");
        let mut file = std::fs::File::create(&md_path).expect("create md file");
        writeln!(file, "[broken](/nonexistent-target)").expect("write md content");
        let cli = make_cli(&dir, "text", false, false);
        assert_eq!(dispatch(&cli), 1);
    }

    #[test]
    fn test_dispatch_broken_link_quiet_returns_one() {
        let dir = TempDir::new().expect("create tempdir");
        let md_path = dir.path().join("page.md");
        let mut file = std::fs::File::create(&md_path).expect("create md file");
        writeln!(file, "[broken](/nonexistent-target)").expect("write md content");
        let cli = make_cli(&dir, "text", true, false);
        assert_eq!(dispatch(&cli), 1);
    }

    #[test]
    fn test_dispatch_nonexistent_dir_returns_one() {
        let cli = Cli {
            verbose: false,
            quiet: false,
            output: "text".to_owned(),
            no_color: false,
            command: Commands::Links(LinksCmd {
                subcommand: LinksSubcommand::Check(LinksCheckArgs {
                    content: "/nonexistent/xyz/ayokoding-cli-dispatch-test".to_owned(),
                }),
            }),
        };
        assert_eq!(dispatch(&cli), 1);
    }

    #[test]
    fn test_dispatch_verbose_empty_dir_returns_zero() {
        let dir = TempDir::new().expect("create tempdir");
        let cli = make_cli(&dir, "text", false, true);
        assert_eq!(dispatch(&cli), 0);
    }

    #[test]
    fn test_dispatch_quiet_empty_dir_returns_zero() {
        let dir = TempDir::new().expect("create tempdir");
        let cli = make_cli(&dir, "text", true, false);
        assert_eq!(dispatch(&cli), 0);
    }
}
