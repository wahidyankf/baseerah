# Findings — Cost-of-Living Calculator Test-Fixing

This file carries the combined findings from two complementary testing passes against the
ayokoding-www Cost-of-Living Calculator at
`http://localhost:3101/{en,id}/tools/cost-of-living-calculator`. The two sources are kept in
**separate, labelled sections** — exploratory (spec-aware, `EWT-###`) and usability (spec-blind,
`UWT-###`) — so a reader can always tell which lens produced a given finding. A
**cross-reference note** at the end flags where an exploratory and a usability finding describe the
same underlying defect so the shared root cause is fixed once.

**Tested**: 2026-06-20 · breakpoints 375 px / 768 px / 1280 px · locales en + id.

---

## Exploratory findings (EWT-###)

Source: `web-exploratory-tester` (spec-aware). Compared live behaviour against
`specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/cost-of-living-calculator.feature`.
Core math (tax bands, FX, OECD household scaling, savings) independently verified **correct**; the
findings below are defects around that correct core.

### EWT-001 — `html lang` hardcoded `"en"` on all routes (Indonesian pages mislabelled)

- **Severity**: Critical · **Category**: Accessibility (WCAG 3.1.1)
- **Where**: `/id/tools/cost-of-living-calculator` — all breakpoints — ID locale
- **Reproduce**: Navigate to the `/id/` route; read `document.documentElement.lang`.
- **Expected**: `lang="id"` (page primary language programmatically determinable).
- **Actual**: `lang="en"` — `apps/ayokoding-www/src/app/layout.tsx` hardcodes `<html lang="en">`;
  `src/app/[locale]/layout.tsx` never sets `lang`.
- **Fix locus**: `apps/ayokoding-www/src/app/layout.tsx` / `[locale]/layout.tsx` — make `lang`
  locale-aware.

### EWT-002 — City detail shows relocation costs in local currency only (USD missing; spec divergence)

- **Severity**: Major · **Category**: Functional (spec divergence)
- **Where**: `?tab=cost&city=london` — 1280 px — EN
- **Reproduce**: Open a city detail; read the "One-time relocation sunk cost" / "Liquidity reserve"
  rows.
- **Expected**: Spec requires "split relocation in both local currency **and USD**".
- **Actual**: London shows `GBP 7,100` only; Jakarta `IDR 31,000,000` only — no USD equivalent
  anywhere in the detail.
- **Fix locus**: `src/features/cost-of-living-calculator/shell/city-detail.tsx` — render
  `relocationSunkUsd` / `liquidityReserveUsd` from `calc.ts` alongside local-currency amounts.

### EWT-003 — GeoFilters dropdowns not hydrated from URL params on deep link

- **Severity**: Major · **Category**: Functional (spec divergence) / behavioural consistency
- **Where**: `?tab=cost&country=id` — 1280 px — EN
- **Reproduce**: Deep-link to `?tab=cost&country=id`; observe the filtered table vs the dropdowns.
- **Expected**: Region pre-selected to "ASEAN", Country to "Indonesia".
- **Actual**: Table filtered to Jakarta but both dropdowns read "All …" — `geo-filters.tsx`
  initialises from `useState(null)` and ignores the URL params.
- **Fix locus**: `src/features/cost-of-living-calculator/shell/geo-filters.tsx` — initialise state
  from the parent-decoded `initialCityId` / `initialCountryId` (already in `calculator-content.tsx`)
  or from search params.

### EWT-004 — City filter not pre-selected after clicking a city link in the table

- **Severity**: Major · **Category**: Functional (spec divergence)
- **Where**: `/en/tools/cost-of-living-calculator` — 1280 px — EN
- **Reproduce**: On the Cost-of-living tab, click a city name (e.g. "Singapore"); read the City
  dropdown.
- **Expected**: City filter pre-selected to that city.
- **Actual**: City dropdown stays "All cities"; the detail panel renders but `GeoFilters` state is
  not updated from `handleTableClick`.
- **Fix locus**: `src/app/[locale]/tools/cost-of-living-calculator/calculator-content.tsx` — lift the
  geo state or push the click-derived `cityId` into `GeoFilters`.

### EWT-005 — Salary input accepts negative values (nonsensical negative annual gross)

- **Severity**: Major · **Category**: Edge case / functional
- **Where**: `?tab=savings` — all breakpoints — EN + ID
- **Reproduce**: On the Savings tab, type `-5000` into the gross-monthly field.
- **Expected**: Clamp to 0 or show an inline validation error.
- **Actual**: Annual gross shows "−60,000 USD"; net goes negative
  (`parseFloat('-5000')` passes through as `-5000`).
