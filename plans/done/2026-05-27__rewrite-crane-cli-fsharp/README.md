# Rewrite crane-cli from Rust to F# (Hexagonal Architecture)

## Context

`apps/crane-cli/` is the Content Retrieval And Normalization Engine — an 11-subcommand CLI
that powers the PDF-to-Markdown conversion pipeline. It was originally written in F# (delivered
in plan `2026-05-15__crane-cli`) and then ported to Rust (plan `2026-05-26__crane-cli-rust-migration`).
The Rust port was expedient but the original F# implementation was idiomatic and well-tested. This
plan ports crane-cli back to F#, adopting the strict hexagonal (ports-and-adapters) architecture
convention and the Impureim Sandwich pattern, to align with future F# work on the platform.

The F# archived source at `archived/crane-cli/` [Repo-grounded] serves as the primary reference
for domain logic. The new implementation does **not** copy code wholesale — every module is rebuilt
TDD-first, and the architecture is elevated to strict hexagonal with explicit port type aliases in
`Core/Ports.fs`.

## Scope

**In-scope:**

- Archive the Rust source: `git mv` Rust-specific files to `archived/crane-cli-rust/`
- Create new F# project structure: `apps/crane-cli/crane-cli.fsproj`, `tests/unit/`, `tests/integration/`
- Implement all 11 CLI commands (pdf, text, heading, nesting, table, figure, mermaid, ocr,
  report, skiplist, check-all) with port parity to the Rust version
- Implement all domain logic: 10 Core/Logic modules, 2 Adapter/Out modules, 3 Domain types,
  1 Ports module
- Write unit tests (xUnit 2.9.2 + TickSpec 2.0.5) for all pure Core/Logic modules
- Write integration tests (TickSpec 2.0.5) consuming Gherkin feature files in
  `specs/apps/crane/behavior/cli/gherkin/` [Repo-grounded]
- Update `apps/crane-cli/project.json` Nx targets from `cargo` to `dotnet` commands
- Update `.github/workflows/crane-cli-integration.yml` to remove the `cargo` assumption
  (already uses `./.github/actions/setup-dotnet`) [Repo-grounded]
- Update `apps/crane-cli/README.md` to reflect the F# rewrite
- Update `repo-governance/development/pattern/hexagonal-architecture-cli.md` to document the
  F# hexagonal layout alongside the Rust layout [Repo-grounded]
- **Prerequisite**: Amend `plans/in-progress/remove-inactive-tech-stack-remnants/` to exclude
  dotnet/F#/C# artifacts from its Phase 1 scope

**Out-of-scope:**

- Changing the CLI user-facing interface (same subcommands, same flags, same exit codes)
- `ose-app-be` or `organiclever-be` (separate Rust apps, unaffected)
- `archived/crane-cli/` F# source (preserved as-is, used only as reference)
- Any other apps, libs, or governance files beyond those listed above
- New features not present in the current Rust implementation

## Document Navigation

| Document                       | Content                                                              |
| ------------------------------ | -------------------------------------------------------------------- |
| [brd.md](./brd.md)             | Business goal, impact, affected roles, success metrics, risks        |
| [prd.md](./prd.md)             | Product requirements, user stories, Gherkin acceptance criteria      |
| [tech-docs.md](./tech-docs.md) | Architecture, design decisions, dependency validation, F# hex layout |
| [delivery.md](./delivery.md)   | Phased TDD delivery checklist                                        |

## Status

In Progress
