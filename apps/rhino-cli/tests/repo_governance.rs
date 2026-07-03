//! Cucumber-rs integration tests for the `repo-governance` governance-audit
//! commands: `repo-governance vendor validate`, `repo-governance
//! layer-coherence validate`, `repo-governance traceability validate`, and
//! `repo-governance audit`.
//!
//! Wires the behavior-contract feature files at
//! `specs/apps/rhino/behavior/rhino-cli/gherkin/repo-governance/` to step
//! definitions that synthesize markdown fixtures inside a fresh git-rooted
//! temp workspace and drive the compiled `rhino-cli` binary, asserting on its
//! output and exit code.

// Test step-definition scaffolding: private World state and step fns are
// self-documenting via their #[given]/#[when]/#[then] gherkin strings.
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::doc_markdown)]

use std::fmt::Write as _;
use std::path::PathBuf;
use std::process::Output;

use assert_cmd::cargo::cargo_bin;
use cucumber::{World as _, given, then, when};
use serde_json::Value;
use tempfile::TempDir;

/// Shared scenario state. Each scenario gets a fresh git-rooted temp workspace
/// so the binary's `findGitRoot` resolves inside the fixture.
#[derive(cucumber::World)]
#[world(init = Self::new)]
struct GovernanceWorld {
    work: TempDir,
    /// Repo-relative path of the fixture file or directory to audit. Left
    /// empty for subcommands that take no positional path (layer-coherence,
    /// traceability, audit).
    target: String,
    /// Full CLI subcommand path under test (e.g.
    /// `["repo-governance", "vendor", "validate"]`), excluding the trailing
    /// target path and `--no-color` flag.
    subcommand: Vec<&'static str>,
    /// Extra CLI flags (e.g. `-o json`, `--include-category vendor-audit`),
    /// appended after the target path and before `--no-color`.
    extra_args: Vec<String>,
    /// Extra environment variables set on the spawned `rhino-cli` process.
    envs: Vec<(&'static str, String)>,
    output: Option<Output>,
    /// Captured stdout from each invocation of a repeated-invocation step
    /// (e.g. the byte-determinism scenario).
    captured_runs: Vec<String>,
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
            extra_args: Vec::new(),
            envs: Vec::new(),
            output: None,
            captured_runs: Vec::new(),
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
        let mut args: Vec<String> = self.subcommand.iter().map(|s| (*s).to_string()).collect();
        if !self.target.is_empty() {
            args.push(self.target.clone());
        }
        args.extend(self.extra_args.iter().cloned());
        args.push("--no-color".to_string());

        let mut cmd = std::process::Command::new(cargo_bin("rhino-cli"));
        cmd.args(&args).current_dir(self.work.path());
        for (key, value) in &self.envs {
            cmd.env(*key, value);
        }
        let out = cmd.output().expect("run rhino-cli");
        self.output = Some(out);
    }

    fn stdout(&self) -> String {
        String::from_utf8_lossy(&self.output.as_ref().expect("ran").stdout).into_owned()
    }

    /// Parses the last captured stdout as JSON.
    ///
    /// # Panics
    ///
    /// Panics when no run has been captured yet or stdout is not valid JSON.
    fn stdout_json(&self) -> Value {
        serde_json::from_str(&self.stdout()).expect("stdout is valid JSON")
    }

    fn exit_code(&self) -> i32 {
        self.output
            .as_ref()
            .expect("ran")
            .status
            .code()
            .unwrap_or(-1)
    }

    /// Writes identical layer declarations to both governance layer-coherence
    /// documents (`repository-governance-architecture.md` and `README.md`) so
    /// `repo-governance layer-coherence validate` reports zero findings for
    /// them, leaving only whatever other fixtures the calling step wrote.
    fn write_matching_layer_docs(&self, layers: &[(u32, &str)]) {
        let doc = layer_doc(layers);
        self.write(
            "repo-governance/repository-governance-architecture.md",
            &doc,
        );
        self.write("repo-governance/README.md", &doc);
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

/// Builds a Markdown fragment declaring each `(number, name)` pair using the
/// bold `**Layer N: Name**` syntax the layer-coherence audit recognizes.
fn layer_doc(layers: &[(u32, &str)]) -> String {
    let mut s = String::new();
    for (n, name) in layers {
        let _ = writeln!(s, "**Layer {n}: {name}**");
    }
    s
}

/// Returns `value` as a JSON array, panicking with `field` (used to name the
/// expected field in the failure message) when it is not one.
fn json_array<'a>(value: &'a Value, field: &str) -> &'a Vec<Value> {
    let msg = format!("{field} is not a JSON array: {value}");
    value.as_array().expect(&msg)
}

