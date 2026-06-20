# Product Requirements — Cost-of-Living Calculator Fix

## Product Overview

This plan restores presentation fidelity to the ayokoding-www cost-of-living calculator. The tool
is functionally correct but presentation-degraded: the calculation engine is sound, yet the rendered
surface drops a headline feature (dual currency), contradicts its own name, degrades the Indonesian
locale and the empty/mobile states, and has an incomplete mobile nav drawer. The product scope is
a **fidelity-restoration fix pass** — no new calculator features and no changes to the engine or
data model. The two net-new UI elements (empty-state prompts) fill documented gaps, not new product
ideas.

## Personas

- **Wahid** — Indonesian, mobile-first (375 px), uses `/id/…`; expects every label and name in
  Indonesian.
- **Alex** — English, desktop (1280 px+); wants fast cross-city comparison with unambiguous
  currency.
- **Sam** — first-time visitor from search, no prior context; needs each tab's purpose and each
  empty state to be self-explanatory within ~20 seconds.

## Product Scope

### In scope (all 29 findings from the three-tester pass)

- Dual-currency display in all money cells across all three tabs and breakpoints (DWT-001, UWT-009)
- Indonesian locale names in all desktop tables (EWT-002/003, DWT-008/009)
- Tool identity consistency — H1, `<title>`, active tab name (DWT-004, UWT-001, UWT-013)
- Mobile cost card country name (EWT-001, DWT-002)
- Empty states for Savings and Minimum-role tabs (UWT-003, UWT-007) — net-new UI
- Tab label cleanup — remove fused descriptions (UWT-002, DWT-005)
- Design-system primitives for all controls (DWT-003/006, UWT-006/008, DWT-007)
- Tools index localized text (UWT-004)
- Area-toggle visible feedback (UWT-005)
- 320 px household control layout (UWT-010, DWT-011)
- `id` Area label length (DWT-010)
- Locale URL casing redirect `/EN/…` → `/en/…` (UWT-012)
- City-detail visible section headings (EWT-004)
- HSTS header verify-only (EWT-005)
- Mobile nav drawer — populate with site nav, localize label (UWT-011)

### Out of scope

- Changes to the calculator engine or data model (engine is correct; tests lock it)
- New calculator tabs, features, or data sources
- Deployment configuration beyond HSTS header check
- Localization of languages other than `en` and `id`
- Redesign of any surface beyond restoring fidelity to the already-committed hi-fi mockups

## User Stories

1. As any visitor, I want the page heading, browser title, and active tab name to agree on one tool
   name, so that I never doubt I'm on the right page. _(DWT-004, UWT-001, UWT-013)_
2. As a visitor comparing cities, I want every monetary figure to show both the local currency and
   USD, so that I can compare without remembering which currency each row uses. _(DWT-001, UWT-009)_
3. As an Indonesian-locale visitor, I want city and country names to appear in Indonesian wherever a
   translation exists, on mobile and desktop, on every tab, so that the localization promise is
   fulfilled on the most-used surfaces. _(EWT-002/003, DWT-008/009)_
4. As a mobile visitor, I want each cost card header to show the city and its country, so that I can
   identify the city's location without switching tabs. _(EWT-001, DWT-002)_
5. As a first-time visitor on the Savings or Minimum-role tab, I want to see an instructional empty
   state — never a wall of red negatives — until I enter a salary or target, so that I understand
   what input is needed rather than concluding the tool is broken. _(UWT-003/007)_
6. As any visitor, I want tab labels to read as clean single phrases ("Savings"), with descriptions
   not fused into the label, so that tabs are scannable at a glance. _(UWT-002, DWT-005)_
7. As any visitor, I want text inputs and the baseline selector to look like real, styled controls
   from the design system, so that the page reads as a finished product. _(DWT-003/006, UWT-006/008)_
8. As any visitor on `/tools`, I want to see readable localized text, not raw i18n keys, so that the
   section index looks like a working page. _(UWT-004)_
9. As a visitor toggling Area (City center / Rural), I want a visible signal that the table
   recalculated, so that I know my change took effect. _(UWT-005)_
10. As a mobile visitor at 320 px, I want every household control to keep its label attached to its
    input, so that controls remain usable on the smallest supported viewport. _(UWT-010, DWT-011)_
11. As a visitor with an uppercase locale URL (`/EN/…`), I want to be redirected to the canonical
    lowercase form, so that bookmarks and external links do not produce 404 errors. _(UWT-012)_

## Acceptance Criteria (Gherkin)

These are the binding behaviours the fix must satisfy. They become the first failing tests in
`delivery.md` and the spec additions in `specs/**` (see [spec-gaps.md](./spec-gaps.md) for the full
proposal set).

