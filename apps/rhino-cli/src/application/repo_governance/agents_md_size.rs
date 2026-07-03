//! Size auditing for the `AGENTS.md` root instruction file.
//!
//! Byte-for-byte port of `apps/rhino-cli/internal/repo-governance/agents_md_size.go`.

use anyhow::{Context, Error};

use crate::application::fs::port::Fs;

/// Soft target size for AGENTS.md (30 KB).
pub const AGENTS_MD_TARGET_SIZE: i64 = 30_000;
/// Second-tier warning threshold (35 KB).
pub const AGENTS_MD_WARNING_SIZE: i64 = 35_000;
/// Hard upper bound that fails the audit (40 KB).
pub const AGENTS_MD_HARD_LIMIT_SIZE: i64 = 40_000;

/// The result of a single `AGENTS.md` size check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentsMdSizeFinding {
    /// Absolute or relative path to the file that was inspected.
    pub file: String,
    /// Measured file size in bytes.
    pub size: i64,
    /// Outcome: `"ok"`, `"warn"`, or `"fail"`.
    pub severity: String,
    /// Human-readable description of the finding.
    pub message: String,
}

/// Checks the byte size of the `AGENTS.md` file at `path` and classifies it
/// against the three-tier thresholds.
///
/// # Errors
///
/// Returns an error when the file cannot be stat-ed (e.g., it does not exist or
/// the process lacks permission).
pub fn check_agents_md_size(
    fs: &dyn Fs,
    path: &str,
) -> std::result::Result<AgentsMdSizeFinding, Error> {
    let size = fs
        .file_size(std::path::Path::new(path))
        .context("stat AGENTS.md")? as i64;
    let (severity, message) = classify(size);
    Ok(AgentsMdSizeFinding {
        file: path.to_string(),
        size,
        severity: severity.to_string(),
        message,
    })
}

/// Maps a byte `size` to a `(severity, message)` pair using the three-tier
/// thresholds defined by the module constants.
fn classify(size: i64) -> (&'static str, String) {
    if size <= AGENTS_MD_TARGET_SIZE {
        (
            "ok",
            format!("AGENTS.md is {size} bytes (within {AGENTS_MD_TARGET_SIZE}-byte target)"),
        )
    } else if size <= AGENTS_MD_WARNING_SIZE {
        (
            "warn",
            format!("AGENTS.md is {size} bytes (over {AGENTS_MD_TARGET_SIZE}-byte target)"),
        )
    } else if size <= AGENTS_MD_HARD_LIMIT_SIZE {
        (
            "warn",
            format!(
                "AGENTS.md is {size} bytes (over {AGENTS_MD_WARNING_SIZE}-byte warning threshold)"
            ),
        )
    } else {
        (
            "fail",
            format!("AGENTS.md is {size} bytes (over {AGENTS_MD_HARD_LIMIT_SIZE}-byte hard limit)"),
        )
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::infrastructure::fs::real::RealFs;
    use std::fs;
    use tempfile::TempDir;

    fn write_size(dir: &std::path::Path, bytes: usize) -> std::path::PathBuf {
        let p = dir.join("AGENTS.md");
        fs::write(&p, "x".repeat(bytes)).unwrap();
        p
    }

    #[test]
    fn ok_under_target() {
        let tmp = TempDir::new().unwrap();
        let p = write_size(tmp.path(), 25_000);
        let f = check_agents_md_size(&RealFs, p.to_str().unwrap()).unwrap();
        assert_eq!(f.severity, "ok");
        assert!(f.message.contains("within 30000-byte target"));
    }

    #[test]
    fn warn_over_target_under_warning() {
        let tmp = TempDir::new().unwrap();
        let p = write_size(tmp.path(), 32_000);
        let f = check_agents_md_size(&RealFs, p.to_str().unwrap()).unwrap();
        assert_eq!(f.severity, "warn");
        assert!(f.message.contains("over 30000-byte target"));
    }

    #[test]
    fn warn_over_warning_under_hard_limit() {
        let tmp = TempDir::new().unwrap();
        let p = write_size(tmp.path(), 38_000);
        let f = check_agents_md_size(&RealFs, p.to_str().unwrap()).unwrap();
        assert_eq!(f.severity, "warn");
        assert!(f.message.contains("over 35000-byte warning threshold"));
    }

    #[test]
    fn fail_over_hard_limit() {
        let tmp = TempDir::new().unwrap();
        let p = write_size(tmp.path(), 45_000);
        let f = check_agents_md_size(&RealFs, p.to_str().unwrap()).unwrap();
        assert_eq!(f.severity, "fail");
        assert!(f.message.contains("over 40000-byte hard limit"));
    }

    #[test]
    fn missing_file_errors() {
        let err = check_agents_md_size(&RealFs, "/nonexistent").unwrap_err();
        assert!(err.to_string().contains("stat AGENTS.md"));
    }

    #[test]
    fn mocked_fs_drives_size_check_without_touching_real_disk() {
        use crate::application::fs::mock::MockFs;

        let fs = MockFs::new().with_file("/repo/AGENTS.md", "x".repeat(32_000));
        let finding = check_agents_md_size(&fs, "/repo/AGENTS.md").unwrap();
        assert_eq!(finding.size, 32_000);
        assert_eq!(finding.severity, "warn");
        assert!(finding.message.contains("over 30000-byte target"));
    }
}
