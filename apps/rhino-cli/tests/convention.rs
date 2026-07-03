//! Cucumber-rs integration test scaffold for the `convention` command group
//! (`convention emoji validate`, `convention license validate`,
//! `convention audit`).
//!
//! This binary is a deliberately **empty step scaffold** — it binds
//! `specs/apps/rhino/behavior/rhino-cli/gherkin/convention/` (the feature
//! files split out of `gherkin/repo-governance/` in the Phase 1 §1·0 rename
//! step) but registers no `#[given]/#[when]/#[then]` step definitions yet.
//! Every scenario in this directory therefore executes as "skipped" (no
//! matching step), which is the expected state for a pure rename/re-binding
//! step: the scenario count moves from being counted inside
//! `repo_governance`'s skip total to this binary's skip total, with no net
//! change in the suite-wide skip count. De-hollowing (real step definitions
//! wired to the `convention emoji validate` / `convention license validate`
//! commands) happens in a later Phase 1 gap-fill step.

#![allow(clippy::missing_docs_in_private_items)]

use std::path::PathBuf;

use cucumber::World as _;

#[derive(Debug, Default, cucumber::World)]
struct ConventionWorld;

#[tokio::main]
async fn main() {
    ConventionWorld::run(feature_dir()).await;
}

fn feature_dir() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .join("../../specs/apps/rhino/behavior/rhino-cli/gherkin/convention")
        .canonicalize()
        .expect("feature dir resolvable")
}
