//! Smoke tests for the `ose-cli` binary.
use assert_cmd::Command;
use predicates::str::contains;
use tempfile::TempDir;

fn cmd() -> Command {
    Command::cargo_bin("ose-cli").expect("binary not found")
}

#[test]
fn help_flag_exits_success() {
    cmd().arg("--help").assert().success();
}

#[test]
fn unknown_subcommand_exits_failure() {
    cmd().arg("not-a-real-command").assert().failure();
}

#[test]
fn invalid_output_format_exits_failure() {
    cmd()
        .args(["--output", "bad-format", "links", "check"])
        .assert()
        .failure();
}

#[test]
// @covers specs/apps/ose/behavior/ose-cli/gherkin/links/links-check.feature:A content directory with all valid internal links passes validation
fn links_check_passes_on_empty_dir() {
    let dir = TempDir::new().expect("tempdir");
    cmd()
        .args([
            "links",
            "check",
            "--content",
            dir.path().to_str().expect("valid path"),
        ])
        .assert()
        .success();
}

#[test]
// @covers specs/apps/ose/behavior/ose-cli/gherkin/links/links-check.feature:External URLs are not validated
fn links_check_ignores_external_urls() {
    let dir = TempDir::new().expect("tempdir");
    std::fs::write(
        dir.path().join("index.md"),
        "[external](https://example.com/page)\n[mail](mailto:user@example.com)\n",
    )
    .expect("write file");
    cmd()
        .args([
            "links",
            "check",
            "--content",
            dir.path().to_str().expect("valid path"),
        ])
        .assert()
        .success();
}

#[test]
// @covers specs/apps/ose/behavior/ose-cli/gherkin/links/links-check.feature:A broken internal link is detected and reported
fn links_check_reports_broken_link() {
    let dir = TempDir::new().expect("tempdir");
    std::fs::write(dir.path().join("index.md"), "[broken](/does-not-exist)\n").expect("write file");
    cmd()
        .args([
            "links",
            "check",
            "--content",
            dir.path().to_str().expect("valid path"),
        ])
        .assert()
        .failure()
        .stdout(contains("does-not-exist"));
}

#[test]
// @covers specs/apps/ose/behavior/ose-cli/gherkin/links/links-check.feature:JSON output produces structured results
fn links_check_json_output_is_valid() {
    let dir = TempDir::new().expect("tempdir");
    let output = cmd()
        .args([
            "links",
            "check",
            "--content",
            dir.path().to_str().expect("valid path"),
            "-o",
            "json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let s = String::from_utf8(output).expect("valid utf8");
    let v: serde_json::Value = serde_json::from_str(&s).expect("valid JSON");
    assert!(v.get("status").is_some());
    assert!(v.get("checked").is_some());
    assert!(v.get("broken_links").is_some());
}

#[test]
fn links_check_markdown_output_has_headings() {
    let dir = TempDir::new().expect("tempdir");
    cmd()
        .args([
            "links",
            "check",
            "--content",
            dir.path().to_str().expect("valid path"),
            "-o",
            "markdown",
        ])
        .assert()
        .success()
        .stdout(contains("# Link Check Report"))
        .stdout(contains("## Summary"));
}

#[test]
fn links_check_quiet_mode_no_output_on_success() {
    let dir = TempDir::new().expect("tempdir");
    cmd()
        .args([
            "links",
            "check",
            "--content",
            dir.path().to_str().expect("valid path"),
            "--quiet",
        ])
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn links_check_nonexistent_dir_exits_failure() {
    cmd()
        .args([
            "links",
            "check",
            "--content",
            "/nonexistent/path/that/does/not/exist",
        ])
        .assert()
        .failure();
}
