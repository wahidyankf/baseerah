# In-Progress Plans

Active project plans currently being worked on.

## Active Plans

| Plan                                                                               | Description                                                                                                                                                                                                                                                                                            |
| ---------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [standardize-repo-toolchain-parity](./standardize-repo-toolchain-parity/README.md) | Converge the whole repo toolchain (CI, git hooks, rhino-cli hexagonal arch + union commands, `{domain}:{work}` targets, Mermaid state-diagram validation, governance docs) across ose-public/ose-infra/ose-primer; A/B/E/F parallel-safe, C/D/G reference-first (public leads)                         |
| [rewrite-be-fsharp-drop-crane](./rewrite-be-fsharp-drop-crane/README.md)           | Rewrite `organiclever-be` + `ose-app-be` from Rust (Axum/sqlx/async-nats) to F# (Giraffe/EF Core 10/DbUp/NATS.Net) preserving the OpenAPI contracts, and remove the crane-be media service + PDF→Markdown feature entirely (3→2 GHCR images); upstream prerequisite for the ose-infra k3s deploy plans |

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
