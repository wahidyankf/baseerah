//! `specs structure validate` — merged structural validator.
//!
//! Runs adoption → tree → counts in sequence over one tree walk, aggregating failures with
//! per-layer labels (`adoption:` / `tree:` / `counts:`). Replaces the three removed leaf
//! commands `specs validate adoption`, `specs validate tree`, `specs validate counts`.

use anyhow::{Error, anyhow};
use clap::Args;

use crate::application::repo_config;
use crate::application::specs::{
    validate_spec_adoption_ddd_aware, validate_spec_counts, validate_spec_gherkin_domains,
    validate_spec_tree,
};
use crate::domain::cliout::OutputFormat;
use crate::internal::bcregistry;
use crate::internal::git;
use crate::internal::glossary;

/// CLI arguments for `specs structure validate`.
#[derive(Args, Debug)]
pub struct ValidateStructureArgs {
    /// Optional single app name.
    #[arg(value_name = "app")]
    pub app: Option<String>,
    /// Comma-separated app names.
    #[arg(long = "apps", value_delimiter = ',')]
    pub apps: Vec<String>,
}

/// Discover all spec area directories under `specs/apps/`.
fn discover_spec_areas(repo_root: &std::path::Path) -> Vec<String> {
    let specs_apps = repo_root.join("specs/apps");
    let Ok(entries) = std::fs::read_dir(&specs_apps) else {
        return vec![];
    };
    let mut areas: Vec<String> = entries
        .flatten()
        .filter(|e| e.path().is_dir())
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    areas.sort();
    areas
}

/// Resolve the list of spec areas to validate from positional/flag inputs or filesystem.
fn resolve_apps(
    positional: Option<&String>,
    flag: &[String],
    repo_root: &std::path::Path,
) -> Vec<String> {
    if let Some(p) = positional {
        return vec![p.clone()];
    }
    if !flag.is_empty() {
        return flag.to_vec();
    }
    discover_spec_areas(repo_root)
}

/// Run the `specs structure validate` command.
///
/// # Errors
///
/// Returns an error if the git root cannot be found or findings are detected.
pub fn run(args: &ValidateStructureArgs, _output: OutputFormat) -> std::result::Result<(), Error> {
    let repo_root =
        git::root::find_root().map_err(|e| anyhow!("failed to find git repository root: {e}"))?;
    run_at_root(&repo_root, args, &mut std::io::stdout())
}

