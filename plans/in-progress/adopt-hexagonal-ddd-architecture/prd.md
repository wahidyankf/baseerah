# PRD — Adopt Hexagonal Architecture + DDD

## Product Overview

This plan delivers five governance convention documents and a full codebase refactor that
installs hexagonal architecture across every CLI and client-facing web app, and DDD + hexagonal
architecture across every backend app, in the `ose-public` monorepo.

The product of this plan is not a user-visible feature — it is a structural substrate that
makes every future feature cheaper and safer to build.

## Personas

**App developer (maintainer hat)**: writes features after this plan completes and expects
clear placement rules for every new file.

**Repo governance agent**: executes delivery checklists, runs validators, and checks
convention compliance via `rhino-cli` and the Nx quality-gate targets.

**Plan checker / plan execution checker**: validates plan quality before execution and
verifies deliverables after execution.

## User Stories

### Governance documents

**US-1**: As an app developer, I want a single authoritative convention document that defines
hexagonal architecture in this repo so that I never have to guess where a new file belongs.

**US-2**: As an app developer, I want CLI-specific, web-specific, and BE-specific convention
documents so that the general pattern is instantiated for each language and app type.

**US-3**: As an app developer, I want an OpenAPI contract-first convention document so that
I know exactly how to add a new endpoint, wire codegen, and keep the spec from drifting.

### CLI apps

**US-4**: As an app developer, I want every Rust CLI to have `src/domain/`,
`src/application/`, `src/infrastructure/`, and `src/commands/` (with `rhino-cli` using
`src/internal/` for inner layers per its existing convention) so that business logic never
touches Clap argument parsing code.

### Web apps

**US-5**: As an app developer, I want every bounded context in every web app to have all four
layers (`domain/`, `application/`, `infrastructure/`, `presentation/`) so that domain types
never import React or Next.js.

**US-6**: As an app developer, I want `organiclever-web` gap-filled to the four-layer standard
in every context (currently `app-shell`, `health`, `landing`, `routing`, `stats`,
`workout-session` are missing one or more layers) so that the reference implementation is
itself complete.

### Backend apps

**US-7**: As an app developer, I want `organiclever-be` reorganized into
`src/contexts/<name>/{domain,application,infrastructure,http}/` modules so that each bounded
context has its own isolated domain model and the Axum router is confined to the `http/` layer.

**US-8**: As an app developer, I want `ose-app-be` reorganized into per-context subdirectory
structure inside `src/OseAppBe/contexts/<name>/{Domain,Application,Infrastructure,Http}/`
with F# compilation order enforcing the dependency rule (domain types compiled before http)
so that F# module loading never violates the dependency direction.

### OpenAPI contracts

**US-9**: As an app developer, I want `organiclever-be` to have a `codegen` Nx target that
generates Rust request/response types from the OpenAPI spec so that the server never handles
a shape the spec does not describe.

**US-10**: As an app developer, I want `organiclever-web` and `ose-app-web` to have `codegen`
Nx targets that generate TypeScript client types from the OpenAPI spec so that FE code never
calls an endpoint with a wrong payload shape.

**US-11**: As an app developer, I want CI to fail if generated contract output drifts from
the committed output so that spec drift is caught at pull-request time, not in production.

## Acceptance Criteria

### AC-1: Governance document — hexagonal architecture overview

```gherkin
Scenario: Core hexagonal architecture document exists and is complete
  Given the plan has been executed
  When I read repo-governance/development/pattern/hexagonal-architecture.md
  Then the document defines ports, adapters, domain, application, infrastructure, and presentation layers
  And it cross-links to the three specialization documents (CLI, web, BE)
  And it explains the dependency rule: outer layers depend on inner layers, never the reverse
  And markdownlint reports zero violations
```

### AC-2: Governance document — CLI hexagonal architecture

```gherkin
Scenario: CLI hexagonal architecture document exists and is complete
  Given the plan has been executed
  When I read repo-governance/development/pattern/hexagonal-architecture-cli.md
  Then the document specifies commands/ as the outermost inbound-adapter layer
  And it specifies domain/, application/, infrastructure/ as inner layers
  And it includes a directory layout table for Rust CLIs
  And markdownlint reports zero violations
```

### AC-3: Governance document — web hexagonal architecture

