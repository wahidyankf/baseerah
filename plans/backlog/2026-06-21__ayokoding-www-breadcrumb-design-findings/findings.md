# Design Findings — AyoKoding Cost-of-Living Calculator Breadcrumb

Evaluated 2026-06-21. Sorted by severity then area.

---

## DWT-B-001 — Breadcrumb link labels are hardcoded English on the Indonesian locale

**Severity:** Minor
**Priority:** High (visible to all Indonesian visitors)
**Area / Component:** Breadcrumb — `CalculatorBreadcrumb`
**Defect type:** Localization / Consistency

### Violated Ground Truth or Principle

Design principle — Consistency and Repetition: all other text on the `/id/` page is translated;
the breadcrumb is the sole untranslated surface.

`apps/ayokoding-www/src/features/i18n/core/translations.ts` line 176 already provides
`toolsPageTitle: "Alat"` for Indonesian. The `CalculatorBreadcrumb` bypasses the translation
layer with hardcoded strings.

### Environment

- URL: `http://localhost:3101/id/tools/cost-of-living-calculator`
- Breakpoints: 375 px / 768 px / 1280 px (all identical)
- Locale: `id` — confirmed by `html[lang]="id"`
- Date: 2026-06-21

### Steps to Reproduce

1. Navigate to `http://localhost:3101/id/tools/cost-of-living-calculator`.
2. Inspect the breadcrumb `<nav aria-label="Breadcrumb">`.
3. Read item text: "Home", "Tools", "Calculator" render on the Indonesian-locale page.

### Expected Result

Items in Indonesian: "Beranda" (Home), "Alat" (Tools), "Kalkulator" (Calculator).

### Actual Result

Items in English: "Home", "Tools", "Calculator". Confirmed by Playwright
`li.textContent` on the `id`-locale URL with `html[lang]="id"`.

### Evidence

- `./evidence/phase-1-breadcrumb-id-375px.png`
- `./evidence/phase-1-breadcrumb-id-1280px.png`

### Reproducibility

Always

### Suggested Fix Locus

`apps/ayokoding-www/src/features/cost-of-living-calculator/shell/calculator-breadcrumb.tsx` —
replace hardcoded `"Home"` / `"Tools"` / `"Calculator"` with
`t(locale, "navHome")` / `t(locale, "toolsPageTitle")` / `t(locale, "calcTitle")`. Add
`navHome: "Home"` / `"Beranda"` to `translations.ts`.

---

## DWT-B-002 — Breadcrumb `ol` lacks `flex-wrap`, creating a latent reflow risk

**Severity:** Minor
**Priority:** Medium (safe now; becomes a blocker when DWT-B-001 localization is applied)
**Area / Component:** Breadcrumb — `CalculatorBreadcrumb`
**Defect type:** Responsive / Spacing-density

### Violated Ground Truth or Principle

Design-system-primitive divergence: the project-wide `Breadcrumb` component
(`features/navigation/shell/breadcrumb.tsx` line 17) uses `flex flex-wrap items-center gap-1`.
`CalculatorBreadcrumb` uses `flex items-center gap-1 text-sm text-muted-foreground` — no
`flex-wrap`.

WCAG 2.1 SC 1.4.10 (Reflow) requires content to reflow without loss at 320 px.

### Environment

- URL: `http://localhost:3101/id/tools/cost-of-living-calculator`
- Breakpoints: 375 px (primary risk), 320 px (secondary)
- Date: 2026-06-21

### Steps to Reproduce

1. Lengthen a breadcrumb label (or apply the DWT-B-001 localization fix).
2. Navigate at 375 px.
3. The item overflows the viewport without `flex-wrap` to allow wrapping.

Current English labels do not overflow (right edge 359 px at 375 px viewport) — the structural
absence is the finding.

### Expected Result

`ol` carries `flex-wrap`, matching the sibling `Breadcrumb` component.

### Actual Result

`ol` class: `"flex items-center gap-1 text-sm text-muted-foreground"` — no `flex-wrap`.
Confirmed by Playwright `el.className`.

### Evidence

Source: `calculator-breadcrumb.tsx` line 11 vs. `navigation/shell/breadcrumb.tsx` line 17.

### Reproducibility

Always (structural; overflow only triggered by long labels)

### Suggested Fix Locus

`apps/ayokoding-www/src/features/cost-of-living-calculator/shell/calculator-breadcrumb.tsx`
line 11 — add `flex-wrap` to the `ol` className.

---

## DWT-B-003 — Breadcrumb separator is literal "/" text, not a `ChevronRight` icon

**Severity:** Minor
**Priority:** Low (cosmetic; does not block use)
**Area / Component:** Breadcrumb — `CalculatorBreadcrumb`
**Defect type:** Consistency / Typography

### Violated Ground Truth or Principle

