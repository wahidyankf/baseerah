# In-Progress Plans

Active project plans currently being worked on.

## Active Plans

- [baseerah-repo-reset](./baseerah-repo-reset/README.md) — Strips this `ose-public` clone down to
  `rhino-cli` and the engineering harness, then stands up Baseerah — a personal-assistant product
  within the OSE ecosystem — as `baseerah-be` (:19320), `baseerah-be-e2e`, `baseerah-fe` (:19310),
  and `baseerah-fe-e2e`. Deleted the other two plans that were previously in this folder
  (`ayokoding-learning-path-04-course-authoring`, `vercel-function-cost-reduction`) along with the
  apps they targeted.

Ready-to-execute plans wait in [`../backlog/`](../backlog/README.md); promote one here when
work begins.

## Instructions

**Idea Capture**: For ideas not ready for formal planning, write a two-pager in `../ideas/`.

**Naming**: Plans in `in-progress/` use NO date prefix — just the slug (e.g., `baseerah-repo-reset/`). `backlog/` also uses no date prefix, so moving from `backlog/` is a pure move.

When starting work on a plan:

1. Move the plan folder: `git mv backlog/[identifier]/ in-progress/[identifier]/` (no rename — neither stage carries a date prefix)
2. Update the plan's README.md status to "In Progress"
3. Add the plan to this list

When completing a plan:

1. Rename and move: `git mv in-progress/[identifier]/ done/YYYY-MM-DD__[identifier]/` using today's completion date
2. Update this list
