# Adopt Hexagonal Architecture (All Apps) + DDD (BE Apps)

**Status**: Done
**Plan identifier**: `adopt-hexagonal-ddd-architecture`
**Worktree path**: `worktrees/adopt-hexagonal-ddd-architecture/`

## Context

This plan adopts hexagonal architecture across every CLI and client-facing web app in the
monorepo, and pairs it with Domain-Driven Design (DDD) for all backend apps. It also
establishes OpenAPI contract-first communication between each BE and its client.

The hexagonal architecture pattern separates business logic from infrastructure concerns by
defining explicit ports (interfaces) and adapters (implementations). DDD adds bounded contexts
and a rich domain model on top. The two patterns complement each other: hexagonal architecture
enforces dependency inversion; DDD gives the inner layers (domain and application) their
vocabulary and structure.

`organiclever-web` is the reference implementation for TypeScript hexagonal architecture. The
refactoring of all other web apps follows its pattern.

## Scope

**In scope:**

- Five governance convention documents (`repo-governance/development/pattern/`)
- Four CLI apps: `rhino-cli`, `crane-cli`, `ose-cli`, `ayokoding-cli` (all Rust)
- Five client-facing web apps: `organiclever-web` (gap-fill), `ose-app-web`, `wahidyankf-web`,
  `ayokoding-web`, `ose-web`
- Two backend apps: `organiclever-be` (Rust/Axum), `ose-app-be` (F#/Giraffe)
- OpenAPI contract infrastructure for `organiclever-be`↔`organiclever-web` and
  `ose-app-be`↔`ose-app-web`

**Out of scope:**

- E2E test refactoring (`*-e2e` projects)
- Business endpoint expansion in OpenAPI specs (only governance + initial codegen setup)
- New features or product functionality
- `libs/rust-commons/` (library, not an app)

## Business Rationale

See [brd.md](./brd.md) for full business requirements.

**Summary:** The current codebase mixes infrastructure concerns into domain logic across all
app types. This makes it harder to test, reason about, and extend. Hexagonal architecture
enforces a hard boundary; DDD gives the domain layer its own vocabulary. Together they reduce
defect rates, accelerate feature velocity, and unify the mental model across the polyglot stack.

## Product Requirements

See [prd.md](./prd.md) for full product requirements and Gherkin acceptance criteria.

## Technical Approach

See [tech-docs.md](./tech-docs.md) for architecture, design decisions, and per-language details.

**Layer canonical names by app type:**

| App type         | Layer 1 (innermost)    | Layer 2                     | Layer 3                        | Layer 4 (outermost)  |
| ---------------- | ---------------------- | --------------------------- | ------------------------------ | -------------------- |
| CLI (Rust)       | `domain/`              | `application/`              | `infrastructure/`              | `commands/`          |
| Web (TS/Next.js) | `domain/`              | `application/`              | `infrastructure/`              | `presentation/`      |
| BE (Rust/Axum)   | `contexts/<n>/domain`  | `contexts/<n>/application`  | `contexts/<n>/infrastructure`  | `contexts/<n>/http`  |
| BE (F#/Giraffe)  | `contexts/<n>/Domain/` | `contexts/<n>/Application/` | `contexts/<n>/Infrastructure/` | `contexts/<n>/Http/` |

## Delivery

See [delivery.md](./delivery.md) for the phased delivery checklist.

**Phases:**

- Phase 0: Environment Setup and Baseline (`repo-setup-manager`)
- Phase 1: Governance Convention Documents
- Phase 2: CLI App Refactoring (all four Rust CLIs)
- Phase 3: Web App Layer Completion (all five TS web apps)
- Phase 4: BE App Refactoring (`organiclever-be`, `ose-app-be`)
- Phase 5: OpenAPI Contract Infrastructure

## Navigation

| Document                       | Purpose                                                  |
| ------------------------------ | -------------------------------------------------------- |
| [brd.md](./brd.md)             | Business goals, impact, success metrics, risks           |
| [prd.md](./prd.md)             | User stories, Gherkin acceptance criteria, product scope |
| [tech-docs.md](./tech-docs.md) | Architecture, design decisions, per-language specifics   |
| [delivery.md](./delivery.md)   | Phased TDD delivery checklist                            |
