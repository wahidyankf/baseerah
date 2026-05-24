// Port of `apps/rhino-cli/cmd/docs_validate_mermaid.go`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Error, anyhow};
use clap::Args;
use walkdir::WalkDir;

use crate::internal::cliout::OutputFormat;
use crate::internal::git;
use crate::internal::mermaid::{
    MermaidBlock, ValidateOptions, extract_blocks, format_json, format_markdown, format_text,
    validate_blocks,
};

#[derive(Args, Debug)]
pub struct ValidateMermaidArgs {
    /// Only validate staged files (pre-commit use).
    #[arg(long = "staged-only")]
    pub staged_only: bool,
    /// Only validate files changed since upstream (pre-push use).
    #[arg(long = "changed-only")]
    pub changed_only: bool,
    /// Max characters in a node label.
    #[arg(long = "max-label-len", default_value_t = 30)]
    pub max_label_len: usize,
    /// Max nodes at the same rank.
    #[arg(long = "max-width", default_value_t = 4)]
    pub max_width: usize,
    /// Depth threshold for the both-exceeded warning (0 = unlimited).
    #[arg(long = "max-depth", default_value_t = 0)]
    pub max_depth: usize,
    /// Max direct child nodes per subgraph.
    #[arg(long = "max-subgraph-nodes", default_value_t = 6)]
    pub max_subgraph_nodes: usize,
    /// Optional positional paths to scan.
    pub positional: Vec<String>,
}

const SKIP_DIRS: &[&str] = &[".next", "node_modules", ".git"];

pub fn run(
    args: &ValidateMermaidArgs,
    output_format: OutputFormat,
) -> std::result::Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;

    let md_files: Vec<PathBuf> = if args.staged_only {
        get_staged_files(&repo_root)?
    } else if args.changed_only {
        get_changed_files(&repo_root)?
    } else if !args.positional.is_empty() {
        collect_md_files(&repo_root, &args.positional)
    } else {
        collect_md_default_dirs(&repo_root)
    };

    let mut all_blocks: Vec<MermaidBlock> = Vec::new();
    let mut file_set: std::collections::HashSet<String> = std::collections::HashSet::new();
    for f in &md_files {
        let Ok(content) = fs::read_to_string(f) else {
            continue;
        };
        let blocks = extract_blocks(&f.to_string_lossy(), &content);
        if !blocks.is_empty() {
            file_set.insert(f.to_string_lossy().to_string());
        }
        all_blocks.extend(blocks);
    }

    let max_depth = if args.max_depth == 0 {
        usize::MAX
    } else {
        args.max_depth
    };
    let opts = ValidateOptions {
        max_label_len: args.max_label_len,
        max_width: args.max_width,
        max_depth,
        max_subgraph_nodes: args.max_subgraph_nodes,
    };
    let mut result = validate_blocks(all_blocks, opts);
    result.files_scanned = file_set.len();

    match output_format {
        OutputFormat::Text => print!("{}", format_text(&result, false, false)),
        OutputFormat::Json => print!("{}", format_json(&result)?),
        OutputFormat::Markdown => print!("{}", format_markdown(&result)),
    }

    if !result.violations.is_empty() {
        return Err(anyhow!("found {} violation(s)", result.violations.len()));
    }
    Ok(())
}

fn get_staged_files(repo_root: &Path) -> std::result::Result<Vec<PathBuf>, Error> {
    let out = Command::new("git")
        .args([
            "-C",
            &repo_root.to_string_lossy(),
            "diff",
            "--cached",
            "--name-only",
            "--diff-filter=ACMR",
        ])
        .output()
        .context("git diff --cached")?;
    let text = String::from_utf8_lossy(&out.stdout);
    Ok(filter_md_paths(repo_root, text.lines()))
}

fn get_changed_files(repo_root: &Path) -> std::result::Result<Vec<PathBuf>, Error> {
    let out = Command::new("git")
        .args([
            "-C",
            &repo_root.to_string_lossy(),
            "diff",
            "--name-only",
            "@{u}..HEAD",
        ])
        .output();
    let text = match out {
        Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
        Err(_) => return Ok(collect_md_default_dirs(repo_root)),
    };
    let files = filter_md_paths(repo_root, text.lines());
    if files.is_empty() {
        Ok(collect_md_default_dirs(repo_root))
    } else {
        Ok(files)
    }
}

fn filter_md_paths<'a, I: IntoIterator<Item = &'a str>>(
    repo_root: &Path,
    paths: I,
) -> Vec<PathBuf> {
    paths
        .into_iter()
        .filter(|p| !p.is_empty() && p.ends_with(".md"))
        .map(|p| repo_root.join(p))
        .collect()
}

fn collect_md_files(repo_root: &Path, paths: &[String]) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for p in paths {
        let abs = if Path::new(p).is_absolute() {
            PathBuf::from(p)
        } else {
            repo_root.join(p)
        };
        files.extend(walk_md_files(&abs));
    }
    files
}

fn collect_md_default_dirs(repo_root: &Path) -> Vec<PathBuf> {
    let dirs = ["docs", "repo-governance", ".claude", "plans"];
    let mut files = Vec::new();
    for d in &dirs {
        let dp = repo_root.join(d);
        if dp.exists() {
            files.extend(walk_md_files(&dp));
        }
    }
    if let Ok(entries) = fs::read_dir(repo_root) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_file() && p.extension().is_some_and(|e| e == "md") {
                files.push(p);
            }
        }
    }
    files
}

fn walk_md_files(dir: &Path) -> Vec<PathBuf> {
    if !dir.exists() {
        return Vec::new();
    }
    let mut files = Vec::new();
    let walker = WalkDir::new(dir).into_iter().filter_entry(|e| {
        if e.file_type().is_dir() {
            let name = e.file_name().to_string_lossy().to_string();
            !SKIP_DIRS.contains(&name.as_str())
        } else {
            true
        }
    });
    for entry in walker.flatten() {
        if entry.file_type().is_file() && entry.path().extension().is_some_and(|e| e == "md") {
            files.push(entry.path().to_path_buf());
        }
    }
    files
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn filter_md_paths_filters_md_only() {
        let tmp = TempDir::new().unwrap();
        let inputs = ["a.md", "b.txt", "", "docs/c.md"];
        let filtered = filter_md_paths(tmp.path(), inputs.iter().copied());
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn walk_md_files_skips_node_modules() {
        let tmp = TempDir::new().unwrap();
        let nm = tmp.path().join("node_modules");
        std::fs::create_dir(&nm).unwrap();
        std::fs::write(nm.join("ignored.md"), "x").unwrap();
        std::fs::write(tmp.path().join("kept.md"), "x").unwrap();
        let files = walk_md_files(tmp.path());
        assert_eq!(files.len(), 1);
        assert!(files[0].to_string_lossy().ends_with("kept.md"));
    }

    #[test]
    fn collect_md_files_handles_absolute_paths() {
        let tmp = TempDir::new().unwrap();
        let p = tmp.path().join("foo.md");
        std::fs::write(&p, "x").unwrap();
        let collected = collect_md_files(
            Path::new("/nonexistent"),
            &[p.to_string_lossy().to_string()],
        );
        assert_eq!(collected.len(), 1);
    }
}
