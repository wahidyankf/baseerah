# Exploratory Testing: AyoKoding Cost-of-Living Calculator

## Context

Spec-aware exploratory testing of the AyoKoding cost-of-living calculator at
`http://localhost:3101/en/tools/cost-of-living-calculator` (and `/id/` locale) on 2026-06-21.
The running app was the `ayokoding-www-cost-of-living-calculator-test-fixing` worktree.

**Tester**: `web-exploratory-tester` (sonnet-tier agent, non-destructive observational pass)

**Testing depth**: Standard — charters across all three tabs, both locales, key breakpoints,
edge-case inputs, and spec-scenario validation.

## Target URLs and Environment

| Property      | Value                                                            |
| ------------- | ---------------------------------------------------------------- |
| EN base URL   | `http://localhost:3101/en/tools/cost-of-living-calculator`       |
| ID base URL   | `http://localhost:3101/id/tools/cost-of-living-calculator`       |
| Tools index   | `http://localhost:3101/en/tools`                                 |
| App           | `ayokoding-www` (Next.js 16, App Router, TypeScript)             |
| Worktree      | `worktrees/ayokoding-www-cost-of-living-calculator-test-fixing/` |
| Date observed | 2026-06-21                                                       |
| Browser       | Chromium (headless, Playwright)                                  |
| Breakpoints   | 375 px, 768 px, 1280 px, 1440 px                                 |
| Locales       | en, id (both tested)                                             |

## Testing Goal

Verify functional correctness, interactive flows, and locale fidelity across all three calculator
tabs (Cost of living, Savings, Minimum role). Compare live behaviour against the Gherkin spec at
`specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/cost-of-living-calculator.feature`.

## Charters Run

1. **Tab navigation and label fidelity** — Explore tab labels, accessibility roles, and locale
   translation to discover label or translation defects.
2. **Savings tab: salary input and annual gross** — Explore the salary input → annual derivation →
   net calculation to discover arithmetic or display defects.
3. **Minimum role tab: baseline source flows and zero-target edge** — Explore all three baseline
   source modes and the zero-savings-target edge to discover ranking and edge-case defects.
4. **Area toggle, school type, and household controls** — Explore SegmentedControls and selects to
   discover responsive/interactive defects.
5. **Geo filters, URL params, and deep-link restoration** — Explore country/city/region cascades
   and URL sync to discover state-management defects.
6. **Locale: EN vs ID parity** — Explore the `/id/` locale to discover untranslated strings and
   locale-specific defects.
7. **Mobile (375 px) and tablet (768 px) breakpoints** — Explore responsive card layout, tab
   visibility, and touch targets.
8. **Tools index link and page title** — Explore the tools index and browser title for navigation
   and metadata defects.

## Coverage Map

### Dimensions covered

| Dimension                        | Covered       | Notes                                                                         |
| -------------------------------- | ------------- | ----------------------------------------------------------------------------- |
| Tab navigation (all 3 tabs)      | Yes           | EN + ID, 375/768/1280/1440 px                                                 |
| Salary input → annual gross      | Yes           | Verified 8000 × 12 = 96,000                                                   |
| Net take-home after tax          | Yes           | Dubai (UAE 0%) correctly passes through; other cities taxed                   |
| Area toggle (SegmentedControl)   | Yes           | Rural confirmed to lower housing; radiogroup confirmed present                |
| School type toggle visibility    | Yes           | Hidden with 0 school kids; appears with 2 school kids                         |
| School type: public vs private   | Yes           | Private correctly more expensive (23× ratio on first city)                    |
| Household: adults + childcare    | Yes           | 2-adult sub-linear scaling confirmed; preschool → childcare correct           |
| Geo filters cascade              | Yes           | Region → Country → City cascade works; URL updated                            |
| URL state (filter → URL)         | Yes           | Country filter writes `?country=id` to URL                                    |
| Deep-link restoration            | Partial       | `?country=id` restores; `?country=Indonesia` (full name) does not             |
| Minimum role: savings target     | Yes           | 8000 target shows 1 marker + divider                                          |
| Minimum role: zero target edge   | Yes           | DEFECT: no marker shown for zero target                                       |
| Minimum role: reference role     | Yes           | UI inputs render                                                              |
| Minimum role: my salary          | Yes           | UI inputs render                                                              |
| Cost-of-living table structure   | Yes           | Country left of City; all columns present                                     |
| OOP legend                       | Yes           | Visible on Cost of Living tab                                                 |
| No Israeli cities                | Yes           | None found in any tab                                                         |
| Data last updated + disclaimer   | Yes           | Both visible                                                                  |
| ID locale: tabs, labels, caption | Yes           | All correctly translated                                                      |
| Mobile (375 px) cards            | Yes           | Savings and role cards render                                                 |
| Page title                       | Yes           | DEFECT: duplicate "AyoKoding" suffix                                          |
| Tools index                      | Yes           | Calculator link present                                                       |
| Security headers                 | Yes (passive) | X-Frame-Options, CSP, X-Content-Type-Options present; HSTS absent (local dev) |
| Empty-salary deficit + em dash   | Yes           | Correctly shows negative savings with "—" percentage                          |
| Sub-national indicator           | Yes           | Shows "(fed+state)" for US/CA/CH cities                                       |
| Sorting by savings               | Yes           | Sort button present                                                           |

