//! LCOV format parser and result computer.
//!
//! Byte-for-byte port of `apps/rhino-cli/internal/testcoverage/lcov_coverage.go`.
//!
//! Uses the same `SF:`/`DA:`/`BRDA:` state machine, the same
//! "duplicate `DA` → take max count" rule, and the same branch-coverage
//! classification:
//!
//! - all branch counts positive → **covered**
//! - any branch count positive → **partial**
//! - none positive → **missed**

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{Context, Error, anyhow};

use super::types::{FileResult, Format, Result as CoverageResult};

/// Parsed data for a single source file recorded in an LCOV info file.
#[derive(Debug, Default, Clone)]
pub(crate) struct LcovFile {
    /// Source-file path as recorded in the `SF:` line.
    pub path: String,
    /// Line execution counts keyed by 1-based line number (`DA:` records).
    /// When a line appears multiple times, only the maximum count is kept.
    pub da_lines: HashMap<i64, i64>,
    /// Branch hit counts keyed by 1-based line number (`BRDA:` records).
    /// Each entry is a `Vec` of counts, one per branch (in declaration order).
    pub brda_data: HashMap<i64, Vec<i64>>,
}

/// Reads and parses an LCOV info file from `filename`.
///
/// Returns one `LcovFile` per `end_of_record` section in the file.
///
/// # Errors
///
/// Returns an error when the file cannot be opened or an I/O error occurs while reading.
pub(crate) fn parse_lcov(filename: &str) -> std::result::Result<Vec<LcovFile>, Error> {
    let file = File::open(filename).map_err(|_| anyhow!("file not found: {filename}"))?;
    let mut files = Vec::new();
    let mut current = LcovFile::default();

    for raw in BufReader::new(file).lines() {
        let line = raw.context("read lcov file")?;
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("SF:") {
            current.path = rest.to_string();
        } else if let Some(rest) = trimmed.strip_prefix("DA:") {
            parse_da(rest, &mut current);
        } else if let Some(rest) = trimmed.strip_prefix("BRDA:") {
            parse_brda(rest, &mut current);
        } else if trimmed == "end_of_record" {
            files.push(std::mem::take(&mut current));
        }
    }
    Ok(files)
}

/// Parses the portion of a `DA:` record after the prefix and updates `current`.
///
/// Expected format: `<line_no>,<count>[,<checksum>]`.
/// Silently ignores malformed records. When the same line number appears more
/// than once, keeps only the maximum count (matching the Go implementation).
fn parse_da(rest: &str, current: &mut LcovFile) {
    let parts: Vec<&str> = rest.splitn(3, ',').collect();
    if parts.len() < 2 {
        return;
    }
    let Ok(ln) = parts[0].parse::<i64>() else {
        return;
    };
    let Ok(cnt) = parts[1].parse::<i64>() else {
        return;
    };
    let existing = current.da_lines.get(&ln).copied();
    if existing.is_none_or(|e| cnt > e) {
        current.da_lines.insert(ln, cnt);
    }
}

/// Parses the portion of a `BRDA:` record after the prefix and updates `current`.
///
/// Expected format: `<line_no>,<block>,<branch>,<taken>` where `<taken>` may be
/// `"-"` (never executed). Silently ignores malformed records.
fn parse_brda(rest: &str, current: &mut LcovFile) {
    let parts: Vec<&str> = rest.splitn(4, ',').collect();
    if parts.len() < 4 {
        return;
    }
    let Ok(ln) = parts[0].parse::<i64>() else {
        return;
    };
    let cnt_str = parts[3];
    let cnt: i64 = if cnt_str == "-" || cnt_str.is_empty() {
        0
    } else {
        cnt_str.parse().unwrap_or(0)
    };
    current.brda_data.entry(ln).or_default().push(cnt);
}

/// Returns `true` when `counts` is non-empty and every value is greater than zero.
fn all_positive(counts: &[i64]) -> bool {
    !counts.is_empty() && counts.iter().all(|c| *c > 0)
}

/// Returns `true` when at least one value in `counts` is greater than zero.
fn any_positive(counts: &[i64]) -> bool {
    counts.iter().any(|c| *c > 0)
}

