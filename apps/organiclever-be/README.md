# organiclever-be

F# / Giraffe / ASP.NET 10 REST API backend for OrganicLever. Ships one endpoint: health check.

## Quick Start

```bash
nx dev organiclever-be   # http://localhost:8202
```

## Commands

| Nx target                                 | What it does                                   |
| ----------------------------------------- | ---------------------------------------------- |
| `nx dev organiclever-be`                  | Dev server (localhost:8202)                    |
| `nx build organiclever-be`                | Production build (`dotnet publish`)            |
| `nx run organiclever-be:test:quick`       | DDD checks + unit tests + coverage (≥90% line) |
| `nx run organiclever-be:test:unit`        | Unit tests only                                |
| `nx run organiclever-be:test:integration` | Integration tests (Docker + real DB)           |
| `nx run organiclever-be:lint`             | F# strict lint (`TreatWarningsAsErrors`)       |
| `nx run organiclever-be:typecheck`        | `dotnet build` (type checks the project)       |
| `nx run organiclever-be:fmt`              | `fantomas .`                                   |
| `nx run organiclever-be:fmt:check`        | `fantomas --check .`                           |
| `nx run organiclever-be:specs:coverage`   | Gherkin step coverage (rhino-cli)              |

## Prerequisites

- **.NET 10 SDK**
- **Docker** (for `test:integration`)

## Environment Variables

| Variable       | Default                                                    | Description               |
| -------------- | ---------------------------------------------------------- | ------------------------- |
| `DATABASE_URL` | `postgres://postgres:postgres@localhost:5432/organiclever` | PostgreSQL connection URL |
| `PORT`         | `8202`                                                     | TCP port to listen on     |
| `CORS_ORIGINS` | `*`                                                        | Allowed CORS origins      |

See `.env.example` for a local template.

## Tech Stack

- **Language**: F# (.NET 10)
- **Web framework**: Giraffe (ASP.NET Core)
- **Database**: PostgreSQL
- **Port**: 8202 | **API base**: `/api/v1`
- **Architecture**: Hexagonal ports-and-adapters
- **Linting**: F# strict (`TreatWarningsAsErrors`) + G-Research.FSharp.Analyzers + Fantomas

## Behavior & Architecture

| Artifact      | Location                                                                                                              |
| ------------- | --------------------------------------------------------------------------------------------------------------------- |
| API reference | [specs/…/components/be/api.md](../../specs/apps/organiclever/components/be/api.md)                                    |
| Gherkin specs | [specs/…/behavior/organiclever-be/gherkin/](../../specs/apps/organiclever/behavior/organiclever-be/gherkin/README.md) |
| Deployment    | [specs/…/containers/deployment.md](../../specs/apps/organiclever/containers/deployment.md)                            |

## Related

- [organiclever-be-e2e](../organiclever-be-e2e/README.md) — Playwright BE E2E tests
- [specs/apps/organiclever/](../../specs/apps/organiclever/README.md) — full spec tree
