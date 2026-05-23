// envbackup — port of `apps/rhino-cli/internal/envbackup/`.

use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Error};
use serde::Serialize;
use walkdir::WalkDir;

pub const DEFAULT_MAX_SIZE: i64 = 1024 * 1024;
pub const DEFAULT_BACKUP_DIR: &str = "ose-open-env-backup";

pub fn default_skip_dirs() -> &'static [&'static str] {
    &[
        ".git",
        "node_modules",
        "bower_components",
        ".nx",
        ".next",
        ".turbo",
        ".cache",
        ".parcel-cache",
        ".nyc_output",
        "dist",
        "build",
        "coverage",
        "__pycache__",
        ".venv",
        "venv",
        "target",
        ".gradle",
        "vendor",
        "_build",
        "deps",
        ".elixir_ls",
        ".mix",
        ".dart_tool",
        ".cargo",
        "zig-cache",
        ".stack-work",
        "elm-stuff",
        "_deps",
        ".terraform",
        ".pulumi",
        "generated-contracts",
    ]
}

pub struct ConfigPattern {
    pub rel_path: &'static str,
    pub description: &'static str,
    pub category: &'static str,
}

pub fn default_config_patterns() -> &'static [ConfigPattern] {
    &[
        ConfigPattern {
            rel_path: ".claude/settings.local.json",
            description: "Claude Code local settings",
            category: "ai-tools",
        },
        ConfigPattern {
            rel_path: ".claude/settings.local.json.bkup",
            description: "Claude Code settings backup",
            category: "ai-tools",
        },
        ConfigPattern {
            rel_path: ".cursor/mcp.json",
            description: "Cursor MCP configuration",
            category: "ai-tools",
        },
        ConfigPattern {
            rel_path: ".windsurfrules",
            description: "Windsurf project rules",
            category: "ai-tools",
        },
        ConfigPattern {
            rel_path: ".clinerules",
            description: "Cline project rules",
            category: "ai-tools",
        },
        ConfigPattern {
            rel_path: ".aider.conf.yml",
            description: "Aider configuration",
            category: "ai-tools",
        },
        ConfigPattern {
            rel_path: ".aiderignore",
            description: "Aider ignore patterns",
            category: "ai-tools",
        },
        ConfigPattern {
            rel_path: ".continue/config.json",
            description: "Continue configuration",
            category: "ai-tools",
        },
        ConfigPattern {
            rel_path: ".gemini/settings.json",
            description: "Gemini CLI settings",
            category: "ai-tools",
        },
        ConfigPattern {
            rel_path: ".amazonq/mcp.json",
            description: "Amazon Q MCP configuration",
            category: "ai-tools",
        },
        ConfigPattern {
            rel_path: ".roomodes",
            description: "Roo Code custom modes",
            category: "ai-tools",
        },
        ConfigPattern {
            rel_path: "docker-compose.override.yml",
            description: "Docker Compose local overrides",
            category: "docker",
        },
        ConfigPattern {
            rel_path: "mise.local.toml",
            description: "mise local overrides",
            category: "version-mgrs",
        },
        ConfigPattern {
            rel_path: ".envrc",
            description: "direnv environment setup",
            category: "environment",
        },
    ]
}

