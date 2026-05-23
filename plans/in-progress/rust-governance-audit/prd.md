---
title: Rust Governance Audit — Product Requirements
status: in-progress
created: 2026-05-23
---

# Product Requirements

## Functional Requirements

### FR-1. Single source of truth for Rust toolchain version

All version references (`docs/.../rust/README.md`, `coding-standards.md` examples, `Cargo.toml` `rust-version`, `rust-toolchain.toml` `channel`, any `repo-governance/` mention, `swe-rust-dev.md`, `SKILL.md`) MUST resolve to a single MSRV statement that is internally consistent. Rationale: contributors should never have to choose between four numbers.

- MSRV declared in `Cargo.toml` is the authoritative MSRV for the crate.
- `rust-toolchain.toml` channel pin is the authoritative installed-toolchain version (can be ≥ MSRV).
- All prose docs cite the MSRV with a link to `Cargo.toml` rather than repeating a hardcoded number.

### FR-2. Spec README reflects current Rust testing pipeline

`specs/apps/rhino/README.md` MUST describe the Rust testing flow accurately:

- No Go-specific tooling references (`godog`, `cmd/`, `//go:build` tags, `go run`, `go test`).
- Cucumber harness status acknowledged honestly (currently deferred per the memory `[[project_rhino_cli_rust_cucumber_gap]]`); 754 unit tests + shadow-diff path documented.
- Adding-new-specs instructions point to Rust file patterns (`tests/cucumber/`, `assert_cmd`, `predicates`).
- Commands shown are Rust commands (`cargo test`, `nx run rhino-cli:test:quick`, `nx run rhino-cli:test:integration`).

### FR-3. Dependency currency decisions documented

For every Cargo dependency that the kickoff web-research report flagged as behind upstream:

