# In-Progress Plans

Active project plans currently being worked on.

## Active Plans

- [fundamentally-strong-shared-course-tracks](./fundamentally-strong-shared-course-tracks/README.md)
  — re-architects the fundamentally-strong curriculum into a shared, path-neutral course library
  (`/en/c/learn/courses/<course-id>`) consumed by three converging path manifests at
  `/en/c/learn/paths/<path-id>` — `interview-ready/software-engineer` (interview-first),
  `immediately-effective/software-engineer` (build-fast-first), and
  `fundamentally-strong/software-engineer` (theory-first, complete mastery). 121 courses composed
  curated-and-converging (121/119/116 per path) over one prerequisite DAG; legacy `_index.md`
  browse preserved additively. Delivery Mode: `worktree-to-pr`. PQG: strict double-zero.
  Build work follows the governance plan below.

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