#[derive(Debug, Clone, Default)]
pub struct Options {
    pub repo_root: PathBuf,
    pub backup_dir: PathBuf,
    pub skip_dirs: Vec<String>,
    pub max_size: i64,
    pub worktree_aware: bool,
    pub worktree_name: String,
    pub force: bool,
    pub include_config: bool,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct FileEntry {
    #[serde(rename = "relPath")]
    pub rel_path: String,
    #[serde(skip_serializing_if = "String::is_empty", rename = "absPath")]
    pub abs_path: String,
    #[serde(skip_serializing_if = "is_zero_i64")]
    pub size: i64,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub skipped: bool,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub reason: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub source: String,
}

fn is_zero_i64(n: &i64) -> bool {
    *n == 0
}

#[derive(Debug, Clone, Default)]
pub struct Result {
    pub direction: String,
    pub dir: String,
    pub files: Vec<FileEntry>,
    pub copied: usize,
    pub skipped: usize,
    pub errors: Vec<String>,
    pub worktree_name: String,
    pub cancelled: bool,
}

pub fn expand_tilde(path: &str) -> std::result::Result<PathBuf, Error> {
    if !path.starts_with('~') {
        return Ok(PathBuf::from(path));
    }
    let home = std::env::var_os("HOME").ok_or_else(|| anyhow!("HOME not set"))?;
    let mut p = PathBuf::from(home);
    let tail = &path[1..];
    if let Some(stripped) = tail.strip_prefix('/') {
        p.push(stripped);
    } else if !tail.is_empty() {
        p.push(tail);
    }
    Ok(p)
}

fn is_inside_repo(backup_dir: &Path, repo_root: &Path) -> bool {
    backup_dir.strip_prefix(repo_root).is_ok()
}

pub fn discover(opts: &Options) -> std::result::Result<Vec<FileEntry>, Error> {
    let max_size = if opts.max_size <= 0 {
        DEFAULT_MAX_SIZE
    } else {
        opts.max_size
    };
    let skip_set: std::collections::HashSet<&str> =
        opts.skip_dirs.iter().map(|s| s.as_str()).collect();

    let mut entries: Vec<FileEntry> = Vec::new();
    let mut walker = WalkDir::new(&opts.repo_root).into_iter();
    loop {
        let item = walker.next();
        let entry = match item {
            None => break,
            Some(Err(_)) => continue,
            Some(Ok(e)) => e,
        };
        let path = entry.path().to_path_buf();
        let base = entry.file_name().to_string_lossy().into_owned();

        if entry.file_type().is_dir() {
            if path == opts.repo_root {
                continue;
            }
            // hidden dirs starting with "."
            if base.starts_with('.') {
                walker.skip_current_dir();
                continue;
            }
            if skip_set.contains(base.as_str()) {
                walker.skip_current_dir();
                continue;
            }
            continue;
        }
        if !base.starts_with(".env") {
            continue;
        }
        let rel = match path.strip_prefix(&opts.repo_root) {
            Ok(r) => r.to_string_lossy().into_owned(),
            Err(_) => continue,
        };
        let meta = match fs::symlink_metadata(&path) {
            Ok(m) => m,
            Err(_) => continue,
        };
        let ft = meta.file_type();
        if ft.is_symlink() {
            entries.push(FileEntry {
                rel_path: rel,
                abs_path: path.to_string_lossy().into_owned(),
                skipped: true,
                reason: "symlink".to_string(),
                ..Default::default()
            });
            continue;
        }
        let size = meta.len() as i64;
        if size > max_size {
            entries.push(FileEntry {
                rel_path: rel,
                abs_path: path.to_string_lossy().into_owned(),
                size,
                skipped: true,
                reason: "exceeds 1 MB".to_string(),
                ..Default::default()
            });
            continue;
        }
        entries.push(FileEntry {
            rel_path: rel,
            abs_path: path.to_string_lossy().into_owned(),
            size,
            ..Default::default()
        });
    }
    entries.sort_by(|a, b| a.rel_path.cmp(&b.rel_path));
    Ok(entries)
}

pub fn discover_config(
    repo_root: &Path,
    patterns: &[ConfigPattern],
    max_size: i64,
) -> std::result::Result<Vec<FileEntry>, Error> {
    let max = if max_size <= 0 {
        DEFAULT_MAX_SIZE
    } else {
        max_size
    };
    let mut entries: Vec<FileEntry> = Vec::new();
    for p in patterns {
        let abs = repo_root.join(p.rel_path);
        let meta = match fs::symlink_metadata(&abs) {
            Ok(m) => m,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
            Err(e) => return Err(anyhow!("lstat {}: {e}", p.rel_path)),
        };
        if meta.is_dir() {
            continue;
        }
        if meta.file_type().is_symlink() {
            entries.push(FileEntry {
                rel_path: p.rel_path.to_string(),
                abs_path: abs.to_string_lossy().into_owned(),
                skipped: true,
                reason: "symlink".to_string(),
                source: "config".to_string(),
                ..Default::default()
            });
            continue;
        }
        let size = meta.len() as i64;
        if size > max {
            entries.push(FileEntry {
                rel_path: p.rel_path.to_string(),
                abs_path: abs.to_string_lossy().into_owned(),
                size,
                skipped: true,
                reason: format!("file too large ({size} bytes > {max})"),
                source: "config".to_string(),
            });
            continue;
        }
        entries.push(FileEntry {
            rel_path: p.rel_path.to_string(),
            abs_path: abs.to_string_lossy().into_owned(),
            size,
            source: "config".to_string(),
            ..Default::default()
        });
    }
    entries.sort_by(|a, b| a.rel_path.cmp(&b.rel_path));
    Ok(entries)
}

fn copy_file(src: &Path, dst: &Path) -> std::result::Result<(), Error> {
    fs::copy(src, dst).with_context(|| format!("copy {} -> {}", src.display(), dst.display()))?;
    Ok(())
}

pub fn find_existing(entries: &[FileEntry], dest_root: &Path) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for e in entries {
        if e.skipped {
            continue;
        }
        let dst = dest_root.join(&e.rel_path);
        if dst.exists() {
            out.push(e.rel_path.clone());
        }
    }
    out
}

