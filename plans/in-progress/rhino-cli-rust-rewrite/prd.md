# Product Requirements Document — Rewrite rhino-cli to Rust

## Product Overview

`rhino-cli` is a command-line utility that exposes ~30 commands for repository hygiene, agent management, governance audits, documentation validation, specs validation, doctor (toolchain probing), and coverage validation. It is invoked by humans (`nx run`), by Husky git hooks, by GitHub Actions workflows, and by other apps' `test:quick` / `spec-coverage` Nx targets.

This plan delivers a Rust implementation that satisfies the same observable contract — same flags, same exit codes, same stdout/stderr — defined by the Gherkin scenarios in [`specs/apps/rhino/behavior/cli/gherkin/`](../../../specs/apps/rhino/behavior/cli/gherkin/) [Repo-grounded].

## Personas

- **Developer Diane** — invokes `rhino-cli` through `nx run` and via pre-commit/pre-push hooks. Cares that the binary is fast, gives correct exit codes, and never breaks her existing workflow.
- **CI Runner Carl** — runs `nx affected -t test:quick spec-coverage` on every push. Cares that targets remain cacheable, that workflows still pass, and that no setup step is more brittle than the Go version.
- **Maintainer Mona** — adds new commands and features to `rhino-cli`. Cares that the Rust implementation has clear module boundaries, type-checked invariants, and a low barrier to porting future commands.

## User stories

- **US-1 — As Developer Diane**, I want every `nx run` command I previously ran against `rhino-cli` (Go) to keep working with identical observable output, so my muscle memory and scripts don't break.
- **US-2 — As Developer Diane**, I want `npm run doctor -- --fix` to install the Rust toolchain if it's missing, so the first-run setup is identical to today's first-run setup with Go.
- **US-3 — As Developer Diane**, I want pre-commit and pre-push hooks to remain fast on a warm cache (≤ Go baseline + 20%), so I'm not penalized for the tooling change.
- **US-4 — As CI Runner Carl**, I want every existing GitHub Actions workflow to keep passing without changes beyond a `setup-rust-toolchain` Setup step, so the migration is mechanical.
- **US-5 — As CI Runner Carl**, I want the `test:quick` Nx target gate on every other app to keep returning the same pass/fail result for the same code, so the test:quick contract is preserved.
- **US-6 — As Maintainer Mona**, I want the Rust modules to mirror the Go `internal/` layout 1:1, so I can locate equivalent code by name.
- **US-7 — As Maintainer Mona**, I want every Gherkin `.feature` file to drive a `cucumber-rs` test at both unit (mocked I/O) and integration (real `/tmp`) levels, so the BDD spec-to-test mapping from [`repo-governance/development/infra/bdd-spec-test-mapping.md`](../../../repo-governance/development/infra/bdd-spec-test-mapping.md) [Repo-grounded] is preserved across language transitions.

## Gherkin Acceptance Criteria

### Critical-path command parity

```gherkin
Feature: test-coverage validate parity

  Scenario: Rust validator returns identical pass/fail for a passing cover.out
    Given a Go cover.out file with measured line coverage 92.4%
    And the threshold argument is 90
    When I run `rhino-cli test-coverage validate <path> 90` against the Rust binary
    And I run `rhino-cli test-coverage validate <path> 90` against the Go binary
    Then both binaries exit with code 0
    And both binaries print the same "Line coverage: X% (...)" summary
    And both binaries print the same "PASS: X% >= 90% threshold" line

  Scenario: Rust validator returns identical pass/fail for a failing cover.out
    Given a Go cover.out file with measured line coverage 85.0%
    And the threshold argument is 90
    When I run the validator against both binaries
    Then both binaries exit with code 1
    And both binaries print "Error: coverage 85.00% is below threshold 90%" to stderr

  Scenario: Rust validator accepts LCOV, JaCoCo, and Cobertura formats
    Given a coverage file in <format> format
    When I run `rhino-cli test-coverage validate <path> <threshold>` against the Rust binary
    Then the binary auto-detects the format and computes the same value as the Go binary
    Examples:
      | format    |
      | lcov      |
      | jacoco    |
      | cobertura |
```

```gherkin
Feature: spec-coverage validate parity

  Scenario: Rust spec-coverage validator returns identical results for a fully-covered package
    Given the directory `apps/ayokoding-cli` whose Go tests reference every scenario in `specs/apps/ayokoding/behavior/cli/gherkin/`
    When I run `rhino-cli spec-coverage validate specs/apps/ayokoding/behavior/cli/gherkin apps/ayokoding-cli --shared-steps` against both binaries
    Then both exit with code 0
    And both print the same per-scenario coverage table

  Scenario: Rust spec-coverage validator flags an uncovered scenario
    Given a feature file containing a scenario with no matching `// Scenario: <title>` comment in the test files
    When I run the validator against both binaries
    Then both exit with code 1
    And both name the uncovered scenario in stderr
