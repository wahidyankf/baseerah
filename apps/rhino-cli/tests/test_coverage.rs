//! Cucumber-rs integration tests for the `test-coverage` command family.
//!
//! Wires the behavior-contract feature files at
//! `specs/apps/rhino/behavior/rhino-cli/gherkin/test-coverage/` to step
//! definitions. `test-coverage validate` is the only real CLI verb (see
//! `apps/rhino-cli/src/cli.rs`'s `TestCoverageCommands`), so its scenarios
//! drive the compiled `rhino-cli` binary as a subprocess, exactly like the
//! other cucumber suites in this directory. `diff` and `merge` are not
//! exposed as CLI subcommands (per tech-docs §1.5 — diff/merge logic lives in
//! `application::testcoverage` but is not wired to a command, matching the
//! live surface in the sibling repos), so their scenarios assert directly
//! against the internal `application::testcoverage::{diff,merge}` functions
//! in-process instead of inventing a non-existent CLI verb.

// Test step-definition scaffolding: private World state and step fns are
// self-documenting via their #[given]/#[when]/#[then] gherkin strings.
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::needless_pass_by_value)] // cucumber-rs binds regex captures by value

use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::Output;
use std::sync::Mutex;

use assert_cmd::cargo::cargo_bin;
use cucumber::{World as _, given, then, when};
use rhino_cli::application::testcoverage::diff::{DiffCoverageOptions, compute_diff_coverage};
use rhino_cli::application::testcoverage::merge::{
    CoverageMap, merge_coverage_maps, result_from_coverage_map, to_coverage_map, write_lcov,
};
use rhino_cli::application::testcoverage::types::Result as CoverageResult;
use tempfile::TempDir;

/// Shared scenario state spanning all three `test-coverage` feature files.
///
/// `test-coverage validate` scenarios drive the compiled binary as a
/// subprocess (state lives in `output`); `diff`/`merge` scenarios call the
/// internal functions in-process (state lives in `ip_success`/`ip_error` plus
/// the feature-specific fields below). The two exit-code assertions shared
/// verbatim across all three feature files (`then_exit_ok`/`then_exit_fail`)
/// read whichever mechanism populated its state.
#[derive(cucumber::World)]
#[world(init = Self::new)]
struct TestCoverageWorld {
    work: TempDir,

    // --- subprocess state (test-coverage validate scenarios) ---
    output: Option<Output>,

    // --- in-process state (diff/merge scenarios) ---
    ip_success: Option<bool>,
    ip_error: Option<String>,

    // --- validate-scenario fixture state ---
    coverage_file: String,
    multi_file_paths: Vec<String>,
    excluded_path: String,

    // --- merge-scenario fixture state ---
    merge_inputs: Vec<PathBuf>,
    merge_output_path: Option<PathBuf>,

    // --- diff-scenario fixture state ---
    diff_coverage_file: Option<PathBuf>,
    diff_exclude_patterns: Vec<String>,
    diff_result: Option<CoverageResult>,
}

impl std::fmt::Debug for TestCoverageWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TestCoverageWorld")
            .field("coverage_file", &self.coverage_file)
            .finish_non_exhaustive()
    }
}

impl TestCoverageWorld {
    fn new() -> Self {
        let work = TempDir::new().expect("temp workspace");
        init_git_repo(work.path());
        Self {
            work,
            output: None,
            ip_success: None,
            ip_error: None,
            coverage_file: String::new(),
            multi_file_paths: Vec::new(),
            excluded_path: String::new(),
            merge_inputs: Vec::new(),
            merge_output_path: None,
            diff_coverage_file: None,
            diff_exclude_patterns: Vec::new(),
            diff_result: None,
        }
    }