/// Parses `filename` as an LCOV info file and computes aggregated coverage.
///
/// # Errors
///
/// Returns an error when `parse_lcov` fails (file not found or I/O error).
pub fn compute_lcov_result(
    filename: &str,
    threshold: f64,
) -> std::result::Result<CoverageResult, Error> {
    let files = parse_lcov(filename)?;

    let mut covered = 0usize;
    let mut partial = 0usize;
    let mut missed = 0usize;
    let mut per_file: Vec<FileResult> = Vec::new();

    for file in &files {
        let mut fc = 0usize;
        let mut fp = 0usize;
        let mut fm = 0usize;

        // Classify DA lines first.
        for (line_no, count) in &file.da_lines {
            let branches = file.brda_data.get(line_no);
            if *count > 0 {
                if let Some(b) = branches {
                    if !b.is_empty() && !all_positive(b) {
                        fp += 1;
                    } else {
                        fc += 1;
                    }
                } else {
                    fc += 1;
                }
            } else {
                fm += 1;
            }
        }

        // Then BRDA-only lines.
        for (line_no, branch_counts) in &file.brda_data {
            if !file.da_lines.contains_key(line_no) {
                if all_positive(branch_counts) {
                    fc += 1;
                } else if any_positive(branch_counts) {
                    fp += 1;
                } else {
                    fm += 1;
                }
            }
        }

        covered += fc;
        partial += fp;
        missed += fm;

        let ft = fc + fp + fm;
        let fpct = if ft > 0 {
            100.0 * fc as f64 / ft as f64
        } else {
            100.0
        };
        per_file.push(FileResult {
            path: file.path.clone(),
            covered: fc,
            partial: fp,
            missed: fm,
            total: ft,
            pct: fpct,
        });
    }

    let total = covered + partial + missed;
    let pct = if total > 0 {
        100.0 * covered as f64 / total as f64
    } else {
        100.0
    };

    Ok(CoverageResult {
        file: filename.to_string(),
        format: Format::Lcov,
        covered,
        partial,
        missed,
        total,
        pct,
        threshold,
        passed: pct >= threshold,
        files: per_file,
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn write(dir: &std::path::Path, name: &str, content: &str) -> std::path::PathBuf {
        let p = dir.join(name);
        fs::write(&p, content).unwrap();
        p
    }

    #[test]
    fn parse_lcov_file_not_found() {
        let err = parse_lcov("/nonexistent/lcov.info").unwrap_err();
        assert!(err.to_string().contains("file not found"));
    }

    #[test]
    fn parse_lcov_basic_record() {
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "lcov.info",
            "SF:src/foo.rs\nDA:10,5\nDA:11,0\nend_of_record\n",
        );
        let files = parse_lcov(p.to_str().unwrap()).unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].path, "src/foo.rs");
        assert_eq!(files[0].da_lines.get(&10), Some(&5));
        assert_eq!(files[0].da_lines.get(&11), Some(&0));
    }

    #[test]
    fn parse_lcov_duplicate_da_takes_max() {
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "lcov.info",
            "SF:src/foo.rs\nDA:10,3\nDA:10,7\nDA:10,5\nend_of_record\n",
        );
        let files = parse_lcov(p.to_str().unwrap()).unwrap();
        assert_eq!(files[0].da_lines.get(&10), Some(&7));
    }

    #[test]
    fn parse_lcov_brda_lines_collected() {
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "lcov.info",
            "SF:src/foo.rs\nBRDA:10,0,0,3\nBRDA:10,0,1,-\nend_of_record\n",
        );
        let files = parse_lcov(p.to_str().unwrap()).unwrap();
        let branches = files[0].brda_data.get(&10).unwrap();
        assert_eq!(branches, &vec![3, 0]); // "-" parses to 0
    }

    #[test]
    fn compute_lcov_all_covered_passes() {
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "lcov.info",
            "SF:src/foo.rs\nDA:1,1\nDA:2,5\nend_of_record\n",
        );
        let result = compute_lcov_result(p.to_str().unwrap(), 90.0).unwrap();
        assert_eq!(result.format, Format::Lcov);
        assert_eq!(result.covered, 2);
        assert_eq!(result.missed, 0);
        assert!(result.passed);
    }

    #[test]
    fn compute_lcov_partial_branches_classified_as_partial() {
        let tmp = TempDir::new().unwrap();
        // DA line covered (count > 0), but BRDA shows one branch uncovered → partial.
        let p = write(
            tmp.path(),
            "lcov.info",
            "SF:src/foo.rs\nDA:10,3\nBRDA:10,0,0,3\nBRDA:10,0,1,0\nend_of_record\n",
        );
        let result = compute_lcov_result(p.to_str().unwrap(), 50.0).unwrap();
        assert_eq!(result.partial, 1);
        assert_eq!(result.covered, 0);
    }

    #[test]
    fn compute_lcov_missed_line() {
        let tmp = TempDir::new().unwrap();
        let p = write(
            tmp.path(),
            "lcov.info",
            "SF:src/foo.rs\nDA:1,0\nend_of_record\n",
        );
        let result = compute_lcov_result(p.to_str().unwrap(), 90.0).unwrap();
        assert_eq!(result.missed, 1);
        assert!(!result.passed);
    }

    #[test]
    fn compute_lcov_brda_only_line_classified_via_branches() {
        let tmp = TempDir::new().unwrap();
        // BRDA line without DA: all branches positive → covered.
        let p = write(
            tmp.path(),
            "lcov.info",
            "SF:src/foo.rs\nBRDA:10,0,0,3\nBRDA:10,0,1,7\nend_of_record\n",
        );
        let result = compute_lcov_result(p.to_str().unwrap(), 50.0).unwrap();
        assert_eq!(result.covered, 1);
        assert_eq!(result.partial, 0);
        assert_eq!(result.missed, 0);
    }
}
