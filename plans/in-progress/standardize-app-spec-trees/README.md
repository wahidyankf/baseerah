# Standardize App Spec Trees (Consolidate OSE Specs)

**Status**: Not Started
**Stage**: in-progress
**Type**: Multi-file plan (BRD + PRD + tech-docs + delivery)

## Context

Every app family in this repo owns exactly one spec tree under `specs/apps/<family>/`
([`specs-directory-structure.md`][conv]). `organiclever-*` apps consume
`specs/apps/organiclever/`; `ayokoding-*`, `wahidyankf-*`, `crane-*`, `rhino-*` follow the same
one-family-one-tree shape.

**OSE is the lone outlier.** The `apps/ose-*` family is served by **two** spec trees:

- `specs/apps/ose-app/` — the GRC **application** (`app.oseplatform.com` / `api.oseplatform.com`):
  rich DDD with four bounded contexts (`regulatory-source`, `internal-policy`, `gap-analysis`,
  `ai-orchestration`), consumed by `ose-app-be`, `ose-app-web`, and their e2e suites.
- `specs/apps/ose-platform/` — the **marketing/updates site** (`oseplatform.com`) plus the
  `ose-cli` link validator, consumed by `ose-web`, `ose-web-fe-e2e`, `ose-web-be-e2e`, `ose-cli`.

This plan **consolidates both trees into a single `specs/apps/ose/`** so every `apps/ose-*`
project points at one OSE spec tree — matching the rest of the repo — and **promotes the
resulting shape to a repo-wide standard** by amending the specs convention to cover families
that ship multiple deployables under one brand.

## Scope

**In scope:**

- Merge `specs/apps/ose-app/` + `specs/apps/ose-platform/` → `specs/apps/ose/` (flat layout,
  surface-disambiguated `behavior/` subtrees).
- Rewrite every consumer reference (`project.json`, `playwright.config.ts`, step-file comments,
  app READMEs, codegen paths, `specs/README.md`).
- Rename the contracts Nx project `ose-app-contracts` → `ose-contracts` and move it to
  `specs/apps/ose/containers/contracts/`.
- Unify the backend-HTTP behavior perspective name from `api` (platform) to `be`.
- Normalize the `ose-cli` Gherkin into a single canonical `behavior/cli/gherkin/`.
- Amend [`specs-directory-structure.md`][conv] to standardize the **multi-deployable family**
  layout and the `be` perspective name; wire enforcement into `specs-checker`.
- Conformance audit: confirm all other `apps/` families already satisfy the standard.

**Out of scope:**

- Renaming any `apps/ose-*` project (apps keep their names; only specs move).
- Splitting/merging `apps/ose-*` deployables themselves.
- Authoring new Gherkin scenarios or product features (move + restructure only).
- Executing the migration — **this plan is a planning deliverable only** (see Approach).

## Approach Summary

Phased, each phase green before the next ([per grilling decision](#design-decisions)):

- **Phase 0** — Environment setup + recorded clean baseline.
- **Phase A** — Migrate `ose-app` → `specs/apps/ose/` app surfaces (`app-be`, `app-web`) +
  contracts project rename; rewrite `ose-app-*` consumers; green.
- **Phase B** — Migrate `ose-platform` → `specs/apps/ose/` platform surfaces (`platform-be`,
  `platform-web`, `cli`), `api`→`be` rename, cli normalize; rewrite `ose-web*`/`ose-cli`
  consumers; green.
- **Phase C** — Merge C4 framing (`product`, `system-context`, `containers`, `components`,
  `ddd`, `README`) into unified OSE docs; update `specs/README.md`; green.
- **Phase D** — Promote to standard: amend the specs convention for multi-deployable families,
  update `specs-checker`, run conformance audit, sweep `AGENTS.md` + docs cross-refs.

## Design Decisions

Resolved via pre-write grilling (2026-06-11):

| Decision            | Choice                                                                                |
| ------------------- | ------------------------------------------------------------------------------------- |
| Consolidation model | **Flat merge, disambiguate** — single `specs/apps/ose/`, surface-prefixed `behavior/` |
| Deliverable         | **Plan only** — write the plan, commit + push; migration executed later               |
| Perspective naming  | **Unify to `be`** — platform `api` → `be`                                             |
| `ose-cli` layout    | **Normalize** — fold into single canonical `behavior/cli/gherkin/`                    |
| C4 framing dirs     | **Unified single docs** — merge two C4/DDD models into one framing set                |
| Contracts project   | **Move + rename** — `ose-app-contracts` → `ose-contracts`                             |
| Sequencing          | **Phased** — app first, then platform                                                 |

## Navigation

- [brd.md](./brd.md) — business rationale (WHY)
- [prd.md](./prd.md) — product requirements + Gherkin acceptance criteria (WHAT)
- [tech-docs.md](./tech-docs.md) — target layout, migration map, file impact, rollback (HOW)
- [delivery.md](./delivery.md) — phased delivery checklist (DO)

[conv]: ../../../repo-governance/conventions/structure/specs-directory-structure.md
