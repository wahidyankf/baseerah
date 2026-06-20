# Findings — ayokoding-www Cost-of-Living Calculator (Three-Tester Pass)

Combined findings from the [web-ux-test-fixing-planning](../../../repo-governance/workflows/web/web-ux-test-fixing-planning.md)
workflow run on **2026-06-20** against the running local dev server.

- Target: `http://localhost:3101/{en,id}/tools/cost-of-living-calculator`
- Locales: en, id (both)
- Breakpoints: 320, 375, 768, 1024, 1280, 1440 px
- Sources stay attributed: **Exploratory** (`EWT-###`, spec-aware), **Usability** (`UWT-###`, spec-blind),
  **Design** (`DWT-###`, design-aware). A reader must always be able to tell which lens raised a finding.

> Snapshot of the site as tested. Re-run all three testers if the site changes materially before this plan is executed.

## Exploratory findings (EWT-###)

Source: `web-exploratory-tester` (spec-aware). Compared live behaviour against
`specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/cost-of-living-calculator.feature`
and the source under `apps/ayokoding-www/src/features/cost-of-living-calculator/`.

**Count: 5** (3 Major, 2 Minor).

### EWT-001 — Mobile cost-of-living card header omits country name

- **Severity:** Major | **Type:** Functional / Consistency (spec violation)
- **Area:** Cost-of-living tab — mobile city cards (`[data-testid="mobile-city-cards"]`)
- **Environment:** Both locales, viewports 320 px and 375 px
- **Steps to reproduce:**
  1. Open `http://localhost:3101/en/tools/cost-of-living-calculator` at 375 px.
  2. Default tab "Cost of living"; wait for cards to render.
  3. Observe the first card header (Bangkok — city name ≠ country name).
- **Expected:** Per spec (`Scenario: Mobile city cards show the country name alongside the city`): "each card header shows both the city name and its country name."
- **Actual:** Header shows `"Bangkokmandatory payroll insurance"` — city name + healthcare badge only; "Thailand" absent.
- **Fix locus:** `cost-of-living-calculator/shell/cost-of-living.tsx` (~line 219) — add a country `<span>` (`r.country?.name[locale] ?? r.country?.name.en`), mirroring the savings-tab mobile card (`savings.tsx` ~line 202).

### EWT-002 — Desktop cost-of-living table uses English names on ID locale

- **Severity:** Major | **Type:** Functional / Localization
- **Area:** Cost-of-living tab — desktop table Country and City columns
- **Environment:** `/id/tools/cost-of-living-calculator`, viewports ≥768 px
- **Steps to reproduce:**
  1. Open `http://localhost:3101/id/tools/cost-of-living-calculator` at 1280 px.
  2. Default "Cost of living" tab; read the Country and City columns.
- **Expected:** Indonesian country names where translations exist (e.g. "Jepang" not "Japan", "Jerman" not "Germany", "Singapura" not "Singapore").
- **Actual:** Country shows "Japan", "Germany", "United Kingdom"; City shows "Tokyo", "Berlin", "London" — identical to the EN locale. Mobile cards correctly show "Singapura".
- **Root cause (hypothesis):** `cost-of-living.tsx` lines 128 & 131 hardcode `.name.en` instead of `name[locale] ?? name.en`.
- **Fix locus:** `cost-of-living.tsx` lines 128 and 131 — use the `localeName(name, locale)` helper (see `geo-filters.tsx`).

### EWT-003 — Desktop savings table uses English names on ID locale

- **Severity:** Major | **Type:** Functional / Localization
- **Area:** Savings tab — desktop table Country and City columns
- **Environment:** `/id/tools/cost-of-living-calculator?tab=savings`, viewports ≥768 px
- **Steps to reproduce:**
  1. Open the ID page at 1280 px; click the "Tabungan" (Savings) tab.
  2. Read the Country and City columns.
