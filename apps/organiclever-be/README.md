# organiclever-be

Rust/Axum REST API backend for OrganicLever. Ships one endpoint: health check.

## Quick Start

```bash
nx dev organiclever-be   # http://localhost:8202
```

## Commands

| Nx target                                 | What it does                               |
| ----------------------------------------- | ------------------------------------------ |
| `nx dev organiclever-be`                  | Dev server (localhost:8202)                |
| `nx build organiclever-be`                | Production build (`cargo build --release`) |
| `nx run organiclever-be:test:quick`       | DDD checks + llvm-cov (≥90% line)          |
| `nx run organiclever-be:test:unit`        | Unit tests only                            |
| `nx run organiclever-be:test:integration` | Integration tests (Docker + real DB)       |
| `nx run organiclever-be:lint`             | Clippy pedantic `-D warnings`              |
| `nx run organiclever-be:typecheck`        | `cargo check --all-targets`                |
| `nx run organiclever-be:fmt`              | `cargo fmt`                                |
| `nx run organiclever-be:fmt:check`        | `cargo fmt --check`                        |
| `nx run organiclever-be:deny:check`       | License + advisory check (`cargo deny`)    |
| `nx run organiclever-be:check:msrv`       | Compile with Rust 1.88 (MSRV)              |
| `nx run organiclever-be:specs:coverage`   | Gherkin step coverage (rhino-cli)          |

## Prerequisites

- **Rust toolchain 1.95.0** (pinned via `rust-toolchain.toml`; installed by `rustup`)
- **cargo-llvm-cov** (`cargo install cargo-llvm-cov --locked`)
- **cargo-deny** (`cargo install cargo-deny --locked`)
- **Docker** (for `test:integration`)

## Environment Variables

| Variable       | Default                                                    | Description               |
| -------------- | ---------------------------------------------------------- | ------------------------- |
| `DATABASE_URL` | `postgres://postgres:postgres@localhost:5432/organiclever` | PostgreSQL connection URL |
| `PORT`         | `8202`                                                     | TCP port to listen on     |
| `CORS_ORIGINS` | `*`                                                        | Allowed CORS origins      |

See `.env.example` for a local template.

## Tech Stack

- **Language**: Rust (edition 2024, MSRV 1.88)
- **Framework**: Axum 0.8.9
- **Database**: SQLx 0.8 + PostgreSQL
- **Port**: 8202 | **API base**: `/api/v1`
- **Testing**: cargo-llvm-cov (≥90% line coverage), cucumber-rs BDD integration tests
- **Lints**: `unsafe_code = "forbid"`, `missing_docs = "deny"`, clippy pedantic

## Behavior & Architecture

| Artifact      | Location                                                                                                              |
| ------------- | --------------------------------------------------------------------------------------------------------------------- |
| API reference | [specs/…/components/be/api.md](../../specs/apps/organiclever/components/be/api.md)                                    |
| Gherkin specs | [specs/…/behavior/organiclever-be/gherkin/](../../specs/apps/organiclever/behavior/organiclever-be/gherkin/README.md) |
| Deployment    | [specs/…/containers/deployment.md](../../specs/apps/organiclever/containers/deployment.md)                            |

## Related

- [organiclever-be-e2e](../organiclever-be-e2e/README.md) — Playwright BE E2E tests
- [specs/apps/organiclever/](../../specs/apps/organiclever/README.md) — full spec tree
