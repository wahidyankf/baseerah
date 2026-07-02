//! Cucumber-rs integration tests for the `harness bindings generate`,
//! `harness claude validate`, `harness sync validate`, `harness naming validate`,
//! and `harness bindings validate` commands (the agent/binding-machinery domain;
//! feature file names and Gherkin step text still say "agents" for historical
//! reasons — the underlying CLI subcommands live under the `harness` noun today).
//!
//! Wires the behavior-contract feature files at
//! `specs/apps/rhino/behavior/rhino-cli/gherkin/agents/` to step definitions that
//! synthesize `.claude/` and `.opencode/` fixtures inside a fresh git-rooted
//! temp workspace and drive the compiled `rhino-cli` binary, asserting on
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
use tempfile::TempDir;

/// Shared scenario state. Each scenario gets a fresh git-rooted temp workspace
/// so the binary's `findGitRoot` resolves inside the fixture.
#[derive(cucumber::World)]
#[world(init = Self::new)]
struct AgentsWorld {
    work: TempDir,
    /// Extra CLI args (flags) for the next exec.
    extra_args: Vec<String>,
    output: Option<Output>,
}

impl std::fmt::Debug for AgentsWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AgentsWorld")
            .field("extra_args", &self.extra_args)
            .finish_non_exhaustive()
    }
}