- **Expected:** Indonesian names where translations exist, consistent with the ID promise and with mobile savings cards (which correctly show "Uni Emirat Arab", "Jepang").
- **Actual:** Country shows "India", "Japan", "Germany" (English). Mobile savings cards are correct.
- **Root cause (hypothesis):** `savings.tsx` lines 150 & 153 hardcode `.name.en`.
- **Fix locus:** `savings.tsx` lines 150 and 153 — use `localeName(name, locale)`.

### EWT-004 — City-detail section labels are aria-label only (not visible)

- **Severity:** Minor | **Type:** Accessibility / Content
- **Area:** City detail view (`?tab=cost&city=<id>`)
- **Environment:** Both locales, all breakpoints
- **Steps to reproduce:**
  1. Open `…?tab=cost&city=jakarta`.
  2. Look for a visible "Monthly expenses" / "Relocation costs" heading.
- **Expected:** The translation keys `sectionMonthlyExpenses` / `sectionRelocationCosts` imply visible section labels.
- **Actual:** Both are `aria-label` attributes on `<section>` elements — read by screen readers but invisible to sighted users; sections separated only by a border. All data is correct (label-visibility only).
- **Fix locus:** `cost-of-living-calculator/shell/city-detail.tsx` — add a visible heading per section, OR accept aria-label-only and update the spec.

### EWT-005 — Missing Strict-Transport-Security header

- **Severity:** Minor | **Type:** Security
- **Area:** HTTP security headers
- **Environment:** `http://localhost:3101` (local dev HTTP)
- **Steps to reproduce:** `curl -sS -D - -o /dev/null http://localhost:3101/en/tools/cost-of-living-calculator`.
- **Expected:** `Strict-Transport-Security` present (e.g. `max-age=31536000; includeSubDomains`).
- **Actual:** Absent. Present: CSP, `X-Content-Type-Options: nosniff`, `X-Frame-Options: DENY`, `Referrer-Policy: strict-origin-when-cross-origin`.
- **Note:** Expected on local HTTP (HSTS is HTTPS-only). Verify the Vercel/production config sets it before treating as a blocker.
- **Fix locus:** Vercel project headers or `next.config.ts` headers.

## Usability findings (UWT-###)

Source: `web-usability-tester` (spec-blind — deliberately ignored specs/source/mockups). Judged
first-time-user perception against Nielsen's 10 heuristics, cognitive walkthrough, information scent,
and UX laws.

**Count: 13** (4 sev-4, 4 sev-3, 4 sev-2, 1 sev-1). The walkthrough transcript is in `walkthrough.md`.

### UWT-001 — Page H1 "Salary Savings Calculator" ≠ browser tab title "Cost of Living Calculator"

- **Heuristic:** H4 (Consistency) | **Severity:** 4 (catastrophe)
- **Area:** Page identity / IA | **Locales:** en + id
- **Steps:** Open the en page; tab title reads "Cost of Living Calculator | AyoKoding"; H1 reads "Salary Savings Calculator". Two different names for the same tool.
- **Expected:** H1 and `<title>` agree on one name.
- **Actual:** H1 = "Salary Savings Calculator" / "Kalkulator Tabungan Gaji"; `<title>` = "Cost of Living Calculator …" (untranslated on both locales). Confirmed via curl.
- **Fix:** Align `<title>` and H1; translate the title for id.

### UWT-002 — Tab labels render as one merged run: "SavingsSee how much you'd save"

- **Heuristic:** H8 + H6 + H4 | **Severity:** 4 (catastrophe)
- **Area:** Tab navigation bar | **Locales:** en + id
- **Steps:** Open en at 1280 px; inactive tab text content = `"SavingsSee how much you'd save"` and `"Minimum roleFind the min role you need"` — no separator between name and sub-label.
- **Expected:** Tab name prominent; sub-label on its own line / after a separator (standard two-line tab pattern).
- **Actual:** Sub-label fused into the name (DOM-confirmed). Screen reader announces the run-on string.
- **Fix:** `display:flex; flex-direction:column` (or block) on the button inner wrapper; separate name from description.

### UWT-003 — Savings tab shows all-negative table before any salary is entered (no empty state)

