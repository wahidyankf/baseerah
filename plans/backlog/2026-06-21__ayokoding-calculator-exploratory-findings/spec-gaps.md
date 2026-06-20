# Spec Gaps

Behaviours observed on the live target during the 2026-06-21 exploratory session that the existing
`specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/cost-of-living-calculator.feature` does
not yet describe. Each entry is a proposal for maintainer confirmation — the agent asserts "this
behaviour exists and is unprotected", not "the spec is wrong". On promotion these proposals seed
`specs-maker` and Specs/Gherkin Completeness coverage steps.

---

## SG-001 — Savings tab: empty salary shows negative savings with em-dash percentage

**Observed behaviour**: When the gross monthly salary field is empty (initial state, no entry), the
Savings tab already renders all city rows. Each row shows a negative essential-savings amount (the
negation of that city's essential expenses in USD) and an em-dash (`—`) in the percentage cell
because there is no net income to divide by.

Example observed: first city shows `INR -54,500 / $-578 (—)` in the essential-savings cell when
the salary input is empty.

**Where observed**: `/en/tools/cost-of-living-calculator?tab=savings`, viewport 1280 px, 2026-06-21.

**Why it is spec-worthy**: The spec has one existing scenario (`Scenario: Zero or empty salary shows
deficit with suppressed percentage`) that describes exactly this behaviour for when "the gross
monthly salary field is empty or zero". The live app correctly implements it. However, the scenario
does not explicitly cover the initial-load state (no user interaction yet) — only "empty or zero".
The gap is that the spec does not assert the behaviour on initial page load before the user touches
the input. This is a subtle but important protection for regressions that only affect the
uninteracted initial state.

**Proposed Gherkin**:

```gherkin
Scenario: Savings table initialises with empty salary showing deficit and suppressed percentage
  Given a user navigates to "/en/tools/cost-of-living-calculator"
  When the user clicks the "Savings" tab without entering a salary
  Then each city row shows a negative essential-savings amount equal to the negation of that city's essential expenses in USD
  And each percentage cell shows an em dash because there is no net income to compute a percentage from
  And the annual gross label shows "0 USD"
```

**Target feature file**: extend
`specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/cost-of-living-calculator.feature`
(add after the existing `Zero or empty salary shows deficit with suppressed percentage` scenario).

---

## SG-002 — Sub-national tax indicator shown alongside net for US, CA, CH cities

**Observed behaviour**: In the Savings table at 1440 px (lg breakpoint), cells for cities in
federal countries (US, Canada, Switzerland) display a small `(fed+state)` suffix next to the net
take-home value. Cities in unitary countries show no such indicator. Confirmed present for 5 rows
when a $8,000 gross salary was entered.

**Where observed**: `/en/tools/cost-of-living-calculator?tab=savings` with gross=8000, viewport 1440 px,
2026-06-21. The rendered text reads e.g. `USD 5,440 / $5,440 (fed+state)`.

**Why it is spec-worthy**: The spec has a scenario `Sub-national tax lowers net only in federal
countries` that describes the arithmetic correctly but does not describe the UI affordance (the
`(fed+state)` label). A developer could change or remove the label without breaking the existing
spec. Adding a scenario that asserts the label's presence protects this user-visible affordance.

**Proposed Gherkin**:

```gherkin
Scenario: Sub-national tax indicator appears next to net for federal-country cities
  Given I am on the "Savings" tab with a gross monthly salary entered
  When I read the Net column for a city in the United States, Canada, or Switzerland
  Then the cell shows a "(fed+state)" indicator suffix alongside the net take-home amount
  And no such indicator appears for cities in unitary countries
```

**Target feature file**: extend
`specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/cost-of-living-calculator.feature`
(add after the existing `Sub-national tax lowers net only in federal countries` scenario).

---

## SG-003 — Geo filter Country select automatically narrows City options

**Observed behaviour**: When the user selects "Indonesia" in the Country dropdown, the City
dropdown immediately narrows to only Indonesian cities ("All cities" + "Jakarta" when only Jakarta
is in the dataset). The URL simultaneously updates to `?tab=cost&country=id`. This cascade is a
distinct, independently verifiable UI behaviour.

**Where observed**: `/en/tools/cost-of-living-calculator`, Country select changed to Indonesia,
viewport 1280 px, 2026-06-21.

**Why it is spec-worthy**: The spec scenario `Region narrows the country filter and country narrows
the city filter` describes the cascade logic but is written in terms of the Region→Country→City
cascade. A scenario focused on the Country→City cascade alone (without first selecting a Region)
protects the simpler two-level path that users are more likely to follow.

**Proposed Gherkin**:

```gherkin
Scenario: Selecting a country narrows the city dropdown without requiring a region selection
  Given I am on "/en/tools/cost-of-living-calculator"
  And the "Cost of living" tab is active
  When I select the country "Indonesia" from the Country filter without selecting a region first
  Then the City filter lists only Indonesian cities
  And the URL updates to include "country=id"
  And the table shows only rows for Indonesian cities
```

**Target feature file**: extend
`specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/cost-of-living-calculator.feature`
(add after the existing `Region narrows the country filter and country narrows the city filter`
scenario).

---

## SG-004 — Area SegmentedControl is a radiogroup, not a dropdown

**Observed behaviour**: The Area control (City center / Rural) is rendered as a `role="radiogroup"`
containing `role="radio"` buttons, not as a `<select>` element. The same radiogroup pattern is used
for School type. This is an accessible segmented-control widget.

**Where observed**: `/en/tools/cost-of-living-calculator`, Cost of Living tab controls area,
viewport 1280 px and 375 px, 2026-06-21.

**Why it is spec-worthy**: The spec's `Rural area lowers housing versus city center` scenario
describes the functional effect but not the UI widget type. A future refactor could change the Area
toggle to a `<select>` without breaking the functional scenario. Adding a scenario asserting the
radiogroup role protects the accessible widget pattern.

**Proposed Gherkin**:

```gherkin
Scenario: Area control is a segmented radio control, not a dropdown
  Given I am on the cost-of-living calculator
  When I inspect the Area control
  Then the Area control is rendered as a radiogroup with radio buttons for "City center" and "Rural"
  And the currently selected option has aria-checked set to true
```

**Target feature file**: extend
`specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/cost-of-living-calculator.feature`
(new scenario under the area-toggle group).

---

## SG-005 — Baseline source SegmentedControl on Minimum role tab is a radiogroup

**Observed behaviour**: The "Baseline source" control on the Minimum role tab renders as a
`role="radiogroup"` with three `role="radio"` options: "Monthly savings target", "Reference role",
"My salary". The three modes show/hide different input sub-forms depending on which radio is active.

**Where observed**: `/en/tools/cost-of-living-calculator?tab=min-role`, viewport 1280 px, 2026-06-21.

**Why it is spec-worthy**: The spec's three baseline-source scenarios describe the computation and
output but not the widget type or the show/hide behaviour of the sub-forms. Protecting the widget
type ensures accessible interaction is maintained.

**Proposed Gherkin**:

```gherkin
Scenario: Baseline source selector shows the matching input sub-form
  Given I am on the "Minimum role" tab
  When I select "Reference role" from the baseline source control
  Then the Reference city and Reference role dropdowns are visible
  And the Monthly savings target input is hidden
  And the My salary inputs are hidden
```

**Target feature file**: extend
`specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/cost-of-living-calculator.feature`
(add after the existing baseline-source scenarios).
