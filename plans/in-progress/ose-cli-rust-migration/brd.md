# Business Requirements Document — ose-cli Rust Migration

## Business Goal

Eliminate the Go runtime dependency from the `ose-cli` tooling path by migrating `apps/ose-cli/` to Rust, aligned with the polyglot consolidation direction established by the `rhino-cli` port (2026-05). The Rust crate `libs/rust-commons/` created in Phase 0 provides a durable, shared link-checking library for all current and future Rust CLI tools in the monorepo.

## Business Rationale

The `apps/ose-cli/` tool is a prerequisite for the ose-web CI quality gate. It validates internal links in `apps/ose-web/content/` on every push. Continuing to maintain a separate Go toolchain for a single small CLI adds operational overhead:

- **Go toolchain sprawl**: Go is currently used by `ose-cli`, `ayokoding-cli`, `ose-app-be` (pending migration), and two shared libs. Moving CLIs to Rust reduces the number of distinct runtime stacks that must be kept current.
- **Shared library leverage**: The link-checking logic in `libs/golang-link-commons/` is duplicated in spirit across two CLI tools. A Rust equivalent (`libs/rust-commons/`) can serve both `ose-cli` (this plan) and `ayokoding-cli` (sibling plan), making maintenance centralized.
- **Consistency with rhino-cli**: `rhino-cli` is already Rust and provides the Nx target patterns, linting configuration, and smoke-test harness. Porting `ose-cli` to the same stack reduces cognitive overhead for future contributors.

_Judgment call_: The estimated effort reduction from maintaining one fewer Go binary in CI is modest but accumulates over time as more CLIs migrate. The Rust toolchain is already present in CI via `rhino-cli`, so no new infrastructure is required.

## Affected Roles

This is a solo-maintainer repository. The plan affects:

- **Maintainer as toolchain operator** — responsible for keeping `ose-cli` functional and up to date
- **Maintainer as ose-web site editor** — relies on `ose-cli links check` as part of the `test:quick` quality gate
- **CI pipeline** — the `spec-coverage` and `test:quick` Nx targets for `ose-cli` run in GitHub Actions on every push

No sign-off ceremonies or external stakeholder approvals apply.

## Business-Level Success Metrics

- **Observable fact**: `nx run ose-cli:test:quick` passes in CI after the migration, verifying the link-check functionality is preserved.
- **Observable fact**: `nx run ose-cli:lint` and `nx run ose-cli:typecheck` pass, confirming the Rust implementation meets quality standards.
- **Observable fact**: `libs/rust-commons/` exists as a standalone Nx project with its own `test:quick` target passing at 90% line coverage.
- _Judgment call_: Removing the Go build dependency from `ose-cli` CI path is qualitatively simpler to maintain. No hard latency measurement is required for a tool of this scope.

## Business-Scope Non-Goals

- Introducing new link-checking features (external link validation, anchor checking) — these belong in a separate feature plan
- Migrating `libs/golang-link-commons/` to Rust as part of this plan — blocked by `ayokoding-cli` dependency; tracked in that sibling plan
- Changing the behavior of the `ose-cli links check` command from the user's perspective — pure port, functional parity only
- Automating the archival of Go source at `archived/ose-cli/` via CI — done manually as a delivery step

## Business Risks and Mitigations

| Risk                                                                                | Impact                                                         | Mitigation                                                                                                                                           |
| ----------------------------------------------------------------------------------- | -------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| Rust port has behavioral regression (missed edge case in link walker)               | ose-web CI gate passes silently with broken links              | Port unit tests from Go checker; maintain integration test against real `apps/ose-web/content/`; require 90% line coverage                           |
| `libs/rust-commons/` API design diverges from what `ayokoding-cli` needs            | Sibling plan requires breaking changes to `libs/rust-commons/` | Design `libs/rust-commons/` with a minimal, stable public API; coordinate with `ayokoding-cli-rust-migration` plan before finalizing the API surface |
| Go source deletion breaks `libs/golang-link-commons/` (still used by ayokoding-cli) | `ayokoding-cli` CI gate fails                                  | Explicitly prohibited in this plan: `libs/golang-link-commons/` is NOT removed; only `apps/ose-cli/` Go source is archived                           |
| CI Rust toolchain not present for `libs/rust-commons/`                              | New Nx project fails in CI                                     | `rust-toolchain.toml` pins 1.95.0 in both new projects; same channel as `rhino-cli` which already passes CI                                          |
