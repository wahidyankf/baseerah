# beaver-nest-be

F# / Giraffe / ASP.NET 10 REST API backend for BeaverNest. The application uses an explicit
listener and SQLite directory contract; real values remain in uncommitted local environment files.

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

| Variable                                          | Default                | Description                                            |
| ------------------------------------------------- | ---------------------- | ------------------------------------------------------ |
| `BEAVER_NEST_BE_HTTP_LISTEN_ADDRESS`              | `127.0.0.1`            | Listener address; containers explicitly use `0.0.0.0`. |
| `BEAVER_NEST_BE_HTTP_LISTEN_PORT`                 | `19300`                | Listener port; Nx development explicitly uses `19320`. |
| `BEAVER_NEST_BE_DEVELOPMENT_DATA_DIRECTORY`       | —                      | Required developer-owned local SQLite directory.       |
| `BEAVER_NEST_BE_DATA_DIRECTORY`                   | `/var/lib/beaver-nest` | In-process SQLite directory.                           |
| `BEAVER_NEST_BE_HOST_DATA_DIRECTORY`              | —                      | Production Compose durable-data bind source.           |
| `BEAVER_NEST_BE_SQLITE_BUSY_TIMEOUT_MILLISECONDS` | `1000`                 | Finite SQLite lock wait.                               |
| `BEAVER_NEST_BE_VPN_HOST_IP`                      | —                      | Explicit production host publication address.          |
| `BEAVER_NEST_BE_PUBLIC_PORT`                      | `19300`                | Production-facing host port.                           |
| `BEAVER_NEST_BE_BACKUP_DIRECTORY`                 | —                      | Production Compose backup bind source.                 |

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
