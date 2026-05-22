# Rewrite rhino-cli from Go to Rust

**Status**: In Progress
**Scope**: `ose-public` — rewrite `apps/rhino-cli/` (Go) as a behavior-equivalent Rust binary, then archive the Go implementation to `archived/rhino-cli-go/`. Touches every downstream caller (10+ `apps/*/project.json` files, both `.husky/` hooks).
**Created**: 2026-05-22

## Problem

The current `rhino-cli` is the platform's repository-hygiene swiss army knife — ~30 commands across 11 namespaces (`agents`, `ddd`, `docs`, `doctor`, `env`, `git`, `repo-governance`, `specs`, `spec-coverage`, `test-coverage`, `workflows`) implemented in Go and sitting on the critical path for pre-commit hooks, pre-push hooks, CI workflows, and every other app's `test:quick` / `spec-coverage` Nx targets. We want to consolidate the platform's tooling on Rust for binary distribution, performance, type-system rigor, and toolchain ergonomics — without changing the observable behavior that the Gherkin specs in [`specs/apps/rhino/behavior/cli/gherkin/`](../../../specs/apps/rhino/behavior/cli/gherkin/) define.

## Goal

A drop-in Rust replacement that:

1. Passes every existing Gherkin scenario in `specs/apps/rhino/behavior/cli/gherkin/` at both unit (mocked) and integration (real `/tmp` fixtures) levels via `cucumber-rs`.
2. Produces byte-identical stdout/stderr for every documented happy-path command (text, JSON, and markdown output formats).
3. Maintains the line-coverage floor of 90% (validator metric) across the Rust crate.
4. Replaces every Go-binary call site (Nx project.json shell-outs, husky hooks, GitHub Actions workflows) with the Rust binary.
5. Leaves the Go implementation intact in `archived/rhino-cli-go/` for git-history traceability — never modified after archival.

## What changes

1. **New crate** at `apps/rhino-cli-rs/` during migration: Cargo workspace, single binary, modules mirroring the Go `internal/` layout. `clap` v4 + `cucumber-rs` + `assert_cmd` + `cargo-llvm-cov`.
2. **Phased command port**: critical-path commands first (`test-coverage validate`, `spec-coverage validate`, output formatter), then governance, docs, agents/workflows, specs/ddd, doctor/env/git, test-coverage helpers. Each command shadow-diffs against the Go binary before its Nx target switches.
3. **Caller graph migration**: every `apps/*/project.json` that shells out to `go run -C apps/rhino-cli main.go ...` flips to the Rust binary as soon as the relevant command is ported.
4. **Husky hooks**: `.husky/pre-commit` flips `git pre-commit` invocation; `.husky/pre-push` already routes through Nx targets and only sees the swap implicitly.
5. **Archival**: after a clean two-CI-run soak window with zero Go-binary references, `git mv apps/rhino-cli archived/rhino-cli-go/` and `git mv apps/rhino-cli-rs apps/rhino-cli`. Update `archived/README.md` table.
6. **ose-primer sync** (downstream propagation) is out of scope here — handled by a separate follow-up plan once `ose-public` stabilizes on Rust.

## Documents

- [brd.md](./brd.md) — why move from Go to Rust
- [prd.md](./prd.md) — product requirements + Gherkin acceptance criteria
- [tech-docs.md](./tech-docs.md) — architecture, dependency choices, shadow-diff mechanics, archival mechanics
- [delivery.md](./delivery.md) — phased step-by-step checklist (Phase 0–8)

## Non-Goals

- Behavior changes — the Rust port must not "improve" any command's behavior beyond byte-identical parity.
- Re-architecting the command surface — same namespaces, same flags, same exit codes.
- Porting `apps/ayokoding-cli` or `apps/ose-cli` — those Go siblings stay Go for now.
- Touching the `ose-primer` downstream template's own `rhino-cli` — propagation is a follow-up plan.
- Rewriting `libs/golang-commons` — the new Rust crate replaces its consumers; the lib stays for the two Go siblings.
