---
title: Rust Governance Audit — Delivery Checklist
status: in-progress
created: 2026-05-23
---

# Delivery Checklist

Granular, item-per-commit-friendly checklist. Items use `- [ ]` so a future executor (or `plan-executor` agent) can tick them off. Phases run sequentially unless marked PARALLEL.

## Phase 0 — Kickoff & artifact freeze

- [ ] **0.1** Capture the kickoff web-research output to `generated-reports/rust-governance-audit__kickoff-research__2026-05-23.md` (UUID chain header per `repo-generating-validation-reports` skill).
- [ ] **0.2** Verify `git status` clean on `main` before starting.
- [ ] **0.3** Run baseline locally and record numbers:
  - `nx run rhino-cli:typecheck` → expect 0
  - `nx run rhino-cli:lint` → expect 0
  - `nx run rhino-cli:test:quick` → expect 0, note coverage %
  - `cargo clippy --manifest-path apps/rhino-cli/Cargo.toml --all-targets -- -D warnings -D unsafe_code` → expect 0
  - `grep -rE '\bunsafe\b' apps/rhino-cli/src/` → expect zero matches
  - Record outputs in `generated-reports/rust-governance-audit__baseline__2026-05-23.md`.

## Phase 1 — Inventory & static contradiction sweep

- [ ] **1.1** Build an artefact list by `find` over the five categories listed in `tech-docs.md §1`; save to `local-temp/rust-audit-artefacts.txt`.
- [ ] **1.2** For each of the 14 docs in `docs/.../rust/`, grep for hardcoded Rust version numbers (`1\.[0-9]+`); record findings.
- [ ] **1.3** Grep `repo-governance/` for the same Rust version pattern; record findings.
- [ ] **1.4** Grep `specs/apps/rhino/` for Go-era strings: `godog`, `\.go\b`, `go test`, `go run`, `//go:build`, `cmd/`; record line numbers.
- [ ] **1.5** Run pair-wise MUST/MUST NOT contradiction scan across the 14 standards docs (manual review of high-signal pairs identified in `tech-docs.md §5.1`); record findings.
- [ ] **1.6** Compile findings into `generated-reports/rust-governance-audit__inventory__2026-05-23.md` with a finding ID per row (F-01, F-02, ...).

## Phase 2 — Standards-doc consistency fixes

- [ ] **2.1** `docs/.../rust/README.md`: replace any hardcoded "Rust 1.82+" / "Rust 1.X" prose with a link of the form `MSRV declared in Cargo.toml` pointing at `apps/rhino-cli/Cargo.toml` (relative path computed at edit time).
- [ ] **2.2** `docs/.../rust/coding-standards.md` line 176: update `channel = "1.82.0"` example to current pin (`1.95.0` or whatever `rust-toolchain.toml` shows on edit day).
- [ ] **2.3** Pair-wise resolve every remaining contradiction in `tech-docs.md §3` table (C-01 through C-06). One commit per resolution.
- [ ] **2.4** Add a discoverable link from `repo-governance/development/quality/code.md` to `docs/.../rust/code-quality-standards.md` §246 (`forbid(unsafe_code)` MUST clause) — resolves C-04.
- [ ] **2.5** Cross-check `swe-rust-dev.md` and `swe-programming-rust/SKILL.md` for any version claim; align with Cargo.toml link.
- [ ] **2.6** Run `npm run lint:md` after each edit batch.

## Phase 3 — `specs/apps/rhino/README.md` rewrite

- [ ] **3.1** Read current README end-to-end; flag every Go reference inline in `local-temp/spec-readme-findings.md`.
- [ ] **3.2** Read `apps/rhino-cli/tests/` directory to confirm the actual Rust test pipeline (unit + integration shape).
- [ ] **3.3** Read `apps/rhino-cli/project.json` test targets to confirm exact `nx` commands.
- [ ] **3.4** Read the memory entry `project_rhino_cli_rust_cucumber_gap.md` to capture cucumber harness deferral context.
- [ ] **3.5** Rewrite "Running the Tests" section using `cargo test` / `nx run rhino-cli:test:quick` / `nx run rhino-cli:test:integration`; remove every `go ...` line.
- [ ] **3.6** Rewrite "Adding New Specs" section pointing at `tests/cucumber/` patterns (acknowledging deferral) + `assert_cmd`/`predicates` for binary integration tests.
- [ ] **3.7** Rewrite "Dual Consumption" table with Rust file patterns.
- [ ] **3.8** Update "Convention" link if BDD spec-test-mapping doc has Rust-specific guidance; if not, file a follow-up note.
- [ ] **3.9** Verify with `npm run lint:md` and a manual read.