/// Run `specs structure validate` from a known `repo_root` (testable entry point).
///
/// # Errors
///
/// Returns an error if output cannot be written or findings are detected.
pub fn run_at_root(
    repo_root: &std::path::Path,
    args: &ValidateStructureArgs,
    w: &mut dyn std::io::Write,
) -> std::result::Result<(), Error> {
    let config = repo_config::load(repo_root).unwrap_or_default();
    let ddd_areas = &config.specs.ddd_areas;
    let apps = resolve_apps(args.app.as_ref(), &args.apps, repo_root);
    let mut total = 0usize;

    for app in &apps {
        let is_ddd_area = ddd_areas.iter().any(|a| a == app);

        // Layer 1: adoption (ddd-aware)
        let adoption_findings = validate_spec_adoption_ddd_aware(repo_root, app, is_ddd_area);
        for f in &adoption_findings {
            writeln!(w, "adoption: {}: HIGH: {}", f.file, f.evidence)?;
        }
        total += adoption_findings.len();

        // Layer 2: tree (includes gherkin-domain checks)
        let mut tree_findings = validate_spec_tree(repo_root, app);
        tree_findings.extend(validate_spec_gherkin_domains(repo_root, app));
        for f in &tree_findings {
            writeln!(w, "tree: {}: HIGH: {}", f.file, f.evidence)?;
        }
        total += tree_findings.len();

        // Layer 3: counts
        let counts_findings = validate_spec_counts(repo_root, &format!("specs/apps/{app}"));
        for f in &counts_findings {
            writeln!(w, "counts: {}: HIGH: {}", f.file, f.evidence)?;
        }
        total += counts_findings.len();

        // Layers 4+5: bc/ul — only for ddd-areas
        let mut bc_count = 0usize;
        let mut ul_count = 0usize;
        if is_ddd_area {
            match bcregistry::validate_all(&bcregistry::ValidateOptions {
                repo_root: repo_root.to_path_buf(),
                app: app.clone(),
                severity: None,
            }) {
                Ok(findings) => {
                    for f in &findings {
                        writeln!(w, "bc: {}: {}: {}", f.file, f.severity.code(), f.message)?;
                    }
                    bc_count = findings.len();
                }
                Err(e) => {
                    writeln!(w, "bc: {app}: HIGH: {e}")?;
                    bc_count = 1;
                }
            }
            match glossary::validate_all(&glossary::ValidateOptions {
                repo_root: repo_root.to_path_buf(),
                app: app.clone(),
                severity: None,
            }) {
                Ok(findings) => {
                    for f in &findings {
                        writeln!(w, "ul: {}: {}: {}", f.file, f.severity.code(), f.message)?;
                    }
                    ul_count = findings.len();
                }
                Err(e) => {
                    writeln!(w, "ul: {app}: HIGH: {e}")?;
                    ul_count = 1;
                }
            }
            total += bc_count + ul_count;
        }

        if adoption_findings.is_empty()
            && tree_findings.is_empty()
            && counts_findings.is_empty()
            && bc_count == 0
            && ul_count == 0
        {
            writeln!(w, "specs structure validate: 0 finding(s) for \"{app}\"")?;
        }
    }

    if total > 0 {
        return Err(anyhow!(
            "{total} finding(s) found by specs structure validate"
        ));
    }
    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn resolve_apps_positional() {
        let dir = tempfile::tempdir().unwrap();
        let v = resolve_apps(Some(&"foo".to_string()), &[], dir.path());
        assert_eq!(v, vec!["foo".to_string()]);
    }

    #[test]
    fn resolve_apps_flag() {
        let dir = tempfile::tempdir().unwrap();
        let v = resolve_apps(None, &["a".to_string(), "b".to_string()], dir.path());
        assert_eq!(v, vec!["a", "b"]);
    }

    #[test]
    fn resolve_apps_default_discovers_filesystem_areas() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("specs/apps/alpha")).unwrap();
        std::fs::create_dir_all(dir.path().join("specs/apps/beta")).unwrap();
        let v = resolve_apps(None, &[], dir.path());
        assert_eq!(
            v,
            vec!["alpha", "beta"],
            "must discover dirs from specs/apps/"
        );
    }

    fn write_repo_config(root: &std::path::Path, ddd_areas: &[&str]) {
        let areas = ddd_areas
            .iter()
            .map(|s| format!("  - {s}"))
            .collect::<Vec<_>>()
            .join("\n");
        let yaml = if areas.is_empty() {
            "specs:\n  ddd-areas: []\n  domain-areas: []\n".to_string()
        } else {
            format!("specs:\n  ddd-areas:\n{areas}\n  domain-areas: []\n")
        };
        std::fs::write(root.join("repo-config.yml"), yaml).unwrap();
    }

    fn build_complete_corpus(root: &std::path::Path, app: &str, with_ddd: bool) {
        let required = [
            "product",
            "system-context",
            "containers",
            "components",
            "behavior",
        ];
        for sub in required {
            let p = root.join(format!("specs/apps/{app}/{sub}"));
            std::fs::create_dir_all(&p).unwrap();
            std::fs::write(p.join("README.md"), "# readme").unwrap();
            std::fs::write(p.join("spec.md"), "# spec").unwrap();
        }
        std::fs::write(
            root.join(format!("specs/apps/{app}/behavior/a.feature")),
            "x",
        )
        .unwrap();
        if with_ddd {
            std::fs::create_dir_all(root.join(format!("specs/apps/{app}/ddd"))).unwrap();
            std::fs::write(
                root.join(format!("specs/apps/{app}/ddd/bounded-contexts.yaml")),
                "y",
            )
            .unwrap();
        }
    }

    #[test]
    fn run_at_root_clean_corpus() {
        // Complete valid corpus — "x" is NOT a ddd-area, so no ddd/ folder is present or required.
        // bc/ul checks only run for ddd-areas, so this test isolates adoption+tree+counts layers.
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_repo_config(root, &[]); // "x" not in ddd-areas
        build_complete_corpus(root, "x", false); // no ddd/ folder
        let args = ValidateStructureArgs {
            app: Some("x".to_string()),
            apps: vec![],
        };
        let mut out = Vec::new();
        let result = run_at_root(root, &args, &mut out);
        let output = String::from_utf8(out).unwrap();
        assert!(
            result.is_ok(),
            "clean corpus must produce no errors; got: {output}"
        );
        assert!(
            output.contains("0 finding(s)"),
            "should report 0 findings; got: {output}"
        );
    }

    #[test]
    fn run_at_root_missing_adoption_emits_adoption_label() {
        // Empty dir: no behavior/ → adoption layer fires
        let dir = tempfile::tempdir().unwrap();
        let args = ValidateStructureArgs {
            app: Some("missing".to_string()),
            apps: vec![],
        };
        let mut out = Vec::new();
        let result = run_at_root(dir.path(), &args, &mut out);
        assert!(result.is_err(), "missing spec dirs must be an error");
        let output = String::from_utf8(out).unwrap();
        assert!(
            output.contains("adoption:"),
            "adoption layer must be labelled; got: {output}"
        );
    }

    // ---- RED4 tests: identical-structure rules (not yet implemented) ----

    #[test]
    fn non_ddd_area_without_ddd_dir_should_be_clean() {
        // "widget-app" is NOT in specs.ddd-areas → ddd/ must NOT be required.
        // RED: currently validate_spec_adoption always requires ddd/bounded-contexts.yaml,
        // so it will produce an adoption error even though the area is not a ddd-area.
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_repo_config(root, &[]); // empty ddd-areas
        build_complete_corpus(root, "widget-app", false); // no ddd/ folder
        let args = ValidateStructureArgs {
            app: Some("widget-app".to_string()),
            apps: vec![],
        };
        let mut out = Vec::new();
        let result = run_at_root(root, &args, &mut out);
        let output = String::from_utf8(out).unwrap();
        assert!(
            result.is_ok(),
            "non-ddd area without ddd/ must be clean; got: {output}"
        );
    }

    #[test]
    fn non_ddd_area_with_unexpected_ddd_dir_should_error() {
        // "widget-app" NOT in specs.ddd-areas but HAS ddd/ → must error with adoption: label.
        // RED: currently no check for unexpected ddd/ on non-ddd areas; run_at_root returns Ok.
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_repo_config(root, &[]); // empty ddd-areas
        build_complete_corpus(root, "widget-app", true); // WITH ddd/ folder
        let args = ValidateStructureArgs {
            app: Some("widget-app".to_string()),
            apps: vec![],
        };
        let mut out = Vec::new();
        let result = run_at_root(root, &args, &mut out);
        let output = String::from_utf8(out).unwrap();
        assert!(
            result.is_err(),
            "non-ddd area with unexpected ddd/ must error; got: {output}"
        );
        assert!(
            output.contains("adoption:"),
            "must emit adoption label for unexpected ddd/; got: {output}"
        );
    }

    #[test]
    fn all_spec_areas_discovered_from_filesystem_not_just_ddd_allowlist() {
        // When no args given, command must walk ALL dirs under specs/apps/ (not a hardcoded list).
        // resolve_apps(None, &[]) discovers spec areas from the filesystem, so a
        // "discovered-app" (not in any hardcoded list) is checked and appears in output.
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_repo_config(root, &[]);
        // Create an incomplete spec area — after GREEN this should be discovered and flagged.
        std::fs::create_dir_all(root.join("specs/apps/discovered-app")).unwrap();
        let args = ValidateStructureArgs {
            app: None,
            apps: vec![],
        };
        let mut out = Vec::new();
        let _result = run_at_root(root, &args, &mut out);
        let output = String::from_utf8(out).unwrap();
        assert!(
            output.contains("discovered-app"),
            "output must mention discovered-app when all dirs are walked; got: {output}"
        );
    }

    // ---- RED5 tests: bc/ul checks folded into specs structure validate ----

    #[test]
    fn ddd_area_emits_bc_label_in_output() {
        // specs structure validate must run bounded-context parity (bc: label) on ddd-areas.
        // RED: currently no bc check runs inside structure validate, so "bc:" never appears.
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_repo_config(root, &["my-ddd"]);
        build_complete_corpus(root, "my-ddd", true);
        let args = ValidateStructureArgs {
            app: Some("my-ddd".to_string()),
            apps: vec![],
        };
        let mut out = Vec::new();
        let _result = run_at_root(root, &args, &mut out);
        let output = String::from_utf8(out).unwrap();
        assert!(
            output.contains("bc:"),
            "ddd-area must produce bc: label in output (bc check not yet wired); got: {output}"
        );
    }

    #[test]
    fn ddd_area_emits_ul_label_in_output() {
        // specs structure validate must run ubiquitous-language parity (ul: label) on ddd-areas.
        // RED: currently no ul check runs inside structure validate, so "ul:" never appears.
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_repo_config(root, &["my-ddd"]);
        build_complete_corpus(root, "my-ddd", true);
        let args = ValidateStructureArgs {
            app: Some("my-ddd".to_string()),
            apps: vec![],
        };
        let mut out = Vec::new();
        let _result = run_at_root(root, &args, &mut out);
        let output = String::from_utf8(out).unwrap();
        assert!(
            output.contains("ul:"),
            "ddd-area must produce ul: label in output (ul check not yet wired); got: {output}"
        );
    }
}
