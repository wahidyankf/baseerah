# Product Requirements — AyoKoding Cost-of-Living Calculator Design Findings

## Personas

**Ardi** — Indonesian software engineer on a mid-range Android phone (375 px viewport). Uses the
site in Indonesian. Switches to dark mode in the evening. Expects the calculator to look like the
polished tools he uses at work.

**Sarah** — Senior SWE in Europe, desktop browser, dark mode always on. Uses the Min-role tab to
benchmark relocation scenarios. Expects clear visual feedback on which tab is selected.

**Reza** — Product lead evaluating the AyoKoding brand for a team blog mention. Judges by
first-impression design quality.

## User Stories and Acceptance Criteria

### US-01 — Tab bar fits within the viewport on mobile, both locales

As Ardi, the tab bar for the three calculator tabs fits within the 375 px viewport without any label
being cut off, in both English and Indonesian, so I can tap any tab without confusion.

```gherkin
Scenario: Tab bar visible at 375 px — EN locale
  Given I navigate to /en/tools/cost-of-living-calculator on a 375 px viewport
  When the page loads
  Then all three tab labels are visible and no tab element extends past the right edge of the viewport
  And the active tab label is visually distinguished from inactive tabs

Scenario: Tab bar visible at 375 px — ID locale
  Given I navigate to /id/tools/cost-of-living-calculator on a 375 px viewport
  When the page loads
  Then all three tab labels ("Biaya hidup", "Tabungan", "Jabatan minimum") are fully visible
  And no tab right-edge overflows the viewport boundary (375 px)
```

### US-02 — Active tab is clearly distinguishable in dark mode

As Sarah, the currently active tab shows a clear visual fill in dark mode (not a near-invisible
translucent background), so I always know which section I'm viewing without having to read the
label twice.

```gherkin
Scenario: Active tab state in dark mode
  Given I navigate to /en/tools/cost-of-living-calculator with dark mode enabled
  When I view the tab bar
  Then the active tab has a clearly distinguishable background fill
  And the active tab background colour uses the design-token primary colour (not raw hex)
  And the active tab text is legible against its background with at least a 4.5:1 contrast ratio
```

### US-03 — Savings tab gross-salary input is visually styled

As any user on the Savings tab, the gross salary input field looks like a proper form field —
with a visible border, border radius, and consistent height — matching the mockup in
`plans/done/2026-06-19__ayokoding-www-salary-savings-calculator/assets/ui-savings-option-a-net-savings-table.png`.

```gherkin
Scenario: Gross salary input is styled at 375 px
  Given I navigate to /en/tools/cost-of-living-calculator?tab=savings on a 375 px viewport
  When the page loads
  Then the gross salary input has a visible border using the design-system border token
  And the input uses a consistent border-radius matching the theme radius scale
  And the input renders with a minimum height of 44 px for touch-target compliance

Scenario: Gross salary input is styled at 1280 px
  Given I navigate to /en/tools/cost-of-living-calculator?tab=savings on a 1280 px viewport
  When the page loads
  Then the gross salary input matches the styled input shown in the hi-fi mockup
  And no raw hex or off-token border color is computed on the element
```

### US-04 — Min-role baseline-source control renders as SegmentedControl

As Sarah, the "My salary / Reference role / Monthly savings target" selector on the Min-role tab
appears as a three-option SegmentedControl matching the hi-fi mockup, not as a plain dropdown
select.

```gherkin
Scenario: Baseline-source control renders as SegmentedControl at 1280 px
  Given I navigate to /en/tools/cost-of-living-calculator?tab=min-role on a 1280 px viewport
  When the page loads
  Then a segmented control with three options is visible:
    "Monthly savings target", "Reference role", and "My salary"
  And the selected option has the primary background fill
  And no bare <select> element is rendered for this control

Scenario: Baseline-source SegmentedControl fits within 375 px (EN locale)
  Given I navigate to /en/tools/cost-of-living-calculator?tab=min-role on a 375 px viewport
  When the page loads
  Then the three-option segmented control fits within the viewport without overflow
```

### US-05 — Geo-filter selects carry consistent border theming

As any user, the Region, Country, and City filter selects on the Cost-of-living tab look consistent
with the household Adult/Kids selects above them — same border token, same border-radius.

```gherkin
Scenario: Geo-filter selects are consistently styled
  Given I navigate to /en/tools/cost-of-living-calculator on a 1280 px viewport
  When I inspect the Region, Country, and City selects
  Then each select carries the same border token as the Adults and School-age-kids selects
  And all selects share the same border-radius from the theme radius scale
```

## In-Scope

- Tab bar overflow fix (DWT-001)
- Dark mode active-tab colour restoration (DWT-002)
- Savings tab gross-salary input styling (DWT-003)
- Min-role baseline-source SegmentedControl (DWT-004)
- Geo-filter select border consistency (DWT-005)
- H1 title mismatch investigation (DWT-006, if confirmed a translation regression)
- Baseline SegmentedControl ID-locale overflow at 375 px (DWT-007, resolved by DWT-004 or label
  shortening)

## Out-of-Scope

- New features or data changes to the calculator
- Tablet (768 px) breakpoint pass — no regression reported; deferrable
- City-detail drill-down design evaluation — not exercised in this run
- Correctness / behavioural testing (belongs to `web-exploratory-tester`)
- Usability testing (belongs to `web-usability-tester`)
- Source-level token/a11y audit (belongs to `swe-ui-checker`)
