// Byte-for-byte port of `apps/rhino-cli/internal/testcoverage/go_coverage.go`.
// Algorithm steps (matching tech-docs §Coverage Validator Port):
//   1. Parse cover.out blocks via regex.
//   2. Group blocks by file.
//   3. For each line in each file, collect ALL counts across all blocks covering that line.
//   4. Look up the source-file line; skip blank / comment-only / brace-only lines.
//   5. Classify: covered (all counts > 0), partial (mixed), missed (all counts == 0).
//   6. pct = 100 * covered / (covered + partial + missed). Partial counts as missed in the denominator.
//   7. passed = pct >= threshold.

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use anyhow::{Context, Error, anyhow};
use regex::Regex;

use super::types::{FileResult, Format, Result as CoverageResult};

fn cover_block_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        // Mirrors Go's coverBlockRe at apps/rhino-cli/internal/testcoverage/go_coverage.go:13.
        Regex::new(r"^(.+):(\d+)\.\d+,(\d+)\.\d+ \d+ (\d+)$").expect("valid regex")
    })
}

/// Reads `go.mod` in `dir` and returns the module path (the value after `module`).
/// Returns empty string when go.mod is absent — matching Go's `getModuleNameFrom`.
pub(crate) fn get_module_name_from(dir: &Path) -> String {
    let path = dir.join("go.mod");
    let Ok(file) = File::open(&path) else {
        return String::new();
    };
    for line in BufReader::new(file).lines().map_while(std::io::Result::ok) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 && parts[0] == "module" {
            return parts[1].to_string();
        }
    }
    String::new()
}

/// Returns a map of `line_no` → content for a source file resolved relative
/// to `base_dir`. Returns `None` if the file cannot be opened.
pub(crate) fn get_source_lines_from(
    base_dir: &Path,
    rel_path: &str,
) -> Option<HashMap<usize, String>> {
    let file = File::open(base_dir.join(rel_path)).ok()?;
    let mut lines = HashMap::new();
    for (idx, line) in BufReader::new(file)
        .lines()
        .map_while(std::io::Result::ok)
        .enumerate()
    {
        lines.insert(idx + 1, line);
    }
    Some(lines)
}

/// Classifies whether a source line contains executable Go code. Mirrors
/// `apps/rhino-cli/internal/testcoverage/go_coverage.go:59`.
///
/// Excluded:
///   - Blank lines
///   - Comment-only lines (`//` prefix after trim)
///   - Brace-only lines (`{` or `}` after trim)
///
/// Note: `(` and `)` are NOT excluded (only `{` and `}` are filtered).
pub(crate) fn is_go_code_line(content: &str) -> bool {
    let s = content.trim();
    if s.is_empty() {
        return false;
    }
    if s.starts_with("//") {
        return false;
    }
    if s == "{" || s == "}" {
        return false;
    }
    true
}

#[derive(Debug, Clone)]
pub(crate) struct CoverBlock {
    pub filepath: String,
    pub start_line: usize,
    pub end_line: usize,
    pub count: usize,
}

/// Parses a Go cover.out file. Returns an error matching Go's `file not found: <filename>`
/// when the file cannot be opened.
pub(crate) fn parse_cover_out(filename: &str) -> std::result::Result<Vec<CoverBlock>, Error> {
    let file = File::open(filename).map_err(|_| anyhow!("file not found: {filename}"))?;
    let re = cover_block_re();
    let mut blocks = Vec::new();
    for raw in BufReader::new(file).lines() {
        let line = raw.context("read cover.out")?;
        let trimmed = line.trim();
        if trimmed.starts_with("mode:") || trimmed.is_empty() {
            continue;
        }
        let Some(caps) = re.captures(trimmed) else {
            continue;
        };
        let filepath = caps
            .get(1)
            .expect("capture group 1 always present")
            .as_str()
            .to_string();
        let start_line: usize = caps
            .get(2)
            .expect("capture group 2 always present")
            .as_str()
            .parse()
            .unwrap_or(0);
        let end_line: usize = caps
            .get(3)
            .expect("capture group 3 always present")
            .as_str()
            .parse()
            .unwrap_or(0);
        let count: usize = caps
            .get(4)
            .expect("capture group 4 always present")
            .as_str()
            .parse()
            .unwrap_or(0);
        blocks.push(CoverBlock {
            filepath,
            start_line,
            end_line,
            count,
        });
    }
    Ok(blocks)
}

