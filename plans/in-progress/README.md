# In-Progress Plans

Active project plans currently being worked on.

## Active Plans

- [ayokoding-learning-path-01-url-restructure](./ayokoding-learning-path-01-url-restructure/README.md)
  — **Wave 1.** Removes the `/c/` namespace, resolves everything under `/en/learn` to three buckets
  (`paths/`, `courses/`, `legacy/`), and ships the per-course + per-domain redirect tables.

Ready-to-execute plans wait in [`../backlog/`](../backlog/README.md); promote one here when
work begins.

## Instructions

**Idea Capture**: For ideas not ready for formal planning, write a two-pager in `../ideas/`.

**Naming**: Plans in `in-progress/` use NO date prefix — just the slug (e.g., `organiclever-web-responsive-breakpoints/`). `backlog/` also uses no date prefix, so moving from `backlog/` is a pure move.

When starting work on a plan:

1. Move the plan folder: `git mv backlog/[identifier]/ in-progress/[identifier]/` (no rename — neither stage carries a date prefix)
2. Update the plan's README.md status to "In Progress"
3. Add the plan to this list

When completing a plan:

1. Rename and move: `git mv in-progress/[identifier]/ done/YYYY-MM-DD__[identifier]/` using today's completion date
2. Update this list