### Dimensions not covered

| Dimension                         | Reason not covered                                               |
| --------------------------------- | ---------------------------------------------------------------- |
| Cross-browser (Safari, Firefox)   | Out of scope for this run; Chromium only                         |
| Performance (Lighthouse / CWV)    | Not requested; qualitative observation only (load appeared fast) |
| Full WCAG automated scan          | Not requested; keyboard and aria spot-checked only               |
| Healthcare scheme badge rendering | Not explicitly checked in isolation; badge presence not verified |
| Lifestyle savings column edge     | Not computed independently                                       |
| Dual-currency cells on min-role   | Present in DOM; value correctness not independently recomputed   |
| Session expiry / back navigation  | Not exercised                                                    |
| Empty-state search / no cities    | Not exercised (scoped dataset with results always returned)      |

### Specs scenarios: coverage buckets

| Bucket              | Scenarios                                                                                                                                                                                                                                                                                                                                                         |
| ------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Covered + passing   | Annual gross 12× monthly; Net < gross for taxed cities; Rural lowers housing; School type toggle hidden/shown; Private school > public; Preschool → childcare no school; No Israeli cities; Data snapshot date; OOP legend; ID locale translated; Tools index link; SE roles caption; Mobile savings cards; Empty salary deficit + em dash; URL updates on filter |
| Covered + diverging | Zero savings target: spec says all qualify + lowest marked; app shows no marker (EWT-001). Page title: spec says includes tool name; app includes it twice (EWT-002).                                                                                                                                                                                             |
| Not exercised       | USS-001/USS-002 (empty-state, known stub); healthcare scheme badge isolation; income-band boundary (exactly at threshold); city-link click navigation; country-link click navigation; mobile city-card country-name display; no-role-reaches-bar message; household + area joint ranking update                                                                   |

## Risk Summary

**Overall impression**: The calculator is functionally solid across the main flows — tax calculations
apply correctly (except UAE which is correctly zero), area and school controls work, household
scaling matches the OECD formula, translations are complete, and all three tab UIs render properly
at all breakpoints. Two real defects were found.

**Top risks**:

1. **Zero savings target shows no minimum role marker** (EWT-001, Major) — A user setting a
   zero target sees no guidance because the app treats 0 as "no baseline set", contradicting
   the Gherkin scenario that says the lowest role should be marked when every role clears the bar.
2. **Page title duplicates the site name** (EWT-002, Minor) — "AyoKoding" appears twice in
   the `<title>` tag because the page exports a title already containing the suffix which Next.js
   then wraps in the root template. Affects SEO and browser tab readability.

## Document Map

- `findings.md` — defect catalog with steps to reproduce
- `brd.md` — business framing
- `prd.md` — corrected-behaviour user stories and Gherkin acceptance criteria
- `spec-gaps.md` — behaviours the live app exhibits that existing Gherkin does not yet describe
- `evidence/` — committed screenshots and test logs

`tech-docs.md` and `delivery.md` are not authored here. When this plan is promoted to
`plans/in-progress/`, `plan-maker` grills the maintainer and adds those documents with a
TDD-shaped delivery checklist and Specs/Gherkin completeness steps.

---

## Closure Note — URL / Deep-Link Finding

> **Closed by** `plans/in-progress/ayokoding-calculator-url-state` (2026-06-21).
>
> The Coverage Map entry "Deep-link restoration: Partial — `?country=id` restores;
> `?country=Indonesia` (full name) does not" is resolved. The new URL-state implementation:
>
> - All 9 controls (tab, region, country, city, adults, preschool, schoolkids, schooltype, area)
>   now serialize to and restore from the URL query string.
> - Deep links using city ID (e.g. `?city=singapore`) automatically backfill region and country.
> - Unknown or invalid params are sanitized to defaults on load (canonicalize-on-mount).
> - Using full-name values like `?country=Indonesia` intentionally remains unsupported — the URL
>   contract uses lowercase IDs (`?country=sg`), which is consistent with how the dataset models
>   countries.
>
> Verified via Playwright MCP on 2026-06-21.
