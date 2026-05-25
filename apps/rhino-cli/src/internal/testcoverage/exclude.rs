//! File-exclusion helpers using Go `filepath.Match` glob semantics.
//!
//! Port of `apps/rhino-cli/internal/testcoverage/exclude.go`.

use std::path::Path;

use super::types::Result as CoverageResult;

/// Drop files matching any glob pattern. Recalculate aggregates from remaining files.
pub fn exclude_files(r: &mut CoverageResult, patterns: &[String]) {
    if patterns.is_empty() || r.files.is_empty() {
        return;
    }

    let kept: Vec<_> = r
        .files
        .iter()
        .filter(|f| !matches_any_exclude_pattern(&f.path, patterns))
        .cloned()
        .collect();

    let mut covered = 0usize;
    let mut partial = 0usize;
    let mut missed = 0usize;
    for f in &kept {
        covered += f.covered;
        partial += f.partial;
        missed += f.missed;
    }
    let total = covered + partial + missed;
    let pct = if total > 0 {
        100.0 * covered as f64 / total as f64
    } else {
        100.0
    };

    r.files = kept;
    r.covered = covered;
    r.partial = partial;
    r.missed = missed;
    r.total = total;
    r.pct = pct;
    r.passed = pct >= r.threshold;
}

/// True if `path` matches any glob pattern using Go's filepath.Match semantics.
/// Go's Match does not honor `**`; only `?`, `*`, and character classes within a single segment.
pub fn matches_any_exclude_pattern(path: &str, patterns: &[String]) -> bool {
    let base = Path::new(path)
        .file_name()
        .map_or_else(|| path.to_string(), |s| s.to_string_lossy().to_string());

    for pattern in patterns {
        if go_filepath_match(pattern, path) {
            return true;
        }
        if go_filepath_match(pattern, &base) {
            return true;
        }
    }
    false
}

/// Port of Go's `path/filepath.Match`. Single-segment globbing; `*` does not cross `/`.
///
/// Supports `?` (any single non-`/` char), `*` (any sequence of non-`/` chars),
/// `[…]` character classes with optional `^` negation and `lo-hi` ranges, and
/// `\` escaping of literal metacharacters.
fn go_filepath_match(pattern: &str, name: &str) -> bool {
    let p: Vec<char> = pattern.chars().collect();
    let n: Vec<char> = name.chars().collect();
    go_match_rec(&p, 0, &n, 0)
}

/// Recursive match engine backing `go_filepath_match`.
///
/// `pi` is the current index into the pattern slice `p`; `ni` is the current
/// index into the name slice `n`. Returns `true` when the remaining pattern
/// matches the remaining name characters.
fn go_match_rec(p: &[char], mut pi: usize, n: &[char], mut ni: usize) -> bool {
    while pi < p.len() {
        match p[pi] {
            '*' => {
                pi += 1;
                // Match zero or more chars (not '/').
                if pi == p.len() {
                    // Trailing '*' matches anything up to '/' or end.
                    while ni < n.len() {
                        if n[ni] == '/' {
                            return false;
                        }
                        ni += 1;
                    }
                    return true;
                }
                let mut k = ni;
                loop {
                    if go_match_rec(p, pi, n, k) {
                        return true;
                    }
                    if k >= n.len() || n[k] == '/' {
                        return false;
                    }
                    k += 1;
                }
            }
            '?' => {
                if ni >= n.len() || n[ni] == '/' {
                    return false;
                }
                ni += 1;
                pi += 1;
            }
            '[' => {
                if ni >= n.len() {
                    return false;
                }
                let ch = n[ni];
                let mut j = pi + 1;
                let negate = j < p.len() && p[j] == '^';
                if negate {
                    j += 1;
                }
                let mut matched = false;
                while j < p.len() && p[j] != ']' {
                    let lo = p[j];
                    let mut hi = lo;
                    if j + 2 < p.len() && p[j + 1] == '-' {
                        hi = p[j + 2];
                        j += 2;
                    }
                    if ch >= lo && ch <= hi {
                        matched = true;
                    }
                    j += 1;
                }
                if j >= p.len() {
                    return false; // malformed
                }
                if matched == negate {
                    return false;
                }
                pi = j + 1;
                ni += 1;
            }
            '\\' if pi + 1 < p.len() => {
                if ni >= n.len() || n[ni] != p[pi + 1] {
                    return false;
                }
                pi += 2;
                ni += 1;
            }
            c => {
                if ni >= n.len() || n[ni] != c {
                    return false;
                }
                pi += 1;
                ni += 1;
            }
        }
    }
    ni == n.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::internal::testcoverage::types::{FileResult, Format, Result as CoverageResult};

    fn fr(path: &str, c: usize, m: usize) -> FileResult {
        FileResult {
            path: path.into(),
            covered: c,
            partial: 0,
            missed: m,
            total: c + m,
            pct: 100.0,
        }
    }

    fn res(files: Vec<FileResult>) -> CoverageResult {
        let covered: usize = files.iter().map(|f| f.covered).sum();
        let missed: usize = files.iter().map(|f| f.missed).sum();
        CoverageResult {
            file: String::new(),
            format: Format::Lcov,
            covered,
            partial: 0,
            missed,
            total: covered + missed,
            pct: 0.0,
            threshold: 0.0,
            passed: true,
            files,
        }
    }

    #[test]
    fn matches_basename_glob() {
        assert!(matches_any_exclude_pattern(
            "src/foo_test.go",
            &["*_test.go".into()]
        ));
    }

    #[test]
    fn matches_path_glob() {
        assert!(matches_any_exclude_pattern(
            "generated/foo.go",
            &["generated/*".into()]
        ));
    }

    #[test]
    fn no_match_returns_false() {
        assert!(!matches_any_exclude_pattern(
            "src/foo.go",
            &["*_test.go".into()]
        ));
    }

    #[test]
    fn exclude_files_drops_matching() {
        let mut r = res(vec![fr("src/a.go", 1, 0), fr("src/a_test.go", 1, 0)]);
        exclude_files(&mut r, &["*_test.go".into()]);
        assert_eq!(r.files.len(), 1);
        assert_eq!(r.files[0].path, "src/a.go");
    }

    #[test]
    fn exclude_files_recomputes_aggregates() {
        let mut r = res(vec![fr("src/a.go", 1, 0), fr("src/b.go", 0, 1)]);
        r.threshold = 50.0;
        exclude_files(&mut r, &["src/b.go".into()]);
        assert_eq!(r.covered, 1);
        assert_eq!(r.missed, 0);
        assert!((r.pct - 100.0).abs() < 1e-9);
        assert!(r.passed);
    }

    #[test]
    fn empty_patterns_no_change() {
        let before = res(vec![fr("src/a.go", 1, 0)]);
        let mut r = before.clone();
        exclude_files(&mut r, &[]);
        assert_eq!(r, before);
    }

    #[test]
    fn star_does_not_cross_slash() {
        assert!(!go_filepath_match("a/*", "a/b/c"));
        assert!(go_filepath_match("a/*", "a/b"));
    }

    #[test]
    fn question_mark_single_char() {
        assert!(go_filepath_match("?.go", "a.go"));
        assert!(!go_filepath_match("?.go", "ab.go"));
    }

    #[test]
    fn char_class_match() {
        assert!(go_filepath_match("[ab].go", "a.go"));
        assert!(go_filepath_match("[ab].go", "b.go"));
        assert!(!go_filepath_match("[ab].go", "c.go"));
    }
}
