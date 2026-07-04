//! Cucumber-rs harness for the `links check` subcommand, wiring
//! `specs/apps/ose/behavior/ose-cli/gherkin/links/links-check.feature` to the
//! real `ose-cli` binary via `assert_cmd`.

#![allow(clippy::missing_docs_in_private_items)]

use std::path::PathBuf;

use assert_cmd::Command;
use cucumber::{World as _, given, then, when};
use tempfile::TempDir;

#[derive(cucumber::World)]
#[world(init = Self::new)]
struct LinksCheckWorld {
    content_dir: TempDir,
    stdout: Option<String>,
    success: Option<bool>,
}

impl std::fmt::Debug for LinksCheckWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LinksCheckWorld").finish_non_exhaustive()
    }
}

impl LinksCheckWorld {
    fn new() -> Self {
        Self {
            content_dir: TempDir::new().expect("temp content dir"),
            stdout: None,
            success: None,
        }
    }

    fn write_page(&self, name: &str, contents: &str) {
        let path = self.content_dir.path().join(name);
        std::fs::write(path, contents).expect("write page");
    }
}

// @covers specs/apps/ose/behavior/ose-cli/gherkin/links/links-check.feature:A content directory with all valid internal links passes validation
#[given("ose-www content where all internal links resolve correctly")]
fn given_valid_content(w: &mut LinksCheckWorld) {
    w.write_page("target.md", "# Target page");
    w.write_page("index.md", "[link](/target)");
}

// @covers specs/apps/ose/behavior/ose-cli/gherkin/links/links-check.feature:A broken internal link is detected and reported
#[given("ose-www content with a link pointing to a non-existent page")]
fn given_broken_link(w: &mut LinksCheckWorld) {
    w.write_page("index.md", "[broken](/does-not-exist)");
}

// @covers specs/apps/ose/behavior/ose-cli/gherkin/links/links-check.feature:External URLs are not validated
#[given("ose-www content with only external HTTPS links")]
fn given_external_only_content(w: &mut LinksCheckWorld) {
    w.write_page("index.md", "[external](https://example.com/page)");
}

#[when("the developer runs links check")]
fn when_run_links_check(w: &mut LinksCheckWorld) {
    let assert = Command::cargo_bin("ose-cli")
        .expect("binary not found")
        .args(["links", "check", "--content"])
        .arg(w.content_dir.path())
        .assert();
    let output = assert.get_output();
    w.success = Some(output.status.success());
    w.stdout = Some(String::from_utf8_lossy(&output.stdout).into_owned());
}

// @covers specs/apps/ose/behavior/ose-cli/gherkin/links/links-check.feature:JSON output produces structured results
#[when("the developer runs links check with JSON output")]
fn when_run_links_check_json(w: &mut LinksCheckWorld) {
    let assert = Command::cargo_bin("ose-cli")
        .expect("binary not found")
        .args(["links", "check", "--content"])
        .arg(w.content_dir.path())
        .args(["-o", "json"])
        .assert();
    let output = assert.get_output();
    w.success = Some(output.status.success());
    w.stdout = Some(String::from_utf8_lossy(&output.stdout).into_owned());
}

#[then("the command exits successfully")]
fn then_exits_successfully(w: &mut LinksCheckWorld) {
    assert!(
        w.success.expect("command must have run"),
        "expected success"
    );
}

#[then("the command exits with a failure code")]
fn then_exits_with_failure(w: &mut LinksCheckWorld) {
    assert!(
        !w.success.expect("command must have run"),
        "expected failure"
    );
}

#[then("the output is valid JSON")]
fn then_output_is_valid_json(w: &mut LinksCheckWorld) {
    let stdout = w.stdout.as_ref().expect("command must have run");
    let v: serde_json::Value = serde_json::from_str(stdout).expect("stdout must be valid JSON");
    assert!(v.get("status").is_some());
    assert!(v.get("checked").is_some());
    assert!(v.get("broken_links").is_some());
}

#[tokio::main]
async fn main() {
    LinksCheckWorld::cucumber()
        .fail_on_skipped()
        .run_and_exit(feature_dir())
        .await;
}

fn feature_dir() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .join("../../specs/apps/ose/behavior/ose-cli/gherkin/links")
        .canonicalize()
        .expect("feature dir resolvable")
}
