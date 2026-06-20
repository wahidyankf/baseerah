# Product Requirements — Cost-of-Living Calculator Test-Fixing

## Product overview

This plan remediates the defects found by the combined exploratory + usability testing pass against
the ayokoding-www Cost-of-Living Calculator. It does not add new product surfaces; it repairs
correctness, accessibility, localisation, and comprehension defects on the existing three-tab tool
(Cost of living / Savings / Minimum role) and folds the surfaced spec coverage into the feature file.

The **primary UI-bearing change** is a reorder of the comparison-table summary columns (move Total +
Essentials left, right after City), with the breakdown columns following. The **secondary UI-bearing
change** is the city-detail screen, which gains household-adjusted rows and dual-currency relocation
figures. Both screens carry a UI-design-funnel in [`assets/`](./assets/).

## Personas

Solo-maintainer repo — these are hats the maintainer wears and agents that consume the product:

- **Alex (relocation-minded software engineer)** — the tool's archetypal first-time user from the
  cognitive walkthrough; a Jakarta developer evaluating a Singapore move. Wants a trustworthy,
  shareable cost comparison in either locale.
- **Indonesian-locale user** — reads `/id/…` routes; needs a fully translated UI and a correct
  language signal for assistive tech and machine translation.
- **Maintainer-as-frontend-engineer** — implements the fixes.
- **`web-exploratory-tester`** — re-verifies the running tool in the Rule-15 round.

## User stories

- As a relocation-minded engineer, I want the comparison table's **Total** to be visible without
  horizontal scrolling, so that I can read the answer I came for at a glance. (`UWT-004`)
- As a relocation-minded engineer, I want the visible category columns to **sum to the stated
  subtotal** under my household size, so that I trust the numbers. (`EWT-006`/`EWT-007`)
- As a relocation-minded engineer, I want relocation and liquidity figures in **both local currency
  and USD with a definition**, so that I understand the one-time costs of moving. (`EWT-002`/`UWT-005`)
- As a relocation-minded engineer, I want my **filter selections reflected in the URL**, so that I
  can bookmark and share a specific comparison. (`EWT-003`/`UWT-003`)
- As an Indonesian-locale user, I want the page to **declare `lang="id"`** and present **fully
  translated** labels, so that assistive tech and translation tooling work correctly.
  (`EWT-001`/`UWT-006`, `EWT-008`–`EWT-011`)
- As a first-time user, I want the **page name, tab labels, and jargon** to be clear and consistent,
  so that I know I am on the right tool and what each control does. (`UWT-002`/`UWT-007`/`UWT-012`)
- As a keyboard / assistive-tech user, I want **sort state exposed** and a **reachable mobile sort
  control**, so that I can sort the savings table. (`EWT-012`/`EWT-014`)
- As a careful user, I want **negative salary input rejected**, so that the tool never shows a
  nonsensical negative net. (`EWT-005`)

## Acceptance criteria (Gherkin)

Every scenario uses exactly one primary `Given`, one `When`, one `Then`; extras chain with
`And`/`But`. Scenarios are grouped by finding cluster. The target feature file is
`specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/cost-of-living-calculator.feature`.

### Cluster A — Locale / `html lang` (`EWT-001` ⇄ `UWT-006`; folds `USS-004`)

```gherkin
Scenario: Indonesian locale page declares lang="id"
  Given I navigate to "/id/tools/cost-of-living-calculator"
  When the page HTML is rendered
  Then the html element carries lang="id"

Scenario: English locale page declares lang="en"
  Given I navigate to "/en/tools/cost-of-living-calculator"
  When the page HTML is rendered
  Then the html element carries lang="en"
```

### Cluster B — Indonesian translation gaps (`EWT-008`, `EWT-009`, `EWT-010`, `EWT-011`)

