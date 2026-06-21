# Product Requirements — AyoKoding Calculator URL-State Exploratory

## Personas

**Relocation-evaluator (primary)**: A software engineer comparing cities for potential relocation.
Shares a calculator URL with a specific city and household configuration with a recruiter or
partner. Expects the recipient to land in exactly the same view.

**Indonesian-language reader (secondary)**: A user browsing ayokoding.com in Indonesian. Expects
all visible text — including breadcrumb navigation — to be in Indonesian, consistent with the rest
of the id-locale experience.

## User Stories

### EWT-001 — Breadcrumb "Calculator" label in Indonesian

As an Indonesian-language user,
when I navigate to `/id/tools/cost-of-living-calculator`,
I want the breadcrumb to read "Beranda / Alat / Kalkulator" (or the localized equivalents),
so that the navigation feels consistent with the rest of the Indonesian-locale UI.

## Gherkin Acceptance Criteria

```gherkin
Scenario: Breadcrumb current-page crumb is localized on the id locale
  Given the user navigates to "/id/tools/cost-of-living-calculator"
  When the page renders
  Then the breadcrumb nav shows a current-page crumb with localized Indonesian text
  And the crumb text is not the English word "Calculator"
```

## In-Scope

- Localize the "Calculator" breadcrumb crumb in `CalculatorBreadcrumb` using the i18n `t()` helper.
- Verify the en locale breadcrumb retains its current "Calculator" text.
- Verify all breadcrumb `href` values are unaffected by the change.

## Out-of-Scope

- Replacing the bespoke `CalculatorBreadcrumb` with the shared `Breadcrumb` design-system
  primitive (that is a separate design-system concern tracked in DWT-B-004).
- Any changes to the URL state logic, calculator controls, or other tabs.