```gherkin
Scenario: Web hexagonal architecture document exists and is complete
  Given the plan has been executed
  When I read repo-governance/development/pattern/hexagonal-architecture-web.md
  Then the document specifies contexts/<name>/{domain,application,infrastructure,presentation}/ layout
  And it explains the Effect.ts Context.Tag port pattern for TypeScript
  And it explains Next.js Server Components as inbound adapters in the presentation layer
  And markdownlint reports zero violations
```

### AC-4: Governance document — BE hexagonal + DDD architecture

```gherkin
Scenario: BE hexagonal + DDD architecture document exists and is complete
  Given the plan has been executed
  When I read repo-governance/development/pattern/hexagonal-architecture-be.md
  Then the document specifies contexts/<name>/{domain,application,infrastructure,http}/ layout
  And it covers Rust trait-as-port pattern
  And it covers F# compilation-order constraint and railway-oriented programming
  And markdownlint reports zero violations
```

### AC-5: Governance document — OpenAPI contract-first

```gherkin
Scenario: OpenAPI contract-first document exists and is complete
  Given the plan has been executed
  When I read repo-governance/development/pattern/openapi-contract-first.md
  Then the document specifies spec location (specs/apps/<name>/containers/contracts/openapi.yaml)
  And it lists codegen tooling: hey-api/openapi-ts for TypeScript, openapi-generator for Rust
  And it specifies the drift enforcement mechanism (codegen in CI, fail on diff)
  And markdownlint reports zero violations
```

### AC-6: rhino-cli CLI layer conformance

```gherkin
Scenario: rhino-cli adopts hexagonal inner layers under src/internal/
  Given the plan has been executed
  When I list apps/rhino-cli/src/internal/
  Then I see domain/, application/, and infrastructure/ directories
  And the commands/ directory at apps/rhino-cli/src/commands/ remains unchanged
  When I run nx run rhino-cli:test:quick
  Then the command exits 0 with no new failures
```

### AC-7: crane-cli CLI layer conformance

```gherkin
Scenario: crane-cli adopts canonical hexagonal layer names
  Given the plan has been executed
  When I list apps/crane-cli/src/
  Then I see domain/ (renamed from core/), application/, infrastructure/ (renamed from adapters/), and commands/
  And I do not see core/ or adapters/ directories
  When I run nx run crane-cli:test:quick
  Then the command exits 0 with no new failures
```

### AC-8: ose-cli CLI layer conformance

```gherkin
Scenario: ose-cli adds missing inner layers
  Given the plan has been executed
  When I list apps/ose-cli/src/
  Then I see domain/, application/, and infrastructure/ directories alongside commands/
  When I run nx run ose-cli:test:quick
  Then the command exits 0 with no new failures
```

### AC-9: ayokoding-cli CLI layer conformance

```gherkin
Scenario: ayokoding-cli adds missing inner layers
  Given the plan has been executed
  When I list apps/ayokoding-cli/src/
  Then I see domain/, application/, and infrastructure/ directories alongside commands/
  When I run nx run ayokoding-cli:test:quick
  Then the command exits 0 with no new failures
```

### AC-10: organiclever-web all contexts have four layers

```gherkin
Scenario: Every organiclever-web context has all four hexagonal layers
  Given the plan has been executed
  When I list every subdirectory under apps/organiclever-web/src/contexts/
  Then each context directory contains domain/, application/, infrastructure/, and presentation/
  And no context directory contains only presentation/ or only a README.md
  When I run nx run organiclever-web:test:quick
  Then the command exits 0 with no new failures
```

### AC-11: ose-app-web all contexts have four layers

```gherkin
Scenario: Every ose-app-web context has all four hexagonal layers
  Given the plan has been executed
  When I list every subdirectory under apps/ose-app-web/src/contexts/
  Then each context directory contains domain/, application/, infrastructure/, and presentation/
  And no context directory contains only a README.md
  When I run nx run ose-app-web:test:quick
  Then the command exits 0 with no new failures
```

### AC-12: wahidyankf-web all contexts have four layers

```gherkin
Scenario: Every wahidyankf-web context has all four hexagonal layers
  Given the plan has been executed
  When I list every subdirectory under apps/wahidyankf-web/src/contexts/
  Then each context directory contains domain/, application/, infrastructure/, and presentation/
  When I run nx run wahidyankf-web:test:quick
  Then the command exits 0 with no new failures
```

### AC-13: ayokoding-web all contexts have four layers