    /// Writes `content` at repo-relative path `rel` inside the fixture
    /// workspace, creating parent directories as needed, and returns the
    /// absolute path written.
    fn write(&self, rel: &str, content: &str) -> PathBuf {
        let p = self.work.path().join(rel);
        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent).expect("mk fixture dir");
        }
        std::fs::write(&p, content).expect("write fixture");
        p
    }

    /// Writes `content` at `rel` and `git add`s it (repo-relative `rel` used
    /// as the git-reported path).
    fn stage_new_file(&self, rel: &str, num_lines: usize) {
        let mut content = String::new();
        for i in 1..=num_lines {
            let _ = writeln!(content, "line{i}");
        }
        self.write(rel, &content);
        run_git(self.work.path(), &["add", rel]);
    }

    /// Writes a synthetic Go `cover.out` file with `covered` distinct
    /// single-line hit blocks followed by `missed` distinct single-line
    /// unhit blocks, all against a placeholder file path. No `go.mod` exists
    /// in the fixture root, so `compute_go_result`'s source-line lookup fails
    /// and every recorded line is counted directly (no non-code filtering).
    fn write_go_cover(&self, rel: &str, covered: usize, missed: usize) -> PathBuf {
        self.write_go_cover_for_path(rel, "fake/file.go", covered, missed)
    }

    /// Same as [`Self::write_go_cover`] but against an explicit `path`
    /// (matching a git-diff hunk's file path) instead of a placeholder.
    fn write_go_cover_for_path(
        &self,
        rel: &str,
        path: &str,
        covered: usize,
        missed: usize,
    ) -> PathBuf {
        let mut content = String::from("mode: set\n");
        for i in 1..=covered {
            let _ = writeln!(content, "{path}:{i}.1,{i}.2 1 1");
        }
        for i in 1..=missed {
            let line = covered + i;
            let _ = writeln!(content, "{path}:{line}.1,{line}.2 1 0");
        }
        self.write(rel, &content)
    }

    /// Writes a single-file LCOV `.info` fixture recording `covered` hit
    /// lines followed by `missed` unhit lines under `SF:{path_in_report}`.
    fn write_lcov_single(
        &self,
        rel: &str,
        path_in_report: &str,
        covered: usize,
        missed: usize,
    ) -> PathBuf {
        let mut content = String::from("TN:\n");
        let _ = writeln!(content, "SF:{path_in_report}");
        for i in 1..=covered {
            let _ = writeln!(content, "DA:{i},3");
        }
        for i in 1..=missed {
            let line = covered + i;
            let _ = writeln!(content, "DA:{line},0");
        }
        content.push_str("end_of_record\n");
        self.write(rel, &content)
    }

    /// Writes a multi-file LCOV `.info` fixture, one `SF:` section per entry
    /// in `files` (path, covered-line-count, missed-line-count).
    fn write_lcov_multi(&self, rel: &str, files: &[(&str, usize, usize)]) -> PathBuf {
        let mut content = String::new();
        for (path, covered, missed) in files {
            content.push_str("TN:\n");
            let _ = writeln!(content, "SF:{path}");
            for i in 1..=*covered {
                let _ = writeln!(content, "DA:{i},3");
            }
            for i in 1..=*missed {
                let line = covered + i;
                let _ = writeln!(content, "DA:{line},0");
            }
            content.push_str("end_of_record\n");
        }
        self.write(rel, &content)
    }

    /// Writes a single-class Cobertura XML fixture recording `covered` hit
    /// lines followed by `missed` unhit lines, no branch data.
    fn write_cobertura_single(
        &self,
        rel: &str,
        path_in_report: &str,
        covered: usize,
        missed: usize,
    ) -> PathBuf {
        let mut lines = String::new();
        for i in 1..=covered {
            let _ = writeln!(lines, "<line number=\"{i}\" hits=\"3\" branch=\"false\"/>");
        }
        for i in 1..=missed {
            let line = covered + i;
            let _ = writeln!(
                lines,
                "<line number=\"{line}\" hits=\"0\" branch=\"false\"/>"
            );
        }
        let xml = format!(
            "<?xml version=\"1.0\"?>\n<coverage><packages><package name=\"pkg\">\
             <classes><class filename=\"{path_in_report}\"><lines>\n{lines}</lines>\
             </class></classes></package></packages></coverage>\n"
        );
        self.write(rel, &xml)
    }

    /// Writes a single Cobertura `<line>` with partial branch coverage
    /// (`condition-coverage="50% (1/2)"`) — hit but not all branches taken.
    fn write_cobertura_partial_branch(&self, rel: &str) -> PathBuf {
        let xml = "<?xml version=\"1.0\"?>\n<coverage><packages><package name=\"pkg\">\
                   <classes><class filename=\"src/foo.py\"><lines>\
                   <line number=\"10\" hits=\"5\" branch=\"true\" condition-coverage=\"50% (1/2)\"/>\
                   </lines></class></classes></package></packages></coverage>\n";
        self.write(rel, xml)
    }

    /// Runs the compiled `rhino-cli` binary's `test-coverage validate`
    /// subcommand against `self.coverage_file` with the given `threshold`
    /// and any `extra` flags (e.g. `--per-file`, `--exclude <pattern>`).
    fn exec_validate(&mut self, threshold: &str, extra: &[String]) {
        let mut cmd = std::process::Command::new(cargo_bin("rhino-cli"));
        cmd.args(["test-coverage", "validate", &self.coverage_file, threshold]);
        cmd.args(extra);
        cmd.arg("--no-color");
        cmd.current_dir(self.work.path());
        self.output = Some(cmd.output().expect("run rhino-cli"));
    }

    /// Same as [`Self::exec_validate`] but requests JSON output via the
    /// global `-o json` flag.
    fn exec_validate_json(&mut self, threshold: &str) {
        let mut cmd = std::process::Command::new(cargo_bin("rhino-cli"));
        cmd.args([
            "-o",
            "json",
            "test-coverage",
            "validate",
            &self.coverage_file,
            threshold,
        ]);
        cmd.arg("--no-color");
        cmd.current_dir(self.work.path());
        self.output = Some(cmd.output().expect("run rhino-cli"));
    }

    fn stdout(&self) -> String {
        String::from_utf8_lossy(&self.output.as_ref().expect("ran").stdout).into_owned()
    }

    /// Concatenates stdout and stderr, mirroring how a developer watching the
    /// terminal sees both streams interleaved.
    fn combined_output(&self) -> String {
        let out = self.output.as_ref().expect("ran");
        format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        )
    }

    /// `true` when the scenario's command (subprocess or in-process) is
    /// considered to have succeeded.
    fn succeeded(&self) -> bool {
        self.ip_success.unwrap_or_else(|| {
            self.output
                .as_ref()
                .expect("a command ran")
                .status
                .success()
        })
    }

    /// Diagnostic text attached to exit-code assertion failures.
    fn diagnostics(&self) -> String {
        self.output.as_ref().map_or_else(
            || format!("in-process error: {:?}", self.ip_error),
            |out| {
                format!(
                    "stdout: {}\nstderr: {}",
                    String::from_utf8_lossy(&out.stdout),
                    String::from_utf8_lossy(&out.stderr)
                )
            },
        )
    }

    /// Runs `compute_diff_coverage` in-process against `self.diff_coverage_file`
    /// and `self.diff_exclude_patterns`, recording the result in
    /// `self.diff_result` and the pass/fail signal in `self.ip_success`.
    fn run_diff(&mut self, threshold: f64) {
        let coverage_file = self
            .diff_coverage_file
            .clone()
            .map_or_else(String::new, |p| p.to_str().expect("utf8 path").to_string());
        let opts = DiffCoverageOptions {
            coverage_file,
            base: String::new(),
            staged: true,
            threshold,
            per_file: true,
            exclude_patterns: self.diff_exclude_patterns.clone(),
        };
        let dir = self.work.path().to_path_buf();
        match compute_diff_coverage_in(&dir, &opts) {
            Ok(result) => {
                self.ip_success = Some(result.passed);
                self.diff_result = Some(result);
            }
            Err(e) => {
                self.ip_error = Some(e.to_string());
                self.ip_success = Some(false);
            }
        }
    }
}

