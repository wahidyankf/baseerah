# Design Evaluation — AyoKoding Cost-of-Living Calculator

## Context

Design-aware live-site evaluation of the cost-of-living / salary savings calculator at
`ayokoding-www` (port 3101), performed 2026-06-21 by the `web-design-tester` agent. This plan
records design-fidelity and design-practice findings against the committed mockups, design tokens,
and `libs/web-ui` primitives. It is a **findings record only** — code changes are not authored
here; those come when `plan-maker` promotes this to `in-progress/`.

## Target URLs

- `http://localhost:3101/en/tools/cost-of-living-calculator` (English)
- `http://localhost:3101/id/tools/cost-of-living-calculator` (Indonesian)

## Environment

- App: `ayokoding-www` (Next.js 16, TypeScript, Tailwind CSS v4, shadcn/ui via `libs/web-ui`)
- Browser: Playwright Chromium (headless)
- Date observed: 2026-06-21
- Build: local dev server (`nx dev ayokoding-www`)
- Locales tested: `en`, `id` (all supported locales per
  `apps/ayokoding-www/src/features/i18n/core/config.ts`)
- Breakpoints tested: 375 px (mobile), 1280 px (desktop)
- Dark mode: evaluated by toggling `.dark` class on `<html>` programmatically

## Design Goal

Check design fidelity against design system primitives (shadcn/ui, Radix UI, Tailwind CSS tokens),
visual hierarchy, spacing, typography, color contrast (WCAG AA), and component consistency across:

- SegmentedControl (radio group for area, school type, baseline source) — visual selected state
- Tab labels — single phrases only
- Table design — currency formatting (dual-currency: USD + local)
- Empty state placeholder text (if visible)
- Dark mode compatibility
- Responsive layout at 375 px (mobile) and 1280 px (desktop)

## Design Ground-Truth Sources Used

1. **Committed plan-folder mockup assets** — hi-fi PNG mockups from
   `plans/done/2026-06-19__ayokoding-www-salary-savings-calculator/assets/` (the original design
   finalists, referenced by `plans/in-progress/ayokoding-www-cost-of-living-calculator-test-fixing/assets/README.md`):
   - `ui-cost-of-living-option-a-category-table.png` (desktop/tablet/mobile)
   - `ui-savings-option-a-net-savings-table.png` (desktop/tablet/mobile)
   - `ui-min-role-option-a-ladder-table.png` (desktop/tablet/mobile)
   - Empty-state lo-fi wireframes from
     `plans/in-progress/ayokoding-www-cost-of-living-calculator-test-fixing/assets/ui-empty-states-low-fi.md`
2. **Design tokens at runtime** — computed styles read via Playwright on the live page, compared to
   `libs/web-ui-token/src/tokens.css` and `libs/web-ui-token/src/ayokoding.css` and
   `apps/ayokoding-www/src/app/globals.css`
3. **Design-system primitives** — `libs/web-ui` primitives checked for reuse vs. bespoke
   reimplementation
4. **External design source** — none provided at invocation; skipped per methodology
5. **Design best practice** — visual hierarchy, alignment, spacing, typography, colour, consistency
   evaluated; WCAG AA contrast checked per
   [Accessibility First principle](../../../repo-governance/principles/content/accessibility-first.md)

## Coverage Map

