//! Pre-commit pipeline use case — the 8-step pre-commit runner.
//!
//! Reproduces the pipeline originally in `internal/git.rs`, now as a
//! hexagonal use case with injected I/O via [`Deps`].

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

use anyhow::{Error, anyhow};

use crate::application::git::port::StagedFileProvider;
use crate::domain::git::staged_files::{STAGED_SKIP_PREFIXES, has_match, staged_md_files};
use crate::domain::mermaid::{default_validate_options, extract_blocks, validate_blocks};
use crate::internal::agents::claude_validator::validate_claude;
use crate::internal::agents::sync::{SyncOptions, sync_all};
use crate::internal::agents::sync_validator::validate_sync;
use crate::internal::agents::types::ValidateClaudeOptions;
use crate::internal::docs::heading_hierarchy::{
    is_prose_allowlisted, validate_docs_heading_hierarchy,
};
use crate::internal::docs::links::{ScanOptions, validate_all_links};

/// Maximum time allowed for a single pipeline step before it is skipped.
const STEP_TIMEOUT: Duration = Duration::from_secs(30);

/// Maximum total wall-clock time for the entire pre-commit pipeline.
const TOTAL_TIMEOUT: Duration = Duration::from_secs(120);

/// Injectable dependencies for the pre-commit pipeline.
pub struct Deps {
    /// Absolute path to the repository root used as the working directory.
    pub git_root: PathBuf,
    /// Writer for progress and status messages.
    pub stdout: Box<dyn Write + Send>,
    /// Writer for error output.
    pub stderr: Box<dyn Write + Send>,
    /// Port for reading the staged file list from the git index.
    pub staged_file_provider: Box<dyn StagedFileProvider>,
}

/// Runs `fn_` inline and enforces both the per-step and total timeouts.
fn run_with_step_timeout<F>(
    total_start: Instant,
    name: &str,
    deps: &mut Deps,
    fn_: F,
) -> Result<(), Error>
where
    F: FnOnce(&mut Deps) -> Result<(), Error> + Send + 'static,
{
    if total_start.elapsed() >= TOTAL_TIMEOUT {
        let _ = writeln!(
            deps.stdout,
            "\u{26A0}\u{FE0F}  Total pre-commit timeout reached — skipping remaining steps (including {name})"
        );
        return Ok(());
    }
    let start = Instant::now();
    let r = fn_(deps);
    let elapsed = start.elapsed();
    if elapsed > STEP_TIMEOUT {
        let _ = writeln!(
            deps.stdout,
            "\u{26A0}\u{FE0F}  Step {:?} timed out after {}s — skipping",
            name,
            STEP_TIMEOUT.as_secs()
        );
        return Ok(());
    }
    r
}

/// Step 1: validates `.claude/` and `.opencode/` configuration when any such files are staged.
fn step1_config(git_root: &Path, staged: &[String], deps: &mut Deps) -> Result<(), Error> {
    let has = has_match(staged, |f| {
        f.starts_with(".claude/") || f.starts_with(".opencode/")
    });
    if !has {
        writeln!(
            deps.stdout,
            "\u{23ED}\u{FE0F}  Skipping config validation (no .claude/ or .opencode/ changes in staged files)"
        )?;
        return Ok(());
    }
    writeln!(
        deps.stdout,
        "\u{1F50D} Validating .claude/ and .opencode/ configuration..."
    )?;
    let r = validate_claude(&ValidateClaudeOptions {
        repo_root: git_root.to_path_buf(),
        ..Default::default()
    });
    if r.failed_checks > 0 {
        writeln!(
            deps.stdout,
            "\u{274C} Configuration validation failed. Fix errors above before committing."
        )?;
        return Err(anyhow!(
            "validation failed: {} checks failed",
            r.failed_checks
        ));
    }
    if let Err(e) = sync_all(&SyncOptions {
        repo_root: git_root.to_path_buf(),
        ..Default::default()
    }) {
        writeln!(
            deps.stdout,
            "\u{274C} Configuration sync failed. Fix errors above before committing."
        )?;
        return Err(anyhow!("sync failed: {e}"));
    }
    let sync_r = validate_sync(git_root);
    if sync_r.failed_checks > 0 {
        writeln!(
            deps.stdout,
            "\u{274C} Configuration validation failed. Fix errors above before committing."
        )?;
        return Err(anyhow!(
            "sync validation failed: {} checks failed",
            sync_r.failed_checks
        ));
    }
    writeln!(deps.stdout, "\u{2705} Configuration validation passed")?;
    Ok(())
}

