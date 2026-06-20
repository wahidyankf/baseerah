# Spec Gaps & Suggestions — Cost-of-Living Calculator

Proposed `specs/**` additions surfaced during the three-tester pass. Each is a candidate for maintainer
confirmation; the Phase 4 grill decides which are accepted into
`specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/cost-of-living-calculator.feature`.

Sources stay distinct:

- `SG-###` — exploratory **spec-gaps** (spec-aware; correct-but-unspecced behaviour, edge cases).
- `SG-###` (Design) — design-tester design-spec proposals (on-design behaviours worth protecting).
- `USS-###` — usability **spec-suggestions** (spec-blind; behaviour a first-timer expects but the page lacks).

## Exploratory spec-gaps (SG-###)

### SG-001 — Negative salary input is clamped to zero

```gherkin
Scenario: Negative salary input is clamped to zero
  Given I am on the "Savings" tab
  When I enter a gross monthly salary of "-1000"
  Then the annual gross displayed is "0 USD"
  And each city row shows the same deficit as for a zero salary entry
```

### SG-002 — Decimal salary computes annual gross correctly

```gherkin
Scenario: Decimal monthly salary produces correct annual gross
  Given I am on the "Savings" tab
  When I enter a gross monthly salary of "8000.5"
  Then the annual gross is shown as "96,006 USD"
  And the annual figure equals twelve times the monthly figure
```

### SG-003 — Very large salary does not produce NaN or Infinity

```gherkin
Scenario: Very large salary produces valid savings figures
  Given I am on the "Savings" tab
  When I enter a gross monthly salary of "99999999"
  Then no city row shows "NaN" or "Infinity" in any column
  And each city row shows a positive net take-home
```

### SG-004 — Selecting only a country updates the URL

```gherkin
Scenario: Selecting only a country updates the URL tab and country parameter
  Given a user is on the cost-of-living calculator page
  When the user selects Country "Indonesia" without selecting a city
  Then the URL updates to include "tab=cost" and "country=id"
  And opening that URL in a new tab shows only Indonesian cities in the table
  And the Country filter is pre-selected to "Indonesia"
```

### SG-005 — School type toggle appears when school-age children ≥ 1

```gherkin
Scenario: School type toggle appears when school-age children is set to one or more
  Given I am on "/en/tools/cost-of-living-calculator"
  And the household has no school-age children
  When I set the household to 1 school-age child
  Then the school type toggle is shown with "Public" and "Private" options
  And the default selection is "Public"
```

### SG-006 — Housing scales sub-linearly (1.25×) for a 2-adult household

```gherkin
Scenario: Housing preview scales sub-linearly for 2-adult household
  Given I am on the cost-of-living calculator
  And the default household is 1 adult with no children in city center
  When I change the Adults control to 2
  Then the Housing preview amount is exactly 1.25 times the 1-adult amount
  And the Utilities preview amount is exactly 1.25 times the 1-adult amount
  And the Food preview amount is exactly 1.5 times the 1-adult amount
  And the Transport preview amount is unchanged from the 1-adult amount
```

## Usability spec-suggestions (USS-###)

Spec-blind candidates (the usability tester did NOT read `specs/**`). The Phase 4 grill must confirm
each is not already covered before accepting it into the suite.

### USS-001 — Savings tab empty-state when no salary entered (pairs UWT-003)

```gherkin
Scenario: Savings tab shows empty-state guidance when no salary entered
  Given a user has opened the Cost of Living Calculator
  When they click the Savings tab
  And the gross monthly salary field contains no value or zero
  Then the savings comparison table is not shown
  And an instructional message reads "Enter your gross monthly salary above to see your savings per city" (or the locale equivalent)
  And no negative savings figures are visible

Scenario: Savings tab shows results after salary is entered
  Given a user is on the Savings tab with the empty-state message displayed
  When they enter a positive gross monthly salary value
  Then the instructional message disappears
  And the savings comparison table is shown with computed savings figures
```

### USS-002 — Minimum Role tab empty-state when no target entered (pairs UWT-007)

