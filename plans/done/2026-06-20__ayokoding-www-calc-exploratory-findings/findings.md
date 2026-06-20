# Findings — Cost-of-Living Calculator Exploratory Testing

**Session date**: 2026-06-19 (initial); 2026-06-19 (re-run, merged)
**Tester agent**: `web-exploratory-tester`
**Ground truth**: `specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/cost-of-living-calculator.feature`

---

## Re-verification of prior findings (2026-06-19 re-run)

A second spec-aware exploratory pass re-checked every original finding. All seven remain open — none
show evidence of a fix attempt.

| ID      | Verdict       | Evidence                                                                                                                 |
| ------- | ------------- | ------------------------------------------------------------------------------------------------------------------------ |
| EWT-001 | STILL-PRESENT | `?country=id` sets country select value to `""`; table filters to 1 row but both dropdowns show "All countries/regions". |
| EWT-002 | STILL-PRESENT | `?city=jakarta` sets city select value to `""`; city detail renders, filter dropdown shows "All cities".                 |
| EWT-003 | STILL-PRESENT | `html lang` on `/id/` = `"en"` on dev **and** production; root `layout.tsx` hardcodes `lang="en"`.                       |
| EWT-004 | STILL-PRESENT | Desktop country column on `/id/` returns English names; `cost-of-living.tsx` still uses `.name.en`.                      |
| EWT-005 | STILL-PRESENT | Min-role `/id/` best-city text still English (`Austin, United States`); `min-role.tsx` still `.name.en`.                 |
| EWT-006 | STILL-PRESENT | Sort button measured `h: 20`, `paddingTop/Bottom: 0px`. No change.                                                       |
| EWT-007 | STILL-PRESENT | Salary input `min` attribute is `null`; annual gross at `−1000` shows `−12,000 USD`. No validation added.                |

The re-run also surfaced six new defects (EWT-008…EWT-013) and two new spec gaps (SG-005, SG-006).

---

## EWT-001 — Country deep link does not pre-select geo filter dropdowns

- **Severity**: Major
- **Priority**: High
- **Area**: Navigation / Deep Links / Geo Filters
- **Defect type**: Functional
- **Reproducibility**: Always

**Environment**

URL: `http://localhost:3101/en/tools/cost-of-living-calculator?tab=cost&country=id`
Browser: Chromium (Playwright), viewport 1440 x 900

**Steps to reproduce**

1. Open `http://localhost:3101/en/tools/cost-of-living-calculator?tab=cost&country=id` directly.
2. Wait for page to finish loading.
3. Observe the Region, Country, and City filter dropdowns.

**Expected result**

Per spec `cost-of-living-calculator.feature > Scenario: Clicking a country opens Cost-of-living filtered to that country`: the Country dropdown shows "Indonesia", Region shows "ASEAN", City shows "All cities". Table filtered to Indonesian cities.

**Actual result**

Region: "All regions". Country: "All countries". City: "All cities". Table IS correctly filtered to 1 Jakarta row, but filter UI shows no active state. User sees no indication of which country is filtered.

**Suggested fix locus**

`apps/ayokoding-www/src/features/cost-of-living-calculator/shell/geo-filters.tsx` — `useState(null)` initializers for region/countryId/cityId are never seeded from URL params. Parent `CostOfLivingCalculatorContent` reads URL params into `geoScope` correctly but does not pass initial values to `<GeoFilters>`. Fix: add `initialScope` prop or have `GeoFilters` read URL params at mount.

---

## EWT-002 — City deep link does not pre-select City filter dropdown

- **Severity**: Major
- **Priority**: High
- **Area**: Navigation / Deep Links / Geo Filters
- **Defect type**: Functional
- **Reproducibility**: Always

**Environment**

URL: `http://localhost:3101/en/tools/cost-of-living-calculator?tab=cost&city=jakarta`
Browser: Chromium (Playwright), viewport 1440 x 900

**Steps to reproduce**

1. Open `http://localhost:3101/en/tools/cost-of-living-calculator?tab=cost&city=jakarta` directly.
2. Wait for page to finish loading.
3. Observe the City filter dropdown.

**Expected result**

