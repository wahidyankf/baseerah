# In-Progress Plans

Active project plans currently being worked on.

## Active Plans

| Plan                                                                                           | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| ---------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [restructure-fsharp-be-and-web-app-tiers](./restructure-fsharp-be-and-web-app-tiers/README.md) | Rewrite both backends from Rust to F# (Giraffe/EF Core 10/DbUp/NATS.Net) + drop crane media (3→2 GHCR images, published early to unblock ose-infra k3s); make `organiclever-app-be` a real backend (minimal journal CRUD); split + rename the organiclever web tier to the `*-app-*` family (new simple `organiclever-web` marketing + `organiclever-app-web`); add a shared `libs/ts-ui` consumed by all four frontends; simplify the marketing sites to the wahidyankf-web pattern; restructure the matching specs/. Prod cutover deferred downstream |

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
