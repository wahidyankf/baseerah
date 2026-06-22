# Findings — ayokoding-www cost-of-living calculator UX hardening

> Consolidated findings from one `web-ux-test-fixing-planning` run (2026-06-22) against the live local
> dev server. Three source lenses are kept in separate, labelled sections: spec-aware **Exploratory
> (EWT)**, spec-blind **Usability (UWT)**, and design-aware **Design (DWT)**. IDs are preserved from
> each tester.

**Target URLs**

- `http://localhost:3101/en/tools/cost-of-living-calculator`
- `http://localhost:3101/id/tools/cost-of-living-calculator`
- City-detail deep links (e.g. `?city=tokyo`, `?city=singapore&schoolkids=1&schooltype=public`)

**Locales**: en, id (both). **Breakpoints**: 320, 375, 768, 1024, 1280, 1440.

---

## Exploratory findings (EWT-###)

Source: `web-exploratory-tester` (spec-aware). Compared live behaviour against
`specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/cost-of-living-calculator.feature`.

**Severity counts**: Major 2 · Minor 4 · Trivial 0. **Spec gaps**: 0.

### EWT-001 — All three tab descriptions render visible simultaneously (malformed CSS class)

- **Severity**: Major · **Priority**: High · **Area**: Calculator tab bar · **Type**: Functional / UI
- **Environment**: en+id, all breakpoints
- **Steps**: Open `/en/tools/cost-of-living-calculator` (Cost tab active) → observe area below the tab bar → all three tab descriptions are visible at once.
- **Expected**: Only the active tab's description is visible; inactive descriptions hidden (spec USS-004 / "no tab description text duplicated elsewhere").
- **Actual**: `data-testid="tab-desc-savings"` and `tab-desc-min-role` carry the class string `"mt-1 text-sm text-muted-foregroundhidden"` — `text-muted-foregroundhidden` is not a valid Tailwind class. A space is missing before `hidden`, so the `hidden` utility never applies and inactive descriptions never hide. All three remain visible (`height: 20` each) regardless of active tab.
- **Root cause**: className template literal missing a space before `"hidden"` in the tab-description assembly (shell tab/content component).

### EWT-002 — Tab triggers (29px) and segmented radio controls (28px) below 44px touch target

- **Severity**: Major · **Priority**: High · **Area**: Tab bar + Controls (school type, area, salary currency) · **Type**: Accessibility (WCAG 2.5.8)
- **Environment**: en+id, 375px
- **Measured**: tab triggers 29px; school-type / area / salary-currency radios 28px. (Baseline-source buttons 48px OK; geo selects 44px OK; mobile sort button 44px OK.)
- **Expected**: interactive targets ≥ 44px preferred (spec AC-4 confirms 44px for geo selects and implicitly all interactive controls).
- **Root cause**: segmented radio groups use an `h-9` (36px) container whose buttons render 28–29px after padding; add `min-h-[44px]` to each radio button + tab trigger. `controls.tsx` + shared tablist.

### EWT-003 — `school-foreigner-flag-<cityId>` testid absent from city-detail view

- **Severity**: Minor · **Priority**: Medium · **Area**: City detail · **Type**: Functional
- **Environment**: `?city=singapore&schoolkids=1&schooltype=public`, en+id
- **Expected**: when `country.foreignerPublicSchool.access !== "open"` and `schoolKids>0` and `schoolType==="public"`, the city detail renders `data-testid="school-foreigner-flag-<cityId>"` (matching the cost-of-living table rows).
- **Actual**: cost-of-living table renders the flag for all 8 non-open cities ✓; city detail amends the school label to "School (public n/a → private)" and renders `foreigner-public-school-note` ✓ **but** `school-foreigner-flag-<cityId>` is absent (grep 0 matches) in city detail.
- **Root cause**: `shell/city-detail.tsx` school row should add `data-testid={\`school-foreigner-flag-${cityId}\`}`when`schoolForeignerFallback` is true.

### EWT-004 — OOP `<abbr>` title hardcoded to English "out-of-pocket" in id locale

