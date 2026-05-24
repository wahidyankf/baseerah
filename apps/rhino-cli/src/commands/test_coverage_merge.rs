// Port of `apps/rhino-cli/cmd/test_coverage_merge.go`.

use anyhow::{Error, anyhow};
use clap::Args;

use crate::internal::cliout::OutputFormat;
use crate::internal::gitutil;
use crate::internal::testcoverage::merge::{
    CoverageMap, merge_coverage_maps, result_from_coverage_map, to_coverage_map, write_lcov,
};
use crate::internal::testcoverage::{exclude::matches_any_exclude_pattern, reporter};

#[derive(Args, Debug)]
pub struct MergeArgs {
    /// Coverage files (minimum 2) relative to git repo root.
    #[arg(num_args = 2.., required = true)]
    pub files: Vec<String>,
    /// Output file path (LCOV format).
    #[arg(long = "out-file", default_value = "")]
    pub out_file: String,
    /// Validate merged coverage against threshold.
    #[arg(long = "validate", default_value = "")]
    pub validate: String,
    /// Exclude files matching glob pattern (repeatable).
    #[arg(long = "exclude", value_name = "PATTERN")]
    pub exclude: Vec<String>,
    #[arg(long, short = 'v')]
    pub verbose: bool,
    #[arg(long, short = 'q')]
    pub quiet: bool,
}

pub fn run(args: &MergeArgs, output: OutputFormat) -> std::result::Result<(), Error> {
    let repo_root =
        gitutil::find_git_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;

    let mut maps: Vec<CoverageMap> = Vec::with_capacity(args.files.len());
    for arg in &args.files {
        let abs_path = repo_root.join(arg);
        let abs_path_str = abs_path
            .to_str()
            .ok_or_else(|| anyhow!("non-utf8 coverage file path"))?;
        let cm =
            to_coverage_map(abs_path_str).map_err(|e| anyhow!("failed to parse {arg}: {e}"))?;
        maps.push(cm);
    }

    let mut merged = merge_coverage_maps(&maps);

    if !args.exclude.is_empty() {
        let to_drop: Vec<String> = merged
            .keys()
            .filter(|p| matches_any_exclude_pattern(p, &args.exclude))
            .cloned()
            .collect();
        for p in to_drop {
            merged.remove(&p);
        }
    }

    if !args.out_file.is_empty() {
        let out_path = repo_root.join(&args.out_file);
        write_lcov(&merged, &out_path).map_err(|e| anyhow!("failed to write output: {e}"))?;
    }

    let threshold: f64 = if args.validate.is_empty() {
        0.0
    } else {
        args.validate.parse().map_err(|_| {
            anyhow!(
                "invalid --validate threshold {:?}: must be a number",
                args.validate
            )
        })?
    };

    let mut result = result_from_coverage_map(&merged, threshold);
    result.file = "merged".into();

    match output {
        OutputFormat::Text => print!(
            "{}",
            reporter::format_text(&result, args.verbose, args.quiet)
        ),
        OutputFormat::Json => println!("{}", reporter::format_json(&result, false, 0.0)?),
        OutputFormat::Markdown => print!("{}", reporter::format_markdown(&result, false, 0.0)),
    }

    if !args.validate.is_empty() && !result.passed {
        return Err(anyhow!(
            "merged coverage {:.2}% is below threshold {:.0}%",
            result.pct,
            threshold
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn args_default_values() {
        let _ = MergeArgs {
            files: vec!["a".into(), "b".into()],
            out_file: String::new(),
            validate: String::new(),
            exclude: vec![],
            verbose: false,
            quiet: false,
        };
    }
}