Cross-surface visual consistency: the project-wide `Breadcrumb`
(`features/navigation/shell/breadcrumb.tsx` lines 20-21) uses
`<ChevronRight className="h-3 w-3 shrink-0" />`. The `CalculatorBreadcrumb` uses
`<li aria-hidden="true" className="select-none">/</li>`.

### Environment

- URLs: both `/en/` and `/id/` calculator pages, all breakpoints
- Date: 2026-06-21

### Steps to Reproduce

1. Navigate to `http://localhost:3101/en/tools/cost-of-living-calculator`.
2. Inspect separator `<li>` in the breadcrumb `<ol>`.
3. Compare to article breadcrumbs using the shared `Breadcrumb` component (ChevronRight icon).

### Expected Result

Separator is a `<ChevronRight className="h-3 w-3 shrink-0" />` icon, matching the
project-wide breadcrumb.

### Actual Result

Separator: `<li aria-hidden="true" class="select-none">/</li>`. Computed: 14 px, weight 400,
same color as the link text `lab(39.7237 1.18545 6.89404)`.

### Evidence

Source: `calculator-breadcrumb.tsx` lines 17-19 and 25-27.

Playwright — Item 1: `"/" | aria-hidden="true" | font-size=14px | font-weight=400`.

### Reproducibility

Always

### Suggested Fix Locus

Replace `<li aria-hidden="true" className="select-none">/</li>` with
`<ChevronRight className="h-3 w-3 shrink-0" aria-hidden="true" />` inline inside the adjacent
`<li>`. Best fixed as part of DWT-B-004.

---

## DWT-B-004 — `CalculatorBreadcrumb` is a bespoke reimplementation instead of reusing the shared `Breadcrumb`

**Severity:** Minor
**Priority:** Medium (increases maintenance surface; root cause of DWT-B-001 to DWT-B-003)
**Area / Component:** Breadcrumb — `CalculatorBreadcrumb`
**Defect type:** Primitive-reuse / Consistency

### Violated Ground Truth or Principle

Design-system-primitive reuse (Ground Truth Source 3): `libs/web-ui` does not ship a breadcrumb,
but the project-wide breadcrumb is `apps/ayokoding-www/src/features/navigation/shell/breadcrumb.tsx`.
`CalculatorBreadcrumb` reimplements the same `<nav aria-label="Breadcrumb"><ol>` structure
independently, diverging on separator, flex behavior, and localization.

### Environment

- Source files:
  - `apps/ayokoding-www/src/features/navigation/shell/breadcrumb.tsx`
  - `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/calculator-breadcrumb.tsx`
- Date: 2026-06-21

### Steps to Reproduce

1. Read `features/navigation/shell/breadcrumb.tsx` (ChevronRight, `flex-wrap`, props-driven).
2. Read `features/cost-of-living-calculator/shell/calculator-breadcrumb.tsx` ("/", no wrap,
   hardcoded strings).
3. Both render `<nav aria-label="Breadcrumb">` with different implementations.

### Expected Result

The calculator breadcrumb reuses or extends the shared `Breadcrumb` component. No independent
reimplementation is needed.

### Actual Result

Two distinct breadcrumb implementations exist in the same app, creating maintenance divergence.

### Evidence

- `apps/ayokoding-www/src/features/cost-of-living-calculator/shell/calculator-breadcrumb.tsx`
  (34 lines)
- `apps/ayokoding-www/src/features/navigation/shell/breadcrumb.tsx` (29 lines)

### Reproducibility

Always (structural)

### Suggested Fix Locus

Replace `CalculatorBreadcrumb` with a call to the shared `Breadcrumb` component (or generalise
the shared component's interface to accept explicit `href` per segment). Resolves DWT-B-001,
DWT-B-002, and DWT-B-003 as side effects.

---

## Non-Findings (Confirmed On-Design)

| Item                                    | Result                                                                    |
| --------------------------------------- | ------------------------------------------------------------------------- |
| Breadcrumb overflow at 375 px (EN)      | No overflow — right edge 359 px, viewport 375 px.                         |
| Breadcrumb overflow at 375 px (ID)      | No overflow at current English labels (DWT-B-002 is latent only).         |
| Breadcrumb-to-H1 spacing                | 16 px gap via `space-y-4` container. On-design.                           |
| "Calculator" current-page visual weight | `font-medium text-foreground` vs `text-muted-foreground` links. Distinct. |
| WCAG AA light-mode contrast             | 4.74:1 computed. Passes AA (4.5:1 min).                                   |
| WCAG AA dark-mode contrast              | 7.58:1 computed. Passes AA.                                               |
| html[lang] attribute                    | `lang="en"` and `lang="id"` correct per locale.                           |
| Typography tokens                       | `text-sm` (14 px), `text-muted-foreground` / `text-foreground`. On-token. |
| Dark mode color adaptation              | Links resolve to `rgb(148,163,184)`, current item to `rgb(248,250,252)`.  |
| Filter dropdown spacing                 | `space-y-4` container rhythm. On-design.                                  |
