# In-Progress Plans

Active project plans currently being worked on.

## Active Plans

- [Web Design Tester Agent](./web-design-tester-agent/) — Add a new `web-design-tester` agent completing the live-site advocate triad (correctness / usability / design), make the three testers reciprocally complement each other, rename the combined web workflow to `web-ux-test-fixing-planning`, and expand User-Facing Delivery Hardening Rule 15 into a three-tester near-end round for web-UI feature-change plans. Direct-on-`main` (no worktrees), topic-identical across all three sibling repos, with a `repo-rules-maker` consistency sweep per repo.

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
