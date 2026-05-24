# In-Progress Plans

Active project plans currently being worked on.

## Active Plans

- [guard-env-file-access](./guard-env-file-access/README.md) — hard-block AI agents from directly reading/writing/editing `.env*` except `.env.example`; scripts under `apps`/`libs`/`scripts` exempt (PreToolUse hook + deny rules + convention)

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
