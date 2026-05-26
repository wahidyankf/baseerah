# BRD — Adopt Hexagonal Architecture + DDD

## Business Goal

Establish a single, consistent architectural pattern across the entire `ose-public` monorepo:
hexagonal architecture for all app types, paired with Domain-Driven Design (DDD) for backend
apps. Every new feature written after this plan lands will live in correctly layered code with
no infrastructure bleed into domain logic.

## Business Impact

### Pain Points (current state)

- **Mixed concerns**: Domain logic currently sits alongside Axum handler code in
  `apps/organiclever-be/src/health/` (a single `mod.rs` contains the HTTP handler, not just
  domain logic). [Repo-grounded: verified via `Read apps/organiclever-be/src/health/mod.rs`]
- **Incomplete layers in web apps**: Five of nine web contexts in `organiclever-web` lack at
  least one of the four required layers. All five `ose-app-web` contexts have no inner layers
  at all — only a `README.md`. [Repo-grounded: verified via context directory audits]
- **Flat CLI structure**: `ose-cli` has only `commands/` and no `domain/`, `application/`, or
  `infrastructure/` modules. `crane-cli` uses non-canonical names (`core/` instead of
  `domain/`, `adapters/` instead of `infrastructure/`). [Repo-grounded: verified via `src/`
  directory listings]
- **No contract enforcement at build time**: The OpenAPI specs for both `organiclever-be` and
  `ose-app-be` contain only a health endpoint. There is no server-side type generation and no
  CI drift gate ensuring the spec stays in sync with implementation.
  [Repo-grounded: verified via `specs/apps/*/containers/contracts/`]
- **Inconsistent mental model**: A contributor switching between `rhino-cli`, `organiclever-web`,
  and `ose-app-be` currently encounters three different structural metaphors, raising the
  cognitive load for every change.

### Expected Benefits

- Single architectural vocabulary across all app types and languages in the repo.
- Domain logic is testable in isolation (no Axum, no Next.js, no file system in unit tests).
- OpenAPI contracts become the enforced BE↔client interface; drift is caught at CI time.
- New bounded contexts in BE apps have a clear structural home from day one.
- Governance convention documents give agents and contributors precise placement rules,
  eliminating ambiguity about where code belongs.

## Affected Roles

This is a solo-maintainer repository. The roles below describe the hats the maintainer wears:

- **App developer**: writes features in any of the four language stacks (Rust, TypeScript, F#,
  Go — no Go after the CLI migrations, but governance docs reference historical context)
- **Repo governance agent**: the AI coding agents that execute delivery checklists, check code
  quality, and validate conventions
- **Reviewer / plan checker**: validates that plan structure and delivery steps meet quality
  gates before execution starts

## Business-Level Success Metrics

The following metrics define "done" for this plan:

1. **Convention completeness** — Five governance convention documents exist at
   `repo-governance/development/pattern/hexagonal-architecture*.md` and
   `repo-governance/development/pattern/openapi-contract-first.md`.
   _Observable fact: `test -f` passes for all five paths after Phase 1._

2. **CLI layer conformance** — All four Rust CLIs (`rhino-cli`, `crane-cli`, `ose-cli`,
   `ayokoding-cli`) have `src/domain/`, `src/application/`, `src/infrastructure/`, and
   `src/commands/` (or the equivalent inner layout for `rhino-cli`, which uses
   `src/internal/` for inner layers). All existing tests continue to pass.
   _Observable fact: `nx run <cli>:test:quick` exits 0 for all four CLIs after Phase 2._

3. **Web app layer completeness** — Every bounded context in all five web apps has all four
   layers: `domain/`, `application/`, `infrastructure/`, `presentation/`. No context directory
   contains only a `README.md` or a subset of layers.
   _Observable fact: `nx run <app>:test:quick` exits 0 for all five apps after Phase 3._

4. **BE refactor completeness** — `organiclever-be` source is reorganized into
   `src/contexts/<name>/{domain,application,infrastructure,http}/` modules. `ose-app-be`
   source is reorganized into per-context subdirectory structure with F# files correctly
   ordered in `OseAppBe.fsproj`. All existing tests pass.
   _Observable fact: `nx run organiclever-be:test:quick` and `nx run ose-app-be:test:quick`
   both exit 0 after Phase 4._

5. **OpenAPI codegen wired** — `organiclever-be` has a `codegen` Nx target that generates
   Rust types from the OpenAPI spec. `organiclever-web` and `ose-app-web` have `codegen`
   targets that generate TypeScript types. CI fails if generated output drifts from committed
   output.
   _Observable fact: `nx run organiclever-contracts:lint` exits 0; `nx run organiclever-be:codegen`
   exits 0 after Phase 5._

6. **Zero regression** — Every project's `test:quick` target passes after each phase. No
   preexisting test failures are introduced by this plan's changes.

_Judgment call: these are binary observable outcomes, not growth KPIs. No numeric velocity
claims are made._

## Business-Scope Non-Goals

- **No new product features**: this plan restructures existing code; it does not add new
  business logic, UI screens, or API endpoints.
- **No E2E test refactoring**: `*-e2e` projects are excluded; structural refactoring of E2E
  test helpers is future work.
- **No business endpoint expansion in OpenAPI**: the specs already have a health endpoint.
  This plan wires codegen tooling; adding `/users`, `/workouts`, etc. is out of scope.
- **No `libs/rust-commons/` restructuring**: a shared library has different layering concerns
  than a deployable app; it is excluded.
- **No `ose-primer` propagation**: downstream template sync is a separate workflow and is not
  part of this plan.

## Business Risks and Mitigations

| Risk                                                             | Likelihood | Impact | Mitigation                                                                                                                                                                 |
| ---------------------------------------------------------------- | ---------- | ------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Rust module renames break `use` import paths across the codebase | High       | Medium | TDD-shaped steps: run `nx run <app>:test:unit` (RED) before renaming, fix all imports (GREEN), verify coverage (REFACTOR). Rename modules in one commit per app.           |
| F# fsproj compilation-order mistakes cause build failures        | Medium     | High   | Explicitly order every `<Compile Include="...">` entry in the fsproj after restructuring; run `nx run ose-app-be:build` immediately after each file move.                  |
| OpenAPI Rust server-side codegen quality is insufficient         | Medium     | Medium | Evaluate `openapi-generator` output during Phase 5; if quality is insufficient, fall back to hand-written type aliases and annotate the plan's tech-docs with the finding. |
| Incomplete layer directories confuse DDD validators              | Low        | Medium | After each web app phase, run `nx run rhino-cli:validate:specs-tree` to confirm DDD tree expectations still pass.                                                          |
| Layer-empty placeholder directories are not tracked by Git       | Low        | Low    | Add a `.gitkeep` file to every newly created empty directory. Use `git status` to verify all new dirs appear in the staging area before committing.                        |