```

### Output format parity

```gherkin
Feature: Output format parity across all commands

  Scenario Outline: Rust binary produces identical output for -o flag
    Given the command `rhino-cli <command> <args>`
    When I run it with `-o text` against both binaries
    Then stdout, stderr, and exit code match byte-for-byte
    When I run it with `-o json` against both binaries
    Then stdout is valid JSON and matches byte-for-byte
    When I run it with `-o markdown` against both binaries
    Then stdout matches byte-for-byte
    When I run it with `-o xml` against both binaries
    Then both exit with code 1
    And both print `unknown output format "xml": must be text, json, or markdown` to stderr

    Examples:
      | command                          | args                                       |
      | repo-governance audit            |                                            |
      | repo-governance emoji-audit      |                                            |
      | docs validate-frontmatter        |                                            |
      | docs validate-naming             |                                            |
      | agents validate-naming           |                                            |
      | workflows validate-naming        |                                            |
      | test-coverage validate           | apps/rhino-cli/cover.out 90                |
      | spec-coverage validate           | specs/apps/rhino/behavior/cli/gherkin apps/rhino-cli |
```

### Doctor toolchain parity

```gherkin
Feature: Doctor toolchain probe matches Go binary

  Scenario: Rust doctor detects same toolchain set with identical pass/fail counts
    Given a workstation with all required toolchains installed at supported versions
    When I run `rhino-cli doctor` against both binaries
    Then both exit with code 0
    And both report the same number of PASS / WARN / FAIL tools

  Scenario: Rust doctor --fix installs the same toolchains as Go doctor --fix
    Given a workstation missing `rustup`
    When I run `rhino-cli doctor --fix --dry-run` against both binaries
    Then both print the same install plan
```

### Husky hook parity

```gherkin
Feature: Pre-commit and pre-push hooks invoke the Rust binary

  Scenario: pre-commit `git pre-commit` command runs against the Rust binary
    Given the pre-commit hook calls `rhino-cli git pre-commit`
    When a developer commits a change with mixed JS/Go/markdown files
    Then the hook completes successfully
    And the formatter/linter pipeline runs identically to the Go-binary baseline

  Scenario: pre-push validate:* Nx targets pass against the Rust binary
    Given the pre-push hook runs `nx affected -t typecheck lint test:quick spec-coverage`
    When a developer pushes changes affecting rhino-cli
    Then all targets pass
    And the pre-push wall-clock time is within +20% of the Go-binary baseline on a warm cache
```

### Cutover invariants

```gherkin
Feature: Archival cutover leaves no Go-binary references

  Scenario: No app's project.json shells out to the Go rhino-cli after cutover
    When I run `grep -rE "apps/rhino-cli/main\.go|go run -C apps/rhino-cli" .github/workflows/ .husky/ apps/*/project.json`
    Then the command produces no matches

  Scenario: Go implementation is archived with full git history
    Given the cutover commit lands on `main`
    Then `archived/rhino-cli-go/` exists
    And `archived/README.md` lists the archival date and successor link
    And `git log --follow archived/rhino-cli-go/main.go` shows the original history
    And `apps/rhino-cli/Cargo.toml` exists in place of the original Go module
```

## Product Scope

### In scope

- Every command currently delivered by the Go `rhino-cli` (verified by `apps/rhino-cli/cmd/*.go` excluding `*_test.go` and `*.integration_test.go`) [Repo-grounded].
- Every Gherkin `.feature` file under `specs/apps/rhino/behavior/cli/gherkin/` [Repo-grounded] (including the `specs/` subdirectory containing `validate-adoption.feature`, `validate-counts.feature`, `validate-links.feature`, `validate-tree.feature`).
- Every `apps/*/project.json` reference to the Go binary — caller-graph migration.
- Husky hooks at `.husky/pre-commit` and `.husky/pre-push` [Repo-grounded].
- GitHub Actions workflows that invoke the Go binary [Repo-grounded — `.github/workflows/pr-validate-links.yml`].
- Archival of `apps/rhino-cli/` (Go) to `archived/rhino-cli-go/` per the existing archival pattern [Repo-grounded — `archived/README.md` table format].

### Out of scope

- Rewriting `apps/ayokoding-cli` or `apps/ose-cli` (the two Go siblings stay Go).
- Touching `libs/golang-commons` (kept for the two siblings).
- Migrating `ose-primer/apps/rhino-cli` (downstream sync; separate follow-up plan).
- New features, new commands, new flags, or behavior improvements on existing commands.
- Cross-vendor agent-parity work — that has its own in-progress plan (`2026-05-03__cross-vendor-agent-parity/`).
- Distribution beyond `cargo run --release` and `nx build dist/`.

## Product Risks

- **PR-1 — Gherkin scenarios are the contract**: if any scenario is ambiguous, both binaries can satisfy it differently. Mitigation: shadow-diff harness compares actual outputs, not Gherkin interpretation.
- **PR-2 — Coverage validator algorithm drift**: line-coverage with partial-as-missed must match byte-for-byte. Mitigation: corpus-based diff test (Phase 1) over real `cover.out` files from `ayokoding-cli` / `ose-cli` / `rhino-cli` / `organiclever-be` / `ose-app-be`.
- **PR-3 — Hidden Go-specific assumptions** in some commands (e.g., go.mod parsing logic in `agents detect-duplication` or `repo-governance license-audit`). Mitigation: tech-docs §Command-Specific Risks lists each known assumption with a mitigation strategy.
- **PR-4 — Caller-graph misses**: a project.json reference to the Go binary that the migration overlooks results in mixed Go+Rust invocation post-archival. Mitigation: the cutover Gherkin scenario `grep -rE` check is a hard gate in Phase 8.
