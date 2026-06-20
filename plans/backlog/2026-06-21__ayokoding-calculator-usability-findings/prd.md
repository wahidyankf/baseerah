# Product Requirements — AyoKoding Calculator Usability Fixes

## Product Overview

Resolve the twelve usability findings catalogued in `findings.md` to make the cost-of-living
calculator self-evident to a first-time visitor with no documentation. Priority is highest on the
three severity-3 findings (UWT-001, UWT-002, UWT-003) that block or badly confuse users on the
Savings and Minimum Role tabs.

## Personas

### Primary — First-time relocation-considering engineer

- A software engineer or manager researching whether to relocate.
- Arrives via a search result or a shared link.
- Has general web literacy; no knowledge of this tool.
- Expects a calculator to guide them with prompts; does not read documentation.
- Uses desktop primarily; also tested on mobile.
- May have a non-USD salary (EUR, SGD, IDR).

### Secondary — Indonesian-language visitor

- Same goals as primary but reads the `/id/` locale.
- Expects the same clarity and completeness as the English version.

## User Stories

### US-001 (UWT-001) — Savings tab empty-state guidance

As a first-time visitor on the Savings tab,
when I have not yet entered my salary,
I can see a clear prompt or placeholder that tells me to enter my salary,
so that I do not interpret the all-negative savings table as a broken calculator.

### US-002 (UWT-002) — Minimum Role mode selector group label

As a first-time visitor on the Minimum Role tab,
when I see the three mode buttons ("Monthly savings target", "Reference role", "My salary"),
I can read a visible label above them that explains what the group controls (e.g. "Ranking mode:"),
so that I know which mode to choose without having to click all three to discover their effects.

### US-003 (UWT-003) — Visible tab sub-descriptions

As a first-time visitor scanning the three tabs,
I can read a brief visible sub-description under each tab label (e.g. "compare monthly expenses",
"see how much you'd keep", "find the seniority you need"),
so that I can self-route to the right tab without clicking each one to discover its purpose.

### US-004 (UWT-004) — Savings tab: salary currency selector

