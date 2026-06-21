# AyoKoding Cost-of-Living Calculator — URL State Reflection

> **Status**: In Progress
> **Stage**: `plans/in-progress/ayokoding-calculator-url-state/`
> **App**: `ayokoding-www` (Next.js 16, port 3101)
> **Worktree**: `worktrees/ayokoding-calculator-url-state/`

## Context

The cost-of-living calculator at `/[locale]/tools/cost-of-living-calculator` exposes three control
groups — the tab (`cost | savings | min-role`), the geo filters (region / country / city), and the
cost-basis controls (household size, school type, area). Today only a subset of this state reaches
the URL, and it does so inconsistently:

- Tab changes are **read** from the URL on mount but **never written back** (`handleTabChange` in
  `calculator-content.tsx` never calls the router) `[Repo-grounded]`.
- Geo filters persist only `city` OR `country` (city wins); `region` is **never** persisted
  `[Repo-grounded]`.
- The cost-basis controls (adults, preschool kids, school kids, school type, area) are **never**
  serialized `[Repo-grounded]`.
- State lives in **three drifting copies** — `GeoFilters` holds its own `useState`,
  `calculator-content` holds a duplicate `geoScope`, and the URL holds a third partial view
  `[Repo-grounded]`.

The result: back-navigation and bookmarking silently reset most of the user's selections, deep
links only partially restore, and invalid params (`?country=Indonesia` full-name,
`?city=atlantis`) leave the page in a broken or stale state.

## Scope

This plan makes **the URL the single source of truth** for all nine calculator controls, with
backfill, cascade-clear, sanitize, and canonicalize semantics, plus on-page navigation escape
links and full spec/test coverage.

### In scope

- Serialize all nine controls to the query string: `tab`, `region`, `country`, `city`, `adults`,
  `preschool`, `schoolkids`, `schooltype`, `area`.
- Refactor `calculator-content.tsx`, `geo-filters.tsx`, and `controls.tsx` to be controlled by the
  URL (`useSearchParams` derives state; `useRouter` writes it). Eliminate the three-way drift.
- **Backfill**: selecting a narrow filter (city) backfills broader ones (country + region).
- **Cascade-clear**: selecting a broader filter (region) clears now-impossible narrower ones
  (country + city) — the user's canonical Singapore → Europe example.
- **Sanitize + canonicalize on load**: drop unknown/invalid/out-of-range params, resolve
  region/city conflicts (narrower wins), then rewrite to canonical clean form via `router.replace`.
- **Clean URLs**: omit default values; a pristine calculator has a bare URL.
- **Nav escape links**: a breadcrumb (Home / Tools / Calculator) so users can always escape the
  `router.push` history stack.
- **Spec reconciliation + new scenarios** in
  `specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/cost-of-living-calculator.feature`.
- **Unit tests** for the new pure `core/url-state.ts` (encode/decode/sanitize/cascade/backfill).
- **Playwright FE e2e** (`ayokoding-www-fe-e2e`) for URL round-trip, back-button stepping, and
  deep-link restore.
- **Absorb and close** three URL-related tester findings (see below).

### Out of scope

- Visual/design findings (tab-bar overflow, dark-mode) — the
  `2026-06-21__ayokoding-www-cost-of-living-design-findings` plan owns these.
- Savings-tab salary-input behavior, new cities/data, any non-URL usability finding.

## Absorbed Findings (closed by this plan)

| Finding           | Source plan                                                                     | Summary                                                                      |
| ----------------- | ------------------------------------------------------------------------------- | ---------------------------------------------------------------------------- |
| **UWT-005**       | `plans/backlog/2026-06-21__ayokoding-calculator-usability-findings/findings.md` | Tab state not reflected in URL; back-nav/bookmark reset to "Cost of living". |
| **UWT-010**       | same `findings.md` (back-link)                                                  | "Back to all cities" link `href="?tab=cost"` drops region/country state.     |
| **EXP deep-link** | `plans/backlog/2026-06-21__ayokoding-calculator-exploratory-findings/`          | `?country=Indonesia` (full name) does not restore — covered by sanitize.     |

## Document Map

| File                             | Purpose                                                                    |
| -------------------------------- | -------------------------------------------------------------------------- |
| [`brd.md`](./brd.md)             | Business rationale — why URL state matters, impact, risks                  |
| [`prd.md`](./prd.md)             | Product requirements — personas, user stories, Gherkin acceptance criteria |
| [`tech-docs.md`](./tech-docs.md) | Architecture, the `core/url-state.ts` design, file impact, test strategy   |
| [`delivery.md`](./delivery.md)   | Phased, TDD-shaped, `[AI]`/`[HUMAN]`-tagged delivery checklist             |

## Approach Summary

A new pure module `core/url-state.ts` holds `encodeState`, `decodeState`, and
`sanitizeState`/`canonicalize` — FCIS pure core, fully unit-tested, reusing the existing
`core/geo-filter.ts` helpers (`countriesForRegion`, `citiesForCountry`, `scopedCities`) for
backfill and cascade-clear. The shell (`calculator-content.tsx` + the two control components) calls
these functions with `useSearchParams` / `useRouter`, becoming thin controlled views over the URL.

## Definition of Done

1. All nine params round-trip (write-on-change + restore-on-load); cascade-clear and backfill work.
2. Sanitize + canonicalize on load rewrites invalid/contradictory params to canonical clean form.
3. Nav escape links (Tools index + Home) present on the calculator page.
4. Specs reconciled + new scenarios; unit tests for the pure module; Playwright e2e; three absorbed
   findings marked closed.
