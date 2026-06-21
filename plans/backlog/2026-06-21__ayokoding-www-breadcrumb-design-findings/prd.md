# Product Requirements — AyoKoding Breadcrumb Design Findings

## Personas

**Budi** — Indonesian software engineer on a mobile phone (375 px). Navigates the calculator in
Indonesian. Expects the breadcrumb to say "Beranda / Alat / Kalkulator", not English words.

**Mia** — English-speaking developer evaluating AyoKoding design consistency for a blog review.
Notices the "/" separator on the calculator breadcrumb differs from the ChevronRight icons used
in article breadcrumbs elsewhere on the site.

**Wira** — Contributing engineer who finds two breadcrumb implementations in the same codebase
and must now maintain both when the visual design is updated.

## User Stories and Acceptance Criteria

### US-01 — Breadcrumb labels are localized per locale

As Budi, the breadcrumb on `/id/tools/cost-of-living-calculator` shows Indonesian-language labels
("Beranda", "Alat", "Kalkulator") so the navigation is fully bilingual.

```gherkin
Scenario: Breadcrumb labels localized — Indonesian
  Given I navigate to /id/tools/cost-of-living-calculator
  When the page loads
  Then the breadcrumb nav shows "Beranda" as the Home link label
  And the breadcrumb nav shows "Alat" as the Tools link label
  And the breadcrumb nav shows "Kalkulator" as the current-page label
  And html[lang] is "id"

Scenario: Breadcrumb labels localized — English
  Given I navigate to /en/tools/cost-of-living-calculator
  When the page loads
  Then the breadcrumb nav shows "Home" as the Home link label
  And the breadcrumb nav shows "Tools" as the Tools link label
  And the breadcrumb nav shows "Calculator" as the current-page label
  And html[lang] is "en"
```

### US-02 — Breadcrumb reflows safely at 375 px after localization

As Budi, the breadcrumb remains single-line or wraps gracefully on a 375 px screen even with
longer Indonesian labels, with no element protruding past the viewport edge.

```gherkin
Scenario: Breadcrumb does not overflow at 375 px — Indonesian locale
  Given I navigate to /id/tools/cost-of-living-calculator on a 375 px viewport
  When the page loads
  Then the breadcrumb ol right edge does not exceed 375 px
  And all breadcrumb items are visible without horizontal scroll
```

### US-03 — Breadcrumb separator is visually consistent with site breadcrumbs

As Mia, the cost-of-living calculator breadcrumb uses the same separator style as the article
and content-page breadcrumbs on AyoKoding, so the site design is cohesive.

```gherkin
Scenario: Breadcrumb separator style matches site standard
  Given I navigate to /en/tools/cost-of-living-calculator
  When I inspect the breadcrumb separator
  Then the separator uses the ChevronRight icon (not a literal "/" character)
  And the separator icon has aria-hidden="true"
  And the separator is visually consistent with breadcrumbs on article pages
```

### US-04 — No duplicate breadcrumb implementations

As Wira, there is a single breadcrumb implementation that all pages in the app share, so
design changes need to be made in only one place.

```gherkin
Scenario: Calculator uses the shared Breadcrumb component
  Given the features/navigation/shell/breadcrumb.tsx shared component exists
  When the cost-of-living calculator page is built
  Then CalculatorBreadcrumb does not reimplement breadcrumb markup independently
  And the calculator breadcrumb is built on or replaced by the shared Breadcrumb primitive
```

## In-Scope

- Localization of breadcrumb link labels for EN and ID locales (DWT-B-001)
- Adding `flex-wrap` to the breadcrumb `ol` for safe reflow when translated labels are applied
  (DWT-B-002)
- Replacing the literal "/" separator with a `ChevronRight` icon for visual consistency
  (DWT-B-003)
- Consolidating `CalculatorBreadcrumb` onto the shared `Breadcrumb` component or extending the
  shared component to support the fixed-path calculator use-case (DWT-B-004)

## Out-of-Scope

- Changing the breadcrumb URL paths or navigation structure
- Adding a breadcrumb to any other page (that is a separate feature decision)
- Redesigning the spacing or color of the breadcrumb beyond token compliance
- Calculator table or controls visual regression (covered in prior plan
  `2026-06-21__ayokoding-www-cost-of-living-design-findings`)
- Source-level token/a11y audit (belongs to `swe-ui-checker`)
- Correctness / behavioral testing (belongs to `web-exploratory-tester`)
