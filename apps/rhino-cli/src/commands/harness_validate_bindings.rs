//! `agents validate-bindings` — deterministic guard that enforces parity
// between the committed Amazon Q bridge files and their canonical content,
// plus catalog coverage for every present known binding directory. Exits
// non-zero on any drift. See `crate::internal::agents::bindings`.

use anyhow::{Error, anyhow};
use clap::Args;

use crate::domain::cliout::OutputFormat;
use crate::internal::agents::bindings::validate_bindings;
use crate::internal::agents::reporter::{
    format_validation_json, format_validation_markdown, format_validation_text,
};
use crate::internal::git;

/// CLI arguments for `agents validate-bindings`.
#[derive(Args, Debug)]
pub struct ValidateBindingsArgs {
    /// Verbose output (show all checks).
    #[arg(long, short = 'v')]
    pub verbose: bool,
    /// Quiet output (summary only).
    #[arg(long, short = 'q')]
    pub quiet: bool,
}

/// Run the `agents validate-bindings` command.
///
/// # Errors
///
/// Returns an error if the git root cannot be found or if any binding
/// validation checks fail.
pub fn run(
    args: &ValidateBindingsArgs,
    output_format: OutputFormat,
) -> std::result::Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    let result = validate_bindings(&repo_root);

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
            "binding validation failed: {} checks failed",
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
        let a = ValidateBindingsArgs {
            verbose: false,
            quiet: false,
        };
        assert!(!a.verbose);
        assert!(!a.quiet);
    }
}
