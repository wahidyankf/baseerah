// Sync orchestration ported from `apps/rhino-cli/internal/agents/sync.go`.
//
// SyncAll iterates `.claude/agents/` and writes the converted OpenCode
// equivalents into `.opencode/agents/`. Skills are NOT copied (OpenCode
// reads `.claude/skills/<name>/SKILL.md` natively); the SkillsOnly flag is
// retained for CLI backwards compatibility but is a no-op.

use std::path::PathBuf;
use std::time::{Duration, Instant};

use super::converter::{convert_all_agents, ConversionWarning};

#[derive(Debug, Clone, Default)]
pub struct SyncOptions {
    pub repo_root: PathBuf,
    pub dry_run: bool,
    pub agents_only: bool,
    pub skills_only: bool,
    pub verbose: bool,
    pub quiet: bool,
}

#[derive(Debug, Clone, Default)]
pub struct SyncResult {
    pub agents_converted: usize,
    pub agents_failed: usize,
    pub skills_copied: usize,
    pub skills_failed: usize,
    pub failed_files: Vec<String>,
    pub warnings: Vec<ConversionWarning>,
    pub duration: Duration,
}

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
