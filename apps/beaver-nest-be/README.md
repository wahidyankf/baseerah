# beaver-nest-be

F# / Giraffe / ASP.NET 10 REST API backend for BeaverNest. Phase 1 hello-world quad: exactly two
`GET` routes and a JSON 404 handler. Stateless — no database, no in-memory store.

## Quick Start

```bash
nx dev beaver-nest-be   # http://localhost:19320
```

## Commands

| Nx target                                | What it does                                    |
| ---------------------------------------- | ----------------------------------------------- |
| `nx dev beaver-nest-be`                  | Dev server (localhost:19320)                    |
| `nx build beaver-nest-be`                | Production build (`dotnet publish`)             |
| `nx run beaver-nest-be:test:quick`       | Typecheck + lint + unit tests + coverage (≥90%) |
| `nx run beaver-nest-be:test:unit`        | Unit tests only                                 |
| `nx run beaver-nest-be:test:integration` | In-process host boot test                       |
| `nx run beaver-nest-be:lint`             | F# strict lint (`TreatWarningsAsErrors`)        |
| `nx run beaver-nest-be:typecheck`        | `dotnet build` (type checks the project)        |
| `nx run beaver-nest-be:specs:coverage`   | Gherkin step coverage (rhino-cli)               |

## Prerequisites

- **.NET 10 SDK**

## Environment Variables

| Variable                      | Default | Description           |
| ----------------------------- | ------- | --------------------- |
| `BEAVER_NEST_BE_PORT`         | `19320` | TCP port to listen on |
| `BEAVER_NEST_BE_CORS_ORIGINS` | `*`     | Allowed CORS origins  |

See `.env.example` for a local template.

## Tech Stack

- **Language**: F# (.NET 10)
- **Web framework**: Giraffe (ASP.NET Core)
- **Port**: 19320 | **API base**: `/api/v1`
- **Linting**: F# strict (`TreatWarningsAsErrors`) + G-Research.FSharp.Analyzers + Fantomas

## Behavior & Architecture

| Artifact      | Location                                                                                                           |
| ------------- | ------------------------------------------------------------------------------------------------------------------ |
| API reference | [specs/…/containers/contracts/](../../specs/apps/beaver-nest/containers/contracts/README.md)                       |
| Gherkin specs | [specs/…/behavior/beaver-nest-be/gherkin/](../../specs/apps/beaver-nest/behavior/beaver-nest-be/gherkin/README.md) |

## Related

- [specs/apps/beaver-nest/](../../specs/apps/beaver-nest/README.md) — full spec tree
