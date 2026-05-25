# organiclever-be Rust Migration

Migrate `apps/organiclever-be/` from Java/Spring Boot 4 to Rust (Axum + SQLx + PostgreSQL),
following the established ose-primer `crud-be-rust-axum` pattern and the rhino-cli linting
standards. Port scope is limited to what currently exists: health endpoint, CORS config, and
global exception handling.

## Context

`apps/organiclever-be/` currently runs on Java/Spring Boot 4 (JVM). The repo's platform
direction is Rust for backend services (see `apps/rhino-cli/` port completed 2026-05). The
ose-primer reference implementation `apps/crud-be-rust-axum/` provides a proven Axum + SQLx
pattern. This migration aligns `organiclever-be` with the Rust-first platform direction without
adding new features.

The Java source is small: one health controller (`GET /api/v1/health` returning
`{"status": "UP"}`), one CORS config (allow-all), and one global exception handler returning
RFC 9457 Problem Detail responses. Stale F# build artifacts (`src/OrganicLeverBe/`) are present
from a prior abandoned migration and will be cleaned up in Phase 0.

## Scope

**In scope:**

- Delete all Java-specific and stale F# artifacts from `apps/organiclever-be/`
- Create Rust project files: `Cargo.toml`, `rust-toolchain.toml`, `deny.toml`
- Implement Axum server: `src/main.rs`, `src/lib.rs`, `src/app.rs`, `src/config.rs`
- Port health endpoint (`GET /api/v1/health` → `{"status": "ok"}`) in `src/health/mod.rs`
- Port CORS config (allow-all via `tower-http`) in `src/app.rs`
- Port global error handling (`AppError` + `IntoResponse`) in `src/errors.rs`
- Unit tests in `tests/unit/main.rs` and cucumber-rs BDD tests in `tests/integration/main.rs`
- Update `apps/organiclever-be/project.json` Nx targets (cargo replaces mvn)
- Update `docker-compose.integration.yml` (Rust app service replaces Java)
- Update codegen target to generate Rust types instead of Java types
- Update Nx tags to `["type:app", "platform:axum", "lang:rust", "domain:organiclever"]`
- Add `.env.example` with `DATABASE_URL`, `PORT`, `CORS_ORIGINS`

**Out of scope:**

- New API endpoints beyond the health check
- Database CRUD operations (no entities exist in current Java)
- Authentication or authorization
- Deployment pipeline changes
- `organiclever-web`, `organiclever-be-e2e`, or `organiclever-web-e2e` changes
- Deleting the OpenAPI contract spec (it stays; only codegen generator changes)

## Approach Summary

1. **Phase 0** — Delete Java artifacts and stale F# artifacts; update `.gitignore`
2. **Phase 1** — Rust skeleton: `Cargo.toml`, `rust-toolchain.toml`, `deny.toml`, `main.rs`,
   `lib.rs`, `app.rs`, `config.rs`, `health/mod.rs`, `errors.rs`, `.env.example`
3. **Phase 2** — Tests: unit test harness + cucumber-rs BDD integration tests +
   `docker-compose.integration.yml` adaptation
4. **Phase 3** — Update `project.json` Nx targets; update codegen to Rust generator
5. **Phase 4** — Local quality gates (typecheck → lint → test:unit → test:quick → spec-coverage)
6. **Phase 5** — Manual API verification (curl against running server)
7. **Phase 6** — Post-push CI verification
8. **Phase 7** — Plan archival

## Documents

- [Business Requirements (BRD)](./brd.md)
- [Product Requirements (PRD)](./prd.md)
- [Technical Documentation](./tech-docs.md)
- [Delivery Checklist](./delivery.md)

## Status

In Progress