/// Returns the `vendor-audit` category object from a `repo-governance audit`
/// JSON envelope.
fn vendor_audit_category(json: &Value) -> &Value {
    json_array(&json["result"]["categories"], "categories")
        .iter()
        .find(|c| c["name"].as_str() == Some("vendor-audit"))
        .expect("vendor-audit category present in result")
}

/// Returns the `file` field of every finding in `category`, with backslashes
/// normalized to forward slashes for cross-platform comparison.
fn finding_files(category: &Value) -> Vec<String> {
    json_array(&category["findings"], "findings")
        .iter()
        .map(|f| {
            f["file"]
                .as_str()
                .expect("finding file is a string")
                .replace('\\', "/")
        })
        .collect()
}

// ===========================================================================
// Given steps — repo-governance vendor validate
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

// ===========================================================================
// Given steps — repo-governance layer-coherence validate
// ===========================================================================

#[given("a repository where both governance docs list layers 0 through 5 with identical names")]
fn given_layers_identical(w: &mut GovernanceWorld) {
    w.write_matching_layer_docs(&[
        (0, "Vision"),
        (1, "Principles"),
        (2, "Conventions"),
        (3, "Development"),
        (4, "Agents"),
        (5, "Workflows"),
    ]);
}

#[given("a repository where the governance docs list layers 0, 1, and 3 with no layer 2")]
fn given_layers_gap(w: &mut GovernanceWorld) {
    w.write_matching_layer_docs(&[(0, "Vision"), (1, "Principles"), (3, "Development")]);
}

#[given(
    "a repository where the two governance docs assign different names to the same layer number"
)]
fn given_layers_name_mismatch(w: &mut GovernanceWorld) {
    w.write(
        "repo-governance/repository-governance-architecture.md",
        "**Layer 0: Vision**\n",
    );
    w.write("repo-governance/README.md", "**Layer 0: Mission**\n");
}

// ===========================================================================
// Given steps — repo-governance traceability validate
// ===========================================================================

#[given("a repository where every governance document carries the required traceability sections")]
fn given_traceability_clean(w: &mut GovernanceWorld) {
    w.write(
        "repo-governance/principles/p.md",
        "# P\n\n## Vision Supported\n\nBody.\n",
    );
    w.write(
        "repo-governance/conventions/c.md",
        "# C\n\n## Principles Implemented/Respected\n\nBody.\n",
    );
    w.write(
        "repo-governance/development/d.md",
        "# D\n\n## Principles Implemented/Respected\n\n## Conventions Implemented/Respected\n\nBody.\n",
    );
    w.write(
        "repo-governance/workflows/w.md",
        "# W\n\nSee `.claude/agents/foo-bar.md`.\n",
    );
}

#[given("a repository with a principle file that is missing the \"## Vision Supported\" heading")]
fn given_principle_missing_vision(w: &mut GovernanceWorld) {
    w.write(
        "repo-governance/principles/p.md",
        "# P\n\nNo heading here.\n",
    );
}

#[given(
    "a repository with a convention file that is missing the \"## Principles Implemented/Respected\" heading"
)]
fn given_convention_missing_principles(w: &mut GovernanceWorld) {
    w.write(
        "repo-governance/conventions/c.md",
        "# C\n\nNo heading here.\n",
    );
}

#[given(
    "a repository with a development file that is missing the \"## Conventions Implemented/Respected\" heading"
)]
fn given_development_missing_conventions(w: &mut GovernanceWorld) {
    w.write(
        "repo-governance/development/d.md",
        "# D\n\n## Principles Implemented/Respected\n\nBody.\n",
    );
}

