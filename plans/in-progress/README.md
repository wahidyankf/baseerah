# In-Progress Plans

Active project plans currently being worked on.

## Active Plans

- [plan-execution-knowledge-capture](./plan-execution-knowledge-capture/) — Capture generalizable learnings from every plan execution and route them into durable knowledge surfaces (docs/rules/agents/skills/code) across all 3 repos. **Executes first.**
- [worktree-to-pr-default-delivery-mode](./worktree-to-pr-default-delivery-mode/) — Flip the default plan-delivery mode from worktree→main to worktree→PR (four named modes + precedence) across all 3 repos. **Executes second** (depends on knowledge-capture).

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
