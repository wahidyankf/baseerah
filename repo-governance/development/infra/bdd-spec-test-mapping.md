---
title: "BDD Spec-to-Test Mapping Convention"
description: Gherkin spec consumption rules for CLI apps (1:1 command mapping) and demo-be backends (three-level unit/integration/e2e)
category: explanation
subcategory: development
tags:
  - bdd
  - gherkin
  - integration-testing
  - specs:coverage
  - demo-be
created: 2026-03-06
---

# BDD Spec-to-Test Mapping Convention

This convention defines how Gherkin specifications are consumed across the monorepo:

- **CLI apps**: Mandatory 1:1 mapping between commands and Gherkin specs via the Rust test harness at both unit and integration test levels
- **Demo-be backends**: Three-level consumption of shared Gherkin specs (unit/integration/e2e) with different step implementations at each level

## Principles Implemented/Respected

This practice respects the following core principles:

- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**: Every command's behavior is explicitly specified in Gherkin before implementation. No undocumented commands.

- **[Automation Over Manual](../../principles/software-engineering/automation-over-manual.md)**: `rhino-cli specs coverage` automatically enforces the mapping at file, scenario, and step levels.

- **[Documentation First](../../principles/content/documentation-first.md)**: Specs are written alongside or before the command implementation, serving as living documentation.

## Conventions Implemented/Respected

- **[Acceptance Criteria Convention](./acceptance-criteria.md)**: Feature files follow Gherkin standards defined there, including the **step-keyword cardinality HARD rule** — every `Scenario` uses exactly one primary `Given`, one `When`, and one `Then`; additional steps chain with `And`/`But`. `Background` blocks and `Scenario Outline` `Examples` tables are exempt.

## CLI Apps: Command-to-Spec Mapping

### Core Rule

**Every Clap subcommand file must have a corresponding `@tag` in a Gherkin feature file under `specs/`.**

Infrastructure files (`main.rs`, `helpers.rs`) and parent command files (e.g., `agents.rs`, `docs.rs`) that do not implement logic are exempt.

## Domain-Prefixed Subcommands

All CLI apps in this monorepo use **Clap subcommands** grouped by domain. The domain is the prefix in every artifact:

```
rhino-cli {domain} {action}
ayokoding-cli {domain} {action}
ose-cli {domain} {action}
```

## Mapping Layers

The mapping operates at three levels:

### 1. Command to Tag (mandatory)