Per spec `cost-of-living-calculator.feature > Scenario: Clicking a city name opens its single-city cost-of-living detail > And the City filter is pre-selected to that city`: City dropdown shows "Jakarta".

**Actual result**

City dropdown: "All cities". City detail view for Jakarta displays correctly, but the filter UI does not reflect the active scope.

**Suggested fix locus**

Same as EWT-001: `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/geo-filters.tsx`.

---

## EWT-003 — Indonesian locale page carries `html lang="en"` instead of `lang="id"`

- **Severity**: Major
- **Priority**: High (WCAG 3.1.1 Level A violation; affects all screen-reader users on /id/)
- **Area**: Locale / Accessibility
- **Defect type**: Accessibility
- **Reproducibility**: Always

**Environment**

URL: `http://localhost:3101/id/tools/cost-of-living-calculator`
Verified via Playwright `page.getAttribute('html', 'lang')` returning `"en"`.

**Steps to reproduce**

1. Open `http://localhost:3101/id/tools/cost-of-living-calculator`.
2. Inspect the `<html>` element or run `curl -s http://localhost:3101/id/tools/cost-of-living-calculator | grep '<html'`.

**Expected result**

WCAG 3.1.1 (Level A): default human language must be programmatically determinable. `/id/` pages require `lang="id"`. Screen readers announce content in the correct language; browser translation correctly identifies the page.

**Actual result**

`<html lang="en">` on both `/en/` and `/id/` pages. Playwright confirmed.

**Root cause hypothesis**

`apps/ayokoding-www/src/app/layout.tsx` hardcodes `<html lang="en">`. In Next.js App Router the root layout owns `<html>`; `[locale]/layout.tsx` cannot override it. The locale param exists in `[locale]/layout.tsx` but is not threaded back to root layout's `lang` attribute.

**Suggested fix locus**

`apps/ayokoding-www/src/app/layout.tsx` — make `lang` dynamic. In Next.js App Router this requires reading the locale URL segment in the root layout (via params or `generateStaticParams`) and passing it to `lang`.

---

## EWT-004 — Desktop table shows English city/country names on Indonesian locale; mobile cards show Indonesian names

- **Severity**: Major
- **Priority**: Medium
- **Area**: Locale / Behavioural Consistency
- **Defect type**: Functional + Consistency
- **Reproducibility**: Always

**Environment**

URL: `http://localhost:3101/id/tools/cost-of-living-calculator`
Tested at viewport 1440 x 900 (desktop) and 375 x 812 (mobile)

**Steps to reproduce**

1. Open `http://localhost:3101/id/tools/cost-of-living-calculator`.
2. At >= 768 px, observe Country and City columns in the cost-of-living table.
3. Resize to < 768 px; observe city card headers.

**Expected result**

Per spec `Scenario: Indonesian locale is fully translated`: city/country names use Indonesian translations uniformly on desktop and mobile (e.g. "Singapura", not "Singapore").

**Actual result**

Desktop table Country column: "Singapore", "Thailand", "Indonesia", "Malaysia" (English).
Mobile card city names: "Singapura", "Bangkok", "Jakarta", "Kuala Lumpur" (Indonesian).
Same /id/ page, same data, different text at different breakpoints — internal inconsistency.

**Evidence**

Playwright confirmed:

```
Country col on ID locale (desktop): ['Singapore', 'Thailand', 'Indonesia', 'Malaysia', 'Vietnam']
Mobile card city names (ID locale): ['Singapura', 'Bangkok', 'Jakarta', 'Kuala Lumpur', 'Ho Chi Minh City']
```

**Root cause hypothesis**

`cost-of-living.tsx` lines 107 and 110: `r.country?.name.en` and `r.city.name.en` (hardcoded `.en`).
`cost-of-living.tsx` line 148 (mobile card): `r.city.name[locale] ?? r.city.name.en` (locale-aware).
Same pattern in `savings.tsx` lines 144/147 (desktop) vs 183/185 (mobile).

**Suggested fix locus**

- `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/cost-of-living.tsx` lines 107, 110: replace `.name.en` with `.name[locale] ?? .name.en` (`locale` prop already in scope).
- `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/savings.tsx` lines 144, 147: same fix.

---

