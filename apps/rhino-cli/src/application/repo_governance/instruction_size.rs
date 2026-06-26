//! Instruction-file size budget gate.
//!
//! Loads per-surface byte budgets from `instruction-size-budget.yaml`, globs
//! for instruction files, classifies each against a three-tier threshold, and
//! optionally resolves the transitive `@`-import tree to check aggregate size.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Error};
use serde::Deserialize;

// ---------------------------------------------------------------------------
// Configuration types
// ---------------------------------------------------------------------------

/// Budget thresholds for a single glob surface.
#[derive(Debug, Clone, Deserialize)]
pub struct Surface {
    /// Glob pattern (relative to repo root) to match instruction files.
    pub glob: String,
    /// Ideal maximum size in bytes.
    pub target: u64,
    /// Warning threshold: files between target and warn are flagged `Warn`.
    pub warn: u64,
    /// Hard upper bound: files exceeding this fail the gate.
    pub fail: u64,
}

/// Budget thresholds for the fully-resolved transitive `@`-import tree.
#[derive(Debug, Clone, Deserialize)]
pub struct ResolvedTree {
    /// Root file to start import resolution from (relative to repo root).
    pub root: String,
    /// Ideal maximum resolved size in bytes.
    pub target: u64,
    /// Warning threshold in bytes.
    pub warn: u64,
    /// Hard upper bound in bytes.
    pub fail: u64,
}

/// Top-level configuration loaded from `instruction-size-budget.yaml`.
#[derive(Debug, Clone, Deserialize)]
pub struct BudgetConfig {
    /// Per-surface budget entries.
    pub surfaces: Vec<Surface>,
    /// Resolved-tree budget entry.
    pub resolved_tree: ResolvedTree,
}

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

/// Severity classification for a single size finding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Severity {
    /// File is within the target budget.
    Ok,
    /// File exceeds the target but stays within the fail limit.
    Warn,
    /// File exceeds the fail limit.
    Fail,
}

/// Human-readable label for a [`Severity`].
pub fn severity_label(sev: &Severity) -> &'static str {
    match sev {
        Severity::Ok => "ok",
        Severity::Warn => "warn",
        Severity::Fail => "fail",
    }
}

/// A single finding produced by [`check_instruction_sizes`] or
/// [`check_resolved_tree`].
#[derive(Debug, Clone)]
pub struct Finding {
    /// Repo-relative path of the file (or `"resolved-tree"` for the tree check).
    pub path: String,
    /// Measured size in bytes.
    pub size: u64,
    /// Target budget in bytes.
    pub target: u64,
    /// Warning threshold in bytes.
    pub warn: u64,
    /// Fail threshold in bytes.
    pub fail: u64,
    /// Severity classification.
    pub severity: Severity,
    /// Human-readable description.
    pub message: String,
}

// ---------------------------------------------------------------------------
// Progressive disclosure reference
// ---------------------------------------------------------------------------

/// Reference text appended to every `Fail` finding message.
///
/// Both "progressive disclosure" and the full governance path must appear so
/// that lint tests can verify them with simple substring checks.
const PROGRESSIVE_DISCLOSURE_REF: &str =
    "progressive disclosure — see repo-governance/principles/content/progressive-disclosure.md";

// ---------------------------------------------------------------------------
// Config loader
// ---------------------------------------------------------------------------

/// Load and parse `instruction-size-budget.yaml` from `path`.
///
/// # Errors
///
/// Returns an error when the file cannot be read or contains invalid YAML.
pub fn load_budget_config(path: &Path) -> Result<BudgetConfig, Error> {
    let data =
        fs::read_to_string(path).with_context(|| format!("cannot read {}", path.display()))?;
    serde_norway::from_str(&data).with_context(|| format!("failed to parse {}", path.display()))
}

// ---------------------------------------------------------------------------
// classify
// ---------------------------------------------------------------------------

/// Classify a byte `size` against three-tier budget thresholds.
///
/// The `warn` parameter is used for message generation (see
/// [`surface_message`]) but does not create a separate `Severity` level:
/// both "over target" and "over warn threshold" map to [`Severity::Warn`].
///
/// - [`Severity::Ok`] when `size <= target`
/// - [`Severity::Warn`] when `target < size <= fail`
/// - [`Severity::Fail`] when `size > fail`
pub fn classify(size: u64, target: u64, _warn: u64, fail: u64) -> Severity {
    if size <= target {
        Severity::Ok
    } else if size <= fail {
        Severity::Warn
    } else {
        Severity::Fail
    }
}

// ---------------------------------------------------------------------------
// Message builders
// ---------------------------------------------------------------------------