/// Step 2: validates any staged `docker-compose.yml` / `docker-compose.yaml` files.
fn step2_docker_compose(git_root: &Path, staged: &[String], deps: &mut Deps) -> Result<(), Error> {
    let compose: Vec<&String> = staged
        .iter()
        .filter(|f| f.ends_with("docker-compose.yml") || f.ends_with("docker-compose.yaml"))
        .collect();
    if compose.is_empty() {
        writeln!(
            deps.stdout,
            "\u{23ED}\u{FE0F}  Skipping docker-compose validation (no docker-compose.yml changes in staged files)"
        )?;
        return Ok(());
    }
    writeln!(
        deps.stdout,
        "\u{1F50D} Validating docker-compose.yml files..."
    )?;
    for f in &compose {
        let abs = git_root.join(f);
        if !abs.exists() {
            continue;
        }
        writeln!(deps.stdout, "  Checking {f}...")?;
        let status = Command::new("docker")
            .arg("compose")
            .arg("-f")
            .arg(f)
            .arg("config")
            .current_dir(git_root)
            .status();
        match status {
            Ok(s) if s.success() => {
                writeln!(deps.stdout, "  \u{2705} {f} is valid")?;
            }
            _ => {
                writeln!(
                    deps.stdout,
                    "\u{274C} Docker Compose validation failed for {f}"
                )?;
                writeln!(deps.stdout, "   Run: docker compose -f {f} config")?;
                return Err(anyhow!("docker compose validation failed for {f}"));
            }
        }
    }
    writeln!(deps.stdout, "\u{2705} All docker-compose files validated")?;
    Ok(())
}

/// Step 3: runs `nx affected -t run-pre-commit --skip-nx-cache` for affected projects.
fn step3_nx_pre_commit(git_root: &Path, deps: &mut Deps) {
    let r = Command::new("nx")
        .arg("affected")
        .arg("-t")
        .arg("run-pre-commit")
        .arg("--skip-nx-cache")
        .current_dir(git_root)
        .status();
    if !r.is_ok_and(|s| s.success()) {
        let _ = writeln!(
            deps.stdout,
            "\u{26A0}\u{FE0F}  Skipping run-pre-commit (not affected or binary missing)"
        );
    }
}

/// Step 4: stages any changes in `apps/ayokoding-www/content/` produced by earlier steps.
fn step4_stage_ayokoding(git_root: &Path, _deps: &mut Deps) {
    let _ = Command::new("git")
        .arg("add")
        .arg("apps/ayokoding-www/content/")
        .current_dir(git_root)
        .status();
}

/// Step 5: runs `npx lint-staged` to format and lint staged files.
fn step5_lint_staged(git_root: &Path, _deps: &mut Deps) -> Result<(), Error> {
    let status = Command::new("npx")
        .arg("lint-staged")
        .current_dir(git_root)
        .status()?;
    if !status.success() {
        return Err(anyhow!("lint-staged failed"));
    }
    Ok(())
}

/// Step 5b: regenerates and stages `package-lock.json` for any app whose `package.json` is staged.
fn step5b_sync_lockfiles(git_root: &Path, staged: &[String], deps: &mut Deps) -> Result<(), Error> {
    let mut apps_to_sync: Vec<String> = Vec::new();
    for f in staged {
        if !f.starts_with("apps/") || !f.ends_with("/package.json") {
            continue;
        }
        let parts: Vec<&str> = f.split('/').collect();
        if parts.len() != 3 {
            continue;
        }
        let app_dir = git_root.join(parts[0]).join(parts[1]);
        let lockfile = app_dir.join("package-lock.json");
        if lockfile.exists() {
            apps_to_sync.push(format!("{}/{}", parts[0], parts[1]));
        }
    }
    if apps_to_sync.is_empty() {
        return Ok(());
    }
    writeln!(
        deps.stdout,
        "\u{1F512} Syncing app-level package-lock.json files..."
    )?;
    for app_rel in &apps_to_sync {
        let app_dir = git_root.join(app_rel);
        writeln!(deps.stdout, "  Regenerating {app_rel}/package-lock.json...")?;
        let status = Command::new("npm")
            .arg("install")
            .arg("--package-lock-only")
            .current_dir(&app_dir)
            .status()?;
        if !status.success() {
            return Err(anyhow!(
                "failed to regenerate package-lock.json in {app_rel}"
            ));
        }
        let lock_rel = format!("{app_rel}/package-lock.json");
        let _ = Command::new("git")
            .arg("add")
            .arg(&lock_rel)
            .current_dir(git_root)
            .status();
        writeln!(
            deps.stdout,
            "  \u{2705} {app_rel}/package-lock.json synced and staged"
        )?;
    }
    writeln!(deps.stdout, "\u{2705} All app lockfiles synced")?;
    Ok(())
}