#[allow(clippy::collapsible_if, clippy::collapsible_match)]
pub fn backup(opts: &mut Options) -> std::result::Result<Result, Error> {
    if opts.max_size <= 0 {
        opts.max_size = DEFAULT_MAX_SIZE;
    }
    if opts.skip_dirs.is_empty() {
        opts.skip_dirs = default_skip_dirs().iter().map(|s| s.to_string()).collect();
    }
    let backup_dir_str = opts.backup_dir.to_string_lossy().into_owned();
    let expanded = expand_tilde(&backup_dir_str)?;
    opts.backup_dir = expanded;

    if is_inside_repo(&opts.backup_dir, &opts.repo_root) {
        return Err(anyhow!(
            "backup dir {} is inside repo root {}; choose a directory outside the repo",
            opts.backup_dir.display(),
            opts.repo_root.display()
        ));
    }

    let mut entries = discover(opts)?;
    if opts.include_config {
        for e in entries.iter_mut() {
            if e.source.is_empty() {
                e.source = "env".to_string();
            }
        }
        let config = discover_config(&opts.repo_root, default_config_patterns(), opts.max_size)?;
        entries.extend(config);
        entries.sort_by(|a, b| a.rel_path.cmp(&b.rel_path));
    }

    let dest_root = if opts.worktree_aware && !opts.worktree_name.is_empty() {
        opts.backup_dir.join(&opts.worktree_name)
    } else {
        opts.backup_dir.clone()
    };

    fs::create_dir_all(&dest_root).with_context(|| "create backup dir")?;

    let mut result = Result {
        direction: "backup".to_string(),
        dir: opts.backup_dir.to_string_lossy().into_owned(),
        files: entries.clone(),
        worktree_name: opts.worktree_name.clone(),
        ..Default::default()
    };

    for e in &entries {
        if e.skipped {
            result.skipped += 1;
            continue;
        }
        let dst = dest_root.join(&e.rel_path);
        if let Some(p) = dst.parent() {
            if let Err(err) = fs::create_dir_all(p) {
                result
                    .errors
                    .push(format!("mkdir for {}: {err}", e.rel_path));
                result.skipped += 1;
                continue;
            }
        }
        if let Err(err) = copy_file(Path::new(&e.abs_path), &dst) {
            result.errors.push(format!("copy {}: {err}", e.rel_path));
            result.skipped += 1;
            continue;
        }
        result.copied += 1;
    }
    Ok(result)
}

