//! Cucumber-rs integration tests for the `specs behavior-coverage validate` command.
//!
//! Wires the behavior-contract feature file at
//! `specs/apps/rhino/behavior/rhino-cli/gherkin/spec-coverage/` to step definitions
//! that synthesize specs/app fixtures and drive the compiled `rhino-cli`
//! binary, asserting on its output and exit code.

// Test step-definition scaffolding: private World state and step fns are
// self-documenting via their #[given]/#[when]/#[then] gherkin strings.
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::doc_markdown)]

use std::path::PathBuf;
use std::process::Output;

use assert_cmd::cargo::cargo_bin;
use cucumber::{World as _, given, then, when};
use tempfile::TempDir;

#[derive(cucumber::World)]
#[world(init = Self::new)]
struct SpecWorld {
    work: TempDir,
    /// Whether the scenario expects shared-steps mode.
    shared_steps: bool,
    output: Option<Output>,
}

impl std::fmt::Debug for SpecWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpecWorld")
            .field("shared_steps", &self.shared_steps)
            .finish_non_exhaustive()
    }
}

impl SpecWorld {
    fn new() -> Self {
        let work = TempDir::new().expect("temp workspace");
        init_git_repo(work.path());
        std::fs::create_dir_all(work.path().join("specs")).expect("mk specs");
        std::fs::create_dir_all(work.path().join("app")).expect("mk app");
        Self {
            work,
            shared_steps: false,
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

    fn exec(&mut self, extra: &[&str]) {
        let mut args = vec!["specs", "behavior-coverage", "validate", "specs", "app"];
        args.extend_from_slice(extra);
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

/// Initialises a minimal real git repository at `dir` (no commits needed) so
/// `git rev-parse --show-toplevel` — which `find_root()` shells out to —
/// resolves successfully. A bare `.git` directory (not an initialized
/// repository) is insufficient for `git rev-parse` to succeed.
fn init_git_repo(dir: &std::path::Path) {
    let out = std::process::Command::new("git")
        .args(["init", "-q"])
        .current_dir(dir)
        .output()
        .expect("run git init");
    assert!(out.status.success(), "git init failed: {out:?}");
}

// --- Given steps ---

#[given("a specs directory where every feature file has a corresponding test file")]
fn given_all_covered(w: &mut SpecWorld) {
    w.write(
        "specs/user-login.feature",
        "Feature: Login\nScenario: Logs in\n  Given a user\n",
    );
    // A TS test file matching the stem, with the scenario title and step text.
    w.write(
        "app/user-login.test.ts",
        "Scenario(\"Logs in\", () => {});\nGiven(\"a user\", () => {});\n",
    );
}

#[given("a specs directory containing a feature file with no corresponding test file")]
fn given_missing_test(w: &mut SpecWorld) {
    w.write(
        "specs/orphan-spec.feature",
        "Feature: Orphan\nScenario: Nope\n  Given nothing\n",
    );
    // App dir has unrelated files only.
    w.write("app/unrelated.ts", "const x = 1;\n");
}

#[given("a feature file with a scenario whose title does not appear in any test file")]
fn given_missing_scenario(w: &mut SpecWorld) {
    w.write(
        "specs/user-login.feature",
        "Feature: Login\nScenario: Missing Title\n  Given a user\n",
    );
    // Matching test file exists, with the step but a DIFFERENT scenario title.
    w.write(
        "app/user-login.test.ts",
        "Scenario(\"Other Title\", () => {});\nGiven(\"a user\", () => {});\n",
    );
}

#[given("a feature file with a step text that does not appear in any test file")]
fn given_missing_step(w: &mut SpecWorld) {
    w.write(
        "specs/user-login.feature",
        "Feature: Login\nScenario: Logs in\n  Given an unimplemented step\n",
    );
    // Matching test file with the scenario title but NOT the step.
    w.write(
        "app/user-login.test.ts",
        "Scenario(\"Logs in\", () => {});\nGiven(\"some other step\", () => {});\n",
    );
}

#[given("feature files with steps implemented in shared step files")]
fn given_shared_steps(w: &mut SpecWorld) {
    w.shared_steps = true;
    w.write(
        "specs/a.feature",
        "Feature: A\nScenario: One\n  Given a shared step\n",
    );
    w.write(
        "specs/b.feature",
        "Feature: B\nScenario: Two\n  Given a shared step\n",
    );
    // Shared step file (no per-feature matching needed in shared-steps mode).
    w.write(
        "app/shared.steps.ts",
        "Given(\"a shared step\", () => {});\n",
    );
}

#[given("feature files with test implementations in multiple languages")]
fn given_multilang(w: &mut SpecWorld) {
    // Three features, each matched by a DIFFERENT language's test-file naming
    // convention (Go `_test.go`, Python `test_`, TS `.test.ts`). Each test file
    // carries the scenario title (via the appropriate convention) and the step.
    w.write(
        "specs/go-feature.feature",
        "Feature: Go\nScenario: G\n  Given a go step\n",
    );
    w.write(
        "app/go_feature_test.go",
        "// Scenario: G\nfunc x(sc *godog.ScenarioContext) {\n  sc.Step(`^a go step$`, fn)\n}\n",
    );

    w.write(
        "specs/py-feature.feature",
        "Feature: Py\nScenario: P\n  Given a python step\n",
    );
    w.write(
        "app/test_py_feature.py",
        "@scenario(\"py-feature.feature\", \"P\")\ndef t():\n    pass\n@given(\"a python step\")\ndef s():\n    pass\n",
    );

    w.write(
        "specs/ts-feature.feature",
        "Feature: Ts\nScenario: T\n  Given a ts step\n",
    );
    w.write(
        "app/ts-feature.test.ts",
        "Scenario(\"T\", () => {});\nGiven(\"a ts step\", () => {});\n",
    );
}

// --- When steps ---

#[when("the developer runs spec-coverage validate on the specs and app directories")]
fn when_run(w: &mut SpecWorld) {
    w.exec(&[]);
}

#[when("the developer runs spec-coverage validate with shared-steps flag")]
fn when_run_shared(w: &mut SpecWorld) {
    w.exec(&["--shared-steps"]);
}

// --- Then steps ---

#[then("the command exits successfully")]
fn then_exit_ok(w: &mut SpecWorld) {
    assert_eq!(w.exit_code(), 0, "stdout: {}", w.stdout());
}

#[then("the command exits with a failure code")]
fn then_exit_fail(w: &mut SpecWorld) {
    assert_eq!(w.exit_code(), 1, "stdout: {}", w.stdout());
}

#[then("the output reports all specs as covered")]
fn then_all_covered(w: &mut SpecWorld) {
    assert!(
        w.stdout().contains("Spec coverage valid!"),
        "got: {}",
        w.stdout()
    );
}

#[then("the output identifies the feature file as an uncovered spec")]
fn then_uncovered_spec(w: &mut SpecWorld) {
    let out = w.stdout();
    assert!(out.contains("Missing test files"), "got: {out}");
    assert!(out.contains("orphan-spec.feature"), "got: {out}");
}

#[then("the output identifies the scenario as an unimplemented scenario")]
fn then_unimpl_scenario(w: &mut SpecWorld) {
    let out = w.stdout();
    assert!(out.contains("Missing scenarios"), "got: {out}");
    assert!(out.contains("Missing Title"), "got: {out}");
}

#[then("the output identifies the step as an undefined step")]
fn then_undefined_step(w: &mut SpecWorld) {
    let out = w.stdout();
    assert!(out.contains("Missing steps"), "got: {out}");
    assert!(out.contains("an unimplemented step"), "got: {out}");
}

#[then("the command validates steps across all source files without file matching")]
fn then_shared_no_matching(w: &mut SpecWorld) {
    // Shared-steps mode reports no file gaps; both features' shared step is covered.
    assert_eq!(w.exit_code(), 0, "stdout: {}", w.stdout());
    assert!(
        w.stdout().contains("Spec coverage valid!"),
        "got: {}",
        w.stdout()
    );
}

#[then("test files are matched using language-specific conventions")]
fn then_multilang_matched(w: &mut SpecWorld) {
    // All multi-language steps are recognized → no step gaps → success.
    assert_eq!(w.exit_code(), 0, "stdout: {}", w.stdout());
}

#[tokio::main]
async fn main() {
    let features = repo_feature_dir();
    SpecWorld::cucumber()
        .fail_on_skipped()
        .run_and_exit(features)
        .await;
}

fn repo_feature_dir() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .join("../../specs/apps/rhino/behavior/rhino-cli/gherkin/spec-coverage")
        .canonicalize()
        .expect("feature dir resolvable")
}