## EWT-005 — Min-role tab city/country names always English regardless of locale

- **Severity**: Minor
- **Priority**: Low
- **Area**: Locale
- **Defect type**: Functional
- **Reproducibility**: Always

**Environment**

URL: `http://localhost:3101/id/tools/cost-of-living-calculator` → "Jabatan minimum" tab
Browser: Chromium (Playwright), viewport 1440 x 900

**Steps to reproduce**

1. Open `http://localhost:3101/id/tools/cost-of-living-calculator`.
2. Switch to "Jabatan minimum" tab.
3. Enter savings target of 100.
4. Observe "Kota terbaik" column — city name shows.
5. Switch to "Jabatan referensi" mode; observe reference-city dropdown options.

**Expected result**

City/country names show Indonesian translations on /id/ locale.

**Actual result**

"Best city" column: "Tokyo, Japan" (English). Reference-city and my-salary-city dropdown options: English names.

**Root cause hypothesis**

`apps/ayokoding-www/src/features/cost-of-living-calculator/shell/min-role.tsx` uses `.name.en` at lines 134, 214, 256, 351. The `locale` prop is passed to `MinRoleTable` but not forwarded to city/country name lookups.

**Suggested fix locus**

`apps/ayokoding-www/src/features/cost-of-living-calculator/shell/min-role.tsx` lines 134, 214, 256, 351: replace `.name.en` with `.name[locale] ?? .name.en`.

---

## EWT-006 — Sort button in savings tab has 20 px touch target (below WCAG 2.5.8 24 px minimum)

- **Severity**: Minor
- **Priority**: Low
- **Area**: Accessibility / Savings Tab
- **Defect type**: Accessibility
- **Reproducibility**: Always

**Environment**

URL: `http://localhost:3101/en/tools/cost-of-living-calculator` → Savings tab → enter salary
Browser: Chromium (Playwright), measured via `getBoundingClientRect()`

**Steps to reproduce**

1. Navigate to Savings tab; enter any gross salary (e.g. 8000).
2. Observe the "Savings after essentials" column header (it is a clickable sort button).
3. Measure rendered button height.

**Expected result**

WCAG 2.5.8 (Level AA, Target Size Minimum): interactive targets must be >= 24 × 24 CSS px, or have offset spacing providing equivalent effective target area.

**Actual result**

Measured: width 178.3 px, height **20 px**. `paddingTop: "0px"`, `paddingBottom: "0px"`. Parent `<th>` is 40 px tall but `<button>` does not fill it.

**Evidence**

```
Sort button sizes: [{ h: 20, w: 178.328125, paddingTop: '0px', paddingBottom: '0px' }]
```

**Suggested fix locus**

`apps/ayokoding-www/src/features/cost-of-living-calculator/shell/savings.tsx` — sort `<button>` needs `py-1` (adds ~8 px each side → 36 px) or restructure `<th>` so clicking anywhere in header cell triggers sort.

---

## EWT-007 — Negative salary accepted; produces semantically invalid output

- **Severity**: Minor
- **Priority**: Low
- **Area**: Forms & Validation / Savings Tab / Min-role Tab
- **Defect type**: Functional
- **Reproducibility**: Always

**Environment**

URL: `http://localhost:3101/en/tools/cost-of-living-calculator` → Savings tab
Browser: Chromium (Playwright)

**Steps to reproduce**

1. Navigate to Savings tab.
2. In "Gross monthly salary" field, type `-1000`.
3. Observe "Annual gross" display and savings table.

**Expected result**

Salary input rejects or constrains negative values. Either `min="0"` on `<input type="number">` (browser prevents negative in native UI) or a visible client-side validation message appears.

**Actual result**

`-1000` accepted without error. "Annual gross" displays `-12,000 USD`. All city rows show negative savings computed from a negative net income. Mathematically consistent but semantically nonsensical.

**Evidence**

```
Annual gross at -1000 input: -12,000 USD
```

**Suggested fix locus**

- `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/savings.tsx` ~line 98: add `min="0"` to gross salary `<input type="number">`.
- `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/min-role.tsx`: add `min="0"` to `#target-amount-input` and `#my-gross-input`.

---