#[given("a repository with a workflow file that contains no reference to any .claude/agents/ file")]
fn given_workflow_missing_agent_ref(w: &mut GovernanceWorld) {
    w.write("repo-governance/workflows/w.md", "# W\n\nno agent here.\n");
}

// ===========================================================================
// Given steps — repo-governance audit
// ===========================================================================

#[given("a repository where every deterministic governance category reports zero findings")]
fn given_audit_all_clean(w: &mut GovernanceWorld) {
    w.write_matching_layer_docs(&[(0, "Vision")]);
}

#[given(
    "a repository with forbidden vendor terms in repo-governance prose and also in out-of-scope paths such as build caches, app source, and worktrees"
)]
fn given_audit_vendor_scope(w: &mut GovernanceWorld) {
    w.write(
        "repo-governance/conventions/foo.md",
        "We use Claude Code internally.\n",
    );
    w.write("AGENTS.md", "Edited with Cursor today.\n");
    w.write("CLAUDE.md", "Powered by Anthropic models.\n");
    w.write(".nx/cache/x.md", "Built on OpenCode.\n");
    w.write("apps/web/y.md", "Built on OpenCode.\n");
    w.write("worktrees/wt/z.md", "Built on OpenCode.\n");
}

#[given(
    "a repository where two deterministic governance categories report findings and the rest pass"
)]
fn given_audit_mixed(w: &mut GovernanceWorld) {
    // layer-coherence passes: matching single-layer docs.
    w.write_matching_layer_docs(&[(0, "Vision")]);
    // vendor-audit fails: one forbidden term.
    w.write("repo-governance/doc.md", "We use Claude Code daily.\n");
    // traceability-audit fails: a principle missing its required heading.
    w.write("repo-governance/principles/p.md", "# P\n\nNo heading.\n");
}

#[given("a repository where deterministic governance categories return a fixed finding set")]
fn given_audit_fixed_set(w: &mut GovernanceWorld) {
    w.write_matching_layer_docs(&[(0, "Vision")]);
    w.write("repo-governance/doc.md", "We use Claude Code daily.\n");
}

#[given("a repository where a finding key matches a known-false-positives entry")]
fn given_audit_false_positive(w: &mut GovernanceWorld) {
    w.write_matching_layer_docs(&[(0, "Vision")]);
    w.write("repo-governance/doc.md", "We use Claude Code daily.\n");

    // Prime a run to learn the finding's exact key, then register it as a
    // known false positive so the measured run in the `When` step suppresses
    // it instead of counting it toward total_findings.
    w.subcommand = vec!["repo-governance", "audit"];
    w.extra_args = vec!["-o".to_string(), "json".to_string()];
    w.exec();
    let json = w.stdout_json();
    let key = vendor_audit_category(&json)["findings"][0]["key"]
        .as_str()
        .expect("priming run reports a vendor-audit finding key")
        .to_string();
    w.write(
        "generated-reports/.known-false-positives.md",
        &format!("- `{key}`\n"),
    );
}

#[given("a repository where deterministic governance categories return any finding set")]
fn given_audit_any_set(w: &mut GovernanceWorld) {
    w.write("repo-governance/doc.md", "We use Claude Code daily.\n");
}

// ===========================================================================
// When steps
// ===========================================================================

#[when("the developer runs repo-governance vendor validate on the file")]
#[when("the developer runs repo-governance vendor validate on the directory")]
fn when_run_vendor_validate(w: &mut GovernanceWorld) {
    w.exec();
}

#[when("the developer runs repo-governance layer-coherence validate")]
fn when_run_layer_coherence(w: &mut GovernanceWorld) {
    w.subcommand = vec!["repo-governance", "layer-coherence", "validate"];
    w.exec();
}

#[when("the developer runs repo-governance traceability validate")]
fn when_run_traceability(w: &mut GovernanceWorld) {
    w.subcommand = vec!["repo-governance", "traceability", "validate"];
    w.exec();
}

