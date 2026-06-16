# ayokoding-www Salary Savings Calculator

Add a bilingual (en/id) interactive tool to `apps/ayokoding-www` that estimates how much money a
person can save per month given a salary, across major tech-hub cities worldwide. Results show
savings as both a **percentage** of salary and an amount in the city's **local currency**.

## Status

In progress (created 2026-06-16)

## Context

`ayokoding-www` is a bilingual educational site (Next.js 16, App Router, `[locale]` routing). Today
its pages are markdown-driven content. This plan adds the site's first **interactive tool page** — a
client-side calculator — establishing a `tools/` area that future calculators can reuse.

The tool answers a practical question for tech workers and relocation planners: _"For this salary,
where in the world do I save the most, and how much?"_

## Scope

**In scope:**

- New interactive route `/[locale]/tools/salary-savings` (client component).
- Two modes via a tab toggle:
  - **Compare all** — enter one salary (USD); table lists tech-hub cities with estimated living
    cost (local currency + USD), savings %, and savings in local currency; sortable by savings.
  - **Single city** — pick one city, enter salary in that city's local currency; show a breakdown.
- Shared cost-basis controls (apply to both modes):
  - **Household type** — single, married, or married with 1–3 kids (scales living cost).
  - **Area** — city center vs rural (discounts living cost outside the center).
  - **School type** — public vs private median per-child cost, shown only when the household has
    kids (adds schooling on top of living cost).
- Living cost **assumes public transport** (a typical monthly transit pass) in every city; car
  ownership/fuel/parking is not modelled (fixed v1 assumption, not a toggle).
- A **static, hand-curated dataset** covering **as many tech-hub cities worldwide as we reasonably
  can** (`cities.ts`): name (en/id), country, currency, single-person city-center monthly living
  cost, median public/private school cost per child, and an FX-to-USD snapshot rate with a recorded
  snapshot date — plus shared household/area multiplier tables.
- The whole feature is **client-side rendered (CSR)** — a `'use client'` page that computes
  everything in the browser; no server-side rendering of results, no backend, no runtime network.
- Pure calculation functions in a separate module, fully unit-tested (TDD).
- UI built from the shared `@open-sharia-enterprise/web-ui` kit. The kit already covers tabs,
  inputs, toggles, dropdown radio groups, command/combobox, badges, alerts, and cards; the one
  missing piece — a **`Table`** primitive for the Compare-all view — is added to `libs/web-ui` as a
  prerequisite (see delivery Phase 2).
- Bilingual UI strings (en/id) via the existing i18n mechanism.
- Vitest unit tests for the calculation module and component; one fe-e2e smoke test.

**Out of scope (future iterations):**

- Live cost-of-living or FX APIs (dataset is static for v1).
- Tax modelling, savings-rate goals, currency other than the city default.
- Israeli cities are deliberately excluded from the dataset. This is a country-level choice about
  the state of Israel and its political stance, **not** a choice about any ethnic, racial, or
  religious group. People of any background are out of scope of the exclusion — only the country
  Israel and its political stance are.
- Persisting user inputs, sharing/export, charts.

## Approach Summary

1. **Phase 0 — Setup & baseline**: worktree, deps, green baseline for `ayokoding-www`.
2. **Phase 1 — Data + calculation core (TDD)**: `cities.ts` dataset + pure `calc` module with tests.
3. **Phase 2 — Interactive page (TDD)**: add the missing `Table` primitive to `libs/web-ui`, then
   build the `/[locale]/tools/salary-savings` page, both modes, and component tests.
4. **Phase 3 — Bilingual strings + polish**: en/id UI strings, accessibility, responsive.
5. **Phase 4 — E2E + local quality gates**: fe-e2e smoke test, typecheck/lint/test:quick.
6. **Phase 5 — Post-push CI verification**.
7. **Phase 6 — Plan archival**.

## Worktree

Worktree path: `worktrees/ayokoding-www-salary-savings-calculator/`

Provision (run from repo root):

```bash
claude --worktree ayokoding-www-salary-savings-calculator
```

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

## Documents

| Document                       | Purpose                                                          |
| ------------------------------ | ---------------------------------------------------------------- |
| [brd.md](./brd.md)             | WHY — business rationale, affected roles, success metrics, risks |
| [prd.md](./prd.md)             | WHAT — user stories, Gherkin acceptance criteria, product scope  |
| [tech-docs.md](./tech-docs.md) | HOW — architecture, design decisions, file impact, dependencies  |
| [delivery.md](./delivery.md)   | DO — phased execution checklist                                  |