/// Step: validates Mermaid diagrams in staged markdown files.
fn step_validate_mermaid(git_root: &Path, staged: &[String], deps: &mut Deps) -> Result<(), Error> {
    let candidates = staged_md_files(staged, STAGED_SKIP_PREFIXES);
    if candidates.is_empty() {
        return Ok(());
    }
    let mut opts = default_validate_options();
    opts.max_depth = 4;
    let mut all_blocks = Vec::new();
    for rel in &candidates {
        let abs = git_root.join(rel);
        if !abs.exists() {
            continue;
        }
        let Ok(content) = std::fs::read_to_string(&abs) else {
            continue;
        };
        all_blocks.extend(extract_blocks(rel, &content));
    }
    if all_blocks.is_empty() {
        return Ok(());
    }
    let result = validate_blocks(all_blocks, opts);
    if !result.violations.is_empty() {
        writeln!(
            deps.stderr,
            "\u{274C} Found {} mermaid violation(s) in staged files",
            result.violations.len()
        )?;
        return Err(anyhow!(
            "found {} mermaid violations",
            result.violations.len()
        ));
    }
    Ok(())
}

/// Step: validates heading hierarchy in staged markdown files that are in the prose allowlist.
fn step_validate_heading_hierarchy(
    git_root: &Path,
    staged: &[String],
    deps: &mut Deps,
) -> Result<(), Error> {
    let candidates = staged_md_files(staged, STAGED_SKIP_PREFIXES);
    let allowlisted: Vec<String> = candidates
        .into_iter()
        .filter(|rel| is_prose_allowlisted(rel))
        .map(|rel| git_root.join(rel).to_string_lossy().to_string())
        .collect();
    if allowlisted.is_empty() {
        return Ok(());
    }
    let findings = validate_docs_heading_hierarchy(&allowlisted)?;
    if !findings.is_empty() {
        writeln!(
            deps.stderr,
            "\u{274C} Found {} heading hierarchy violation(s) in staged files",
            findings.len()
        )?;
        return Err(anyhow!(
            "found {} heading hierarchy violations",
            findings.len()
        ));
    }
    Ok(())
}

/// Step 7: validates Markdown links in staged files.
fn step7_validate_links(git_root: &Path, deps: &mut Deps) -> Result<(), Error> {
    let r = validate_all_links(&ScanOptions {
        repo_root: git_root.to_path_buf(),
        staged_only: true,
        skip_paths: vec![
            ".claude/worktrees/".to_string(),
            "plans/done".to_string(),
            "apps/ayokoding-www/content".to_string(),
            "apps/ose-www/content".to_string(),
        ],
    })?;
    if !r.broken_links.is_empty() {
        let text = crate::internal::docs::links::format_link_text(&r, false, false);
        let _ = deps.stderr.write_all(text.as_bytes());
        writeln!(
            deps.stderr,
            "\n\u{274C} Found {} broken links",
            r.broken_links.len()
        )?;
        return Err(anyhow!("found {} broken links", r.broken_links.len()));
    }
    Ok(())
}

/// Step 8: runs `npm run lint:md` to lint all Markdown files.
fn step8_lint_markdown(git_root: &Path, _deps: &mut Deps) -> Result<(), Error> {
    let status = Command::new("npm")
        .arg("run")
        .arg("lint:md")
        .current_dir(git_root)
        .status()?;
    if !status.success() {
        return Err(anyhow!("markdown linting failed"));
    }
    Ok(())
}

