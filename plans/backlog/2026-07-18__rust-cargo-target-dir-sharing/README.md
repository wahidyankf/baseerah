# Rust `target/` Directory Sharing Across Worktrees

> Five-document plan. This README is the navigation hub; substantive content lives in the
> sibling documents.

## Navigation

- [Business Requirements (`brd.md`)](./brd.md) — WHY this exists
- [Product Requirements (`prd.md`)](./prd.md) — WHAT gets built (user stories + Gherkin)
- [Technical Documentation (`tech-docs.md`)](./tech-docs.md) — HOW it is built
- [Delivery Checklist (`delivery.md`)](./delivery.md) — DO (phased, executable)
- [Learnings Log (`learnings.md`)](./learnings.md) — Knowledge Capture running log

## Context

Rust build artifacts under `target/` are large and are **duplicated per git worktree**. Each
worktree of a repo gets its own `apps/<crate>/target/` (and `libs/<crate>/target/`), so the same
crates are recompiled and stored many times over. Observed footprint: roughly ten worktrees at up
to 11 GB each, ~32 GB total, most of it redundant copies of identical debug/release/test/incremental
artifacts. [Judgment call: figures are the maintainer's observed estimate, not an instrumented
measurement.]

The baseline bloat comes from unstripped debug builds, the whole dependency tree compiled to
`.rlib`, debug + release + test artifacts kept side by side, and never-garbage-collected
`target/*/incremental/` caches.

## Scope

**In scope**

- A per-crate `target/` **symlink** into a shared, persistent cache
  (`$HOME/.cache/ose-cargo-target/<repo>/<crate>`), created at repo-init time.
- Wiring the symlink creation into the repo init/doctor path (`npm run doctor`) and into worktree
  provisioning, idempotently, via a `scripts/` helper — **without editing `apps/rhino-cli/**`** and
  therefore **without touching the rhino-cli byte-identity boundary\*\*. [Repo-grounded]
- A hard **CI guard**: the symlink is created for local development ONLY and must no-op under CI.
- Nx `outputs` adjustment for the three ose-public crates that currently cache `{projectRoot}/target`.
- Parallel application across all three repos — `ose-public`, `ose-primer`, `ose-infra` — tracked by
  this single plan (three peer PRs).
- Documented cleanup guidance to prevent regrowth (`cargo clean` / `cargo sweep`).
- An **optional, clearly-separated** secondary phase: a `[profile.dev]` debuginfo trim.

**Out of scope**

- Any edit to `apps/rhino-cli/src/**`, its `Cargo.toml`, `Cargo.lock`, or `project.json` in the core
  mechanism (the symlink is a filesystem concern, not a rhino-cli source concern). The optional
  Phase 5 debuginfo trim is the sole, explicitly-flagged exception and is byte-identity-coupled.
- Wiring `cargo-sweep` installation into `rhino-cli doctor`'s tool list (that would touch the
  byte-identity boundary + Gherkin) — cleanup stays documented/manual.
- Changing CI's build strategy or the self-hosted runner configuration.

## Approach summary

For each Rust crate in a repo, replace the plain `apps/<crate>/target` directory with a symlink to
`$HOME/.cache/ose-cargo-target/<repo>/<crate>`. Because `target/` is gitignored and every build
command's `cp apps/<crate>/target/release/<bin> …/dist/` resolves **through** the symlink, no tracked
`Cargo.toml` or `project.json` build command needs to change. Worktrees of the same repo+crate then
share one physical directory, eliminating cross-worktree duplication. See
[`tech-docs.md`](./tech-docs.md) for the full design and rejected alternatives.

## Delivery mode

`worktree-to-pr` (multi-repo — one peer PR per repo). See
[`delivery.md`](./delivery.md#delivery-mode-worktree-to-pr).
