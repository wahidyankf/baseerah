# Unify rhino-cli, SDLC & Repo Structure Across the Three OSE Repos (Second Pass)

**Status**: In Progress
**Created**: 2026-07-02
**Authored in**: `ose-public` (this repo)
**Type**: Multi-file plan (5 documents) — **one giant 3-repo execution plan**
**Predecessor**: [`done/2026-07-01__standardize-rhino-cli-sdlc-parity`](../../done/2026-07-01__standardize-rhino-cli-sdlc-parity/README.md)

> This is a **single comprehensive plan that executes across all three repos** (`ose-public` →
> Phases 0–2, `ose-primer` → Phase 3, `ose-infra` → Phase 4, cross-repo verify → Phase 5), exactly
> like the first plan. It is a **second pass** whose north star is closing the gap between the first
> plan's _claimed_ `"identical"` end-state and the _actual_ divergence a fresh audit found.

## Context

The [first plan](../../done/2026-07-01__standardize-rhino-cli-sdlc-parity/README.md) standardized the
SDLC gate **mechanics** and the rhino-cli **target set / command set** across `ose-public`,
`ose-primer`, and `ose-infra`. It archived on 2026-07-01 with a large set of items marked done and a
handful marked deferred/`⚠️` ("functionally equivalent, mechanism differs, documented").

