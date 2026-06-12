//! Env-backup module — port of `apps/rhino-cli/internal/envbackup/`.
//!
//! Discovers `.env*` files (and optionally AI-tool config files) under a
//! repository root, then copies them to or from an external backup directory.
//!
//! Primary entry points: [`backup`], [`restore`], [`discover`],
//! [`discover_config`], [`format_text`], [`format_json`], [`format_markdown`].

use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Error, anyhow};
use serde::Serialize;
use walkdir::WalkDir;

/// Maximum file size (in bytes) that will be backed up or restored (1 MiB).
pub const DEFAULT_MAX_SIZE: i64 = 1024 * 1024;

/// Default name of the backup directory placed outside the repository.
pub const DEFAULT_BACKUP_DIR: &str = "ose-public-env-backup";

/// Returns the default list of directory names that the walker skips.
///
/// Includes build artifacts, package caches, IDE directories, and other
/// directories that are unlikely to contain `.env` files.
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

/// Describes a well-known config file that can be included in a backup.
pub struct ConfigPattern {
    /// Relative path from the repo root (e.g. `".claude/settings.local.json"`).
    pub rel_path: &'static str,
    /// Human-readable description shown in reports.
    pub description: &'static str,
    /// Logical category (e.g. `"ai-tools"`, `"docker"`, `"environment"`).
    pub category: &'static str,
}

/// Returns the default list of config-file patterns checked during
/// `--include-config` backup/restore operations.
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

/// Options for a [`backup`] or [`restore`] operation.
#[derive(Debug, Clone, Default)]
pub struct Options {
    /// Absolute path to the repository root to scan.
    pub repo_root: PathBuf,
    /// Destination directory for backups (or source for restores).
    pub backup_dir: PathBuf,
    /// Directory names to skip during the walk; defaults to [`default_skip_dirs`] when empty.
    pub skip_dirs: Vec<String>,
    /// Maximum file size in bytes; defaults to [`DEFAULT_MAX_SIZE`] when `<= 0`.
    pub max_size: i64,
    /// When `true`, appends [`worktree_name`](Options::worktree_name) as a subdirectory.
    pub worktree_aware: bool,
    /// Worktree name used when [`worktree_aware`](Options::worktree_aware) is `true`.
    pub worktree_name: String,
    /// When `true`, overwrite existing files in the destination without prompting.
    pub force: bool,
    /// When `true`, also back up / restore the config files from [`default_config_patterns`].
    pub include_config: bool,
    /// When `true`, run discovery but perform no filesystem writes.
    pub dry_run: bool,
}

/// A single file discovered during a scan, along with its copy outcome.
#[derive(Debug, Clone, Default, Serialize)]
pub struct FileEntry {
    /// Repository-relative path (e.g. `"apps/foo/.env.local"`).
    #[serde(rename = "relPath")]
    pub rel_path: String,
    /// Absolute path at the time of discovery (omitted when empty).
    #[serde(skip_serializing_if = "String::is_empty", rename = "absPath")]
    pub abs_path: String,
    /// File size in bytes (omitted when zero).
    #[serde(skip_serializing_if = "is_zero_i64")]
    pub size: i64,
    /// When `true`, this file was not copied (see [`reason`](FileEntry::reason)).
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub skipped: bool,
    /// Human-readable explanation of why the file was skipped (omitted when empty).
    #[serde(skip_serializing_if = "String::is_empty")]
    pub reason: String,
    /// `"env"` for `.env*` files, `"config"` for config-pattern files (omitted when empty).
    #[serde(skip_serializing_if = "String::is_empty")]
    pub source: String,
}

/// Returns `true` when `n` is zero; used to suppress zero-valued size fields in JSON.
#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_zero_i64(n: &i64) -> bool {
    *n == 0
}

/// Outcome of a [`backup`] or [`restore`] operation.
#[derive(Debug, Clone, Default)]
pub struct Result {
    /// `"backup"` or `"restore"`.
    pub direction: String,
    /// Absolute path to the backup directory.
    pub dir: String,
    /// All files considered (both copied and skipped).
    pub files: Vec<FileEntry>,
    /// Number of files successfully copied.
    pub copied: usize,
    /// Number of files skipped (oversized, symlinks, or copy errors).
    pub skipped: usize,
    /// Non-fatal errors encountered during the operation.
    pub errors: Vec<String>,
    /// Worktree name used when the backup was worktree-aware.
    pub worktree_name: String,
    /// When `true`, the user cancelled the operation before it started.
    pub cancelled: bool,
    /// When `true`, no filesystem writes were performed (dry-run mode).
    pub dry_run: bool,
}