- **Heuristic:** H1 + H5 + H9 | **Severity:** 4 (catastrophe — most damaging trust failure)
- **Area:** Savings tab empty/zero-salary state | **Locales:** en + id
- **Steps:** Open the page; click "Savings" without entering a salary. Table shows red negatives ("Savings after essentials: -578 (—)") for every city; Net = 0.
- **Expected:** Instructional empty state ("Enter your gross salary above …") or a disabled/greyed table until a salary is entered.
- **Actual:** Pre-populated red negatives indistinguishable from a real zero result.
- **Fix:** Add an empty-state component on the Savings panel; hide/disable rows until salary > 0.

### UWT-004 — Tools index pages (`/en/tools`, `/id/tools`) render raw i18n keys

- **Heuristic:** H9 + H2 | **Severity:** 4 (catastrophe)
- **Area:** `/{en,id}/tools` index (parent of the calculator in the IA) | **Locales:** en + id
- **Steps:** Navigate to `/en/tools`; H1 renders literal `toolsPageTitle`, link renders `toolsPageCalcLink`. Same on `/id/tools`.
- **Expected:** Readable localized heading + link label.
- **Actual:** Raw translation keys visible (curl-confirmed).
- **Fix:** Add the missing `toolsPageTitle` / `toolsPageCalcLink` entries in en and id locale files.

### UWT-005 — Area toggle (City center / Rural) gives no confirmation the data changed

- **Heuristic:** H1 | **Severity:** 3 (major)
- **Area:** Area toggle / Cost of Living tab
- **Steps:** Cost of Living tab; note Singapore Total 4,578; click "Rural". Button active-style changes; table silently re-renders (Housing 3,500→2,625) with no animation/highlight/notice.
- **Expected:** Brief table highlight/transition or an "updated" indicator confirming recalculation.
- **Fix:** Add a fade/row-highlight transition or an updating "Rural estimates" label on area change.

### UWT-006 — "Baseline source" label on Minimum Role tab is opaque jargon

- **Heuristic:** H2 + H10 | **Severity:** 3 (major)
- **Area:** Minimum Role tab — "Baseline source" dropdown (id: "Sumber baseline")
- **Steps:** Open Minimum Role tab; label "Baseline source" with value "Monthly savings target"; no tooltip/help explains what it controls.
- **Expected:** Plain-language label + one-line inline help.
- **Fix:** Rename to e.g. "How to set the savings target" and add a one-line explanation.

### UWT-007 — Minimum Role tab shows populated role table before any target is entered

- **Heuristic:** H1 + H5 | **Severity:** 3 (major) — same class as UWT-003
- **Area:** Minimum Role tab empty/zero-amount state
- **Steps:** Click Minimum Role without entering a target; full role table (CTO, SVP Eng…) appears computed against a blank target.
- **Expected:** Empty state until a target > 0, or an explicit "based on $0 target" label.
- **Fix:** Show an instructional empty state until the savings target has a positive value.

### UWT-008 — "Savings after essentials" column has an unexplained sort icon

- **Heuristic:** H6 + H2 | **Severity:** 2 (minor)
- **Area:** Savings tab column header
- **Steps:** Savings tab with a salary entered; the column header shows a double-arrow icon with no tooltip/aria-label.
- **Fix:** Add `aria-label`/`title` (e.g. "Sort by savings"); show a hover tooltip.

### UWT-009 — Comparison table shows bare numbers in differing local currencies without inline currency code

- **Heuristic:** H4 | **Severity:** 2 (minor)
- **Area:** Cost of Living comparison table
- **Steps:** Default view; Singapore row 4,578 (SGD) next to Indonesia 15,700,000 (IDR) — currency only inferable from the city name; a first-timer may misread Indonesia as "more expensive".
- **Expected:** Inline ISO currency code (SGD/IDR) on Total & Essentials, or a normalize-to-one-currency option.
- **Fix:** Add the currency code inline to Total/Essentials cells (as the summary chips already do).

