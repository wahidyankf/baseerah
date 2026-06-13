//! `specs validate-adoption` — checks that spec directories follow required structure for DDD apps.
//!
//! Port of `apps/rhino-cli/cmd/specs_validate_adoption.go`.

use anyhow::{Error, anyhow};
use clap::Args;

use crate::domain::cliout::OutputFormat;
use crate::internal::allowlist::apps_with_ddd;
use crate::internal::git;
use crate::internal::specs::validate_spec_adoption;

/// CLI arguments for `specs validate-adoption`.
#[derive(Args, Debug)]
pub struct ValidateAdoptionArgs {
    /// Single positional app name.
    #[arg(value_name = "app")]
    pub app: Option<String>,
    /// Comma-separated list of apps to validate.
    #[arg(long = "apps", value_delimiter = ',')]
    pub apps: Vec<String>,
}

/// Resolve the list of apps to validate from positional and flag inputs.
fn resolve_apps(positional: Option<&String>, flag: &[String]) -> Vec<String> {
    if let Some(p) = positional {
        return vec![p.clone()];
    }
    if !flag.is_empty() {
        return flag.to_vec();
    }
    apps_with_ddd().iter().map(|s| (*s).to_string()).collect()
}

/// Run the `specs validate-adoption` command.
///
/// # Errors
///
/// Returns an error if the git root cannot be found or findings are detected.
pub fn run(args: &ValidateAdoptionArgs, _output: OutputFormat) -> std::result::Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    run_at_root(&repo_root, args, &mut std::io::stdout())
}

/// Run `specs validate-adoption` from a known `repo_root` (testable entry point).
///
/// # Errors
///
/// Returns an error if output cannot be written or findings are detected.
pub fn run_at_root(
    repo_root: &std::path::Path,
    args: &ValidateAdoptionArgs,
    w: &mut dyn std::io::Write,
) -> std::result::Result<(), Error> {
    let apps = resolve_apps(args.app.as_ref(), &args.apps);
    let mut total = 0usize;
    for app in &apps {
        let findings = validate_spec_adoption(repo_root, app);
        if findings.is_empty() {
            writeln!(w, "specs validate-adoption: 0 finding(s) for \"{app}\"")?;
            continue;
        }
        for f in &findings {
            writeln!(w, "{}: HIGH: {}", f.file, f.evidence)?;
        }
        total += findings.len();
    }
    if total > 0 {
        return Err(anyhow!(
            "{total} finding(s) found by specs validate-adoption"
        ));
    }
    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn resolve_apps_positional() {
        let v = resolve_apps(Some(&"foo".to_string()), &[]);
        assert_eq!(v, vec!["foo".to_string()]);
    }

    #[test]
    fn resolve_apps_flag() {
        let v = resolve_apps(None, &["a".to_string(), "b".to_string()]);
        assert_eq!(v, vec!["a", "b"]);
    }

    #[test]
    fn resolve_apps_default() {
        let v = resolve_apps(None, &[]);
        assert_eq!(v.len(), 2);
    }

    #[test]
    fn run_at_root_clean_corpus() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("specs/apps/x/behavior")).unwrap();
        std::fs::create_dir_all(dir.path().join("specs/apps/x/ddd")).unwrap();
        std::fs::write(dir.path().join("specs/apps/x/behavior/a.feature"), "x").unwrap();
        std::fs::write(
            dir.path().join("specs/apps/x/ddd/bounded-contexts.yaml"),
            "y",
        )
        .unwrap();
        let mut buf: Vec<u8> = Vec::new();
        let args = ValidateAdoptionArgs {
            app: Some("x".to_string()),
            apps: vec![],
        };
        run_at_root(dir.path(), &args, &mut buf).unwrap();
        assert!(String::from_utf8_lossy(&buf).contains("0 finding(s)"));
    }

    #[test]
    fn run_at_root_findings_error() {
        let dir = tempfile::tempdir().unwrap();
        let mut buf: Vec<u8> = Vec::new();
        let args = ValidateAdoptionArgs {
            app: Some("missing".to_string()),
            apps: vec![],
        };
        let r = run_at_root(dir.path(), &args, &mut buf);
        assert!(r.is_err());
    }
}