```gherkin
Scenario: Every ayokoding-web context has all four hexagonal layers
  Given the plan has been executed
  When I list every subdirectory under apps/ayokoding-web/src/contexts/
  Then each context directory contains domain/, application/, infrastructure/, and presentation/
  When I run nx run ayokoding-web:test:quick
  Then the command exits 0 with no new failures
```

### AC-14: ose-web all contexts have four layers

```gherkin
Scenario: Every ose-web context has all four hexagonal layers
  Given the plan has been executed
  When I list every subdirectory under apps/ose-web/src/contexts/
  Then each context directory contains domain/, application/, infrastructure/, and presentation/
  When I run nx run ose-web:test:quick
  Then the command exits 0 with no new failures
```

### AC-15: organiclever-be bounded-context layout

```gherkin
Scenario: organiclever-be adopts DDD bounded-context module layout
  Given the plan has been executed
  When I list apps/organiclever-be/src/
  Then I see a contexts/ directory
  And each subdirectory under contexts/ contains domain/, application/, infrastructure/, and http/ modules
  And the top-level src/ no longer contains health/mod.rs, app.rs, or errors.rs as domain files
  When I run nx run organiclever-be:test:quick
  Then the command exits 0 with no new failures
```

### AC-16: ose-app-be per-context subdirectory layout

```gherkin
Scenario: ose-app-be adopts per-context subdirectory layout
  Given the plan has been executed
  When I list apps/ose-app-be/src/OseAppBe/contexts/
  Then each context subdirectory contains Domain/, Application/, Infrastructure/, and Http/ subdirectories
  And the OseAppBe.fsproj <Compile Include> entries are ordered domain-first, http-last for each context
  When I run nx run ose-app-be:test:quick
  Then the command exits 0 with no new failures
```

### AC-17: OpenAPI codegen wired for organiclever

```gherkin
Scenario: organiclever codegen targets generate types from spec
  Given the plan has been executed
  When I run nx run organiclever-be:codegen
  Then the command exits 0
  And Rust type files are generated in apps/organiclever-be/src/generated_contracts/ or equivalent
  When I run nx run organiclever-contracts:lint
  Then the command exits 0
```

### AC-18: OpenAPI codegen wired for ose-app

```gherkin
Scenario: ose-app-web codegen target generates TypeScript types from spec
  Given the plan has been executed
  When I run nx run ose-app-web:codegen
  Then the command exits 0
  And TypeScript type files are generated in apps/ose-app-web/src/generated-contracts/
```

## Product Scope

### In-scope features

- Five governance convention documents in `repo-governance/development/pattern/`
- Structural refactoring of four Rust CLI apps to adopt canonical hexagonal layer names
- Structural gap-filling of five TypeScript web apps to reach the four-layer standard in every
  bounded context
- Structural refactoring of `organiclever-be` (Rust/Axum) into DDD bounded-context modules
- Structural refactoring of `ose-app-be` (F#/Giraffe) into per-context subdirectory structure
- OpenAPI codegen tooling wired for `organiclever-be` (Rust server types) and
  `organiclever-web` / `ose-app-web` (TypeScript client types)
- CI drift gate: `codegen` exits non-zero if generated output differs from committed output

### Out-of-scope features

- New product features, UI screens, or API endpoints
- E2E test refactoring or new E2E scenarios
- Business endpoint expansion in OpenAPI specs
- Migration of `libs/rust-commons/` to a hexagonal structure
- `ose-primer` downstream template propagation

## Product-Level Risks

| Risk                                                                            | Mitigation                                                                                                                                                        |
| ------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Renaming Rust modules breaks all `use` import paths                             | Run `test:unit` immediately after each rename; fix all compile errors before moving to the next module                                                            |
| Empty placeholder directories are invisible to Git                              | Add `.gitkeep` to every new empty directory; verify with `git status` before each commit                                                                          |
| F# compilation-order mistakes cause build failures                              | Order `<Compile Include>` entries explicitly; run `nx run ose-app-be:build` after every file move                                                                 |
| Rust server-side OpenAPI codegen produces unusable output                       | Evaluate generated output quality during Phase 5; document findings in `tech-docs.md` and fall back to hand-written types if needed (annotated `[Judgment call]`) |
| DDD validator (`rhino-cli validate:specs-tree`) fails after web context changes | Run `nx run rhino-cli:validate:specs-tree` after each web app phase; fix any tree violations before proceeding                                                    |
