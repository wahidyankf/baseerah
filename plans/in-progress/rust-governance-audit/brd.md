---
title: Rust Governance Audit — Business Rationale
status: in-progress
created: 2026-05-23
---

# Business Rationale

## Problem

The `rhino-cli` Go→Rust port (completed 2026-05-23) and the subsequent `forbid(unsafe_code)` patch were focused on shipping working code. They did not include a sweep of every Rust-touching artefact in the repository. The result is a measurable governance drift surface:

1. **Toolchain version stated four different ways** (`1.82+` in `docs/.../rust/README.md`, `1.82.0` example in `coding-standards.md`, `1.88` in `Cargo.toml` MSRV, `1.95.0` actual pin in `rust-toolchain.toml`).
2. **`specs/apps/rhino/README.md` documents Go tooling** (`godog`, `.go` test files, `//go:build integration` tags, `go run main.go ...`) for what is now a Rust crate — readers following this README will fail.
3. **Three production dependencies behind upstream** with one major-version gap (`sha2 0.10.9 → 0.11.0`) and one breaking rename (`tempfile 3.14.0 → 3.27.0`); none of these decisions is documented per the Dependency Bump Policy.
4. **No structural code-review pass against platform Rust standards** since the port — module boundaries, error-handling patterns, public API surface, lint configuration have not been validated against `docs/.../rust/*-standards.md`.
5. **`unsafe` is a non-negotiable no** but the forbid attribute only lives in two crate roots. Governance docs do not yet codify "all OSE application crates MUST forbid unsafe", so a future crate could re-introduce it without violating a written rule.

## Cost of not acting

- **Onboarding friction**: a new contributor reading `specs/apps/rhino/README.md` runs Go commands against a Rust crate, hits a wall, blames the docs, files a support ticket.
- **Silent dependency rot**: `sha2 0.11.0` carries a non-trivial API migration. Sleeping on it pushes the cost forward and makes a future security-driven upgrade harder.
- **Standards erosion**: every additional Rust crate added in this state inherits the same drift and copies the same `forbid(unsafe_code)` ad-hoc instead of from a written rule.
- **Audit findability**: nothing in `repo-governance/` currently enumerates Rust artefacts in one place — every future Rust audit starts from a `grep -r rust` and rediscovers the same surface area.

## Outcome (success state)

After this plan executes, the following statements are true and verifiable:

- A single MSRV is stated identically in `Cargo.toml`, `rust-toolchain.toml`, and every governance/standards doc that names a Rust version. The MSRV is current as of 2026-05-23 and cited.
- `specs/apps/rhino/README.md` describes the Rust testing pipeline accurately (cucumber-rs status acknowledged, `cargo test`, `nx run rhino-cli:test:quick` / `test:integration`, no Go residue).
- Every Cargo dependency that is behind upstream has either been bumped, OR has an explicit waiver entry citing the Dependency Bump Policy path (A/B/C) and the reason.
- `repo-governance/` contains a written rule mandating `#![forbid(unsafe_code)]` for OSE application crates with a documented escape hatch only for infrastructure crates (the existing `code-quality-standards.md` MUST clause is referenced from a discoverable index entry).
- `rhino-cli` source code passes a structural compliance audit against the platform Rust standards (naming, error handling, module layout, public API surface, lint configuration).
- A static `cargo audit` and `cargo deny check` invocation are wired into either CI or a pre-merge target so the next dependency CVE is caught automatically.
- All six audit findings from `tech-docs.md §3` (two version-reconciliation rows C-01/C-02, one spec-README Go-residue row C-03, one cross-link gap C-04, one dependency-status gap C-05, one MSRV-vs-pin clarification C-06) plus the four dependency-currency rows from `tech-docs.md §2.2–2.3` (chrono, glob, sha2, tempfile) are each closed with a corresponding diff in the commit history.

## Non-goals

- We are **not** rewriting tutorials in `ayokoding-web`.
- We are **not** introducing Axum/Tokio/SQLx into `rhino-cli` (it is and will remain a CLI).
- We are **not** changing the platform Rust framework stack recommendations — only verifying they are stated consistently.

## Risk register

| Risk                                                             | Likelihood | Mitigation                                                                                                               |
| ---------------------------------------------------------------- | ---------- | ------------------------------------------------------------------------------------------------------------------------ |
| `sha2 0.11.0` major upgrade breaks `rhino-cli` compile           | Medium     | Bump in isolation with full test:quick + clippy verification; if migration cost > value, document a Path C waiver.       |
| `tempfile 3.27.0` rename (`into_path` → `keep`) breaks dev tests | Medium     | Targeted grep + rename within `tests/`; verify with `cargo test --tests`.                                                |
| Spec README rewrite reveals untested behaviour                   | Low        | Audit existing `.feature` files against the `rhino-cli` command surface during the audit phase, before rewriting README. |
| Governance docs become inconsistent with future crates           | Low        | Add a `repo-rules-checker`-style audit step to the delivery list so the next Rust crate triggers the same checks.        |

## Success metric

Zero findings on a re-run of the kickoff audit checklist after delivery, plus all seven Gherkin scenarios in [prd.md](./prd.md) pass.
