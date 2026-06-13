//! `specs audit` — runs all specs validators in sequence.
//!
//! Runs validate-adoption, validate-counts, validate-links, validate-tree,
//! and gherkin-cardinality with default arguments.  The specs bc, ul, and
//! coverage validators require domain-specific arguments and are excluded.
//! Use `--skip <name>` to exclude individual validators.

use anyhow::{Error, anyhow};
use clap::Args;

use crate::commands::{
    specs_gherkin_cardinality, specs_validate_adoption, specs_validate_counts,
    specs_validate_links, specs_validate_tree,
};
use crate::domain::cliout::OutputFormat;

/// Member validators run by `specs audit` in order.
///
/// `coverage`, `bc`, and `ul` are intentionally excluded because they require
/// domain-specific positional arguments that `audit` cannot reasonably default.
const MEMBERS: &[&str] = &[
    "validate-adoption",
    "validate-counts",
    "validate-links",
    "validate-tree",
    "gherkin-cardinality",
];

/// CLI arguments for `specs audit`.
#[derive(Args, Debug)]
pub struct AuditArgs {
    /// Validator name to skip (repeatable).
    #[arg(long = "skip")]
    pub skip: Vec<String>,
}

/// Run the `specs audit` command.
///
/// Runs specs validators with default arguments.  Collects all failures
/// and returns an aggregated error when any validator fails.
///
/// # Errors
///
/// Returns an aggregated error listing each failing validator.
pub fn run(args: &AuditArgs, output_format: OutputFormat) -> std::result::Result<(), Error> {
    let mut failures: Vec<String> = Vec::new();

    for &name in MEMBERS {
        if args.skip.iter().any(|s| s == name) {
            continue;
        }
        let result = run_member(name, output_format);
        if let Err(e) = result {
            failures.push(format!("{name}: {e}"));
        }
    }

    if failures.is_empty() {
        println!(
            "SPECS AUDIT PASSED: all {} validators passed",
            MEMBERS.len() - args.skip.len()
        );
        Ok(())
    } else {
        eprintln!(
            "SPECS AUDIT FAILED: {} validator(s) reported failures",
            failures.len()
        );
        for f in &failures {
            eprintln!("  {f}");
        }
        Err(anyhow!("specs audit found {} failure(s)", failures.len()))
    }
}

/// Dispatch a single validator by name with default arguments.
fn run_member(name: &str, output_format: OutputFormat) -> std::result::Result<(), Error> {
    match name {
        "validate-adoption" => specs_validate_adoption::run(
            &specs_validate_adoption::ValidateAdoptionArgs {
                app: None,
                apps: vec![],
            },
            output_format,
        ),
        "validate-counts" => specs_validate_counts::run(
            &specs_validate_counts::ValidateCountsArgs {
                folder: None,
                apps: vec![],
            },
            output_format,
        ),
        "validate-links" => specs_validate_links::run(
            &specs_validate_links::ValidateLinksArgs {
                folder: None,
                apps: vec![],
            },
            output_format,
        ),
        "validate-tree" => specs_validate_tree::run(
            &specs_validate_tree::ValidateTreeArgs {
                app: None,
                apps: vec![],
            },
            output_format,
        ),
        "gherkin-cardinality" => specs_gherkin_cardinality::run(
            &specs_gherkin_cardinality::GherkinKeywordCardinalityArgs {
                path: vec![],
                positional: vec![],
            },
            output_format,
        ),
        _ => Err(anyhow!("unknown specs validator: {name}")),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn members_list_has_expected_count() {
        assert_eq!(MEMBERS.len(), 5);
    }

    #[test]
    fn members_list_contains_expected_validators() {
        assert!(MEMBERS.contains(&"validate-adoption"));
        assert!(MEMBERS.contains(&"validate-counts"));
        assert!(MEMBERS.contains(&"validate-links"));
        assert!(MEMBERS.contains(&"validate-tree"));
        assert!(MEMBERS.contains(&"gherkin-cardinality"));
    }

    #[test]
    fn run_with_all_skipped_succeeds() {
        let args = AuditArgs {
            skip: MEMBERS.iter().map(|&s| s.to_string()).collect(),
        };
        let result = run(&args, OutputFormat::Text);
        assert!(result.is_ok());
    }

    #[test]
    fn run_member_unknown_returns_error() {
        let r = run_member("not-a-thing", OutputFormat::Text);
        assert!(r.is_err());
        let msg = r.unwrap_err().to_string();
        assert!(msg.contains("unknown specs validator"));
    }

    #[test]
    fn run_member_covers_all_known_names_without_panic() {
        for &name in MEMBERS {
            let _ = run_member(name, OutputFormat::Text);
        }
    }

    #[test]
    fn run_with_partial_skip_does_not_panic() {
        // Skip all but one to trigger failure path.
        let args = AuditArgs {
            skip: vec![
                "validate-adoption".to_string(),
                "validate-counts".to_string(),
                "validate-links".to_string(),
                "validate-tree".to_string(),
            ],
        };
        // gherkin-cardinality may error (no git root), triggering failure aggregation.
        let _ = run(&args, OutputFormat::Text);
    }

    #[test]
    fn audit_args_skip_is_vec() {
        let a = AuditArgs {
            skip: vec!["validate-adoption".to_string()],
        };
        assert_eq!(a.skip.len(), 1);
    }
}
