# Product Requirements Document

## Overview

Two defects in the AyoKoding cost-of-living calculator must be corrected. The required behaviours
are already described in `specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/cost-of-living-calculator.feature`.
This PRD translates those existing scenarios into user stories and restates the corrected Gherkin
acceptance criteria for each fix.

## Personas

**Career explorer** — a software engineer evaluating which city-role combination lets them reach
their savings goals. They use the Minimum role tab to find the minimum seniority needed. A zero
savings target is a valid starting point: they want to see which role is the entry point above zero
savings.

**SEO visitor / first-time user** — a user arriving from a search engine result. The browser tab
title and the search snippet shape their first impression of the tool.

## User Stories

### US-001: Zero savings target shows all roles qualifying with the lowest role marked

As a career explorer,
when I set a monthly savings target of zero USD,
I want to see all roles above the qualifying divider with the lowest-ranked role marked as the
minimum,
so that I understand that any role in the ladder can achieve a non-negative savings outcome.

**In scope**: Savings target input with value 0 (or typed "0").
**Out of scope**: Negative savings targets (the input has `min="0"`).

### US-002: Calculator page title contains the tool name once, not twice

As a search-engine visitor,
when I see the browser tab or search-result title for the calculator page,
I want it to read "Cost of Living Calculator | AyoKoding" (or the Indonesian equivalent),
so that the tool name and site name each appear exactly once.

**In scope**: `/en/tools/cost-of-living-calculator` and `/id/tools/cost-of-living-calculator`.
**Out of scope**: Other pages on the site (separate audit).

## Acceptance Criteria

### US-001 Gherkin (corrects EWT-001)

Maps to existing scenario `cost-of-living-calculator.feature › Scenario: Zero savings target marks
the lowest role as the minimum`.

```gherkin
Scenario: Zero savings target marks the lowest role as the minimum
  Given I am on the "Minimum role" tab
  And I set the baseline source to "savings target"
  When I enter a monthly savings target of zero USD
  Then the qualifying divider is shown
  And the minimum marker appears on the lowest-ranked role in the ladder
  And all roles appear above the divider because every role clears a zero target
```

### US-002 Gherkin (corrects EWT-002)

Maps to existing scenario `cost-of-living-calculator.feature › Scenario: Page title includes tool
name on load`.

```gherkin
Scenario: Page title includes tool name on load
  Given a user navigates to the cost-of-living calculator
  When the page finishes loading with default filter state
  Then the browser tab title includes the name of the tool
  And the site name "AyoKoding" appears exactly once in the title
```

## In Scope

- `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/min-role.tsx` (EWT-001 guard
  logic)
- `apps/ayokoding-www/src/app/[locale]/tools/cost-of-living-calculator/page.tsx` (EWT-002
  generateMetadata)
- Companion unit and integration tests for both fixes
- Gherkin scenario verification in `specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/`

## Out of Scope

- Cosmetic or layout changes beyond the defect fix
- Any other page's title tag (separate task)
- Stub scenarios USS-001 / USS-002 (empty-state behaviours listed as known not-yet-implemented
  in the spec file — excluded by the task brief)
