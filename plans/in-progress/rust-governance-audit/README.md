---
title: Rust Governance Audit
status: in-progress
created: 2026-05-23
owners:
  - wahidyankf
scope:
  - ose-public
---

# Rust Governance Audit

End-to-end audit ensuring every Rust artefact in `ose-public` is **consistent**, **correct**, **up-to-date**, and **free of contradiction** — covering the authoritative standards documentation, the single active Rust crate (`rhino-cli`), the behavioural specifications under `specs/apps/rhino/`, cross-cutting governance touch-points, the developer agent and skill, and the code structure itself.

## Why this plan exists

Between the `rhino-cli` Go→Rust port (landed 2026-05-23, see [plans/done/2026-05-23\_\_rhino-cli-rust-rewrite/](../../done/2026-05-23__rhino-cli-rust-rewrite/)) and the recent `forbid(unsafe_code)` addition, governance drift accumulated:

- `specs/apps/rhino/README.md` still describes the Go-era tooling (`godog`, `.go` files, `//go:build integration` tags) — the spec README is now factually wrong about its own crate.
- The Rust toolchain version is declared in three places with three different values: `docs/.../rust/README.md` says `1.82+`, `coding-standards.md` example pins `1.82.0`, `Cargo.toml` has `rust-version = "1.88"`, and `rust-toolchain.toml` pins `1.95.0`.
- Three Cargo dependencies are behind upstream (`chrono` minor, `glob` patch, `sha2` **major**, `tempfile` 13 patches with a breaking rename); none triggers a CVE but the Dependency Bump Policy expects an explicit decision per crate.
- `unsafe` Rust is a hard-line non-goal; the `forbid(unsafe_code)` invariant must be encoded into governance docs (not just crate roots) so it cannot regress.
- Code structure compliance (module boundaries, error-handling shape, lint configuration, public API surface) has not been audited against the platform Rust standards since the port.

## Document map

| File                           | Purpose                                                                                                            |
| ------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| [README.md](./README.md)       | This overview                                                                                                      |
| [brd.md](./brd.md)             | Business rationale: drift cost, risk, success outcome                                                              |
| [prd.md](./prd.md)             | Requirements and Gherkin acceptance criteria                                                                       |
| [tech-docs.md](./tech-docs.md) | Audit dimensions, inventory matrix, web-verified currency table, contradiction catalogue, code-structure checklist |
| [delivery.md](./delivery.md)   | Phase-by-phase very granular task checklist                                                                        |

## Scope at a glance

**In scope**:

- `docs/explanation/software-engineering/programming-languages/rust/` (14 docs + templates)
- `apps/rhino-cli/` (Cargo.toml, rust-toolchain.toml, project.json, src/, tests/, scripts/, README.md)
- `specs/apps/rhino/` (README.md, behavior/cli/gherkin/\*.feature)
- `repo-governance/development/` files that name Rust (`infra/ci-conventions.md`, `infra/nx-targets.md`, `quality/code.md`, `quality/three-level-testing-standard.md`, `workflow/native-first-toolchain.md`, `workflow/dependency-bump-policy.md`, `agents/ai-agents.md`)
- `.claude/agents/swe-rust-dev.md` + `.opencode/agents/swe-rust-dev.md` (mirror)
- `.claude/skills/swe-programming-rust/SKILL.md`
- AyoKoding Rust learning content cross-references only (no content edits — separate platform)

**Out of scope**:

- Editing `apps/ayokoding-web/content/.../rust/` tutorials (separate `ayokoding-web` content plan)
- Adding new Rust crates
- Migrating any unrelated language's governance

## Success in one sentence

A new contributor reading any Rust artefact in this repo gets the same answer to "what Rust version, what dependency versions, what lint set, where is unsafe allowed" — every claim cited, every dependency justified, every spec scenario mapped to a real `rhino-cli` command, and no `unsafe` keyword anywhere in the source tree.

## Related plans

- [plans/done/2026-05-23\_\_rhino-cli-rust-rewrite/](../../done/2026-05-23__rhino-cli-rust-rewrite/) — the port that triggered most of this drift
- [plans/in-progress/2026-05-03\_\_cross-vendor-agent-parity/](../2026-05-03__cross-vendor-agent-parity/) — adjacent governance audit pattern reference
