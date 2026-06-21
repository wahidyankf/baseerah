# AyoKoding Calculator URL-State Exploratory Findings

## Context

**Testing goal**: Spec-aware exploratory verification of the newly-added URL state reflection
feature on the AyoKoding cost-of-living calculator. The feature serializes all nine calculator
controls (tab, region, country, city, adults, preschool, schoolkids, schooltype, area) to URL
query params. The dev server was at `http://localhost:3101` (Next.js 16, bilingual en/id, worktree
`ayokoding-calculator-url-state`).

**Target URLs tested**:

- `http://localhost:3101/en/tools/cost-of-living-calculator` (English, default)
- `http://localhost:3101/id/tools/cost-of-living-calculator` (Indonesian)
- `http://localhost:3101/en/tools/cost-of-living-calculator?city=singapore` (deep link)
- Multiple dirty/out-of-range/contradictory params for canonicalization tests

**Ground truth**:
`specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/cost-of-living-calculator.feature`

**Date observed**: 2026-06-21

**Browser/environment**: Playwright Chromium (headless), macOS Darwin 24.5.0, Next.js 16 dev server

## Testing Charters

Three charters were executed:

**Charter 1 — URL state serialization**
Explore all nine controls with select changes at desktop breakpoints (en + id)
to discover which controls correctly update the URL and which do not.

**Charter 2 — URL state deserialization (deep links, canonicalization, back button)**
Explore dirty, contradictory, and out-of-range params
to discover canonicalization, backfill, and history-management defects.

**Charter 3 — Breadcrumb, locale integrity, accessibility, and security surface**
Explore the breadcrumb at 375 / 1280 px in en + id locales, HTML lang attribute, page titles,
security headers, and touch targets
to discover locale, accessibility, and passive-security findings.

## Coverage Map

### Dimensions tested

| Dimension                                       | Status            | Notes                                                          |
| ----------------------------------------------- | ----------------- | -------------------------------------------------------------- |
| Functional flows — URL encode/decode round-trip | Covered           | All 9 controls verified                                        |
| Functional flows — deep link backfill           | Covered           | city, country, region combinations                             |
| Functional flows — canonicalization             | Covered           | dirty city, OOB adults, full country name                      |
| Functional flows — cascade-clear                | Covered           | region change, country change                                  |
| Functional flows — back button                  | Covered           | tab nav, city selection, canonicalize-no-history               |
| Functional flows — reload restores state        | Covered           | tab=savings, adults=2 preserved on reload                      |
| Breadcrumb — en and id locales                  | Covered           | href correct at both 375 px and 1280 px                        |
| Responsive breakpoints — 375 px                 | Covered           | mobile breadcrumb, hamburger, touch target                     |
| Responsive breakpoints — 1280 px                | Covered           | primary test viewport                                          |
| Responsive breakpoints — 768 / 1024 / 1440 px   | Not covered       | viewport budget                                                |
| Accessibility — html lang attribute             | Covered           | en and id both correct                                         |
| Accessibility — keyboard navigation             | Not covered       | playwright interaction only, no tab-key test                   |
| Accessibility — screen-reader / aria            | Partially covered | aria-labels on select elements confirmed                       |
| Performance (Core Web Vitals)                   | Not covered       | dev server; Lighthouse not run                                 |
| Cross-browser (Safari, Firefox, Edge)           | Not covered       | Chromium only                                                  |
| Security surface (passive)                      | Partially covered | x-powered-by, x-content-type-options, x-frame-options observed |

### Specs scenarios mapped

All 13 URL-state scenarios added to the feature file (URL-001 through URL-013) were exercised
against the live target. Every scenario passed.

All previously-existing spec scenarios are not in scope for this run (URL-state feature focus).

#### Spec scenarios — Covered + passing (URL-state scenarios, URL-001 through URL-013)

| Scenario                                                          | Result |
| ----------------------------------------------------------------- | ------ |
| URL-001 Out-of-range adults=4 reset to default                    | PASS   |
| URL-002 Full country name country=Indonesia dropped               | PASS   |
| URL-003 Selecting city backfills country and region               | PASS   |
| URL-004 Region change clears incompatible country+city            | PASS   |
| URL-005 Contradictory region+city deep link — city wins           | PASS   |
| URL-006 City-detail back link preserves parent geo scope          | PASS   |
| URL-007 Tab change written to URL                                 | PASS   |
| URL-008 Cost-basis control change written to URL                  | PASS   |
| URL-009 Breadcrumb offers Home and Tools escape links             | PASS   |
| URL-010 Region selection writes region to URL                     | PASS   |
| URL-011 City deep link restores city and backfills country+region | PASS   |
| URL-012 Unknown city param dropped on load                        | PASS   |
| URL-013 Canonicalization does not add browser history entry       | PASS   |

#### Spec scenarios — Covered + diverging

None found.

#### Spec scenarios — Behaviours observed but not yet in specs

See `spec-gaps.md` for the four proposed additions.

## Risk Summary

**Overall impression**: The URL state feature is solid. All 70 Playwright checks (49 in the
primary suite, 21 in the edge-case suite) passed with zero failures. The core encode/decode/
canonicalize/backfill/cascade-clear logic is well-tested by both the unit test suite and the
live-site Playwright verification.

**Top risks**:

1. No finding of functional severity; the feature behaves as the spec describes.
2. One minor finding: the breadcrumb "Calculator" label is hardcoded in English on the
   `/id/` locale (see `findings.md` EWT-001). This is a Minor/High-priority defect because
   the breadcrumb is user-visible and the id locale is fully supported.
3. `Strict-Transport-Security` header is absent on the dev server (expected in dev; must be
   confirmed present in production). Noted as a passive-security observation, not a finding.
4. The `CalculatorBreadcrumb` component is a bespoke reimplementation rather than the shared
   `Breadcrumb` design-system primitive — this design debt was already filed separately by the
   `web-design-tester` as DWT-B-004 and is not duplicated here.

## Document Map

- **`README.md`** (this file) — context, charters, coverage map, risk summary
- **`brd.md`** — business framing of the findings
- **`prd.md`** — user stories and Gherkin acceptance criteria describing the corrected behaviour
- **`findings.md`** — defect catalog (1 Minor finding)
- **`spec-gaps.md`** — 4 proposed spec additions for unprotected but correct behaviours
- **`evidence/`** — 26 committed screenshots (one per test phase / locale / breakpoint)

`tech-docs.md` and `delivery.md` are not authored here — they are produced when this plan is
promoted to `plans/in-progress/` via `plan-maker`.