### UWT-010 — Mobile 320 px: household controls row wraps so labels detach from their dropdowns

- **Heuristic:** H4 + Fitts | **Severity:** 3 (major)
- **Area:** Household controls row at 320 px | **Locales:** en + id
- **Steps:** Open at 320 px; "Preschool children" label detaches from its "0" dropdown; the dropdown sits next to the "School-age children" label → false visual association.
- **Expected:** Each control stacks full-width (label above its own select) below ~360 px.
- **Fix:** `flex-wrap: wrap` with each item `width:100%` under ~360 px.

### UWT-011 — Mobile nav drawer shows only "English Content" (English label even on id locale)

- **Heuristic:** H3 + H6 (+ H4 locale consistency) | **Severity:** 2 (minor)
- **Area:** Mobile nav drawer | **Locales:** en + id (id shows the English string "English Content")
- **Steps:** Open at 375 px; tap hamburger; drawer shows a single "English Content >" item — no site nav links; on id locale the label is still English.
- **Expected:** Mobile drawer mirrors desktop nav (WCAG 3.2.3 Consistent Navigation); label localized.
- **Fix:** Populate the drawer with the top-level nav; localize the label.

### UWT-012 — `/EN/tools/…` (uppercase locale) returns 404 with no redirect to `/en/`

- **Heuristic:** H4 (URLs as UI) | **Severity:** 2 (minor)
- **Area:** Locale routing
- **Steps:** Navigate to `/EN/tools/cost-of-living-calculator` → HTTP 404; no redirect to lowercase.
- **Expected:** 301/302 normalize the locale segment to lowercase.
- **Fix:** Next.js middleware redirect lowercasing the locale path segment.

### UWT-013 — Browser `<title>` not translated to Indonesian on the id locale

- **Heuristic:** H4 (locale consistency) | **Severity:** 1 (cosmetic)
- **Area:** `<title>` on id locale
- **Steps:** Open the id page; tab title = "Cost of Living Calculator …" while H1 = "Kalkulator Tabungan Gaji".
- **Expected:** Localized title ("Kalkulator Tabungan Gaji | AyoKoding").
- **Fix:** Route the localized tool name into the Next.js metadata title. (Shares root cause with UWT-001.)

## Design findings (DWT-###)

Source: `web-design-tester` (design-aware — RUNNING page vs design, never auditing component source).
Ground truth: committed mockups in
[`plans/done/2026-06-19__ayokoding-www-salary-savings-calculator/assets/`](../../done/2026-06-19__ayokoding-www-salary-savings-calculator/assets),
runtime tokens (`--color-primary: rgb(37,99,235)`), `libs/web-ui` primitives, and design best-practice.
The tool was renamed (route/title "Cost of Living Calculator") but several surfaces still carry the
original "Salary Savings Calculator" design lineage — the root of DWT-004.

**Count: 11** (2 Critical, 6 Major, 2 Minor, 1 Trivial). Evidence: `local-temp/dwt-cost-of-living/`.

### DWT-001 — Money cells show bare numbers, not dual-currency (local + USD) — CORE FEATURE GAP

- **Severity:** Critical | **Dimension:** Mockup fidelity / Typography | **Locales:** en + id | **BP:** ≥768
- **Ground truth:** Mockups `ui-cost-of-living-option-a-category-table.png` + `ui-savings-option-a-net-savings-table.png` and the PRD ("every monetary figure is always shown in BOTH the city's local currency AND USD … Neither currency is ever shown alone").
- **Steps:** Open en at 1280 px; inspect the Singapore row money cells (Total/Essentials/Housing/…).
- **Expected:** Each money cell shows local + USD (e.g. `SGD 3,500 / $2,250`), as the mockups show ("Rp 9.4M / $600").
- **Actual:** Bare numbers, no currency label and no USD pair (`3,500`, `4,578`). `fmtDualCurrency()` exists in `city-detail.tsx` but the table components use `fmtNum()`.
- **Fix locus:** `cost-of-living.tsx` + `savings.tsx` table cells — use a dual-currency formatter mirroring `fmtDualCurrency(local, localCcy, usd)`.
- **Relation:** The design dimension of UWT-009 (bare numbers, no inline currency). Both fixed by the same dual-currency render.

