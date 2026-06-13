//! `doctor` — checks and optionally installs required development tools.
//!
//! Port of `apps/rhino-cli/cmd/doctor.go`.

use anyhow::{Error, anyhow};
use clap::Args;

use crate::domain::cliout::OutputFormat;
use crate::internal::doctor::{
    self, CheckOptions, FixOptions, Scope, check_all, fix_all, format_fix_summary, format_json,
    format_markdown, format_text,
};
use crate::internal::git;

/// CLI arguments for the `doctor` command.
#[derive(Args, Debug)]
pub struct DoctorArgs {
    /// Tool scope: full or minimal.
    #[arg(long = "scope", default_value = "full")]
    pub scope: String,
    /// Attempt to install missing tools.
    #[arg(long = "fix")]
    pub fix: bool,
    /// Preview what --fix would install (only effective with --fix).
    #[arg(long = "dry-run")]
    pub dry_run: bool,
    /// Verbose output.
    #[arg(long, short = 'v')]
    pub verbose: bool,
    /// Quiet output.
    #[arg(long, short = 'q')]
    pub quiet: bool,
}

/// Run the `doctor` command.
///
/// # Errors
///
/// Returns an error if the git root cannot be found, if any tools fail to
/// install (when `--fix` is set), or if missing tools are detected.
pub fn run(args: &DoctorArgs, output: OutputFormat) -> std::result::Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;

    let parsed_scope = Scope::parse(&args.scope).unwrap_or(Scope::Full);

    let opts = CheckOptions {
        repo_root,
        runner: None,
        scope: parsed_scope,
    };

    let result = check_all(&opts);

    match output {
        OutputFormat::Text => print!("{}", format_text(&result, args.verbose, args.quiet)),
        OutputFormat::Json => println!("{}", format_json(&result)?),
        OutputFormat::Markdown => print!("{}", format_markdown(&result)),
    }

    if args.fix && result.missing_count > 0 {
        let mut buf = String::new();
        let fr = fix_all(
            &result,
            &opts,
            &FixOptions {
                dry_run: args.dry_run,
                runner: None,
            },
            |m| {
                buf.push_str(m);
                print!("{m}");
            },
        );
        print!("{}", format_fix_summary(&fr));
        if fr.failed > 0 {
            return Err(anyhow!("{} tool(s) failed to install", fr.failed));
        }
        if !args.dry_run && fr.fixed > 0 {
            return Ok(());
        }
    }

    if args.fix && result.missing_count == 0 {
        println!("\nNothing to fix — all tools are installed.");
    }

    if result.missing_count > 0 {
        return Err(anyhow!(
            "{} tool(s) not found in PATH",
            result.missing_count
        ));
    }

    // Silence unused-import lint on platforms that omit Doctor sub-helpers.
    let _ = doctor::is_minimal_tool;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn args_default_values() {
        let _ = DoctorArgs {
            scope: "full".into(),
            fix: false,
            dry_run: false,
            verbose: false,
            quiet: false,
        };
    }
}
