# Backlog Plans

Planned projects for future implementation.

## Planned Projects

- [AyoKoding Calculator Exploratory Findings](./2026-06-21__ayokoding-calculator-exploratory-findings/) — Spec-aware exploratory testing of the cost-of-living calculator (all three tabs, EN + ID locales, 375/768/1280/1440 px). Two defects found: zero savings target shows no minimum-role marker (EWT-001, Major) and page title duplicates the site name (EWT-002, Minor). Five spec gaps proposed.
- [AyoKoding Cost-of-Living Calculator — Design Findings](./2026-06-21__ayokoding-www-cost-of-living-design-findings/) — Design-aware evaluation of the calculator at 375 px / 1280 px across en + id locales. 7 findings: 2 Major (tab overflow at mobile ID, dark mode active-tab colour loss), 3 Minor (unstyled savings input, baseline-source select vs SegmentedControl, geo-filter border inconsistency), 1 Trivial, 1 Cosmetic. 4 spec-gap proposals.
- [AyoKoding Calculator Usability Findings](./2026-06-21__ayokoding-calculator-usability-findings/) — Spec-blind heuristic evaluation + cognitive walkthrough of the cost-of-living calculator at `/en/` and `/id/` locales, desktop and mobile. 12 findings (3 severity-3, 5 severity-2, 4 severity-1); top friction: Savings tab all-negative empty state (UWT-001), Minimum Role mode selector unlabelled (UWT-002), tab sub-descriptions invisible to sighted users (UWT-003). 7 spec suggestions filed.
- [AyoKoding Calculator Breadcrumb — Design Findings](./2026-06-21__ayokoding-www-breadcrumb-design-findings/) — Design-aware evaluation of the newly-added `CalculatorBreadcrumb` component across EN + ID locales at 375 / 768 / 1280 px. 4 Minor findings: breadcrumb labels hardcoded in English on the ID locale (DWT-B-001, High priority), missing `flex-wrap` (DWT-B-002, latent reflow risk), "/" text separator inconsistent with site-wide ChevronRight breadcrumb standard (DWT-B-003), and bespoke reimplementation instead of reusing the shared `Breadcrumb` component (DWT-B-004). WCAG AA contrast, token compliance, no overflow at any breakpoint all confirmed on-design. 2 spec-gap proposals filed.
- [AyoKoding Calculator URL-State Exploratory Findings](./2026-06-21__ayokoding-calculator-url-state-exploratory/) — Spec-aware exploratory testing of the newly-added URL state serialization feature (all 9 controls, EN + ID locales, 375 / 1280 px). 70 Playwright checks: 70 PASS, 0 FAIL. 1 Minor finding: breadcrumb "Calculator" crumb hardcoded in English on the id locale (EWT-001, High priority). All 13 URL-state spec scenarios (URL-001 through URL-013) passed. 4 spec-gap proposals filed.

## Instructions

**Quick Idea Capture**: For 1-3 liner ideas not ready for formal planning, use `../ideas.md`.

When creating a new plan:

1. Create folder: `YYYY-MM-DD__[project-identifier]/`
2. Add standard files: README.md, brd.md, prd.md, tech-docs.md, delivery.md
3. Add the plan to this list