### DWT-002 — Mobile cost-of-living card header omits country name

- **Severity:** Major | **Dimension:** Mockup fidelity / Responsive | **Locales:** en + id | **BP:** 320/375
- **Ground truth:** Mobile mockup shows header "Jakarta, Indonesia" (both as links); PRD "each city renders as a stacked card headed 'City, Country'".
- **Actual:** Header shows only the city link + healthcare badge ("Singaporemandatory payroll insurance"); no country.
- **Fix locus:** `cost-of-living.tsx` mobile card header — add a country link beside the city link.
- **Relation:** Same root cause as **EWT-001** (fix once).

### DWT-003 — Savings-tab gross-salary input is unstyled (bare browser-default `<input>`)

- **Severity:** Major | **Dimension:** Design-system-primitive reuse / Token | **Locales:** en + id | **BP:** all
- **Ground truth:** Savings mockup shows a bordered, rounded input; PRD R5 "reuses `libs/web-ui`: `input`, `label`"; the `Input` primitive exists.
- **Actual:** Bare `<input type="number">` — `border:0; borderRadius:0; padding:0; height:24px; background:transparent`. Near-invisible as a field.
- **Fix locus:** `savings.tsx` ~93–104 — replace raw `<label>/<input>` with `<Label>` + `<Input>` from `@open-sharia-enterprise/web-ui`.

### DWT-004 — H1 "Salary Savings Calculator" mismatches title/tab "Cost of Living Calculator"

- **Severity:** Critical | **Dimension:** Mockup fidelity / Hierarchy | **Locales:** en + id | **BP:** all
- **Ground truth:** Mockup heading "Cost of Living Calculator"; route + `generateMetadata` title both "Cost of Living Calculator".
- **Actual:** H1 = "Salary Savings Calculator" / "Kalkulator Tabungan Gaji" (translation key `calcTitle`) — contradicts the page `<title>` and tab.
- **Fix locus:** `i18n/core/translations.ts` — `calcTitle` → "Cost of Living Calculator" (en) / "Kalkulator Biaya Hidup" (id).
- **Relation:** Same root cause as **UWT-001** (fix once). Also resolves **UWT-013** (id title) if the title is localized at the same time.

### DWT-005 — Tab labels fuse visible + sr-only text ("SavingsSee how much you'd save")

- **Severity:** Major | **Dimension:** Typography / Mockup fidelity | **Locales:** en + id | **BP:** all
- **Ground truth:** Mockup shows clean single-phrase tab labels.
- **Actual:** Tab `textContent` = "SavingsSee how much you'd save" — the `sr-only` span sits inside `<TabsTrigger>`, fusing into the accessible name.
- **Fix locus:** `calculator-content.tsx` ~122–134 — move the sr-only span out of the trigger; reference via `aria-describedby`.
- **Relation:** Same root cause as **UWT-002** (fix once).

### DWT-006 — Min-role "Baseline source" is an unstyled `<select>`, not the designed segmented control

- **Severity:** Major | **Dimension:** Mockup fidelity / Primitive reuse | **Locales:** en + id | **BP:** all
- **Ground truth:** Min-role mockup shows "My salary / Reference role / Savings target" as a segmented button group (matching the Area toggle); `SegmentedControl` already exists and is used for Area + School-type.
- **Actual:** Plain unstyled `<select id="baseline-source-select">` (`border:0; borderRadius:0`) — visually inconsistent with every other toggle.
- **Fix locus:** `min-role.tsx` ~163–175 — replace the `<select>` with `SegmentedControl` from `controls.tsx`.
- **Relation:** The design dimension of UWT-006 (the usability finding is the opaque label; pair the relabel with the control restyle).

### DWT-007 — Savings sort button in the table header has no visual styling (invisible control)

