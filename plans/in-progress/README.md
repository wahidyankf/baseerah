# In-Progress Plans

Active project plans currently being worked on.

## Active Plans

| Plan                                                                                           | Description                                                                                                                                                                                    |
| ---------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [ose-web-remove-ddd](./ose-web-remove-ddd/README.md)                                           | Remove DDD scaffolding from `ose-web` (keep hexagonal feature modules); de-DDD `rhino-cli` allowlist.                                                                                          |
| [wahidyankf-web-remove-ddd-and-hexagonal](./wahidyankf-web-remove-ddd-and-hexagonal/README.md) | Remove DDD **and** hexagonal layout from `wahidyankf-web` (flatten `contexts/` to `features/`); add static-site opt-out clause to the hexagonal-web pattern doc; de-DDD `rhino-cli` allowlist. |

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
