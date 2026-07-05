# In-Progress Plans

Active project plans currently being worked on.

## Active Plans

- [upgrade-opencode-go-models](./upgrade-opencode-go-models/README.md) — bump the OpenCode secondary
  binding's model mapping off two stale/retired `opencode-go` model IDs onto a 3-tier mapping
  (thinking + execution → `opencode-go/glm-5.2`, fast → `opencode-go/minimax-m3`) across all 3
  repos' code, config, and governance docs; also pins Pi (`pi.dev`) to the same tiers in
  `ose-public`.

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
