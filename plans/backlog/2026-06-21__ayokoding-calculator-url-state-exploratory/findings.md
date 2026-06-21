# Findings — AyoKoding Calculator URL-State Exploratory

Total: 1 finding (1 Minor).

---

## EWT-001 — Breadcrumb "Calculator" crumb hardcoded in English on the id locale

**Severity**: Minor
**Priority**: High
**Area / Component**: `CalculatorBreadcrumb`
**Defect type**: Content / Localization
**Environment**: `http://localhost:3101/id/tools/cost-of-living-calculator` — Playwright Chromium,
1280 px and 375 px, locale id, 2026-06-21
**Reproducibility**: Always

**Steps to Reproduce**:

1. Start the ayokoding-www dev server on port 3101.
2. Navigate to `/id/tools/cost-of-living-calculator`.
3. Read the breadcrumb navigation at the top of the page.

**Expected Result**: The final breadcrumb crumb renders the Indonesian translation of "Calculator"
(e.g., "Kalkulator"), consistent with the localized H1 "Kalkulator Biaya Hidup" and all other
page labels, per the spec scenario "Indonesian locale is fully translated"
(`cost-of-living-calculator.feature › Scenario: Indonesian locale is fully translated`).

**Actual Result**: The breadcrumb renders "Home / Tools / Calculator" — the English word
"Calculator" appears as the final crumb even though the rest of the page is fully in Indonesian.

**Evidence**:

- `./evidence/phase-11-id-locale-1280px.png` — id locale at 1280 px showing English crumb
- `./evidence/phase-21-mobile-id-375px.png` — id locale at 375 px confirming the same issue

**Suggested fix locus**:
`apps/ayokoding-www/src/features/cost-of-living-calculator/shell/calculator-breadcrumb.tsx` —
the "Calculator" string literal on line 28 is hardcoded in English. Replace with
`t(locale, "<breadcrumb_key>")` and add the key to the en and id translation catalogs. The
component already imports `useLocale()`.