## EWT-008 — Housing column displays base (city-center) amount when Rural area is selected

- **Severity**: Major
- **Priority**: Medium
- **Area**: Functional / Cost-of-Living Tab / Cost Basis Controls
- **Defect type**: Functional
- **Reproducibility**: Always

**Environment**

URL: `http://localhost:3101/en/tools/cost-of-living-calculator`
Browser: Chromium (Playwright), viewport 1440 x 900

**Steps to reproduce**

1. Open the Cost-of-Living tab (default tab on load).
2. Observe the Housing column for any city (e.g. Jakarta: `8,000,000 IDR`; Singapore: `3,500 SGD`).
3. Click the "Rural" area button in the cost-basis controls.
4. Wait for values to recalculate (~500 ms).
5. Observe the Housing column value.

**Expected result**

Per spec `cost-of-living-calculator.feature > Scenario: Rural area lowers housing versus city center`: the housing expense decreases when Rural is selected. The Housing column should reflect the discounted value: `city.expenses.housing.amount × AREA_MULTIPLIERS["rural"]` (0.75). For Jakarta: `8,000,000 × 0.75 = 6,000,000 IDR`. The displayed per-row Housing figure should match the value used in the Essentials calculation.

**Actual result**

Housing column: `8,000,000 IDR` (unchanged — same as City Center). Essentials column: drops correctly from `13,700,000` to `11,700,000` (a `2,000,000` IDR reduction = `8,000,000 × 0.25`), confirming the calculation applies the 0.75 multiplier internally. The Housing cell displays the base/center amount while the total uses the discounted amount — a display/calculation split. Confirmed identically for Singapore (`3,500 SGD` housing stays unchanged while Essentials drops from `4,328` to `3,453`).

**Evidence**

```
Jakarta Housing before rural: 8,000,000 IDR
Jakarta Housing after rural:  8,000,000 IDR  (no change — wrong)
Jakarta Essentials before rural: 13,700,000 IDR
Jakarta Essentials after rural:  11,700,000 IDR  (correctly discounted)
Expected housing after rural: 8,000,000 × 0.75 = 6,000,000 IDR
```

**Suggested fix locus**

`apps/ayokoding-www/src/features/cost-of-living-calculator/shell/cost-of-living.tsx` line 57: `housing: e.housing.amount` — replace with `housing: e.housing.amount * AREA_MULTIPLIERS[area]` (or extract a `housingLocal(city, area)` helper matching the pattern of `essentialsLocal`). The `area` prop is already passed to `CostOfLivingTable` (line 47).

---

## EWT-009 — Calculator page `<title>` is the generic site name, not the page name

- **Severity**: Minor
- **Priority**: Medium
- **Area**: URL / IA Quality / SEO / Meta
- **Defect type**: Content / Functional
- **Reproducibility**: Always

**Environment**

URL: `http://localhost:3101/en/tools/cost-of-living-calculator` and production `https://www.ayokoding.com/en/tools/cost-of-living-calculator`
Verified via `page.title()` in Playwright and `curl … | grep '<title>'` on production.

**Steps to reproduce**

1. Open `http://localhost:3101/en/tools/cost-of-living-calculator` in a browser.
2. Read the browser tab title (or `document.title`).

**Expected result**

Per `apps/ayokoding-www/src/app/layout.tsx` line 9 (`template: "%s | AyoKoding"`): any page that exports its own `metadata.title` gets a composed title like `"Salary Savings Calculator | AyoKoding"`. The H1 is `"Salary Savings Calculator"` — the page has a clear identity; the title should express it. Browsers, screen readers, history, search engines, and link-sharing previews all rely on the `<title>`.

**Actual result**

Title on both dev and production: `"AyoKoding"` (the default fallback). The `/en/tools/cost-of-living-calculator/page.tsx` exports no `metadata` or `generateMetadata`, so the title template never fires. `/id/` page title is also `"AyoKoding"` (no Indonesian equivalent exposed). No `og:title`, no `twitter:title`, and the meta description is the generic site description rather than a calculator-specific one.

**Evidence**

```
Dev: document.title = "AyoKoding"
Prod: <title>AyoKoding</title>
H1: "Salary Savings Calculator"
OG title: none
Meta description: "Bilingual educational platform for software engineering - helping the Indonesian tech community learn and grow"
```

