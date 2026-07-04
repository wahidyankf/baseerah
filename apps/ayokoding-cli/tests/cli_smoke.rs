//! Smoke tests for the `ayokoding-cli` binary.
use assert_cmd::Command;
use predicates::str::contains;
use tempfile::TempDir;

fn cmd() -> Command {
    Command::cargo_bin("ayokoding-cli").expect("binary not found")
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
