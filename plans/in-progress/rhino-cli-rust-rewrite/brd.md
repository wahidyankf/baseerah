# Business Requirements Document — Rewrite rhino-cli to Rust

## Business Goal

Consolidate platform tooling on Rust to reduce long-run maintenance cost, improve distribution ergonomics, and align the platform's hygiene tooling with the language we already commit to for systems-level binaries elsewhere on the roadmap (`crud-be-rust-axum` demo, future runtime components). The goal is **tooling consolidation**, not feature expansion — the Rust `rhino-cli` does exactly what the Go one does today, no more.

## Why now

- The Go `rhino-cli` is the only Go binary on the critical path of every other app's quality gates. Keeping one Go-language footprint solely for tooling means the doctor must keep probing Go for every contributor, every CI run, every Vercel build. Removing it lets us optionally drop Go from the developer toolchain entirely once `ayokoding-cli` and `ose-cli` follow (out of scope for this plan).
- The doctor already probes `rustc` and `cargo-llvm-cov` [Repo-grounded — `apps/rhino-cli/internal/doctor/checker.go:parseRustVersion`, `parseCargoLlvmCov`]. Rust toolchain readiness is already a requirement — the missing piece is using it.
- The recent CLI alignment work (commit `d4bacc851`) settled the three Go CLIs at parity. That parity is the natural staging point: lift `rhino-cli` to Rust and the other two siblings continue using whichever rhino implementation is on `main`.

## Why Rust over alternative consolidation targets

[Judgment call] Rust beats the realistic alternatives — staying on Go, porting to TypeScript, or porting to F# — for this specific binary:

- **Stay on Go**: zero cost today but accumulates as the Rust footprint elsewhere grows. Doesn't address tooling-stack consolidation.
- **Port to TypeScript**: would require Node.js startup overhead on every pre-commit hook fire (hundreds of times a week). Slower, heavier toolchain.
- **Port to F#**: aligns with the `organiclever-be` / `ose-app-be` / `crane-cli` direction, but .NET runtime startup penalty rules it out for a hook-fired CLI.
- **Port to Rust**: native binary, zero-runtime startup, type-system rigor matching the systems-level use case, and the toolchain is already required by the doctor.

## Affected roles

| Role                            | Impact                                                                                                                                                               |
| ------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Repository maintainer**       | Day-to-day commands stay identical. Compile-once binary (`cargo build --release`) replaces `go run main.go` on every fire — faster hook execution after first build. |
| **Contributor (any role)**      | `npm run doctor -- --fix` installs `rustup` + stable toolchain if missing; otherwise no change.                                                                      |
| **CI workflows**                | Per-workflow Setup step adds `actions-rust-lang/setup-rust-toolchain@v1` where rhino-cli is invoked. Build cache via `Swatinem/rust-cache@v2`.                       |
| **Pre-commit / pre-push hooks** | Same observable behavior, faster execution after first compile (warm cache on dev workstations and CI).                                                              |

## Business-level success criteria

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
