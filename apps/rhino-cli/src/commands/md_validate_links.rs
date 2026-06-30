//! `md validate-links` — checks markdown files for broken internal links.
//!
//! Port of `apps/rhino-cli/cmd/docs_validate_links.go`.

use anyhow::{Context, Error, anyhow};
use clap::Args;

use crate::domain::cliout::OutputFormat;
use crate::internal::docs::links::{
    ScanOptions, format_link_json, format_link_markdown, format_link_text, validate_all_links,
};
use crate::internal::git;

/// CLI arguments for `docs validate-links`.
#[derive(Args, Debug)]
pub struct ValidateLinksArgs {
    /// Only validate staged files.
    #[arg(long = "staged-only")]
    pub staged_only: bool,
    /// Repository-relative path prefixes to exclude from scanning.
    /// May be specified multiple times.
    #[arg(long = "exclude")]
    pub exclude: Vec<String>,
}

/// Run the `docs validate-links` command.
///
/// # Errors
///
/// Returns an error if the git root cannot be found, if the scan fails, or if
/// broken links are found.
pub fn run(
    args: &ValidateLinksArgs,
    output_format: OutputFormat,
) -> std::result::Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    let opts = ScanOptions {
        repo_root,
        staged_only: args.staged_only,
        skip_paths: args.exclude.clone(),
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
        let args = ValidateLinksArgs {
            staged_only: false,
            exclude: Vec::new(),
        };
        assert!(!args.staged_only);
    }

    #[test]
    fn args_staged_only_can_be_set() {
        let args = ValidateLinksArgs {
            staged_only: true,
            exclude: Vec::new(),
        };
        assert!(args.staged_only);
    }

    /// (a) Phase 1 RED — `--exclude` flag is threaded into `skip_paths`.
    #[test]
    fn args_exclude_is_threaded_to_skip_paths() {
        let args = ValidateLinksArgs {
            staged_only: false,
            exclude: vec!["plans/done".to_string(), "apps/ayokoding-www".to_string()],
        };
        assert_eq!(args.exclude.len(), 2);
        assert_eq!(args.exclude[0], "plans/done");
    }

    // ---- P1-1b-RED6: md links validate covers specs/ paths ---

    #[test]
    fn md_links_validate_covers_specs_dir() {
        // Prove md links validate already covers specs/ files — making specs validate links
        // redundant. Seeds a broken relative link inside specs/apps/x/foo.md and asserts the
        // scanner flags it. No specs:links-validation Nx target needed once this passes.
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        let spec_dir = root.join("specs/apps/x");
        std::fs::create_dir_all(&spec_dir).expect("create spec dir");
        std::fs::write(
            spec_dir.join("foo.md"),
            "# Spec\n\nSee [missing](./nonexistent.md).\n",
        )
        .expect("write spec file");
        let result = validate_all_links(&ScanOptions {
            repo_root: root.to_path_buf(),
            staged_only: false,
            skip_paths: vec![],
        })
        .expect("scan must not hard-error on a temp dir");
        assert!(
            !result.broken_links.is_empty(),
            "md links validate must flag broken link inside specs/; got 0 broken links"
        );
        let has_specs_hit = result
            .broken_links
            .iter()
            .any(|b| b.source_file.contains("specs"));
        assert!(
            has_specs_hit,
            "at least one broken link must originate from specs/; links: {:?}",
            result.broken_links
        );
    }
}