**Suggested fix locus**

`apps/ayokoding-www/src/app/[locale]/tools/cost-of-living-calculator/page.tsx` — add a `generateMetadata` that returns a locale-specific `title` (`"Salary Savings Calculator"` / `"Kalkulator Tabungan Gaji"`), a calculator-specific `description`, and matching `openGraph` fields so the title template fires.

---

## EWT-010 — Footer links below WCAG 2.5.8 minimum touch target at mobile viewports

- **Severity**: Minor
- **Priority**: Low
- **Area**: Accessibility / Footer
- **Defect type**: Accessibility
- **Reproducibility**: Always

**Environment**

URL: `http://localhost:3101/en/tools/cost-of-living-calculator`
Browser: Chromium (Playwright), viewport 375 × 812 (mobile)

**Steps to reproduce**

1. Open the page at 375 px viewport.
2. Scroll to the footer.
3. Measure the rendered height of the "FSL-1.1-MIT" and "Source-Available Project" links.

**Expected result**

WCAG 2.5.8 (Level AA, Target Size Minimum): interactive targets must be ≥ 24 × 24 CSS px, or have adjacent offset spacing providing equivalent effective target area.

**Actual result**

- `FSL-1.1-MIT` link: height `17 px`, width `77 px`, no padding or margin. Parent element height `20 px`. Effective area `17 px` — below the 24 px threshold with no offset spacing to compensate.
- `Source-Available Project` link: height `20 px`, width `158 px`, no padding or margin. Effective area `20 px` — also below threshold.

Both links lack the CSS spacing that would qualify them under the "offset spacing" exception.

**Evidence**

```
FSL-1.1-MIT:          h=17, w=77,  parentH=20, padding="0px 0px"
Source-Available:     h=20, w=158, parentH=48, padding="0px 0px"
```

**Suggested fix locus**

Footer link component (rendered by `apps/ayokoding-www/src/features/app-shell/shell/footer.tsx` — exact line TBD): add `py-2` (adds `8 px` top+bottom padding → effective height ≥ 33 px) or wrap in an element with `min-h-6` to meet the 24 px threshold.

---

## EWT-011 — Nav header causes horizontal overflow (8 px) at 320 px viewport

- **Severity**: Minor
- **Priority**: Low
- **Area**: Responsive / Navigation
- **Defect type**: Responsive
- **Reproducibility**: Always at 320 px; absent at 375 px and above

**Environment**

URL: `http://localhost:3101/en/tools/cost-of-living-calculator`
Browser: Chromium (Playwright), viewport 320 × 568

**Steps to reproduce**

1. Open the page at 320 px viewport width (Galaxy S5/SE baseline).
2. Observe `document.body.scrollWidth` vs `document.body.clientWidth`.
3. Observe the "Toggle theme" button in the header.

**Expected result**

Per WCAG 1.4.10 (Reflow, Level AA): content must reflow at 320 px without horizontal scrolling. No element should extend beyond the viewport, requiring the user to scroll horizontally to access content.

**Actual result**

`document.body.scrollWidth = 328`, `document.body.clientWidth = 320` — 8 px horizontal overflow. The offending element is the "Toggle theme" button (id `radix-_R_1r6kndlb_`): `left: 292 px, width: 36 px → right: 328 px`. The nav header flex row `mx-auto flex h-16 max-w-screen-2xl items-center gap-4 px-4` pushes the theme button 8 px off-screen at 320 px.

**Evidence**

```
body.scrollWidth=328, body.clientWidth=320 (overflow: true) at 320px
Offending element: BUTTON id=radix-_R_1r6kndlb_ (Toggle theme)
  right=328, left=292, width=36
Parent: DIV.mx-auto.flex.h-16.max-w-screen-2xl.items-center.gap-4.px-4
Viewport 375px: overflow false (not affected)
```

**Suggested fix locus**

`apps/ayokoding-www/src/features/app-shell/shell/header.tsx` — the nav bar flex row at 320 px overflows. Fix: reduce `gap-4` to `gap-2` at the smallest breakpoint, or add `min-w-0` on flex children to allow shrinking, or hide the theme toggle at the smallest breakpoint and include it in the mobile menu.

