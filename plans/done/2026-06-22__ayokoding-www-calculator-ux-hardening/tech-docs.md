# Technical Docs — Calculator UX Hardening

Root-cause analysis and chosen fix approach per cluster. File paths are grounded against the actual
source (the testers correctly identified bugs but mis-named two files — the tab strip lives in the **app
route**, not a `shell/calculator-content.tsx`).

**Key paths**

- Tab strip: `apps/ayokoding-www/src/app/[locale]/tools/cost-of-living-calculator/calculator-content.tsx`
- Feature shell: `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/{cost-of-living,city-detail,controls,min-role,savings,geo-filters}.tsx`
- Pure core: `…/cost-of-living-calculator/core/{calc.ts,data/cities.ts,url-state.ts}`
- i18n: `apps/ayokoding-www/src/features/i18n/core/translations.ts`
- CSP/GA: `apps/ayokoding-www/next.config.ts`
- Design-system primitives: `libs/web-ui/` (`SelectField`/`Input`/`Badge`/`Table`/`Tabs`)

---

## Cluster 1 — Tab descriptions always visible (EWT-001 ≡ DWT-001, Major)

**Root cause** (confirmed): `calculator-content.tsx` lines 263 / 270 / 276 build the className via a
template literal with **no space before the conditional**:

```tsx
className={`mt-1 text-sm text-muted-foreground${activeTab === "cost" ? "" : "hidden"}`}
```

When inactive this yields the non-existent class `text-muted-foregroundhidden`, so `hidden` never
applies and all three descriptions stay visible.

**Fix**: insert a space so the conditional is its own class token:

```tsx
className={`mt-1 text-sm text-muted-foreground ${activeTab === "cost" ? "" : "hidden"}`}
```

(Same for the `savings` and `min-role` lines.) **Confidence: HIGH.** Guard with SG-001 spec + a unit
test asserting inactive descriptions have the `hidden` class / are not visible.

---

## Cluster 2 — Touch targets & segmented/sort a11y (EWT-002, EWT-005, UWT-008, UWT-011)

- **EWT-002** (tab triggers 29px; school-type/area/salary-currency radios 28px < 44px): the segmented
  radios in `controls.tsx`/`min-role.tsx` use an `h-9`-ish container whose buttons render 28px after
  padding; tab triggers inherit the `libs/web-ui` `TabsTrigger` height. **Fix**: add `min-h-[44px]` (+
  centring) to each segmented-radio button and ensure the `TabsTrigger` instances reach 44px at mobile.
  Prefer fixing at the `SegmentedControl` primitive so all four groups inherit it. **Confidence: HIGH.**
- **UWT-008** (Area toggle has no `aria-pressed`): the segmented radios render as buttons without pressed
  state and signal active purely via `bg-primary`. **Fix**: the `SegmentedControl` already uses
  `role="radio"`/`radiogroup` per the design tester — ensure each option exposes `aria-checked`
  (radio-correct) **and** add a non-colour active indicator (e.g. ring/underline). If implemented as
  toggle buttons, use `aria-pressed`. Reconcile to one ARIA pattern. **Confidence: MEDIUM** (verify the
  current ARIA role at execution; pick `aria-checked` if radiogroup, `aria-pressed` if toggle). Pairs USS-003.
- **UWT-011** (disabled school-type buttons lack description): add `id="school-type-hint"` to the existing
  "add school-age children to choose" hint and `aria-describedby="school-type-hint"` + `aria-disabled="true"`
  to both buttons in `controls.tsx`. **Confidence: HIGH.** Pairs USS-004.
- **EWT-005** (sort `<th>` lacks `aria-sort`): add `aria-sort={sortActive ? (asc ? "ascending" :
"descending") : "none"}` to the sortable header `<th>` in `savings.tsx`. **Confidence: HIGH.**

---

## Cluster 3 — Foreigner public-school flag (EWT-003, UWT-002, DWT-006)

- **EWT-003** (city-detail missing testid): `cost-of-living.tsx:231` renders
  `data-testid={\`school-foreigner-flag-${r.city.id}\`}`but`city-detail.tsx`only amends the school
