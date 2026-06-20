# Spec Gaps & Suggestions — Cost-of-Living Calculator

Proposed Gherkin coverage surfaced during testing. Two clearly distinct sources are kept separate:

- **`SG-###` — Exploratory spec-gaps** (spec-aware): correct-but-unspecced live behaviour the
  `web-exploratory-tester` verified against `specs/**`. Safe to fold into the feature file once the
  Phase 3 grill accepts them.
- **`USS-###` — Usability spec-suggestions** (spec-blind): behaviour a first-time user _expects_ but
  the page lacks, proposed by `web-usability-tester` **without reading any spec**. These are
  candidates for spec-aware reconciliation, not confirmed gaps — appended by Phase 2.

Target feature file for all proposals:
`specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/cost-of-living-calculator.feature`.

---

## Exploratory spec-gaps (SG-###)

### SG-001 — Zero/empty salary deficit with suppressed percentage

```gherkin
Scenario: Zero or empty salary shows deficit with suppressed percentage
  Given I am on the "Savings" tab
  When the gross monthly salary field is empty or zero
  Then each city row shows a negative essential-savings amount equal to the negation of that city's essential expenses in USD
  And each percentage cell shows "—" because there is no net income to compute a percentage from
```

### SG-002 — Rural area × multi-adult household multiply housing sub-linearly

```gherkin
Scenario: Rural area and multi-adult household multiply the housing estimate sub-linearly
  Given I am on the "Cost of living" tab
  And I set the household to 2 adults with no children
  When I switch the area from "city center" to "rural"
  Then the housing estimate in the expense preview decreases to base × subLinear(2 adults) × 0.75
  And the essentials total in the preview decreases accordingly
```

### SG-003 — Selecting a city from the City filter opens its detail view

```gherkin
Scenario: Selecting a city from the City filter opens its detail view
  Given I am on the "Cost of living" tab
  When I select a city from the City dropdown filter
  Then the single-city cost-of-living detail for that city is shown
  And the detail is identical to the one shown when clicking the city name in the table
```

### SG-004 — Income-band boundary handling (strict less-than)

```gherkin
Scenario: Income exactly at the low-to-mid threshold falls into the mid band
  Given I am on the "Savings" tab
  And Singapore's low-to-mid band threshold is 3500 USD
  When I enter a gross monthly salary of exactly "3500" USD
  Then Singapore's net take-home uses the mid band effective tax rate (not the low rate)
  And the net shown is 3500 × (1 − mid_rate)

Scenario: Income one dollar below the low-to-mid threshold uses the low band
  Given I am on the "Savings" tab
  And Singapore's low-to-mid band threshold is 3500 USD
  When I enter a gross monthly salary of "3499" USD
  Then Singapore's net take-home uses the low band effective tax rate
  And the net shown is 3499 × (1 − low_rate)
```

### SG-005 — Mobile city cards show the country name alongside the city

```gherkin
Scenario: Mobile city cards show the country name alongside the city
  Given I am viewing the "Cost of living" tab on a viewport narrower than 768 px
  When the mobile city cards render
  Then each card header shows both the city name and its country name
```

### SG-006 — Zero savings target marks the lowest role as the minimum

```gherkin
Scenario: Zero savings target marks the lowest role as the minimum
  Given I am on the "Minimum role" tab
  And I set the baseline source to "savings target"
  When I enter a monthly savings target of "0" USD
  Then the qualifying divider is shown
  And the minimum marker appears on the lowest-ranked role in the ladder (SWE I)
  And all roles appear above the divider because every role clears a zero target
```

### SG-007 — Expense preview updates in real time; zero childcare/school when no kids

```gherkin
Scenario: Expense preview updates in real time when household controls change
  Given I am on "/en/tools/cost-of-living-calculator"
  And the default household is 1 adult, no children, city center
  When I change the Adults control to 2
  Then the Housing preview amount increases to base × subLinear(2 adults)
  And the Childcare and School preview amounts remain 0
  And the Total preview updates immediately without a page reload

Scenario: Expense preview is zero for childcare when preschool kids are zero
  Given I am on the cost-of-living tab with the default single-adult household
  When the preschool children count is 0
  Then the Childcare preview badge shows the city currency and 0
  And the School preview badge shows the city currency and 0
```

---

## Usability spec-suggestions (USS-###)

