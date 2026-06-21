# Product Requirements — Calculator Test-Fixing

## Product Overview

Close the still-valid three-tester findings on the AyoKoding cost-of-living calculator so the tool
is correct, accessible (WCAG AA), visually consistent, and regression-protected by Gherkin specs.
The product surface is the client-rendered calculator page and the tools index, in both `en` and
`id` locales.

## Personas

Solo-maintainer hats and consuming agents:

- **Job-seeking developer (end user)** — compares cities/roles to decide where to work; needs
  correct results, discoverable controls, and a tool that works on a 320 px phone.
- **Screen-reader / keyboard user (end user)** — relies on associated descriptions, real `<abbr>`
  semantics, and adequate touch targets.
- **Frontend engineer (maintainer)** — implements fixes against the FCIS structure.
- **E2E engineer (maintainer)** — proves runtime behaviour across locales/breakpoints.
- **Spec author (maintainer)** — encodes behaviour as Gherkin.

## User Stories

- **US-1 (EWT-001)**: As a job-seeking developer on the Minimum-role tab, I want the
  qualifying/non-qualifying divider to appear even when my savings target is 0, so the tool clearly
  marks which role is the minimum.
- **US-2 (DWT-B-003/004, UWT-013)**: As a user navigating the calculator, I want the breadcrumb to
  match the site's shared style and end with the full page title, so navigation is consistent and
  unambiguous.
- **US-3 (UWT-016/DWT-005, UWT-008)**: As a mobile user, I want geo-filter controls large enough to
  tap and a layout with no horizontal scroll at 320 px, so I can use the tool one-handed.
- **US-4 (UWT-011, UWT-003, UWT-012)**: As a screen-reader/sighted user, I want every tab to have a
  discoverable, associated description and acronyms wrapped in `<abbr>`, so the UI is understandable.
- **US-5 (UWT-004, UWT-006)**: As a user on the Savings and Minimum-role tabs, I want a clear active
  currency and an empty-state guidance message before I enter input, so the tool is not misleading.
- **US-6 (UWT-007, UWT-014, UWT-015, UWT-009)**: As a user filtering by geography, I want the region
  set to be complete, auto-changes to be advised, deep-link back-links to be predictable, and the
  tools-index link to carry a description, so the tool behaves predictably.

## Acceptance Criteria (Gherkin)

Each scenario uses exactly one primary `Given` / `When` / `Then`, with extras chained via
`And`/`But`.

### AC-1 — Minimum-role divider at a zero savings target (EWT-001)

```gherkin
Scenario: Zero savings target still renders the qualifying divider
  Given I am on the Minimum-role tab with the baseline source set to savings target
  When I set the monthly savings target to "0"
  Then the qualifying divider is rendered above the below-minimum roles
  And the lowest-clearing role is marked as the minimum
```

### AC-2 — Breadcrumb uses the shared primitive with chevrons (DWT-B-003/004)

```gherkin
Scenario: Calculator breadcrumb renders chevron separators from the shared primitive
  Given I am on the calculator page
  When the breadcrumb renders
  Then the separators are ChevronRight icons rather than literal "/" characters
  And the breadcrumb is produced by the shared navigation Breadcrumb component
```

### AC-3 — Final crumb equals the page title in both locales (UWT-013)

```gherkin
Scenario Outline: Breadcrumb final crumb matches the H1 in each locale
  Given I am on the calculator page in locale "<locale>"
  When the breadcrumb renders its current-page crumb
  Then the current-page crumb text equals the page H1 "<title>"

  Examples:
    | locale | title                     |
    | en     | Cost of Living Calculator |
    | id     | Kalkulator Biaya Hidup    |
```

### AC-4 — Geo-filter selects meet the 44 px touch target (UWT-016/DWT-005)

```gherkin
Scenario: Geo-filter selects are at least 44px tall
  Given I am on the calculator page at a 375px viewport
  When I measure each of the region, country, and city selects
  Then each select has a rendered height of at least 44 pixels
  And each select uses the shared web-ui control styling
```

### AC-5 — No horizontal overflow at 320 px (UWT-008)

```gherkin
Scenario: The calculator page does not scroll horizontally at 320px
  Given I am on the calculator page at a 320px viewport
  When the page finishes rendering
  Then the document scroll width does not exceed the 320px viewport width
```

### AC-6 — Every tab has a discoverable, associated description (UWT-011, UWT-003)

```gherkin
Scenario: All three tabs expose a visible, associated description
  Given I am on the calculator page
  When I inspect each tab trigger and its description element
  Then each trigger references its description via aria-describedby
  And each description (including the Cost-of-living tab) is visibly rendered rather than sr-only
```

### AC-7 — OOP is wrapped in an abbr element (UWT-012)

```gherkin
Scenario: The OOP acronym uses an abbr element
  Given I am on a tab that shows the Healthcare (OOP) column
  When I inspect the OOP acronym in the markup
  Then it is wrapped in an <abbr> element whose title is "out-of-pocket"
```

### AC-8 — Active currency is surfaced on the Savings tab (UWT-004)

```gherkin
Scenario: The Savings tab surfaces a currency selector instead of a hardcoded USD label
  Given I am on the Savings tab
  When I read the gross monthly salary control
  Then a currency selector (or an explicit active-currency indicator) is shown
  But the gross salary label does not hardcode the literal "USD"
```

### AC-9 — Minimum-role empty-state guidance (UWT-006)

```gherkin
Scenario: Minimum-role tab hides the table and shows guidance before a target is entered
  Given I am on the Minimum-role tab with the savings-target baseline and a blank target
  When the page renders
  Then the full role table is hidden
  And a guidance message instructs me to enter a target
```

### AC-10 — Region set is complete and grouped as intended (UWT-007)

```gherkin
Scenario: The region selector lists every intended region
  Given I am on the calculator page
  When I open the region selector
  Then it lists exactly the intended regions present in the dataset
  And no intended region is missing from the list
```

### AC-11 — Country auto-changing the region is advised (UWT-014)

```gherkin
Scenario: Selecting a country that changes the region shows an advisory
  Given I am on the calculator page with no region selected
  When I select a country whose only region differs from the current selection
  Then the region selection updates to that country's region
  And a visible advisory explains that the region was set automatically
```

### AC-12 — City deep-link back-link is predictable (UWT-015)

```gherkin
Scenario: A city-only deep link produces a predictable back-link
  Given I open the calculator with a city-only deep link such as "?city=london"
  When I open the single-city detail and read the back link
  Then the back link returns to the documented predictable target
  And the back-link behaviour matches the assumption recorded in tech-docs.md
```

### AC-13 — Tools-index link carries a description (UWT-009)

```gherkin
Scenario: The tools index calculator link has a description sibling
  Given I am on the tools index page
  When I read the cost-of-living calculator entry
  Then a description element distinct from the link text is shown
```

## Product Scope

**In scope**: the calculator page, the tools index, both locales, and companion specs. **Out of
scope**: new calculator features, dataset/FX/tax changes, deployment changes, and resolved findings.

## Product-Level Risks

- A stale finding (UWT-007 framed as a missing "expansion" group) could drive an unnecessary
  change; mitigated by verifying the dataset's true region set first (see `tech-docs.md`).
- An empty-state or currency change could alter existing passing specs; mitigated by RED-first TDD
  and re-running the full unit + specs gate per phase.
