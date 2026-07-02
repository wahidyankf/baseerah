//! `test-coverage validate` — validates coverage from LCOV, `JaCoCo`, Cobertura, or Go files.
//!
//! Port of `apps/rhino-cli/cmd/test_coverage_validate.go`.
//! Same output, same exit codes, same byte-for-byte text format.

use std::path::Path;

use anyhow::{Context, Error, anyhow};
use clap::Args;

use crate::application::testcoverage::{
    cobertura, detect, go_coverage, jacoco, lcov, reporter,
    types::{Format, Result as CoverageResult},
};
use crate::domain::cliout::OutputFormat;
use crate::internal::git;

/// CLI arguments for `test-coverage validate`.
#[derive(Args, Debug)]
pub struct ValidateArgs {
    /// Coverage file path relative to git repo root.
    pub coverage_file: String,
    /// Threshold percentage (e.g. 85).
    pub threshold: String,
    /// Show per-file coverage breakdown.
    #[arg(long)]
    pub per_file: bool,
    /// With --per-file, show only files below this coverage percentage.
    #[arg(long, default_value_t = 0.0)]
    pub below_threshold: f64,
    /// Exclude files matching glob pattern (repeatable).
    #[arg(long, value_name = "PATTERN")]
    pub exclude: Vec<String>,
}

/// Run the `test-coverage validate` command.
///
/// # Errors
///
/// Returns an error if the git root cannot be found, the threshold cannot be
/// parsed, the coverage file cannot be parsed, or coverage is below the threshold.
pub fn run(args: &ValidateArgs, output_format: OutputFormat) -> std::result::Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    let abs_path = repo_root.join(&args.coverage_file);

    let threshold: f64 = args.threshold.parse().map_err(|_| {
        anyhow!(
            "invalid threshold {:?}: must be a number (e.g. 85)",
            args.threshold
        )
    })?;

    let abs_path_str = abs_path
        .to_str()
        .ok_or_else(|| anyhow!("non-utf8 coverage file path"))?;

    let result: CoverageResult = match detect::detect_format(abs_path_str) {
        Format::Lcov => {
            lcov::compute_lcov_result(abs_path_str, threshold).context("coverage check failed")?
        }
        Format::Jacoco => jacoco::compute_jacoco_result(abs_path_str, threshold)
            .context("coverage check failed")?,
        Format::Cobertura => cobertura::compute_cobertura_result(abs_path_str, threshold)
            .context("coverage check failed")?,
        Format::Go => go_coverage::compute_go_result(abs_path_str, threshold)
            .context("coverage check failed")?,
        Format::Diff => {
            return Err(anyhow!(
                "diff format is not a valid input format for validate"
            ));
        }
    };

    // Apply exclude patterns (filenames matching any glob pattern are dropped).
    let filtered = if args.exclude.is_empty() {
        result
    } else {
        apply_exclude(result, &args.exclude)
    };

    let per_file_text = if args.per_file {
        reporter::format_text_per_file(&filtered, args.below_threshold)
    } else {
        String::new()
    };

    let output = match output_format {
        OutputFormat::Text => format!(
            "{}{}",
            reporter::format_text(&filtered, false, false),
            per_file_text
        ),
        OutputFormat::Json => {
            reporter::format_json(&filtered, args.per_file, args.below_threshold)?
        }
        OutputFormat::Markdown => {
            reporter::format_markdown(&filtered, args.per_file, args.below_threshold)
        }
    };

    print!("{output}");

    if !filtered.passed {
        return Err(anyhow!(
            "coverage {:.2}% is below threshold {:.0}%",
            filtered.pct,
            threshold
        ));
    }
    Ok(())
}