- **Severity:** Minor | **Dimension:** Hierarchy / Primitive reuse | **Locales:** en + id | **BP:** ≥768
- **Actual:** Sort `<button>` computes `background:transparent; border:0; cursor:default` — indistinguishable from static text; only the ↕ glyph hints at sortability.
- **Fix locus:** `savings.tsx` ~121–129 — wrap in `Button variant="ghost" size="sm"` (or at minimum `cursor-pointer underline`).
- **Relation:** Same control as **UWT-008** (no tooltip/affordance) — fix the affordance + the aria-label together.

### DWT-008 — id-locale cost-of-living desktop table shows English city/country names

- **Severity:** Major | **Dimension:** Cross-surface consistency | **Locales:** id | **BP:** ≥768
- **Actual:** Desktop cells use `.name.en` while id mobile cards correctly use `name[locale]` ("Singapura"). Same data renders differently across breakpoints of the same locale.
- **Fix locus:** `cost-of-living.tsx` ~128–132 — `name[locale] ?? name.en`.
- **Relation:** Same root cause as **EWT-002** (fix once).

### DWT-009 — id-locale Min-role best-city cells show English names

- **Severity:** Major | **Dimension:** Cross-surface consistency | **Locales:** id | **BP:** ≥768
- **Actual:** `RoleRow` uses `entry.bestCity.name.en` / `entry.bestCountry.name.en` → "Austin, United States" on id.
- **Fix locus:** `min-role.tsx` ~133–134 + mobile role cards — `name[locale] ?? name.en`.
- **Relation:** Same root cause family as **EWT-002/003**, **DWT-008** (one locale-name fix sweep across `cost-of-living.tsx`, `savings.tsx`, `min-role.tsx`).

### DWT-010 — id "Area" label "Wilayah tempat tinggal" is long, wrapping the segmented control at ≤375 px

- **Severity:** Minor | **Dimension:** Mockup fidelity / Density | **Locales:** id | **BP:** 320/375
- **Actual:** 22-char label forces "Pusat kota" button text to wrap to two lines at 375 px; mockup uses short "Area".
- **Fix locus:** `i18n/core/translations.ts` — shorten id `labelArea` (e.g. "Area"/"Lokasi") and/or `whitespace-nowrap`.

### DWT-011 — 320 px: "Preschool children" label detaches from its select (pair wraps mid-unit)

- **Severity:** Trivial | **Dimension:** Responsive / Density | **Locales:** en + id | **BP:** 320
- **Actual:** flex-wrap splits the label and its select onto different lines, breaking the visual label↔input association.
- **Fix locus:** `controls.tsx` household selector — wrap each label+select as a `flex items-center gap-1` unit so pairs wrap intact.
- **Relation:** Same surface as **UWT-010** (320 px household controls) — one responsive fix.

## Cross-reference note

Findings from different lenses that share a single root cause — fix once, verify across all three:

- **Locale-name leak (id desktop tables show English):** `EWT-002`, `EWT-003`, `DWT-008`, `DWT-009` — one `name[locale] ?? name.en` sweep across `cost-of-living.tsx`, `savings.tsx`, `min-role.tsx` (desktop + mobile) resolves all four.
- **Mobile cost card omits country:** `EWT-001` = `DWT-002`.
- **Tool-name / title mismatch:** `UWT-001` = `DWT-004`; localizing the title at the same time also closes `UWT-013`.
- **Fused tab labels:** `UWT-002` = `DWT-005`.
- **Bare numbers / no currency:** `UWT-009` is the usability dimension; `DWT-001` is the design-fidelity dimension (dual-currency) — the dual-currency render fixes both.
- **Savings sort control:** `UWT-008` (no tooltip/affordance) + `DWT-007` (invisible styling) — fix affordance + aria-label together.
- **Min-role baseline control:** `UWT-006` (opaque label) + `DWT-006` (unstyled select) — relabel + restyle as a segmented control together.
- **320 px household controls:** `UWT-010` + `DWT-011` — one responsive stacking fix.