- **Severity**: Minor · **Priority**: Low · **Area**: Healthcare (OOP) column header; id locale · **Type**: Content / i18n
- **Actual**: `Kesehatan (<abbr title="out-of-pocket">OOP</abbr>)` — `title` is a literal English string in `cost-of-living.tsx` (~line 131), while the OOP legend below the table IS translated. Inconsistent within id locale.
- **Root cause**: change `title="out-of-pocket"` to `title={t(locale, "healthcareOutOfPocket")}` (id translation `"bayar sendiri"` already exists).

### EWT-005 — Sort column `<th>` lacks `aria-sort` attribute

- **Severity**: Minor · **Priority**: Low · **Area**: Savings tab table · **Type**: Accessibility
- **Actual**: the "Savings after essentials ↕" `<th>` contains a `<button aria-pressed=… aria-label="Sort by savings">` but the `<th>` itself has no `aria-sort`. Screen readers traversing the header row get no sortable-column signal.
- **Root cause**: `shell/savings.tsx` — add `aria-sort={sortActive ? (sortAsc ? "ascending" : "descending") : "none"}` to the `<th>`.

### EWT-006 — Google Analytics script blocked by own CSP (console error every load)

- **Severity**: Minor · **Priority**: Low · **Area**: Security / CSP · **Type**: Functional (CSP misconfig)
- **Actual**: two console errors per load — `Loading the script 'https://www.googletagmanager.com/gtag/js?id=G-1NHDR7S3GV' violates … script-src 'self' 'unsafe-inline' 'unsafe-eval'`. The GA `<script>` is injected but the CSP does not whitelist googletagmanager/google-analytics.
- **Root cause**: `apps/ayokoding-www/next.config.ts` — either add `https://www.googletagmanager.com https://www.google-analytics.com` to `script-src`/`connect-src`, or remove the GA tag if analytics are unused.

**Exploratory coverage map**: control×surface matrix (11 controls × 4 surfaces — all propagate, no silent no-ops); per-control URL round-trip (all 16 `PARAM_KEYS` round-trip; defaults omitted; `gross` debounced); declared-invariant conformance (18 invariants enumerated — 16 HOLD, 2 FAIL = EWT-001, EWT-003). Recurrence re-check: classes 1,3,4,5,9,11,12,13 CLEAR; class 6 overflow-clear/touch-targets→EWT-002; class 7→EWT-004; class 8→EWT-006; class 10→EWT-005. Changed surfaces A (PASS w/ EWT-003), B PASS, C partial→EWT-002, D PASS, E PASS. **Not covered**: cross-browser (Safari/Firefox), SR live-region announcements, Lighthouse CWV, full keyboard-only pass, color-contrast audit.

---

## Usability findings (UWT-###)

Source: `web-usability-tester` (spec-blind — judged first-time-user perception only, no specs/source/mockups
read). Heuristics 0–4 severity. **Counts**: sev-3 × 2 · sev-2 × 10 · sev-1 × 2.

> **Cross-reference note (shared root causes — fix once):**
>
> - **UWT-014** (OOP unexplained inline) shares root with **EWT-004** (OOP `<abbr>` title hardcoded
>   English in id) — both fixed by glossing the OOP column header via i18n `title`.
> - **UWT-008 / UWT-011** (Area toggle + disabled school-type buttons lack ARIA state/description) sit
>   alongside **EWT-002** (segmented controls) and **EWT-005** (sort `aria-sort`) — the calculator's
>   segmented-control + sort a11y wiring is the shared cluster.
> - **UWT-002** (in-cell "public n/a → private" badge cryptic) is the usability read of the same
>   foreigner-school surface as **EWT-003** (missing city-detail flag testid) — fix the flag's clarity
>   and parity together.

### UWT-001 — "Baseline source" label carries zero information scent (sev-3, High)

- Heuristic 2/6 · Min-role segment-group label · en+id. "Baseline source" is internal vocabulary; a
  first-timer cannot predict what the three buttons (Monthly savings target / Match a role / My salary)
  do. No tooltip/gloss. **Fix**: relabel to "How to set your target" (or "Target method") both locales.