/// Filters out `FileResult` entries whose path matches any of the supplied glob patterns
/// and recomputes the aggregate counts. Mirrors Go's `testcoverage.ExcludeFiles`.
fn apply_exclude(mut result: CoverageResult, patterns: &[String]) -> CoverageResult {
    let matchers: Vec<glob::Pattern> = patterns
        .iter()
        .filter_map(|p| glob::Pattern::new(p).ok())
        .collect();
    if matchers.is_empty() {
        return result;
    }
    result.files.retain(|f| {
        !matchers
            .iter()
            .any(|m| m.matches(&f.path) || m.matches_path(Path::new(&f.path)))
    });
    let mut covered = 0usize;
    let mut partial = 0usize;
    let mut missed = 0usize;
    for f in &result.files {
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
    result.covered = covered;
    result.partial = partial;
    result.missed = missed;
    result.total = total;
    result.pct = pct;
    result.passed = pct >= result.threshold;
    result
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn write(dir: &Path, name: &str, content: &str) -> std::path::PathBuf {
        let p = dir.join(name);
        fs::write(&p, content).unwrap();
        p
    }

    #[test]
    fn apply_exclude_drops_matching_files_and_recomputes() {
        let mut result = CoverageResult {
            file: "x".into(),
            format: Format::Go,
            covered: 10,
            partial: 0,
            missed: 5,
            total: 15,
            pct: 66.67,
            threshold: 80.0,
            passed: false,
            files: vec![
                crate::application::testcoverage::types::FileResult {
                    path: "src/test_mock.rs".into(),
                    covered: 0,
                    partial: 0,
                    missed: 5,
                    total: 5,
                    pct: 0.0,
                },
                crate::application::testcoverage::types::FileResult {
                    path: "src/real.rs".into(),
                    covered: 10,
                    partial: 0,
                    missed: 0,
                    total: 10,
                    pct: 100.0,
                },
            ],
        };
        result = apply_exclude(result, &["src/test_*.rs".to_string()]);
        assert_eq!(result.files.len(), 1);
        assert_eq!(result.files[0].path, "src/real.rs");
        assert_eq!(result.covered, 10);
        assert_eq!(result.missed, 0);
        assert_eq!(result.total, 10);
        assert!((result.pct - 100.0).abs() < 0.001);
        assert!(result.passed);
    }

    #[test]
    fn run_on_passing_go_cover_out_succeeds() {
        let tmp = TempDir::new().unwrap();
        write(tmp.path(), "go.mod", "module example.com/p\n");
        write(tmp.path(), "foo.go", "func F() { x := 1 }\n");
        let cover = write(
            tmp.path(),
            "cover.out",
            "mode: set\nexample.com/p/foo.go:1.1,1.20 1 1\n",
        );
        let r = crate::application::testcoverage::go_coverage::compute_go_result(
            cover.to_str().unwrap(),
            50.0,
        )
        .unwrap();
        assert!(r.passed);
    }

    #[test]
    fn apply_exclude_no_patterns_returns_unchanged() {
        let result = CoverageResult {
            file: "x".into(),
            format: Format::Go,
            covered: 5,
            partial: 0,
            missed: 5,
            total: 10,
            pct: 50.0,
            threshold: 80.0,
            passed: false,
            files: vec![],
        };
        let unchanged = apply_exclude(result.clone(), &[]);
        assert_eq!(unchanged.covered, result.covered);
        assert_eq!(unchanged.total, result.total);
    }

    #[test]
    fn apply_exclude_with_invalid_glob_skips_pattern() {
        let result = CoverageResult {
            file: "x".into(),
            format: Format::Go,
            covered: 0,
            partial: 0,
            missed: 0,
            total: 0,
            pct: 100.0,
            threshold: 80.0,
            passed: true,
            files: vec![crate::application::testcoverage::types::FileResult {
                path: "a.rs".into(),
                covered: 5,
                partial: 0,
                missed: 0,
                total: 5,
                pct: 100.0,
            }],
        };
        let r = apply_exclude(result, &["[invalid".to_string()]);
        // Invalid glob is filtered out → no patterns remain → matchers.is_empty() → return early.
        assert_eq!(r.files.len(), 1);
    }

    #[test]
    fn apply_exclude_recomputes_aggregate_to_zero_total() {
        let result = CoverageResult {
            file: "x".into(),
            format: Format::Go,
            covered: 0,
            partial: 0,
            missed: 0,
            total: 0,
            pct: 100.0,
            threshold: 80.0,
            passed: true,
            files: vec![crate::application::testcoverage::types::FileResult {
                path: "drop.rs".into(),
                covered: 5,
                partial: 0,
                missed: 0,
                total: 5,
                pct: 100.0,
            }],
        };
        let r = apply_exclude(result, &["drop.rs".to_string()]);
        assert_eq!(r.files.len(), 0);
        assert_eq!(r.total, 0);
        assert!((r.pct - 100.0).abs() < 0.001);
        assert!(r.passed); // 100% on empty → passes any threshold
    }
}