```gherkin
Scenario: Filter dropdowns show Indonesian country and city names in the ID locale
  Given I am on "/id/tools/cost-of-living-calculator"
  When I open the Country and City filter dropdowns
  Then each option label uses the Indonesian name where one exists
  And it falls back to the English name only when no Indonesian name exists

Scenario: Relocation column header is fully translated in the ID locale
  Given I am on "/id/tools/cost-of-living-calculator"
  When I read the relocation sunk-cost column header
  Then the header is written entirely in Indonesian with no untranslated English word

Scenario: Skip-to-content link is translated in the ID locale
  Given I am on "/id/tools/cost-of-living-calculator"
  When the skip-to-content link is rendered
  Then its visible text is the Indonesian "skipToContent" translation

Scenario: Clear-region control aria-label is translated in the ID locale
  Given I am on "/id/tools/cost-of-living-calculator"
  When the clear-region control is rendered
  Then its aria-label uses the Indonesian "clearRegion" translation
```

### Cluster C — Household scaling display (`EWT-006` table columns ⇄ `EWT-007` city-detail rows; folds `SG-002`, `SG-007`)

```gherkin
Scenario: Comparison-table category columns sum to the essentials subtotal under a multi-adult household
  Given I am on the "Cost of living" tab with the household set to 2 adults
  When I read a city row in the comparison table
  Then each per-category column shows the household-adjusted amount
  And the sum of the per-category columns equals the essentials subtotal shown for that row

Scenario: City-detail rows show household-adjusted amounts that reconcile to the subtotal
  Given I am viewing a city detail with the household set to 2 adults
  When I read the per-category rows
  Then each row shows the household-adjusted amount using the same scaling as the essentials subtotal
  And the rows add up to the essentials subtotal shown in the detail

Scenario: Rural area and multi-adult household multiply the housing estimate sub-linearly
  Given I am on the "Cost of living" tab with the household set to 2 adults and no children
  When I switch the area from "city center" to "rural"
  Then the housing estimate decreases to base times subLinear(2 adults) times 0.75
  And the essentials total decreases accordingly

Scenario: Expense preview updates in real time when household controls change
  Given I am on "/en/tools/cost-of-living-calculator" with the default single-adult household
  When I change the Adults control to 2
  Then the Housing preview amount increases to base times subLinear(2 adults)
  And the Childcare and School preview amounts remain 0
  And the Total preview updates immediately without a page reload
```

### Cluster D — Relocation / liquidity columns: USD + definitions (`EWT-002` ⇄ `UWT-005`; folds `USS-003`)

```gherkin
Scenario: City detail shows relocation and liquidity figures in both local currency and USD
  Given I am viewing a city detail
  When I read the relocation sunk-cost and liquidity-reserve rows
  Then each figure is shown in the city's local currency
  And each figure is also shown with its USD equivalent

Scenario: Relocation and liquidity column headers carry definition tooltips
  Given I am viewing the comparison table
  When I hover or focus the "Relocation (sunk)" and "Liquidity reserve" column headers
  Then a tooltip explains what each figure includes
  And the tooltip clarifies that each is a one-time figure rather than a monthly figure
```

### Cluster E — URL ⇄ filter bidirectional sync (`EWT-003` ⇄ `UWT-003`; folds `USS-002`, `SG-003`)

```gherkin
Scenario: Filter dropdowns hydrate from URL query params on deep link
  Given I deep-link to "/en/tools/cost-of-living-calculator?tab=cost&country=id"
  When the page resolves the deep link
  Then the Region filter is pre-selected to "ASEAN" and the Country filter to "Indonesia"
  And the table is filtered to Indonesian cities

Scenario: Selecting filters writes the selection to the URL
  Given I am on "/en/tools/cost-of-living-calculator"
  When I select Region "ASEAN", Country "Indonesia", and City "Jakarta"
  Then the URL updates to include query parameters reflecting those selections
  And opening the updated URL in a new tab restores the same filter state

Scenario: Clicking a city name pre-selects the City filter
  Given I am on the "Cost of living" tab
  When I click a city name in the comparison table
  Then the single-city detail for that city is shown
  And the City filter is pre-selected to that city

Scenario: Selecting a city from the City filter opens its detail view
  Given I am on the "Cost of living" tab
  When I select a city from the City dropdown filter
  Then the single-city cost-of-living detail for that city is shown
  And the detail matches the one shown when clicking the city name in the table
```

