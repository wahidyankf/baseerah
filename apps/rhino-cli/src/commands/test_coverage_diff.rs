//! `test-coverage diff` — computes test coverage for lines changed since a base ref.
//!
//! Port of `apps/rhino-cli/cmd/test_coverage_diff.go`.

use anyhow::{Error, anyhow};
use clap::Args;

use crate::domain::cliout::OutputFormat;
use crate::internal::git;
use crate::internal::testcoverage::diff::{DiffCoverageOptions, compute_diff_coverage};
use crate::internal::testcoverage::reporter;

/// CLI arguments for `test-coverage diff`.
#[derive(Args, Debug)]
pub struct DiffArgs {
    /// Coverage file path relative to git repo root.
    pub coverage_file: String,
    /// Git ref to diff against.
    #[arg(long = "base", default_value = "main")]
    pub base: String,
    /// Fail if diff coverage below this percentage.
    #[arg(long = "threshold", default_value_t = 0.0)]
    pub threshold: f64,
    /// Diff staged changes instead of branch diff.
    #[arg(long = "staged")]
    pub staged: bool,
    /// Show per-file diff coverage breakdown.
    #[arg(long = "per-file")]
    pub per_file: bool,
    /// Exclude files matching glob pattern (repeatable).
    #[arg(long = "exclude", value_name = "PATTERN")]
    pub exclude: Vec<String>,
    /// Verbose output.
    #[arg(long, short = 'v')]
    pub verbose: bool,
    /// Quiet output.
    #[arg(long, short = 'q')]
    pub quiet: bool,
}

/// Run the `test-coverage diff` command.
///
/// # Errors
///
/// Returns an error if the git root cannot be found, the diff computation fails,
/// or coverage is below the threshold.
pub fn run(args: &DiffArgs, output: OutputFormat) -> std::result::Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    let abs_path = repo_root.join(&args.coverage_file);
    let abs_path_str = abs_path
        .to_str()
        .ok_or_else(|| anyhow!("non-utf8 coverage file path"))?;

    let opts = DiffCoverageOptions {
        coverage_file: abs_path_str.to_string(),
        base: args.base.clone(),
        staged: args.staged,
        threshold: args.threshold,
        per_file: args.per_file,
        exclude_patterns: args.exclude.clone(),
    };

    let result = compute_diff_coverage(&opts).map_err(|e| anyhow!("diff coverage failed: {e}"))?;

    let per_file_text = if args.per_file {
        reporter::format_text_per_file(&result, 0.0)
    } else {
        String::new()
    };

    match output {
        OutputFormat::Text => print!(
            "{}{}",
            reporter::format_text(&result, args.verbose, args.quiet),
            per_file_text
        ),
        OutputFormat::Json => println!("{}", reporter::format_json(&result, args.per_file, 0.0)?),
        OutputFormat::Markdown => {
            print!("{}", reporter::format_markdown(&result, args.per_file, 0.0));
        }
    }

    if args.threshold > 0.0 && !result.passed {
        return Err(anyhow!(
            "diff coverage {:.2}% is below threshold {:.0}%",
            result.pct,
            args.threshold
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn args_default_values() {
        let _ = DiffArgs {
            coverage_file: "x".into(),
            base: "main".into(),
            threshold: 0.0,
            staged: false,
            per_file: false,
            exclude: vec![],
            verbose: false,
            quiet: false,
        };
    }
}