- **Fix locus**: `src/features/cost-of-living-calculator/shell/savings.tsx` — add `min="0"` and clamp
  in `onChange`: `Math.max(0, parseFloat(e.target.value) || 0)`.

### EWT-006 — Per-category expense columns ignore household scaling (column sum ≠ Essentials)

- **Severity**: Major · **Category**: Functional / UI-UX
- **Where**: `/en/tools/cost-of-living-calculator` (Cost-of-living tab) — 1280 px — EN
- **Reproduce**: Set Adults to 2. Sum Singapore's visible per-category columns (3,500 + 400 + 128 +
  180 + 120 = 4,328) vs the Essentials column (5,508).
- **Expected**: Either the category columns apply household scaling so they sum to Essentials, OR the
  headers are labelled "per-person baseline".
- **Actual**: `cost-of-living.tsx` renders raw `e.housing.amount` for category columns but
  `essentialsLocal(...)` (household-adjusted) for Essentials — only the OECD-scaled categories
  diverge. Childcare/School columns DO scale, making the inconsistency partial.
- **Fix locus**: `src/features/cost-of-living-calculator/shell/cost-of-living.tsx` — apply
  `subLinear`/`perCapita` multipliers in the row mapping, matching `essentialsLocal`.

### EWT-007 — City detail panel shows raw (unadjusted) housing/food/utilities/healthcare (same root as EWT-006)

- **Severity**: Major · **Category**: Functional / UI-UX
- **Where**: `?tab=cost&city=singapore` — 1280 px — EN
- **Reproduce**: Set Adults to 2 in the city detail; Housing shows `SGD 3,500` but Essentials shows
  `SGD 5,508`; rows do not add up to the subtotal.
- **Expected**: Rows reflect household-adjusted amounts (4,375 / 600 / 225 / 180 SGD for 2 adults).
- **Actual**: `city-detail.tsx` uses raw `e.housing.amount` for rows but `essentialsLocal(...)` for
  the subtotal.
- **Fix locus**: `src/features/cost-of-living-calculator/shell/city-detail.tsx` — compute row amounts
  with the same `subLinear`/`perCapita` multipliers as `essentialsLocal`.

### EWT-008 — ID locale: country/city names in filter dropdowns always English

- **Severity**: Minor · **Category**: Localisation / behavioural consistency
- **Where**: `/id/tools/cost-of-living-calculator` — all breakpoints — ID locale
- **Reproduce**: On the `/id/` route, open the Country filter dropdown.
- **Expected**: Indonesian names (e.g. "Inggris", "Jerman") via `c.name.id`.
- **Actual**: English names — `geo-filters.tsx` uses `c.name.en` for all `<option>` labels (countries
  and cities) regardless of locale.
- **Fix locus**: `src/features/cost-of-living-calculator/shell/geo-filters.tsx` — use
  `c.name[locale] ?? c.name.en`.

### EWT-009 — ID locale: relocation column header "Relokasi (sunk)" (English "sunk" untranslated)

- **Severity**: Minor · **Category**: Localisation
- **Where**: `/id/tools/cost-of-living-calculator` — 1280 px — ID locale
- **Expected**: Fully Indonesian header (e.g. "Relokasi (biaya hangus)").
- **Actual**: `translations.ts` `id.colRelocationSunk = "Relokasi (sunk)"`.
- **Fix locus**: `src/features/i18n/core/translations.ts` — update the `id` entry.

### EWT-010 — ID locale: skip-to-content link hardcoded English

- **Severity**: Minor · **Category**: Localisation / accessibility
- **Where**: `/id/tools/cost-of-living-calculator` — all breakpoints — ID locale
- **Expected**: "Langsung ke konten" (the `skipToContent` key exists in the ID locale).
- **Actual**: "Skip to content" — `[locale]/layout.tsx` hardcodes the English string instead of
  `t(locale, "skipToContent")`.
- **Fix locus**: `src/app/[locale]/layout.tsx`.

### EWT-011 — ID locale: "Clear region" button `aria-label` hardcoded English

- **Severity**: Minor · **Category**: Localisation / accessibility
- **Where**: `/id/tools/cost-of-living-calculator` — all breakpoints — ID locale
- **Expected**: Indonesian `aria-label` (e.g. "Hapus wilayah").
- **Actual**: `geo-filters.tsx` hardcodes `aria-label="Clear region"` even though the visible text
  correctly calls `t(locale, "clearRegion")`.
- **Fix locus**: `src/features/cost-of-living-calculator/shell/geo-filters.tsx`.

