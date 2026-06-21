# Design Evaluation — AyoKoding Cost-of-Living Calculator Breadcrumb

## Context

Design-aware live-site evaluation of the newly-added breadcrumb navigation component and the
existing calculator controls at `ayokoding-www` (port 3101), performed 2026-06-21 by the
`web-design-tester` agent. This plan records design-fidelity and design-practice findings against
the design tokens, `libs/web-ui` primitives, and the project-wide sibling breadcrumb component. It
is a **findings record only** — code changes are not authored here; those come when `plan-maker`
promotes this to `in-progress/`.

## Target URLs

- `http://localhost:3101/en/tools/cost-of-living-calculator` (English)
- `http://localhost:3101/id/tools/cost-of-living-calculator` (Indonesian)

## Environment

- App: `ayokoding-www` (Next.js 16, TypeScript, Tailwind CSS v4, shadcn/ui via `libs/web-ui`)
- Browser: Playwright Chromium (headless), node-playwright direct (not `@playwright/test`)
- Date observed: 2026-06-21
- Build: local dev server (`nx dev ayokoding-www`)
- Locales tested: `en`, `id` (all supported locales per
  `apps/ayokoding-www/src/features/i18n/core/config.ts`)
- Breakpoints tested: 375 px (mobile), 768 px (tablet), 1280 px (desktop) — all three standard
  breakpoints
- Dark mode: evaluated by toggling `.dark` class on `<html>` programmatically for each
  breakpoint × locale combination
- No external design source provided at invocation; skipped per methodology

## Design Goal

Evaluate visual design fidelity of the newly-added `CalculatorBreadcrumb` component and the
existing calculator controls, checking:

1. Breadcrumb typography token usage (font size, color, weight)
2. Breadcrumb reflow at mobile (375 px) without overflow
3. Breadcrumb-to-H1 spacing gap
4. "/" separator legibility vs. design-system icon separator
5. "Calculator" current-page item visual distinction from linked items
6. Filter dropdowns consistent spacing and sizing
7. Dark mode token compliance
8. WCAG AA color contrast on breadcrumb links
9. Consistency with the existing sibling `Breadcrumb` component in
   `apps/ayokoding-www/src/features/navigation/shell/breadcrumb.tsx`

## Design Ground-Truth Sources Used

1. **Committed plan-folder mockup assets** — no breadcrumb mockup exists in the plan assets
   folders; this dimension is N/A for the breadcrumb (new component with no prior mockup). The
   calculator table/controls were previously evaluated in
   `plans/backlog/2026-06-21__ayokoding-www-cost-of-living-design-findings/`.
2. **Design tokens at runtime** — computed styles read via Playwright, compared to
   `libs/web-ui-token/src/tokens.css` and `apps/ayokoding-www/src/app/globals.css`; contrast ratios
   calculated programmatically from token values
3. **Design-system primitives** — `libs/web-ui` (no breadcrumb primitive exists); project-wide
   breadcrumb pattern is `apps/ayokoding-www/src/features/navigation/shell/breadcrumb.tsx` — used
   as the design-system reference for this component type
4. **External design source** — none provided; skipped
5. **Design best practice / WCAG AA** — visual hierarchy, separator legibility, localization
   completeness, `flex-wrap` reflow safety evaluated per WCAG 2.1 SC 1.4.3 (contrast) and
   SC 1.4.10 (reflow)

## Coverage Map

| Dimension                     | en / 375 px | en / 768 px | en / 1280 px | id / 375 px | id / 768 px | id / 1280 px |
| ----------------------------- | ----------- | ----------- | ------------ | ----------- | ----------- | ------------ |
| Mockup fidelity               | N/A         | N/A         | N/A          | N/A         | N/A         | N/A          |
| Runtime token fidelity        | Evaluated   | Evaluated   | Evaluated    | Evaluated   | Evaluated   | Evaluated    |
| Design-system-primitive reuse | Evaluated   | n/a         | n/a          | n/a         | n/a         | n/a          |
| Visual hierarchy              | Evaluated   | Evaluated   | Evaluated    | Evaluated   | Evaluated   | Evaluated    |
| Alignment & grid              | Evaluated   | Evaluated   | Evaluated    | Evaluated   | Evaluated   | Evaluated    |
| Spacing & density             | Evaluated   | Evaluated   | Evaluated    | Evaluated   | Evaluated   | Evaluated    |
| Typography                    | Evaluated   | Evaluated   | Evaluated    | Evaluated   | Evaluated   | Evaluated    |
| Colour & state styling        | Evaluated   | Evaluated   | Evaluated    | Evaluated   | Evaluated   | Evaluated    |
| Dark mode                     | Evaluated   | Evaluated   | Evaluated    | Evaluated   | Evaluated   | Evaluated    |
| Responsive design fidelity    | Evaluated   | Evaluated   | Evaluated    | Evaluated   | Evaluated   | Evaluated    |
| Localization completeness     | Evaluated   | Evaluated   | Evaluated    | Evaluated   | Evaluated   | Evaluated    |
| Cross-surface consistency     | Evaluated   | n/a         | n/a          | n/a         | n/a         | n/a          |
| WCAG AA contrast              | Evaluated   | n/a         | n/a          | n/a         | n/a         | n/a          |

**Not covered:**

- Hover/focus-ring screenshots (focus styles not captured; visual observation only)
- Interactive state for the "Tools" link (not clicked; nav behavior not exercised)
- Calculator table visual regression at 768 px (prior plan
  `2026-06-21__ayokoding-www-cost-of-living-design-findings` covers 375/1280 px; 768 px not
  previously exercised — noted as gap but not a breadcrumb concern)

## Overall Design-Fidelity Impression

The breadcrumb renders without overflow at all three breakpoints and both locales, uses design
tokens correctly for colors, spacing, and typography, and correctly marks the current page with
`font-medium text-foreground` (visually distinct from the dimmer linked items). WCAG AA contrast
passes in both light mode (4.74:1) and dark mode (7.58:1).

**Four findings were identified**, all Minor severity:

1. **DWT-B-001 (Minor)** — Breadcrumb link labels are hardcoded in English on the Indonesian
   locale. "Home", "Tools", "Calculator" should be "Beranda" / "Alat" / "Kalkulator" at `/id/`.
2. **DWT-B-002 (Minor)** — `CalculatorBreadcrumb` `ol` lacks `flex-wrap`, unlike the sibling
   `Breadcrumb` component. Safe now with short English labels; fragile when ID translations are
   applied (longer words).
3. **DWT-B-003 (Minor)** — Separator uses literal `"/"` plain text instead of `ChevronRight`
   icon, creating inconsistency with the project-wide `Breadcrumb` component and a less visually
   refined separator.
4. **DWT-B-004 (Minor)** — The `CalculatorBreadcrumb` is a bespoke reimplementation rather than
   reusing or extending the existing project-wide `Breadcrumb` component
   (`features/navigation/shell/breadcrumb.tsx`), fragmenting the breadcrumb design language.

## Document Map

- **`README.md`** (this file) — context, coverage, overall impression
- **`brd.md`** — business framing
- **`prd.md`** — personas, user stories, Gherkin acceptance criteria
- **`findings.md`** — full design-defect catalog (developer worklist)
- **`spec-gaps.md`** — design-spec proposals worth adding to `specs/`
- **`evidence/`** — committed screenshots, named by phase/locale/breakpoint

`tech-docs.md` and `delivery.md` are not authored here — they are produced when `plan-maker`
promotes this plan to `plans/in-progress/`.
