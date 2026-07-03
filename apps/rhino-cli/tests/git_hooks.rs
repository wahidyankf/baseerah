//! Cucumber-rs integration tests for the git pre-commit/pre-push hook chain's
//! staged-file markdown validators: `md links validate`, `md mermaid
//! validate`, and `md heading-hierarchy validate`.
//!
//! Wires the behavior-contract feature file at
//! `specs/apps/rhino/behavior/rhino-cli/gherkin/git/` to step definitions that
//! synthesize markdown fixtures inside a fresh git-rooted temp workspace,
//! stage them, and drive the compiled `rhino-cli` binary the same way
//! `.husky/pre-commit` (via `lint-staged`) and `.husky/pre-push` do.
//!
//! `md heading-hierarchy validate` has no `--staged-only` flag — lint-staged
//! invokes it with the staged file's path as a positional argument, which is
//! also how the real hook exercises it, so the `When` steps below mirror that
//! exactly. `md links validate` and `md mermaid validate` do support
//! `--staged-only`, matching `.husky/pre-push`'s own invocation (plus the same
//! `--exclude plans/done` the real pre-push command passes).
//!
//! The first scenario's `Then` steps ask about "the stderr output"; the
//! command's detailed per-link report is actually printed to stdout, with
//! only a one-line summary (`Error: found N broken links`) on stderr. Since a
//! developer watching a failed git hook in their terminal sees both streams
//! interleaved, these steps assert against the combined stdout+stderr text —
//! see [`GitHooksWorld::combined_output`].

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
struct GitHooksWorld {
    work: TempDir,
    /// Repo-relative path of the fixture file staged by the current
    /// scenario's `Given` step, asserted by `Then` steps.
    target_file: String,
    output: Option<Output>,
}

impl std::fmt::Debug for GitHooksWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GitHooksWorld")
            .field("target_file", &self.target_file)
            .finish_non_exhaustive()
    }
}

impl GitHooksWorld {
    fn new() -> Self {
        let work = TempDir::new().expect("temp workspace");
        init_git_repo(work.path());
        Self {
            work,
            target_file: String::new(),
            output: None,
        }
    }

    /// Writes `content` at repo-relative path `rel` inside the fixture
    /// workspace, creating parent directories as needed.
    fn write(&self, rel: &str, content: &str) {
        let p = self.work.path().join(rel);
        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent).expect("mk fixture dir");
        }
        std::fs::write(p, content).expect("write fixture");
    }

    /// Writes `content` at `rel` and `git add`s it, recording `rel` as the
    /// scenario's `target_file` for later `Then`-step assertions.
    fn write_and_stage(&mut self, rel: &str, content: &str) {
        self.target_file = rel.to_string();
        self.write(rel, content);
        self.git(&["add", rel]);
    }

    fn git(&self, args: &[&str]) {
        run_git(self.work.path(), args);
    }

    fn exec(&mut self, args: &[&str]) {
        let mut cmd = std::process::Command::new(cargo_bin("rhino-cli"));
        cmd.args(args)
            .arg("--no-color")
            .current_dir(self.work.path());
        self.output = Some(cmd.output().expect("run rhino-cli"));
    }

    fn stdout(&self) -> String {
        String::from_utf8_lossy(&self.output.as_ref().expect("ran").stdout).into_owned()
    }

    /// Concatenates stdout and stderr — see the module doc for why the first
    /// scenario's "stderr output" `Then` steps assert against this instead of
    /// stderr alone.
    fn combined_output(&self) -> String {
        let out = self.output.as_ref().expect("ran");
        format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        )
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

fn run_git(dir: &std::path::Path, args: &[&str]) {
    std::process::Command::new("git")
        .args(args)
        .current_dir(dir)
        .env("GIT_AUTHOR_NAME", "t")
        .env("GIT_AUTHOR_EMAIL", "t@t")
        .env("GIT_COMMITTER_NAME", "t")
        .env("GIT_COMMITTER_EMAIL", "t@t")
        .output()
        .expect("git command");
}

/// Initialises a minimal real git repo with one commit so `findGitRoot`
/// resolves here and staged-file queries succeed.
fn init_git_repo(dir: &std::path::Path) {
    run_git(dir, &["init", "-q"]);
    std::fs::write(dir.join("seed.txt"), "seed\n").expect("seed file");
    run_git(dir, &["add", "-A"]);
    run_git(dir, &["commit", "-q", "-m", "seed"]);
}

fn mermaid_block(body: &str) -> String {
    format!("# Diagram\n\n```mermaid\n{body}\n```\n")
}

// ===========================================================================
// Given steps
// ===========================================================================

#[given("staged markdown files contain a link to a non-existent target")]
fn given_links_broken_staged(w: &mut GitHooksWorld) {
    w.write_and_stage(
        "docs/index.md",
        "# Index\nSee [missing](./does-not-exist.md).\n",
    );
}

#[given(
    "a staged markdown file under docs containing a mermaid diagram with a label exceeding the maximum length"
)]
fn given_mermaid_label_too_long_staged(w: &mut GitHooksWorld) {
    w.write_and_stage(
        "docs/diagram.md",
        &mermaid_block(
            "flowchart TD\n    A[This label is definitely longer than thirty characters total]",
        ),
    );
}

