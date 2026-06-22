# ayokoding-www Cost-of-Living Calculator — UX Hardening

> Fix-ready plan consolidating one `web-ux-test-fixing-planning` run (2026-06-22) over the live
> cost-of-living calculator, across **three lenses** (spec-aware Exploratory, spec-blind Usability,
> design-aware Design) and **both locales** (en, id) at **six breakpoints** (320–1440). The outcome of
> the planning workflow was this plan; the fixes are executed via
> [plan-execution](../../../repo-governance/workflows/plan/plan-execution.md).

## Status

- **Stage**: done (completed 2026-06-22)
- **Type**: web-UI feature-change / UX-hardening (UI-bearing)
- **App**: `apps/ayokoding-www` — `src/features/cost-of-living-calculator/`
- **Targets tested**: `http://localhost:3101/{en,id}/tools/cost-of-living-calculator` (+ city-detail & URL deep links)

## Document map

| File                           | Purpose                                                         |
| ------------------------------ | --------------------------------------------------------------- |
| [README.md](./README.md)       | This index: risk summary + consolidated coverage map            |
| [brd.md](./brd.md)             | Business rationale and scope                                    |
| [prd.md](./prd.md)             | Product requirements (acceptance criteria per finding)          |
| [findings.md](./findings.md)   | The three source-attributed findings sections (EWT/UWT/DWT)     |
| [tech-docs.md](./tech-docs.md) | Root-cause analysis + chosen fix approach per finding/cluster   |
| [delivery.md](./delivery.md)   | TDD-shaped delivery checklist (`[AI]`/`[HUMAN]`, Phase 0 first) |
| [assets/](./assets)            | Lo-fi UI wireframes for the visually-bearing fixes              |

## Findings at a glance

| Source                           | Count | IDs                              |
| -------------------------------- | ----- | -------------------------------- |
| Exploratory (EWT)                | 6     | EWT-001 … EWT-006                |
| Usability (UWT)                  | 14    | UWT-001 … UWT-014                |
| Design (DWT)                     | 6     | DWT-001, 002, 003, 004, 006, 007 |
| Exploratory spec-gaps            | 0     | —                                |
| Usability spec-suggestions (USS) | 4     | USS-001 … USS-004                |
| Design spec-proposals (SG)       | 3     | SG-001, SG-002, SG-003           |

After de-duplicating shared root causes, the work resolves to **~18 distinct fixes** (see tech-docs clusters).

## Top risks

1. **[Exploratory] / [Design] — EWT-001 ≡ DWT-001 (Major)**: a fused Tailwind class
   (`text-muted-foregroundhidden`) leaves **all three tab descriptions permanently visible** on every
   tab. Independently confirmed by two testers; ships today. One-character fix + a guarding spec.
2. **[Exploratory] — EWT-002 (Major)**: tab triggers (29px) and the school-type / area / salary-currency
   segmented radios (28px) are **below the 44px touch target** at mobile widths (WCAG 2.5.8). A11y
   regression on the most-used controls.
3. **[Usability] — UWT-001 (sev-3)**: the "Baseline source" segment-group label has **zero information
   scent** — a first-timer cannot predict what the three options do.
4. **[Usability] / [Design] / [Exploratory] — UWT-002 ≈ DWT-006 ≈ EWT-003**: the **foreigner public-school
   flag** ("public n/a → private") is cryptic (UWT), lacks visual hierarchy (DWT), and is missing its
   `school-foreigner-flag-<cityId>` testid in the city-detail view (EWT). The newest feature surface needs
   a clarity + styling + parity pass.
5. **[Design] — DWT-002 / DWT-003**: household + min-role currency `<select>`s render the **native OS
   dropdown arrow** (`appearance:auto`) next to the styled geo selects — two dropdown chromes on one page.

## Consolidated coverage map (Phase 3.5 cross-tester completeness critic)

**Lenses run**: Exploratory (spec-aware) → Usability (spec-blind) → Design (design-aware), sequential,
each integrated before the next. **Locales**: en + id (both, every tester). **Breakpoints**: 320, 375,
768, 1024, 1280, 1440 (every tester).

**Control × surface grid** — every interactive control was exercised by at least one tester on every
surface it appears; no silent no-op found:

| Control family                                  | Cost tab                         | Savings tab | Min-role tab  | City-detail     | Lenses                            |
| ----------------------------------------------- | -------------------------------- | ----------- | ------------- | --------------- | --------------------------------- |
| Geo selects (region/country/city)               | ✓                                | ✓           | ✓             | back-link       | E,U,D                             |
| Household selects (adults/preschool/schoolkids) | ✓                                | ✓           | ✓             | ✓               | E,U,D (DWT-002)                   |
| School-type segmented                           | ✓ (disabled w/o kids)            | ✓           | ✓             | ✓               | E,U,D (EWT-002, UWT-011)          |
| Area segmented                                  | ✓                                | ✓           | ✓             | ✓               | E,U,D (EWT-002, UWT-008)          |
| Baseline-source segmented                       | —                                | —           | ✓             | —               | U,D (UWT-001, DWT-004)            |
| Salary-currency segmented                       | —                                | —           | ✓ (my-salary) | —               | E,U,D (EWT-002, UWT-007, DWT-007) |
| Gross/target inputs                             | —                                | ✓           | ✓             | —               | E,U,D (debounce PASS)             |
| Display/target currency selects                 | —                                | —           | ✓             | —               | D (DWT-003)                       |
| Sort button                                     | —                                | ✓           | —             | —               | E (EWT-005)                       |
| Tab triggers + descriptions                     | ✓                                | ✓           | ✓             | —               | E,D (EWT-001/DWT-001), EWT-002    |
| Foreigner-school flag                           | ✓ (8 non-open cities enumerated) | —           | —             | ✓ (EWT-003 gap) | E,U,D                             |

**Recurrence classes (Phase 0)** — all re-checked. Fixed & holding: i18n leakage, dual-currency, H1
identity, mobile label-detachment, URL-IA, tab-label-fusion, household scaling, salary validation,
empty-state presence. Still open → new findings: touch targets (EWT-002), jargon labels
(UWT-001/003/009/010/013/014, EWT-004), sort a11y (EWT-005), security/CSP (EWT-006), raw selects
(DWT-002/003), Area-toggle state (UWT-008), disabled-control description (UWT-011).

**Changed surfaces (Phase 0 diff)** — all five exercised: **A** foreigner-school → EWT-003 + UWT-002 +
DWT-006; **B** non-salary 2-line header → UWT-003 (jargon), design PASS; **C** salary-currency toggle →
EWT-002 + UWT-007 + DWT-007; **D** scroll-preservation → PASS (all three); **E** debounced salary → PASS
(all three).

**Declared-invariant conformance**: the foreigner-school flag was enumerated across **all 8 non-open
cities** (not sampled) — flag correct in the table for every one; the only gap is the city-detail testid
(EWT-003).

**Areas not covered (recorded, not silently dropped)**: dark mode (design tester ran light-mode only);
cross-browser (Safari/Firefox); screen-reader live-region announcements; Lighthouse Core Web Vitals;
formal color-contrast audit; zero-result empty state (no non-destructive filter path yields it);
throttled-network loading state. No matrix cell that a tester _should_ have covered was left unexercised,
so **no targeted re-run was required**.

## Notes

- **Snapshot caveat**: this plan reflects the site as tested on 2026-06-22. If the calculator changes
  materially before execution, re-run the three testers.
- **Non-destructive**: no app/lib source was modified by the planning workflow. All evidence captured to
  `local-temp/` (throwaway).
