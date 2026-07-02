//! Cucumber-rs regression suite for the agent-naming validator trigger path.
//!
//! Drives `rhino-cli harness naming validate` against a synthetic repo with an
//! invalid-suffix agent file (must fail), and asserts no trigger path in
//! `.husky/pre-push` or `apps/rhino-cli/src` references the buggy singular
//! `.opencode/agent/` form (canonical is the plural `.opencode/agents/`).

#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::unwrap_used, clippy::panic)]

use std::path::{Path, PathBuf};
use std::process::Output;

use assert_cmd::cargo::cargo_bin;
use cucumber::{World as _, given, then, when};
use tempfile::TempDir;

#[derive(cucumber::World)]
#[world(init = Self::new)]
struct AgentNamingWorld {
    /// Synthetic git repo carrying an invalid-suffix agent file.
    repo: TempDir,
    /// Output of `harness naming validate`.
    output: Option<Output>,
}

impl std::fmt::Debug for AgentNamingWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AgentNamingWorld").finish_non_exhaustive()
    }
}

impl AgentNamingWorld {
    fn new() -> Self {
        Self {
            repo: TempDir::new().expect("temp repo"),
            output: None,
        }
    }
}

fn init_git_repo(dir: &Path) {
    let out = std::process::Command::new("git")
        .args(["init", "-q"])
        .current_dir(dir)
        .output()
        .expect("run git init");
    assert!(out.status.success(), "git init failed: {out:?}");
}

/// Repo root of the workspace under test (two levels up from the crate manifest).
fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolvable")
}

/// Collect files under `dir` (recursively) that contain the singular
/// `.opencode/agent/` form. The plural `.opencode/agents/` never matches this
/// substring (the char after `agent` is `s`, not `/`).
fn files_with_singular_form(paths: &[PathBuf]) -> Vec<String> {
    let mut offenders = Vec::new();
    for path in paths {
        if path.is_file() {
            if let Ok(content) = std::fs::read_to_string(path)
                && content.contains(".opencode/agent/")
            {
                offenders.push(path.display().to_string());
            }
        } else if path.is_dir() {
            for entry in walkdir_rs(path) {
                if let Ok(content) = std::fs::read_to_string(&entry)
                    && content.contains(".opencode/agent/")
                {
                    offenders.push(entry.display().to_string());
                }
            }
        }
    }
    offenders
}

/// Minimal recursive `.rs` file walker (avoids a dev-dep on walkdir here).
fn walkdir_rs(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return out;
    };
    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_dir() {
            out.extend(walkdir_rs(&p));
        } else if p.extension().and_then(|e| e.to_str()) == Some("rs") {
            out.push(p);
        }
    }
    out
}

#[given("an agent file renamed to an invalid suffix")]
fn given_invalid_agent_file(w: &mut AgentNamingWorld) {
    let root = w.repo.path();
    init_git_repo(root);
    // foo-bar is an invalid role suffix (bar ∉ {maker, checker, fixer, …}).
    let agents = root.join(".claude/agents");
    std::fs::create_dir_all(&agents).unwrap();
    std::fs::write(agents.join("foo-bar.md"), "---\nname: foo-bar\n---\n").unwrap();
    std::fs::create_dir_all(root.join(".opencode/agents")).unwrap();
}

#[when("the naming validator runs (triggered on .opencode/agents/ changes)")]
fn when_naming_validator_runs(w: &mut AgentNamingWorld) {
    let out = std::process::Command::new(cargo_bin("rhino-cli"))
        .args(["harness", "naming", "validate", "--no-color"])
        .current_dir(w.repo.path())
        .env("PWD", w.repo.path())
        .output()
        .expect("run rhino-cli harness naming validate");
    w.output = Some(out);
}

#[then("it detects the invalid name and fails")]
fn then_detects_and_fails(w: &mut AgentNamingWorld) {
    let out = w.output.as_ref().expect("validator ran");
    assert!(
        !out.status.success(),
        "naming validator must fail on an invalid-suffix agent file; stdout={} stderr={}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        combined.contains("foo-bar") || combined.contains("role-suffix"),
        "output must name the offending file or violation kind; got: {combined}"
    );
}

#[then("no trigger path references the singular .opencode/agent/")]
fn then_no_singular_trigger_path(_w: &mut AgentNamingWorld) {
    let root = repo_root();
    let targets = vec![
        root.join(".husky/pre-push"),
        root.join("apps/rhino-cli/src"),
    ];
    let offenders = files_with_singular_form(&targets);
    assert!(
        offenders.is_empty(),
        "no trigger path may reference the singular .opencode/agent/ form; offenders: {offenders:?}"
    );
}

#[tokio::main]
async fn main() {
    AgentNamingWorld::run(feature_dir()).await;
}

fn feature_dir() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .join("../../specs/apps/rhino/behavior/rhino-cli/gherkin/agent-naming")
        .canonicalize()
        .expect("feature dir resolvable")
}