#[allow(clippy::collapsible_if, clippy::collapsible_match)]
pub fn restore(opts: &mut Options) -> std::result::Result<Result, Error> {
    if opts.max_size <= 0 {
        opts.max_size = DEFAULT_MAX_SIZE;
    }
    let backup_dir_str = opts.backup_dir.to_string_lossy().into_owned();
    opts.backup_dir = expand_tilde(&backup_dir_str)?;

    let src_root = if opts.worktree_aware && !opts.worktree_name.is_empty() {
        opts.backup_dir.join(&opts.worktree_name)
    } else {
        opts.backup_dir.clone()
    };
    if !src_root.exists() {
        return Err(anyhow!("backup dir does not exist: {}", src_root.display()));
    }

    let discover_opts = Options {
        repo_root: src_root.clone(),
        skip_dirs: vec![".git".to_string()],
        max_size: opts.max_size,
        ..Default::default()
    };
    let mut entries = discover(&discover_opts)?;
    if opts.include_config {
        for e in entries.iter_mut() {
            if e.source.is_empty() {
                e.source = "env".to_string();
            }
        }
        let config = discover_config(&src_root, default_config_patterns(), opts.max_size)?;
        entries.extend(config);
        entries.sort_by(|a, b| a.rel_path.cmp(&b.rel_path));
    }

    let mut result = Result {
        direction: "restore".to_string(),
        dir: opts.backup_dir.to_string_lossy().into_owned(),
        worktree_name: opts.worktree_name.clone(),
        ..Default::default()
    };

    for e in entries {
        let base = Path::new(&e.rel_path)
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default();
        if e.source != "config" && !base.starts_with(".env") {
            continue;
        }
        result.files.push(e.clone());
        if e.skipped {
            result.skipped += 1;
            continue;
        }
        let dst = opts.repo_root.join(&e.rel_path);
        if let Some(p) = dst.parent() {
            if let Err(err) = fs::create_dir_all(p) {
                result
                    .errors
                    .push(format!("mkdir for {}: {err}", e.rel_path));
                result.skipped += 1;
                continue;
            }
        }
        if let Err(err) = copy_file(Path::new(&e.abs_path), &dst) {
            result.errors.push(format!("copy {}: {err}", e.rel_path));
            result.skipped += 1;
            continue;
        }
        result.copied += 1;
    }
    Ok(result)
}

pub struct WorktreeInfo {
    pub is_worktree: bool,
    pub worktree_name: String,
}

pub fn detect_worktree(repo_root: &Path) -> std::result::Result<WorktreeInfo, Error> {
    let git_path = repo_root.join(".git");
    let meta = fs::symlink_metadata(&git_path).map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            anyhow!("no .git found at {}", repo_root.display())
        } else {
            anyhow!("stat .git: {e}")
        }
    })?;
    let name = repo_root
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    if meta.is_dir() {
        return Ok(WorktreeInfo {
            is_worktree: false,
            worktree_name: name,
        });
    }
    let data = fs::read_to_string(&git_path).map_err(|e| anyhow!("read .git file: {e}"))?;
    let line = data.trim();
    if !line.starts_with("gitdir:") {
        return Err(anyhow!(
            ".git file does not start with 'gitdir:' (got: {line:?})"
        ));
    }
    Ok(WorktreeInfo {
        is_worktree: true,
        worktree_name: name,
    })
}

// ---- Reporters ----

pub fn format_text(r: &Result, verbose: bool, quiet: bool) -> String {
    let mut sb = String::new();
    if r.cancelled {
        let label = if r.direction.is_empty() {
            "operation".to_string()
        } else {
            r.direction.clone()
        };
        let _ = writeln!(sb, "{} cancelled.", capitalize(&label));
        return sb;
    }
    if !quiet {
        for f in &r.files {
            if f.skipped {
                if verbose {
                    let _ = writeln!(sb, "  SKIPPED  {}  ({})", f.rel_path, f.reason);
                }
                continue;
            }
            let tag = if f.source == "config" {
                " [config]"
            } else {
                ""
            };
            let _ = writeln!(sb, "  {}  {}{tag}", r.direction.to_uppercase(), f.rel_path);
        }
        for e in &r.errors {
            let _ = writeln!(sb, "  WARNING  {e}");
        }
    }
    let label = if r.direction.is_empty() {
        "processed".to_string()
    } else {
        r.direction.clone()
    };
    let _ = write!(
        sb,
        "{} complete: {} file(s) {}d, {} skipped",
        capitalize(&label),
        r.copied,
        label,
        r.skipped
    );
    let config_count = r
        .files
        .iter()
        .filter(|f| f.source == "config" && !f.skipped)
        .count();
    if config_count > 0 {
        let _ = write!(sb, " ({config_count} config)");
    }
    if !r.worktree_name.is_empty() {
        let _ = write!(sb, "  [worktree: {}]", r.worktree_name);
    }
    sb.push('\n');
    sb
}

