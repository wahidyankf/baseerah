# Design Spec Gaps — AyoKoding Cost-of-Living Calculator

These are on-design behaviours observed (or mandated by the design ground truth) that existing
`specs/**` Gherkin files do not yet describe. Each is a proposal for maintainer confirmation — not
yet committed to `specs/`.

---

## SG-001 — Tab bar must not overflow viewport at any supported breakpoint and locale

**Observed/desired behaviour**: The three-tab bar must fit within the viewport at 375 px for every
supported locale, including Indonesian where labels are longer.

**Where it applies**: `en` and `id` locales, 375 px viewport, tab bar on all three tabs.

**Why it is spec-worthy**: This is a responsive design rule that protects against regression if
future tab labels are lengthened or new locales with longer strings are added. A Gherkin scenario
makes this a machine-checkable regression gate.

**Proposed Gherkin**:

```gherkin
Scenario: Tab bar fits within 375 px viewport for all supported locales
  Given I navigate to the cost-of-living calculator in the "<locale>" locale
  And the viewport width is 375 px
  When the page loads
  Then all tab trigger elements have their right edge within the viewport width
  And no tab trigger requires horizontal scrolling to reach

  Examples:
    | locale |
    | en     |
    | id     |
```

**Target specs file**: `specs/apps/ayokoding-www/cost-of-living-calculator/responsive.feature`
(new file)

---

## SG-002 — Active tab maintains primary-colour fill in dark mode

**Observed/desired behaviour**: When dark mode is active, the selected tab in the calculator tab bar
retains a clearly visible primary-colour background (`--color-primary` token), not a translucent
muted background.

**Where it applies**: Both locales, 375 px and 1280 px, dark mode enabled.

**Why it is spec-worthy**: The dark-mode token override for the active tab is a fragile CSS
specificity interaction. Without a Gherkin scenario, this regression is invisible to the Playwright
E2E suite.

**Proposed Gherkin**:

```gherkin
Scenario: Active tab has visible fill in dark mode
  Given I navigate to the cost-of-living calculator with dark mode enabled
  When I view the tab bar
  Then the active tab trigger has a non-transparent background
  And the computed background-color of the active tab trigger is not "rgba(0, 0, 0, 0)"
  And the computed background-color is not a translucent variant of the input token
```

**Target specs file**:
`specs/apps/ayokoding-www/cost-of-living-calculator/design-tokens.feature` (new file)

---

## SG-003 — All interactive form controls meet 44 px minimum touch-target height

**Observed/desired behaviour**: Every select, input, and button on the calculator page must render
with at least 44 px height for touch-target compliance, per WCAG 2.5.5 (AAA) and the
[Accessibility First principle](../../../repo-governance/principles/content/accessibility-first.md).

**Where it applies**: All controls — household selects, geo-filter selects, savings input, sort
button, SegmentedControl buttons.

**Why it is spec-worthy**: Touch-target compliance is a design-system rule applied to all
user-facing controls. The GeoFilters selects (29 px) and the savings input currently violate it.

**Proposed Gherkin**:

```gherkin
Scenario: Form controls meet 44 px touch-target height
  Given I navigate to the cost-of-living calculator on a 375 px viewport
  When the page loads
  Then every <select>, <input>, and <button> element in the calculator controls section
    has a rendered height of at least 44 px
  And every <button> in any SegmentedControl has a rendered height of at least 44 px
```

**Target specs file**:
`specs/apps/ayokoding-www/cost-of-living-calculator/accessibility.feature` (new file)

---

## SG-004 — Baseline-source control renders as SegmentedControl on Min-role tab

**Observed/desired behaviour**: The "My salary / Reference role / Monthly savings target" selector
on the Min-role tab renders as an accessible SegmentedControl (`role="radiogroup"` with
`role="radio"` buttons), not a native `<select>`.

**Where it applies**: Min-role tab, both locales, both breakpoints.

**Why it is spec-worthy**: The control type is part of the committed design; replacing the current
`<select>` with a SegmentedControl changes the DOM structure. A Gherkin scenario prevents future
regression back to a select.

**Proposed Gherkin**:

```gherkin
Scenario: Baseline-source control is a SegmentedControl on Min-role tab
  Given I navigate to the cost-of-living calculator Min-role tab
  When the page loads
  Then a radiogroup element is present with aria-label matching "Baseline source" or "Sumber baseline"
  And the radiogroup contains exactly 3 radio buttons
  And the buttons are labelled "Monthly savings target", "Reference role", and "My salary"
    (or their translated equivalents)
```

**Target specs file**:
`specs/apps/ayokoding-www/cost-of-living-calculator/min-role.feature` (extend existing, or create)
