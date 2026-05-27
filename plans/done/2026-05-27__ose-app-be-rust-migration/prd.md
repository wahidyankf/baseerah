# Product Requirements Document — ose-app-be Rust Migration

## Product Overview

Replace the F#/Giraffe implementation of `apps/ose-app-be/` with a Rust/Axum implementation
that is structurally identical to `apps/organiclever-be/`. The product delivered is a
compilable, tested, fully-wired Rust backend with one real bounded context (`health`) and four
empty stubs, all integrated into the Nx workspace and validated by the same quality gates used
across the monorepo.

## Personas

- **Platform engineer (maintainer)** — runs `nx run ose-app-be:test:quick`, `nx run
ose-app-be:lint`, and `nx run ose-app-be:typecheck` in CI and locally.
- **Backend engineer (maintainer)** — writes Rust code in `apps/ose-app-be/src/`.
- **System operator** — calls `GET /api/v1/health` to confirm the service is live.
- **Spec author (maintainer)** — reads `bounded-contexts.yaml` to navigate the DDD model.

## User Stories

**US-1 — Health endpoint**
As a system operator, I want `GET /api/v1/health` to return `{"status": "healthy"}` with HTTP
200, so that orchestrators can confirm the service is alive.

**US-2 — Rust toolchain**
As a backend engineer, I want `cargo build --release` and `cargo clippy -- -D warnings` to
pass with zero diagnostics, so that I can develop without fighting compiler noise.

**US-3 — Unit test coverage**
As a platform engineer, I want `nx run ose-app-be:test:quick` to validate ≥90% line coverage
via `cargo llvm-cov`, so that regressions are caught before push.

**US-4 — Stub contexts**
As a spec author, I want all five bounded contexts to have their Rust module hierarchy in place
(`domain/`, `application/`, `infrastructure/`, `api/` layers), so that future feature work can
start without scaffolding work. Note: `specs/apps/ose-app/ddd/bounded-contexts.yaml` lists four
stub contexts (`regulatory-source`, `internal-policy`, `gap-analysis`, `ai-orchestration`); the
`health` context is code-only and has no YAML entry.

**US-5 — DDD validation**
As a platform engineer, I want `rhino-cli ddd bc ose-app` and `ddd ul ose-app` to pass, so
that the code layout stays aligned with `bounded-contexts.yaml`.

**US-6 — No .NET artifacts**
As a platform engineer, I want no `.fsproj`, `global.json`, `dotnet-tools.json`, or F# source
files remaining in `apps/ose-app-be/`, so that the .NET toolchain is not a CI dependency.

## Acceptance Criteria

```gherkin
Feature: BE health endpoint

  # Step wording matches specs/apps/ose-app/behavior/be/gherkin/health/health.feature exactly
  Scenario: Health endpoint returns 200
    Given the ose-app-be service is running
    When I send GET /api/v1/health
    Then the response status is 200
    And the response body has a "status" field equal to "healthy"
```

The following additional acceptance criterion is verified manually (curl in Phase 7) but is
not yet captured in `health.feature`. If this behaviour needs automated Gherkin coverage, add
a second scenario to `health.feature` before execution:

```gherkin
  # Additional criterion — not yet in health.feature; verified via curl in Phase 7
  Scenario: Health endpoint returns JSON content type
    Given the ose-app-be service is running
    When I send GET /api/v1/health
    Then the response Content-Type header contains "application/json"
```

```gherkin
Feature: Rust toolchain quality gates

  Scenario: Release build is clean
    Given apps/ose-app-be/Cargo.toml is present
    When cargo build --release --manifest-path apps/ose-app-be/Cargo.toml runs
    Then the exit code is 0
    And no compiler warnings are emitted

  Scenario: Clippy pedantic passes
    Given apps/ose-app-be/Cargo.toml is present
    When cargo clippy --all-targets -- -D warnings runs
    Then the exit code is 0

  Scenario: Unit test coverage meets threshold
    Given unit tests exist under apps/ose-app-be/tests/unit/
    When cargo llvm-cov --test unit --fail-under-lines 90 runs
    Then the exit code is 0
    And line coverage is at least 90%
```

