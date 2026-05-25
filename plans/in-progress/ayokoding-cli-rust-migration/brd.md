# Business Requirements Document — ayokoding-cli Rust Migration

## Business Goal

Eliminate the Go dependency from `apps/ayokoding-cli/` by rewriting it in Rust, and remove the
now-unused Go shared libraries (`libs/golang-link-commons/`, `libs/golang-commons/`) once both
Go CLIs have been migrated. The result is a mono-language CLI tier (Rust only), reducing toolchain
maintenance burden and enabling a single shared crate for link-checking logic across all CLI tools.

## Business Impact

**Pain points addressed:**

- The monorepo currently maintains two separate language toolchains for CLI tools: Go (ayokoding-cli,
  ose-cli) and Rust (rhino-cli). After ose-cli migrates via the sibling plan, ayokoding-cli is the
  last Go CLI remaining.
- Two Go shared libraries (`libs/golang-link-commons/`, `libs/golang-commons/`) exist solely to
  support the two Go CLIs. Once both CLIs migrate, these libs become dead code that Nx still tracks
  and CI still tests.
- The link-checking logic is duplicated across Go (`libs/golang-link-commons/`) and Rust
  (`libs/rust-commons/`). After this migration the Rust version is the sole canonical implementation.

**Expected benefits:**

- Go toolchain and `golangci-lint` become optional after this plan completes — only Rust is needed
  for CLI development.
- Dead Go shared library code is removed, reducing repo surface area and CI build time.
  _Judgment call: CI time savings are not measured; claim is directionally plausible given removal of
  two library build/test targets._
- Uniform Rust toolchain (`rust-toolchain.toml`, `cargo deny`, `cargo llvm-cov`) applied
  consistently across all CLI tools.

## Affected Roles

This is a solo-maintainer repository. No sign-off ceremonies apply. Roles affected:

- **CLI developer** (maintainer hat): writes and maintains the Rust source.
- **Content author** (maintainer hat): uses `ayokoding-cli links check` to validate internal links
  in `apps/ayokoding-web/content` before pushing.
- **CI pipeline** (automated): runs `nx run ayokoding-cli:test:quick` and `nx run ayokoding-cli:lint`
  on every push.
- **swe-rust-dev agent**: executes Rust implementation steps in the delivery plan.

## Business-Level Success Metrics

- **Observable fact**: `nx run ayokoding-cli:test:quick` exits 0 after migration — measured by CI.
- **Observable fact**: `nx run ayokoding-cli:lint` exits 0 after migration — measured by CI.
- **Observable fact**: `libs/golang-link-commons/` and `libs/golang-commons/` directories no longer
  exist in the repo after cleanup — verified by `git ls-files libs/golang-*`.
- **Observable fact**: `go build` is no longer required in the repo's doctor tool for any non-archived
  app or lib — verifiable once ose-cli also completes its migration.
- _Judgment call_: developer experience improvement from single-language CLI tier is subjective;
  no numeric target is asserted.

## Business-Scope Non-Goals

- Changing the CLI's external behavior (flags, subcommands, output formats, exit codes) — preserved
  exactly.
- Migrating `apps/ayokoding-web/` or any Next.js app.
- Migrating `libs/golang-commons/` independently of this plan's final cleanup step.
- Adding new link-checking features beyond what the Go version provides.
- Removing the Go toolchain from CI before both CLI migrations are complete. _This plan handles
  ayokoding-cli only; Go removal from the doctor tool is a separate concern._

## Business Risks and Mitigations

| Risk                                                                                            | Likelihood | Mitigation                                                                                                                                                       |
| ----------------------------------------------------------------------------------------------- | ---------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `libs/rust-commons/` API differs from what was designed for ose-cli                             | Medium     | Phase 0 hard gate verifies the lib exists and its public API before any code is written.                                                                         |
| Another app still imports `libs/golang-link-commons/` or `libs/golang-commons/` at cleanup time | Low        | Phase 3 runs `grep -r` across all `apps/` and `libs/` before deleting; deletion is gated on zero matches.                                                        |
| Rust migration introduces a behavioral regression in link checking                              | Low        | The existing Gherkin feature file (`specs/apps/ayokoding/behavior/cli/gherkin/links/links-check.feature`) drives TDD; all four scenarios must pass before merge. |
| `cargo llvm-cov` coverage target of 90% is not met                                              | Low        | The Go version already achieves 90%; the Rust port covers the same scenarios via the same Gherkin specs.                                                         |