#[derive(Serialize)]
struct JsonOut<'a> {
    direction: &'a str,
    dir: &'a str,
    files: Vec<JsonEntry<'a>>,
    copied: usize,
    skipped: usize,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    errors: &'a Vec<String>,
    #[serde(skip_serializing_if = "str::is_empty", rename = "worktreeName")]
    worktree_name: &'a str,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    cancelled: bool,
}

#[derive(Serialize)]
struct JsonEntry<'a> {
    #[serde(rename = "relPath")]
    rel_path: &'a str,
    #[serde(skip_serializing_if = "is_zero_i64")]
    size: i64,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    skipped: bool,
    #[serde(skip_serializing_if = "str::is_empty")]
    reason: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    source: &'a str,
}

pub fn format_json(r: &Result) -> std::result::Result<String, Error> {
    let files: Vec<JsonEntry> = r
        .files
        .iter()
        .map(|f| JsonEntry {
            rel_path: &f.rel_path,
            size: f.size,
            skipped: f.skipped,
            reason: &f.reason,
            source: &f.source,
        })
        .collect();
    let out = JsonOut {
        direction: &r.direction,
        dir: &r.dir,
        files,
        copied: r.copied,
        skipped: r.skipped,
        errors: &r.errors,
        worktree_name: &r.worktree_name,
        cancelled: r.cancelled,
    };
    Ok(serde_json::to_string_pretty(&out)?)
}

