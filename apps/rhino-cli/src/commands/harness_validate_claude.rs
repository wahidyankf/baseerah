//! `harness validate-claude` — validates `.claude/agents/` and `.claude/skills/` definitions.
//!
//! Port of `apps/rhino-cli/cmd/agents_validate_claude.go`.

use anyhow::{Error, anyhow};
use clap::Args;

use crate::domain::cliout::OutputFormat;
use crate::internal::agents::claude_validator::validate_claude;
use crate::internal::agents::reporter::{
    format_validation_json, format_validation_markdown, format_validation_text,
};
use crate::internal::agents::types::ValidateClaudeOptions;
use crate::internal::git;

/// CLI arguments for `agents validate-claude`.
#[derive(Args, Debug)]
pub struct ValidateClaudeArgs {
    /// Validate only agents (skip skills).
    #[arg(long = "agents-only")]
    pub agents_only: bool,
    /// Validate only skills (skip agents).
    #[arg(long = "skills-only")]
    pub skills_only: bool,
    /// Verbose output (show all checks).
    #[arg(long, short = 'v')]
    pub verbose: bool,
    /// Quiet output (errors only).
    #[arg(long, short = 'q')]
    pub quiet: bool,
}

/// Run the `agents validate-claude` command.
///
/// # Errors
///
/// Returns an error if `--agents-only` and `--skills-only` are both set, if
/// the git root cannot be found, or if any validation checks fail.
pub fn run(
    args: &ValidateClaudeArgs,
    output_format: OutputFormat,
) -> std::result::Result<(), Error> {
    if args.agents_only && args.skills_only {
        return Err(anyhow!(
            "cannot use --agents-only and --skills-only together"
        ));
    }

    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    let opts = ValidateClaudeOptions {
        repo_root,
        agents_only: args.agents_only,
        skills_only: args.skills_only,
    };
    let result = validate_claude(&opts);

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
    use crate::internal::agents::types::ValidationResult;

    #[test]
    fn args_flags_default_off() {
        let args = ValidateClaudeArgs {
            agents_only: false,
            skills_only: false,
            verbose: false,
            quiet: false,
        };
        assert!(!args.agents_only);
        assert!(!args.skills_only);
    }

    #[test]
    fn validate_claude_options_field_passthrough() {
        let opts = ValidateClaudeOptions {
            repo_root: std::path::PathBuf::from("/tmp"),
            agents_only: true,
            skills_only: false,
        };
        assert!(opts.agents_only);
        assert!(!opts.skills_only);
    }

    #[test]
    fn empty_result_renders_passed() {
        let result = ValidationResult::default();
        let s = format_validation_text(&result, false, false);
        assert!(s.contains("VALIDATION PASSED"));
    }
}