### Cluster F — Comparison-table summary-first reorder + overflow affordance (`UWT-004`; folds `USS-006`)

```gherkin
Scenario: Summary columns appear immediately after the City column
  Given I am on the "Cost of living" tab at 1280px viewport width
  When the comparison table renders
  Then the Total and Essentials columns appear immediately after the City column
  And the per-category breakdown columns follow the summary columns

Scenario: Total column is visible without horizontal scrolling at desktop width
  Given I am viewing the comparison table at 1280px viewport width with no horizontal scrolling
  When the table renders
  Then the Total column is visible within the initial viewport
  And the Essentials column is visible within the initial viewport

Scenario: Overflowing table shows a right-edge scroll affordance
  Given the comparison table extends beyond the viewport width
  When the right edge of the table container is reached visually
  Then a visual indicator signals that additional columns exist to the right
```

### Cluster G — Negative salary input (`EWT-005`)

```gherkin
Scenario: Negative gross salary input is clamped to zero
  Given I am on the "Savings" tab
  When I type "-5000" into the gross monthly salary field
  Then the field value is clamped so the annual gross is not negative
  And no city row shows a negative gross-derived figure

Scenario: Zero or empty salary shows deficit with suppressed percentage
  Given I am on the "Savings" tab
  When the gross monthly salary field is empty or zero
  Then each city row shows a negative essential-savings amount equal to the negation of that city's essential expenses in USD
  And each percentage cell shows an em dash because there is no net income to compute a percentage from
```

### Cluster H — Savings sort accessibility and mobile reachability (`EWT-012`, `EWT-014`)

```gherkin
Scenario: Savings sort control exposes its state to assistive technology
  Given I am on the "Savings" tab
  When I read the sort control in the accessibility tree
  Then the control exposes its current sort direction via aria-pressed or aria-sort

Scenario: A visible sort control is reachable in the mobile savings layout
  Given I am on the "Savings" tab at 375px viewport width
  When the mobile card layout renders
  Then a visible, tappable sort control is present in the mobile layout
  And no hidden desktop-only sort button remains in the keyboard tab order
```

### Cluster I — Naming and metadata (`UWT-002`, `UWT-007`; folds `USS-005`)

```gherkin
Scenario: A subtitle ties the H1 to the cost-of-living purpose
  Given I am on "/en/tools/cost-of-living-calculator"
  When the page renders its heading area
  Then the H1 still reads "Salary Savings Calculator"
  And a subtitle describes it as a cost-of-living comparison tool

Scenario: Page title names the tool on load
  Given I navigate to the cost-of-living calculator with default filter state
  When the page finishes loading
  Then the browser tab title names the tool rather than only "AyoKoding"
```

### Cluster J — Comprehension polish (`UWT-009`, `UWT-010`, `UWT-011`, `UWT-012`, `UWT-014`)

```gherkin
Scenario: Tab labels carry predictive information scent
  Given I am on the calculator
  When I read the "Savings" and "Minimum role" tab labels
  Then each label or its subtitle predicts the panel content rather than using a bare ambiguous word

Scenario: Mobile interactive controls meet the 44px preferred target height
  Given I am on the calculator at a viewport narrower than 768px
  When an interactive control renders
  Then the control has a minimum height of at least 44px

Scenario: The Indonesian Area label does not reflow the city-center toggle at 375px
  Given I am on "/id/tools/cost-of-living-calculator" at 375px viewport width
  When the Area control renders
  Then the Area label fits on one line without wrapping the city-center and rural toggle onto a new row

Scenario: Healthcare scheme badges are sentence-cased and defined
  Given I am on the calculator
  When I read a healthcare-scheme badge
  Then the badge text is sentence-cased rather than all-caps
  And a header tooltip defines the healthcare-scheme taxonomy

Scenario: The OOP abbreviation is wrapped for assistive tech
  Given I am on a tab that shows the "Healthcare (OOP)" column
  When I read the OOP abbreviation
  Then it is wrapped in an abbr element whose title expands to "out-of-pocket"
```

