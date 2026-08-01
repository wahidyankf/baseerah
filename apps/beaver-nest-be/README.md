# baseerah-be

F# / Giraffe / ASP.NET 10 REST API backend for Baseerah. Phase 1 hello-world quad: exactly two
`GET` routes and a JSON 404 handler. Stateless — no database, no in-memory store.

## Quick Start

```bash
nx dev baseerah-be   # http://localhost:19320
```

## Commands

| Nx target                             | What it does                                    |
| ------------------------------------- | ----------------------------------------------- |
| `nx dev baseerah-be`                  | Dev server (localhost:19320)                    |
| `nx build baseerah-be`                | Production build (`dotnet publish`)             |
| `nx run baseerah-be:test:quick`       | Typecheck + lint + unit tests + coverage (≥90%) |
| `nx run baseerah-be:test:unit`        | Unit tests only                                 |
| `nx run baseerah-be:test:integration` | In-process host boot test                       |
| `nx run baseerah-be:lint`             | F# strict lint (`TreatWarningsAsErrors`)        |
| `nx run baseerah-be:typecheck`        | `dotnet build` (type checks the project)        |
| `nx run baseerah-be:specs:coverage`   | Gherkin step coverage (rhino-cli)               |

## Prerequisites

- **.NET 10 SDK**

## Environment Variables

| Variable                   | Default | Description           |
| -------------------------- | ------- | --------------------- |
| `BASEERAH_BE_PORT`         | `19320` | TCP port to listen on |
| `BASEERAH_BE_CORS_ORIGINS` | `*`     | Allowed CORS origins  |

See `.env.example` for a local template.

## Tech Stack

- **Language**: F# (.NET 10)
- **Web framework**: Giraffe (ASP.NET Core)
- **Port**: 19320 | **API base**: `/api/v1`
- **Linting**: F# strict (`TreatWarningsAsErrors`) + G-Research.FSharp.Analyzers + Fantomas

## Behavior & Architecture

| Artifact      | Location                                                                                                  |
| ------------- | --------------------------------------------------------------------------------------------------------- |
| API reference | [specs/…/containers/contracts/](../../specs/apps/baseerah/containers/contracts/README.md)                 |
| Gherkin specs | [specs/…/behavior/baseerah-be/gherkin/](../../specs/apps/baseerah/behavior/baseerah-be/gherkin/README.md) |

## Related

- [specs/apps/baseerah/](../../specs/apps/baseerah/README.md) — full spec tree
