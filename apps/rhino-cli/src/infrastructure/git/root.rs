//! Git repository root locator — IO adapter for `git rev-parse --show-toplevel`.

use std::path::PathBuf;
use std::process::Command;

use anyhow::{Context, Error, anyhow};

/// Returns the absolute path to the Git repository root.
///
/// Executes `git rev-parse --show-toplevel` from the current working directory.
///
/// # Errors
///
/// Returns an error when `git` is not found, the command fails, or the output
/// is empty or not valid UTF-8.
pub fn find_root() -> std::result::Result<PathBuf, Error> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .context("failed to invoke git rev-parse")?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_root_returns_repo_root() {
        let root = find_root().expect("git root resolvable in test");
        assert!(
            root.join("Cargo.toml").exists() || root.join("AGENTS.md").exists(),
            "expected repo root to contain Cargo.toml or AGENTS.md, got {root:?}"
        );
    }
}
