# Standardize rhino-cli Checks & SDLC Commands Across the Three OSE Repos

**Status**: In Progress
**Created**: 2026-06-22
**Authored in**: `ose-public` (this repo) — propagated to `ose-primer` and `ose-infra` via the multi-repo parity loop
**Type**: Multi-file plan (5 documents)

## Context

The three sibling repos — `ose-public`, `ose-primer`, `ose-infra` — each ship the same `rhino-cli`
Rust tool and the same SDLC quality machinery (commit-msg, pre-commit, pre-push, PR quality gate,
main-branch CI, markdown/env validation, and scheduled "test + deploy" CRON pipelines). Over time
the wiring has **drifted**: the same logical gate is named differently, scoped differently, placed
in a different workflow file, or invoked through a different mechanism (inline shell vs. an Nx-wrapped
rhino-cli target) in each repo.

This plan inventories **every** rhino-cli command, triages each as **wired** (invoked by some
lifecycle automation) or **not wired** (exists but only runnable manually), maps the full SDLC
surface across all three repos, derives a single **best-of-three target standard**, and converges
all three repos to produce `"identical"` gate **mechanics** — identical in hook ordering, gate
names, workflow filenames, validator sets, and invocation mechanism. App-set differences (which
deploy CRONs exist, which language gates run) remain legitimately divergent.

## Scope

**In scope** (all three repos):

- Triage of every `rhino-cli` subcommand → wired / not-wired (see [tech-docs.md §Command Triage](./tech-docs.md#2-rhino-cli-command-triage-wired-vs-not-wired)).
- Cross-repo SDLC matrix for: commit-msg, pre-commit, pre-push, PR quality-gate, main-branch CI, markdown-validate, env-validate, and the "test local + deploy stag" / "test stag + deploy prod" CRON pipelines.
- **Nx target-name standardization** — every Nx target invoked by a hook/CI uses one canonical name (`test:unit`, `test:integration`, `test:e2e`, `test:quick`, `lint`, `typecheck`, `format`, `format:check`, and the `{domain}:{work}` validation targets) identical across all three repos; the rhino-cli target set itself converges. See [tech-docs §1.1](./tech-docs.md#11-nx-target-name-standard-targets-invoked-by-hooksci) and [§4.1](./tech-docs.md#41-nx-target-name-drift-rhino-cli).
- A single **target standard** for gate mechanics, derived best-of-three.
- Per-repo convergence edits to reach that standard.

**Out of scope** (legitimate divergence — see [tech-docs.md §Divergence Policy](./tech-docs.md#3-divergence-policy-allowed-vs-drift)):

- Which deployable apps each repo has, and therefore which per-app CRON deploy workflows exist.
- Which programming-language gates run (public = content/web apps; primer = polyglot demo backends; infra = coralpolyp + IaC).
- Infra-only IaC gates (terraform / ansible / yamllint).
- The behaviour of individual validators (this plan standardizes **wiring**, not validator logic).

## Approach Summary

1. **Phase 0** — environment baseline in `ose-public`.
2. **Phase 1** — author the committed analysis artifacts (command triage, SDLC matrix, target standard) under `docs/reference/` so the standard is durable and reviewable.
3. **Phase 2** — converge `ose-public` to the standard.
4. **Phase 3** — propagate the plan + standard to `ose-primer` and converge it.
5. **Phase 4** — propagate the plan + standard to `ose-infra` and converge it.
6. **Phase 5** — cross-repo parity verification + archival.

## Navigation

- [brd.md](./brd.md) — why this matters (business rationale)
- [prd.md](./prd.md) — what "done" looks like (personas, user stories, Gherkin acceptance criteria)
- [tech-docs.md](./tech-docs.md) — the command triage, SDLC matrix, target standard, drift catalog, and diagrams
- [delivery.md](./delivery.md) — the phased execution checklist

## Related

- [AGENTS.md §Related Repositories](../../../AGENTS.md) — the three-repo parity model
- [plan-multi-repo-parity-planning workflow](../../../repo-governance/workflows/plan/plan-multi-repo-parity-planning.md) — the propagation mechanism for Phases 3–4
- [repo-governance/development/infra/nx-targets.md](../../../repo-governance/development/infra/nx-targets.md) — canonical Nx target names
