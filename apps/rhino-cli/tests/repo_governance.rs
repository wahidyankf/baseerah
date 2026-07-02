//! Cucumber-rs integration tests for the `repo-governance vendor validate` and
//! `specs gherkin-cardinality validate` commands (feature file names and
//! Gherkin step text still say "gherkin-keyword-cardinality" for historical
//! reasons — the underlying CLI subcommand moved under the `specs` noun
//! during the phase 9a/b/c command-surface rationalization).
//!
//! Wires the behavior-contract feature files at
//! `specs/apps/rhino/behavior/rhino-cli/gherkin/repo-governance/` to step definitions
//! that synthesize markdown / feature-file fixtures inside a fresh git-rooted
//! temp workspace and drive the compiled `rhino-cli` binary, asserting on its
//! output and exit code.

// Test step-definition scaffolding: private World state and step fns are
// self-documenting via their #[given]/#[when]/#[then] gherkin strings.
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::doc_markdown)]

use std::path::PathBuf;
use std::process::Output;

use assert_cmd::cargo::cargo_bin;
use cucumber::{World as _, given, then, when};
use tempfile::TempDir;

/// Shared scenario state. Each scenario gets a fresh git-rooted temp workspace
/// so the binary's `findGitRoot` resolves inside the fixture.
#[derive(cucumber::World)]
#[world(init = Self::new)]
struct GovernanceWorld {
    work: TempDir,
    /// Repo-relative path of the fixture file or directory to audit.
    target: String,
    /// Full CLI subcommand path under test (e.g.
    /// `["repo-governance", "vendor", "validate"]`), excluding the trailing
    /// target path and `--no-color` flag.
    subcommand: Vec<&'static str>,
    output: Option<Output>,
}

impl std::fmt::Debug for GovernanceWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GovernanceWorld")
            .field("target", &self.target)
            .finish_non_exhaustive()
    }
}

impl GovernanceWorld {
    fn new() -> Self {
        let work = TempDir::new().expect("temp workspace");
        init_git_repo(work.path());
        Self {
            work,
            target: String::new(),
            subcommand: vec!["repo-governance", "vendor", "validate"],
            output: None,
        }
    }

    fn write(&self, rel: &str, content: &str) {
        let p = self.work.path().join(rel);
        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent).expect("mk fixture dir");
        }
        std::fs::write(p, content).expect("write fixture");
    }

    fn exec(&mut self) {
        let mut args: Vec<&str> = self.subcommand.clone();
        args.push(&self.target);
        args.push("--no-color");
        let out = std::process::Command::new(cargo_bin("rhino-cli"))
            .args(&args)
            .current_dir(self.work.path())
            .output()
            .expect("run rhino-cli");
        self.output = Some(out);
    }

    fn stdout(&self) -> String {
        String::from_utf8_lossy(&self.output.as_ref().expect("ran").stdout).into_owned()
    }

    fn exit_code(&self) -> i32 {
        self.output
            .as_ref()
            .expect("ran")
            .status
            .code()
            .unwrap_or(-1)
    }
}

fn init_git_repo(dir: &std::path::Path) {
    std::process::Command::new("git")
        .args(["init", "-q"])
        .current_dir(dir)
        .env("GIT_AUTHOR_NAME", "t")
        .env("GIT_AUTHOR_EMAIL", "t@t")
        .env("GIT_COMMITTER_NAME", "t")
        .env("GIT_COMMITTER_EMAIL", "t@t")
        .output()
        .expect("git init");
}

// ===========================================================================
// Given steps
// ===========================================================================

// Matches any quoted term/path in plain prose, including the Scenario Outline
// placeholders (`"<term>"`, `"<path>"`) and the substituted example values
// (e.g. `"Junie"`, `".junie/"`). The captured token is embedded verbatim in the
// fixture prose so the scanner has something to flag.
#[given(regex = r#"^a governance markdown file containing "([^"]+)" in plain prose$"#)]
#[allow(clippy::needless_pass_by_value)] // cucumber-rs binds the capture by value
fn given_term_in_prose(w: &mut GovernanceWorld, term: String) {
    w.target = "repo-governance/doc.md".to_string();
    w.write(
        &w.target.clone(),
        &format!("# Doc\n\nWe use {term} daily.\n"),
    );
}

