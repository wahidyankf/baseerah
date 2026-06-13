//! Sync orchestration ported from `apps/rhino-cli/internal/agents/sync.go`.
//
// SyncAll iterates `.claude/agents/` and writes the converted OpenCode
// equivalents into `.opencode/agents/`. Skills are NOT copied (OpenCode
// reads `.claude/skills/<name>/SKILL.md` natively); the SkillsOnly flag is
// retained for CLI backwards compatibility but is a no-op.

use std::path::PathBuf;
use std::time::{Duration, Instant};

use super::converter::{ConversionWarning, convert_all_agents};

/// Options controlling a `sync_all` run.
#[derive(Debug, Clone, Default)]
pub struct SyncOptions {
    /// Absolute path to the repository root.
    pub repo_root: PathBuf,
    /// When true, perform a dry run: compute conversions but do not write files.
    pub dry_run: bool,
    /// When true, sync only agents (skills are never synced either way).
    pub agents_only: bool,
    /// When true, skip agent sync (no-op since skills are not copied).
    pub skills_only: bool,
    /// When true, emit verbose output in the reporter.
    pub verbose: bool,
    /// When true, suppress progress headers in the reporter.
    pub quiet: bool,
}

/// Aggregated result of a `sync_all` run.
#[derive(Debug, Clone, Default)]
pub struct SyncResult {
    /// Number of agents successfully converted.
    pub agents_converted: usize,
    /// Number of agents that failed to convert.
    pub agents_failed: usize,
    /// Number of skills copied (always 0 — skills are not copied).
    pub skills_copied: usize,
    /// Number of skills that failed to copy (always 0).
    pub skills_failed: usize,
    /// Filenames of files that failed during sync.
    pub failed_files: Vec<String>,
    /// Collected conversion warnings.
    pub warnings: Vec<ConversionWarning>,
    /// Wall time for the sync operation.
    pub duration: Duration,
}

/// Run the agent sync: convert `.claude/agents/` → `.opencode/agents/`.
///
/// # Errors
///
/// Returns an error if `.claude/agents/` cannot be read.
pub fn sync_all(opts: &SyncOptions) -> Result<SyncResult, String> {
    let start = Instant::now();
    let mut result = SyncResult::default();

    if !opts.skills_only {
        let r = convert_all_agents(&opts.repo_root, opts.dry_run)?;
        result.agents_converted = r.converted;
        result.agents_failed = r.failed;
        result.failed_files.extend(r.failed_files);
        result.warnings.extend(r.warnings);
    }

    result.duration = start.elapsed();
    Ok(result)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn write(path: &std::path::Path, content: &str) {
        if let Some(p) = path.parent() {
            std::fs::create_dir_all(p).unwrap();
        }
        std::fs::write(path, content).unwrap();
    }

    #[test]
    fn sync_all_converts() {
        let dir = tempdir().unwrap();
        write(
            &dir.path().join(".claude/agents/a.md"),
            "---\nname: a\ndescription: a\ntools: Read\nmodel: sonnet\n---\nBody\n",
        );
        let opts = SyncOptions {
            repo_root: dir.path().to_path_buf(),
            ..Default::default()
        };
        let r = sync_all(&opts).unwrap();
        assert_eq!(r.agents_converted, 1);
        assert_eq!(r.agents_failed, 0);
        assert!(dir.path().join(".opencode/agents/a.md").exists());
    }

    #[test]
    fn sync_all_dry_run_no_writes() {
        let dir = tempdir().unwrap();
        write(
            &dir.path().join(".claude/agents/a.md"),
            "---\nname: a\ndescription: a\ntools: Read\nmodel: sonnet\n---\nBody\n",
        );
        let opts = SyncOptions {
            repo_root: dir.path().to_path_buf(),
            dry_run: true,
            ..Default::default()
        };
        let r = sync_all(&opts).unwrap();
        assert_eq!(r.agents_converted, 1);
        assert!(!dir.path().join(".opencode/agents/a.md").exists());
    }

    #[test]
    fn sync_all_skills_only_noop() {
        let dir = tempdir().unwrap();
        write(
            &dir.path().join(".claude/agents/a.md"),
            "---\nname: a\ndescription: a\ntools: Read\nmodel: sonnet\n---\nBody\n",
        );
        let opts = SyncOptions {
            repo_root: dir.path().to_path_buf(),
            skills_only: true,
            ..Default::default()
        };
        let r = sync_all(&opts).unwrap();
        assert_eq!(r.agents_converted, 0);
    }

    #[test]
    fn sync_all_collects_warnings() {
        let dir = tempdir().unwrap();
        write(
            &dir.path().join(".claude/agents/a.md"),
            "---\nname: a\ndescription: a\ntools: Read\nmodel: sonnet\nmcpServers:\n  one: two\n---\nBody\n",
        );
        let opts = SyncOptions {
            repo_root: dir.path().to_path_buf(),
            ..Default::default()
        };
        let r = sync_all(&opts).unwrap();
        assert!(r.warnings.iter().any(|w| w.field == "mcpServers"));
    }

    #[test]
    fn sync_all_missing_claude_dir_errors() {
        let dir = tempdir().unwrap();
        let opts = SyncOptions {
            repo_root: dir.path().to_path_buf(),
            ..Default::default()
        };
        let r = sync_all(&opts);
        assert!(r.is_err());
    }
}