### EWT-012 — Savings sort button does not expose sort state to assistive tech

- **Severity**: Minor · **Category**: Accessibility (WCAG 4.1.2)
- **Where**: `?tab=savings` — all breakpoints — EN + ID
- **Expected**: `aria-pressed` (or `aria-sort` on the header) reflecting `sortAsc`.
- **Actual**: `aria-pressed` absent; `sortAsc` not exposed to the a11y tree.
- **Fix locus**: `src/features/cost-of-living-calculator/shell/savings.tsx` — add
  `aria-pressed={sortAsc}`.

### EWT-013 — Security response headers missing; `X-Powered-By` discloses framework

- **Severity**: Minor · **Category**: Security (passive observation)
- **Where**: all URLs (dev server)
- **Expected**: `Content-Security-Policy`, `X-Content-Type-Options: nosniff`, `X-Frame-Options`/
  CSP `frame-ancestors`, `Referrer-Policy`; `X-Powered-By` removed.
- **Actual**: `X-Powered-By: Next.js` present; all four security headers absent.
- **Note**: Dev-server observation — verify the production Vercel/`next.config.js` header config is
  in place.
- **Fix locus**: `apps/ayokoding-www/next.config.js` — add a `headers()` block.

### EWT-014 — Savings sort control unreachable/invisible on mobile (375 px)

- **Severity**: Minor · **Category**: Responsive / accessibility
- **Where**: `?tab=savings` — 375 px — EN + ID
- **Reproduce**: Open Savings at 375 px — mobile cards render; the sort `<button>` lives inside the
  `hidden … md:block` desktop table (visually hidden but still keyboard-focusable).
- **Expected**: A visible, tappable sort control in the mobile card layout.
- **Actual**: No visible mobile sort control; the hidden desktop button remains in the tab order.
- **Fix locus**: `src/features/cost-of-living-calculator/shell/savings.tsx` — add a visible mobile
  sort toggle or hoist the control above both views.

### EWT-015 — `confidence-flag` spec scenario not implemented in the live DOM

- **Severity**: Minor · **Category**: Functional (spec divergence)
- **Where**: Minimum-role tab — 1280 px — EN
- **Reproduce**: Inspect the DOM for `[data-testid="confidence-flag"]` on low-confidence cells.
- **Expected**: The feature spec scenario "Low-confidence cells flagged" implies a confidence-flag
  affordance.
- **Actual**: No `[data-testid="confidence-flag"]` elements found; the scenario exists in the spec
  but is not implemented on the page. (Surfaced in the coverage map; promote to a tracked finding so
  the gap is either implemented or the scenario retired.)
- **Fix locus**: decide per Phase 3 grill — implement the flag affordance or retire/adjust the
  scenario.

### Exploratory spec coverage map (summary)

Most spec scenarios are **covered + passing** (core math, FX, OECD scaling, region/country/city
narrowing, healthcare scheme, deficit display, min-role ranking, no-Israeli-cities). The
**diverging** scenarios map to findings above:

- "Country and city always shown together" → EWT-004 (+ SG-005 mobile-card country)
- "Clicking city name opens detail" → EWT-002 (no dual-currency relocation) + EWT-004 (filter not
  pre-selected)
- "Clicking country filters cost-of-living" → EWT-003 (controls not hydrated)
- "Indonesian locale fully translated" → EWT-001, EWT-008, EWT-009, EWT-010, EWT-011
- "Low-confidence cells flagged" → EWT-015 (not implemented)

Not exercised (time-bounded, no defect implied): min-role reference-role baseline path, my-salary
baseline path, min-role display-currency selector, household-composition effect on min role.

---

## Usability findings (UWT-###)

Source: `web-usability-tester` (spec-blind — did not read `specs/**`, source, or mockups). Judges
only first-time-user perception against Nielsen's 10 heuristics (0–4 severity), cognitive walkthrough,
information scent, WCAG Understandable, and UX laws. Severity uses the Nielsen 0–4 scale.

### UWT-001 — "Savings" and "Minimum role" tabs appear to produce no content change (perceived non-functional tabs)

- **Severity**: 4 (catastrophe) · **Heuristic**: H1 visibility of system status, H4 consistency
- **Where**: `/en/tools/cost-of-living-calculator` — 1280 px — EN
- **Reproduce**: Click "Savings" or "Minimum role"; the tab button activates but the spec-blind
  observation saw the "Cost of living" panel still rendered (`data-state: active`) and the alternative
  panels empty/`hidden`.
