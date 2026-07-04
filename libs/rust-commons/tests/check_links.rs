//! Cucumber-rs harness for `check_links`, wiring
//! `specs/libs/rust-commons/behavior/gherkin/links/check-links.feature` to the
//! real `rust_commons::links::check_links` function.

#![allow(clippy::missing_docs_in_private_items)]

use std::path::PathBuf;

use cucumber::{World as _, given, then, when};
use rust_commons::links::{CheckResult, check_links};
use tempfile::TempDir;

#[derive(cucumber::World)]
#[world(init = Self::new)]
struct LinksWorld {
    content_dir: TempDir,
    result: Option<CheckResult>,
}

impl std::fmt::Debug for LinksWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LinksWorld").finish_non_exhaustive()
    }
}

impl LinksWorld {
    fn new() -> Self {
        Self {
            content_dir: TempDir::new().expect("temp content dir"),
            result: None,
        }
    }

    fn write_page(&self, name: &str, contents: &str) {
        let path = self.content_dir.path().join(name);
        std::fs::write(path, contents).expect("write page");
    }
}

// Scenario: A broken internal link is reported
// cucumber-rs regex captures require an owned FromStr type (`&str` has no
// std FromStr impl), so clippy's needless_pass_by_value is a false positive here.
#[allow(clippy::needless_pass_by_value)]
#[given(regex = r#"^a content directory with a markdown file linking to "([^"]+)"$"#)]
fn given_page_linking_to(w: &mut LinksWorld, target: String) {
    w.write_page("page.md", &format!("[link]({target})"));
}

// Scenario: A valid internal link is not reported as broken
#[given("a content directory with a markdown file linking to an existing page")]
fn given_page_linking_to_existing_page(w: &mut LinksWorld) {
    w.write_page("target.md", "# Target page");
    w.write_page("page.md", "[link](/target)");
}

#[when("I run check_links on the content directory")]
fn when_run_check_links(w: &mut LinksWorld) {
    let result = check_links(w.content_dir.path()).expect("check_links failed");
    w.result = Some(result);
}

#[allow(clippy::needless_pass_by_value)]
#[then(regex = r"^the result should contain (\d+) broken links?$")]
fn then_result_contains_n_broken_links(w: &mut LinksWorld, n: String) {
    let expected: usize = n.parse().expect("broken-link count");
    let result = w.result.as_ref().expect("check_links must have run");
    assert_eq!(
        result.broken_links.len(),
        expected,
        "expected {expected} broken link(s), got {:?}",
        result.broken_links
    );
}

#[tokio::main]
async fn main() {
    LinksWorld::cucumber()
        .fail_on_skipped()
        .run_and_exit(feature_dir())
        .await;
}

fn feature_dir() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .join("../../specs/libs/rust-commons/behavior/gherkin/links")
        .canonicalize()
        .expect("feature dir resolvable")
}
