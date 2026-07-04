# In-Progress Plans

Active project plans currently being worked on.

## Active Plans

- [enforce-repo-wide-scenario-implementation](./enforce-repo-wide-scenario-implementation/README.md) — roll out `@covers` + level tags + per-tier fail-on-skip across all 51 apps/libs in ose-public, ose-primer, and ose-infra, and upgrade `behavior-coverage` to a runtime cross-check so every Gherkin scenario is genuinely implemented. **Depends on** `enforce-identical-rhino-cli-gherkin` (done). Created 2026-07-03.

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