- **Friction**: A first-time user who clicks "Savings" sees no change and concludes the tab is broken.
- **Remediation**: Ensure the panel content swaps on tab activation; if a tab is a placeholder,
  disable it with a "Coming soon" affordance.
- **⚠ Cross-source conflict** — see the cross-reference note: the **exploratory** tester actively used
  these tabs (entered a salary on Savings, exercised the sort button per EWT-012/EWT-014, tested the
  Minimum-role divider per SG-006), which contradicts a fully non-functional reading. This is likely
  an observation artifact (lazy panel mount / Radix transition timing in the spec-blind snapshot) OR
  an intermittent defect. **Resolve during Phase 3 grill / re-verification before any fix lands** —
  do not implement a tab rewrite on the strength of UWT-001 alone.

### UWT-002 — H1 "Salary Savings Calculator" conflicts with URL slug `cost-of-living-calculator`

- **Severity**: 3 (major) · **Heuristic**: H4 consistency, URL naturalness, H6 recognition
- **Where**: all breakpoints, both locales
- **Reproduce**: H1 reads "Salary Savings Calculator"; URL is `…/cost-of-living-calculator`;
  `<title>` is "AyoKoding" — three names for one tool.
- **Friction**: A search-engine arrival pauses to check they are on the right page.
- **Remediation**: Pick one canonical name; align H1, URL slug, and `<title>`.

### UWT-003 — Filter state not reflected in the URL (no bookmarkable/shareable view)

- **Severity**: 3 (major) · **Heuristic**: H6 recognition, H3 user control & freedom
- **Where**: 1280 px — EN
- **Reproduce**: Select Region/Country/City; the URL never changes. Copy-paste into a new tab → all
  filters reset.
- **Friction**: A comparison view cannot be shared or bookmarked; state is lost on navigation.
- **Remediation**: Persist filter state in URL query params and hydrate from them on load.
  (Shares root cause with exploratory EWT-003 — the dropdowns also fail to hydrate _from_ the URL.)

### UWT-004 — Comparison table overflows the desktop viewport; summary columns hidden with no scroll affordance

- **Severity**: 3 (major) · **Heuristic**: H6 recognition, H8 minimalist design
- **Where**: 1280 px — EN + ID
- **Reproduce**: At 1280 px the table is ~1,564 px in a ~1,120 px container; the right edge ends near
  "Healthcare (OOP)" — "Essentials", "Total", "Relocation (sunk)", "Liquidity reserve" are clipped with
  no fade/shadow/"scroll for more" hint.
- **Friction**: The "Total" — the answer users came for — is off-screen and discovered only by
  accidental horizontal scroll.
- **Remediation**: Pin/sticky the Total & Essentials columns, reorder summary columns leftward, or add
  a right-edge scroll affordance; consider a "summary only" toggle.

### UWT-005 — "Relocation (sunk)" and "Liquidity reserve" columns carry no definition (jargon without context)

- **Severity**: 3 (major) · **Heuristic**: H2 match real world, H10 help & documentation
- **Where**: 1280 px — EN + ID
- **Reproduce**: No tooltip / `abbr` / footnote on either header (only "OOP" has a footnote).
- **Friction**: Users can't tell if these are one-time or monthly, or what they include.
- **Remediation**: Add header tooltips / footnotes defining each term and its one-time vs monthly
  nature. (Relates to exploratory EWT-002 — these same columns also lack the USD equivalent.)

### UWT-006 — Indonesian page sets `html lang="en"` (wrong document language signal)

- **Severity**: 3 (major) · **Heuristic**: WCAG 3.1.1, ISO 9241-110 self-descriptive, H4 consistency
- **Where**: `/id/…` — all breakpoints — ID locale
- **Reproduce**: `document.documentElement.lang === "en"` on the `/id/` route.
- **Friction**: Translation prompts and assistive tech misidentify the page language.
- **Remediation**: Set `<html lang={locale}>`. **Same root cause as exploratory EWT-001** — fix once.

### UWT-007 — Page `<title>` is always "AyoKoding" (no tool name / city / tab in tab & bookmarks)

- **Severity**: 2 (minor) · **Heuristic**: H1 visibility, H6 recognition
- **Where**: all pages, both locales
- **Remediation**: Set a descriptive `<title>` (tool name + site, optionally the active city).

### UWT-008 — Tab container `role="tablist"` association not confirmed for assistive tech

- **Severity**: 1 (cosmetic) · **Heuristic**: WCAG 3.2.4
- **Where**: both locales
- **Remediation**: Verify the container wrapping the three `role="tab"` buttons carries
  `role="tablist"` with its `aria-label` (Radix usually handles this — confirm rendered output).

