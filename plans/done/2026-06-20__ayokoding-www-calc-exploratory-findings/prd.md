# Product Requirements — Cost-of-Living Calculator Defect Fix

**Source**: Exploratory testing session 2026-06-19
**Plan**: `plans/backlog/2026-06-19__ayokoding-www-calc-exploratory-findings/`

---

## Personas

**Reza** — Indonesian software engineer considering relocation from Jakarta to Singapore or Dubai.
Uses ayokoding.com on desktop via the `/id/` locale. Relies on city/country names being in
Indonesian for orientation. Shares calculator links with colleagues.

**Amara** — Nigerian developer researching remote-work salary adequacy across ASEAN cities.
Navigates from a colleague-shared link like `?tab=cost&country=sg`. Expects the filter UI to
reflect that she is viewing Singapore data.

**Screen reader user** — any visitor to `/id/` using NVDA, JAWS, or VoiceOver. Depends on
`html lang="id"` to get correct phonetic rendering of Indonesian text.

---

## In-Scope

- Fix geo filter dropdown sync from URL params (EWT-001, EWT-002).
- Fix `html lang` attribute for Indonesian locale (EWT-003).
- Fix desktop table city/country name locale binding in `cost-of-living.tsx` and `savings.tsx`
  (EWT-004).
- Fix min-role tab city/country name locale binding in `min-role.tsx` (EWT-005).
- Fix sort button touch target in savings tab (EWT-006).
- Add `min="0"` to salary/savings-target number inputs (EWT-007).
- Update or add Gherkin scenarios in
  `specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/cost-of-living-calculator.feature`
  for each fix.

## Out-of-Scope

- Changes to the calculator's data model, city list, or FX rates.
- New features or tabs.
- Cross-browser testing (Firefox/Safari/Edge) — separate effort.
- Performance or Core Web Vitals work.

---

## User Stories

1. As a user navigating to a deep-linked country URL (`?country=id`), I see the Country filter
   dropdown pre-selected to "Indonesia" so I know the table is filtered and can adjust the scope
   from that state.

2. As a user navigating to a deep-linked city URL (`?city=jakarta`), I see the City filter
   dropdown pre-selected to "Jakarta".

3. As an Indonesian-locale user, I see `<html lang="id">` so my screen reader pronounces page
   content in Indonesian and my browser translation service identifies the page language correctly.

4. As an Indonesian-locale user on desktop, I see city and country names in Indonesian (e.g.
   "Singapura", "Jepang") in the cost-of-living table and savings table — the same names I see
   on mobile.

5. As an Indonesian-locale user on the min-role tab, I see city and country names in Indonesian
   in the "Best city" column and in the reference-city dropdown.

6. As a touch-device user, I can tap the savings sort button reliably (target height >= 24 px).

7. As a user, typing a negative number in the salary field triggers a validation message or the
   field prevents negative input, so I cannot accidentally produce a nonsensical negative output.

---

## Gherkin Acceptance Criteria

### AC-001: Geo filter pre-selected from country URL param

```gherkin
Feature: Cost-of-Living Calculator — Geo Filter Deep Link

  Scenario: Country URL param pre-selects geo filter dropdowns
    Given I open the calculator with URL param "?tab=cost&country=id"
    When the page finishes loading
    Then the Country dropdown shows "Indonesia"
    And the Region dropdown shows "ASEAN"
    And the City dropdown shows "All cities"
    And the cost-of-living table is filtered to Indonesian cities only
```

### AC-002: Geo filter pre-selected from city URL param

```gherkin
  Scenario: City URL param pre-selects geo filter dropdowns
    Given I open the calculator with URL param "?tab=cost&city=jakarta"
    When the page finishes loading
    Then the City dropdown shows "Jakarta"
    And the cost-of-living table shows only the Jakarta city detail
```

### AC-003: html lang matches active locale

```gherkin
Feature: Cost-of-Living Calculator — Locale Accessibility

  Scenario: Indonesian locale page has correct html lang attribute
    Given I navigate to "/id/tools/cost-of-living-calculator"
    Then the page's html element has attribute lang="id"

  Scenario: English locale page has correct html lang attribute
    Given I navigate to "/en/tools/cost-of-living-calculator"
    Then the page's html element has attribute lang="en"
```

### AC-004: Desktop table uses locale city/country names

```gherkin
Feature: Cost-of-Living Calculator — Locale Name Consistency

  Scenario: Desktop table city names match locale on id locale
    Given I am on the Indonesian locale cost-of-living calculator at a desktop viewport
    When the cost-of-living table is visible
    Then the Country column shows "Singapura" for the Singapore row
    And the City column shows "Singapura" for the Singapore city row

  Scenario: Desktop table city names match mobile card names on id locale
    Given I am on the Indonesian locale cost-of-living calculator
    Then the desktop table city names match the mobile card city names for every row
```

### AC-005: Min-role tab uses locale city/country names

```gherkin
  Scenario: Min-role tab best-city column uses locale names on id locale
    Given I am on the Indonesian locale calculator at the "Jabatan minimum" tab
    And I enter a savings target of 100
    Then the "Kota terbaik" column shows city names in Indonesian
    And the country names in that column are in Indonesian
```

### AC-006: Sort button meets WCAG 2.5.8 touch target minimum

```gherkin
Feature: Cost-of-Living Calculator — Accessibility Touch Target

  Scenario: Savings tab sort button meets minimum touch target height
    Given I am on the Savings tab with a gross salary entered
    When I inspect the "Savings after essentials" column header button
    Then its rendered height is at least 24 CSS pixels
```

### AC-007: Salary input rejects negative values

```gherkin
Feature: Cost-of-Living Calculator — Input Validation

  Scenario: Negative salary is not accepted
    Given I am on the Savings tab
    When I type "-1000" in the gross monthly salary field
    Then the field value is constrained to zero or empty
    And the annual gross display does not show a negative value
```
