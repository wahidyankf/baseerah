//! Cucumber-rs integration tests for the `repo-governance workflows naming validate` command.
//!
//! Wires the behavior-contract feature file at
//! `specs/apps/rhino/behavior/rhino-cli/gherkin/workflows/` to step definitions that
//! synthesize a `repo-governance/workflows/` tree inside a fresh git-rooted
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
struct WorkflowsWorld {
    work: TempDir,
    output: Option<Output>,
}

impl std::fmt::Debug for WorkflowsWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WorkflowsWorld").finish_non_exhaustive()
    }
}

impl WorkflowsWorld {
    fn new() -> Self {
        let work = TempDir::new().expect("temp workspace");
        init_git_repo(work.path());
        Self { work, output: None }
    }

    fn write(&self, rel: &str, content: &str) {
        let p = self.work.path().join(rel);
        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent).expect("mk fixture dir");
        }
        std::fs::write(p, content).expect("write fixture");
    }

    /// Writes a workflow file under `repo-governance/workflows/` with valid
    /// frontmatter whose `name:` matches the filename (sans .md).
    fn write_workflow(&self, file: &str) {
        let name = file.strip_suffix(".md").unwrap_or(file);
        self.write(
            &format!("repo-governance/workflows/{file}"),
            &format!("---\nname: {name}\n---\n# Body\n"),
        );
    }

    fn exec(&mut self) {
        let out = std::process::Command::new(cargo_bin("rhino-cli"))
            .args([
                "repo-governance",
                "workflows",
                "naming",
                "validate",
                "--no-color",
            ])
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

#[given("a repository where every workflow filename ends with an allowed type suffix")]
fn given_all_valid(w: &mut WorkflowsWorld) {
    w.write_workflow("plan-quality-gate.md");
    w.write_workflow("plan-execution.md");
    w.write_workflow("infra-setup.md");
    // README.md is exempt even though it lacks a type suffix.
    w.write("repo-governance/workflows/README.md", "# Workflows\n");
}

#[given("a repository with one workflow whose filename ends in an unknown suffix")]
fn given_bad_suffix(w: &mut WorkflowsWorld) {
    w.write_workflow("plan-quality-gate.md");
    // `plan-widget` has no allowed type suffix.
    w.write(
        "repo-governance/workflows/plan-widget.md",
        "---\nname: plan-widget\n---\n# Body\n",
    );
}

#[given("a repository with a workflow file whose frontmatter name differs from its filename")]
fn given_frontmatter_mismatch(w: &mut WorkflowsWorld) {
    w.write_workflow("plan-quality-gate.md");
    // Filename plan-setup.md but frontmatter says wrong-name.
    w.write(
        "repo-governance/workflows/plan-setup.md",
        "---\nname: wrong-name\n---\n# Body\n",
    );
}

#[given(
    "a repository with a file under repo-governance/workflows/meta/ whose name does not follow the type-suffix rule"
)]
fn given_meta_exempt(w: &mut WorkflowsWorld) {
    w.write_workflow("plan-quality-gate.md");
    // A non-conforming name under meta/ is exempt.
    w.write(
        "repo-governance/workflows/meta/reference-doc.md",
        "---\nname: anything\n---\n# Reference\n",
    );
}

// ===========================================================================
// When steps
// ===========================================================================

#[when("the developer runs repo-governance workflows naming validate")]
fn when_run(w: &mut WorkflowsWorld) {
    w.exec();
}

// ===========================================================================
// Then steps
// ===========================================================================

#[then("the command exits successfully")]
fn then_exit_ok(w: &mut WorkflowsWorld) {
    assert_eq!(w.exit_code(), 0, "stdout: {}", w.stdout());
}

#[then("the command exits with a failure code")]
fn then_exit_fail(w: &mut WorkflowsWorld) {
    assert_eq!(w.exit_code(), 1, "stdout: {}", w.stdout());
}

#[then("the output reports zero naming violations")]
fn then_zero(w: &mut WorkflowsWorld) {
    let out = w.stdout();
    assert!(
        out.contains("VALIDATION PASSED (0 violations)"),
        "got: {out}"
    );
}

#[then("the output identifies the offending workflow file and its unknown suffix")]
fn then_bad_suffix(w: &mut WorkflowsWorld) {
    let out = w.stdout();
    assert!(out.contains("plan-widget"), "got: {out}");
    assert!(out.contains("type-suffix"), "got: {out}");
}

#[then("the output identifies the frontmatter mismatch")]
fn then_frontmatter_mismatch(w: &mut WorkflowsWorld) {
    let out = w.stdout();
    assert!(out.contains("frontmatter-mismatch"), "got: {out}");
    assert!(out.contains("wrong-name"), "got: {out}");
}

#[tokio::main]
async fn main() {
    WorkflowsWorld::cucumber()
        .fail_on_skipped()
        .run_and_exit(feature_dir())
        .await;
}

fn feature_dir() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .join("../../specs/apps/rhino/behavior/rhino-cli/gherkin/workflows")
        .canonicalize()
        .expect("feature dir resolvable")
}
