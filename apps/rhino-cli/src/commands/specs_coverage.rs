//! `specs coverage` — checks that Gherkin spec steps are covered by test implementations.
//!
//! Port of `apps/rhino-cli/cmd/spec_coverage_validate.go`.
//! Same args (positional specs-dirs + final app-dir), same flags, same exit
//! codes, same byte-for-byte output.

use std::path::{Path, PathBuf};

use anyhow::{Context, Error, anyhow};
use clap::Args;

use crate::domain::cliout::OutputFormat;
use crate::internal::git;
use crate::internal::speccoverage::{checker, reporter, types::ScanOptions};

/// CLI arguments for `spec-coverage validate`.
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
    /// Directory containing unit test implementations (three-level mode).
    #[arg(long = "unit-dir", value_name = "DIR")]
    pub unit_dir: Option<String>,
    /// Directory containing integration test implementations (three-level mode).
    #[arg(long = "integration-dir", value_name = "DIR")]
    pub integration_dir: Option<String>,
    /// Directory containing e2e test implementations (three-level mode).
    #[arg(long = "e2e-dir", value_name = "DIR")]
    pub e2e_dir: Option<String>,
}

/// Level name paired with its absolute directory path.
struct LevelDir {
    /// Short name used in diagnostic output (e.g. `"unit"`, `"integration"`, `"e2e"`).
    name: &'static str,
    /// Absolute path to the directory that contains the level's test implementations.
    dir: PathBuf,
}

/// Determine whether all three level dirs are provided, none, or a partial set.
///
/// Returns `Some(vec)` when all three are present, `None` when none are present,
/// and an `Err` when only some are provided.
fn resolve_level_dirs(
    args: &ValidateArgs,
    repo_root: &Path,
) -> std::result::Result<Option<Vec<LevelDir>>, Error> {
    let count = [&args.unit_dir, &args.integration_dir, &args.e2e_dir]
        .iter()
        .filter(|o| o.is_some())
        .count();

    match count {
        0 => Ok(None),
        3 => Ok(Some(vec![
            LevelDir {
                name: "unit",
                dir: repo_root.join(
                    args.unit_dir
                        .as_deref()
                        .expect("unit_dir is Some in count==3 arm"),
                ),
            },
            LevelDir {
                name: "integration",
                dir: repo_root.join(
                    args.integration_dir
                        .as_deref()
                        .expect("integration_dir is Some in count==3 arm"),
                ),
            },
            LevelDir {
                name: "e2e",
                dir: repo_root.join(
                    args.e2e_dir
                        .as_deref()
                        .expect("e2e_dir is Some in count==3 arm"),
                ),
            },
        ])),
        _ => Err(anyhow!(
            "must provide all three or none of --unit-dir, --integration-dir, --e2e-dir"
        )),
    }
}

/// Check coverage for a single level; print prefixed output and return whether gaps exist.
fn run_level_check(
    level: &LevelDir,
    specs_dirs: &[PathBuf],
    repo_root: &Path,
    args: &ValidateArgs,
    output_format: OutputFormat,
) -> std::result::Result<bool, Error> {
    println!("=== {} level ===", capitalize(level.name));

    let opts = ScanOptions {
        repo_root: repo_root.to_path_buf(),
        specs_dir: specs_dirs[0].clone(),
        specs_dirs: specs_dirs.to_vec(),
        app_dir: level.dir.clone(),
        verbose: false,
        quiet: false,
        shared_steps: args.shared_steps,
        exclude_dirs: args.exclude_dir.clone(),
    };

    let result = checker::check_all(&opts)
        .with_context(|| format!("spec coverage check failed for {} level", level.name))?;

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

    if has_gaps && matches!(output_format, OutputFormat::Text) {
        eprintln!(
            "\n❌ [{}] spec coverage gaps found: {} file gap(s), {} scenario gap(s), {} step gap(s), {} orphan step impl(s)",
            level.name,
            result.gaps.len(),
            result.scenario_gaps.len(),
            result.step_gaps.len(),
            result.orphan_step_impls.len()
        );
    }

    Ok(has_gaps)
}

/// Capitalize the first character of `s`, leaving the rest unchanged.
fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Run three-level mode: one pass per level dir, fail if any level has gaps.
fn run_three_level(
    levels: &[LevelDir],
    specs_dirs: &[PathBuf],
    repo_root: &Path,
    args: &ValidateArgs,
    output_format: OutputFormat,
) -> std::result::Result<(), Error> {
    let mut failing_levels: Vec<&'static str> = Vec::new();

    for level in levels {
        let has_gaps = run_level_check(level, specs_dirs, repo_root, args, output_format)?;
        if has_gaps {
            failing_levels.push(level.name);
        }
    }

    if !failing_levels.is_empty() {
        return Err(anyhow!(
            "spec coverage gaps found at level(s): {}",
            failing_levels.join(", ")
        ));
    }
    Ok(())
}

