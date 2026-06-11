# ose-app-be

Rust/Axum REST API backend for the OSE Application (Governance, Risk and Compliance) platform.

## Quick Start

```bash
# Run development server (localhost:8302)
nx dev ose-app-be

# Run unit tests
nx run ose-app-be:test:unit
```

## Commands

| Command                              | Description                                            |
| ------------------------------------ | ------------------------------------------------------ |
| `nx dev ose-app-be`                  | Start development server on localhost:8302             |
| `nx build ose-app-be`                | Production build (`cargo build --release`)             |
| `nx run ose-app-be:test:quick`       | DDD validation + unit tests + llvm-cov coverage (≥90%) |
| `nx run ose-app-be:test:unit`        | Unit tests only                                        |
| `nx run ose-app-be:test:integration` | Integration tests via Docker Compose (cucumber BDD)    |
| `nx run ose-app-be:lint`             | Clippy with `-D warnings`                              |
| `nx run ose-app-be:fmt:check`        | Rustfmt format check                                   |
| `nx run ose-app-be:typecheck`        | `cargo check --all-targets`                            |
| `nx run ose-app-be:spec-coverage`    | Validate BDD spec coverage via rhino-cli               |
| `nx run ose-app-be:deny:check`       | License and vulnerability audit via cargo-deny         |

## Prerequisites

- Rust 1.95 (managed via `rust-toolchain.toml`)
- `cargo-llvm-cov` (`cargo install cargo-llvm-cov`)
- `cargo-deny` (`cargo install cargo-deny`)
- Docker (for integration tests)
- Volta + Node.js (for `nx` commands)

## Environment Variables

Copy `.env.example` to `.env` and fill in values:

| Variable              | Description                                   |
| --------------------- | --------------------------------------------- |
| `DATABASE_URL`        | PostgreSQL connection URL                     |
| `PORT`                | TCP port to listen on (default: `8302`)       |
| `CORS_ORIGINS`        | Allowed CORS origins (default: `*`)           |
| `OPENROUTER_API_KEY`  | OpenRouter API key (never commit real key)    |
| `OPENROUTER_MODEL`    | Model identifier (default: `openrouter/auto`) |
| `OPENROUTER_BASE_URL` | OpenRouter base URL                           |

## Tech Stack

- **Language**: Rust (edition 2024, MSRV 1.88)
- **Web framework**: Axum 0.8
- **Database**: PostgreSQL via SQLx 0.8
- **Testing**: cargo-llvm-cov (unit) + cucumber-rs (integration BDD)
- **Linting**: Clippy pedantic + cargo-deny

## Architecture

Five DDD bounded contexts (hexagonal layout):

- **`health`** — liveness endpoint (`GET /api/v1/health`)
- **`regulatory-source`** — stub (feature plan pending)
- **`internal-policy`** — stub (feature plan pending)
- **`gap-analysis`** — stub (feature plan pending)
- **`ai-orchestration`** — stub (feature plan pending; uses OpenRouter)

> **Note**: The four stub contexts (`regulatory-source`, `internal-policy`, `gap-analysis`,
> `ai-orchestration`) are tracked in `specs/apps/ose/ddd/bounded-contexts.yaml`. The
> `health` context is code-only and has no YAML entry.

## Related

- **Specs**: `specs/apps/ose/`
- **Contracts**: `specs/apps/ose/containers/contracts/` (OpenAPI 3.1)
- **E2E tests**: `apps/ose-app-be-e2e/`