impl AgentsWorld {
    fn new() -> Self {
        let work = TempDir::new().expect("temp workspace");
        init_git_repo(work.path());
        Self {
            work,
            extra_args: Vec::new(),
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

    /// Writes a valid Claude agent file under `.claude/agents/`.
    fn write_claude_agent(&self, name: &str, model: &str, skills: &[&str]) {
        let mut content = format!(
            "---\nname: {name}\ndescription: Agent {name}.\ntools: Read, Write\nmodel: {model}\ncolor: blue\n"
        );
        if !skills.is_empty() {
            content.push_str("skills:\n");
            for s in skills {
                let _ = writeln!(content, "  - {s}");
            }
        }
        content.push_str("---\n# Body\n");
        self.write(&format!(".claude/agents/{name}.md"), &content);
    }

    /// Writes a valid Claude skill directory.
    fn write_claude_skill(&self, name: &str) {
        self.write(
            &format!(".claude/skills/{name}/SKILL.md"),
            &format!("---\nname: {name}\ndescription: Skill {name}.\n---\n# Skill body\n"),
        );
    }

    fn bin() -> PathBuf {
        cargo_bin("rhino-cli")
    }

    fn exec(&mut self, base: &[&str]) {
        let mut args: Vec<String> = base.iter().map(|s| (*s).to_string()).collect();
        args.extend(self.extra_args.iter().cloned());
        args.push("--no-color".to_string());
        let out = std::process::Command::new(Self::bin())
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

/// Initialises a minimal real git repo so `findGitRoot` resolves here.
fn init_git_repo(dir: &std::path::Path) {
    run_git(dir, &["init", "-q"]);
}

// ===========================================================================
// agents sync — Given steps
// ===========================================================================

#[given("a .claude/ directory with valid agents and skills")]
#[given("a .claude/ directory with agents and skills to convert")]
#[given("a .claude/ directory with both agents and skills")]
fn given_claude_agents_and_skills(w: &mut AgentsWorld) {
    w.write_claude_skill("my-skill");
    w.write_claude_agent("foo-maker", "sonnet", &["my-skill"]);
    w.write_claude_agent("bar-checker", "haiku", &[]);
}

#[given(r#"a .claude/ agent configured with the "sonnet" model"#)]
fn given_claude_sonnet_agent(w: &mut AgentsWorld) {
    w.write_claude_agent("foo-maker", "sonnet", &[]);
}

// ===========================================================================
// agents sync — When steps
// ===========================================================================

#[when("the developer runs agents sync")]
fn when_sync(w: &mut AgentsWorld) {
    w.exec(&["harness", "bindings", "generate", "--harness", "opencode"]);
}

#[when("the developer runs agents sync with the --dry-run flag")]
fn when_sync_dry_run(w: &mut AgentsWorld) {
    w.exec(&[
        "harness",
        "bindings",
        "generate",
        "--harness",
        "opencode",
        "--dry-run",
    ]);
}

#[when("the developer runs agents sync with the --agents-only flag")]
fn when_sync_agents_only(w: &mut AgentsWorld) {
    // `harness bindings generate` has no per-step `--agents-only` flag: the
    // OpenCode sync step never copies skills either way (OpenCode reads
    // `.claude/skills/<name>/SKILL.md` natively), so the plain opencode-only
    // invocation already exhibits the "agents only" behavior this scenario checks.
    w.exec(&["harness", "bindings", "generate", "--harness", "opencode"]);
}

// ===========================================================================
// agents sync — Then steps
// ===========================================================================

#[then("the .opencode/ directory contains the converted configuration")]
fn then_opencode_has_config(w: &mut AgentsWorld) {
    let p = w.work.path().join(".opencode/agents/foo-maker.md");
    assert!(p.exists(), "expected {} to exist", p.display());
    let content = std::fs::read_to_string(&p).expect("read converted agent");
    assert!(
        content.contains("model: opencode-go/minimax-m2.7"),
        "got: {content}"
    );
    assert!(
        content.contains("permission:\n  read: allow\n  write: allow"),
        "got: {content}"
    );
}

#[then("the output describes the planned operations")]
fn then_output_describes_plan(w: &mut AgentsWorld) {
    let out = w.stdout();
    assert!(out.contains("Agents: 2 converted"), "got: {out}");
}

#[then("no files are written to the .opencode/ directory")]
fn then_no_opencode_files(w: &mut AgentsWorld) {
    let p = w.work.path().join(".opencode/agents/foo-maker.md");
    assert!(
        !p.exists(),
        "expected {} NOT to exist after dry-run",
        p.display()
    );
}

#[then("only agent files are written to the .opencode/ directory")]
fn then_only_agents_written(w: &mut AgentsWorld) {
    // Agents are written; skills are never copied (read natively).
    assert!(w.work.path().join(".opencode/agents/foo-maker.md").exists());
    assert!(!w.work.path().join(".opencode/skills").exists());
    assert!(!w.work.path().join(".opencode/skill").exists());
}

#[then(
    r#"the corresponding .opencode/ agent uses the "opencode-go/minimax-m2.7" model identifier"#
)]
fn then_opencode_model_minimax(w: &mut AgentsWorld) {
    let content =
        std::fs::read_to_string(w.work.path().join(".opencode/agents/foo-maker.md")).expect("read");
    assert!(
        content.contains("model: opencode-go/minimax-m2.7"),
        "got: {content}"
    );
}

// ===========================================================================
// agents validate-sync — Given steps
// ===========================================================================

/// Writes a synced pair: a Claude agent and its byte-correct OpenCode form.
fn write_synced_pair(w: &AgentsWorld, name: &str) {
    w.write_claude_agent(name, "", &[]);
    w.write(
        &format!(".opencode/agents/{name}.md"),
        &format!(
            "---\ndescription: Agent {name}.\nmodel: opencode-go/minimax-m2.7\npermission:\n  read: allow\n  write: allow\n---\n# Body\n"
        ),
    );
}

#[given(".claude/ and .opencode/ configurations that are fully synchronised")]
fn given_fully_synced(w: &mut AgentsWorld) {
    write_synced_pair(w, "foo-maker");
    write_synced_pair(w, "bar-checker");
}

#[given("an agent in .claude/ whose description differs from its .opencode/ counterpart")]
fn given_description_mismatch(w: &mut AgentsWorld) {
    write_synced_pair(w, "foo-maker");
    // Overwrite the opencode side with a different description.
    w.write(
        ".opencode/agents/foo-maker.md",
        "---\ndescription: A totally different description.\nmodel: opencode-go/minimax-m2.7\npermission:\n  read: allow\n  write: allow\n---\n# Body\n",
    );
}

#[given(".claude/ containing more agents than .opencode/")]
fn given_count_mismatch(w: &mut AgentsWorld) {
    write_synced_pair(w, "foo-maker");
    // Extra Claude agent with no OpenCode counterpart.
    w.write_claude_agent("bar-checker", "", &[]);
}

// ===========================================================================
// agents validate-sync — When/Then steps
// ===========================================================================

#[when("the developer runs agents validate-sync")]
fn when_validate_sync(w: &mut AgentsWorld) {
    w.exec(&["harness", "sync", "validate"]);
}

#[then("the output reports all sync checks as passing")]
fn then_sync_all_passing(w: &mut AgentsWorld) {
    let out = w.stdout();
    assert!(out.contains("Failed: 0"), "got: {out}");
    assert!(out.contains("VALIDATION PASSED"), "got: {out}");
}

#[then("the output identifies the agent with the mismatched description")]
fn then_identifies_desc_mismatch(w: &mut AgentsWorld) {
    let out = w.stdout();
    assert!(out.contains("Agent: foo-maker.md"), "got: {out}");
    assert!(out.contains("Description mismatch"), "got: {out}");
}

#[then("the output reports the agent count mismatch")]
fn then_reports_count_mismatch(w: &mut AgentsWorld) {
    let out = w.stdout();
    assert!(out.contains("Agent Count"), "got: {out}");
    assert!(
        out.contains("missing one or more Claude agents"),
        "got: {out}"
    );
}

// ===========================================================================
// agents validate-claude — Given steps
// ===========================================================================

#[given("a .claude/ directory where all agents and skills are valid")]
fn given_all_valid(w: &mut AgentsWorld) {
    w.write_claude_skill("my-skill");
    w.write_claude_agent("foo-maker", "", &["my-skill"]);
}

#[given(r#"a .claude/ directory where one agent is missing the required "tools" field"#)]
fn given_missing_tools(w: &mut AgentsWorld) {
    w.write_claude_skill("my-skill");
    w.write(
        ".claude/agents/foo-maker.md",
        "---\nname: foo-maker\ndescription: d\nmodel:\ncolor: blue\n---\n# Body\n",
    );
}

#[given("a .claude/ directory containing two agent files declaring the same name")]
fn given_duplicate_name(w: &mut AgentsWorld) {
    // Two files; both declare name `dup-maker`. The filename-match rule will
    // also flag one, but the uniqueness rule fires on the second by name.
    w.write(
        ".claude/agents/dup-maker.md",
        "---\nname: dup-maker\ndescription: d\ntools: Read\nmodel:\ncolor: blue\n---\n# Body\n",
    );
    w.write(
        ".claude/agents/other-maker.md",
        "---\nname: dup-maker\ndescription: d\ntools: Read\nmodel:\ncolor: blue\n---\n# Body\n",
    );
}

#[given("a .claude/ directory where agents are valid but skills have issues")]
fn given_valid_agents_bad_skills(w: &mut AgentsWorld) {
    // Skill missing description (invalid), but agent does not reference it.
    w.write(
        ".claude/skills/broken/SKILL.md",
        "---\nname: broken\n---\n# body\n",
    );
    w.write_claude_agent("foo-maker", "", &[]);
}

#[given("a .claude/ directory where skills are valid but agents have issues")]
fn given_valid_skills_bad_agents(w: &mut AgentsWorld) {
    w.write_claude_skill("my-skill");
    // Agent with an invalid color (issue), but we will only validate skills.
    w.write(
        ".claude/agents/foo-maker.md",
        "---\nname: foo-maker\ndescription: d\ntools: Read\nmodel:\ncolor: chartreuse\n---\n# Body\n",
    );
}

// ===========================================================================
// agents validate-claude — When/Then steps
// ===========================================================================

#[when("the developer runs agents validate-claude")]
fn when_validate_claude(w: &mut AgentsWorld) {
    w.exec(&["harness", "claude", "validate"]);
}

#[when("the developer runs agents validate-claude with the --agents-only flag")]
fn when_validate_claude_agents_only(w: &mut AgentsWorld) {
    w.exec(&["harness", "claude", "validate", "--agents-only"]);
}

#[when("the developer runs agents validate-claude with the --skills-only flag")]
fn when_validate_claude_skills_only(w: &mut AgentsWorld) {
    w.exec(&["harness", "claude", "validate", "--skills-only"]);
}

#[then("the output reports all checks as passing")]
fn then_claude_all_passing(w: &mut AgentsWorld) {
    let out = w.stdout();
    assert!(out.contains("Failed: 0"), "got: {out}");
    assert!(out.contains("VALIDATION PASSED"), "got: {out}");
}

#[then("the output identifies the agent and the missing field")]
fn then_identifies_missing_field(w: &mut AgentsWorld) {
    let out = w.stdout();
    assert!(out.contains("foo-maker.md"), "got: {out}");
    assert!(out.contains("Required Fields"), "got: {out}");
    assert!(out.contains("tools"), "got: {out}");
}

#[then("the output reports the duplicate agent name")]
fn then_reports_duplicate(w: &mut AgentsWorld) {
    let out = w.stdout();
    assert!(out.contains("Name Uniqueness"), "got: {out}");
    assert!(out.contains("dup-maker"), "got: {out}");
}

// ===========================================================================
// agents validate-naming — Given steps
// ===========================================================================

/// Writes a synced agent pair (Claude + OpenCode) with a valid role suffix.
fn write_named_pair(w: &AgentsWorld, name: &str) {
    w.write(
        &format!(".claude/agents/{name}.md"),
        &format!(
            "---\nname: {name}\ndescription: d\ntools: Read\nmodel:\ncolor: blue\n---\n# Body\n"
        ),
    );
    w.write(
        &format!(".opencode/agents/{name}.md"),
        "---\ndescription: d\nmodel: opencode-go/minimax-m2.7\npermission:\n  read: allow\n---\n# Body\n",
    );
}

#[given(
    "a repository where every agent filename ends with an allowed role suffix and mirrors across harnesses"
)]
fn given_naming_clean(w: &mut AgentsWorld) {
    write_named_pair(w, "foo-maker");
    write_named_pair(w, "bar-checker");
}

