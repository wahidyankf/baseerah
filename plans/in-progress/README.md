# In-Progress Plans

Active project plans currently being worked on.

## Active Plans

| Plan                                                                                   | Description                                                                                                                                                          |
| -------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [remove-inactive-tech-stack-remnants](./remove-inactive-tech-stack-remnants/README.md) | Remove docs, agents, skills, CI gates, and toolchain artifacts for F#, C#, Java, Kotlin, Elixir, Clojure, Dart, Python — stacks no longer active in ose-public       |
| [rewrite-crane-cli-fsharp](./rewrite-crane-cli-fsharp/README.md)                       | Rewrite `apps/crane-cli/` from Rust to F# with strict hexagonal (ports-and-adapters) architecture; Impureim Sandwich pattern; TickSpec 2.0.5 + xUnit 2.9.2 BDD tests |

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