- EITHER bump to current with passing `nx run rhino-cli:test:quick` and `cargo clippy --all-targets -- -D warnings`,
- OR add an explicit waiver entry (in `apps/rhino-cli/README.md` "Dependency Status" section or this plan's `tech-docs.md`) citing the [Dependency Bump Policy](../../../repo-governance/development/workflow/dependency-bump-policy.md) path (A/B/C) and the reason.

No silent stale dependencies.

### FR-4. `forbid(unsafe_code)` codified in governance

`code-quality-standards.md` line 246 already says application crates MUST `forbid(unsafe_code)`. The audit MUST verify that:

- The MUST clause is discoverable from `repo-governance/development/quality/code.md` (the cross-language quality index) via at least one direct link or paraphrase.
- Every active Rust application crate in the repo carries `#![forbid(unsafe_code)]` in both `lib.rs` and `main.rs` (currently rhino-cli only).
- The standard names an explicit exception clause for infrastructure crates (FFI bindings, performance-critical SIMD) so the rule is operationally sustainable.

### FR-5. Code structure complies with platform Rust standards

`apps/rhino-cli/src/` MUST pass a structural audit against the eleven standards documents under `docs/.../rust/` (excluding the `README.md` index, `build-configuration.md`, and the `templates/` subdir which are reference-only):

| Standard                         | Specific check                                                                                                                    |
| -------------------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| `coding-standards.md`            | snake_case functions/modules, PascalCase types, no abbreviations in public API                                                    |
| `code-quality-standards.md`      | rustfmt clean, clippy `-D warnings` clean, no undocumented `unsafe` (forbid satisfies this), `// SAFETY:` comments not applicable |
| `type-safety-standards.md`       | No `unwrap()` / `expect()` outside tests, newtypes for domain primitives where appropriate                                        |
| `error-handling-standards.md`    | Library code uses `thiserror`-derived errors; binary `main` uses `anyhow::Result`                                                 |
| `memory-management-standards.md` | No unnecessary `clone()` in hot paths; reasoned `Cow`/`&str` usage                                                                |
| `concurrency-standards.md`       | N/A for CLI without async runtime — explicitly noted                                                                              |
| `api-standards.md`               | N/A for CLI without HTTP API — explicitly noted                                                                                   |
| `testing-standards.md`           | Three-level testing structure honoured; unit/integration boundaries clear                                                         |
| `performance-standards.md`       | Release profile in `Cargo.toml` aligns with documented defaults                                                                   |
| `security-standards.md`          | `cargo audit` clean; `cargo deny check` configured or explicitly deferred                                                         |
| `ddd-standards.md`               | N/A for tooling CLI — explicitly noted                                                                                            |

### FR-6. Cross-doc contradictions resolved

Pair-wise comparison of every Rust-naming governance doc against every other yields zero contradictory MUST/MUST NOT statements. Comparison scope: 13 standards docs plus 1 templates subdir under `docs/.../rust/`, 9 cross-cutting governance files enumerated in `tech-docs.md §1.4`, plus the `swe-rust-dev` agent file (Claude + OpenCode mirror) and the `swe-programming-rust` skill file.

### FR-7. Web-research findings encoded

Every claim in this plan's [tech-docs.md](./tech-docs.md) "Currency table" derived from the kickoff `web-research-maker` invocation is reproducible via the cited URL and includes a `[Verified]` / `[Likely]` confidence label.

## Non-Functional Requirements

### NFR-1. No new tooling

The audit MUST be executable with tools already pinned by `npm run doctor` (rustc, cargo, clippy, rustfmt). No new global installs.

### NFR-2. Reversible at every step

Each delivery item is a discrete commit. Reverting any one commit MUST leave the repo in a consistent (if pre-audit) state.

### NFR-3. Trunk-Based Development

Per repo convention, all commits land directly on `main`. No long-lived feature branch.

### NFR-4. Audit re-runnable

After delivery, the audit checklist itself can be re-run by a future contributor (or a checker agent) and produces a clean (zero-findings) report. The checklist artefact is the `delivery.md` `- [ ]` items.

## Acceptance Criteria (Gherkin)

```gherkin
Feature: Rust governance is consistent, correct, current, and contradiction-free

  Background:
    Given the ose-public repository at commit head of main
    And the kickoff web-research report dated 2026-05-23

  Scenario: MSRV is stated exactly once
    Given Cargo.toml declares rust-version = "X.Y"
    When I grep all governance docs, agent files, and skill files for a Rust version number
    Then every occurrence either restates "X.Y" verbatim or is replaced by a link to Cargo.toml
    And no occurrence references a Rust version older than the current MSRV

  Scenario: specs/apps/rhino/README.md is a Rust document
    Given specs/apps/rhino/README.md
    When I read the file end to end
    Then I find zero references to "godog", "go test", "go run", ".go" file extensions, or "//go:build" tags
    And I find at least one reference to "cargo test" and "nx run rhino-cli:test"
    And the cucumber harness status is described honestly

  Scenario: Dependency currency is justified
    Given Cargo.toml lists dependency D pinned at version V_pinned
    And the web-research report says latest is V_latest
    When V_pinned != V_latest
    Then EITHER Cargo.toml has been updated to V_latest with passing test:quick + clippy
    Or apps/rhino-cli/README.md (or this plan's tech-docs.md) contains a waiver block citing the Dependency Bump Policy path and the reason

  Scenario: Unsafe Rust is forbidden everywhere it could leak in
    Given an active Rust application crate in apps/
    When I open its src/lib.rs and src/main.rs
    Then each file's first non-comment, non-blank line is `#![forbid(unsafe_code)]`
    And running `cargo clippy --all-targets -- -D warnings -D unsafe_code` exits 0
    And grepping `unsafe` across the crate's .rs files (excluding target/) returns zero matches

  Scenario: Code structure complies with platform Rust standards
    Given apps/rhino-cli/src/
    When I cross-reference each module against the eleven standards documents under docs/.../rust/ (excluding the README index, build-configuration.md, and templates/)
    Then every applicable standard's MUST clauses are satisfied
    And any N/A standard (api, ddd, concurrency for a CLI) is explicitly justified in tech-docs.md

  Scenario: Cross-document contradictions are zero
    Given the 13 standards docs under docs/.../rust/, the 9 cross-cutting governance files enumerated in tech-docs.md §1.4, the swe-rust-dev agent (Claude + OpenCode mirror), and the swe-programming-rust skill
    When I run a pair-wise audit comparing MUST/MUST NOT statements
    Then no two documents make conflicting normative claims

  Scenario: The audit is re-runnable to zero findings
    Given the completed plan delivery.md checklist
    When a future contributor (or checker agent) re-executes every item
    Then every check passes without remediation
    And the contributor needs no out-of-band information to complete the audit
```