/// Build a human-readable message for a surface finding.
fn surface_message(
    path: &str,
    size: u64,
    target: u64,
    warn: u64,
    fail: u64,
    severity: &Severity,
) -> String {
    match severity {
        Severity::Ok => format!("{path} is {size} bytes (within {target}-byte target)"),
        Severity::Warn if size <= warn => {
            format!("{path} is {size} bytes (over {target}-byte target)")
        }
        Severity::Warn => {
            format!("{path} is {size} bytes (over {warn}-byte warn threshold)")
        }
        Severity::Fail => format!(
            "{path} is {size} bytes (over {fail}-byte fail limit); apply {PROGRESSIVE_DISCLOSURE_REF}",
        ),
    }
}

/// Build a human-readable message for the resolved-tree finding.
fn resolved_tree_message(size: u64, rt: &ResolvedTree, severity: &Severity) -> String {
    match severity {
        Severity::Ok => format!("resolved tree ({}) is {size} bytes (ok)", rt.root),
        Severity::Warn if size <= rt.warn => {
            format!(
                "resolved tree ({}) is {size} bytes (over {}-byte target)",
                rt.root, rt.target
            )
        }
        Severity::Warn => {
            format!(
                "resolved tree ({}) is {size} bytes (over {}-byte warn threshold)",
                rt.root, rt.warn
            )
        }
        Severity::Fail => format!(
            "resolved tree ({}) is {size} bytes (over {}-byte fail limit); apply {PROGRESSIVE_DISCLOSURE_REF}",
            rt.root, rt.fail
        ),
    }
}

// ---------------------------------------------------------------------------
// check_instruction_sizes
// ---------------------------------------------------------------------------

/// Check all instruction file surfaces against their budgets.
///
/// Returns one [`Finding`] per matched file that is not within budget (`Warn`
/// or `Fail`). Globs that match no files produce no findings.  `Ok`-severity
/// files are not included in the result.
pub fn check_instruction_sizes(repo_root: &Path, config: &BudgetConfig) -> Vec<Finding> {
    let mut findings: Vec<Finding> = Vec::new();
    for surface in &config.surfaces {
        let pattern = repo_root.join(&surface.glob);
        let pattern_str = pattern.to_string_lossy().to_string();
        let Ok(paths) = glob::glob(&pattern_str) else {
            continue;
        };
        for entry in paths.flatten() {
            let size = fs::metadata(&entry).map_or(0, |m| m.len());
            let severity = classify(size, surface.target, surface.warn, surface.fail);
            if severity == Severity::Ok {
                continue;
            }
            let rel_path = entry.strip_prefix(repo_root).map_or_else(
                |_| entry.to_string_lossy().to_string(),
                |p| p.to_string_lossy().to_string(),
            );
            let message = surface_message(
                &rel_path,
                size,
                surface.target,
                surface.warn,
                surface.fail,
                &severity,
            );
            findings.push(Finding {
                path: rel_path,
                size,
                target: surface.target,
                warn: surface.warn,
                fail: surface.fail,
                severity,
                message,
            });
        }
    }
    findings
}

// ---------------------------------------------------------------------------
// resolve_tree_size + check_resolved_tree
// ---------------------------------------------------------------------------

/// Compute the total byte size of `root` and all transitively imported files.
///
/// Files declare imports via lines starting with `@`; the remainder of the
/// line (after trimming whitespace) is the relative import path.  The
/// recursion depth is capped at 4.  Cycles are detected via a set of
/// canonicalized absolute paths; a cycle returns 0 bytes for the repeated
/// node.
pub fn resolve_tree_size(root: &Path) -> u64 {
    let mut visited: HashSet<PathBuf> = HashSet::new();
    resolve_recursive(root, 0, &mut visited)
}

/// Recursive helper for [`resolve_tree_size`].
///
/// Returns 0 when the `depth` limit is exceeded or the `path` has already
/// been visited (cycle guard).
fn resolve_recursive(path: &Path, depth: usize, visited: &mut HashSet<PathBuf>) -> u64 {
    if depth > 4 {
        return 0;
    }
    let canonical = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    if !visited.insert(canonical) {
        return 0; // cycle guard
    }
    let size = fs::metadata(path).map_or(0, |m| m.len());
    let content = fs::read_to_string(path).unwrap_or_default();
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let imported: u64 = content
        .lines()
        .filter(|line| line.starts_with('@'))
        .map(|line| {
            let import_path = line[1..].trim();
            resolve_recursive(&parent.join(import_path), depth + 1, visited)
        })
        .sum();
    size + imported
}