A fresh verification sweep (2026-07-02, this plan's Phase 0 pre-work) found that:

1. **Most of the first plan's _deferred_ items are already resolved** by post-archival follow-up
   commits (primer's echo-stubbed `specs:behavior:coverage` are now real; infra's env-guard is now the
   Rust command; `harness duplication` was re-wired; `cucumber` is now a dependency; `test:specs`
   landed; the domain-scoping gate is wired).
2. **But the headline `"identical"` claim is stale.** rhino-cli is _not_ identical across the three
   repos (three different points of a functional-core refactor: public 155 src files, primer 231,
   infra 235; infra differs from public in 100 of ~155 files with a different module-naming scheme and
   a `cli.rs` 132 lines longer). SDLC wiring is byte-identical between public and primer but
   **ose-infra diverges throughout** (`npx nx`/`npm run` wrappers instead of direct `cargo run`,
   inline tool-lint instead of lint-staged, Title-Case workflow names, missing CI jobs). Several
   smaller gaps and **two latent bugs** remain (see [tech-docs §2](./tech-docs.md#2-current-state-verified-2026-07-02)).

This second pass **re-audits against reality, ignores stale "done" notes, and drives all three repos
to a genuinely `"identical"` structure** — including the rhino-cli source itself, not just its target
set — so that working cross-repo is truly identical. Per the user directive: _"the rhino-cli should
also be 'identical', because the overall structure of the repo will be 'identical'."_

## Scope

**Same surface as the first plan** (see [first-plan scope](../../done/2026-07-01__standardize-rhino-cli-sdlc-parity/README.md#scope))
— every rhino-cli command, the full SDLC surface (commit-msg, pre-commit, pre-push, PR quality gate,
main-branch CI, env/markdown/specs/governance validation, CRON test+deploy pipelines), Nx target
names, per-project target contents, specs C4 structure, unified `repo-config.yml`, harness bindings,
and canonical GitHub CI — **plus** the following second-pass additions:

- **rhino-cli source `"identical"`** (new, load-bearing) — the Rust source, `Cargo.toml`,
  `Cargo.lock`, and `project.json` of `apps/rhino-cli` converge to one canonical form **100%
  byte-identical across all three repos, zero carve-outs** (infra relicensed to MIT; repo-specific
  inputs data-driven from `repo-config.yml`). See
  [tech-docs §4](./tech-docs.md#4-rhino-cli-source-identity-standard).
- **cucumber-rs BDD harness in all three** — primer's fully-wired harness (`tests/*.rs` +
  `specs/apps/rhino/behavior/rhino-cli/gherkin/*.feature`) becomes canonical and is present +
  passing identically in all three repos (public + infra currently declare the dep but wire nothing).
- **Full `namedInputs.specs` rollout** — every Nx-registered project in every repo (not the current
  16/29, 20/26, 6/8, counted against the full `nx show projects` graph — which includes the
  `*-contracts` projects rooted under `specs/apps/*/containers/contracts/`, invisible to a
  directory-only `apps`/`libs` scan) wires the specs input so a specs-only change is caught at
  pre-push/PR, not just main-ci.
- **Governance/docs convergence** — the reference docs, governance conventions, and `AGENTS.md`
  sections describing the standard stay identical across the three repos.
- **Latent-bug fixes** (root-cause, per repo policy) — the `.opencode/agent/` (singular) trigger-path
  bug that silently disables the agent-naming validator in public+primer; the missing
  `gherkin-cardinality` PR-gate step in public.
- **Zero `⚠️` tolerated** — every `⚠️` "functionally-equivalent mechanism divergence" row from the
  first plan's parity table converges to one identical mechanism.

**Out of scope** (legitimate divergence — carried forward from the first plan's
[divergence policy](./tech-docs.md#7-divergence-policy-allowed-vs-drift)):

- Which deployable apps each repo has, and therefore which per-app CRON deploy workflows exist.
- Which programming-language gates run (public = content/web; primer = polyglot demo backends;
  infra = coralpolyp + IaC).
- Infra-only IaC gates (terraform / ansible / yamllint) and the self-hosted runner label — the
  runner label is CI-workflow-layer allowed-divergence, **not** part of `apps/rhino-cli`.
- `apps/rhino-cli` has **zero carve-outs** — it is 100% byte-identical across all three repos
  (`src/`, `Cargo.toml`, `Cargo.lock`, `project.json`). infra's rhino-cli is relicensed to MIT
  (Decision 3) and every repo-specific input (env-validation scan paths) is data-driven from
  `repo-config.yml` (Decision 5), so nothing in `apps/rhino-cli` legitimately differs.
- `repo-config.yml` per-repo **data values** (domain-areas, ddd-areas, env-validation scan paths)
  differ by repo; its **schema, header comment, and harness list** are identical.
- Validator _behaviour_ (this plan standardizes wiring + source shape, not validator logic).
- **No new drift-enforcement tooling** — an automated parity check is explicitly _not_ built this
  pass (mission = verify-&-closeout, not tooling). Noted as a possible future follow-up.

## Approach Summary

1. **Phase 0** — fresh re-audit committed as evidence; clean baseline in all three repos.
2. **Phase 1** — synthesize the **canonical rhino-cli** in `ose-public` (pull primer's cucumber +
   testcoverage back into public; drive all repo-specific behaviour from `repo-config.yml`; fix the
   two latent bugs); finalize the canonical SDLC/docs standard.
3. **Phase 2** — converge `ose-public`'s own remaining gaps (full `namedInputs.specs`,
   `coverage.projects` registry, stale orphan spec, `gherkin-cardinality` PR-gate step).
4. **Phase 3** — propagate to `ose-primer` (align rhino-cli 5-file delta + cucumber version to
   canonical; `.opencode/agents` path; `*.cs/.clj/.dart` mechanism; full `namedInputs.specs`).
5. **Phase 4** — propagate to `ose-infra` (**largest workstream**: regenerate rhino-cli to canonical;
   `npx nx`/`npm run` → direct `cargo run`; inline tool-lint → lint-staged; add missing CI jobs;
   workflow renames; 6 projects' missing targets; full `namedInputs.specs`; wire cucumber). Isolated
   as a gated phase so it can be descoped without unwinding Phases 1–3.
6. **Phase 5** — cross-repo byte-identity verification + archival.

## Confirmed Decisions (user-ratified 2026-07-02)

Five decisions were grilled one-by-one and ratified:

1. **Canonical rhino-cli** = synthesize best-of-three in `ose-public` (pull primer's cucumber +
   testcoverage back into public), then propagate public→primer→infra.
2. **Infra rhino-cli** = full port to canonical, isolated as gated **Phase 4** (descopable without
   unwinding Phases 1–3).
3. **Infra rhino-cli license** = **relicense to MIT** — the CLI is dev tooling, not the proprietary
   `coralpolyp` app. No license carve-out.
4. **C#/Clojure/Dart formatters** = **native tools inline** (`dotnet csharpier format` / `cljfmt fix`
   / `dart format`); primer + infra converge to public's mechanism (drop `scripts/format-*.sh`).
5. **Env-validation scan paths** = **data-driven from `repo-config.yml`** so `project.json` is
   byte-identical everywhere. Combined with (3), `apps/rhino-cli` has **zero carve-outs** — 100%
   byte-identical across all three repos.

**No open questions remain.** The only sanctioned divergence anywhere is each repo's app/language set,
its infra-only IaC gates, and the self-hosted runner label (CI-workflow layer) — never inside
`apps/rhino-cli`.

## Navigation

- [brd.md](./brd.md) — why this matters (business rationale)
- [prd.md](./prd.md) — what "done" looks like (personas, user stories, Gherkin acceptance criteria)
- [tech-docs.md](./tech-docs.md) — verified current state, canonical standard, source-identity model,
  divergence policy, phase design
- [delivery.md](./delivery.md) — the phased execution checklist

## Related

- [First plan (predecessor)](../../done/2026-07-01__standardize-rhino-cli-sdlc-parity/README.md)
- [AGENTS.md §Related Repositories](../../../AGENTS.md) — the three-repo parity model
- [plan-multi-repo-parity-planning workflow](../../../repo-governance/workflows/plan/plan-multi-repo-parity-planning.md)
- [repo-governance/development/infra/nx-targets.md](../../../repo-governance/development/infra/nx-targets.md)
