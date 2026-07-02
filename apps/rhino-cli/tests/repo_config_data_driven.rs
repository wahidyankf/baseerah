//! Cucumber-rs suite asserting rhino-cli reads its repo-specific behaviour
//! (env globs, domain-areas, ddd-areas) from `repo-config.yml` rather than from
//! source-hard-coded per-repo literals.
//!
//! Wires `specs/apps/rhino/behavior/rhino-cli/gherkin/repo-config/` to step
//! definitions that build a synthetic repo whose `repo-config.yml` declares a
//! custom ddd-area, then drive `specs counts validate` and assert the custom
//! area (not a hard-coded default) drives the scan.

#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::unwrap_used, clippy::panic)]

use std::path::{Path, PathBuf};

use cucumber::{World as _, given, then, when};
use rhino_cli::application::specs::required_spec_folders;
use rhino_cli::commands::specs_validate_counts::{ValidateCountsArgs, run_at_root};
use tempfile::TempDir;

#[derive(cucumber::World)]
#[world(init = Self::new)]
struct RepoConfigDataWorld {
    /// Synthetic repo whose repo-config.yml declares a custom ddd-area.
    repo: TempDir,
    /// Result of running `specs counts validate` with no explicit app list.
    ran_ok: bool,
    /// Captured stdout of the run.
    output: String,
}

impl std::fmt::Debug for RepoConfigDataWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RepoConfigDataWorld")
            .finish_non_exhaustive()
    }
}

impl RepoConfigDataWorld {
    fn new() -> Self {
        Self {
            repo: TempDir::new().expect("temp repo"),
            ran_ok: false,
            output: String::new(),
        }
    }
}

fn write(root: &Path, rel: &str, content: &str) {
    let p = root.join(rel);
    std::fs::create_dir_all(p.parent().unwrap()).unwrap();
    std::fs::write(p, content).unwrap();
}

#[given("rhino-cli's repo-specific behaviour (env globs, domain/ddd areas)")]
fn given_repo_config_declares_custom_ddd_area(w: &mut RepoConfigDataWorld) {
    let root = w.repo.path();
    // repo-config.yml declares a custom ddd-area that is NOT one of the historical
    // hard-coded defaults (organiclever / ose).
    write(
        root,
        "repo-config.yml",
        concat!(
            "harness: []\n",
            "coverage:\n  projects: []\n",
            "specs:\n  ddd-areas:\n    - widget-app\n  domain-areas: []\n",
        ),
    );
    // Give widget-app a complete, clean spec tree so a data-driven scan passes.
    for sub in required_spec_folders() {
        write(root, &format!("specs/apps/widget-app/{sub}/a.md"), "x\n");
    }
}

#[when("rhino-cli runs")]
fn when_specs_counts_runs(w: &mut RepoConfigDataWorld) {
    let args = ValidateCountsArgs {
        folder: None,
        apps: vec![],
    };
    let mut buf: Vec<u8> = Vec::new();
    let result = run_at_root(w.repo.path(), &args, &mut buf);
    w.ran_ok = result.is_ok();
    w.output = String::from_utf8_lossy(&buf).into_owned();
}

#[then("it reads that behaviour from repo-config.yml, not from source hard-coded per repo")]
fn then_reads_from_repo_config(w: &mut RepoConfigDataWorld) {
    assert!(
        w.output.contains("widget-app"),
        "default scan must target the repo-config.yml ddd-area 'widget-app' (data-driven), \
         not source-hard-coded defaults; got output: {}",
        w.output
    );
    assert!(
        w.ran_ok,
        "the config-declared widget-app tree is clean, so the run must succeed; got output: {}",
        w.output
    );
}

#[tokio::main]
async fn main() {
    RepoConfigDataWorld::run(feature_dir()).await;
}

fn feature_dir() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .join("../../specs/apps/rhino/behavior/rhino-cli/gherkin/repo-config")
        .canonicalize()
        .expect("feature dir resolvable")
}