---

## EWT-012 — Calculator page absent from `sitemap.xml`

- **Severity**: Minor
- **Priority**: Low
- **Area**: URL / IA Quality / SEO
- **Defect type**: Content
- **Reproducibility**: Always

**Environment**

Production: `https://www.ayokoding.com/sitemap.xml` fetched via `curl`.

**Steps to reproduce**

1. Fetch `https://www.ayokoding.com/sitemap.xml`.
2. Search for `cost-of-living-calculator` or `tools`.

**Expected result**

A canonical public tool page (available at `/en/tools/cost-of-living-calculator` and `/id/tools/cost-of-living-calculator`) should be included in `sitemap.xml` so search engines can discover and index it.

**Actual result**

Neither `/en/tools/cost-of-living-calculator` nor `/id/tools/cost-of-living-calculator` appears in the sitemap. The `apps/ayokoding-www/src/app/sitemap.ts` only iterates `contentService.getIndex()` (markdown-based content pages); tool pages under `/tools/` are not registered.

**Evidence**

```
curl https://www.ayokoding.com/sitemap.xml | grep "cost-of-living" → no output
sitemap.ts: only loops contentService.getIndex(); no tool URL entries
```

**Suggested fix locus**

`apps/ayokoding-www/src/app/sitemap.ts` — append static entries for the tool URLs after the content loop, or extend `contentService` to include tool routes. At minimum add both `/en/tools/cost-of-living-calculator` and `/id/tools/cost-of-living-calculator`.

---

## EWT-013 — Confidence flags absent from Cost-of-Living and Savings tabs

- **Severity**: Major
- **Priority**: Medium
- **Area**: Functional / Confidence Flagging / Cost-of-Living Tab / Savings Tab
- **Defect type**: Functional
- **Reproducibility**: Always

**Environment**

URL: `http://localhost:3101/en/tools/cost-of-living-calculator`
Browser: Chromium (Playwright), viewport 1440 x 900

**Steps to reproduce**

1. Open the Cost-of-Living tab.
2. Inspect the Healthcare (OOP), Childcare, and School columns for any city.
3. Check for a confidence-flag indicator (the `[data-testid="confidence-flag"]` span used in the Minimum-Role tab, or equivalent).
4. Repeat on the Savings tab (Essentials column).

**Expected result**

Per spec `cost-of-living-calculator.feature > Scenario: Low-confidence cells are flagged`: "any cell backed by a lower-confidence estimate shows a confidence flag." The data in `cities.ts` marks many cells as `"moderate"` confidence (e.g. Jakarta healthcare `500,000 IDR`, Bangkok childcare `15,000 THB`, Singapore lifestyle `250 SGD`, Dubai school-public `1,000 AED` as `"proxy"`). The Min-Role tab correctly shows `[proxy]` via `data-testid="confidence-flag"` for lower-confidence best-city cells. The same mechanism should apply to Cost-of-Living and Savings cell values.

**Actual result**

Neither the Cost-of-Living tab nor the Savings tab renders any confidence flag. `cost-of-living.tsx` has zero references to `confidence`. `savings.tsx` also has zero references. Only `min-role.tsx` (lines 135–137) implements the confidence flag. All `"moderate"`- and `"proxy"`-confidence values in the two larger tables are displayed without any qualifier.

**Evidence**

```
cost-of-living.tsx: grep "confidence" → 0 matches
savings.tsx:        grep "confidence" → 0 matches
min-role.tsx:       confidence flag at lines 135-137 (works correctly)
Playwright: data-testid="confidence-flag" count on cost tab = 0
            data-testid="confidence-flag" count on savings tab = 0
cities.ts: 268 matches for "moderate"/"proxy"; many in healthcare, childcare, school, lifestyle columns
```

**Suggested fix locus**

- `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/cost-of-living.tsx` — add per-cell confidence from `city.expenses.<category>.confidence` and render a `<span data-testid="confidence-flag">` for `moderate` and `proxy` values, following the pattern in `min-role.tsx` line 135.
- `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/savings.tsx` — same for the Essentials cell where the constituting expense values include lower-confidence data.
