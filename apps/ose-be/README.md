# ose-be

F# / Giraffe / ASP.NET 10 REST API backend for the OSE Application (Governance, Risk and Compliance) platform.

## Quick Start

```bash
# Run development server (localhost:8302)
nx dev ose-be

# Run unit tests
nx run ose-be:test:unit
```

## Commands

| Command                          | Description                                                |
| -------------------------------- | ---------------------------------------------------------- |
| `nx dev ose-be`                  | Start development server on localhost:8302                 |
| `nx build ose-be`                | Production build (`dotnet publish`)                        |
| `nx run ose-be:test:quick`       | DDD validation + unit tests + coverage (≥90%)              |
| `nx run ose-be:test:unit`        | Unit tests only                                            |
| `nx run ose-be:test:integration` | Integration tests via Docker Compose (real HTTP + real DB) |
| `nx run ose-be:lint`             | F# strict lint (`TreatWarningsAsErrors`)                   |
| `nx run ose-be:fmt:check`        | Fantomas format check                                      |
| `nx run ose-be:typecheck`        | `dotnet build` (type checks the project)                   |
| `nx run ose-be:specs:coverage`   | Validate BDD spec coverage via rhino-cli                   |

## Prerequisites

- .NET 10 SDK
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

- **Language**: F# (.NET 10)
- **Web framework**: Giraffe (ASP.NET Core)
- **Database**: PostgreSQL
- **Architecture**: Hexagonal ports-and-adapters
- **Linting**: F# strict (`TreatWarningsAsErrors`) + G-Research.FSharp.Analyzers + Fantomas

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
- **E2E tests**: `apps/ose-be-e2e/`
