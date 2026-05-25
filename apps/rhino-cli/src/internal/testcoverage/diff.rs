//! Diff-based coverage: measures line coverage restricted to lines changed in a git diff.
//!
//! Port of `apps/rhino-cli/internal/testcoverage/diff.go` and `gitdiff.go`.

use std::process::Command;

use anyhow::{Error, anyhow};
use regex::Regex;

use super::exclude::matches_any_exclude_pattern;
use super::merge::{has_missed_branch, to_coverage_map};
use super::types::{FileResult, Format, Result as CoverageResult};

/// Options controlling diff-based coverage computation.
pub struct DiffCoverageOptions {
    /// Path to the coverage report file (any supported format).
    pub coverage_file: String,
    /// Base git ref to diff against (e.g. `"main"`). Ignored when `staged` is `true`.
    pub base: String,
    /// When `true`, diffs the index (staged changes) instead of `<base>...HEAD`.
    pub staged: bool,
    /// Minimum coverage percentage required for the result to pass.
    pub threshold: f64,
    /// When `true`, include per-file breakdown in the result.
    pub per_file: bool,
    /// Glob patterns for files to exclude from coverage computation.
    pub exclude_patterns: Vec<String>,
}

/// A single diff hunk: the set of added or modified line numbers for one file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffHunk {
    /// Relative path to the file as reported by `git diff`.
    pub file_path: String,
    /// 1-based line numbers of added/modified lines in the new version of the file.
    pub changed_lines: Vec<i64>,
}

/// Parse unified diff output. Returns one `DiffHunk` per touched file.
///
/// # Panics
///
/// Panics if the hardcoded regular expressions fail to compile (this cannot
/// happen in practice as they are compile-time constants).
pub fn parse_git_diff(diff_output: &str) -> Vec<DiffHunk> {
    // Use Vec to preserve insertion order matching Go's iteration over fileLines map —
    // Go's map iteration is non-deterministic but we use insertion-order here for stability.
    let mut file_order: Vec<String> = Vec::new();
    let mut file_lines: std::collections::HashMap<String, Vec<i64>> =
        std::collections::HashMap::new();

    let diff_file_re = Regex::new(r"^diff --git a/.+ b/(.+)$").expect("valid hardcoded regex");
    let hunk_re =
        Regex::new(r"^@@ -\d+(?:,\d+)? \+(\d+)(?:,(\d+))? @@").expect("valid hardcoded regex");
    let rename_re = Regex::new(r"^rename to (.+)$").expect("valid hardcoded regex");

    let mut current_file = String::new();
    let mut in_hunk = false;
    let mut current_line_no: i64 = 0;

    for line in diff_output.split('\n') {
        if let Some(m) = diff_file_re.captures(line) {
            current_file = m
                .get(1)
                .expect("capture group 1 always present")
                .as_str()
                .to_string();
            in_hunk = false;
            continue;
        }
        if let Some(m) = rename_re.captures(line) {
            current_file = m
                .get(1)
                .expect("capture group 1 always present")
                .as_str()
                .to_string();
            continue;
        }
        if line.starts_with("Binary files") {
            current_file.clear();
            continue;
        }
        if let Some(m) = hunk_re.captures(line) {
            current_line_no = m
                .get(1)
                .expect("capture group 1 always present")
                .as_str()
                .parse()
                .unwrap_or(0);
            in_hunk = true;
            continue;
        }
        if !in_hunk || current_file.is_empty() {
            continue;
        }

        if line.starts_with('+') && !line.starts_with("+++") {
            if !file_lines.contains_key(&current_file) {
                file_order.push(current_file.clone());
            }
            file_lines
                .entry(current_file.clone())
                .or_default()
                .push(current_line_no);
            current_line_no += 1;
        } else if line.starts_with('-') && !line.starts_with("---") {
            // deleted — don't bump new-file counter
        } else {
            current_line_no += 1;
        }
    }

    file_order
        .into_iter()
        .filter_map(|fp| {
            file_lines.remove(&fp).map(|lines| DiffHunk {
                file_path: fp,
                changed_lines: lines,
            })
        })
        .collect()
}