fn run_git(dir: &Path, args: &[&str]) {
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
/// resolves here (needed by `test-coverage validate` subprocess scenarios).
fn init_git_repo(dir: &Path) {
    run_git(dir, &["init", "-q"]);
    std::fs::write(dir.join("seed.txt"), "seed\n").expect("seed file");
    run_git(dir, &["add", "-A"]);
    run_git(dir, &["commit", "-q", "-m", "seed"]);
}

/// Parses each path in `paths` as a coverage report and unions them via
/// `merge_coverage_maps` — the in-process stand-in for a hypothetical
/// `test-coverage merge` command's core logic.
fn merge_all(paths: &[PathBuf]) -> anyhow::Result<CoverageMap> {
    let maps = paths
        .iter()
        .map(|p| to_coverage_map(p.to_str().expect("utf8 path")))
        .collect::<anyhow::Result<Vec<CoverageMap>>>()?;
    Ok(merge_coverage_maps(&maps))
}

/// Process-wide mutex serializing calls to `compute_diff_coverage`, whose
/// `get_git_diff` shells to `git` in the process cwd with no directory
/// override parameter. Mirrors the crate's own (inaccessible from here —
/// `pub(crate)`) `test_support::CwdLock`: cucumber-rs runs scenarios
/// concurrently (up to 64 by default), so the process cwd is contended global
/// state that must be serialized and restored around each use.
static DIFF_CWD_GUARD: Mutex<()> = Mutex::new(());

/// Runs `compute_diff_coverage` with the process cwd temporarily pointed at
/// `dir`, holding [`DIFF_CWD_GUARD`] for the duration and restoring the
/// original cwd afterward.
fn compute_diff_coverage_in(
    dir: &Path,
    opts: &DiffCoverageOptions,
) -> anyhow::Result<CoverageResult> {
    let _guard = DIFF_CWD_GUARD
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let original = std::env::current_dir().expect("cwd readable");
    std::env::set_current_dir(dir).expect("cwd set to fixture");
    let result = compute_diff_coverage(opts);
    std::env::set_current_dir(original).expect("cwd restored");
    result
}

// ===========================================================================
// Given steps — test-coverage validate
// ===========================================================================

#[given(regex = r"^a Go coverage file recording (\d+)% line coverage$")]
fn given_go_coverage_pct(w: &mut TestCoverageWorld, pct: String) {
    let pct: usize = pct.parse().expect("pct");
    w.write_go_cover("coverage/cover.out", pct, 100 - pct);
    w.coverage_file = "coverage/cover.out".to_string();
}

#[given(regex = r"^an LCOV coverage file recording (\d+)% line coverage$")]
fn given_lcov_coverage_pct(w: &mut TestCoverageWorld, pct: String) {
    let pct: usize = pct.parse().expect("pct");
    w.write_lcov_single("coverage/lcov.info", "src/foo.rs", pct, 100 - pct);
    w.coverage_file = "coverage/lcov.info".to_string();
}

#[given("an LCOV coverage file with multiple source files")]
fn given_lcov_multi_files(w: &mut TestCoverageWorld) {
    w.write_lcov_multi(
        "coverage/multi.info",
        &[("src/a.rs", 10, 0), ("src/b.rs", 9, 1)],
    );
    w.coverage_file = "coverage/multi.info".to_string();
    w.multi_file_paths = vec!["src/a.rs".to_string(), "src/b.rs".to_string()];
}

#[given(regex = r"^a Cobertura XML coverage file recording (\d+)% line coverage$")]
fn given_cobertura_pct(w: &mut TestCoverageWorld, pct: String) {
    let pct: usize = pct.parse().expect("pct");
    w.write_cobertura_single("coverage/cobertura.xml", "src/foo.py", pct, 100 - pct);
    w.coverage_file = "coverage/cobertura.xml".to_string();
}

#[given("a Cobertura XML coverage file with partial branch coverage")]
fn given_cobertura_partial(w: &mut TestCoverageWorld) {
    w.write_cobertura_partial_branch("coverage/cobertura-partial.xml");
    w.coverage_file = "coverage/cobertura-partial.xml".to_string();
}

#[given("no coverage file exists at the specified path")]
fn given_missing_coverage_file(w: &mut TestCoverageWorld) {
    w.coverage_file = "coverage/missing-coverage.info".to_string();
}

// ===========================================================================
// When steps — test-coverage validate
// ===========================================================================

#[when(regex = r"^the developer runs test-coverage validate with an (\d+)% threshold$")]
fn when_validate_threshold(w: &mut TestCoverageWorld, pct: String) {
    w.exec_validate(&pct, &[]);
}

#[when(
    regex = r"^the developer runs test-coverage validate with an (\d+)% threshold requesting JSON output$"
)]
fn when_validate_threshold_json(w: &mut TestCoverageWorld, pct: String) {
    w.exec_validate_json(&pct);
}

