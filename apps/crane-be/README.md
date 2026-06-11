# crane-be

NATS + HTTP PDF-to-Markdown service (Content Retrieval And Normalization Engine backend).
F# / Giraffe / ASP.NET 10. Hexagonal architecture.

## Quick Start

```bash
nx dev crane-be   # http://localhost:8300
```

## Commands

| Nx target                          | What it does                                           |
| ---------------------------------- | ------------------------------------------------------ |
| `nx dev crane-be`                  | Dev server (localhost:8300)                            |
| `nx build crane-be`                | Production build (`dotnet publish`)                    |
| `nx run crane-be:test:quick`       | llvm-cov coverage (≥95% line)                          |
| `nx run crane-be:test:unit`        | TickSpec @unit scenarios                               |
| `nx run crane-be:test:integration` | TickSpec @integration (filesystem, no NATS)            |
| `nx run crane-be:lint`             | fsharplint + dotnet format                             |
| `nx run crane-be:typecheck`        | `dotnet build`                                         |
| `nx run crane-be:spec-coverage`    | Gherkin step coverage (rhino-cli, excludes messaging/) |

## Architecture

- **Core**: `CraneCore` library (`libs/fsharp-crane-core`) for PDF-to-Markdown logic
- **HTTP adapter**: `POST /media/pdf-to-md` — accepts PDF bytes, returns Markdown with
  `Content-Type: text/markdown`
- **NATS adapter**: subscribes `crane.convert` (queue group `crane.workers`) on two
  independent connections (organiclever + ose-app)
- **Health**: `GET /health` — stateless liveness check

## Environment Variables

| Variable                         | Required | Default | Description                             |
| -------------------------------- | -------- | ------- | --------------------------------------- |
| `CRANE_BE_PORT`                  | No       | `8300`  | TCP listen port                         |
| `CRANE_BE_ORGANICLEVER_NATS_URL` | Yes      | —       | NATS URL for OrganicLever connection    |
| `CRANE_BE_OSE_APP_NATS_URL`      | Yes      | —       | NATS URL for OSE Application connection |

See `.env.example` for annotated defaults.

## Prerequisites

- **.NET 10 SDK** (`dotnet --version`)
- **Tesseract OCR** (`tesseract --version`) — for integration tests against real PDFs
- **Docker** (for `crane-be-e2e` compose stack)

## Tech Stack

- **Language**: F# (net10.0)
- **Framework**: Giraffe 8.2.0
- **NATS**: NATS.Net 2.7.3
- **PDF**: PdfPig + Tesseract (via `libs/fsharp-crane-core`)
- **Port**: 8300

## Behavior and Specification

| Artifact       | Location                                                                                         |
| -------------- | ------------------------------------------------------------------------------------------------ |
| Gherkin specs  | [specs/…/behavior/crane-be/gherkin/](../../specs/apps/crane/behavior/crane-be/gherkin/README.md) |
| Component docs | [specs/…/components/be/](../../specs/apps/crane/components/be/README.md)                         |

## Related

- [crane-be-e2e](../crane-be-e2e/README.md) — Playwright + NATS E2E tests
- `libs/fsharp-crane-core/` — shared PDF-to-Markdown library
- [specs/apps/crane/](../../specs/apps/crane/README.md) — full spec tree