## Phase 4 — Dependency currency decisions

PARALLEL within phase; each crate decision is an independent commit.

- [ ] **4.1** `chrono` 0.4.39 → 0.4.44: bump in `Cargo.toml`, run `cargo update -p chrono`, `nx run rhino-cli:test:quick`, `cargo clippy --all-targets -- -D warnings`. Commit.
- [ ] **4.2** `glob` 0.3.2 → 0.3.3: same flow as 4.1.
- [ ] **4.3** `sha2` 0.10.9 → 0.11.0 (**major**):
  - [ ] **4.3.1** Grep `apps/rhino-cli/src/` for `sha2::` usage; record call sites.
  - [ ] **4.3.2** Check if any call site uses removed APIs (`compress256`, `compress512`, removed feature flags) per the [RustCrypto sha2 0.11.0 CHANGELOG](https://github.com/RustCrypto/hashes/blob/master/sha2/CHANGELOG.md).
  - [ ] **4.3.3** **Decision branch A** — if migration is straightforward: bump, fix call sites, run full validation, commit. Update `apps/rhino-cli/README.md` "Dependency Status" with the bump.
  - [ ] **4.3.4** **Decision branch B** — if migration cost exceeds value: add a Path C waiver to `apps/rhino-cli/README.md` "Dependency Status" citing reason and review date.
- [ ] **4.4** `tempfile` 3.14.0 → 3.27.0 (dev-dep, breaking rename):
  - [ ] **4.4.1** Grep `apps/rhino-cli/tests/` and any dev module for `into_path`, `Builder::keep(`; record sites.
  - [ ] **4.4.2** Bump in `Cargo.toml`; rename `into_path()` → `keep()` per [tempfile CHANGELOG](https://github.com/Stebalien/tempfile/blob/master/CHANGELOG.md).
  - [ ] **4.4.3** Update any `Builder::keep(bool)` → `Builder::disable_cleanup(bool)`.
  - [ ] **4.4.4** Run `nx run rhino-cli:test:integration` to validate tempdir lifecycle.
- [ ] **4.5** Add a "Dependency Status" section to `apps/rhino-cli/README.md` recording every decision from 4.1-4.4 with date and Path (A/B/C).
- [ ] **4.6** Optional: install `cargo-outdated` locally (do not bake into `npm run doctor` yet) and verify the bumped state matches.

## Phase 5 — `forbid(unsafe_code)` governance hardening

- [ ] **5.1** Verify `apps/rhino-cli/src/lib.rs` line 1 = `#![forbid(unsafe_code)]` (done 2026-05-23 — confirm not regressed).
- [ ] **5.2** Verify `apps/rhino-cli/src/main.rs` line 1 = `#![forbid(unsafe_code)]` (done 2026-05-23 — confirm not regressed).
- [ ] **5.3** Run `grep -rE '\bunsafe\b' apps/rhino-cli/src/ apps/rhino-cli/tests/` — expect zero matches.
- [ ] **5.4** Audit `docs/.../rust/code-quality-standards.md §246` clause wording; ensure it explicitly:
  - [ ] **5.4.1** Mandates `#![forbid(unsafe_code)]` (not `deny`) for application crates.
  - [ ] **5.4.2** Names the exception clause for infrastructure crates with documented justification.
  - [ ] **5.4.3** Says the forbid attribute MUST appear in both crate roots (lib.rs and main.rs) when both exist.
- [ ] **5.5** If §246 lacks any of 5.4.1-5.4.3, add the missing clause(s); cross-link from `quality/code.md`.
- [ ] **5.6** Add a one-line invariant to `apps/rhino-cli/README.md` ("This crate forbids unsafe Rust; see `code-quality-standards.md` §246" with a real relative link to `docs/explanation/software-engineering/programming-languages/rust/code-quality-standards.md`).

## Phase 6 — Code structure compliance audit

For each subsection of `tech-docs.md §4`, walk the `apps/rhino-cli/src/` tree and verify.

- [ ] **6.1** Module layout audit (§4.1): list every `pub mod` declaration, verify `cli`/`commands`/`internal` boundary.
- [ ] **6.2** Public API audit (§4.2): grep every `pub fn`, `pub struct`, `pub enum` in `lib.rs` and immediate descendants; verify intentionality.
- [ ] **6.3** Error handling audit (§4.3): grep `unwrap()`, `expect(`, `panic!`; classify each occurrence as test-only or production.
- [ ] **6.4** Safety audit (§4.4): re-run unsafe grep (also covered in 5.3) — recorded twice intentionally because Phase 5 is forbid-clause-focused and Phase 6 is breadth-focused.
- [ ] **6.5** Lints audit (§4.5):
  - [ ] **6.5.1** Decide whether to add `[lints.rust]` and `[lints.clippy]` blocks to `Cargo.toml` (Cargo manifest format supported since edition 2024).
  - [ ] **6.5.2** If yes, encode `clippy::all = "deny"` and `clippy::pedantic = "warn"` with any inline `#[allow]` justifications.
  - [ ] **6.5.3** Verify `nx run rhino-cli:lint` still exits 0 after the encoding.
- [ ] **6.6** Testing audit (§4.6): cross-reference `tests/` directory against `testing-standards.md` three-level expectation.
- [ ] **6.7** Performance profile audit (§4.7): compare `Cargo.toml` `[profile.release]` block against `build-configuration.md`.
- [ ] **6.8** Build/Nx audit (§4.8):
  - [ ] **6.8.1** Verify each `validate:*` target in `project.json` actually invokes a binary subcommand that still exists in `cli.rs`.
  - [ ] **6.8.2** Check whether `cargo audit` should be wired into a new `audit` Nx target.
  - [ ] **6.8.3** Check whether `cargo deny check` should be wired similarly.
- [ ] **6.9** Compile finding list per subsection into `generated-reports/rust-governance-audit__code-structure__YYYY-MM-DD.md`.

## Phase 7 — Cross-doc final contradiction sweep

- [ ] **7.1** Re-run §1.5 pair-wise scan on the **edited** docs (post Phase 2-6 changes).
- [ ] **7.2** Compile the post-fix contradiction report; should be empty.
- [ ] **7.3** If non-empty, fix and loop until empty.

## Phase 8 — Verification gate

- [ ] **8.1** `nx run rhino-cli:typecheck` → 0
- [ ] **8.2** `nx run rhino-cli:lint` → 0
- [ ] **8.3** `nx run rhino-cli:test:quick` → 0; coverage ≥ 90%
- [ ] **8.4** `nx run rhino-cli:test:integration` → 0
- [ ] **8.5** All ten `nx run rhino-cli:validate:*` targets → 0
- [ ] **8.6** `cargo clippy --manifest-path apps/rhino-cli/Cargo.toml --all-targets -- -D warnings -D unsafe_code` → 0
- [ ] **8.7** `grep -rE '\bunsafe\b' apps/rhino-cli/src/ apps/rhino-cli/tests/` → 0 matches
- [ ] **8.8** `npm run lint:md` → 0
- [ ] **8.9** Re-walk every Gherkin scenario in `prd.md §Acceptance Criteria`; each must demonstrably pass.

## Phase 9 — Web-research re-verification

- [ ] **9.1** Spawn `web-research-maker` with the same prompt used at kickoff; compare new findings against `tech-docs.md §2` currency table.
- [ ] **9.2** If any dependency moved upstream during the audit, re-open Phase 4 for that crate.
- [ ] **9.3** Archive the re-check report at `generated-reports/rust-governance-audit__post-delivery-research__YYYY-MM-DD.md`.

## Phase 10 — Plan close-out

- [ ] **10.1** Move `plans/in-progress/rust-governance-audit/` → `plans/done/YYYY-MM-DD__rust-governance-audit/` (date = completion date).
- [ ] **10.2** Update `plans/done/` index README if it exists.
- [ ] **10.3** Commit close-out.
- [ ] **10.4** Push to `origin main`.
- [ ] **10.5** Trigger and verify CI per `ci-post-push-verification` (poll every 3 min, no `gh run watch`).
- [ ] **10.6** Update auto-memory with anything surprising discovered (e.g. if a doc kept drifting back, note the reason).
- [ ] **10.7** Decide whether Section 4 of `tech-docs.md` should be promoted to a `repo-governance/development/quality/rust-crate-structural-checklist.md` for the next Rust crate.

## Commit hygiene

- One conventional commit per finding-resolution (or small batch per file when atomic).
- `chore(rhino-cli):`, `docs(rust):`, `chore(plans):`, `chore(deps):` scopes as appropriate.
- All commits land on `main` (Trunk Based Development).
- Reference the finding ID (`F-XX`) from the inventory report in each commit body.

## Open questions to resolve during execution

1. Should `cargo audit` and `cargo deny` be wired into a new shared `audit` Nx target, or invoked from the existing `test:quick` pipeline?
2. Should the structural checklist in `tech-docs.md §4` be promoted into governance immediately, or held back until a second Rust crate exists to validate the abstraction?
3. Is the `[lints]` table in `Cargo.toml` (Phase 6.5) the right encoding, or should the attributes stay in `lib.rs`/`main.rs` for visibility?

These do not block the plan; resolve when reaching the relevant phase.