/// Check the resolved import tree of `config.resolved_tree.root` against its
/// budget.
///
/// Returns `None` when the resolved size is within the target.  Returns
/// `Some(Finding)` when the resolved size exceeds the target or fail
/// threshold.
pub fn check_resolved_tree(repo_root: &Path, config: &BudgetConfig) -> Option<Finding> {
    let root_path = repo_root.join(&config.resolved_tree.root);
    let size = resolve_tree_size(&root_path);
    let rt = &config.resolved_tree;
    let severity = classify(size, rt.target, rt.warn, rt.fail);
    if severity == Severity::Ok {
        return None;
    }
    let message = resolved_tree_message(size, rt, &severity);
    Some(Finding {
        path: "resolved-tree".to_string(),
        size,
        target: rt.target,
        warn: rt.warn,
        fail: rt.fail,
        severity,
        message,
    })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    // ---- classify ----

    #[test]
    fn classify_ok_at_target() {
        assert_eq!(classify(24_000, 24_000, 27_000, 30_000), Severity::Ok);
    }

    #[test]
    fn classify_warn_over_target_under_fail() {
        assert_eq!(classify(28_000, 24_000, 27_000, 30_000), Severity::Warn);
    }

    #[test]
    fn classify_fail_over_fail() {
        assert_eq!(classify(31_000, 24_000, 27_000, 30_000), Severity::Fail);
    }

    #[test]
    fn classify_ok_below_target() {
        assert_eq!(classify(1_000, 24_000, 27_000, 30_000), Severity::Ok);
    }

    #[test]
    fn classify_warn_at_warn_boundary() {
        // Exactly at warn is still Warn (not Fail — fail is 30_000)
        assert_eq!(classify(27_000, 24_000, 27_000, 30_000), Severity::Warn);
    }

    #[test]
    fn classify_fail_at_fail_boundary_plus_one() {
        assert_eq!(classify(30_001, 24_000, 27_000, 30_000), Severity::Fail);
    }

    // ---- severity_label ----

    #[test]
    fn severity_label_ok() {
        assert_eq!(severity_label(&Severity::Ok), "ok");
    }

    #[test]
    fn severity_label_warn() {
        assert_eq!(severity_label(&Severity::Warn), "warn");
    }

    #[test]
    fn severity_label_fail() {
        assert_eq!(severity_label(&Severity::Fail), "fail");
    }

    // ---- load_budget_config ----

    #[test]
    fn load_budget_config_parses_agents_md_surface() {
        let tmp = TempDir::new().unwrap();
        let yaml = r#"
surfaces:
  - glob: "AGENTS.md"
    target: 24000
    warn: 27000
    fail: 30000
resolved_tree:
  root: "CLAUDE.md"
  target: 30000
  warn: 34000
  fail: 38000
"#;
        let path = tmp.path().join("instruction-size-budget.yaml");
        fs::write(&path, yaml).unwrap();
        let config = load_budget_config(&path).unwrap();
        assert_eq!(config.surfaces.len(), 1);
        assert_eq!(config.surfaces[0].glob, "AGENTS.md");
        assert_eq!(config.surfaces[0].fail, 30_000);
    }

    #[test]
    fn load_budget_config_error_on_missing_file() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("nonexistent.yaml");
        let result = load_budget_config(&path);
        assert!(result.is_err());
    }

    // ---- check_instruction_sizes ----

    fn simple_config(glob: &str, target: u64, warn: u64, fail: u64) -> BudgetConfig {
        BudgetConfig {
            surfaces: vec![Surface {
                glob: glob.to_string(),
                target,
                warn,
                fail,
            }],
            resolved_tree: ResolvedTree {
                root: "CLAUDE.md".to_string(),
                target: 30_000,
                warn: 34_000,
                fail: 38_000,
            },
        }
    }

    #[test]
    fn check_finds_fail_for_large_agents_md() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("AGENTS.md"), "x".repeat(31_000)).unwrap();
        let config = simple_config("AGENTS.md", 24_000, 27_000, 30_000);
        let findings = check_instruction_sizes(tmp.path(), &config);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].severity, Severity::Fail);
        assert!(findings[0].path == "AGENTS.md");
    }

    #[test]
    fn check_no_finding_for_absent_glob() {
        let tmp = TempDir::new().unwrap();
        let config = simple_config(".github/copilot-instructions.md", 6_000, 8_000, 10_000);
        let findings = check_instruction_sizes(tmp.path(), &config);
        assert!(findings.is_empty());
    }

    #[test]
    fn check_no_finding_when_within_target() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("AGENTS.md"), "x".repeat(10_000)).unwrap();
        let config = simple_config("AGENTS.md", 24_000, 27_000, 30_000);
        let findings = check_instruction_sizes(tmp.path(), &config);
        assert!(findings.is_empty(), "ok-severity files produce no finding");
    }

    #[test]
    fn check_finds_warn_for_medium_size() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("AGENTS.md"), "x".repeat(25_000)).unwrap();
        let config = simple_config("AGENTS.md", 24_000, 27_000, 30_000);
        let findings = check_instruction_sizes(tmp.path(), &config);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].severity, Severity::Warn);
    }

    #[test]
    fn fail_message_contains_progressive_disclosure() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("AGENTS.md"), "x".repeat(31_000)).unwrap();
        let config = simple_config("AGENTS.md", 24_000, 27_000, 30_000);
        let findings = check_instruction_sizes(tmp.path(), &config);
        assert_eq!(findings.len(), 1);
        let msg = &findings[0].message;
        assert!(
            msg.contains("progressive disclosure"),
            "message must contain 'progressive disclosure': {msg}"
        );
        assert!(
            msg.contains("repo-governance/principles/content/progressive-disclosure.md"),
            "message must contain governance path: {msg}"
        );
    }

    // ---- resolve_tree_size ----

    #[test]
    fn resolve_tree_sums_file_and_imports() {
        let tmp = TempDir::new().unwrap();
        let agents_bytes = 20_000usize;
        fs::write(tmp.path().join("AGENTS.md"), "x".repeat(agents_bytes)).unwrap();
        let claude_content = "@AGENTS.md\n";
        let claude_bytes = claude_content.len() as u64;
        fs::write(tmp.path().join("CLAUDE.md"), claude_content).unwrap();
        let total = resolve_tree_size(&tmp.path().join("CLAUDE.md"));
        assert_eq!(total, claude_bytes + agents_bytes as u64);
    }

    #[test]
    fn resolve_tree_missing_import_counts_zero() {
        let tmp = TempDir::new().unwrap();
        let content = "@NONEXISTENT.md\nsome text";
        let content_bytes = content.len() as u64;
        fs::write(tmp.path().join("CLAUDE.md"), content).unwrap();
        let total = resolve_tree_size(&tmp.path().join("CLAUDE.md"));
        // Only CLAUDE.md bytes — nonexistent import contributes 0
        assert_eq!(total, content_bytes);
    }

    #[test]
    fn resolve_tree_handles_cycle() {
        let tmp = TempDir::new().unwrap();
        // A.md imports B.md, B.md imports A.md
        let a_content = "@B.md\naaa";
        let b_content = "@A.md\nbbb";
        fs::write(tmp.path().join("A.md"), a_content).unwrap();
        fs::write(tmp.path().join("B.md"), b_content).unwrap();
        // Should not infinite-loop; result should be finite
        let total = resolve_tree_size(&tmp.path().join("A.md"));
        assert!(total > 0, "should count at least A.md and B.md bytes");
        // A + B counted once each (cycle guard stops re-entry)
        assert_eq!(total, a_content.len() as u64 + b_content.len() as u64);
    }

    // ---- check_resolved_tree ----

    #[test]
    fn check_resolved_tree_returns_fail_when_large() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("AGENTS.md"), "x".repeat(30_000)).unwrap();
        let claude_content = format!("@AGENTS.md\n{}", "y".repeat(9_000));
        fs::write(tmp.path().join("CLAUDE.md"), &claude_content).unwrap();
        let config = BudgetConfig {
            surfaces: vec![],
            resolved_tree: ResolvedTree {
                root: "CLAUDE.md".to_string(),
                target: 30_000,
                warn: 34_000,
                fail: 38_000,
            },
        };
        let finding = check_resolved_tree(tmp.path(), &config);
        assert!(finding.is_some());
        let f = finding.unwrap();
        assert_eq!(f.path, "resolved-tree");
        assert_eq!(f.severity, Severity::Fail);
        assert!(f.message.contains("progressive disclosure"));
        assert!(
            f.message
                .contains("repo-governance/principles/content/progressive-disclosure.md")
        );
    }

    #[test]
    fn check_resolved_tree_returns_none_when_within_budget() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("CLAUDE.md"), "small content").unwrap();
        let config = BudgetConfig {
            surfaces: vec![],
            resolved_tree: ResolvedTree {
                root: "CLAUDE.md".to_string(),
                target: 30_000,
                warn: 34_000,
                fail: 38_000,
            },
        };
        let finding = check_resolved_tree(tmp.path(), &config);
        assert!(finding.is_none());
    }
}