| Dimension                     | en / 375 px                       | en / 1280 px | id / 375 px | id / 1280 px |
| ----------------------------- | --------------------------------- | ------------ | ----------- | ------------ |
| Mockup fidelity               | Evaluated                         | Evaluated    | Evaluated   | Evaluated    |
| Runtime token fidelity        | Evaluated                         | Evaluated    | Evaluated   | Evaluated    |
| Design-system-primitive reuse | Evaluated                         | Evaluated    | n/a         | n/a          |
| Visual hierarchy              | Evaluated                         | Evaluated    | Evaluated   | Evaluated    |
| Alignment & grid              | Evaluated                         | Evaluated    | Evaluated   | Evaluated    |
| Spacing & density             | Evaluated                         | Evaluated    | Evaluated   | Evaluated    |
| Typography                    | Evaluated                         | Evaluated    | Evaluated   | Evaluated    |
| Colour & state styling        | Evaluated                         | Evaluated    | Evaluated   | Evaluated    |
| Dark mode                     | Evaluated                         | Evaluated    | Not tested  | Not tested   |
| Responsive design fidelity    | Evaluated                         | Evaluated    | Evaluated   | Evaluated    |
| Cross-surface consistency     | Partially — tabs, controls, cards |              |             |              |

**Not covered:**

- 768 px (tablet) — not requested in the current evaluation; the mockup breakpoints exist for future
  comparison
- Dark mode for Indonesian locale (de-prioritised; same token set)
- Interactive state screenshot for hover/focus rings (focus style captured for savings input)
- `city-detail` drill-down view (not exercised — would require clicking a city link)

## Overall Design-Fidelity Impression

The calculator is largely on-design. Core elements use `libs/web-ui` primitives correctly (Tabs,
Table, Badge). The SegmentedControl bespoke component applies `--color-primary` tokens correctly and
visual selected state is clear.

**Top risks identified:**

1. **DWT-001 (Major) — Tab list overflows viewport at 375 px / id locale**: the "Jabatan minimum"
   tab label protrudes 12 px past the TabsList right edge, causing a visual clip / overflow.
2. **DWT-002 (Major) — Dark mode active tab loses primary-blue fill**: the override
   `data-[state=active]:bg-primary` on the TabsTrigger is defeated in dark mode by the primitive's
   own `dark:data-[state=active]:bg-input/30` rule, giving a dim translucent background instead of
   the strong primary fill shown in both mockups.
3. **DWT-003 (Minor) — Savings tab gross-salary input unstyled**: the `<input>` for gross monthly
   salary is a bare HTML input without using `libs/web-ui` Input primitive or consistent border
   theming, inconsistent with the mockup which shows a styled, bordered input.
4. **DWT-004 (Minor) — Min-role baseline-source control is a `<select>`, not a SegmentedControl**:
   the mockups (both hi-fi and lo-fi) show three-option SegmentedControl for "My salary / Reference
   role / Savings target"; the live page renders a bare `<select>`.
5. **DWT-005 (Minor) — Geo-filter selects inconsistently styled**: the Region/Country/City selects
   in GeoFilters use raw Tailwind utility classes without the themed border, while the Controls
   section adult/kid selects use `border-border` theming through `[&_select]` group selectors.
6. **DWT-006 (Trivial) — H1 title "Cost of Living Calculator" mismatches translation key**:
   `calcTitle` translation = "Salary Savings Calculator" (en) / "Kalkulator Tabungan Gaji" (id),
   but rendered H1 = "Cost of Living Calculator" / "Kalkulator Biaya Hidup". Likely the dev server
   is on an older build of translations.
7. **DWT-007 (Cosmetic) — Min-role baseline SegmentedControl ID locale overflows 375 px**: at id /
   375 px the "Sumber baseline" SegmentedControl renders at 343 px total width, placing the "Gaji
   saya" button right edge at 355 px, extending 12 px past the viewport edge at 375 px.

## Document Map

- **`README.md`** (this file) — context, coverage, overall impression
- **`brd.md`** — business framing
- **`prd.md`** — personas, user stories, Gherkin acceptance criteria
- **`findings.md`** — full design-defect catalog (developer worklist)
- **`spec-gaps.md`** — design-spec proposals worth adding to `specs/`
- **`evidence/`** — committed screenshots, named by phase/locale/breakpoint

`tech-docs.md` and `delivery.md` are not authored here — they are produced when `plan-maker`
promotes this plan to `plans/in-progress/`.
