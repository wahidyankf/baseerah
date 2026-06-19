# Spec Gaps — Cost-of-Living Calculator

**Session date**: 2026-06-19
**Source specs file**: `specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/cost-of-living-calculator.feature`

These are correct behaviours observed on the live target that existing `specs/**` Gherkin does not
yet describe. Each is a proposal for maintainer confirmation — not a verdict that an existing spec
is wrong. On promotion to `plans/in-progress/`, these seed `specs-maker` and the Specs/Gherkin
Completeness coverage steps.

---

## SG-001 — Deficit row displayed in red when essentials exceed net income

**Observed behaviour**

On the Savings tab, when a city's essential expenses exceed the net take-home (i.e.
`essentialSavings < 0`), the savings cell is rendered with a red destructive colour class
(`text-destructive`). This visual distinction is reproducible and intentional (the code
explicitly branches on `essentialSavings < 0`).

**Why spec-worthy**

No existing scenario in `cost-of-living-calculator.feature` describes the visual treatment of
negative savings cells. The scenario "Essentials above net show deficit" covers the existence of
negative savings but not the red colour signal. Protecting this behaviour prevents a regression
where the red formatting is accidentally removed.

**Proposed Gherkin**

```gherkin
  Scenario: Negative savings rows are visually flagged
    Given I am on the Savings tab
    And I enter a gross monthly salary of 500
    When the savings table renders
    Then any row where essential savings is negative displays the savings value in a red/destructive colour
    And any row where essential savings is positive does not use a red colour
```

**Target spec file**: extend existing `cost-of-living-calculator.feature` under a "Savings tab" section.

---

## SG-002 — Annual gross is derived from monthly gross as monthly × 12

**Observed behaviour**

The savings tab shows "Annual gross: X USD" below the gross monthly salary input. Entering
`8000` shows `96,000 USD` (= 8000 × 12). This is a defined calculation exposed to the user.

**Why spec-worthy**

The existing scenario "Gross monthly shows derived annual" is in the spec but the exact formula
(× 12, no other adjustment) is not expressed. Protecting the multiplier prevents silent change to
the derivation formula.

**Proposed Gherkin**

```gherkin
  Scenario: Annual gross derives from monthly gross via x12 multiplier
    Given I am on the Savings tab
    When I enter a gross monthly salary of 8000
    Then the annual gross display shows "96,000 USD"
```

**Target spec file**: update existing "Gross monthly shows derived annual" scenario in `cost-of-living-calculator.feature` to assert the exact value.

---

## SG-003 — Savings tab sort is descending-first (highest savers at top by default)

**Observed behaviour**

When the savings table first renders with a salary entered, the default sort order places the
highest `essentialSavings` value at the top (descending). Clicking the sort button reverses to
ascending. The initial state `sortAsc = false` in `savings.tsx` confirms this is intentional.

**Why spec-worthy**

No existing scenario describes the default sort direction. A developer refactoring the table
default state could accidentally flip the default to ascending without a failing test.

**Proposed Gherkin**

```gherkin
  Scenario: Savings table defaults to descending sort by savings
    Given I am on the Savings tab
    And I enter a gross monthly salary of 8000
    When the savings table renders
    Then the first row has a higher essential savings value than the last row
    And the sort button is in ascending-toggle state (clicking it will sort ascending)
```

**Target spec file**: extend `cost-of-living-calculator.feature` under "Savings tab" section.

---

## SG-004 — City detail view shows back-to-all-cities link

**Observed behaviour**

When a city detail is opened (e.g. `?tab=cost&city=jakarta`), a "Back to all cities" link
appears in the city detail card header (rendered by `CityDetail` component,
`city-detail.tsx` line 89). The link navigates to `?tab=cost` (removing city param). This is
the recovery path from the city detail view.

**Why spec-worthy**

No existing scenario describes the back-navigation link in the city detail view. If the link
is removed or its href changes, no test currently catches it. The spec scenario "Clicking a
city name opens its single-city cost-of-living detail" does not include the back-link step.

**Proposed Gherkin**

```gherkin
  Scenario: City detail view has a back-to-all-cities link
    Given I open the calculator with URL param "?tab=cost&city=jakarta"
    When the Jakarta city detail is displayed
    Then a "Back to all cities" link is visible in the city detail card
    And clicking that link navigates to the cost-of-living tab without a city filter
```

**Target spec file**: extend existing "Clicking a city name opens its single-city cost-of-living detail" scenario or add a new scenario in `cost-of-living-calculator.feature`.