#[when(
    regex = r"^the developer runs test-coverage validate with an (\d+)% threshold and per-file flag$"
)]
fn when_validate_threshold_per_file(w: &mut TestCoverageWorld, pct: String) {
    w.exec_validate(&pct, &["--per-file".to_string()]);
}

#[when("the developer runs test-coverage validate with exclusion of a source file")]
fn when_validate_exclusion(w: &mut TestCoverageWorld) {
    let excluded = w
        .multi_file_paths
        .last()
        .cloned()
        .expect("multi-file fixture set up");
    w.excluded_path.clone_from(&excluded);
    w.exec_validate(
        "0",
        &["--per-file".to_string(), "--exclude".to_string(), excluded],
    );
}

// ===========================================================================
// Then steps — shared exit-code assertions (all three feature files)
// ===========================================================================

#[then("the command exits successfully")]
fn then_exit_ok(w: &mut TestCoverageWorld) {
    assert!(w.succeeded(), "{}", w.diagnostics());
}

#[then("the command exits with a failure code")]
fn then_exit_fail(w: &mut TestCoverageWorld) {
    assert!(!w.succeeded(), "{}", w.diagnostics());
}

// ===========================================================================
// Then steps — test-coverage validate
// ===========================================================================

#[then("the output reports the measured coverage percentage")]
fn then_reports_measured_pct(w: &mut TestCoverageWorld) {
    let out = w.stdout();
    assert!(
        out.contains("Line coverage:") && out.contains('%'),
        "got: {out}"
    );
}

