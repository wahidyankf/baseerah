# AyoKoding Cost-of-Living Calculator — Usability Findings

**Type**: Usability backlog plan (spec-blind, heuristic evaluation + cognitive walkthrough)

**Evaluation date**: 2026-06-21

**Evaluator**: `web-usability-tester` (spec-blind, sonnet-tier)

---

## Target

| Item              | Detail                                                     |
| ----------------- | ---------------------------------------------------------- |
| Primary URL       | `http://localhost:3101/en/tools/cost-of-living-calculator` |
| Indonesian locale | `http://localhost:3101/id/tools/cost-of-living-calculator` |
| Tools index       | `http://localhost:3101/en/tools` and `/id/tools`           |
| Environment       | Next.js 16 dev server, macOS, Playwright Chromium 1.60.0   |
| Date observed     | 2026-06-21                                                 |

## Usability Goal

Evaluate whether a **first-time visitor** — who has never read documentation or been briefed — can:

1. Understand what the calculator does from the page title and visible controls alone.
2. Use the three tabs (Cost of living / Savings / Minimum role) without being confused by their
   purpose or required inputs.
3. Navigate filters (Region / Country / City / household) intuitively.
4. Understand empty states (no salary entered, no target set).
5. Accomplish a representative task at mobile width (375 px).

## Persona

**First-time visitor** — a software engineer or manager considering relocation, with general web
literacy, no prior knowledge of this tool, no instruction manual. Desktop-primary but also tested
on mobile.

## Tasks Walked

1. **Land and orient** — arrive at the tool, form a mental model of what it does.
2. **Find the cheapest city for a family of 2 adults, 1 school-age child** — use Cost of living
   tab filters.
3. **Estimate monthly savings with a $5,000 USD salary** — switch to Savings tab, enter salary.
4. **Find the minimum seniority level needed to save $3,000/month in Austin** — switch to Minimum
   role tab, enter target.
5. **Complete task 2 on mobile (375 px)** — responsive usability pass.

## Depth

Standard — full heuristic sweep + cognitive walkthroughs for all four tasks at desktop and mobile,
plus URL-naturalness and responsive-usability passes.

## Coverage Map

| Dimension                       | Covered        | Notes                                                  |
| ------------------------------- | -------------- | ------------------------------------------------------ |
| Heuristics 1–10                 | Yes            | All swept                                              |
| Cognitive walkthrough           | Yes            | 4 tasks, 4 questions per step                          |
| First-click / information scent | Yes            | Tools index + tab selection                            |
| URL naturalness                 | Yes            | Locale prefix, tab state, city selection, parent path  |
| Responsive usability            | Yes            | 320, 375, 1280 px                                      |
| Breakpoints                     | 320, 375, 1280 | 768 tablet omitted (budget); would surface same issues |
| Locales                         | en, id         | Both exercised                                         |
| Edge / empty states             | Yes            | Zero-salary Savings tab, zero-target Min Role tab      |
| Destructive actions             | n/a            | No destructive actions present                         |
| Auth flows                      | n/a            | No auth on this tool                                   |
| Performance / Doherty           | Yes            | Filter action measured at 326 ms                       |

**Not covered**: 768 px tablet breakpoint (same class of issues as 375 px would appear); E2E
Playwright test run against a production build.

## Re-evaluation (2026-06-21) — Breadcrumb and URL-State Pass

A targeted follow-up evaluation was run on the same day to verify the newly shipped breadcrumb
navigation and URL-serialised filter controls. Results:

| Previously known issue | Status       | Notes                                                  |
| ---------------------- | ------------ | ------------------------------------------------------ |
| UWT-005 (URL state)    | **RESOLVED** | Tab + filter state in URL; deep-link restore confirmed |
| UWT-010 (back link)    | **RESOLVED** | Back link now carries `?region=…&country=…` context    |

Four new findings were added (UWT-013 through UWT-016):

- **UWT-013** (Severity 2): Breadcrumb current-page label "Calculator" does not match H1 "Cost of
  Living Calculator" — confirmed in both `en` and `id` locales.
- **UWT-014** (Severity 2): Selecting a Country auto-sets Region with no visual advisory — WCAG
  3.2.2 On Input violation.
- **UWT-015** (Severity 1): City-only deep link (`?city=X`) injects inferred region + country
  into the "Back to all cities" href, making the back action non-symmetric.
- **UWT-016** (Severity 2): Geo-filter selects and Area toggle buttons render at 28–29 px on
  mobile (375 px) — below the 44 px preferred tap-target size; inconsistent with household-size
  selects that already apply `min-h-[44px]`.

## Overall Usability Impression

The calculator delivers **substantial data value** and has strong fundamentals: all form controls
carry proper `<label>` associations, ARIA attributes, and reasonable keyboard navigation. The
Cost-of-living tab is self-evident on first contact. However, the **Savings and Minimum Role tabs
both fail the cold-start test**: a first-time user landing on either tab sees negative savings (all
cities at −$X/month) or an unexplained ranking that does not respond to their city preference,
with no guidance on what input is missing. The most impactful friction points are:

- **Savings tab empty state**: the input field has no placeholder and the table instantly shows
  all-negative values — the user does not know why.
- **Minimum Role tab radio-group labels**: three modes ("Monthly savings target", "Reference role",
  "My salary") are unlabelled radio buttons; first-timers cannot predict which to choose without
  experimenting.
- **"Cost of living" tab has no sub-description in the tab label** while the other two tabs do
  (screen-reader-only); sighted users see only the verb-free tab labels with no visible hint of
  what each tab requires.
- **Tools index is a bare link list** with no description per tool.
- **Horizontal scroll at 320 px** with no affordance.

## Document Map

| File                                           | Contents                                                       |
| ---------------------------------------------- | -------------------------------------------------------------- |
| [`findings.md`](./findings.md)                 | Full usability-finding catalog, severity-sorted                |
| [`walkthrough.md`](./walkthrough.md)           | Step-by-step cognitive walkthrough transcript                  |
| [`brd.md`](./brd.md)                           | Business framing and impact                                    |
| [`prd.md`](./prd.md)                           | Personas, user stories, Gherkin ACs                            |
| [`spec-suggestions.md`](./spec-suggestions.md) | Usability-grounded Gherkin proposals                           |
| [`evidence/`](./evidence/)                     | Committed screenshots (phase-N-description-locale-breakpx.png) |

## Promotion Path

On promotion to `plans/in-progress/`, `plan-maker` adds `tech-docs.md` and a TDD-shaped
`delivery.md`. `tech-docs.md` and `delivery.md` are **not authored here** — that is `plan-maker`'s
responsibility.

There is intentionally **no `spec-gaps.md`** — spec-aware gap analysis requires reading `specs/**`,
which this agent refuses to do. `spec-suggestions.md` proposes desired behaviours from usability
principles; a spec-aware reviewer must reconcile them against the existing `specs/**` before
incorporating.
