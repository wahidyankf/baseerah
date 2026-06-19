# Findings — Cost-of-Living Calculator Exploratory Testing

**Session date**: 2026-06-19
**Tester agent**: `exploratory-web-tester`
**Ground truth**: `specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/cost-of-living-calculator.feature`

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
