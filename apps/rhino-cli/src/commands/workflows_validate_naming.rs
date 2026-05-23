// Port of `apps/rhino-cli/cmd/workflows_validate_naming.go`.

use std::fs;
use std::path::Path;

use anyhow::{anyhow, Context, Error};
use clap::Args;
use walkdir::WalkDir;

use crate::internal::cliout::OutputFormat;
use crate::internal::gitutil;
use crate::internal::naming::{self, Violation};

use super::naming_reporter::{format_json, format_markdown, format_text};

const WORKFLOW_TYPES: &[&str] = &["quality-gate", "execution", "setup"];

#[derive(Args, Debug)]
pub struct ValidateNamingArgs {}

pub fn run(
    _args: &ValidateNamingArgs,
    output_format: OutputFormat,
) -> std::result::Result<(), Error> {
    let repo_root =
        gitutil::find_git_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    let violations = workflows_validate_naming(&repo_root.to_string_lossy())?;

    match output_format {
        OutputFormat::Text => print!("{}", format_text("Workflows", &violations, false, false)),
        OutputFormat::Json => print!("{}", format_json("workflows", &violations)?),
        OutputFormat::Markdown => print!("{}", format_markdown("Workflows", &violations)),
    }

    if !violations.is_empty() {
        return Err(anyhow!("{} naming violation(s) found", violations.len()));
    }
    Ok(())
}

fn workflows_validate_naming(repo_root: &str) -> std::result::Result<Vec<Violation>, Error> {
    let root = Path::new(repo_root)
        .join("repo-governance")
        .join("workflows");
    let files = list_workflow_files(&root);
    let mut violations = Vec::new();
    for path in files {
        if let Some(v) = naming::validate_suffix(&path, WORKFLOW_TYPES, "type-suffix") {
            violations.push(v);
        }
        let content = fs::read(&path).with_context(|| format!("read {path}"))?;
        if let Some(v) = naming::validate_frontmatter_name(&path, &content) {
            violations.push(v);
        }
    }
    violations.sort_by(|a, b| a.path.cmp(&b.path).then(a.kind.cmp(&b.kind)));
    Ok(violations)
}

fn list_workflow_files(root: &Path) -> Vec<String> {
    if !root.exists() {
        return Vec::new();
    }
    let mut files = Vec::new();
    let walker = WalkDir::new(root).into_iter().filter_entry(|e| {
        if e.file_type().is_dir() && e.path() != root {
            let name = e.file_name().to_string_lossy().to_string();
            if name == "meta" {
                return false;
            }
        }
        true
    });
    for entry in walker.flatten() {
        if !entry.file_type().is_file() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        if name == "README.md" || !name.ends_with(".md") {
            continue;
        }
        files.push(entry.path().to_string_lossy().to_string());
    }
    files.sort();
    files
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn list_workflow_files_skips_meta_and_readme() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("repo-governance/workflows");
        std::fs::create_dir_all(root.join("meta")).unwrap();
        std::fs::write(root.join("README.md"), "x").unwrap();
        std::fs::write(root.join("meta/foo.md"), "x").unwrap();
        std::fs::write(root.join("plan-quality-gate.md"), "x").unwrap();
        let files = list_workflow_files(&root);
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("plan-quality-gate.md"));
    }

    #[test]
    fn workflows_validate_naming_clean_returns_empty() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("repo-governance/workflows");
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(
            root.join("plan-quality-gate.md"),
            "---\nname: plan-quality-gate\n---\n",
        )
        .unwrap();
        let result = workflows_validate_naming(&tmp.path().to_string_lossy()).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn workflows_validate_naming_detects_bad_suffix() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("repo-governance/workflows");
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(root.join("foo-bar.md"), "---\nname: foo-bar\n---\n").unwrap();
        let result = workflows_validate_naming(&tmp.path().to_string_lossy()).unwrap();
        assert!(result.iter().any(|v| v.kind == "type-suffix"));
    }
}