#[then("the output indicates the coverage passes the threshold")]
fn then_indicates_pass(w: &mut TestCoverageWorld) {
    let out = w.stdout();
    assert!(out.contains("PASS:"), "got: {out}");
}

#[then("the output indicates the coverage fails the threshold")]
fn then_indicates_fail(w: &mut TestCoverageWorld) {
    let out = w.stdout();
    assert!(out.contains("FAIL:"), "got: {out}");
}

#[then("the output is valid JSON")]
fn then_output_valid_json(w: &mut TestCoverageWorld) {
    let out = w.stdout();
    let parsed = serde_json::from_str::<serde_json::Value>(out.trim());
    assert!(parsed.is_ok(), "invalid JSON: {parsed:?}\ngot: {out}");
}

#[then("the JSON includes the coverage percentage and pass/fail status")]
fn then_json_includes_fields(w: &mut TestCoverageWorld) {
    let out = w.stdout();
    let v: serde_json::Value = serde_json::from_str(out.trim()).expect("valid JSON");
    assert!(v.get("pct").is_some(), "got: {out}");
    assert!(v.get("passed").is_some(), "got: {out}");
    assert!(v.get("status").is_some(), "got: {out}");
}

#[then("the output contains per-file coverage breakdown")]
fn then_output_contains_per_file(w: &mut TestCoverageWorld) {
    let out = w.stdout();
    assert!(out.contains("Per-file coverage"), "got: {out}");
}

#[then("the output does not contain the excluded file")]
fn then_output_excludes_file(w: &mut TestCoverageWorld) {
    let out = w.stdout();
    assert!(!out.contains(&w.excluded_path), "got: {out}");
}

#[then("the output describes the missing file")]
fn then_output_describes_missing_file(w: &mut TestCoverageWorld) {
    let out = w.combined_output();
    assert!(out.contains("file not found"), "got: {out}");
    assert!(out.contains("missing-coverage.info"), "got: {out}");
}

// ===========================================================================
// Given steps — test-coverage merge (in-process)
// ===========================================================================

#[given("two LCOV coverage files with different source files")]
fn given_merge_different_files(w: &mut TestCoverageWorld) {
    w.merge_inputs = vec![
        w.write_lcov_single("merge/a.info", "src/a.rs", 5, 0),
        w.write_lcov_single("merge/b.info", "src/b.rs", 5, 0),
    ];
}

#[given("two LCOV coverage files with high coverage")]
fn given_merge_high_coverage(w: &mut TestCoverageWorld) {
    w.merge_inputs = vec![
        w.write_lcov_single("merge/a.info", "src/a.rs", 9, 1),
        w.write_lcov_single("merge/b.info", "src/b.rs", 9, 1),
    ];
}

#[given("two LCOV coverage files with low coverage")]
fn given_merge_low_coverage(w: &mut TestCoverageWorld) {
    w.merge_inputs = vec![
        w.write_lcov_single("merge/a.info", "src/a.rs", 4, 6),
        w.write_lcov_single("merge/b.info", "src/b.rs", 4, 6),
    ];
}

// ===========================================================================
// When steps — test-coverage merge (in-process)
// ===========================================================================

#[when("the developer runs test-coverage merge with an output file")]
fn when_merge_with_output(w: &mut TestCoverageWorld) {
    let out_path = w.work.path().join("merge/out.info");
    let outcome = merge_all(&w.merge_inputs).and_then(|merged| write_lcov(&merged, &out_path));
    match outcome {
        Ok(()) => {
            w.merge_output_path = Some(out_path);
            w.ip_success = Some(true);
        }
        Err(e) => {
            w.ip_error = Some(e.to_string());
            w.ip_success = Some(false);
        }
    }
}