```gherkin
Feature: F# artifact removal

  Scenario: No F# project files remain
    Given the migration is complete
    When the apps/ose-app-be/ directory is inspected
    Then no .fsproj file is present
    And no global.json file is present
    And no dotnet-tools.json file is present
    And no fsharplint.json file is present
    And no src/OseAppBe/ directory is present
    And no tests/OseAppBe.Tests/ directory is present
    And no generated-contracts/OpenAPI/ directory is present
```

```gherkin
Feature: DDD bounded-context validation

  Scenario: bounded-contexts.yaml reflects Rust
    Given specs/apps/ose-app/ddd/bounded-contexts.yaml is updated
    When rhino-cli ddd bc ose-app runs
    Then the exit code is 0

  Scenario: ubiquitous language validation passes
    Given specs/apps/ose-app/ddd/ubiquitous-language/ files are present
    When rhino-cli ddd ul ose-app runs
    Then the exit code is 0
```

```gherkin
Feature: Stub context module hierarchy

  Scenario: Each stub context has all four layers
    Given the Rust project is scaffolded
    When the apps/ose-app-be/src/contexts/ directory is inspected
    Then regulatory-source/domain/mod.rs exists
    And regulatory-source/application/mod.rs exists
    And regulatory-source/infrastructure/mod.rs exists
    And regulatory-source/api/mod.rs exists
    And internal-policy/domain/mod.rs exists
    And internal-policy/application/mod.rs exists
    And internal-policy/infrastructure/mod.rs exists
    And internal-policy/api/mod.rs exists
    And gap-analysis/domain/mod.rs exists
    And gap-analysis/application/mod.rs exists
    And gap-analysis/infrastructure/mod.rs exists
    And gap-analysis/api/mod.rs exists
    And ai-orchestration/domain/mod.rs exists
    And ai-orchestration/application/mod.rs exists
    And ai-orchestration/infrastructure/mod.rs exists
    And ai-orchestration/api/mod.rs exists
```

## Product Scope

**In-scope features:**

- `GET /api/v1/health` returning `{"status": "healthy"}` with HTTP 200
- Rust/Axum project with strict clippy, deny.toml, llvm-cov, rust-toolchain.toml
- Five DDD bounded-context module hierarchies (1 real `health` + 4 stubs); four of these
  (`regulatory-source`, `internal-policy`, `gap-analysis`, `ai-orchestration`) are listed in
  `bounded-contexts.yaml`; `health` is code-only with no YAML entry in this plan
- Full Nx target set: `install`, `build`, `fmt`, `fmt:check`, `lint`, `deny:check`,
  `check:msrv`, `run`, `dev`, `typecheck`, `test:unit`, `test:quick`, `test:integration`,
  `spec-coverage`, `codegen`
- `.env.example` with all required env vars
- `docker-compose.integration.yml` and `Dockerfile.integration` for cucumber integration tests
- Updated `bounded-contexts.yaml` with `code_lang: [rs]`
- Updated `README.md`

**Out-of-scope features:**

- Any real implementation in `regulatory-source`, `internal-policy`, `gap-analysis`, or
  `ai-orchestration`
- OpenRouter HTTP client
- Database schema or migrations (empty `migrations/` only)
- `ose-app-be-e2e` Playwright test updates
- Rust-generated contracts from the OpenAPI spec

## Product-Level Risks

| Risk                                                                                                        | Impact                        | Mitigation                                                                                                                                                      |
| ----------------------------------------------------------------------------------------------------------- | ----------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Gherkin step `the health status should be "healthy"` uses a different string than `"ok"` in organiclever-be | High — test fails if wrong    | Health domain returns `"healthy"` not `"ok"`; unit test asserts the string explicitly                                                                           |
| `spec-coverage` command excludes four stub dirs — if flags are wrong, command exits non-zero                | Medium — CI failure           | Delivery step verifies exact command: `--exclude-dir regulatory-source --exclude-dir internal-policy --exclude-dir gap-analysis --exclude-dir ai-orchestration` |
| `bounded-contexts.yaml` `code` path still points to F# path after migration                                 | Medium — DDD validation fails | Delivery step updates both `code` and `code_lang` fields for all four YAML-listed contexts (`health` has no YAML entry and is not updated)                      |