#[given(r#"a governance markdown file containing "Claude Code" inside a code fence"#)]
fn given_brand_in_fence(w: &mut GovernanceWorld) {
    w.target = "repo-governance/doc.md".to_string();
    w.write(&w.target.clone(), "# Doc\n\n```\nClaude Code\n```\n");
}

#[given(r#"a governance markdown file containing "Claude Code" inside a binding-example fence"#)]
fn given_brand_in_binding_example(w: &mut GovernanceWorld) {
    w.target = "repo-governance/doc.md".to_string();
    w.write(
        &w.target.clone(),
        "# Doc\n\n```binding-example\nClaude Code\n```\n",
    );
}

// Matches any quoted term under a "Platform Binding Examples" heading (the
// term is exempt there). Covers both `"Claude Code"` and the `"Junie"` outline
// example value.
#[given(
    regex = r#"^a governance markdown file containing "([^"]+)" under a "Platform Binding Examples" heading$"#
)]
#[allow(clippy::needless_pass_by_value)] // cucumber-rs binds the capture by value
fn given_term_under_pb_heading(w: &mut GovernanceWorld, term: String) {
    w.target = "repo-governance/doc.md".to_string();
    w.write(
        &w.target.clone(),
        &format!("# Doc\n\n## Platform Binding Examples\n\n{term} is fine here.\n"),
    );
}

#[given("a governance directory with no forbidden terms in prose")]
fn given_clean_directory(w: &mut GovernanceWorld) {
    w.target = "repo-governance".to_string();
    w.write(
        "repo-governance/a.md",
        "# A\n\nVendor-neutral prose only.\n",
    );
    w.write(
        "repo-governance/b.md",
        "# B\n\nThe coding agent does the work.\n",
    );
}

#[given(r#"a governance markdown file containing "Skills" inside a code fence"#)]
fn given_skills_in_fence(w: &mut GovernanceWorld) {
    w.target = "repo-governance/doc.md".to_string();
    w.write(&w.target.clone(), "# Doc\n\n```\nSkills\n```\n");
}

#[given(r#"a feature file containing a scenario with two primary "When" keywords"#)]
fn given_feature_two_primary_whens(w: &mut GovernanceWorld) {
    w.target = "specs/violating.feature".to_string();
    w.write(
        &w.target.clone(),
        "Feature: Fixture\n\n  Scenario: Double when offender\n    Given a start\n    When the first action runs\n    When the second action runs\n    Then the outcome is checked\n",
    );
}

#[given(r#"a feature file whose scenarios each use one primary keyword chained with "And""#)]
fn given_feature_conforming_chained(w: &mut GovernanceWorld) {
    w.target = "specs/conforming.feature".to_string();
    w.write(
        &w.target.clone(),
        "Feature: Fixture\n\n  Scenario: Conforming chained scenario\n    Given a start\n    And another precondition\n    When the action runs\n    Then the outcome is checked\n    And a second outcome is checked\n    But a third outcome is absent\n",
    );
}

#[given(r#"a feature file whose Background block repeats the "Given" keyword"#)]
fn given_feature_background_repeats(w: &mut GovernanceWorld) {
    w.target = "specs/background.feature".to_string();
    w.write(
        &w.target.clone(),
        "Feature: Fixture\n\n  Background:\n    Given one precondition\n    Given another precondition\n\n  Scenario: Conforming body\n    Given a thing\n    When it acts\n    Then it is checked\n",
    );
}

#[given("a feature file with a Scenario Outline whose Examples table has many rows")]
fn given_feature_outline_examples(w: &mut GovernanceWorld) {
    w.target = "specs/outline.feature".to_string();
    w.write(
        &w.target.clone(),
        "Feature: Fixture\n\n  Scenario Outline: Outline body obeys the rule\n    Given a value <v>\n    When it is processed\n    Then it succeeds\n\n    Examples:\n      | v |\n      | 1 |\n      | 2 |\n      | 3 |\n",
    );
}