#[when("the developer runs repo-governance audit")]
fn when_run_governance_audit(w: &mut GovernanceWorld) {
    w.subcommand = vec!["repo-governance", "audit"];
    w.extra_args = vec!["-o".to_string(), "json".to_string()];
    w.exec();
}

#[when("the developer runs repo-governance audit ten consecutive times with a fixed clock")]
fn when_run_governance_audit_ten_times(w: &mut GovernanceWorld) {
    w.subcommand = vec!["repo-governance", "audit"];
    w.extra_args = vec!["-o".to_string(), "json".to_string()];
    w.envs = vec![("RHINO_AUDIT_NOW", "2026-01-01T00:00:00Z".to_string())];
    let mut runs = Vec::with_capacity(10);
    for _ in 0..10 {
        w.exec();
        runs.push(w.stdout());
    }
    w.captured_runs = runs;
}

#[when("the developer runs repo-governance audit with include-category limited to one category")]
fn when_run_governance_audit_include_category(w: &mut GovernanceWorld) {
    w.subcommand = vec!["repo-governance", "audit"];
    w.extra_args = vec![
        "-o".to_string(),
        "json".to_string(),
        "--include-category".to_string(),
        "layer-coherence".to_string(),
    ];
    w.exec();
}

// ===========================================================================
// Then steps — shared exit-code assertions
// ===========================================================================

#[then("the command exits with a failure code")]
fn then_exit_fail(w: &mut GovernanceWorld) {
    assert_eq!(w.exit_code(), 1, "stdout: {}", w.stdout());
}

#[then("the command exits successfully")]
fn then_exit_ok(w: &mut GovernanceWorld) {
    assert_eq!(w.exit_code(), 0, "stdout: {}", w.stdout());
}

// ===========================================================================
// Then steps — repo-governance vendor validate
// ===========================================================================

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

// ===========================================================================
// Then steps — repo-governance layer-coherence validate
// ===========================================================================

#[then("the layer-coherence output reports zero findings")]
fn then_layer_zero_findings(w: &mut GovernanceWorld) {
    let out = w.stdout();
    assert!(
        out.contains("LAYER COHERENCE AUDIT PASSED: zero findings"),
        "got: {out}"
    );
}

#[then("the layer-coherence output identifies the numbering gap")]
fn then_layer_numbering_gap(w: &mut GovernanceWorld) {
    let out = w.stdout();
    assert!(out.contains("numbering-gap"), "got: {out}");
    assert!(out.contains("Layer 2"), "got: {out}");
}

#[then("the layer-coherence output identifies the layer name disagreement")]
fn then_layer_name_disagreement(w: &mut GovernanceWorld) {
    let out = w.stdout();
    assert!(out.contains("cross-file-name-mismatch"), "got: {out}");
    assert!(
        out.contains("Vision") && out.contains("Mission"),
        "got: {out}"
    );
}

// ===========================================================================
// Then steps — repo-governance traceability validate
// ===========================================================================

#[then("the traceability output reports zero findings")]
fn then_traceability_zero(w: &mut GovernanceWorld) {
    let out = w.stdout();
    assert!(
        out.contains("TRACEABILITY AUDIT PASSED: zero findings"),
        "got: {out}"
    );
}

#[then("the traceability output identifies the missing Vision Supported section")]
fn then_traceability_missing_vision(w: &mut GovernanceWorld) {
    let out = w.stdout();
    assert!(out.contains("missing-vision-supported"), "got: {out}");
}

#[then("the traceability output identifies the missing Principles Implemented section")]
fn then_traceability_missing_principles(w: &mut GovernanceWorld) {
    let out = w.stdout();
    assert!(out.contains("missing-principles-implemented"), "got: {out}");
}

#[then("the traceability output identifies the missing Conventions Implemented section")]
fn then_traceability_missing_conventions(w: &mut GovernanceWorld) {
    let out = w.stdout();
    assert!(
        out.contains("missing-conventions-implemented"),
        "got: {out}"
    );
}