### Cluster K — Tools index route (`UWT-013`)

```gherkin
Scenario: The parent tools URL resolves instead of returning 404
  Given I navigate to "/en/tools"
  When the page resolves
  Then an index page is shown rather than an HTTP 404
  And it links to the cost-of-living calculator
```

### Cluster L — Security headers (`EWT-013`)

```gherkin
Scenario: Responses carry baseline security headers and omit the framework banner
  Given the ayokoding-www app serves a calculator route
  When I inspect the HTTP response headers
  Then the response includes Content-Security-Policy, X-Content-Type-Options, frame-ancestors protection, and Referrer-Policy
  And the response does not include an X-Powered-By header
```

### Cluster M — Confidence-flag spec reconciliation (`EWT-015`)

> **Decision (implement vs retire)**: The implement-vs-retire choice is documented here.
> The default resolution is to **retire** the "Low-confidence cells are flagged" scenario with a
> recorded rationale, because no `data-testid="confidence-flag"` DOM element currently exists and
> implementing confidence-flag UI is out of scope for this fixing plan. If the executor discovers
> the affordance already exists during Phase 7, they should instead assert the existing scenario
> passes. The acceptance criterion below reflects the default (retire) path; the executor records
> their actual decision in `delivery.md` under the Phase 7 EWT-015 reconciliation step.

```gherkin
Scenario: Confidence-flag scenario is retired with recorded rationale
  Given the "Low-confidence cells are flagged" scenario in the feature file
  When the live DOM is inspected for a data-testid="confidence-flag" element
  Then the scenario is removed from the feature file with a rationale recorded in delivery.md
```

### Cluster N — UWT-001 re-verification (conflict-flagged; conditions `USS-001`)

```gherkin
Scenario: Savings and Minimum-role tabs swap the active panel on activation
  Given I am on the "Cost of living" tab
  When I activate the "Savings" tab and then the "Minimum role" tab
  Then the active panel content swaps to match the selected tab each time
  And the previously active panel is no longer the rendered active panel
```

> If this scenario passes on re-verification (the expected outcome), the `UWT-001` tab-rewrite and
> the conditioned `USS-001` "disable + Coming soon" suggestion are recorded **void**; remediation
> reduces to the `UWT-012` information-scent label fix in Cluster J.

## Mockup references

The two changed screens are designed through the UI-design-funnel in [`assets/`](./assets/):

- Comparison table (primary change): low-fi alternatives in
  [`ui-comparison-table-low-fi-alternatives.md`](./assets/ui-comparison-table-low-fi-alternatives.md);
  chosen Option A hi-fi at three breakpoints.
- City detail (secondary change): chosen Option A hi-fi at three breakpoints showing
  household-adjusted rows and dual-currency relocation.

See [tech-docs.md](./tech-docs.md) for the per-finding fix approach and the embedded mockup images.

## Product scope

### In scope

- All 15 exploratory (`EWT-001..015`) and 14 usability (`UWT-001..014`) findings — fixed or formally
  voided.
- Folding `SG-001..007` and the spec-reconciled subset of `USS-001..006` into the feature file.
- UI-design-funnel mockups for the comparison-table and city-detail screens.

### Out of scope

- Core math changes (verified correct).
- URL slug rename.
- New cities / roles / datasets.
- Full keyboard-only and screen-reader-order audits beyond the specific findings raised.
- A tools-hub redesign beyond the minimal `/tools` index for `UWT-013`.

## Product-level risks

- **`UWT-001` over-correction** — see the README conflict note; re-verify before any tab change.
- **`USS-###` spec duplication** — reconcile against the existing feature file before folding.
- **Locale regressions** — assert both `en` and `id` outcomes in Gherkin and tests.
- **Column-reorder relearning** — mitigated by keeping all columns and documenting the funnel
  rationale.