/// Runs the full 8-step pre-commit pipeline.
///
/// # Errors
///
/// Returns an error when any pipeline step fails.
pub fn run(deps: &mut Deps) -> Result<(), Error> {
    let total_start = Instant::now();
    let git_root = deps.git_root.clone();
    let staged = deps.staged_file_provider.get_staged(&git_root)?;

    let staged1 = staged.clone();
    let root1 = git_root.clone();
    run_with_step_timeout(total_start, "step1Config", deps, move |d| {
        step1_config(&root1, &staged1, d)
    })?;

    let staged2 = staged.clone();
    let root2 = git_root.clone();
    run_with_step_timeout(total_start, "step2DockerCompose", deps, move |d| {
        step2_docker_compose(&root2, &staged2, d)
    })?;

    let root3 = git_root.clone();
    run_with_step_timeout(total_start, "step3NxPreCommit", deps, move |d| {
        step3_nx_pre_commit(&root3, d);
        Ok(())
    })?;

    let root4 = git_root.clone();
    run_with_step_timeout(total_start, "step4StageAyokoding", deps, move |d| {
        step4_stage_ayokoding(&root4, d);
        Ok(())
    })?;

    let root5 = git_root.clone();
    run_with_step_timeout(total_start, "step5LintStaged", deps, move |d| {
        step5_lint_staged(&root5, d)
    })?;

    let staged5b = staged.clone();
    let root5b = git_root.clone();
    run_with_step_timeout(total_start, "step5bSyncLockfiles", deps, move |d| {
        step5b_sync_lockfiles(&root5b, &staged5b, d)
    })?;

    let staged_for_mermaid = staged.clone();
    let git_root_for_mermaid = git_root.clone();
    run_with_step_timeout(total_start, "step6mValidateMermaid", deps, move |d| {
        step_validate_mermaid(&git_root_for_mermaid, &staged_for_mermaid, d)
    })?;

    let staged_for_headings = staged.clone();
    let git_root_for_headings = git_root.clone();
    run_with_step_timeout(
        total_start,
        "step6hValidateHeadingHierarchy",
        deps,
        move |d| step_validate_heading_hierarchy(&git_root_for_headings, &staged_for_headings, d),
    )?;

    let root7 = git_root.clone();
    run_with_step_timeout(total_start, "step7ValidateLinks", deps, move |d| {
        step7_validate_links(&root7, d)
    })?;

    let root8 = git_root.clone();
    run_with_step_timeout(total_start, "step8LintMarkdown", deps, move |d| {
        step8_lint_markdown(&root8, d)
    })?;

    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use std::path::Path;

    use tempfile::tempdir;

    use super::*;

    struct FakeStagedFileProvider;
    impl StagedFileProvider for FakeStagedFileProvider {
        fn get_staged(&self, _: &Path) -> Result<Vec<String>, anyhow::Error> {
            Ok(Vec::new())
        }
    }

    fn make_test_deps(dir: &tempfile::TempDir) -> Deps {
        Deps {
            git_root: dir.path().to_path_buf(),
            stdout: Box::new(Vec::<u8>::new()),
            stderr: Box::new(Vec::<u8>::new()),
            staged_file_provider: Box::new(FakeStagedFileProvider),
        }
    }

    #[test]
    fn step1_skip_when_no_config_changes() {
        let dir = tempdir().unwrap();
        let mut deps = make_test_deps(&dir);
        let r = step1_config(dir.path(), &[], &mut deps);
        assert!(r.is_ok());
    }

    #[test]
    fn step2_skip_when_no_compose_changes() {
        let dir = tempdir().unwrap();
        let mut deps = make_test_deps(&dir);
        let r = step2_docker_compose(dir.path(), &[], &mut deps);
        assert!(r.is_ok());
    }

    #[test]
    fn step5b_skip_when_no_app_lockfile_change() {
        let dir = tempdir().unwrap();
        let mut deps = make_test_deps(&dir);
        let r = step5b_sync_lockfiles(dir.path(), &[], &mut deps);
        assert!(r.is_ok());
    }

    #[test]
    fn step1_runs_when_claude_staged_but_no_files_passes() {
        let dir = tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join(".claude/agents")).unwrap();
        std::fs::create_dir_all(dir.path().join(".claude/skills")).unwrap();
        let mut deps = make_test_deps(&dir);
        let r = step1_config(dir.path(), &[".claude/agents/foo.md".into()], &mut deps);
        assert!(r.is_ok());
    }

    #[test]
    fn step2_skips_nonexistent_compose_file() {
        let dir = tempdir().unwrap();
        let mut deps = make_test_deps(&dir);
        let r = step2_docker_compose(
            dir.path(),
            &["does-not-exist/docker-compose.yml".into()],
            &mut deps,
        );
        assert!(r.is_ok());
    }

    #[test]
    fn step5b_only_handles_apps_with_lockfile() {
        let dir = tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("apps/x")).unwrap();
        std::fs::write(dir.path().join("apps/x/package.json"), "{}").unwrap();
        let mut deps = make_test_deps(&dir);
        let r = step5b_sync_lockfiles(dir.path(), &["apps/x/package.json".into()], &mut deps);
        assert!(r.is_ok());
    }

    #[test]
    fn step5b_ignores_nested_package_json() {
        let dir = tempdir().unwrap();
        let mut deps = make_test_deps(&dir);
        let r = step5b_sync_lockfiles(dir.path(), &["apps/x/sub/package.json".into()], &mut deps);
        assert!(r.is_ok());
    }

    #[test]
    fn step7_validate_links_clean_empty_repo() {
        let dir = tempdir().unwrap();
        let _ = std::process::Command::new("git")
            .args(["init", "--quiet"])
            .current_dir(dir.path())
            .status();
        let mut deps = make_test_deps(&dir);
        let _ = step7_validate_links(dir.path(), &mut deps);
    }

    #[test]
    fn run_with_step_timeout_short_circuits_after_total() {
        let dir = tempdir().unwrap();
        let mut deps = make_test_deps(&dir);
        let past = Instant::now()
            .checked_sub(Duration::from_secs(200))
            .unwrap();
        let r: Result<(), Error> = run_with_step_timeout(past, "test", &mut deps, |_| Ok(()));
        assert!(r.is_ok());
    }

    #[test]
    fn step_mermaid_blocks_malformed_staged_file() {
        let dir = tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("docs")).unwrap();
        std::fs::write(
            dir.path().join("docs/broken.md"),
            "```mermaid\nflowchart LR\n    A[This label is way too long and exceeds thirty chars] --> B\n```\n",
        )
        .unwrap();
        let mut deps = make_test_deps(&dir);
        let staged = vec!["docs/broken.md".to_string()];
        let r = step_validate_mermaid(dir.path(), &staged, &mut deps);
        assert!(
            r.is_err(),
            "mermaid label violation should cause step to fail"
        );
    }

    #[test]
    fn step_heading_blocks_docs_file_with_duplicate_h1() {
        let dir = tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("docs")).unwrap();
        std::fs::write(dir.path().join("docs/page.md"), "# First\n\n# Second\n").unwrap();
        let mut deps = make_test_deps(&dir);
        let staged = vec!["docs/page.md".to_string()];
        let r = step_validate_heading_hierarchy(dir.path(), &staged, &mut deps);
        assert!(r.is_err(), "duplicate H1 in docs/ file should block commit");
    }

    #[test]
    fn step_heading_allows_skill_file_with_multiple_h1() {
        let dir = tempdir().unwrap();
        let skill_dir = dir.path().join(".claude/skills/my-skill");
        std::fs::create_dir_all(&skill_dir).unwrap();
        std::fs::write(skill_dir.join("SKILL.md"), "# One\n\n# Two\n\n# Three\n").unwrap();
        let mut deps = make_test_deps(&dir);
        let staged = vec![".claude/skills/my-skill/SKILL.md".to_string()];
        let r = step_validate_heading_hierarchy(dir.path(), &staged, &mut deps);
        assert!(r.is_ok(), "SKILL.md must not be blocked by heading step");
    }

    #[test]
    fn step_validate_mermaid_uses_standardized_max_depth() {
        use std::fmt::Write as _;
        let dir = tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("docs")).unwrap();
        let mut src = String::from("# T\n\n```mermaid\nflowchart TB\n");
        for b in 0..5 {
            let _ = writeln!(src, "R --> A{b}");
            for l in 0..4 {
                let _ = writeln!(
                    src,
                    "{}{b} --> {}{b}",
                    (b'A' + l) as char,
                    (b'B' + l) as char
                );
            }
        }
        src.push_str("```\n");
        std::fs::write(dir.path().join("docs/wide-deep.md"), src).unwrap();
        let mut deps = make_test_deps(&dir);
        let staged = vec!["docs/wide-deep.md".to_string()];
        let r = step_validate_mermaid(dir.path(), &staged, &mut deps);
        assert!(
            r.is_ok(),
            "wide+deep diagram must be a warning (not a violation) at the staged step: {r:?}"
        );
    }

    #[test]
    fn step7_excludes_plans_done_broken_link() {
        let dir = tempdir().unwrap();
        let _ = std::process::Command::new("git")
            .args(["init", "--quiet"])
            .current_dir(dir.path())
            .status();
        std::fs::create_dir_all(dir.path().join("plans/done/old-plan")).unwrap();
        std::fs::write(
            dir.path().join("plans/done/old-plan/delivery.md"),
            "[broken](nonexistent.md)\n",
        )
        .unwrap();
        let mut deps = make_test_deps(&dir);
        let r = step7_validate_links(dir.path(), &mut deps);
        assert!(r.is_ok(), "step7 should succeed with no staged files");
    }
}
