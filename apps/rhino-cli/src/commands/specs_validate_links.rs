//! `specs validate-links` — checks that links within spec files resolve correctly.
//!
//! Port of `apps/rhino-cli/cmd/specs_validate_links.go`.

use anyhow::{Error, anyhow};
use clap::Args;

use crate::domain::cliout::OutputFormat;
use crate::internal::allowlist::apps_with_ddd;
use crate::internal::git;
use crate::internal::specs::validate_spec_links;

/// CLI arguments for `specs validate-links`.
#[derive(Args, Debug)]
pub struct ValidateLinksArgs {
    /// Optional single spec folder path.
    #[arg(value_name = "folder")]
    pub folder: Option<String>,
    /// Comma-separated app names.
    #[arg(long = "apps", value_delimiter = ',')]
    pub apps: Vec<String>,
}

/// Resolve the list of folders to validate from positional and flag inputs.
fn resolve_folders(positional: Option<&String>, flag: &[String]) -> Vec<String> {
    if let Some(p) = positional {
        return vec![p.clone()];
    }
    let apps: Vec<String> = if flag.is_empty() {
        apps_with_ddd().iter().map(|s| (*s).to_string()).collect()
    } else {
        flag.to_vec()
    };
    apps.into_iter()
        .map(|a| format!("specs/apps/{a}"))
        .collect()
}

/// Run the `specs validate-links` command.
///
/// # Errors
///
/// Returns an error if the git root cannot be found or broken links are found.
pub fn run(args: &ValidateLinksArgs, _output: OutputFormat) -> std::result::Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    run_at_root(&repo_root, args, &mut std::io::stdout())
}

/// Run `specs validate-links` from a known `repo_root` (testable entry point).
///
/// # Errors
///
/// Returns an error if output cannot be written or findings are detected.
pub fn run_at_root(
    repo_root: &std::path::Path,
    args: &ValidateLinksArgs,
    w: &mut dyn std::io::Write,
) -> std::result::Result<(), Error> {
    let folders = resolve_folders(args.folder.as_ref(), &args.apps);
    let mut total = 0usize;
    for folder in &folders {
        let findings = validate_spec_links(repo_root, folder);
        total += findings.len();
        if findings.is_empty() {
            writeln!(w, "specs validate-links: 0 finding(s) for \"{folder}\"")?;
            continue;
        }
        for f in &findings {
            writeln!(w, "{}: HIGH: {}", f.file, f.evidence)?;
        }
    }
    if total > 0 {
        return Err(anyhow!("{total} finding(s) found by specs validate-links"));
    }
    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn resolve_folders_default() {
        let v = resolve_folders(None, &[]);
        assert_eq!(v.len(), 2);
    }

    #[test]
    fn run_at_root_broken_link_errors() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("specs/apps/x")).unwrap();
        std::fs::write(dir.path().join("specs/apps/x/a.md"), "[bad](./missing.md)").unwrap();
        let mut buf: Vec<u8> = Vec::new();
        let args = ValidateLinksArgs {
            folder: Some("specs/apps/x".to_string()),
            apps: vec![],
        };
        assert!(run_at_root(dir.path(), &args, &mut buf).is_err());
    }

    #[test]
    fn run_at_root_clean_passes() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("specs/apps/x")).unwrap();
        let mut buf: Vec<u8> = Vec::new();
        let args = ValidateLinksArgs {
            folder: Some("specs/apps/x".to_string()),
            apps: vec![],
        };
        run_at_root(dir.path(), &args, &mut buf).unwrap();
        assert!(String::from_utf8_lossy(&buf).contains("0 finding(s)"));
    }
}