As a first-time visitor whose salary is in EUR or SGD,
when I open the Savings tab,
I can select my salary currency from a dropdown (consistent with the Minimum Role tab's "Target
currency" selector),
so that I do not have to mentally convert my salary to USD before entering it.

### US-005 (UWT-005) — Tab state in URL

As a first-time visitor who bookmarks or shares the Savings tab,
when the URL is copied and opened in a new tab,
the page opens on the Savings tab (not always the Cost of living tab),
so that my bookmark and shared links are reliable.

### US-006 (UWT-006) — Minimum Role tab: empty-state guidance

As a first-time visitor on the Minimum Role tab with no savings target entered,
I can see an inline note or empty-state message explaining that I should enter a target to filter
the role ranking,
so that I do not confuse the pre-filled "CTO in Austin" ranking with a recommendation that I need
to be a CTO.

### US-007 (UWT-007) — Region labels expanded

As a first-time visitor from outside Southeast Asia or the Middle East,
when I open the Region dropdown,
I can read "ASEAN (Southeast Asia)" and "MENA (Middle East & North Africa)" instead of bare
acronyms,
so that I know which group covers my destination city.

### US-008 (UWT-008) — 320 px horizontal overflow resolved

As a first-time visitor on an iPhone SE (320 px),
when I load the calculator,
I do not see a page that overflows horizontally without a scroll affordance,
so that I know all controls and data are accessible.

### US-009 (UWT-009) — Tools index: tool descriptions

As a first-time visitor on the tools index page,
I can read a one- or two-sentence description below each tool card title,
so that I can confirm the tool is relevant to my goal before clicking.

### US-010 (UWT-010) — Back link restores filter state

As a first-time visitor who drilled into a city detail from a filtered view (e.g. ASEAN region,
Singapore country),
when I click "Back to all cities",
I return to the same filtered view (ASEAN region, Singapore country)
instead of a reset "All regions / All countries" state.

### US-011 (UWT-011) — "Cost of living" tab gets aria-describedby

As a screen-reader user on the Cost of living tab,
the tab button carries `aria-describedby` pointing to a description span,
consistent with the Savings and Minimum Role tabs.

### US-012 (UWT-012) — OOP abbreviation: inline tooltip

As a first-time visitor reading "Healthcare (OOP)",
when I hover or focus the abbreviation,
I see the definition "out-of-pocket — healthcare you pay yourself" inline,
without having to scan the page for a separate footnote.

## Acceptance Criteria

### US-001 — Savings tab empty-state guidance

```gherkin
Scenario: Savings tab shows guidance when no salary is entered
  Given a user opens the Savings tab for the first time
  When no salary has been entered in the gross-salary input
  Then the gross-salary input shows a placeholder (e.g. "e.g. 5000")
  And a contextual note is visible near the input or above the table
  And the note text explains that entering a salary reveals per-city savings
```

```gherkin
Scenario: Savings tab empty state in Indonesian locale
  Given a user opens the Savings tab on the /id/ locale
  When no salary has been entered
  Then the placeholder and guidance note are displayed in Indonesian
```

### US-002 — Minimum Role mode selector group label

```gherkin
Scenario: Minimum Role tab shows a visible group label for the mode selector
  Given a user opens the Minimum Role tab
  When the three mode radio buttons are visible
  Then a visible label (e.g. "Ranking mode:" or "Find minimum role by:") appears above them
  And the label is readable by sighted users without assistive technology
```

### US-003 — Visible tab sub-descriptions

```gherkin
Scenario: All three tabs show visible sub-descriptions
  Given a user views the calculator page
  When the tab bar is visible
  Then the "Cost of living" tab shows a brief sub-description visible to sighted users
  And the "Savings" tab shows a brief sub-description visible to sighted users
  And the "Minimum role" tab shows a brief sub-description visible to sighted users
```

```gherkin
Scenario: Tab sub-descriptions shown in Indonesian locale
  Given a user views the calculator on the /id/ locale
  When the tab bar is visible
  Then all three tab sub-descriptions are displayed in Indonesian
```

### US-004 — Savings tab: salary currency selector

```gherkin
Scenario: Savings tab provides a salary currency selector
  Given a user opens the Savings tab
  When the salary input is visible
  Then a currency selector is visible adjacent to the salary input
  And the currency options include at least USD, EUR, SGD, IDR, GBP, JPY
  And the label or placeholder reflects the selected currency
```

### US-005 — Tab state in URL

```gherkin
Scenario: Navigating to the Savings tab updates the URL
  Given a user is on the Cost of living tab
  When the user clicks the Savings tab
  Then the URL updates to include a tab parameter (e.g. ?tab=savings)
```

```gherkin
Scenario: Restoring a tab URL opens the correct tab
  Given a URL with ?tab=savings is opened directly
  When the page loads
  Then the Savings tab is active on load
```

```gherkin
Scenario: Restoring a tab URL for Minimum role
  Given a URL with ?tab=min-role is opened directly
  When the page loads
  Then the Minimum role tab is active on load
```

### US-006 — Minimum Role empty-state guidance

```gherkin
Scenario: Minimum Role tab shows empty-state guidance when no target is entered
  Given a user opens the Minimum Role tab in "Monthly savings target" mode
  When no savings target has been entered
  Then a visible note or empty-state message is shown explaining that a target is needed
  And the note text explains what the ranking will show once a target is entered
```

### US-007 — Region labels expanded

```gherkin
Scenario: Region dropdown displays expanded labels for acronym regions
  Given a user opens the Region dropdown on the Cost of living tab
  When the dropdown is open
  Then "ASEAN" is displayed as "ASEAN (Southeast Asia)" or equivalent expanded form
  And "MENA" is displayed as "MENA (Middle East & North Africa)" or equivalent expanded form
```

### US-008 — 320 px horizontal overflow resolved

```gherkin
Scenario: Calculator page does not overflow horizontally at 320 px
  Given a user views the calculator at 320 px viewport width
  When the Cost of living tab is active
  Then document.body.scrollWidth equals window.innerWidth (no horizontal overflow)
  Or a visible scroll affordance (shadow or indicator) is present for any intentionally scrollable container
```

### US-009 — Tools index: tool descriptions

```gherkin
Scenario: Tools index displays a description below each tool title
  Given a user views the /en/tools page
  When the page has loaded
  Then the "Cost of Living Calculator" entry includes a one-to-two sentence description
  And the description indicates the tool's purpose and data scope
```

### US-010 — Back link restores filter state

```gherkin
Scenario: Back to all cities restores the filter state from before drill-in
  Given a user has set Region to "ASEAN" and Country to "Singapore"
  When the user selects City "Singapore" and drills into the city detail
  And the user clicks the "Back to all cities" link
  Then the Region filter returns to "ASEAN"
  And the Country filter returns to "Singapore"
  And the City filter returns to "All cities"
```

### US-011 — Cost of living tab aria-describedby

```gherkin
Scenario: Cost of living tab has an aria-describedby attribute
  Given a screen reader user focuses the Cost of living tab button
  When the tab is announced
  Then the announcement includes a description equivalent to "Compare monthly living costs by city"
  And the description is provided via aria-describedby referencing a span element
```

### US-012 — OOP abbreviation inline tooltip

```gherkin
Scenario: Healthcare (OOP) label provides an inline definition
  Given a user views the city detail or table with Healthcare (OOP) visible
  When the user hovers or focuses the "OOP" abbreviation
  Then a tooltip or title attribute displays "out-of-pocket — healthcare you pay yourself"
  And the full definition is not solely in a separate paragraph distant from the abbreviation
```

## Scope

### In scope

- Savings tab: add salary input placeholder and empty-state guidance note (UWT-001).
- Minimum Role tab: add visible group label for mode radio group (UWT-002).
- Tab bar: surface existing sr-only sub-descriptions as visible text; add Cost-of-living
  description (UWT-003, UWT-011).
- Savings tab: add currency selector consistent with Minimum Role tab (UWT-004).
- URL state: encode active tab in query param (UWT-005).
- Minimum Role tab: add empty-state guidance when target input is blank (UWT-006).
- Region dropdown: expand ASEAN and MENA labels (UWT-007).
- 320 px reflow: fix horizontal overflow (UWT-008).
- Tools index: add per-tool description (UWT-009).
- Back link: preserve filter state in href (UWT-010).
- OOP abbreviation: wrap in `<abbr title="...">` (UWT-012).
- All changes applied to both `en` and `id` locales.

### Out of scope

- Changing calculation methodology or data.
- Adding new cities, roles, or currencies beyond those already present.
- Full WCAG contrast / keyboard-trap audit (deferred to `web-exploratory-tester`).
- Design token / visual regression (deferred to `web-design-tester`).
- 768 px tablet breakpoint (same class of issues as 375 px; no distinct blocker found).

## Product Risks

- Adding visible sub-descriptions to tabs increases visual complexity; review that the
  descriptions remain brief and do not crowd the tab bar at mobile widths.
- Currency selector on the Savings tab requires the calculation to handle non-USD salary inputs;
  this is a data-layer concern to be resolved in `tech-docs.md` at plan-promotion time.
- URL state for tabs must not conflict with the existing `?city=` and `?tab=cost` city-select URL
  pattern.