/// Run the `spec-coverage validate` command.
///
/// # Errors
///
/// Returns an error if the git root cannot be found, fewer than 2 paths are
/// supplied, the coverage check fails, or spec coverage gaps are found.
pub fn run(args: &ValidateArgs, output_format: OutputFormat) -> std::result::Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;

    if args.paths.len() < 2 {
        return Err(anyhow!(
            "spec-coverage validate requires at least 2 positional args (specs-dir... app-dir)"
        ));
    }

    let specs_dirs: Vec<PathBuf> = args.paths[..args.paths.len() - 1]
        .iter()
        .map(|sd| repo_root.join(sd))
        .collect();

    // Resolve three-level mode vs. single-dir mode.
    let level_dirs = resolve_level_dirs(args, &repo_root)?;

    if let Some(levels) = level_dirs {
        return run_three_level(&levels, &specs_dirs, &repo_root, args, output_format);
    }

    // Single-dir (backward-compatible) mode.
    let app_dir: PathBuf = repo_root.join(&args.paths[args.paths.len() - 1]);

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
    use crate::test_support::CwdLock;

    fn base_args(paths: Vec<String>) -> ValidateArgs {
        ValidateArgs {
            paths,
            shared_steps: false,
            exclude_dir: vec![],
            unit_dir: None,
            integration_dir: None,
            e2e_dir: None,
        }
    }

    #[test]
    fn validate_args_requires_two_paths_min() {
        let args = base_args(vec!["only-one".to_string()]);
        assert!(args.paths.len() < 2);
    }

    #[test]
    fn run_returns_err_on_too_few_paths() {
        let _cwd = CwdLock::acquire();
        let args = base_args(vec!["x".to_string()]);
        let err = run(&args, OutputFormat::Text).unwrap_err();
        assert!(err.to_string().contains("requires at least 2"));
    }

    #[test]
    fn run_returns_err_with_gaps_when_specs_missing_test_files() {
        let _cwd = CwdLock::acquire();
        let mut args = base_args(vec![
            "specs/apps/rhino/behavior/rhino-cli/gherkin".to_string(),
            "apps/rhino-cli/scripts".to_string(), // wrong dir → 0 step matchers → step gaps
        ]);
        args.shared_steps = true;
        let err = run(&args, OutputFormat::Text).unwrap_err();
        assert!(err.to_string().contains("spec coverage gaps found"));
    }

    #[test]
    fn run_returns_err_with_json_output_format() {
        let _cwd = CwdLock::acquire();
        let mut args = base_args(vec![
            "specs/apps/rhino/behavior/rhino-cli/gherkin".to_string(),
            "apps/rhino-cli/scripts".to_string(),
        ]);
        args.shared_steps = true;
        let err = run(&args, OutputFormat::Json).unwrap_err();
        assert!(err.to_string().contains("spec coverage gaps found"));
    }

    #[test]
    fn three_level_fails_when_integration_and_e2e_missing() {
        let _cwd = CwdLock::acquire();
        let args = ValidateArgs {
            paths: vec![
                "apps/rhino-cli/tests/fixtures/three-level".to_string(),
                "apps/rhino-cli/tests/fixtures/three-level/unit".to_string(),
            ],
            shared_steps: true,
            exclude_dir: vec![],
            unit_dir: Some("apps/rhino-cli/tests/fixtures/three-level/unit".to_string()),
            integration_dir: Some(
                "apps/rhino-cli/tests/fixtures/three-level/integration".to_string(),
            ),
            e2e_dir: Some("apps/rhino-cli/tests/fixtures/three-level/e2e".to_string()),
        };
        let result = run(&args, OutputFormat::Text);
        assert!(
            result.is_err(),
            "three-level check should fail when integration/e2e dirs are empty"
        );
        let msg = result.unwrap_err().to_string();
        assert!(
            msg.contains("integration") || msg.contains("e2e"),
            "error should mention missing level, got: {msg}"
        );
    }

    #[test]
    fn three_level_passes_when_all_levels_covered() {
        let _cwd = CwdLock::acquire();
        let args = ValidateArgs {
            paths: vec![
                "apps/rhino-cli/tests/fixtures/three-level".to_string(),
                "apps/rhino-cli/tests/fixtures/three-level/unit".to_string(),
            ],
            shared_steps: true,
            exclude_dir: vec![],
            unit_dir: Some("apps/rhino-cli/tests/fixtures/three-level/unit".to_string()),
            integration_dir: Some("apps/rhino-cli/tests/fixtures/three-level/unit".to_string()),
            e2e_dir: Some("apps/rhino-cli/tests/fixtures/three-level/unit".to_string()),
        };
        assert!(
            run(&args, OutputFormat::Text).is_ok(),
            "three-level check should pass when all levels have step implementations"
        );
    }

    #[test]
    fn partial_level_dirs_returns_err() {
        let _cwd = CwdLock::acquire();
        let args = ValidateArgs {
            paths: vec![
                "apps/rhino-cli/tests/fixtures/three-level".to_string(),
                "apps/rhino-cli/tests/fixtures/three-level/unit".to_string(),
            ],
            shared_steps: true,
            exclude_dir: vec![],
            unit_dir: Some("apps/rhino-cli/tests/fixtures/three-level/unit".to_string()),
            integration_dir: None,
            e2e_dir: None,
        };
        let err = run(&args, OutputFormat::Text).unwrap_err();
        assert!(
            err.to_string().contains("must provide all three or none"),
            "partial flags should return error, got: {err}"
        );
    }

    #[test]
    #[ignore = "Real-corpus Gherkin coverage is blocked on the deferred Rust cucumber harness. \
                This test previously passed only because it aggregated the archived/rhino-cli Go step \
                defs, which have now been removed; apps/rhino-cli alone does not yet implement the \
                step definitions. Re-enable once the Rust cucumber harness lands."]
    fn run_returns_ok_on_real_rhino_cli_gherkin() {
        // Runs against the actual repo state. The Rust port's cucumber step definitions are not yet
        // implemented (tracked as deferred work), so the real Gherkin corpus is not fully covered by
        // apps/rhino-cli alone. Ignored until the harness is implemented.
        let _cwd = CwdLock::acquire();
        let args = ValidateArgs {
            paths: vec![
                "specs/apps/rhino/behavior/rhino-cli/gherkin".to_string(),
                "apps/rhino-cli".to_string(),
            ],
            shared_steps: true,
            exclude_dir: vec![],
            unit_dir: None,
            integration_dir: None,
            e2e_dir: None,
        };
        assert!(run(&args, OutputFormat::Text).is_ok());
    }
}
