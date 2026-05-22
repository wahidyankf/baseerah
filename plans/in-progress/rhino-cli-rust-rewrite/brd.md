# Business Requirements Document — Rewrite rhino-cli to Rust

## Business Goal

Give `ose-public`'s fundamental tooling — the binary every contributor's pre-commit, pre-push, and CI pass through ~30+ times an hour — the strongest type system available so a class of latent defects (nil-dereference, untagged sum-type mismatches, silently-ignored variants in output-format and coverage-format switches) becomes a compile error instead of a hook-fired runtime regression on someone's commit. Go's type system is **not strong enough** for tooling this load-bearing: it cannot model exhaustive `match` over sealed variants, cannot enforce ownership of the temporary directories the integration tests scribble into, and cannot prove at compile time that an `OutputFormat` value came from `parseOutputFormat`. Rust can. The goal is **type-safety for fundamental tooling**, not feature expansion — the Rust `rhino-cli` does exactly what the Go one does today, no more.

Tooling consolidation, distribution ergonomics, and toolchain alignment with the systems-level binaries elsewhere on the roadmap (`crud-be-rust-axum` demo, future runtime components) are **secondary benefits** that fall out of this choice — they are not the driver.

## Why now

- The Go `rhino-cli` is the only binary on the critical path of every other app's quality gates. A defect in its `OutputFormat` parser, its coverage-format detector, or its `repo-governance vendor-audit` heading state machine doesn't show up in one app's tests — it fires across every app on every commit. The blast radius justifies the strongest static guarantees we can buy. [Judgment call: based on the fact that `rhino-cli` is wired into every `apps/*/project.json` `test:quick` and `spec-coverage` target, and both Husky hooks.]
- The recent CLI alignment work (commit `d4bacc851`) settled the three Go CLIs at parity — Cobra root pattern, output-format validation, coverage threshold 90. That parity exposed concretely how much of `rhino-cli`'s invariant-keeping is **runtime defensive code** (the `parseOutputFormat` `PersistentPreRunE`, the partial-line coverage classification, the unknown-format error string) that a sealed enum and exhaustive `match` in Rust would lift into the type checker.
- The doctor already probes `rustc` and `cargo-llvm-cov` [Repo-grounded — `apps/rhino-cli/internal/doctor/checker.go:parseRustVersion`, `parseCargoLlvmCov`]. Rust toolchain readiness is already a requirement — the missing piece is using it.
- Removing the Go footprint from the critical-path tooling also lets us optionally drop Go from the developer toolchain entirely once `ayokoding-cli` and `ose-cli` follow (out of scope for this plan). Secondary benefit, not the driver.

## Why Rust over alternative type-system targets

[Judgment call] Rust beats the realistic alternatives — staying on Go, porting to TypeScript, or porting to F# — when **type safety for fundamental tooling** is the primary lens:

- **Stay on Go**: the type system that lets the current bugs through. No sealed sum types, no exhaustive match, no compile-time exhaustiveness on `OutputFormat` / coverage-format / governance-finding variants, no compile-time ownership of fixture directories. Comfortable but exactly the comfort we are paying for in runtime defensive code today. Does not address the root cause.
- **Port to TypeScript**: strong structural typing and discriminated unions are real, but the type system runs out at the FFI boundary every CLI eventually hits (process spawn, filesystem, `go tool cover` output parsing). Plus Node.js startup overhead on every pre-commit hook fire (hundreds of times a week) — slower, heavier toolchain for a hook-fired CLI.
- **Port to F#**: discriminated unions + railway-oriented error handling are an excellent fit for the validator-style commands. But .NET runtime startup penalty rules it out for a hook-fired CLI, and the platform's F# binaries (`organiclever-be`, `ose-app-be`) are server-side where startup cost amortizes — opposite use case.
- **Port to Rust**: native binary with zero-runtime startup, exhaustive `match` on sealed enums, lifetime-checked borrows on temp-dir fixtures, `Result<T, E>` enforced at every fallible boundary, and `#[non_exhaustive]` for forward-compatible variants. The toolchain is already required by the doctor. Type-system depth matches the systems-level use case.

## Why these specific bugs are type-checkable in Rust but not in Go

Concrete examples of `rhino-cli` runtime defensive code that becomes compile-time in Rust [Judgment call: based on patterns visible in the Go source]:

- **Output format parsing**: the `parseOutputFormat` `PersistentPreRunE` in `apps/rhino-cli/cmd/root.go` is a runtime guard that returns an error on unknown variants. In Rust, `OutputFormat` is a sealed `#[derive(Debug)] enum` and every `match` over it must be exhaustive at compile time — adding a new variant breaks compilation everywhere it should, not just at runtime where someone trips it.
- **Coverage format detection**: Go's auto-detect path (Go cover.out vs LCOV vs JaCoCo vs Cobertura) is a chain of type assertions and string matching. In Rust, each format is a struct implementing a `CoverageFormat` trait; the detector returns `Result<Box<dyn CoverageFormat>, DetectError>` and the consumer cannot accidentally skip the error branch.
- **Coverage line classification**: the `covered | partial | missed` triplet that the validator treats as ordinal (partial-counts-as-missed) is currently three boolean flags inferred from raw block counts. In Rust, it's an `enum LineState { Covered, Partial, Missed }` and the threshold comparison cannot accept a stray `bool`.
- **Governance finding criticality**: `CRITICAL | HIGH | MEDIUM | LOW` is enforced today by convention and string comparison. In Rust, it's an `enum Severity` with `Ord` derived — the aggregator cannot accidentally sort string-formatted severity, and a typo in one validator does not compile.
- **Integration-test fixture ownership**: Go's t.TempDir cleanup is a defer hook; if the test panics before defer registers, the directory leaks. Rust's `tempfile::TempDir` is owned by RAII — the borrow checker enforces that no test code outlives its fixture, full stop.

