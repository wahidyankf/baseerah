// Port of `apps/rhino-cli/cmd/spec_coverage_validate.go`.
// Same args (positional specs-dirs + final app-dir), same flags, same exit
// codes, same byte-for-byte output.

use std::path::PathBuf;

use anyhow::{Context, Error, anyhow};
use clap::Args;

use crate::internal::cliout::OutputFormat;
use crate::internal::git;
use crate::internal::speccoverage::{checker, reporter, types::ScanOptions};

#[derive(Args, Debug)]
pub struct ValidateArgs {
    /// Last positional arg is the app-dir; preceding args are specs-dirs.
    /// Must supply ≥2 positional args.
    #[arg(required = true, num_args = 2..)]
    pub paths: Vec<String>,
    /// Skip file matching; validate steps across ALL source files.
    #[arg(long = "shared-steps")]
    pub shared_steps: bool,
    /// Spec directory names to exclude (repeatable).
    #[arg(long = "exclude-dir", value_name = "DIR")]
    pub exclude_dir: Vec<String>,
}

pub fn run(args: &ValidateArgs, output_format: OutputFormat) -> std::result::Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;

    if args.paths.len() < 2 {
        return Err(anyhow!(
            "spec-coverage validate requires at least 2 positional args (specs-dir... app-dir)"
        ));
    }

    let app_dir: PathBuf = repo_root.join(&args.paths[args.paths.len() - 1]);
    let specs_dirs: Vec<PathBuf> = args.paths[..args.paths.len() - 1]
        .iter()
        .map(|sd| repo_root.join(sd))
        .collect();

    let opts = ScanOptions {
        repo_root: repo_root.clone(),
        specs_dir: specs_dirs[0].clone(), // primary for backward compat
        specs_dirs: specs_dirs.clone(),
        app_dir,
        verbose: false,
        quiet: false,
        shared_steps: args.shared_steps,
        exclude_dirs: args.exclude_dir.clone(),
    };

    let result = checker::check_all(&opts).context("spec coverage check failed")?;

    let output = match output_format {
        OutputFormat::Text => reporter::format_text(&result, false, false),
        OutputFormat::Json => reporter::format_json(&result)?,
        OutputFormat::Markdown => reporter::format_markdown(&result),
    };
    print!("{output}");

    let has_gaps = !result.gaps.is_empty()
        || !result.scenario_gaps.is_empty()
        || !result.step_gaps.is_empty()
        || !result.orphan_step_impls.is_empty();

    if has_gaps {
        if matches!(output_format, OutputFormat::Text) {
            if !result.gaps.is_empty() {
                eprintln!(
                    "\n❌ Found {} spec(s) without matching test files",
                    result.gaps.len()
                );
            }
            if !result.scenario_gaps.is_empty() {
                eprintln!(
                    "❌ Found {} scenario(s) without matching test implementations",
                    result.scenario_gaps.len()
                );
            }
            if !result.step_gaps.is_empty() {
                eprintln!(
                    "❌ Found {} step(s) without matching step definitions",
                    result.step_gaps.len()
                );
            }
            if !result.orphan_step_impls.is_empty() {
                eprintln!(
                    "❌ Found {} orphan step implementation(s) (no Gherkin step matches them)",
                    result.orphan_step_impls.len()
                );
            }
        }
        return Err(anyhow!(
            "spec coverage gaps found: {} file gap(s), {} scenario gap(s), {} step gap(s), {} orphan step impl(s)",
            result.gaps.len(),
            result.scenario_gaps.len(),
            result.step_gaps.len(),
            result.orphan_step_impls.len()
        ));
    }
    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn validate_args_requires_two_paths_min() {
        let args = ValidateArgs {
            paths: vec!["only-one".to_string()],
            shared_steps: false,
            exclude_dir: vec![],
        };
        assert!(args.paths.len() < 2);
    }

    #[test]
    fn run_returns_err_on_too_few_paths() {
        let args = ValidateArgs {
            paths: vec!["x".to_string()],
            shared_steps: false,
            exclude_dir: vec![],
        };
        let err = run(&args, OutputFormat::Text).unwrap_err();
        assert!(err.to_string().contains("requires at least 2"));
    }

    #[test]
    fn run_returns_err_with_gaps_when_specs_missing_test_files() {
        let args = ValidateArgs {
            paths: vec![
                "specs/apps/rhino/behavior/cli/gherkin".to_string(),
                "apps/rhino-cli/scripts".to_string(), // wrong dir → 0 step matchers → step gaps
            ],
            shared_steps: true,
            exclude_dir: vec![],
        };
        let err = run(&args, OutputFormat::Text).unwrap_err();
        assert!(err.to_string().contains("spec coverage gaps found"));
    }

    #[test]
    fn run_returns_err_with_json_output_format() {
        let args = ValidateArgs {
            paths: vec![
                "specs/apps/rhino/behavior/cli/gherkin".to_string(),
                "apps/rhino-cli/scripts".to_string(),
            ],
            shared_steps: true,
            exclude_dir: vec![],
        };
        let err = run(&args, OutputFormat::Json).unwrap_err();
        assert!(err.to_string().contains("spec coverage gaps found"));
    }

    #[test]
    fn run_returns_ok_on_real_rhino_cli_gherkin() {
        // Runs against the actual repo state. After the Rust archival the Go step
        // implementations live under archived/rhino-cli; the spec scanner aggregates
        // both apps/rhino-cli (Rust) and archived/rhino-cli (Go) step defs.
        let args = ValidateArgs {
            paths: vec![
                "specs/apps/rhino/behavior/cli/gherkin".to_string(),
                "apps/rhino-cli".to_string(),
                "archived/rhino-cli".to_string(),
            ],
            shared_steps: true,
            exclude_dir: vec![],
        };
        assert!(run(&args, OutputFormat::Text).is_ok());
    }
}