```gherkin
Scenario: Minimum Role tab shows empty-state when no target amount entered
  Given a user has opened the Cost of Living Calculator
  When they click the Minimum Role tab
  And the Monthly savings target field contains no value or zero
  Then the role comparison table is not shown
  And an instructional message reads "Enter a monthly savings target above to see which roles would meet it" (or the locale equivalent)
  And no role salary data is visible
```

### USS-003 — Area toggle confirms data update (pairs UWT-005)

```gherkin
Scenario: Area toggle shows selected state and confirms data update
  Given a user is on the Cost of Living tab
  And "City center" is the currently active area selection
  When the user clicks "Rural"
  Then the "Rural" button displays as the active/selected state
  And a visible signal confirms the table data has been recalculated for rural estimates
```

### USS-004 — Tab name and sub-label are visually/aria distinct (pairs UWT-002)

```gherkin
Scenario: Tab sub-labels are visually separated from tab names
  Given a user views the Cost of Living Calculator tab bar
  When any tab is in the inactive state
  Then the tab primary name and its descriptive sub-label are visually distinct
  And the two pieces of text do not run together without a visual separator
  And a screen reader announces them as separate text nodes
```

### USS-005 — Tools index renders localized text, not raw keys (pairs UWT-004)

```gherkin
Scenario: Tools index page renders all text in the active locale
  Given a user navigates to /en/tools
  When the page renders
  Then the page heading and the calculator link display readable English labels
  And no raw i18n key strings are visible (e.g. "toolsPageTitle", "toolsPageCalcLink")

Scenario: Tools index page renders in Indonesian on /id/tools
  Given a user navigates to /id/tools
  When the page renders
  Then the heading and link labels are in Indonesian
  And no raw i18n key strings are visible
```

## Design spec proposals (SG-### Design)

On-design behaviours worth protecting, from `web-design-tester`. Target file:
`specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/cost-of-living-calculator.feature`.

### SG-D-001 — Dual-currency display in cost-of-living and savings tables (pairs DWT-001)

```gherkin
Scenario: Cost-of-living table shows local currency and USD for each expense cell
  Given the user is on the Cost of living tab at desktop width
  When the table renders with at least one city row
  Then every monetary cell (Housing, Food, Transport, Utilities, Healthcare, Childcare, School, Essentials, Total, Relocation, Liquidity) shows the local currency amount
  And the same cell shows the USD equivalent
  And no money cell shows a bare integer without a currency label

Scenario: Savings table shows local currency and USD for net and savings columns
  Given the user is on the Savings tab with a gross salary entered
  When the table renders
  Then the Net, Essentials, Essential-savings, and After-lifestyle-savings columns show both local and USD amounts
```

### SG-D-002 — Mobile card header shows City, Country (pairs DWT-002 / EWT-001)

```gherkin
Scenario: Mobile cost-of-living card header includes country name
  Given the user views the Cost of living tab at 375px
  When mobile city cards render
  Then each card header shows both the city name and the country name
  And both names are visible without any user interaction
```

### SG-D-003 — Page heading matches tool identity (pairs DWT-004 / UWT-001)

```gherkin
Scenario Outline: H1 matches the tool's official name in each locale
  Given the user opens "/<locale>/tools/cost-of-living-calculator"
  When the page renders
  Then the H1 reads "<expected_h1>"
  And the browser title starts with "Cost of Living Calculator"

  Examples:
    | locale | expected_h1               |
    | en     | Cost of Living Calculator |
    | id     | Kalkulator Biaya Hidup    |
```

### SG-D-004 — id locale uses Indonesian city/country names in all table views (pairs DWT-008/009, EWT-002/003)

```gherkin
Scenario: Id locale cost-of-living table uses Indonesian translations
  Given the user is on "/id/tools/cost-of-living-calculator" at desktop width
  When the cost-of-living table renders
  Then the Country column shows Indonesian country names where translations exist
  And the City column shows Indonesian city names where translations exist

Scenario: Id locale minimum-role table uses Indonesian best-city names
  Given the user is on "/id/tools/cost-of-living-calculator" at desktop width
  And the Minimum role tab is active
  When the ladder table renders
  Then the Best city column shows Indonesian city and country names where translations exist
```