```gherkin
Feature: Cost-of-Living Calculator presentation fidelity

Scenario: Money cells show dual currency in the cost-of-living table
  Given the user is on the "Cost of living" tab at desktop width
  When the table renders with at least one city row
  Then every monetary cell shows the city's local currency amount and the USD equivalent
  And no money cell shows a bare integer without a currency label

Scenario: id-locale tables use Indonesian city and country names
  Given the user is on "/id/tools/cost-of-living-calculator" at desktop width
  When the cost-of-living, savings, or minimum-role table renders
  Then the Country and City columns show Indonesian names where translations exist
  And names lacking an Indonesian translation fall back to English

Scenario: Mobile cost card header shows city and country
  Given the user views the "Cost of living" tab at 375px
  When the mobile cards render
  Then each card header shows both the city name and its country name in the current locale

Scenario: Page heading matches the tool identity in each locale
  Given the user opens "/en/tools/cost-of-living-calculator"
  When the page renders
  Then the H1 reads "Cost of Living Calculator"
  And the browser title starts with "Cost of Living Calculator"
  And the active tab reads "Cost of living"

Scenario: Savings tab shows an empty state before any salary is entered
  Given the user clicks the "Savings" tab with the gross-salary field empty
  When the tab renders
  Then an instructional message is shown
  And no negative savings figures are displayed

Scenario: Minimum-role tab shows an empty state before any target is entered
  Given the user clicks the "Minimum role" tab with the savings-target field empty
  When the tab renders
  Then an instructional message is shown
  And no role salary figures are displayed

Scenario: Gross-salary input uses the design-system Input primitive
  Given the user is on the "Savings" tab
  When the tab renders
  Then the gross-salary field renders with a visible border, design-token radius, and padding
  And it is paired with a Label primitive

Scenario: Baseline selector is a segmented control
  Given the user is on the "Minimum role" tab
  When the tab renders
  Then the baseline-source control renders as a styled segmented button group, not a plain select

Scenario: Tab labels are clean single phrases
  Given the user views the tab bar at any breakpoint
  When the tab bar renders
  Then each tab trigger's visible text is its label only, with the description not fused into it

Scenario: Tools index renders localized text
  Given the user navigates to "/en/tools" or "/id/tools"
  When the tools index page renders
  Then all headings and links display localized text
  And no raw i18n key strings are visible

Scenario: Uppercase locale URL redirects to canonical lowercase
  Given the user requests "/EN/tools/cost-of-living-calculator"
  When the middleware processes the request
  Then the server redirects to "/en/tools/cost-of-living-calculator"

Scenario: Mobile nav drawer shows localized site navigation
  Given the user opens the mobile nav drawer at 375px on the "/id/" locale
  When the drawer renders
  Then it shows the site's top-level navigation links
  And every drawer label is localized (no English "English Content" string on the id locale)
```

## Non-Functional Requirements

- **Accessibility**: section labels visible or properly associated (EWT-004); sort control has an
  `aria-label` (UWT-008); WCAG AA contrast preserved on all restyled controls.
- **Responsive**: mobile-first; verified at 320/375/768/1024/1280/1440 px; no mid-pair wrap at
  320 px.
- **Security**: confirm `Strict-Transport-Security` is set in the Vercel/prod config (EWT-005) —
  verify-only, not a localhost change.
- **Localization**: both `en` and `id` complete; the browser `<title>` localized (UWT-013).

## Product Risks

| Risk                                                                                         | Likelihood | Impact                               | Mitigation                                                                   |
| -------------------------------------------------------------------------------------------- | ---------- | ------------------------------------ | ---------------------------------------------------------------------------- |
| Dual-currency cell width overflow at 768 px / tablet breakpoint                              | Medium     | High (visual regression)             | Verify table at 768 px against mockup; add width constraint if needed        |
| Empty-state prompt strings not translated to id before code lands                            | Medium     | High (raw string on id locale)       | Both en + id strings required in `translations.ts` before 7.2 GREEN          |
| `localeName` helper promoted or moved breaks existing `geo-filters` usage                    | Low        | Medium (locale regression on mobile) | Run full test suite after Phase 3 GREEN; mobile cards already use the helper |
| Middleware lowercase-redirect affects paths other than locale segment                        | Low        | High (404s on legitimate paths)      | Unit-test boundary cases in `middleware.test.ts` before GREEN                |
| Rule-15 retest (Phase 11) discovers net-new Critical/Major regressions from fix interactions | Low        | High (delays archival)               | Phase gates enforce test suite green after each cluster before proceeding    |
| Mobile nav drawer fix breaks desktop navigation (shared component)                           | Low        | High (navigation regression)         | Sidebar test covers both drawer and desktop modes                            |