#[given("a repository with one agent whose filename ends in an unknown suffix")]
fn given_naming_bad_suffix(w: &mut AgentsWorld) {
    write_named_pair(w, "foo-maker");
    // `foo-widget` has no allowed role suffix.
    w.write(
        ".claude/agents/foo-widget.md",
        "---\nname: foo-widget\ndescription: d\ntools: Read\nmodel:\ncolor: blue\n---\n# Body\n",
    );
    w.write(
        ".opencode/agents/foo-widget.md",
        "---\ndescription: d\nmodel: opencode-go/minimax-m2.7\npermission:\n  read: allow\n---\n# Body\n",
    );
}

#[given(
    "a repository with a .claude/agents/ file whose frontmatter name differs from its filename"
)]
fn given_naming_frontmatter_mismatch(w: &mut AgentsWorld) {
    write_named_pair(w, "foo-maker");
    // Filename foo-checker.md but frontmatter name says something-else-maker.
    w.write(
        ".claude/agents/baz-checker.md",
        "---\nname: wrong-checker\ndescription: d\ntools: Read\nmodel:\ncolor: blue\n---\n# Body\n",
    );
    w.write(
        ".opencode/agents/baz-checker.md",
        "---\ndescription: d\nmodel: opencode-go/minimax-m2.7\npermission:\n  read: allow\n---\n# Body\n",
    );
}

