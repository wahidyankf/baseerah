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

---

## SG-005 — Housing column value should reflect the area discount (rural vs city center)

**Observed behaviour**

On the Cost-of-Living tab, selecting "Rural" correctly reduces the Essentials and Total columns (the
`essentialsLocal()` function applies `AREA_MULTIPLIERS["rural"] = 0.75` to housing). However the Housing
column still shows the base/center rate. A correct expected behaviour is that the Housing cell should
agree with the value used in the Essentials sum.

This is recorded as a spec gap rather than folded into EWT-008 because the _intended_ contract — whether
the Housing column should show the discounted or base amount — is not stated explicitly in the spec. The
current code's split (column shows base, total uses discounted) is internally inconsistent and could be
either a bug or an intentional "show the base reference price" UX choice. The maintainer should decide and
the spec should encode the decision.

**Why spec-worthy**

No existing scenario describes what the Housing column should display when Rural is selected. The current
scenarios test that "rural area lowers housing versus city center" (total) but do not assert the Housing
column value. Adding a scenario pins the correct contract.

**Proposed Gherkin**

```gherkin
  Scenario: Housing column reflects the area-adjusted rate
    Given I am on the "Cost of living" tab
    And the area is set to "City center"
    When I read the Housing column for Jakarta
    Then the Housing value is the city-center base rate for Jakarta
    When I switch the area to "Rural"
    Then the Housing value decreases to the rural-discounted rate (base × 0.75)
    And the Essentials value equals Housing (rural) plus the other modeled expenses
```

**Target spec file**: extend `cost-of-living-calculator.feature` under the "Cost basis controls" section, as a follow-on to the existing `Scenario: Rural area lowers housing versus city center`.

---

## SG-006 — Confidence flag coverage extends to Cost-of-Living and Savings tab cells

**Observed behaviour**

The Min-Role tab correctly shows a `[proxy]` confidence flag next to best-city cells backed by
proxy-confidence data (observed when the Indonesia filter is applied, making Jakarta — which has a
`"proxy"` school entry — the best city). The Cost-of-Living and Savings tabs render cells backed by
`"moderate"`-confidence data (healthcare, childcare, lifestyle, relocation for many cities) without any
flag.

The existing spec scenario `Scenario: Low-confidence cells are flagged` is broad ("Given I am on the
calculator") but has no step that names a specific tab. Adding a scenario per tab pins the cross-tab
coverage contract.

**Why spec-worthy**

No scenario asserts that confidence flags appear on the Cost-of-Living or Savings tabs. Without this,
adding flags to those tabs would require no spec change and removing them later would be invisible to the
test suite.

**Proposed Gherkin**

```gherkin
  Scenario: Confidence flags appear in Cost-of-Living table for lower-confidence cells
    Given I am on the "Cost of living" tab
    When the page finishes loading
    Then any cell whose underlying data has "moderate" or "proxy" confidence shows a confidence flag
    And cells with "high" confidence show no flag

  Scenario: Confidence flags appear in Savings table for lower-confidence cities
    Given I am on the "Savings" tab
    And I enter a gross monthly salary
    When the savings table renders
    Then any Essentials cell for a city whose healthcare or childcare data has moderate confidence shows a confidence flag
```

**Target spec file**: extend `cost-of-living-calculator.feature` — add after the existing `Scenario: Low-confidence cells are flagged`.