label + renders`foreigner-public-school-note`— no flag testid. **Fix**: add the`school-foreigner-flag-<cityId>`span to the city-detail school row when`schoolForeignerFallback`.
  **Confidence: HIGH.**
- **UWT-002** (wording cryptic) + **DWT-006** (no hierarchy): the in-cell flag is `ml-1 block text-xs
text-muted-foreground` reading "public n/a → private". **Fix (combined)**: reword to plain language
  (e.g. "Private — public not open to foreigners", localized; keep concise for the cell) and promote to a
  warning-tone token — a `Badge` (outline, honey/amber hue, matching the controls' foreigner note tone)
  or `text-warning` — so it reads as an override flag, not a caption. Apply consistently in the table
  (`cost-of-living.tsx:231`) and city-detail. New i18n keys for the reworded flag. **Confidence: HIGH.**
  See `assets/ui-foreigner-flag-low-fi.md`.

---

## Cluster 4 — Jargon glosses & i18n labels (EWT-004, UWT-001/003/004/009/010/012/013/014)

- **UWT-001** ("Baseline source" opaque): relabel via new i18n value to "How to set your target" (en) /
  "Cara menetapkan target" (id) — or "Target method". **Fix**: `translations.ts` + `min-role.tsx` label.
  **Confidence: HIGH** (label-only).
- **EWT-004 ≡ UWT-014** (OOP): `cost-of-living.tsx:131` and `:298` hardcode `<abbr title="out-of-pocket">`.
  **Fix**: `title={t(locale, "healthcareOutOfPocket")}` (id value "bayar sendiri" already exists; verify
  en value reads "out-of-pocket"). **Confidence: HIGH.**
- **UWT-009** (Relocation(sunk)/Liquidity reserve no gloss): **LIKELY ALREADY ADDRESSED** —
  `cost-of-living.tsx:137,140` already wrap these headers in `<abbr title={t(locale,
"tooltipRelocationSunk")}>` / `tooltipLiquidityReserve`. The spec-blind tester missed the `abbr`
  tooltip. **Action**: verify the tooltips render at execution; if present, no code change — only add the
  SG/USS spec coverage. If the `abbr` is insufficient (e.g. not keyboard-reachable), add a visible info
  affordance. **Confidence: MEDIUM (probable false-positive).**
- **UWT-010** (P25/Median/P75): add `title` glosses (or `<abbr>`) on the min-role distribution headers in
  `min-role.tsx` + new i18n tooltip keys. **Confidence: HIGH.**
- **UWT-013** (ic/mgmt): expand to full words or `<abbr title>` in the Track column (`min-role.tsx`),
  using localized track labels. **Confidence: HIGH.**
- **UWT-003** (Non-salary comp header): already wraps to 2 lines (DWT confirmed). Shorten to "Non-salary
  comp" + `title` expansion ("RSU/equity + bonus — annual, informational only"). `min-role.tsx` +
  translations. **Confidence: HIGH.**
- **UWT-004** (region options English in id; MENA/Nordics unexpanded): region option labels come from a
  static list rendered in `geo-filters.tsx`. **Fix**: localize region display names via `translations.ts`
  (keep the underlying value/key English for URL stability) and expand MENA/Nordics via title/full text in
  both locales. **Confidence: HIGH** (display-only; do not change the serialized region key).
- **UWT-012** (healthcare scheme ALL-CAPS): one scheme label renders upper-case. **Fix**: normalize the
  scheme label strings / badge text-transform to sentence-case in `translations.ts` / the Badge usage.
  **Confidence: HIGH.**

---

## Cluster 5 — UX states (UWT-005, UWT-006, UWT-007)

- **UWT-005** (Savings empty-state prominence): wrap the existing prompt sentence in a bordered
  empty-state panel in the data area and auto-focus the gross input when the Savings tab activates (effect
  in `savings.tsx`/`calculator-content.tsx`). **Confidence: HIGH.** Implements USS-001.
- **UWT-006** (Min-role example panel): label the pre-target city panel "Example (<city>)" via a localized
  caption, or suppress it until a target is entered. Lower-risk: add the caption. `min-role.tsx`.
  **Confidence: HIGH.** Implements USS-002.
- **UWT-007** (Savings at-field currency): add an inline "USD" prefix/suffix to the Savings gross input
  (consistent with My-salary mode). `savings.tsx` + `libs/web-ui` Input adornment if available.
  **Confidence: HIGH.**

---

## Cluster 6 — Design-system fidelity (DWT-002, DWT-003, DWT-004, DWT-007)

- **DWT-002/003** (native `appearance:auto` selects): wrap the household selects (`controls.tsx`) and the
  min-role currency/ref selects (`min-role.tsx`) in the `SelectField` primitive / `GEO_SELECT_CLASS`
  (`appearance-none pr-8 pl-3` + `ChevronDown` overlay) used by geo selects. **Confidence: HIGH.** SG-002.
- **DWT-004** (baseline-source segmented 56px at ≤375px): the 3-option `SegmentedControl` wraps inside a
  fixed row. **Fix**: allow `flex-wrap` with consistent per-item height, or stack vertically at mobile so
  each option keeps 44px and the box doesn't balloon. `controls.tsx` `SegmentedControl` / `min-role.tsx`.
  **Confidence: MEDIUM** (choose wrap-vs-stack — see `assets/ui-baseline-source-mobile-low-fi.md`; default:
  `flex-wrap` keeping 44px rows). SG-003.
- **DWT-007** (salary-currency toggle alignment): ensure the toggle's `fieldGroup` is a direct `items-end`
  flex child and its label is a `<label>` (deterministic column height) so it bottom-aligns with the gross
  input. `min-role.tsx`. **Confidence: MEDIUM** (verify the flex parent at execution).

---

## Cluster 7 — Security/CSP (EWT-006)

**Root cause**: `next.config.ts` injects a GA `gtag` script for `G-1NHDR7S3GV` but the CSP `script-src`
(`'self' 'unsafe-inline' 'unsafe-eval'`) does not whitelist `googletagmanager.com` / `google-analytics.com`,
so the browser blocks it and logs a console error on every load.

**Decision**: whitelist the GA origins in the CSP `script-src` + `connect-src` (and `img-src` for GA
beacons) **iff** GA is intentionally enabled in production; the GA tag is already shipped, so analytics
appear intended. This is the minimal, behaviour-preserving fix (it makes the already-present tag actually
work and clears the console error). If the site owner does **not** want GA, the alternative is removing
the tag — flagged as a `[HUMAN]` decision in delivery. **Default: whitelist** (keeps the shipped intent).
**Confidence: MEDIUM** (depends on GA intent — see delivery `[HUMAN]` gate). Follow the repo CSP +
security-headers conventions; do not weaken other directives.

---

## Specs to fold in (feature-change-completeness + regression-test mandate)

- Accept **SG-001** (active-tab-description), **SG-002** (all selects styled), **SG-003** (baseline-source
  ≤44px) into the calculator `.feature`.
- Accept **USS-001…004** (auto-focus, example-panel, aria-pressed/checked, disabled-button description) as
  Gherkin after spec-aware reconciliation (none duplicate existing scenarios).
- Every behavioural fix above gets a reproducing test (unit/component or Gherkin-bound step) in the same
  commit — the bug→test dual of feature-change-completeness.

## Decisions log (made autonomously; sensible defaults, reversible)

1. **Scope** = fix all 26 findings (small, well-understood, serve the hardening intent). UWT-009 treated as
   verify-then-likely-spec-only.
2. **Foreigner flag** styling = warning-tone `Badge`/`text-warning` + reworded localized label (not bare arrow).
3. **Baseline-source mobile** = `flex-wrap` keeping 44px rows (fallback: vertical stack).
4. **CSP/GA** = whitelist GA origins (keep shipped analytics intent); removal is a `[HUMAN]` alternative.
5. **Region localization** = localize display names only; keep serialized region keys English (URL stability).