### UWT-009 — Mobile interactive controls ~29 px tall (below WCAG 2.5.8 preferred 44 px)

- **Severity**: 2 (minor) · **Heuristic**: Fitts's Law, WCAG 2.5.8 (preferred)
- **Where**: < 768 px — EN + ID
- **Note**: Passes the 24 px hard minimum; misses the 44 px preferred target.
- **Remediation**: Raise control min-height to 44 px on `≤ md` (e.g. Tailwind `h-11`/`min-h-[44px]`).

### UWT-010 — ID-locale "Wilayah tempat tinggal" Area label wraps at 375 px, reflowing the City-center/Rural toggle

- **Severity**: 2 (minor) · **Heuristic**: H8 minimalist, ISO 9241-110 conformity with expectations
- **Where**: `/id/…` — 375 px (and 320 px) — ID locale
- **Remediation**: Shorten the Indonesian label (e.g. "Kawasan") or `whitespace-nowrap` the label.

### UWT-011 — "Healthcare scheme" badges shown in ALL CAPS with no explanation of the taxonomy

- **Severity**: 2 (minor) · **Heuristic**: H2 match real world, H8 minimalist
- **Where**: all breakpoints, both locales
- **Remediation**: Sentence-case the badges; add a header tooltip defining
  "mandatory payroll insurance" vs "tax-funded".

### UWT-012 — "Savings" / "Minimum role" tab labels have weak information scent

- **Severity**: 2 (minor) · **Heuristic**: H6 recognition, information scent
- **Where**: both locales
- **Remediation**: Rename to predictive phrases (e.g. "Savings after expenses",
  "Minimum salary needed") or add a subtitle. (Compounds UWT-001.)

### UWT-013 — Parent URL `/en/tools` returns 404 (URL IA not hackable)

- **Severity**: 2 (minor) · **Heuristic**: URL naturalness, H4 consistency
- **Where**: `/en/tools` — both locales
- **Reproduce**: Shorten the URL to `/en/tools` → HTTP 404.
- **Remediation**: Add a `/tools` index page (also fixes the orphaned-URL/sitemap gap).

### UWT-014 — "OOP" defined only in an interstitial footnote, easy to miss; no `abbr`/tooltip

- **Severity**: 1 (cosmetic) · **Heuristic**: H2 match real world, H6 recognition
- **Where**: 1280 px — EN
- **Remediation**: Wrap "OOP" in `<abbr title="out-of-pocket">` wherever it appears.

### Usability severity tally

Sev 4: 1 (UWT-001) · Sev 3: 5 (UWT-002,003,004,005,006) · Sev 2: 5 (UWT-007,009,010,011,012) ·
Sev 1: 2 (UWT-008,014). Top friction: perceived non-functional tabs (UWT-001, conflict-flagged) and
the H1/URL name mismatch (UWT-002).

Dimensions not covered by the usability pass: full keyboard-only sweep, colour-contrast ratios,
screen-reader announcement order, ARIA-tree completeness, perceived-latency measurement (the tool
updates instantly — no loading state observable). Walkthrough transcript: `./walkthrough.md`.

---

## Cross-reference note (shared root causes across the two sources)

A reader must always be able to tell an exploratory finding from a usability one — the sections above
stay separate. This note only flags where the **same underlying defect** was caught by both lenses, so
the fix lands once:

| Shared defect                             | Exploratory                                                       | Usability                                 | Fix-once note                                                                                                                                               |
| ----------------------------------------- | ----------------------------------------------------------------- | ----------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Indonesian `html lang="en"`               | EWT-001                                                           | UWT-006                                   | Single fix: `<html lang={locale}>`. Both close on it.                                                                                                       |
| URL ⇄ filter-state desync                 | EWT-003 (dropdowns don't hydrate _from_ URL)                      | UWT-003 (selections don't write _to_ URL) | Two halves of one feature — implement bidirectional URL ⇄ filter sync once.                                                                                 |
| Relocation/Liquidity columns under-served | EWT-002 (no USD equivalent)                                       | UWT-005 (no definition)                   | Same two columns — add USD **and** a definition tooltip in one pass.                                                                                        |
| `[Conflict] Savings/Min-role tabs`        | exploratory **used** both tabs successfully (EWT-012/014, SG-006) | UWT-001 saw them as non-functional        | **Do not act on UWT-001 alone.** Re-verify tab behaviour first (Phase 3 / Rule-15 retest). Likely a spec-blind observation artifact; possibly intermittent. |

All other EWT-### and UWT-### findings are independent and must each be addressed on their own.
