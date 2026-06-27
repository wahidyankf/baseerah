# Standardize rhino-cli Checks & SDLC Commands Across the Three OSE Repos

**Status**: In Progress
**Created**: 2026-06-22
**Authored in**: `ose-public` (this repo)
**Type**: Multi-file plan (5 documents) — **one giant 3-repo execution plan**

> This is a **single comprehensive plan that executes across all three repos** (`ose-public` →
> Phases 0–2, `ose-primer` → Phase 3, `ose-infra` → Phase 4, cross-repo verify → Phase 5). The
> primer/infra phases carry their **own granular execution steps and per-project target matrices**
> ([§1.3b](./tech-docs.md#13b-per-project-target-matrix-post-implementation-ose-primer),
> [§1.3c](./tech-docs.md#13c-per-project-target-matrix-post-implementation-ose-infra)) — they are
> not deferred to a later "propagate" pass. The plan folder is copied into each sibling repo at the
> start of its phase so the same checklist drives execution there.

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
- **Nx target-name standardization** — every Nx target invoked by a hook/CI uses one canonical name (`test:unit`, `test:integration`, `test:e2e`, `test:quick`, `test:coverage`, `lint`, `typecheck`, `specs:behavior:coverage`, `specs:domain:coverage`, and the `{domain}:{work}` validation targets) identical across all three repos; the rhino-cli target set itself converges. **Formatting is removed as a per-project target** (no `format`/`format:check`) and handled by file-type lint-staged. See [tech-docs §1.1](./tech-docs.md#11-nx-target-name-standard-targets-invoked-by-hooksci) and [§4.1](./tech-docs.md#41-nx-target-name-drift-rhino-cli).
- **Testing-architecture & target-contents standard** — every project (direct child of `apps/`/`libs/`) declares the mandatory six targets (`test:unit`, `test:integration`, `test:e2e`, `test:quick`, `lint`, `typecheck`) even as `echo` placeholders, plus a native `test:coverage` (≥ 90%, replacing the removed rhino-cli `test-coverage`), `specs:behavior:coverage` (renamed from `specs:coverage`), and `specs:domain:coverage` on `*-be` backends; `test:quick` = typecheck→lint→test:unit in order; the three test levels consume the same Gherkin; BE integration is service-level, FE has none unless DB-backed, `test:e2e` is real only on `*-e2e`; pre-push ≡ PR gate run only `test:quick` (never integration/e2e); rhino-cli enforces feature-file consumption. See [tech-docs §1.2](./tech-docs.md#12-testing-architecture--target-contents-standard) and the symmetric [per-project target matrix §1.3](./tech-docs.md#13-per-project-target-matrix-post-implementation-ose-public).
- **rhino-cli command-naming standardization** — every CLI leaf command converges to **verb-last** `{domain} {sub-domain…} {verb}` (e.g. `convention validate emoji` → `convention emoji validate`), while Nx/`project.json` targets stay `:`-separated `{domain}:{work}`/lifecycle. See [tech-docs §2.0](./tech-docs.md#20-two-naming-conventions-locked) and the triage target column.
- **Unified repo configuration** — merge `instruction-size-budget.yaml` + `env-contract.yaml` + `env-injection.yaml` into a single root `repo-config.yml` (namespaced sections) in all three repos. See [tech-docs §1.1a](./tech-docs.md#11a-unified-repo-configuration-repo-configyml).
- **Codecov removal** — no third-party coverage service in any repo (native `test:coverage` only); delete the last live `ose-infra/codecov.yml` + scrub stale references.
- **Standardized GitHub CI for every project** — the plan completes when every project across all three repos is covered by a GitHub CI whose workflow filenames + job structure follow the canonical **ose-public** convention (`pr-quality-gate.yml`, `validate-markdown.yml`, `validate-env.yml`, `main-ci.yml`). See [tech-docs §4.2](./tech-docs.md#42-github-ci-workflow-inventory-current--target-per-repo).
- **Identical-result invariant** — the end-state of the entire standardization layer (rhino-cli command set + verb-last naming, `:`-separated Nx target conventions, `repo-config.yml` schema, hook/gate mechanics + step order, lint-staged map, canonical CI workflow names) is **identical across all three repos**, so working cross-repo feels identical, logical, and intuitive. The only divergence is each repo's project/app set. See [tech-docs §1 north-star](./tech-docs.md#1-target-standard-best-of-three-synthesis).
- A single **target standard** for gate mechanics, derived best-of-three.
- Per-repo convergence edits to reach that standard.

**Out of scope** (legitimate divergence — see [tech-docs.md §Divergence Policy](./tech-docs.md#3-divergence-policy-allowed-vs-drift)):

- Which deployable apps each repo has, and therefore which per-app CRON deploy workflows exist.
- Which programming-language gates run (public = content/web apps; primer = polyglot demo backends; infra = coralpolyp + IaC).
- Infra-only IaC gates (terraform / ansible / yamllint).
- The behaviour of individual validators (this plan standardizes **wiring**, not validator logic).

## Approach Summary

1. **Phase 0** — environment baseline in `ose-public`.
2. **Phase 1** — author the committed analysis artifacts (command triage, SDLC + testing-architecture standard) under `docs/reference/`; extend the canonical Nx naming docs; add the `specs:behavior:coverage --require-consumption` rhino-cli behaviour.
3. **Phase 2** — converge `ose-public`: target names, hooks, workflow renames, per-project target-contents sweep, **post-merge CI + per-project staging deploy** (2f).
4. **Phase 3** — propagate to `ose-primer` and converge it with **its own granular sub-steps** (3a–3e), including the 26-project mandatory-six sweep and template-mode post-merge CI.
5. **Phase 4** — propagate to `ose-infra` and converge it with **its own granular sub-steps** (4a–4e), including coralpolyp staging deploy and recorded IaC divergence.
6. **Phase 5** — cross-repo parity verification (incl. the per-project matrices and post-merge behaviour) + archival.

The pre-merge gate (pre-push ≡ PR) is `test:quick` only; **`test:integration` + `test:e2e` + staging deploy run post-merge per affected project** (§1.4), with the old stag-deploy CRONs kept as nightly fallback.

## Navigation

- [brd.md](./brd.md) — why this matters (business rationale)
- [prd.md](./prd.md) — what "done" looks like (personas, user stories, Gherkin acceptance criteria)
- [tech-docs.md](./tech-docs.md) — the command triage, SDLC matrix, target standard, drift catalog, and diagrams
- [delivery.md](./delivery.md) — the phased execution checklist

## Related

- [AGENTS.md §Related Repositories](../../../AGENTS.md) — the three-repo parity model
- [plan-multi-repo-parity-planning workflow](../../../repo-governance/workflows/plan/plan-multi-repo-parity-planning.md) — the propagation mechanism for Phases 3–4
- [repo-governance/development/infra/nx-targets.md](../../../repo-governance/development/infra/nx-targets.md) — canonical Nx target names
