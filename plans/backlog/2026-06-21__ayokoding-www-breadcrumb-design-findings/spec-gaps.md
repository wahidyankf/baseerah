# Spec Gaps — AyoKoding Breadcrumb Design

## SG-B-001 — Breadcrumb locale-label parity spec

**Observed/desired design behaviour:** On any bilingual page, breadcrumb item labels should render
in the locale matching `html[lang]`. Currently no Gherkin scenario in `specs/` covers breadcrumb
label localization.

**Where it applies:** `/en/tools/cost-of-living-calculator` and
`/id/tools/cost-of-living-calculator`.

**Why it is spec-worthy:** Localization regressions in navigation chrome are easy to introduce
silently and hard to catch without a locale-covering E2E scenario. A spec scenario would protect
the fix for DWT-B-001 from future regressions.

**Proposed Gherkin:**

```gherkin
Feature: Cost-of-living calculator breadcrumb

  Scenario: Breadcrumb labels are in English on EN locale
    Given I navigate to /en/tools/cost-of-living-calculator
    When the page loads
    Then the breadcrumb displays "Home", "Tools", and "Calculator" in English
    And html[lang] is "en"

  Scenario: Breadcrumb labels are in Indonesian on ID locale
    Given I navigate to /id/tools/cost-of-living-calculator
    When the page loads
    Then the breadcrumb displays "Beranda", "Alat", and "Kalkulator" in Indonesian
    And html[lang] is "id"
```

**Target `specs/` feature file:**
`specs/apps/ayokoding-www/features/cost-of-living-calculator/breadcrumb.feature` (new file).

---

## SG-B-002 — Breadcrumb no-overflow at mobile spec

**Observed/desired design behaviour:** The breadcrumb `ol` must not overflow the 375 px viewport
at any supported locale. Currently no spec guards this.

**Where it applies:** 375 px viewport, both `/en/` and `/id/` locales.

**Why it is spec-worthy:** Without a spec, a future label change (longer translation, additional
crumb) could silently break mobile layout. The spec pins the reflow guarantee.

**Proposed Gherkin:**

```gherkin
  Scenario: Breadcrumb does not overflow at 375 px — EN locale
    Given I navigate to /en/tools/cost-of-living-calculator on a 375 px viewport
    When the page loads
    Then no breadcrumb element extends past the right edge of the 375 px viewport

  Scenario: Breadcrumb does not overflow at 375 px — ID locale
    Given I navigate to /id/tools/cost-of-living-calculator on a 375 px viewport
    When the page loads
    Then no breadcrumb element extends past the right edge of the 375 px viewport
```

**Target `specs/` feature file:**
`specs/apps/ayokoding-www/features/cost-of-living-calculator/breadcrumb.feature` (same new file
as SG-B-001).
