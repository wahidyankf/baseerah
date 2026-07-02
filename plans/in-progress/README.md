# In-Progress Plans

Active project plans currently being worked on.

## Active Plans

- [unify-rhino-cli-sdlc-parity](./unify-rhino-cli-sdlc-parity/README.md) — second pass of the
  standardize-rhino-cli-sdlc-parity effort; drives all three repos (incl. rhino-cli's own source) to a
  genuinely byte-identical structure, grounded in a fresh 2026-07-02 re-audit.

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