#[given("a repository where one .claude/agents/ file has no corresponding .opencode/agent/ file")]
fn given_naming_mirror_drift(w: &mut AgentsWorld) {
    write_named_pair(w, "foo-maker");
    // Claude-only file (no opencode mirror).
    w.write(
        ".claude/agents/lonely-maker.md",
        "---\nname: lonely-maker\ndescription: d\ntools: Read\nmodel:\ncolor: blue\n---\n# Body\n",
    );
}

// ===========================================================================
// agents validate-naming — When/Then steps
// ===========================================================================

#[when("the developer runs agents validate-naming")]
fn when_validate_naming(w: &mut AgentsWorld) {
    w.exec(&["harness", "naming", "validate"]);
}

#[then("the output reports zero naming violations")]
fn then_naming_zero(w: &mut AgentsWorld) {
    let out = w.stdout();
    assert!(
        out.contains("VALIDATION PASSED (0 violations)"),
        "got: {out}"
    );
}

#[then("the output identifies the offending agent file and its unknown suffix")]
fn then_naming_bad_suffix(w: &mut AgentsWorld) {
    let out = w.stdout();
    assert!(out.contains("foo-widget"), "got: {out}");
    assert!(out.contains("role-suffix"), "got: {out}");
}

#[then("the output identifies the frontmatter mismatch")]
fn then_naming_frontmatter_mismatch(w: &mut AgentsWorld) {
    let out = w.stdout();
    assert!(out.contains("frontmatter-mismatch"), "got: {out}");
    assert!(out.contains("wrong-checker"), "got: {out}");
}

