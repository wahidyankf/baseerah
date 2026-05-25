# Product Requirements Document — organiclever-be Rust Migration

## Product Overview

Replace the Java/Spring Boot 4 implementation of `apps/organiclever-be/` with a functionally
equivalent Rust/Axum binary. The product must expose the same HTTP API surface that exists
today (health endpoint on `/api/v1/health`), respond to CORS preflight requests, and return
structured error responses — all verified by the existing Gherkin feature file at
`specs/apps/organiclever/behavior/be/gherkin/health/health-check.feature` [Repo-grounded].

## Personas

- **Operations engineer**: monitors service health via `GET /api/v1/health`; expects HTTP 200
  and `{"status": "ok"}` body regardless of caller identity (no auth required)
- **Backend developer (maintainer)**: develops and maintains `organiclever-be`; expects Rust
  idioms, clippy pedantic, cargo targets, and cucumber-rs BDD tests matching the
  `apps/rhino-cli/` pattern
- **Frontend / API consumer**: sends cross-origin requests from `organiclever-web`; expects CORS
  allow-all headers in responses

## User Stories

### US-1: Health Endpoint

As an operations engineer,
I want to send `GET /api/v1/health` and receive `{"status": "ok"}` with HTTP 200,
So that I can confirm the service is reachable without requiring authentication.

### US-2: CORS Preflight

As a frontend developer,
I want cross-origin requests to be allowed from any origin,
So that `organiclever-web` can call the API without CORS errors during development and production.

### US-3: Structured Error Responses

As an API consumer,
I want unhandled server errors to return a structured JSON error body with HTTP 500,
So that error causes are surfaced in a machine-readable format.

### US-4: Developer Experience — Cargo Targets

As the backend developer,
I want all Nx targets for `organiclever-be` to use `cargo` commands,
So that I can build, test, lint, and run the service with the same tooling as `rhino-cli`.

### US-5: BDD Test Parity

As the backend developer,
I want the existing Gherkin scenarios in
`specs/apps/organiclever/behavior/be/gherkin/health/health-check.feature`
to be covered by cucumber-rs step implementations in `tests/integration/main.rs`,
So that `nx run organiclever-be:spec-coverage` passes.

## Acceptance Criteria

### AC-1: Health Endpoint — Happy Path

```gherkin
Feature: Service Health Check

  Background:
    Given the Rust Axum server is running on port 8202

  Scenario: Health endpoint reports the service as UP
    When an operations engineer sends GET /api/v1/health
    Then the response status code should be 200
    And the response body JSON field "status" should equal "ok"

  Scenario: Anonymous health check does not expose component details
    When an unauthenticated engineer sends GET /api/v1/health
    Then the response status code should be 200
    And the response body JSON field "status" should equal "ok"
    And the response body should not contain "components"
```

Note: The existing feature file at
`specs/apps/organiclever/behavior/be/gherkin/health/health-check.feature` uses `"UP"` as
the status value (Java Spring Boot convention). The Rust port uses `"ok"` (matching the
ose-primer `crud-be-rust-axum` pattern). The Gherkin feature file MUST be updated to reflect
the new value as part of this migration (see delivery.md Phase 2). The canonical acceptance
criterion is HTTP 200 + JSON body with `status` field present and non-error.

### AC-2: CORS Allow-All

```gherkin
  Scenario: CORS preflight request is accepted from any origin
    When a browser sends OPTIONS /api/v1/health with Origin: https://example.com
    Then the response status code should be 200
    And the response should include header "Access-Control-Allow-Origin" with value "*"
```

### AC-3: Error Handling

```gherkin
  Scenario: Unhandled server panic returns structured error
    Given a route that triggers an internal error
    When a client calls that route
    Then the response status code should be 500
    And the response Content-Type should contain "application/json"
```

### AC-4: Nx Targets — Cargo

```gherkin
  Scenario: Build target uses cargo
    When the developer runs "nx run organiclever-be:build"
    Then the command executes "cargo build --release"
    And exits with code 0

  Scenario: Test:quick target uses cargo llvm-cov
    When the developer runs "nx run organiclever-be:test:quick"
    Then cargo llvm-cov runs and reports line coverage >= 90%
    And exits with code 0
```

### AC-5: Spec Coverage

```gherkin
  Scenario: spec-coverage passes after Rust migration
    Given the cucumber-rs step implementations in tests/integration/main.rs
      cover the scenarios in health-check.feature
    When the developer runs "nx run organiclever-be:spec-coverage"
    Then the rhino-cli spec-coverage tool exits with code 0
```

## Product Scope

### In-Scope Features

- `GET /api/v1/health` — returns `{"status": "ok"}` with HTTP 200, no auth required
- CORS middleware: allow all origins, methods, and headers (via `tower-http` CorsLayer)
- Global error handler: catches unhandled errors and returns HTTP 500 JSON
- Nx targets: `build`, `install`, `fmt`, `fmt:check`, `lint`, `deny:check`, `check:msrv`,
  `run`, `dev`, `typecheck`, `test:unit`, `test:quick`, `test:integration`, `spec-coverage`
- `.env.example` documenting `DATABASE_URL`, `PORT`, `CORS_ORIGINS`
- `docker-compose.integration.yml` adapted for Rust app service

### Out-of-Scope Features

- Any CRUD endpoints (users, tasks, expenses, etc.)
- Authentication or JWT handling
- Database migrations or schema management
- Rate limiting
- Request validation beyond what Axum provides by default
- Metrics or distributed tracing endpoints

## Product Risks

| Risk                                                                                                                                          | Mitigation                                                                                                                       |
| --------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| The Gherkin feature file uses `"UP"` (Java convention) but the Rust port returns `"ok"`                                                       | Update the feature file and step definitions as part of Phase 2; document the value change in the delivery checklist             |
| The `spec-coverage` Nx target in `project.json` currently matches `.java` files — updating to `.rs` files requires updating the `inputs` glob | Explicitly update `inputs` in the `spec-coverage` target during Phase 3                                                          |
| The OpenAPI codegen target (`-g java`) will be broken after Java source is deleted                                                            | Update or stub the codegen target in Phase 3; if a Rust generator is unavailable, mark the target as a no-op with a TODO comment |