pub fn format_markdown(r: &Result) -> String {
    let mut sb = String::new();
    let action = capitalize(&r.direction);
    let _ = writeln!(sb, "## {action} Report\n");
    let _ = writeln!(sb, "**Directory**: `{}`\n", r.dir);
    let _ = writeln!(
        sb,
        "**Copied**: {} | **Skipped**: {}\n",
        r.copied, r.skipped
    );
    if !r.worktree_name.is_empty() {
        let _ = writeln!(sb, "**Worktree**: `{}`\n", r.worktree_name);
    }
    if r.cancelled {
        let label = if r.direction.is_empty() {
            "operation".to_string()
        } else {
            r.direction.clone()
        };
        let _ = writeln!(sb, "_{} cancelled._", capitalize(&label));
        return sb;
    }
    if r.files.is_empty() {
        sb.push_str("_No .env files found._\n");
        return sb;
    }
    let has_config = r.files.iter().any(|f| f.source == "config");
    if has_config {
        sb.push_str("| File | Size (bytes) | Source | Status | Reason |\n");
        sb.push_str("|------|-------------|--------|--------|--------|\n");
    } else {
        sb.push_str("| File | Size (bytes) | Status | Reason |\n");
        sb.push_str("|------|-------------|--------|--------|\n");
    }
    for f in &r.files {
        let status = if f.skipped { "skipped" } else { "copied" };
        let reason: &str = if f.skipped { &f.reason } else { "" };
        let display = f.rel_path.replace('\\', "/");
        if has_config {
            let source = if f.source.is_empty() {
                "env"
            } else {
                &f.source
            };
            let _ = writeln!(
                sb,
                "| `{display}` | {} | {} | {} | {} |",
                f.size, source, status, reason
            );
        } else {
            let _ = writeln!(sb, "| `{display}` | {} | {} | {} |", f.size, status, reason);
        }
    }
    if !r.errors.is_empty() {
        sb.push_str("\n### Warnings\n");
        for e in &r.errors {
            let _ = writeln!(sb, "- {e}");
        }
    }
    sb
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn expand_tilde_replaces_home() {
        let r = expand_tilde("~/foo").unwrap();
        assert!(r.to_string_lossy().ends_with("/foo"));
    }

    #[test]
    fn expand_tilde_no_change() {
        let r = expand_tilde("/abs/path").unwrap();
        assert_eq!(r.to_string_lossy(), "/abs/path");
    }

    #[test]
    fn is_inside_repo_true_for_child() {
        assert!(is_inside_repo(
            Path::new("/repo/sub/backup"),
            Path::new("/repo"),
        ));
    }

    #[test]
    fn is_inside_repo_false_for_sibling() {
        assert!(!is_inside_repo(Path::new("/other"), Path::new("/repo"),));
    }

    #[test]
    fn discover_finds_env_files() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join(".env"), "x=1").unwrap();
        std::fs::write(dir.path().join(".env.local"), "y=2").unwrap();
        std::fs::write(dir.path().join("README.md"), "x").unwrap();
        let opts = Options {
            repo_root: dir.path().to_path_buf(),
            skip_dirs: default_skip_dirs().iter().map(|s| s.to_string()).collect(),
            max_size: DEFAULT_MAX_SIZE,
            ..Default::default()
        };
        let e = discover(&opts).unwrap();
        assert_eq!(e.len(), 2);
    }

    #[test]
    fn discover_skips_oversized() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join(".env"), vec![0u8; 100]).unwrap();
        let opts = Options {
            repo_root: dir.path().to_path_buf(),
            skip_dirs: default_skip_dirs().iter().map(|s| s.to_string()).collect(),
            max_size: 10,
            ..Default::default()
        };
        let e = discover(&opts).unwrap();
        assert_eq!(e.len(), 1);
        assert!(e[0].skipped);
        assert!(e[0].reason.contains("exceeds"));
    }

    #[test]
    fn discover_skips_skip_dirs() {
        let dir = tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("node_modules")).unwrap();
        std::fs::write(dir.path().join("node_modules/.env"), "x").unwrap();
        std::fs::write(dir.path().join(".env"), "y").unwrap();
        let opts = Options {
            repo_root: dir.path().to_path_buf(),
            skip_dirs: default_skip_dirs().iter().map(|s| s.to_string()).collect(),
            max_size: DEFAULT_MAX_SIZE,
            ..Default::default()
        };
        let e = discover(&opts).unwrap();
        assert_eq!(e.len(), 1);
        assert_eq!(e[0].rel_path, ".env");
    }

    #[test]
    fn discover_config_picks_up_existing() {
        let dir = tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join(".claude")).unwrap();
        std::fs::write(dir.path().join(".claude/settings.local.json"), "{}").unwrap();
        let e = discover_config(dir.path(), default_config_patterns(), DEFAULT_MAX_SIZE).unwrap();
        assert!(!e.is_empty());
        assert_eq!(e[0].source, "config");
    }

    #[test]
    fn find_existing_returns_intersection() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("a.txt"), "x").unwrap();
        let entries = vec![
            FileEntry {
                rel_path: "a.txt".into(),
                ..Default::default()
            },
            FileEntry {
                rel_path: "b.txt".into(),
                ..Default::default()
            },
        ];
        let r = find_existing(&entries, dir.path());
        assert_eq!(r, vec!["a.txt".to_string()]);
    }

    #[test]
    fn backup_rejects_inside_repo() {
        let dir = tempdir().unwrap();
        let mut opts = Options {
            repo_root: dir.path().to_path_buf(),
            backup_dir: dir.path().join("subdir"),
            ..Default::default()
        };
        let r = backup(&mut opts);
        assert!(r.is_err());
    }

    #[test]
    fn backup_copies_files() {
        let repo = tempdir().unwrap();
        let dest = tempdir().unwrap();
        std::fs::write(repo.path().join(".env"), "k=v").unwrap();
        let mut opts = Options {
            repo_root: repo.path().to_path_buf(),
            backup_dir: dest.path().to_path_buf(),
            skip_dirs: default_skip_dirs().iter().map(|s| s.to_string()).collect(),
            max_size: DEFAULT_MAX_SIZE,
            force: true,
            ..Default::default()
        };
        let r = backup(&mut opts).unwrap();
        assert_eq!(r.copied, 1);
        assert!(dest.path().join(".env").exists());
    }

    #[test]
    fn restore_copies_back() {
        let repo = tempdir().unwrap();
        let dest = tempdir().unwrap();
        std::fs::write(dest.path().join(".env"), "k=v").unwrap();
        let mut opts = Options {
            repo_root: repo.path().to_path_buf(),
            backup_dir: dest.path().to_path_buf(),
            max_size: DEFAULT_MAX_SIZE,
            force: true,
            ..Default::default()
        };
        let r = restore(&mut opts).unwrap();
        assert_eq!(r.copied, 1);
        assert!(repo.path().join(".env").exists());
    }

    #[test]
    fn restore_errors_when_backup_dir_missing() {
        let repo = tempdir().unwrap();
        let mut opts = Options {
            repo_root: repo.path().to_path_buf(),
            backup_dir: PathBuf::from("/nonexistent/path/xyz"),
            max_size: DEFAULT_MAX_SIZE,
            force: true,
            ..Default::default()
        };
        assert!(restore(&mut opts).is_err());
    }

    #[test]
    fn detect_worktree_normal_repo() {
        let dir = tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join(".git")).unwrap();
        let info = detect_worktree(dir.path()).unwrap();
        assert!(!info.is_worktree);
        assert!(!info.worktree_name.is_empty());
    }

    #[test]
    fn detect_worktree_linked() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join(".git"), "gitdir: /elsewhere/.git").unwrap();
        let info = detect_worktree(dir.path()).unwrap();
        assert!(info.is_worktree);
    }

    #[test]
    fn detect_worktree_no_git_fails() {
        let dir = tempdir().unwrap();
        let r = detect_worktree(dir.path());
        assert!(r.is_err());
    }

    fn sample_result() -> Result {
        Result {
            direction: "backup".to_string(),
            dir: "/tmp/bk".to_string(),
            files: vec![
                FileEntry {
                    rel_path: ".env".to_string(),
                    size: 10,
                    ..Default::default()
                },
                FileEntry {
                    rel_path: ".env.large".to_string(),
                    size: 999_999_999,
                    skipped: true,
                    reason: "exceeds 1 MB".to_string(),
                    ..Default::default()
                },
                FileEntry {
                    rel_path: ".envrc".to_string(),
                    size: 50,
                    source: "config".to_string(),
                    ..Default::default()
                },
            ],
            copied: 2,
            skipped: 1,
            ..Default::default()
        }
    }

    #[test]
    fn format_text_default() {
        let r = sample_result();
        let s = format_text(&r, false, false);
        assert!(s.contains("Backup complete"));
        assert!(s.contains("(1 config)"));
    }

    #[test]
    fn format_text_quiet_one_line() {
        let r = sample_result();
        let s = format_text(&r, false, true);
        assert!(s.contains("Backup complete"));
        assert!(!s.contains("BACKUP  .env"));
    }

    #[test]
    fn format_text_verbose_shows_skipped() {
        let r = sample_result();
        let s = format_text(&r, true, false);
        assert!(s.contains("SKIPPED  .env.large"));
    }

    #[test]
    fn format_text_cancelled() {
        let r = Result {
            direction: "backup".into(),
            cancelled: true,
            ..Default::default()
        };
        let s = format_text(&r, false, false);
        assert!(s.contains("cancelled"));
    }

    #[test]
    fn format_json_round_trip() {
        let r = sample_result();
        let s = format_json(&r).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["direction"], "backup");
        assert_eq!(v["copied"], 2);
    }

    #[test]
    fn format_markdown_basic() {
        let r = sample_result();
        let s = format_markdown(&r);
        assert!(s.contains("## Backup Report"));
        assert!(s.contains("**Copied**: 2"));
        assert!(s.contains("| File |"));
    }

    #[test]
    fn format_markdown_cancelled() {
        let r = Result {
            direction: "backup".into(),
            cancelled: true,
            ..Default::default()
        };
        let s = format_markdown(&r);
        assert!(s.contains("cancelled"));
    }

    #[test]
    fn capitalize_basic() {
        assert_eq!(capitalize("backup"), "Backup");
        assert_eq!(capitalize(""), "");
    }
}