/// Expands a leading `~` in `path` to the value of the `HOME` environment variable.
///
/// When `path` does not start with `~`, the path is returned unchanged.
///
/// # Errors
///
/// Returns an error when `path` starts with `~` but the `HOME` environment
/// variable is not set.
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

/// Returns `true` when `backup_dir` is a subdirectory of (or equal to) `repo_root`.
fn is_inside_repo(backup_dir: &Path, repo_root: &Path) -> bool {
    backup_dir.strip_prefix(repo_root).is_ok()
}

/// Returns `true` for files that belong in a secret backup.
///
/// Matched patterns:
/// - `.env` / `.env.*` (any file whose basename starts with `.env`)
/// - `secrets.json` (exact basename)
/// - Any file under `.secrets/` (repo-relative path starts with `.secrets/`)
///
/// Patterns for future activation (IaC):
/// ```text
/// // activate when IaC is added
/// // rel.ends_with(".tfvars") || rel.ends_with(".tfvars.json")
/// ```
fn is_secret_file(rel: &str, base: &str) -> bool {
    base.starts_with(".env") || base == "secrets.json" || rel.starts_with(".secrets/")
}

/// Walks the repo root and returns all `.env*` files found.
///
/// Hidden directories (names starting with `.`) and directories listed in
/// [`Options::skip_dirs`] are skipped entirely.  Files whose size exceeds
/// [`Options::max_size`] or that are symlinks are included but marked
/// `skipped = true` with an explanatory `reason`.
///
/// # Errors
///
/// Does not propagate walk errors; individual entry errors are silently
/// skipped.  The function itself should not fail.
pub fn discover(opts: &Options) -> std::result::Result<Vec<FileEntry>, Error> {
    let max_size = if opts.max_size <= 0 {
        DEFAULT_MAX_SIZE
    } else {
        opts.max_size
    };
    let skip_set: std::collections::HashSet<&str> = opts
        .skip_dirs
        .iter()
        .map(std::string::String::as_str)
        .collect();

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
            // Hidden dirs are skipped, except `.secrets/` which is descended.
            if base.starts_with('.') {
                let is_secrets = path
                    .strip_prefix(&opts.repo_root)
                    .is_ok_and(|r| r == std::path::Path::new(".secrets"));
                if !is_secrets {
                    walker.skip_current_dir();
                }
                continue;
            }
            if skip_set.contains(base.as_str()) {
                walker.skip_current_dir();
                continue;
            }
            continue;
        }
        let rel = match path.strip_prefix(&opts.repo_root) {
            Ok(r) => r.to_string_lossy().into_owned(),
            Err(_) => continue,
        };
        if !is_secret_file(&rel, &base) {
            continue;
        }
        let Ok(meta) = fs::symlink_metadata(&path) else {
            continue;
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

/// Checks each [`ConfigPattern`] relative to `repo_root` and returns entries
/// for any that exist on disk.
///
/// Symlinks are included but marked `skipped = true`.  Files larger than
/// `max_size` (defaults to [`DEFAULT_MAX_SIZE`] when `<= 0`) are similarly
/// marked skipped.
///
/// # Errors
///
/// Returns an error when `lstat` fails for any reason other than
/// `NotFound`.
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

/// Copies `src` to `dst`, returning an error with context on failure.
///
/// # Errors
///
/// Returns an error when `fs::copy` fails (e.g. permission denied or disk full).
fn copy_file(src: &Path, dst: &Path) -> std::result::Result<(), Error> {
    fs::copy(src, dst).with_context(|| format!("copy {} -> {}", src.display(), dst.display()))?;
    Ok(())
}

/// Returns the relative paths of `entries` (excluding skipped ones) that
/// already exist under `dest_root`.
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

/// Copies `.env*` files (and optionally config files) from the repo to the backup directory.
///
/// Mutates `opts` to apply defaults (`max_size`, `skip_dirs`, tilde expansion of
/// `backup_dir`).  The backup directory must not be inside the repository.
///
/// # Errors
///
/// Returns an error when:
/// - `backup_dir` (after expansion) is inside `repo_root`.
/// - [`discover`] or [`discover_config`] fails.
/// - The backup directory cannot be created.
#[allow(clippy::collapsible_if, clippy::collapsible_match)]
pub fn backup(opts: &mut Options) -> std::result::Result<Result, Error> {
    if opts.max_size <= 0 {
        opts.max_size = DEFAULT_MAX_SIZE;
    }
    if opts.skip_dirs.is_empty() {
        opts.skip_dirs = default_skip_dirs()
            .iter()
            .map(std::string::ToString::to_string)
            .collect();
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
        for e in &mut entries {
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

    if !opts.dry_run {
        fs::create_dir_all(&dest_root).with_context(|| "create backup dir")?;
    }

    let mut result = Result {
        direction: "backup".to_string(),
        dir: opts.backup_dir.to_string_lossy().into_owned(),
        files: entries.clone(),
        worktree_name: opts.worktree_name.clone(),
        dry_run: opts.dry_run,
        ..Default::default()
    };

    if opts.dry_run {
        return Ok(result);
    }

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

/// Copies `.env*` files (and optionally config files) from the backup directory back
/// into the repository.
///
/// Mutates `opts` to apply defaults and expand the tilde in `backup_dir`.
///
/// # Errors
///
/// Returns an error when:
/// - `backup_dir` does not exist.
/// - [`discover`] or [`discover_config`] fails.
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
        for e in &mut entries {
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
        dry_run: opts.dry_run,
        ..Default::default()
    };

    for e in entries {
        let base = Path::new(&e.rel_path)
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default();
        if e.source != "config" && !is_secret_file(&e.rel_path, &base) {
            continue;
        }
        result.files.push(e.clone());
        if e.skipped || opts.dry_run {
            if e.skipped {
                result.skipped += 1;
            }
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

/// Information about whether the given path is a Git linked worktree.
pub struct WorktreeInfo {
    /// `true` when `.git` is a file (linked worktree) rather than a directory.
    pub is_worktree: bool,
    /// The name of the worktree directory (last component of `repo_root`).
    pub worktree_name: String,
}

/// Detects whether `repo_root` is a linked Git worktree by inspecting its `.git` entry.
///
/// A regular checkout has `.git` as a directory; a linked worktree has `.git`
/// as a file starting with `"gitdir:"`.
///
/// # Errors
///
/// Returns an error when `.git` does not exist at `repo_root` or when the
/// `.git` file cannot be read or parsed.
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

/// Formats the backup/restore result as human-readable text.
///
/// When `quiet` is `true`, per-file lines are suppressed and only the summary
/// line is printed.  When `verbose` is `true`, skipped files are also listed.
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
        let action = if r.dry_run {
            "WOULD".to_string()
        } else {
            r.direction.to_uppercase()
        };
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
            let _ = writeln!(sb, "  {action}  {}{tag}", f.rel_path);
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
    if r.dry_run {
        let _ = write!(
            sb,
            "Dry-run {}: {} file(s) would be {}d, {} skipped",
            label,
            r.files.iter().filter(|f| !f.skipped).count(),
            label,
            r.skipped
        );
    } else {
        let _ = write!(
            sb,
            "{} complete: {} file(s) {}d, {} skipped",
            capitalize(&label),
            r.copied,
            label,
            r.skipped
        );
    }
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

/// Top-level JSON document for a backup or restore result.
#[derive(Serialize)]
struct JsonOut<'a> {
    /// `"backup"` or `"restore"`.
    direction: &'a str,
    /// Absolute path to the backup directory.
    dir: &'a str,
    /// Per-file entries.
    files: Vec<JsonEntry<'a>>,
    /// Number of files successfully copied.
    copied: usize,
    /// Number of files skipped.
    skipped: usize,
    /// Non-fatal errors (omitted when empty).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    errors: &'a Vec<String>,
    /// Worktree name (omitted when empty).
    #[serde(skip_serializing_if = "str::is_empty", rename = "worktreeName")]
    worktree_name: &'a str,
    /// Whether the operation was cancelled before copying.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    cancelled: bool,
}

/// JSON representation of a single file entry.
#[derive(Serialize)]
struct JsonEntry<'a> {
    /// Repository-relative path.
    #[serde(rename = "relPath")]
    rel_path: &'a str,
    /// File size in bytes (omitted when zero).
    #[serde(skip_serializing_if = "is_zero_i64")]
    size: i64,
    /// Whether the file was skipped.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    skipped: bool,
    /// Reason for skipping (omitted when empty).
    #[serde(skip_serializing_if = "str::is_empty")]
    reason: &'a str,
    /// Source type: `"env"` or `"config"` (omitted when empty).
    #[serde(skip_serializing_if = "str::is_empty")]
    source: &'a str,
}

/// Serialises the backup/restore result to a pretty-printed JSON string.
///
/// # Errors
///
/// Returns an error when `serde_json` serialisation fails.
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

/// Formats the backup/restore result as a Markdown report.
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

/// Capitalises the first character of `s`, leaving the rest unchanged.
fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
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
            skip_dirs: default_skip_dirs()
                .iter()
                .map(std::string::ToString::to_string)
                .collect(),
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
            skip_dirs: default_skip_dirs()
                .iter()
                .map(std::string::ToString::to_string)
                .collect(),
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
            skip_dirs: default_skip_dirs()
                .iter()
                .map(std::string::ToString::to_string)
                .collect(),
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
            skip_dirs: default_skip_dirs()
                .iter()
                .map(std::string::ToString::to_string)
                .collect(),
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

    /// Builds a sample [`Result`] with a copied `.env`, a skipped large file,
    /// and a copied config file.
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

    // Phase 2 RED tests — (a)/(c)/(d) fail until GREEN lands.

    #[test]
    fn discover_finds_secrets_dir_file() {
        let dir = tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join(".secrets")).unwrap();
        std::fs::write(dir.path().join(".secrets/notes.md"), "secret").unwrap();
        let opts = Options {
            repo_root: dir.path().to_path_buf(),
            skip_dirs: default_skip_dirs()
                .iter()
                .map(std::string::ToString::to_string)
                .collect(),
            max_size: DEFAULT_MAX_SIZE,
            ..Default::default()
        };
        let e = discover(&opts).unwrap();
        assert!(
            e.iter().any(|f| f.rel_path == ".secrets/notes.md"),
            "expected .secrets/notes.md in discover result, got: {e:?}"
        );
    }

    #[test]
    fn discover_still_skips_git_dir() {
        let dir = tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join(".git")).unwrap();
        std::fs::write(dir.path().join(".git/config"), "gitconfig").unwrap();
        std::fs::write(dir.path().join(".env"), "k=v").unwrap();
        let opts = Options {
            repo_root: dir.path().to_path_buf(),
            skip_dirs: default_skip_dirs()
                .iter()
                .map(std::string::ToString::to_string)
                .collect(),
            max_size: DEFAULT_MAX_SIZE,
            ..Default::default()
        };
        let e = discover(&opts).unwrap();
        assert!(
            e.iter().all(|f| !f.rel_path.starts_with(".git/")),
            "expected .git/ to be skipped, got: {e:?}"
        );
    }

    #[test]
    fn discover_finds_secrets_json() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("secrets.json"), r#"{"key":"val"}"#).unwrap();
        let opts = Options {
            repo_root: dir.path().to_path_buf(),
            skip_dirs: default_skip_dirs()
                .iter()
                .map(std::string::ToString::to_string)
                .collect(),
            max_size: DEFAULT_MAX_SIZE,
            ..Default::default()
        };
        let e = discover(&opts).unwrap();
        assert!(
            e.iter().any(|f| f.rel_path == "secrets.json"),
            "expected secrets.json in discover result, got: {e:?}"
        );
    }

    #[test]
    fn backup_dry_run_writes_nothing() {
        let repo = tempdir().unwrap();
        let dest = tempdir().unwrap();
        std::fs::write(repo.path().join(".env"), "k=v").unwrap();
        let mut opts = Options {
            repo_root: repo.path().to_path_buf(),
            backup_dir: dest.path().to_path_buf(),
            skip_dirs: default_skip_dirs()
                .iter()
                .map(std::string::ToString::to_string)
                .collect(),
            max_size: DEFAULT_MAX_SIZE,
            force: true,
            dry_run: true,
            ..Default::default()
        };
        let r = backup(&mut opts).unwrap();
        assert_eq!(r.copied, 0, "dry_run backup must copy no files");
        assert!(
            !dest.path().join(".env").exists(),
            "dry_run backup must write no files to disk"
        );
    }

    // Phase 2 RED2: canonical backup default dir.
    // Fails until DEFAULT_BACKUP_DIR is changed to "ose-public-env-backup".
    #[test]
    fn default_backup_dir_is_ose_public_env_backup() {
        assert_eq!(
            DEFAULT_BACKUP_DIR, "ose-public-env-backup",
            "expected ose-public-env-backup but got {DEFAULT_BACKUP_DIR}"
        );
    }
}
