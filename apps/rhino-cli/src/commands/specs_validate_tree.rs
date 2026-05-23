// Port of `apps/rhino-cli/cmd/specs_validate_tree.go`.

use anyhow::{anyhow, Error};
use clap::Args;

use crate::internal::allowlist::apps_with_ddd;
use crate::internal::cliout::OutputFormat;
use crate::internal::gitutil;
use crate::internal::specs::{validate_spec_gherkin_domains, validate_spec_tree};

#[derive(Args, Debug)]
pub struct ValidateTreeArgs {
    #[arg(value_name = "app")]
    pub app: Option<String>,
    #[arg(long = "apps", value_delimiter = ',')]
    pub apps: Vec<String>,
}

fn resolve_apps(positional: Option<&String>, flag: &[String]) -> Vec<String> {
    if let Some(p) = positional {
        return vec![p.clone()];
    }
    if !flag.is_empty() {
        return flag.to_vec();
    }
    apps_with_ddd().iter().map(|s| (*s).to_string()).collect()
}

pub fn run(args: &ValidateTreeArgs, _output: OutputFormat) -> std::result::Result<(), Error> {
    let repo_root =
        gitutil::find_git_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    run_at_root(&repo_root, args, &mut std::io::stdout())
}

pub fn run_at_root(
    repo_root: &std::path::Path,
    args: &ValidateTreeArgs,
    w: &mut dyn std::io::Write,
) -> std::result::Result<(), Error> {
    let apps = resolve_apps(args.app.as_ref(), &args.apps);
    let mut total = 0usize;
    for app in &apps {
        let mut findings = validate_spec_tree(repo_root, app);
        findings.extend(validate_spec_gherkin_domains(repo_root, app));
        if findings.is_empty() {
            writeln!(w, "specs validate-tree: 0 finding(s) for \"{app}\"")?;
            continue;
        }
        for f in &findings {
            writeln!(w, "{}: {}: {}", f.file, f.criticality, f.evidence)?;
        }
        total += findings.len();
    }
    if total > 0 {
        return Err(anyhow!("{total} finding(s) found by specs validate-tree"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_apps_default() {
        let v = resolve_apps(None, &[]);
        assert_eq!(v.len(), 5);
    }

    #[test]
    fn resolve_apps_positional() {
        let v = resolve_apps(Some(&"x".to_string()), &[]);
        assert_eq!(v, vec!["x"]);
    }

    #[test]
    fn run_at_root_missing_app_errors() {
        let dir = tempfile::tempdir().unwrap();
        let mut buf: Vec<u8> = Vec::new();
        let args = ValidateTreeArgs {
            app: Some("missing".to_string()),
            apps: vec![],
        };
        assert!(run_at_root(dir.path(), &args, &mut buf).is_err());
    }

    #[test]
    fn run_at_root_complete_tree_passes() {
        let dir = tempfile::tempdir().unwrap();
        for sub in crate::internal::specs::required_spec_folders() {
            let p = dir.path().join("specs/apps/x").join(sub);
            std::fs::create_dir_all(&p).unwrap();
            std::fs::write(p.join("README.md"), "x").unwrap();
        }
        let mut buf: Vec<u8> = Vec::new();
        let args = ValidateTreeArgs {
            app: Some("x".to_string()),
            apps: vec![],
        };
        assert!(run_at_root(dir.path(), &args, &mut buf).is_ok());
    }
}
