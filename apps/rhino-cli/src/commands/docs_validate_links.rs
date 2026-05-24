// Port of `apps/rhino-cli/cmd/docs_validate_links.go`.

use anyhow::{Context, Error, anyhow};
use clap::Args;

use crate::internal::cliout::OutputFormat;
use crate::internal::docs::links::{
    ScanOptions, format_link_json, format_link_markdown, format_link_text, validate_all_links,
};
use crate::internal::gitutil;

#[derive(Args, Debug)]
pub struct ValidateLinksArgs {
    /// Only validate staged files.
    #[arg(long = "staged-only")]
    pub staged_only: bool,
}

pub fn run(
    args: &ValidateLinksArgs,
    output_format: OutputFormat,
) -> std::result::Result<(), Error> {
    let repo_root =
        gitutil::find_git_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    let opts = ScanOptions {
        repo_root,
        staged_only: args.staged_only,
        skip_paths: Vec::new(),
    };
    let result = validate_all_links(&opts).context("validation failed")?;

    match output_format {
        OutputFormat::Text => print!("{}", format_link_text(&result, false, false)),
        OutputFormat::Json => print!("{}", format_link_json(&result)?),
        OutputFormat::Markdown => print!("{}", format_link_markdown(&result)),
    }

    if !result.broken_links.is_empty() {
        return Err(anyhow!("found {} broken links", result.broken_links.len()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn args_default_staged_only_false() {
        let args = ValidateLinksArgs { staged_only: false };
        assert!(!args.staged_only);
    }

    #[test]
    fn args_staged_only_can_be_set() {
        let args = ValidateLinksArgs { staged_only: true };
        assert!(args.staged_only);
    }
}