Spec-blind suggestions from `web-usability-tester` — behaviour a first-time user _expects_ but the
page lacks. **Not** deduplicated against any spec (the tester never read `specs/**`). Each carries a
spec-blind caveat: a spec-aware reviewer must confirm the behaviour is not already covered before
adding it. The Phase 3 grill decides which to accept.

### USS-001 — Empty/unbuilt tab is disabled and labelled "Coming soon" (pairs UWT-001)

```gherkin
Scenario: Empty tab is visually disabled and labelled
  Given the calculator is open and the "Savings" tab panel has no content
  When a first-time user views the tab bar
  Then the "Savings" tab is rendered with a disabled visual state (greyed, not highlighted)
  And the "Savings" tab carries a tooltip or visible badge reading "Coming soon"
  And clicking the "Savings" tab does not change the active panel
  And the "Savings" tab cannot receive keyboard focus via Tab key
```

> ⚠ Conditioned on resolving the UWT-001 conflict (the exploratory pass used these tabs successfully).
> If the tabs are in fact functional, this suggestion is void — do not disable working tabs.

### USS-002 — Filter state persisted in URL query params (pairs UWT-003; overlaps EWT-003)

```gherkin
Scenario: Selecting filters updates the URL with query parameters
  Given a user is on the cost-of-living calculator page
  When the user selects Region "ASEAN", Country "Indonesia", City "Jakarta", Adults "2", Preschool children "1", and Area "City center"
  Then the URL updates to include query parameters reflecting all selections
  And copying the URL and opening it in a new tab restores the same filter state
  And the city summary card shows Jakarta data on load

Scenario: Sharing a filtered URL restores the configured view
  Given a URL with filter parameters for Jakarta, 2 adults, 1 preschool child
  When a second user opens that URL
  Then the page loads with all filters pre-selected matching those parameters
  And the city summary card displays Jakarta data without any manual input
```

### USS-003 — Definition tooltips on "Relocation (sunk)" and "Liquidity reserve" headers (pairs UWT-005)

```gherkin
Scenario: "Relocation (sunk)" column header displays a definition tooltip
  Given the user is viewing the comparison table
  When the user hovers over or focuses the "Relocation (sunk)" column header
  Then a tooltip appears explaining what costs are included
  And the tooltip clarifies that this is a one-time (not monthly) figure

Scenario: "Liquidity reserve" column header displays a definition tooltip
  Given the user is viewing the comparison table
  When the user hovers over or focuses the "Liquidity reserve" column header
  Then a tooltip appears explaining what the liquidity reserve covers
  And the tooltip clarifies whether this is a recommended buffer or an observed figure
```

### USS-004 — Indonesian locale declares `html lang="id"` (pairs UWT-006; overlaps EWT-001)

```gherkin
Scenario: Indonesian locale page declares correct language
  Given a user navigates to the Indonesian locale calculator URL
  When the page HTML is rendered
  Then the html element carries lang="id"

Scenario: English locale page declares correct language
  Given a user navigates to the English locale calculator URL
  When the page HTML is rendered
  Then the html element carries lang="en"
```

### USS-005 — Descriptive page `<title>` reflecting tool name and active city (pairs UWT-002, UWT-007)

```gherkin
Scenario: Page title includes tool name on load
  Given a user navigates to the cost-of-living calculator
  When the page finishes loading with default filter state
  Then the browser tab title reads "Cost of Living Calculator — AyoKoding"

Scenario: Page title updates when a specific city is selected
  Given a user selects City "Singapore" on the cost-of-living calculator
  When the filter is applied
  Then the browser tab title reads "Singapore — Cost of Living Calculator — AyoKoding"
```

### USS-006 — Summary columns visible without horizontal scroll, or a scroll affordance (pairs UWT-004)

```gherkin
Scenario: Summary columns visible without horizontal scrolling at desktop width
  Given a user views the comparison table at 1280px viewport width
  When no horizontal scrolling has occurred
  Then the "Total" column is visible within the initial viewport
  And the "Essentials" column is visible within the initial viewport

Scenario: Right-edge scroll affordance when table overflows viewport
  Given the comparison table extends beyond the viewport width
  When the right edge of the table container is reached visually
  Then a visual indicator (shadow, arrow, or "scroll for more" label) signals additional content to the right
```
