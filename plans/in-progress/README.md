# In-Progress Plans

Active project plans currently being worked on.

## Active Plans

- [ayokoding-www-calc-exploratory-findings](./ayokoding-www-calc-exploratory-findings/README.md) — Spec-aware exploratory testing of the cost-of-living calculator. After a 2026-06-19 re-run, **13 defects** (5 Major, 8 Minor) and **6 spec gaps** are open; all 7 original findings re-verified STILL-PRESENT. Major: geo-filter dropdowns not seeded from `?country=`/`?city=` deep links, `html lang="en"` on `/id/` pages (WCAG 3.1.1 Level A), English city/country names on the Indonesian desktop table, the Housing column ignoring the rural area discount, and confidence flags missing on the Cost-of-Living/Savings tabs. Core calculator math verified correct.
- [ayokoding-www-calc-usability-findings](./ayokoding-www-calc-usability-findings/README.md) — Spec-blind heuristic usability evaluation of the same cost-of-living calculator (Nielsen 10 heuristics + 0–4 severity, cognitive walkthrough, information scent, responsive mobile/tablet/desktop, URL naturalness). Judges only what a first-time user perceives. Findings catalogued as UWT-001…UWT-014 with a per-task cognitive walkthrough in `walkthrough.md`.

## Instructions

**Quick Idea Capture**: For 1-3 liner ideas not ready for formal planning, use `../ideas.md`.

**Naming**: Plans in `in-progress/` use NO date prefix — just the slug (e.g., `organiclever-web-responsive-breakpoints/`). Strip the date prefix when moving from `backlog/`.

When starting work on a plan:

1. Move and rename the plan folder: `git mv backlog/YYYY-MM-DD__[identifier]/ in-progress/[identifier]/` (strip the date prefix)
2. Update the plan's README.md status to "In Progress"
3. Add the plan to this list

When completing a plan:

1. Rename and move: `git mv in-progress/[identifier]/ done/YYYY-MM-DD__[identifier]/` using today's completion date
2. Update this list
