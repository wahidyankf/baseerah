# Backlog Plans

Planned projects for future implementation.

## Planned Projects

- [doc-command-existence-validation](./doc-command-existence-validation/README.md) — a rhino-cli
  validator that checks every command a doc claims to run against the commands the repo actually
  exposes (Nx targets, npm scripts, rhino-cli subcommands), closing the drift that left ~35 distinct
  cited `rhino-cli:<target>` names unresolvable and `links:validation` cited 40 times despite never
  having existed.

## Instructions

**Quick Idea Capture**: For 1-3 liner ideas not ready for formal planning, use `../ideas.md`.

**Naming**: Plans in `backlog/` use NO date prefix — just the slug (e.g.,
`doc-command-existence-validation/`). A date prefix is applied only when a plan is archived to
`done/`, where it records the completion date.

When creating a new plan:

1. Create folder: `[project-identifier]/`
2. Add standard files: README.md, brd.md, prd.md, tech-docs.md, delivery.md
3. Add the plan to this list