> **Scope note**: The file naming and tag derivation rules below apply to all Rust CLI apps
> (`ayokoding-cli`, `ose-cli`, `rhino-cli`). See the
> ["CLI App Families"](#cli-apps-dual-level-spec-consumption) section for `.rs` file patterns
> and test file locations.

The `@tag` is derived from the Rust filename: replace underscores with hyphens.

| Command File                | Full Invocation          | Feature `@tag`            |
| --------------------------- | ------------------------ | ------------------------- |
| `agents_sync.rs`            | `agents sync`            | `@agents-sync`            |
| `agents_validate_sync.rs`   | `agents validate-sync`   | `@agents-validate-sync`   |
| `agents_validate_claude.rs` | `agents validate-claude` | `@agents-validate-claude` |
| `docs_validate_links.rs`    | `docs validate-links`    | `@docs-validate-links`    |
| `doctor.rs`                 | `doctor`                 | `@doctor`                 |

### 2. Tag to Feature File (flexible)

A feature file may contain **multiple related commands** using separate `Rule` blocks with distinct `@tag` annotations. Semantically related commands (e.g., an action and its validator) can share a feature file:

```gherkin
Feature: Agent Configuration Synchronisation

  @agents-sync
  Rule: agents sync converts .claude/ configuration to .opencode/ format
    Scenario: Syncing converts agents and skills to secondary platform binding format
    ...

  @agents-validate-sync
  Rule: agents validate-sync confirms .claude/ and .opencode/ are equivalent
    Scenario: Directories that are in sync pass validation
    ...
```

Alternatively, a command with its own distinct domain gets its own feature file:

```
specs/apps/rhino/behavior/rhino-cli/gherkin/system/doctor.feature                       <- single @doctor tag
specs/apps/rhino/behavior/rhino-cli/gherkin/agents/agents-sync.feature                  <- @agents-sync + @agents-validate-sync
specs/apps/rhino/behavior/rhino-cli/gherkin/agents/agents-validate-claude.feature       <- single @agents-validate-claude tag
```

### 3. Unit & Integration Test to Tag (mandatory)

Each command has dedicated test files at both levels that filter scenarios by `@tag`. The same tag is used at both levels, pointing to the same feature file:

**Unit test** (inline `#[cfg(test)]` module — runs in `test:quick`):

```rust
// src/commands/agents_validate_sync_test.rs
#[test]
fn unit_agents_validate_sync() {
    // Runs the @agents-validate-sync scenarios from specs/ against the
    // command logic with all I/O mocked via injected function types.
}
```

**Integration test** (`tests/` integration target — runs in `test:integration`):

```rust
// tests/agents_validate_sync_integration_test.rs
#[test]
fn integration_agents_validate_sync() {
    // Same @agents-validate-sync scenarios, driven via process invocation
    // against real /tmp fixtures (different step implementations).
}
```

## File Naming Convention

| Artifact         | Pattern                                                                 | Example                                                             |
| ---------------- | ----------------------------------------------------------------------- | ------------------------------------------------------------------- |
| Parent cmd       | `{domain}.rs`                                                           | `agents.rs`                                                         |
| Command file     | `{domain}_{action}.rs`                                                  | `agents_validate_sync.rs`                                           |
| Unit test        | `{domain}_{action}_test.rs`                                             | `agents_validate_sync_test.rs`                                      |
| Integration test | `tests/{domain}_{action}_integration_test.rs`                           | `agents_validate_sync_integration_test.rs`                          |
| Feature file     | `specs/{app}/behavior/<product>-cli/gherkin/{domain}/{command}.feature` | `specs/apps/rhino/behavior/rhino-cli/gherkin/system/doctor.feature` |

**Unit test files** (`{domain}_{action}_test.rs`) serve dual purpose: they contain both Gherkin step definitions (consuming the command's `@tag` scenarios) and any non-BDD pure function tests for edge cases not covered by the Gherkin scenarios. The step definitions in unit test files use injected I/O function types instead of real filesystem access.

**The universal rule**: All Rust CLI files (command, unit test, integration test) use underscores. Feature files and `@tag`s use hyphens. The `rhino-cli specs coverage` tool normalises hyphens to underscores when matching feature stems to Rust test files.

## Coverage Enforcement

The `rhino-cli specs coverage` command enforces this mapping at three levels:

1. **File-level**: Every `.feature` file must have a matching `*_test.*` file
2. **Scenario-level**: Every `Scenario:` in the feature must appear as `// Scenario:` comment or `Scenario(...)` call in test code
3. **Step-level**: Every Given/When/Then step must have a matching step definition

Run the check:

```bash
rhino-cli specs coverage specs/apps/rhino apps/rhino-cli
```

**Scope**: Spec-coverage enforcement is currently active for **CLI apps only** (Rust + cucumber-rs naming
conventions). Enforcement for demo-be backends is **planned but deferred** — the tool needs
enhancement to support demo-be test file naming conventions (e.g., `health_steps.rs` for Rust)
which differ from the CLI app naming patterns the tool currently expects. This will be addressed in a follow-up plan.

## Adding a New Command

### Rust CLI apps (ayokoding-cli, ose-cli, rhino-cli)

1. Create the feature file `specs/apps/{app}/behavior/<product>-cli/gherkin/{domain}/{domain}-{action}.feature`
2. Create `apps/{app}/src/commands/{domain}_{action}.rs` with the Clap subcommand (register in `main.rs`)
3. Create `apps/{app}/src/commands/{domain}_{action}_test.rs` (or inline `#[cfg(test)]` module) with unit step definitions — mock I/O via injected function types, no special build tag (runs in `test:quick`)
4. Create `apps/{app}/tests/{domain}_{action}_integration_test.rs` with integration steps — drive via process invocation against real `/tmp` fixtures
5. Verify: `rhino-cli specs coverage specs/apps/{app-spec-dir} apps/{app}`

## CLI Apps: Dual-Level Spec Consumption

All Rust CLI apps (`ayokoding-cli`, `ose-cli`, `rhino-cli`) consume Gherkin specs at both the unit and integration test levels. The same feature files serve as the contract for both levels — only the step implementations differ.

### Architecture

| Level       | Nx Target          | Test File Pattern                                 | Step Implementation                             | Dependencies    |
| ----------- | ------------------ | ------------------------------------------------- | ----------------------------------------------- | --------------- |
| Unit        | `test:unit`        | `src/commands/{domain}_{action}_test.rs` (no tag) | Injected function types mock all I/O            | All mocked      |
| Integration | `test:integration` | `tests/{domain}_{action}_integration_test.rs`     | Process invocation against real `/tmp` fixtures | Real filesystem |

### Unit-Level Step Definitions

Unit steps call command logic directly with mocked dependencies. Injected function types (e.g., `readFileFn`, `writeFileFn`, `statFn`) are overridden in step setup to inject controlled behavior without touching the real filesystem.

- No special build tag — included in `cargo test` and `test:quick`
- Coverage is measured at this level (≥90% line coverage)
- Must run all Gherkin scenarios for the command's `@tag`

### Integration-Level Step Definitions

Integration steps drive commands via process invocation against controlled `/tmp` filesystem fixtures. Steps create temporary directory structures, invoke the command binary, and assert on stdout/stderr and exit code.

- Runs via `test:integration` target
- Coverage is NOT measured at this level
- Must run all Gherkin scenarios for the command's `@tag`

### Example: Same Spec, Two Step Implementations

The `@agents-validate-sync` tag lives inside `agents-sync.feature` (shared feature file) and is consumed at both levels:

```
specs/apps/rhino/behavior/rhino-cli/gherkin/agents/agents-sync.feature  (contains @agents-sync + @agents-validate-sync)
  -> Unit steps in:       apps/rhino-cli/src/commands/agents_validate_sync_test.rs
  -> Integration steps in: apps/rhino-cli/tests/agents_validate_sync_integration_test.rs
```

## API Backend: Three-Level Spec Consumption

API backends consume shared Gherkin scenarios from their own `specs/apps/<backend-name>/behavior/<product>-be/gherkin/`
directory at three test levels. The feature files are the shared contract — only the step
implementations change per level.

### Shared Specs

```
specs/apps/<backend-name>/behavior/<product>-be/gherkin/
├── auth/
│   ├── login.feature
│   ├── register.feature
│   └── ...
├── resources/
│   ├── list-items.feature
│   └── ...
└── ... (see gherkin README for full list)
```

### Three Levels

| Level           | Nx Target          | Step Implementations                                        | Dependencies             | What's Real            |
| --------------- | ------------------ | ----------------------------------------------------------- | ------------------------ | ---------------------- |
| **Unit**        | `test:unit`        | Call service/repository functions directly with mocked deps | All mocked               | Application logic only |
| **Integration** | `test:integration` | Call service/repository functions directly with real DB     | Real PostgreSQL (Docker) | Application + database |
| **E2E**         | `test:e2e`         | Playwright HTTP requests to running server                  | Full running server      | Everything             |

### Unit-Level Step Definitions

Unit steps call application service/repository functions directly. All dependencies (database, external APIs) are mocked via in-memory implementations or test doubles.

- No HTTP framework, no database connections
- Steps instantiate services with mocked repositories
- Coverage is measured at this level (≥90% line coverage)
- Must run all shared scenarios

### Integration-Level Step Definitions

Integration steps call application service/repository functions directly against a real PostgreSQL database via docker-compose. No HTTP layer.

- `docker-compose.integration.yml` starts PostgreSQL + test runner
- `Dockerfile.integration` contains language runtime + test execution
- Steps connect to PostgreSQL, run migrations, execute all shared scenarios
- Coverage is NOT measured at this level
- Must run all shared scenarios

### E2E-Level Step Definitions

E2E tests live in a dedicated `*-e2e` Playwright project. Steps make real HTTP requests to a running backend via `playwright-bdd`.

- Tests the full HTTP API contract
- Must run all shared scenarios

### Validation

To verify all scenarios pass at each level for a given backend:

```bash
# Unit tests (mocked dependencies)
nx run <backend-name>:test:unit

# Integration tests (real PostgreSQL via docker-compose)
nx run <backend-name>:test:integration

# E2E tests (Playwright HTTP against running backend)
nx run <backend-name>-e2e:test:e2e
```

All three commands must report all scenarios passing. The Gherkin feature files serve as the single source of truth — if a scenario fails at any level, the backend is non-compliant.

## Related Documentation

- [Acceptance Criteria Convention](./acceptance-criteria.md) - Gherkin format standards
- [Specs Directory Structure Convention](../../conventions/structure/specs-directory-structure.md) - Canonical path patterns and domain subdirectory rules
- [Three-Level Testing Standard](../quality/three-level-testing-standard.md) - Mandatory isolation boundaries for unit, integration, and E2E levels where Gherkin specs are consumed
- [Nx Target Standards](./nx-targets.md) - `test:integration` target definitions and caching rules
- [specs/README.md](../../../specs/README.md) - Spec directory organization
- [specs/apps/rhino/README.md](../../../specs/apps/rhino/README.md) - rhino-cli spec structure
