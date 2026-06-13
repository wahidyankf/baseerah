//! `specs validate-counts` — checks that spec folders have the minimum required file counts.
//!
//! Port of `apps/rhino-cli/cmd/specs_validate_counts.go`.

use anyhow::{Error, anyhow};
use clap::Args;

use crate::domain::cliout::OutputFormat;
use crate::internal::allowlist::apps_with_ddd;
use crate::internal::git;
use crate::internal::specs::validate_spec_counts;

/// CLI arguments for `specs validate-counts`.
#[derive(Args, Debug)]
pub struct ValidateCountsArgs {
    /// Single positional folder.
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

/// Run the `specs validate-counts` command.
///
/// # Errors
///
/// Returns an error if the git root cannot be found or findings are detected.
pub fn run(args: &ValidateCountsArgs, _output: OutputFormat) -> std::result::Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    run_at_root(&repo_root, args, &mut std::io::stdout())
}

/// Run `specs validate-counts` from a known `repo_root` (testable entry point).
///
/// # Errors
///
/// Returns an error if output cannot be written or findings are detected.
pub fn run_at_root(
    repo_root: &std::path::Path,
    args: &ValidateCountsArgs,
    w: &mut dyn std::io::Write,
) -> std::result::Result<(), Error> {
    let folders = resolve_folders(args.folder.as_ref(), &args.apps);
    let mut total = 0usize;
    for folder in &folders {
        let findings = validate_spec_counts(repo_root, folder);
        total += findings.len();
        if findings.is_empty() {
            writeln!(w, "specs validate-counts: 0 finding(s) for \"{folder}\"")?;
            continue;
        }
        for f in &findings {
            writeln!(w, "{}: {}: {}", f.file, f.criticality, f.evidence)?;
        }
    }
    if total > 0 {
        return Err(anyhow!("{total} finding(s) found by specs validate-counts"));
    }
    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn resolve_folders_positional() {
        let v = resolve_folders(Some(&"x".to_string()), &[]);
        assert_eq!(v, vec!["x".to_string()]);
    }

    #[test]
    fn resolve_folders_default() {
        let v = resolve_folders(None, &[]);
        assert_eq!(v.len(), 2);
        assert!(v[0].starts_with("specs/apps/"));
    }

    #[test]
    fn resolve_folders_flag() {
        let v = resolve_folders(None, &["a".to_string()]);
        assert_eq!(v, vec!["specs/apps/a".to_string()]);
    }

    #[test]
    fn run_at_root_missing_folder_errors() {
        let dir = tempfile::tempdir().unwrap();
        let mut buf: Vec<u8> = Vec::new();
        let args = ValidateCountsArgs {
            folder: Some("specs/apps/missing".to_string()),
            apps: vec![],
        };
        assert!(run_at_root(dir.path(), &args, &mut buf).is_err());
    }

    #[test]
    fn run_at_root_clean_folder_passes() {
        let dir = tempfile::tempdir().unwrap();
        for sub in crate::internal::specs::required_spec_folders() {
            let p = dir.path().join("specs/apps/x").join(sub);
            std::fs::create_dir_all(&p).unwrap();
            std::fs::write(p.join("a.md"), "x").unwrap();
        }
        let mut buf: Vec<u8> = Vec::new();
        let args = ValidateCountsArgs {
            folder: Some("specs/apps/x".to_string()),
            apps: vec![],
        };
        assert!(run_at_root(dir.path(), &args, &mut buf).is_ok());
        assert!(String::from_utf8_lossy(&buf).contains("0 finding(s)"));
    }
}