#[then("the traceability output identifies the missing agent reference")]
fn then_traceability_missing_agent_ref(w: &mut GovernanceWorld) {
    let out = w.stdout();
    assert!(out.contains("missing-agent-reference"), "got: {out}");
}

// ===========================================================================
// Then steps — repo-governance audit
// ===========================================================================

#[then("the output reports total_findings equal to zero across all categories")]
fn then_audit_zero_total(w: &mut GovernanceWorld) {
    let json = w.stdout_json();
    assert_eq!(
        json["result"]["total_findings"].as_u64(),
        Some(0),
        "got: {json}"
    );
}

#[then(
    "the vendor-audit category reports findings only from repo-governance, AGENTS.md, and CLAUDE.md"
)]
fn then_audit_vendor_scope(w: &mut GovernanceWorld) {
    let json = w.stdout_json();
    let files = finding_files(vendor_audit_category(&json));
    assert_eq!(files.len(), 3, "got: {files:?}");
    assert!(
        files
            .iter()
            .any(|f| f.ends_with("repo-governance/conventions/foo.md")),
        "got: {files:?}"
    );
    assert!(
        files.iter().any(|f| f.ends_with("/AGENTS.md")),
        "got: {files:?}"
    );
    assert!(
        files.iter().any(|f| f.ends_with("/CLAUDE.md")),
        "got: {files:?}"
    );
}

#[then(
    "forbidden vendor terms in build caches, app source, and worktrees do not appear in the result"
)]
fn then_audit_vendor_scope_excludes(w: &mut GovernanceWorld) {
    let json = w.stdout_json();
    let files = finding_files(vendor_audit_category(&json));
    assert!(!files.iter().any(|f| f.contains("/.nx/")), "got: {files:?}");
    assert!(
        !files.iter().any(|f| f.contains("/apps/")),
        "got: {files:?}"
    );
    assert!(
        !files.iter().any(|f| f.contains("/worktrees/")),
        "got: {files:?}"
    );
}

#[then("the output reports total_findings equal to the sum of category findings")]
fn then_audit_sum_total(w: &mut GovernanceWorld) {
    let json = w.stdout_json();
    let categories = json_array(&json["result"]["categories"], "categories");
    let sum: u64 = categories
        .iter()
        .map(|c| json_array(&c["findings"], "findings").len() as u64)
        .sum();
    let total = json["result"]["total_findings"]
        .as_u64()
        .expect("total_findings is a number");
    assert_eq!(total, sum, "got: {json}");
    let failing = categories
        .iter()
        .filter(|c| c["passed"].as_bool() == Some(false))
        .count();
    assert_eq!(
        failing, 2,
        "expected exactly two failing categories, got: {json}"
    );
}

#[then("every run produces byte-identical JSON output")]
fn then_audit_byte_identical(w: &mut GovernanceWorld) {
    assert_eq!(w.captured_runs.len(), 10, "expected 10 captured runs");
    let first = &w.captured_runs[0];
    for (i, run) in w.captured_runs.iter().enumerate() {
        assert_eq!(run, first, "run {i} diverged from run 0");
    }
}

#[then("the matching finding appears under skipped_false_positives")]
fn then_audit_skipped_false_positive(w: &mut GovernanceWorld) {
    let json = w.stdout_json();
    let skipped = json_array(
        &json["result"]["skipped_false_positives"],
        "skipped_false_positives",
    );
    assert_eq!(skipped.len(), 1, "got: {json}");
}

#[then("the matching finding does not count toward total_findings")]
fn then_audit_false_positive_excluded_from_total(w: &mut GovernanceWorld) {
    let json = w.stdout_json();
    assert_eq!(
        json["result"]["total_findings"].as_u64(),
        Some(0),
        "got: {json}"
    );
}

#[then("only the listed category appears in the result categories list")]
fn then_audit_include_category_filter(w: &mut GovernanceWorld) {
    let json = w.stdout_json();
    let categories = json_array(&json["result"]["categories"], "categories");
    assert_eq!(categories.len(), 1, "got: {json}");
    assert_eq!(
        categories[0]["name"].as_str(),
        Some("layer-coherence"),
        "got: {json}"
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