### UWT-002 — In-cell "public n/a → private" badge is cryptic shorthand (sev-3, High)

- Heuristic 2/6/9 · Cost-of-living School column (foreigner-flag rows) · en+id (same raw string both
  locales). The badge compresses "public school unavailable to foreigners → private rate substituted"
  into 4 tokens; the explanatory note sits spatially far above the table, so scanners hit the badge
  first. **Fix**: render "Private (public unavailable)" + footnote anchor, or add a `title`/tooltip on
  the cell repeating the existing note. (Shared surface w/ EWT-003.)

### UWT-003 — "Non-salary comp (info, annual, RSU/equity)" header is a 42-char jargon string (sev-2, Med)

- Heuristic 2/8 · Min-role last column header · en. Packs subject + "(info" + cadence + "RSU/equity"
  into 192px; "RSU" is stock jargon, "(info" ambiguous. **Fix**: shorten to "Non-salary comp" + `title`
  expansion (explanation already exists in the disclaimer below).

### UWT-004 — Region filter options are English-only in the id locale (sev-2, Med)

- Heuristic 2/4, WCAG 3.1.2 · Region dropdown all tabs · id (and en for unexpanded "MENA"/"Nordics").
  Options stay English ("Africa", "MENA", "Nordics") under an Indonesian UI. **Fix**: translate region
  names in id; expand "MENA"/"Nordics" in both locales.

### UWT-005 — Savings-tab pre-salary empty state lacks prominence (sev-2, Med)

- Heuristic 1/5, Progressive Disclosure · Savings tab · en+id. After clicking Savings the data area is
  blank but for one sentence; fast scanners may think load failed (Cost tab showed a table instantly).
  **Fix**: prominent empty-state panel in the data area, or auto-focus the salary input on tab activate.
  (Pairs USS-001.)

### UWT-006 — Min-role shows a pre-populated Singapore cost panel before any target (sev-2, Med)

- Heuristic 1/6, Progressive Disclosure · Min-role default state · en+id. A real-looking Singapore
  essentials breakdown appears before input, reading as a result. **Fix**: label it "Example (Singapore)"
  or hide until a target is entered. (Pairs USS-002.)

### UWT-007 — Salary currency not shown at the field on the Savings tab (sev-2, Med)

- WCAG 3.3.2, Heuristic 5 · Savings gross input · en+id. USD constraint is only a block note below the
  label; Min-role "My salary" mode pairs a currency selector at the field — inconsistent pattern.
  **Fix**: inline "USD" prefix/suffix in the input row (or a currency selector) to unify with My-salary.

### UWT-008 — Area toggle lacks `aria-pressed`; active state is colour-only (sev-2, Med)

- Heuristic 1, WCAG 1.3.3/3.2.2 · Area toggle all tabs · en+id. Both buttons report `aria-pressed:null`;
  active state distinguished by `bg-primary` colour alone. **Fix**: `aria-pressed` (or `role=radio`) +
  a non-colour active indicator. (Pairs USS-003; same a11y cluster as EWT-002.)

### UWT-009 — "Relocation (sunk)" / "Liquidity reserve" headers have no inline gloss (sev-2, Low)

- Heuristic 2 · Cost-of-living headers · en+id. Finance jargon; definitions exist only in the far-below
  disclaimer. **Fix**: `title` tooltips on the two `<th>` reproducing the existing one-line definitions.

### UWT-010 — "P25 (monthly)/Median/P75" use percentile jargon unexplained (sev-2, Med)