#[given("a feature file whose doc-strings and comments contain primary keyword words")]
fn given_feature_docstrings_comments(w: &mut GovernanceWorld) {
    w.target = "specs/docstring.feature".to_string();
    w.write(
        &w.target.clone(),
        "Feature: Fixture\n\n  Scenario: Docstring and comment heavy\n    Given a setup\n    When something runs with this payload\n      \"\"\"\n      When this line is data, not a step\n      Then neither is this one\n      \"\"\"\n    # Then this comment line is ignored\n    Then the result is checked\n",
    );
}

#[given("a directory of feature files that all obey the one-each keyword rule")]
fn given_conforming_feature_directory(w: &mut GovernanceWorld) {
    w.target = "specs".to_string();
    w.write(
        "specs/a.feature",
        "Feature: A\n\n  Scenario: Conforming chained scenario\n    Given a start\n    And another precondition\n    When the action runs\n    Then the outcome is checked\n",
    );
    w.write(
        "specs/b.feature",
        "Feature: B\n\n  Background:\n    Given one precondition\n    Given another precondition\n\n  Scenario: Conforming body\n    Given a thing\n    When it acts\n    Then it is checked\n",
    );
}

// ===========================================================================
// When steps
// ===========================================================================

#[when("the developer runs repo-governance vendor-audit on the file")]
#[when("the developer runs repo-governance vendor-audit on the directory")]
fn when_run_audit(w: &mut GovernanceWorld) {
    w.exec();
}

#[when("the developer runs repo-governance gherkin-keyword-cardinality on the file")]
#[when("the developer runs repo-governance gherkin-keyword-cardinality on the directory")]
fn when_run_cardinality_audit(w: &mut GovernanceWorld) {
    w.subcommand = vec!["specs", "gherkin-cardinality", "validate"];
    w.exec();
}

// ===========================================================================
// Then steps
// ===========================================================================

#[then("the command exits with a failure code")]
fn then_exit_fail(w: &mut GovernanceWorld) {
    assert_eq!(w.exit_code(), 1, "stdout: {}", w.stdout());
}

#[then("the command exits successfully")]
fn then_exit_ok(w: &mut GovernanceWorld) {
    assert_eq!(w.exit_code(), 0, "stdout: {}", w.stdout());
}

#[then("the output identifies the forbidden term and its location")]
fn then_identifies_term(w: &mut GovernanceWorld) {
    let out = w.stdout();
    assert!(out.contains("GOVERNANCE VENDOR AUDIT FAILED"), "got: {out}");
    assert!(out.contains("doc.md:"), "got: {out}");
}

#[then("the output reports zero findings")]
fn then_zero_findings(w: &mut GovernanceWorld) {
    let out = w.stdout();
    assert!(
        out.contains("GOVERNANCE VENDOR AUDIT PASSED: no violations found"),
        "got: {out}"
    );
}

#[then("the output names the offending file and scenario")]
fn then_names_offending_file_and_scenario(w: &mut GovernanceWorld) {
    let out = w.stdout();
    assert!(
        out.contains("GHERKIN KEYWORD CARDINALITY AUDIT FAILED"),
        "got: {out}"
    );
    assert!(out.contains("violating.feature:"), "got: {out}");
    assert!(out.contains("Double when offender"), "got: {out}");
}

#[then("the output reports zero cardinality findings")]
fn then_zero_cardinality_findings(w: &mut GovernanceWorld) {
    let out = w.stdout();
    assert!(
        out.contains(
            "GHERKIN KEYWORD CARDINALITY AUDIT PASSED: every scenario uses each primary \
             keyword at most once"
        ),
        "got: {out}"
    );
}

#[tokio::main]
async fn main() {
    GovernanceWorld::run(feature_dir()).await;
}

fn feature_dir() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .join("../../specs/apps/rhino/behavior/rhino-cli/gherkin/repo-governance")
        .canonicalize()
        .expect("feature dir resolvable")
}
