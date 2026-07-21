# Backlog Plans

Full, ready-to-execute plans waiting to start. A plan lands here only when it has been **promoted
from a two-pager** in [`../ideas/`](../ideas/README.md) — i.e. its open questions have shrunk to ones
that genuinely need a full plan's depth to answer.

## Planned Projects

- [shared-course-library-and-learning-paths](./shared-course-library-and-learning-paths/README.md)
  — re-architects the fundamentally-strong curriculum into a shared, path-neutral course library
  (`/en/c/learn/courses/<course-id>`) consumed by converging path manifests at
  `/en/c/learn/paths/<path-id>` (`interview-ready`, `immediately-effective`, `fundamentally-strong`
  software-engineer paths). Delivery Mode: `worktree-to-pr`; strict double-zero PQG.
  Other candidate work lives as two-pager idea briefs in [`../ideas/`](../ideas/README.md); promote one
  here when it is ripe.

## Instructions

**Idea Capture**: For ideas not ready for formal planning, write a two-pager in
[`../ideas/`](../ideas/README.md) — not here.

**Naming**: Plans in `backlog/` use NO date prefix — just the slug (e.g.,
`doc-command-existence-validation/`). A date prefix is applied only when a plan is archived to
`done/`, where it records the completion date.

When promoting a two-pager to a plan:

1. Create folder: `[project-identifier]/`
2. Add standard files: README.md, brd.md, prd.md, tech-docs.md, delivery.md — carrying the
   two-pager's problem, scope, and open questions forward
3. Add the plan to this list
4. Delete the two-pager from `../ideas/` and drop its line from `../ideas/README.md`