- Heuristic 2 · Min-role salary-distribution headers · en+id. "P25"/"P75" read as plan codes to
  non-technical users; values are large $ with no "salary" label. **Fix**: `title` expansions ("25th
  percentile monthly salary…") or expand the header text.

### UWT-011 — Disabled school-type buttons lack `aria-describedby` to the prerequisite (sev-2, Med)

- WCAG 1.3.1, Heuristic 5 · School-type control disabled state · en+id. Buttons report
  `aria-describedby:null`, `aria-disabled:null`; AT users get no reason for the disabled state.
  **Fix**: `aria-describedby="school-type-hint"` + `id` on the helper text + `aria-disabled="true"`.
  (Pairs USS-004.)

### UWT-012 — Healthcare-scheme badges mix ALL-CAPS vs lower-case (sev-1, Low)

- Heuristic 4/8 · Healthcare scheme column · en. "MANDATORY PAYROLL INSURANCE" vs "tax-funded" reads as
  an alert. **Fix**: normalise to sentence-case for all scheme classifications.

### UWT-013 — "ic" / "mgmt" Track abbreviations unexpanded (sev-2, Med)

- Heuristic 2 · Min-role Track column · en. Career-ladder shorthand with no legend/tooltip. **Fix**:
  full words ("Individual contributor"/"Management") or `<abbr title>`.

### UWT-014 — "OOP" in "Healthcare (OOP)" header unexplained inline (sev-1, Low)

- Heuristic 2 · Cost-of-living Healthcare header · en. Definition exists in a note above the table but a
  scanner jumping to headers misses it. **Fix**: `title` on the `<th>`. (Shared root w/ EWT-004 — gloss
  via i18n.)

**Usability coverage**: all 10 heuristics; 4 cognitive-walkthrough tasks (3 en, 1 id); control×predictability
matrix (23 controls — all predictable except the jargon-labelled ones above); edge-state matrix per tab;
responsive 320–1440 (no breakage, no label detachment); URL naturalness (all patterns clean, `/en/tools`
200, `/EN/` 308-redirects). Recurrence: tab scent / identity / tab-label-fusion / mobile-detachment / URL-IA
all FIXED; jargon classes NOT fixed (UWT-001/003/009/010/013/014); empty-states partially addressed
(UWT-005/006); Area-toggle feedback partial (UWT-008). Changed surfaces: D (scroll) + E (debounce) PASS
clean; A→UWT-002; B→UWT-003; C→UWT-007. **Not covered**: zero-result empty state (no non-destructive path),
throttled-network loading, deep city-detail drawer (renders inline).

---

## Design findings (DWT-###)

Source: `web-design-tester` (design-aware — judged the running rendered page against committed mockups,
runtime tokens, `libs/web-ui` primitives, app shell, and design best practice). **Counts**: Major 1 ·
Minor 5. **3 SG design-spec proposals.** Every fix changes a user-facing component → **the plan is
UI-bearing**.

> **Cross-reference note (shared root causes):**
>
> - **DWT-001 ≡ EWT-001** — the exact same fused-className bug (`text-muted-foregroundhidden`); two
>   testers independently confirmed it. Fix once. (DWT-005 was the design tester's secondary note on the
>   same bug — folded into DWT-001.)
> - **DWT-006 pairs UWT-002** — the foreigner-school flag: usability says the wording is cryptic; design
>   says it lacks visual hierarchy (same `text-muted-foreground` as captions). Fix the flag's clarity +
>   styling together (and add the missing city-detail testid per EWT-003).
> - **DWT-007 relates to EWT-002 / UWT-008** — the segmented-control field-row alignment + a11y cluster.

### DWT-001 — All three tab descriptions render visible simultaneously (fused className) (Major, High)

- Ground truth: visual-hierarchy best practice · all tabs/locales/breakpoints. Inactive descriptions
  carry `mt-1 text-sm text-muted-foregroundhidden` — `hidden` fused onto `text-muted-foreground` (no
  space), so the utility never applies and all three descriptions stack visibly. **Root cause**:
  `shell/calculator-content.tsx` (~L261-280) template literal `…text-muted-foreground${active ? "" :
"hidden"}` missing a space before the conditional. One-character fix. **Same bug as EWT-001.**

### DWT-002 — Household selects use native `appearance:auto`, inconsistent with geo-filter selects (Minor, Med)

- Ground truth: cross-surface consistency + design-system primitive reuse · all tabs. `controls-adults`,
  `controls-preschool`, `controls-schoolkids` render native OS dropdown arrow (`appearance:auto`), while
  geo selects use the `SelectField` pattern (`appearance-none` + custom `ChevronDown`). Two dropdown
  chromes side by side. **Fix**: `shell/controls.tsx` — wrap the three selects in `SelectField` /
  apply `GEO_SELECT_CLASS`.

### DWT-003 — Min-role currency selects use native `appearance:auto` (Minor, Med)

- Same consistency violation · Min-role tab. `target-currency-select` + `display-currency-select` (and
  the ref-city/ref-role selects sharing `fieldControl`) lack `appearance-none`/custom chevron. **Fix**:
  `shell/min-role.tsx` — add `appearance-none pr-8 pl-3` to `fieldControl` or adopt `SelectField`.

### DWT-004 — "Baseline source" segmented control renders 56px (wraps) at ≤375px (Minor, Med)

- Ground truth: mobile mockup + `min-h-[44px]` control-family rule · Min-role · 320/375px only (44px at
  ≥768px). Three options ("Savings target / Match a role / My salary") wrap inside the fixed row →
  56px box, breaking the 44px rhythm and bottom-alignment. **Fix**: `SegmentedControl`/`min-role.tsx`
  — `flex-wrap` or responsive vertical stack at mobile.

### DWT-006 — Foreigner-school flag has no visual hierarchy above adjacent table text (Minor, Low)

- Ground truth: visual-hierarchy best practice · School column · ≥1280px. The "public n/a → private"
  in-cell flag uses `ml-1 block text-xs text-muted-foreground` — identical colour/size to table
  captions, so an override of the user's explicit "public" choice reads like a footnote. On-token but
  low-hierarchy. **Fix**: `shell/cost-of-living.tsx` — promote to a `Badge`/`text-warning`-tone
  annotation. **Pairs UWT-002.**

### DWT-007 — Salary-currency toggle not bottom-aligned with sibling gross input (Minor, Low)

- Ground truth: mockup + `controls.tsx` "bottom-aligns in items-end field rows" intent · Min-role
  my-salary baseline · ≥768px. The toggle's `fieldGroup` parent computes `align-items:normal` not
  `flex-end`. **Fix**: `shell/min-role.tsx` — ensure the currency `fieldGroup` is a direct `items-end`
  flex child and the label is a `<label>` so the column height is deterministic.

**Design coverage**: element×styling matrix (22 elements — token colours, 44px, primitive reuse all
enumerated); intra-form + cross-surface consistency matrix across 3 tabs; changed surfaces A→DWT-006,
B PASS (2-line header balanced), C→DWT-007, D PASS (spacing/density consistent, only DWT-004 mobile).
Recurrence: dual-currency / H1 identity / mobile card header / tab-label fusion / id-locale names / Area
label-wrap all FIXED; raw-select class re-surfaced (DWT-002/003); segmented 44px holds except baseline
(DWT-004); toggle alignment partial (DWT-007). **Not covered**: dark mode (light only), 320px city-detail
money probes, no external Figma source provided.

---

## Spec proposals

- **Exploratory spec-gaps (SG-###)**: none (feature file already covers observed behaviours).
- **Usability spec-suggestions (USS-###)** (spec-blind — reconcile against existing specs before adding):
  - **USS-001** — Savings tab auto-focuses the salary input on first activation (pairs UWT-005).
  - **USS-002** — Min-role shows an empty state (no pre-populated city panel) before a target is entered (pairs UWT-006).
  - **USS-003** — Area toggle announces active state via `aria-pressed` (pairs UWT-008).
  - **USS-004** — Disabled school-type buttons announce the prerequisite via accessible description (pairs UWT-011).
- **Design spec-proposals (SG-###)** (on-design behaviours worth protecting):
  - **SG-001** — only the active tab's description is visible; it updates on tab change (guards the DWT-001/EWT-001 regression class).
  - **SG-002** — every `<select>` on the calculator uses `appearance:none` + a custom chevron (no native arrow leak).
  - **SG-003** — the "Baseline source" segmented control height ≤ 44px at 320/375/768/1280px.
