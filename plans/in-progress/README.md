# In-Progress Plans

Active project plans currently being worked on.

## Active Plans

| Plan                                                  | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| ----------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [dependency-bump-2026-06](./dependency-bump-2026-06/) | Pre-approved dependency bump across four tiers per the Dependency Bump Stability & Safety Policy: migrate `rhino-cli` off the unmaintained `serde_yml` crate (RUSTSEC-2025-0068), floor tokio ≥ 1.51.0, refresh Node LTS (24.16.0) and the backend Debian runtime base (trixie-slim), migrate the `crane-cli` test stack to xunit.v3 + coverlet 8, remove deprecated `@hey-api/client-fetch`, and confirm-then-bump GitHub Actions major tags. Snapshot as of 2026-06-04; re-verify eligibility before execution. |

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
