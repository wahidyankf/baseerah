# In-Progress Plans

Active project plans currently being worked on.

## Active Plans

| Plan                                                                                         | Description                                                                                                          |
| -------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| [bootstrap-be-messaging-and-crane-media](./bootstrap-be-messaging-and-crane-media/README.md) | Bootstrap BE messaging (NATS), crane-be F# media service, GHCR publish workflow                                      |
| [standardize-app-spec-trees](./standardize-app-spec-trees/README.md)                         | Consolidate ose-app + ose-platform specs into one `specs/apps/ose/`; make one-tree-per-family the enforced standard  |
| [standardize-ci-parity](./standardize-ci-parity/README.md)                                   | Standardize GitHub Actions CI to full parity (except runner target) with ose-infra; anchor of a two-repo sibling set |

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
