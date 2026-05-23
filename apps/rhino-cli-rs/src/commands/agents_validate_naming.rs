// Port of `apps/rhino-cli/cmd/agents_validate_naming.go`.

use std::fs;
use std::path::Path;

use anyhow::{anyhow, Context, Error};
use clap::Args;

use crate::internal::cliout::OutputFormat;
use crate::internal::gitutil;
use crate::internal::naming::{self, Violation};

use super::naming_reporter::{format_json, format_markdown, format_text};

const AGENT_ROLES: &[&str] = &["maker", "checker", "fixer", "dev", "deployer", "manager"];

#[derive(Args, Debug)]
pub struct ValidateNamingArgs {}

pub fn run(
    _args: &ValidateNamingArgs,
    output_format: OutputFormat,
) -> std::result::Result<(), Error> {
    let repo_root =
        gitutil::find_git_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    let violations = agents_validate_naming(&repo_root.to_string_lossy())?;

    match output_format {
        OutputFormat::Text => print!("{}", format_text("Agents", &violations, false, false)),
        OutputFormat::Json => print!("{}", format_json("agents", &violations)?),
        OutputFormat::Markdown => print!("{}", format_markdown("Agents", &violations)),
    }

    if !violations.is_empty() {
        return Err(anyhow!("{} naming violation(s) found", violations.len()));
    }
    Ok(())
}

fn agents_validate_naming(repo_root: &str) -> std::result::Result<Vec<Violation>, Error> {
    let claude_dir = Path::new(repo_root).join(".claude").join("agents");
    let opencode_dir = Path::new(repo_root).join(".opencode").join("agents");

    let claude_files = list_agent_files(&claude_dir);
    let opencode_files = list_agent_files(&opencode_dir);

    let mut violations: Vec<Violation> = Vec::new();

    // Suffix + frontmatter for .claude/agents/*.md
    for path in &claude_files {
        if let Some(v) = naming::validate_suffix(path, AGENT_ROLES, "role-suffix") {
            violations.push(v);
        }
        let content = fs::read(path).with_context(|| format!("read {path}"))?;
        if let Some(v) = naming::validate_frontmatter_name(path, &content) {
            violations.push(v);
        }
    }
    // Suffix-only for .opencode/agents/*.md
    for path in &opencode_files {
        if let Some(v) = naming::validate_suffix(path, AGENT_ROLES, "role-suffix") {
            violations.push(v);
        }
    }
    // Mirror-drift
    violations.extend(naming::validate_mirror(&claude_files, &opencode_files));

    violations.sort_by(|a, b| a.path.cmp(&b.path).then(a.kind.cmp(&b.kind)));
    Ok(violations)
}

fn list_agent_files(dir: &Path) -> Vec<String> {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return Vec::new(),
    };
    let mut files = Vec::new();
    for entry in entries.flatten() {
        if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        if name == "README.md" || name == "ci-monitor-subagent.md" {
            continue;
        }
        if !name.ends_with(".md") {
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
    fn list_agent_files_skips_readme_and_subagent() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path().join(".claude/agents");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("README.md"), "x").unwrap();
        std::fs::write(dir.join("ci-monitor-subagent.md"), "x").unwrap();
        std::fs::write(dir.join("foo-maker.md"), "x").unwrap();
        let files = list_agent_files(&dir);
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("foo-maker.md"));
    }

    #[test]
    fn list_agent_files_missing_dir_is_empty() {
        let tmp = TempDir::new().unwrap();
        let files = list_agent_files(&tmp.path().join("nonexistent"));
        assert!(files.is_empty());
    }

    #[test]
    fn agents_validate_naming_clean_repo_is_empty() {
        let tmp = TempDir::new().unwrap();
        let cd = tmp.path().join(".claude/agents");
        let od = tmp.path().join(".opencode/agents");
        std::fs::create_dir_all(&cd).unwrap();
        std::fs::create_dir_all(&od).unwrap();
        std::fs::write(cd.join("foo-maker.md"), "---\nname: foo-maker\n---\n").unwrap();
        std::fs::write(od.join("foo-maker.md"), "---\n---\n").unwrap();
        let result = agents_validate_naming(&tmp.path().to_string_lossy()).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn agents_validate_naming_detects_suffix_and_mirror() {
        let tmp = TempDir::new().unwrap();
        let cd = tmp.path().join(".claude/agents");
        let od = tmp.path().join(".opencode/agents");
        std::fs::create_dir_all(&cd).unwrap();
        std::fs::create_dir_all(&od).unwrap();
        // Bad suffix + missing mirror in .opencode
        std::fs::write(cd.join("foo-bar.md"), "---\nname: foo-bar\n---\n").unwrap();
        let result = agents_validate_naming(&tmp.path().to_string_lossy()).unwrap();
        let kinds: Vec<&str> = result.iter().map(|v| v.kind.as_str()).collect();
        assert!(kinds.contains(&"role-suffix"));
        assert!(kinds.contains(&"mirror-drift"));
    }
}
