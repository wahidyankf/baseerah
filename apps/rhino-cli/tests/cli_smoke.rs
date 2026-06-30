//! Smoke tests for the `rhino-cli` binary — verifies the binary builds and responds.
use assert_cmd::Command;
use predicates::str::contains;

fn cmd() -> Command {
    Command::cargo_bin("rhino-cli").expect("binary not found")
}

#[test]
fn no_args_exits_success() {
    cmd().assert().success();
}

#[test]
fn help_flag_exits_success() {
    cmd().arg("--help").assert().success();
}

#[test]
fn say_flag_echoes_message() {
    cmd()
        .args(["--say", "hello world"])
        .assert()
        .success()
        .stdout(contains("hello world"));
}

#[test]
fn invalid_output_format_exits_failure() {
    cmd()
        .args(["--output", "not-a-valid-format", "doctor"])
        .assert()
        .failure();
}

#[test]
fn unknown_subcommand_exits_failure() {
    cmd().arg("not-a-real-command").assert().failure();
}

#[test]
fn gherkin_keyword_cardinality_subcommand_exists() {
    cmd()
        .args(["specs", "gherkin-cardinality", "validate", "--help"])
        .assert()
        .success()
        .stdout(contains("Usage"));
}