/// Computes coverage restricted to lines changed in the current git diff.
///
/// Calls `get_git_diff` to obtain the diff, parses it with `parse_git_diff`,
/// then intersects the changed lines with the coverage map from `opts.coverage_file`.
///
/// # Errors
///
/// Returns an error when `get_git_diff` fails (e.g. git not found or non-zero exit)
/// or when the coverage file cannot be parsed by `to_coverage_map`.
pub fn compute_diff_coverage(opts: &DiffCoverageOptions) -> Result<CoverageResult, Error> {
    let diff_output = get_git_diff(&opts.base, opts.staged)?;
    let hunks = parse_git_diff(&diff_output);
    if hunks.is_empty() {
        return Ok(CoverageResult {
            file: opts.coverage_file.clone(),
            format: Format::Diff,
            covered: 0,
            partial: 0,
            missed: 0,
            total: 0,
            pct: 100.0,
            threshold: opts.threshold,
            passed: true,
            files: Vec::new(),
        });
    }

    let mut cm = to_coverage_map(&opts.coverage_file)?;

    if !opts.exclude_patterns.is_empty() {
        let to_drop: Vec<String> = cm
            .keys()
            .filter(|p| matches_any_exclude_pattern(p, &opts.exclude_patterns))
            .cloned()
            .collect();
        for p in to_drop {
            cm.remove(&p);
        }
    }

    let mut covered = 0usize;
    let mut partial = 0usize;
    let mut missed = 0usize;
    let mut per_file: Vec<FileResult> = Vec::new();

    for hunk in &hunks {
        if !opts.exclude_patterns.is_empty()
            && matches_any_exclude_pattern(&hunk.file_path, &opts.exclude_patterns)
        {
            continue;
        }

        let mut fc = 0usize;
        let mut fp = 0usize;
        let mut fm = 0usize;
        let file_cov = cm.get(&hunk.file_path);

        for &line_no in &hunk.changed_lines {
            let Some(file_cov) = file_cov else {
                fm += 1;
                continue;
            };
            let Some(lc) = file_cov.get(&line_no) else {
                // Line not in coverage report — non-executable.
                continue;
            };
            if lc.hit_count > 0 {
                if has_missed_branch(&lc.branches) {
                    fp += 1;
                } else {
                    fc += 1;
                }
            } else {
                fm += 1;
            }
        }

        covered += fc;
        partial += fp;
        missed += fm;

        let ft = fc + fp + fm;
        if ft > 0 {
            let fpct = 100.0 * fc as f64 / ft as f64;
            per_file.push(FileResult {
                path: hunk.file_path.clone(),
                covered: fc,
                partial: fp,
                missed: fm,
                total: ft,
                pct: fpct,
            });
        }
    }

    let total = covered + partial + missed;
    let pct = if total > 0 {
        100.0 * covered as f64 / total as f64
    } else {
        100.0
    };

    Ok(CoverageResult {
        file: opts.coverage_file.clone(),
        format: Format::Diff,
        covered,
        partial,
        missed,
        total,
        pct,
        threshold: opts.threshold,
        passed: opts.threshold == 0.0 || pct >= opts.threshold,
        files: per_file,
    })
}

/// Runs `git diff` and returns its stdout as a `String`.
///
/// When `staged` is `true`, runs `git diff --staged --unified=0`.
/// Otherwise runs `git diff --unified=0 <base>...HEAD`, defaulting `base` to
/// `"main"` when it is empty.
///
/// # Errors
///
/// Returns an error when the `git` process cannot be spawned or exits with a
/// non-zero status code.
pub fn get_git_diff(base: &str, staged: bool) -> Result<String, Error> {
    let mut args: Vec<String> = Vec::new();
    if staged {
        args.push("diff".into());
        args.push("--staged".into());
        args.push("--unified=0".into());
    } else {
        let base = if base.is_empty() { "main" } else { base };
        args.push("diff".into());
        args.push("--unified=0".into());
        args.push(format!("{base}...HEAD"));
    }

    let out = Command::new("git")
        .args(&args)
        .output()
        .map_err(|e| anyhow!("git diff failed: {e}"))?;
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        return Err(anyhow!("git diff failed: {}", stderr.trim()));
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn parse_git_diff_basic() {
        let diff =
            "diff --git a/foo.go b/foo.go\n--- a/foo.go\n+++ b/foo.go\n@@ -1,0 +1,2 @@\n+a\n+b\n";
        let hunks = parse_git_diff(diff);
        assert_eq!(hunks.len(), 1);
        assert_eq!(hunks[0].file_path, "foo.go");
        assert_eq!(hunks[0].changed_lines, vec![1, 2]);
    }

    #[test]
    fn parse_git_diff_handles_rename() {
        let diff = "diff --git a/old.go b/new.go\nrename to new.go\n@@ -0,0 +1,1 @@\n+x\n";
        let hunks = parse_git_diff(diff);
        assert_eq!(hunks.len(), 1);
        assert_eq!(hunks[0].file_path, "new.go");
    }

    #[test]
    fn parse_git_diff_skips_binary() {
        let diff = "diff --git a/img.png b/img.png\nBinary files a/img.png and b/img.png differ\n";
        let hunks = parse_git_diff(diff);
        assert_eq!(hunks.len(), 0);
    }

    #[test]
    fn parse_git_diff_deleted_lines_do_not_increment() {
        // Hunk starting at line 5 in new file: -x removed, +y added at 5.
        let diff = "diff --git a/f.go b/f.go\n@@ -5,1 +5,1 @@\n-x\n+y\n";
        let hunks = parse_git_diff(diff);
        assert_eq!(hunks[0].changed_lines, vec![5]);
    }

    #[test]
    fn parse_git_diff_no_hunk_no_lines() {
        let hunks = parse_git_diff("");
        assert!(hunks.is_empty());
    }
}
