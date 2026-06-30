//! Git repository root locator — IO adapter for `git rev-parse --show-toplevel`.

use std::path::PathBuf;
use std::process::Command;

use anyhow::{Context, Error, anyhow};

/// Returns the absolute path to the Git repository root from an optional working directory.
///
/// Executes `git rev-parse --show-toplevel` from `cwd` when provided, or from the
/// current working directory when `None`. Using `--show-toplevel` makes this
/// worktree-aware: from a linked worktree, it returns the worktree path, not the
/// main repo path.
///
/// # Errors
///
/// Returns an error when `git` is not found, the command fails, or the output
/// is empty or not valid UTF-8.
pub fn find_root_from(cwd: Option<&std::path::Path>) -> std::result::Result<PathBuf, Error> {
    let mut cmd = Command::new("git");
    cmd.args(["rev-parse", "--show-toplevel"]);
    if let Some(dir) = cwd {
        cmd.current_dir(dir);
    }
    let output = cmd.output().context("failed to invoke git rev-parse")?;
    if !output.status.success() {
        return Err(anyhow!(
            "git rev-parse failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    let path = String::from_utf8(output.stdout)?.trim().to_string();
    if path.is_empty() {
        return Err(anyhow!("git rev-parse returned empty path"));
    }
    Ok(PathBuf::from(path))
}

/// Returns the absolute path to the Git repository root.
///
/// Executes `git rev-parse --show-toplevel` from the current working directory.
///
/// # Errors
///
/// Returns an error when `git` is not found, the command fails, or the output
/// is empty or not valid UTF-8.
pub fn find_root() -> std::result::Result<PathBuf, Error> {
    find_root_from(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::CwdLock;
    use std::process::Command as Cmd;
    use tempfile::TempDir;

    #[test]
    fn find_root_returns_repo_root() {
        let _cwd = CwdLock::acquire();
        let root = find_root().expect("git root resolvable in test");
        assert!(
            root.join("Cargo.toml").exists() || root.join("AGENTS.md").exists(),
            "expected repo root to contain Cargo.toml or AGENTS.md, got {root:?}"
        );
    }

    /// Regression lock: `find_root_from` uses `--show-toplevel` which is worktree-aware.
    /// A linked worktree's root resolves to the worktree path, not the main repo.
    ///
    /// Gherkin:
    ///   Given a synthetic linked worktree in the rhino-cli test suite
    ///   When a guardrail command runs inside it
    ///   Then it succeeds, proving repo-root resolution is worktree-aware
    #[test]
    fn find_root_from_worktree_returns_worktree_path() {
        let main_repo = TempDir::new().expect("tempdir");
        let main = main_repo.path();

        // Init a bare-minimum git repo.
        Cmd::new("git")
            .args(["init"])
            .current_dir(main)
            .output()
            .expect("git init");
        Cmd::new("git")
            .args(["config", "user.email", "test@test.com"])
            .current_dir(main)
            .output()
            .expect("git config email");
        Cmd::new("git")
            .args(["config", "user.name", "Test"])
            .current_dir(main)
            .output()
            .expect("git config name");
        std::fs::write(main.join("README.md"), "test").expect("write README");
        Cmd::new("git")
            .args(["add", "."])
            .current_dir(main)
            .output()
            .expect("git add");
        Cmd::new("git")
            .args(["commit", "-m", "init"])
            .current_dir(main)
            .output()
            .expect("git commit");

        // Create a linked worktree.
        let wt_dir = TempDir::new().expect("tempdir wt");
        let wt_path = wt_dir.path();
        let status = Cmd::new("git")
            .args(["worktree", "add", &wt_path.to_string_lossy(), "HEAD"])
            .current_dir(main)
            .status()
            .expect("git worktree add");
        assert!(status.success(), "git worktree add must succeed");

        // find_root_from the worktree path must return the WORKTREE path, not main.
        let resolved = find_root_from(Some(wt_path))
            .expect("find_root_from must succeed inside a linked worktree");
        let resolved_canonical = std::fs::canonicalize(&resolved).expect("canonicalize resolved");
        let wt_canonical = std::fs::canonicalize(wt_path).expect("canonicalize wt_path");
        assert_eq!(
            resolved_canonical, wt_canonical,
            "find_root_from must return the linked worktree path when invoked from it"
        );
    }
}