#[given("a staged markdown file under docs containing two H1 headings")]
fn given_two_h1_staged(w: &mut GitHooksWorld) {
    w.write_and_stage(
        "docs/two-h1.md",
        "# First\n\ntext\n\n# Second\n\nmore text\n",
    );
}

#[given("a staged SKILL.md under .claude/skills with multiple H1 headings")]
fn given_skill_file_multiple_h1_staged(w: &mut GitHooksWorld) {
    w.write_and_stage(
        ".claude/skills/my-skill/SKILL.md",
        "# One\n\n# Two\n\n# Three\n",
    );
}

#[given("a staged markdown file under plans/done containing a broken internal link")]
fn given_plans_done_broken_link_staged(w: &mut GitHooksWorld) {
    w.write_and_stage(
        "plans/done/2024-01-01__old/notes.md",
        "# Notes\nSee [missing](./does-not-exist.md).\n",
    );
}

// ===========================================================================
// When steps
// ===========================================================================

#[when("the pre-commit hook runs md links validate on staged files")]
fn when_run_links_validate_staged(w: &mut GitHooksWorld) {
    // Mirrors the real `.husky/pre-push` invocation: `--staged-only` scopes
    // the scan to staged files, `--exclude plans/done` is always passed.
    w.exec(&[
        "md",
        "links",
        "validate",
        "--staged-only",
        "--exclude",
        "plans/done",
    ]);
}

#[when("the pre-commit hook runs md mermaid validate on the staged file")]
fn when_run_mermaid_validate_staged(w: &mut GitHooksWorld) {
    w.exec(&["md", "mermaid", "validate", "--staged-only"]);
}

#[when("the pre-commit hook runs md heading-hierarchy validate on the staged file")]
fn when_run_heading_hierarchy_validate_staged(w: &mut GitHooksWorld) {
    // `md heading-hierarchy validate` has no `--staged-only` flag; lint-staged
    // invokes it with the staged file's path as a positional argument.
    let file = w.target_file.clone();
    w.exec(&["md", "heading-hierarchy", "validate", &file]);
}

// ===========================================================================
// Then steps — shared exit-code assertions
// ===========================================================================

#[then("the command exits with a failure code")]
fn then_exit_fail(w: &mut GitHooksWorld) {
    assert_eq!(w.exit_code(), 1, "stdout: {}", w.stdout());
}

// ===========================================================================
// Then steps — md links validate (broken-link detection)
// ===========================================================================

#[then("the stderr output identifies the source file containing the broken link")]
fn then_identifies_source_file(w: &mut GitHooksWorld) {
    let out = w.combined_output();
    let file = w.target_file.clone();
    assert!(out.contains(&file), "got: {out}");
}

#[then("the stderr output identifies the line number of the broken link")]
fn then_identifies_line_number(w: &mut GitHooksWorld) {
    let out = w.combined_output();
    assert!(out.contains("Line 2:"), "got: {out}");
}

#[then("the stderr output identifies the broken link target")]
fn then_identifies_link_target(w: &mut GitHooksWorld) {
    let out = w.combined_output();
    assert!(out.contains("./does-not-exist.md"), "got: {out}");
}

// ===========================================================================
// Then steps — md mermaid validate
// ===========================================================================

#[then("the output indicates a mermaid violation was found")]
fn then_mermaid_violation_found(w: &mut GitHooksWorld) {
    let out = w.stdout();
    assert!(out.contains("[FAIL]"), "got: {out}");
    assert!(out.contains("Found 1 violation(s)"), "got: {out}");
}

// ===========================================================================
// Then steps — md heading-hierarchy validate
// ===========================================================================

#[then("the output indicates a heading hierarchy violation was found")]
fn then_heading_violation_found(w: &mut GitHooksWorld) {
    let out = w.stdout();
    assert!(
        out.contains("DOCS HEADING HIERARCHY VALIDATION FAILED"),
        "got: {out}"
    );
    assert!(out.contains("duplicate-h1"), "got: {out}");
}

#[then("the heading hierarchy step does not block the commit for that file")]
fn then_heading_step_does_not_block(w: &mut GitHooksWorld) {
    assert_eq!(w.exit_code(), 0, "stdout: {}", w.stdout());
}

// ===========================================================================
// Then steps — md links validate (exclusion honored)
// ===========================================================================

#[then("the link validation step does not report a broken link for the plans/done file")]
fn then_links_exclusion_honored(w: &mut GitHooksWorld) {
    assert_eq!(w.exit_code(), 0, "stdout: {}", w.stdout());
    let out = w.stdout();
    let file = w.target_file.clone();
    assert!(!out.contains(&file), "got: {out}");
}

#[tokio::main]
async fn main() {
    GitHooksWorld::cucumber()
        .fail_on_skipped()
        .run_and_exit(feature_dir())
        .await;
}

fn feature_dir() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .join("../../specs/apps/rhino/behavior/rhino-cli/gherkin/git")
        .canonicalize()
        .expect("feature dir resolvable")
}
