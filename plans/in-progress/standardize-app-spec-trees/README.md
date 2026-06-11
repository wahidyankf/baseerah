# Standardize App Spec Trees (Flat Product-Surface Parity)

**Status**: Not Started
**Stage**: in-progress
**Type**: Multi-file plan (BRD + PRD + tech-docs + delivery)

## Context

Every app family in this repo owns spec content under `specs/apps/<family>/`
([`specs-directory-structure.md`][conv]). Today two problems coexist:

1. **OSE spans two trees.** The `apps/ose-*` family is served by `specs/apps/ose-app/` (the GRC
   application) **and** `specs/apps/ose-platform/` (the marketing site + `ose-cli`), violating the
   one-family-one-tree rule that every other family follows. `[Repo-grounded]`
2. **Behavior surface names are bare and inconsistent.** The current convention names behavior
   dirs by perspective only — `behavior/be/gherkin/`, `behavior/web/gherkin/`, `behavior/cli/gherkin/`
   ([`specs-directory-structure.md` L168][conv]) — and two families still use a non-standard `api`
   perspective: `specs/apps/ose-platform/behavior/api/` and `specs/apps/ayokoding/behavior/api/`.
   `[Repo-grounded]`

This plan adopts a single **flat product-surface naming scheme** for behavior dirs across every
ose-public family, consolidates OSE into one `specs/apps/ose/` tree, renames every `api` surface to
`be`, codifies the scheme as the enforced standard (convention + `specs-checker` + `specs-maker`),
and writes a rationale doc. It is a **planning deliverable** — no migration is executed here.

### Flat product-surface naming scheme (LOCKED)

Behavior dirs become `specs/apps/<family>/behavior/<product>-<surface>/gherkin/`:

- **Multi-product family (OSE)** — product tokens distinguish products:
  `app-be`, `app-web`, `platform-be`, `platform-web`, `cli` (`cli` stays bare because the
  product's own name _is_ "cli": `ose-cli`).
- **Single-product multi-surface family** — the family name is the product token:
  `organiclever-be`, `organiclever-web`; `ayokoding-be`, `ayokoding-web`, `ayokoding-cli`,
  `ayokoding-build-tools`.
- **Single-surface family (echo, uniform)** — `crane-cli`, `rhino-cli`, `wahidyankf-web`.

This **replaces** the prior "two-tier" framing entirely.

## Scope

**In scope (active remediation, planned — every ose-public family):**

- **ose** — consolidate `specs/apps/ose-app/` + `specs/apps/ose-platform/` → `specs/apps/ose/`;
  behavior surfaces `app-be`, `app-web`, `platform-be` (from platform `api`), `platform-web`, `cli`;
  contracts Nx project `ose-app-contracts` → `ose-contracts`.
- **organiclever** — `behavior/be/` → `behavior/organiclever-be/`; `behavior/web/` →
  `behavior/organiclever-web/`.
- **ayokoding** — `behavior/api/` → `behavior/ayokoding-be/`; `behavior/web/` →
  `behavior/ayokoding-web/`; `behavior/cli/` → `behavior/ayokoding-cli/`; `behavior/build-tools/` →
  `behavior/ayokoding-build-tools/`.
- **crane** — `behavior/cli/` → `behavior/crane-cli/`.
- **rhino** — `behavior/cli/` → `behavior/rhino-cli/` (includes Rust source-default fixes).
- **wahidyankf** — `behavior/web/` → `behavior/wahidyankf-web/`.
- Rewrite every consumer reference for each family (`project.json` spec-coverage commands + inputs,
  `codegen -i`, e2e feature globs, `playwright.config.ts`, `steps/*.ts` `Covers:` comments, app and
  spec READMEs, governance/docs cross-refs, regenerated playwright-bdd artifacts).
- Amend [`specs-directory-structure.md`][conv] with the flat product-surface rule + `be`-over-`api`
  rule + worked examples (multi-product OSE; single-product organiclever).
- Update `.claude/agents/specs-checker.md` + `.claude/agents/specs-maker.md`; re-sync bindings
  (`npm run generate:bindings`).
- Write the rationale doc `docs/explanation/standardize-app-spec-trees-parity-decisions.md`.

**Out of scope:**

- Renaming any `apps/*` Nx project (apps keep their names; only specs move). Exception: the
  contracts spec project `ose-app-contracts` → `ose-contracts`, which lives under `specs/`.
- Splitting/merging app deployables; authoring new Gherkin scenarios or product features.
- Executing the migration — **this plan is a planning deliverable only**.
- The sibling repos' own restructuring (each has its own plan — see Sibling Plans).

## Approach Summary

Phased, each phase green before the next. Families are grouped so each phase is a natural pause:

- **Phase 0** — Environment setup + recorded clean baseline + reference-inventory reconciliation.
- **Phase A** — OSE consolidation (`ose-app` → app surfaces + contracts rename).
- **Phase B** — OSE platform surfaces (`platform-be` from `api`, `platform-web`, `cli`).
- **Phase C** — OSE C4/DDD framing merge + index; old trees removed.
- **Phase D** — organiclever flat product-surface rename.
- **Phase E** — ayokoding flat product-surface rename (incl. `api`→`ayokoding-be`).
- **Phase F** — echo + single-surface families (crane, rhino, wahidyankf), incl. rhino Rust
  source-default TDD.
- **Phase G** — Promote to standard: convention amendment, `specs-checker`/`specs-maker`, bindings
  re-sync, rationale doc, governance/docs sweep, conformance audit.

## Design Decisions

Resolved by the shared decisions brief (2026-06-11) — see
[tech-docs §Cross-Repo Deviation Matrix](./tech-docs.md#cross-repo-deviation-matrix):

| Decision              | Choice                                                                                 |
| --------------------- | -------------------------------------------------------------------------------------- |
| Surface naming scheme | **Flat product-surface** — `behavior/<product>-<surface>/gherkin/` across all families |
| Backend perspective   | **`be`, never `api`** — rename ose-platform `api` + ayokoding `api`                    |
| Blast radius          | **All ose-public families** restructure to conform (planning only)                     |
| OSE consolidation     | **Flat merge** — single `specs/apps/ose/`, surface-prefixed `behavior/`                |
| Contracts project     | **Move + rename** — `ose-app-contracts` → `ose-contracts`                              |
| Deliverable           | **Plan only** — write the plan, commit + push; migration executed later                |
| Delivery mode         | **main-to-main** — direct push to `origin/main` (docs-only, low risk)                  |

## Sibling Plans

This plan is one of three parallel parity plans (one per repo in the
`open-sharia-enterprise` ecosystem). All three adopt the identical flat product-surface rule;
each restructures its own families. `[Repo-grounded]` (this repo) /
`[Unverified]` (sibling-repo paths — they live in separate repositories not checked out here).

- **ose-primer** — `plans/done/2026-06-11__standardize-app-spec-trees/README.md`
  (restructures `crud`, `rhino`; convention text byte-identical to this plan's amendment).
- **ose-infra** — `plans/done/2026-06-11__standardize-app-spec-trees/README.md`
  (restructures `coralpolyp`, `rhino`; convention text adapted, outside the sync loop).

## Navigation

- [brd.md](./brd.md) — business rationale (WHY)
- [prd.md](./prd.md) — product requirements + Gherkin acceptance criteria (WHAT)
- [tech-docs.md](./tech-docs.md) — target layout, migration maps, full consumer impact, deviation
  matrix, rollback (HOW)
- [delivery.md](./delivery.md) — phased delivery checklist (DO)

[conv]: ../../../repo-governance/conventions/structure/specs-directory-structure.md