/// Computes line coverage from a Go cover.out file using a standard
/// line-based algorithm. Source files are resolved relative to the
/// cover.out's directory (mirrors Python cwd behaviour).
pub fn compute_go_result(
    filename: &str,
    threshold: f64,
) -> std::result::Result<CoverageResult, Error> {
    let blocks = parse_cover_out(filename)?;

    // Derive project dir from the cover.out path (mirrors Python cwd behaviour).
    let project_dir: PathBuf = Path::new(filename)
        .parent()
        .map_or_else(|| PathBuf::from("."), Path::to_path_buf);
    let module_name = get_module_name_from(&project_dir);

    // Group blocks by file.
    let mut file_blocks: HashMap<String, Vec<CoverBlock>> = HashMap::new();
    for b in blocks {
        file_blocks.entry(b.filepath.clone()).or_default().push(b);
    }

    let mut covered = 0usize;
    let mut partial = 0usize;
    let mut missed = 0usize;
    let mut per_file: Vec<FileResult> = Vec::new();

    for (fp, fblocks) in &file_blocks {
        // Strip module prefix to get a path relative to the project directory.
        let rel_path: String =
            if !module_name.is_empty() && fp.starts_with(&format!("{module_name}/")) {
                fp[module_name.len() + 1..].to_string()
            } else {
                fp.clone()
            };

        let source = get_source_lines_from(&project_dir, &rel_path);

        // Collect all block counts per line.
        let mut line_counts: HashMap<usize, Vec<usize>> = HashMap::new();
        for b in fblocks {
            for line_no in b.start_line..=b.end_line {
                line_counts.entry(line_no).or_default().push(b.count);
            }
        }

        let mut fc = 0usize;
        let mut fp2 = 0usize;
        let mut fm = 0usize;
        for (line_no, counts) in &line_counts {
            // Skip non-code lines when source is available.
            if let Some(src) = &source {
                let Some(content) = src.get(line_no) else {
                    continue;
                };
                if !is_go_code_line(content) {
                    continue;
                }
            }

            let mut has_covered = false;
            let mut has_missed = false;
            for c in counts {
                if *c > 0 {
                    has_covered = true;
                } else {
                    has_missed = true;
                }
            }

            if has_covered && !has_missed {
                fc += 1;
            } else if has_covered && has_missed {
                fp2 += 1;
            } else {
                fm += 1;
            }
        }

        covered += fc;
        partial += fp2;
        missed += fm;

        let ft = fc + fp2 + fm;
        let fpct = if ft > 0 {
            100.0 * fc as f64 / ft as f64
        } else {
            100.0
        };
        per_file.push(FileResult {
            path: fp.clone(),
            covered: fc,
            partial: fp2,
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
        format: Format::Go,
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

    fn write_temp(dir: &Path, name: &str, content: &str) -> PathBuf {
        let p = dir.join(name);
        fs::write(&p, content).unwrap();
        p
    }

    // ---- isGoCodeLine cases (ported from go_coverage_test.go:18) ----
    #[test]
    fn is_go_code_line_cases() {
        let cases = [
            ("", false),
            ("   ", false),
            ("// comment", false),
            ("\t// indented comment", false),
            ("{", false),
            ("}", false),
            ("  {  ", false),
            ("  }  ", false),
            ("x := 1", true),
            ("return x", true),
            ("func foo() {", true), // has { but is not brace-only
            ("(", true),            // ( is NOT excluded
            (")", true),            // ) is NOT excluded
        ];
        for (input, want) in cases {
            assert_eq!(
                is_go_code_line(input),
                want,
                "is_go_code_line({input:?}) wrong"
            );
        }
    }

    // ---- getModuleNameFrom cases (port of go_coverage_test.go:45 + 53) ----
    #[test]
    fn get_module_name_from_no_go_mod_returns_empty() {
        let tmp = TempDir::new().unwrap();
        assert_eq!(get_module_name_from(tmp.path()), "");
    }

    #[test]
    fn get_module_name_from_with_go_mod_returns_module_path() {
        let tmp = TempDir::new().unwrap();
        write_temp(
            tmp.path(),
            "go.mod",
            "module github.com/example/myapp\n\ngo 1.21\n",
        );
        assert_eq!(get_module_name_from(tmp.path()), "github.com/example/myapp");
    }

    // ---- getSourceLinesFrom cases (port of go_coverage_test.go:66 + 74) ----
    #[test]
    fn get_source_lines_from_missing_file_returns_none() {
        let tmp = TempDir::new().unwrap();
        assert!(get_source_lines_from(tmp.path(), "nonexistent.go").is_none());
    }

    #[test]
    fn get_source_lines_from_valid_file_returns_indexed_lines() {
        let tmp = TempDir::new().unwrap();
        write_temp(tmp.path(), "source.go", "line1\nline2\nline3\n");
        let lines = get_source_lines_from(tmp.path(), "source.go").expect("file readable");
        assert_eq!(lines.get(&1).map(String::as_str), Some("line1"));
        assert_eq!(lines.get(&2).map(String::as_str), Some("line2"));
        assert_eq!(lines.get(&3).map(String::as_str), Some("line3"));
    }

    // ---- parseCoverOut cases (port of go_coverage_test.go:90 + 97) ----
    #[test]
    fn parse_cover_out_file_not_found_errors() {
        let err = parse_cover_out("/nonexistent/cover.out").unwrap_err();
        assert!(err.to_string().contains("file not found"), "got: {err}");
    }

    #[test]
    fn parse_cover_out_valid_block() {
        let tmp = TempDir::new().unwrap();
        let path = write_temp(
            tmp.path(),
            "cover.out",
            "mode: set\ngithub.com/example/myapp/pkg/foo.go:10.1,20.5 1 1\ngithub.com/example/myapp/pkg/foo.go:21.1,30.2 1 0\n",
        );
        let blocks = parse_cover_out(path.to_str().unwrap()).unwrap();
        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0].filepath, "github.com/example/myapp/pkg/foo.go");
        assert_eq!(blocks[0].start_line, 10);
        assert_eq!(blocks[0].end_line, 20);
        assert_eq!(blocks[0].count, 1);
        assert_eq!(blocks[1].count, 0);
    }

    // ---- ComputeGoResult end-to-end ----
    #[test]
    fn compute_go_result_passes_when_above_threshold() {
        let tmp = TempDir::new().unwrap();
        // Module file
        write_temp(tmp.path(), "go.mod", "module example.com/proj\n");
        // Source file
        write_temp(
            tmp.path(),
            "foo.go",
            "package foo\n\n// comment\nfunc Foo() {\n  return\n}\n",
        );
        // Cover file marks lines 4-5 covered
        let cover = write_temp(
            tmp.path(),
            "cover.out",
            "mode: set\nexample.com/proj/foo.go:4.1,5.2 1 1\n",
        );
        let result = compute_go_result(cover.to_str().unwrap(), 50.0).unwrap();
        assert_eq!(result.format, Format::Go);
        assert!(result.passed, "expected pass at 50% threshold");
        assert!(result.pct > 0.0);
    }

    #[test]
    fn compute_go_result_fails_when_below_threshold() {
        let tmp = TempDir::new().unwrap();
        write_temp(tmp.path(), "go.mod", "module example.com/proj\n");
        write_temp(
            tmp.path(),
            "foo.go",
            "func Foo() {\n  x := 1\n  y := 2\n}\n",
        );
        // Cover lines 2-3 uncovered (count 0)
        let cover = write_temp(
            tmp.path(),
            "cover.out",
            "mode: set\nexample.com/proj/foo.go:2.1,3.2 1 0\n",
        );
        let result = compute_go_result(cover.to_str().unwrap(), 90.0).unwrap();
        assert!(!result.passed);
    }

    #[test]
    fn compute_go_result_classifies_partial_as_missed_in_denominator() {
        // Two blocks cover the same line — one with count 1, one with count 0 → partial.
        let tmp = TempDir::new().unwrap();
        write_temp(tmp.path(), "go.mod", "module example.com/proj\n");
        write_temp(
            tmp.path(),
            "foo.go",
            "package foo\nfunc Foo() {\n  x := 1\n  return\n}\n",
        );
        let cover = write_temp(
            tmp.path(),
            "cover.out",
            "mode: set\nexample.com/proj/foo.go:3.1,3.2 1 1\nexample.com/proj/foo.go:3.1,3.2 1 0\n",
        );
        let result = compute_go_result(cover.to_str().unwrap(), 50.0).unwrap();
        // Line 3 is partial (one block had count >0, another 0) — counts as missed in denominator.
        assert!(result.partial > 0 || result.missed > 0);
        // With only partial+missed lines and 0 covered, pct should be 0 → fail at 50% threshold.
        assert!(!result.passed);
    }

    #[test]
    fn compute_go_result_skips_non_code_lines_when_source_available() {
        let tmp = TempDir::new().unwrap();
        write_temp(tmp.path(), "go.mod", "module example.com/proj\n");
        // Source has a `}` brace-only line at line 2 — should be skipped.
        write_temp(tmp.path(), "foo.go", "package foo\n}\nfunc Foo() {}\n");
        let cover = write_temp(
            tmp.path(),
            "cover.out",
            "mode: set\nexample.com/proj/foo.go:2.1,2.2 1 0\n",
        );
        let result = compute_go_result(cover.to_str().unwrap(), 50.0).unwrap();
        // Line 2 is `}` — skipped → total lines 0 → pct 100 → pass.
        assert_eq!(result.total, 0);
        assert!((result.pct - 100.0).abs() < 0.001);
        assert!(result.passed);
    }

    #[test]
    fn compute_go_result_returns_threshold_in_result() {
        let tmp = TempDir::new().unwrap();
        let cover = write_temp(tmp.path(), "cover.out", "mode: set\n");
        let result = compute_go_result(cover.to_str().unwrap(), 87.5).unwrap();
        assert!((result.threshold - 87.5).abs() < 0.001);
    }
}