## Affected roles

| Role                            | Impact                                                                                                                                                               |
| ------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Repository maintainer**       | Day-to-day commands stay identical. Compile-once binary (`cargo build --release`) replaces `go run main.go` on every fire — faster hook execution after first build. |
| **Contributor (any role)**      | `npm run doctor -- --fix` installs `rustup` + stable toolchain if missing; otherwise no change.                                                                      |
| **CI workflows**                | Per-workflow Setup step adds `actions-rust-lang/setup-rust-toolchain@v1` where rhino-cli is invoked. Build cache via `Swatinem/rust-cache@v2`.                       |
| **Pre-commit / pre-push hooks** | Same observable behavior, faster execution after first compile (warm cache on dev workstations and CI).                                                              |

## Business-level success criteria

- **Type-safety landed where it matters**: every invariant listed in [§Why these specific bugs are type-checkable in Rust but not in Go](#why-these-specific-bugs-are-type-checkable-in-rust-but-not-in-go) is enforced by the compiler in the Rust crate, not by runtime checks. Verifiable by code review of the ported modules — sealed enums on output format, coverage format, line state, and severity; RAII fixture ownership in integration tests.
- Zero observable behavior delta on documented commands during the soak window — measured by the shadow-diff harness (Phase 1) comparing Go and Rust output for every Nx target invocation.
- After archival, every `apps/*/project.json`, `.husky/*`, and `.github/workflows/*` that previously referenced `apps/rhino-cli/main.go` references the Rust binary path instead — verifiable via `grep -r "apps/rhino-cli/main.go"`.
- Pre-push hook wall-clock time does not regress beyond +20% on a clean cache, and improves on a warm cache. [Judgment call: based on Rust binary startup typically being faster than `go run` cold-compile.]
- `archived/README.md` lists the Go implementation with its archival date and successor reference.

## Business-scope Non-Goals

- Feature work: no new commands, no behavior changes, no flag additions.
- Cross-platform Windows support beyond what the Go version offers (the Go version already runs on macOS + Linux; the Rust port matches that and nothing more).
- Pre-built binary distribution outside the repo (Homebrew formula, GitHub Releases). All invocations remain via `cargo run --release --manifest-path ...` or via `dist/` artifacts produced by `nx build rhino-cli`.
- Migrating `ose-primer/apps/rhino-cli` (downstream Go template). That sync is a follow-up after `ose-public` stabilizes.
- Touching `libs/golang-commons` — it stays for `ayokoding-cli` / `ose-cli`.

## Business risks

- **R1 — Behavior drift during port** lands silently in someone's pre-commit hook on `main`. **Mitigation**: shadow-diff harness gates each command's switchover (see tech-docs §Shadow-Diff Mechanics); Phase 1 commands run both binaries side-by-side and fail-loud on divergence for one week before flipping.
- **R2 — Coverage validator divergence** breaks `ayokoding-cli`/`ose-cli` `test:quick` because their threshold gate calls `rhino-cli test-coverage validate`. **Mitigation**: Phase 1 ports the validator first with byte-for-byte algorithm parity (line coverage, partial-counts-as-missed); diff-test against the existing `cover.out` corpus before flipping any sibling project.json.
- **R3 — Mermaid validator regression** because Go and Rust tree-sitter bindings track grammar versions independently. **Mitigation**: pin the same `tree-sitter-markdown` grammar SHA in Rust as the Go version uses; diff-test corpus of known-good and known-bad mermaid blocks.
- **R4 — Doctor toolchain probe gaps** if `rustup` or `cargo-llvm-cov` are missing on a contributor machine. **Mitigation**: doctor already probes both; `npm run doctor -- --fix` already covers `rustup` installation. Verify before Phase 0 lands.
- **R5 — CI cold-cache compile time** on every workflow run. **Mitigation**: `Swatinem/rust-cache@v2` in every workflow that invokes `rhino-cli`; build once per workflow and reference `dist/` thereafter.
- **R6 — Plan abandonment partway** leaves two implementations on `main` indefinitely. **Mitigation**: every phase ends at a coherent, mergeable state with both binaries usable. If the plan stalls, the Go version remains canonical and the partial Rust crate sits behind unused Nx targets — no broken `main`.

See [prd.md](./prd.md) for Gherkin acceptance criteria and [tech-docs.md](./tech-docs.md) for the technical realization of these mitigations.