#[then("the output identifies the mirror-drift violation")]
fn then_naming_mirror_drift(w: &mut AgentsWorld) {
    let out = w.stdout();
    assert!(out.contains("mirror-drift"), "got: {out}");
    assert!(out.contains("lonely-maker"), "got: {out}");
}

// ===========================================================================
// agents emit-bindings / validate-bindings
// ===========================================================================

impl AgentsWorld {
    /// Writes a minimal canonical `AGENTS.md` at the repo root.
    fn write_agents_md(&self) {
        self.write("AGENTS.md", "# AGENTS.md\n\nCanonical instructions.\n");
    }

    /// Writes both expected binding files with their canonical content, so a
    /// fresh regenerate matches byte-for-byte. Drives the actual
    /// `harness bindings generate --harness amazonq` command rather than a
    /// hand-maintained literal, so this fixture can never drift out of sync
    /// with the canonical content in `application::agents::bindings`.
    fn write_matching_bindings(&mut self) {
        self.exec(&["harness", "bindings", "generate", "--harness", "amazonq"]);
    }

    /// Writes a platform-bindings catalog naming every binding directory.
    fn write_full_catalog(&self) {
        self.write(
            "docs/reference/platform-bindings.md",
            "# Platform Bindings\n\nDirectories: .claude, .opencode, .codex, .github, .amazonq\n",
        );
    }

    /// Creates the binding directories that the catalog must document, plus
    /// empty `agents/` subdirectories under `.claude/` and `.opencode/` so the
    /// `harness bindings validate` OpenCode-sync-equivalence checks (folded in
    /// from `harness sync validate`) find readable, trivially-equal directories
    /// instead of failing with "directory not found".
    fn make_binding_dirs(&self) {
        for d in [".claude", ".opencode", ".codex", ".github", ".amazonq"] {
            std::fs::create_dir_all(self.work.path().join(d)).expect("mk binding dir");
        }
        std::fs::create_dir_all(self.work.path().join(".claude/agents")).expect("mk agents dir");
        std::fs::create_dir_all(self.work.path().join(".opencode/agents")).expect("mk agents dir");
    }
}

#[given("a repository with a canonical AGENTS.md at the root")]
fn given_canonical_agents_md(w: &mut AgentsWorld) {
    w.write_agents_md();
}

#[given("a repository whose committed binding files match a fresh regenerate")]
fn given_bindings_match(w: &mut AgentsWorld) {
    w.write_agents_md();
    w.write_matching_bindings();
    w.make_binding_dirs();
}

#[given("the platform-bindings catalog documents every binding directory present on disk")]
fn given_catalog_complete(w: &mut AgentsWorld) {
    w.write_full_catalog();
}

#[given(
    "a repository where a committed binding file no longer matches a regenerate from AGENTS.md"
)]
fn given_binding_drift(w: &mut AgentsWorld) {
    w.write_agents_md();
    w.write_matching_bindings();
    w.write_full_catalog();
    w.make_binding_dirs();
    // Corrupt one committed binding file so it drifts from the regenerate.
    w.write(
        ".amazonq/rules/00-agents-md.md",
        "# Edited by hand — drifted\n",
    );
}

#[given(
    "a repository with a binding directory that the platform-bindings catalog does not document"
)]
fn given_catalog_gap(w: &mut AgentsWorld) {
    w.write_agents_md();
    w.write_matching_bindings();
    w.make_binding_dirs();
    // Catalog omits the `.amazonq` directory, which exists on disk.
    w.write(
        "docs/reference/platform-bindings.md",
        "# Platform Bindings\n\nDirectories: .claude, .opencode, .codex, .github\n",
    );
}

#[when("the developer runs agents emit-bindings")]
fn when_emit_bindings(w: &mut AgentsWorld) {
    w.exec(&["harness", "bindings", "generate", "--harness", "amazonq"]);
}

