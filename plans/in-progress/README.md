# In-Progress Plans

Active project plans currently being worked on.

## Active Plans

- [ayokoding-www-calc-exploratory-findings](./ayokoding-www-calc-exploratory-findings/README.md) — Exploratory testing of the cost-of-living calculator surfaced 7 defects (4 Major, 3 Minor) and 4 spec gaps. Major: geo-filter dropdowns not seeded from `?country=`/`?city=` deep links, `html lang="en"` on `/id/` pages (WCAG 3.1.1 Level A), and the desktop table showing English city/country names on the Indonesian locale while mobile cards show Indonesian. Core calculator math verified correct.

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