#[when(regex = r"^the developer runs test-coverage merge with validation at (\d+)% threshold$")]
fn when_merge_with_validation(w: &mut TestCoverageWorld, pct: String) {
    let threshold: f64 = pct.parse().expect("threshold");
    match merge_all(&w.merge_inputs) {
        Ok(merged) => {
            let result = result_from_coverage_map(&merged, threshold);
            w.ip_success = Some(result.passed);
        }
        Err(e) => {
            w.ip_error = Some(e.to_string());
            w.ip_success = Some(false);
        }
    }
}

// ===========================================================================
// Then steps — test-coverage merge (in-process)
// ===========================================================================

#[then("the merged output file exists in LCOV format")]
fn then_merge_output_lcov(w: &mut TestCoverageWorld) {
    let path = w.merge_output_path.clone().expect("merge output recorded");
    let content = std::fs::read_to_string(&path).expect("read merged output");
    assert!(content.contains("SF:src/a.rs"), "got: {content}");
    assert!(content.contains("SF:src/b.rs"), "got: {content}");
    assert!(content.contains("end_of_record"), "got: {content}");
}

// ===========================================================================
// Given steps — test-coverage diff (in-process)
// ===========================================================================

#[given("a coverage file and no git changes")]
fn given_diff_no_changes(w: &mut TestCoverageWorld) {
    let p = w.write_go_cover("diffcov/cover.out", 5, 0);
    w.diff_coverage_file = Some(p);
}

#[given("a coverage file where all changed lines are covered")]
fn given_diff_all_covered(w: &mut TestCoverageWorld) {
    w.stage_new_file("changed/new_code.go", 5);
    let p = w.write_go_cover_for_path("diffcov/cover.out", "changed/new_code.go", 5, 0);
    w.diff_coverage_file = Some(p);
}

#[given("a coverage file where some changed lines are missed")]
fn given_diff_some_missed(w: &mut TestCoverageWorld) {
    w.stage_new_file("changed/new_code.go", 5);
    let p = w.write_go_cover_for_path("diffcov/cover.out", "changed/new_code.go", 1, 4);
    w.diff_coverage_file = Some(p);
}

#[given("a coverage file and changes in excluded files")]
fn given_diff_excluded_changes(w: &mut TestCoverageWorld) {
    w.stage_new_file("excluded/skip.go", 5);
    w.stage_new_file("included/keep.go", 5);
    let mut content = String::from("mode: set\n");
    let _ = writeln!(content, "excluded/skip.go:1.1,5.2 1 0");
    let _ = writeln!(content, "included/keep.go:1.1,5.2 1 1");
    let p = w.write("diffcov/cover.out", &content);
    w.diff_coverage_file = Some(p);
    w.diff_exclude_patterns = vec!["excluded/*".to_string()];
}

// ===========================================================================
// When steps — test-coverage diff (in-process)
// ===========================================================================

#[when("the developer runs test-coverage diff")]
fn when_diff_default(w: &mut TestCoverageWorld) {
    w.run_diff(0.0);
}

#[when("the developer runs test-coverage diff with a threshold")]
fn when_diff_with_threshold(w: &mut TestCoverageWorld) {
    w.run_diff(50.0);
}

#[when("the developer runs test-coverage diff with a high threshold")]
fn when_diff_with_high_threshold(w: &mut TestCoverageWorld) {
    w.run_diff(90.0);
}

#[when("the developer runs test-coverage diff with exclusion")]
fn when_diff_with_exclusion(w: &mut TestCoverageWorld) {
    w.run_diff(0.0);
}

// ===========================================================================
// Then steps — test-coverage diff (in-process)
// ===========================================================================

#[then("the output reports 100% coverage")]
fn then_diff_reports_100(w: &mut TestCoverageWorld) {
    let r = w.diff_result.as_ref().expect("diff computed");
    assert!((r.pct - 100.0).abs() < 0.001, "pct={}", r.pct);
}

#[then("the excluded files do not affect the diff coverage result")]
fn then_diff_excluded_no_effect(w: &mut TestCoverageWorld) {
    let r = w.diff_result.as_ref().expect("diff computed");
    assert!((r.pct - 100.0).abs() < 0.001, "pct={}", r.pct);
    assert!(
        !r.files.iter().any(|f| f.path.starts_with("excluded/")),
        "files: {:?}",
        r.files
    );
}

#[tokio::main]
async fn main() {
    TestCoverageWorld::cucumber()
        .fail_on_skipped()
        .run_and_exit(feature_dir())
        .await;
}

fn feature_dir() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .join("../../specs/apps/rhino/behavior/rhino-cli/gherkin/test-coverage")
        .canonicalize()
        .expect("feature dir resolvable")
}