#[when("the developer runs agents emit-bindings with the --dry-run flag")]
fn when_emit_bindings_dry_run(w: &mut AgentsWorld) {
    w.exec(&[
        "harness",
        "bindings",
        "generate",
        "--harness",
        "amazonq",
        "--dry-run",
    ]);
}

#[when("the developer runs agents validate-bindings")]
fn when_validate_bindings(w: &mut AgentsWorld) {
    w.exec(&["harness", "bindings", "validate"]);
}

#[then("the Amazon Q rules pointer and default agent JSON are written")]
fn then_bindings_written(w: &mut AgentsWorld) {
    assert!(
        w.work
            .path()
            .join(".amazonq/cli-agents/ose-default.json")
            .exists(),
        "stdout: {}",
        w.stdout()
    );
    assert!(
        w.work
            .path()
            .join(".amazonq/rules/00-agents-md.md")
            .exists(),
        "stdout: {}",
        w.stdout()
    );
}

#[then("each generated file references AGENTS.md without duplicating its body")]
fn then_bindings_reference_agents_md(w: &mut AgentsWorld) {
    let json = std::fs::read_to_string(w.work.path().join(".amazonq/cli-agents/ose-default.json"))
        .expect("read json");
    let rule = std::fs::read_to_string(w.work.path().join(".amazonq/rules/00-agents-md.md"))
        .expect("read rule");
    assert!(json.contains("AGENTS.md"), "json: {json}");
    assert!(rule.contains("AGENTS.md"), "rule: {rule}");
    // The generated files point at AGENTS.md rather than copying its body.
    assert!(!json.contains("Canonical instructions."), "json: {json}");
    assert!(!rule.contains("Canonical instructions."), "rule: {rule}");
}

#[then("the output lists the files that would be written")]
fn then_dry_run_lists_files(w: &mut AgentsWorld) {
    let out = w.stdout();
    assert!(
        out.contains("would write .amazonq/cli-agents/ose-default.json"),
        "got: {out}"
    );
    assert!(
        out.contains("would write .amazonq/rules/00-agents-md.md"),
        "got: {out}"
    );
}

#[then("no binding files are created on disk")]
fn then_no_bindings_on_disk(w: &mut AgentsWorld) {
    assert!(
        !w.work
            .path()
            .join(".amazonq/cli-agents/ose-default.json")
            .exists()
    );
    assert!(
        !w.work
            .path()
            .join(".amazonq/rules/00-agents-md.md")
            .exists()
    );
}

#[then("the output reports zero binding drift and zero catalog gaps")]
fn then_zero_drift_zero_gaps(w: &mut AgentsWorld) {
    let out = w.stdout();
    assert!(out.contains("Failed: 0"), "got: {out}");
    assert!(out.contains("VALIDATION PASSED"), "got: {out}");
}

#[then("the output identifies the drifted binding file")]
fn then_identifies_drift(w: &mut AgentsWorld) {
    let out = w.stdout();
    assert!(
        out.contains("Binding: .amazonq/rules/00-agents-md.md"),
        "got: {out}"
    );
    assert!(out.contains("drifted from canonical content"), "got: {out}");
}

#[then("the output identifies the binding directory missing from the catalog")]
fn then_identifies_catalog_gap(w: &mut AgentsWorld) {
    let out = w.stdout();
    assert!(out.contains("Catalog Coverage: .amazonq"), "got: {out}");
    assert!(out.contains("absent from catalog"), "got: {out}");
}

// ===========================================================================
// Shared Then steps (exit codes)
// ===========================================================================

#[then("the command exits successfully")]
fn then_exit_ok(w: &mut AgentsWorld) {
    assert_eq!(w.exit_code(), 0, "stdout: {}", w.stdout());
}

#[then("the command exits with a failure code")]
fn then_exit_fail(w: &mut AgentsWorld) {
    assert_eq!(w.exit_code(), 1, "stdout: {}", w.stdout());
}

#[tokio::main]
async fn main() {
    AgentsWorld::run(feature_dir()).await;
}

fn feature_dir() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .join("../../specs/apps/rhino/behavior/rhino-cli/gherkin/agents")
        .canonicalize()
        .expect("feature dir resolvable")
}
