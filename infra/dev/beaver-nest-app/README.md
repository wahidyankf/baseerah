# BeaverNest Local Development Infrastructure

Docker Compose setup for running the BeaverNest stack locally.

## Services

| Service        | Port  | Description                   |
| -------------- | ----- | ----------------------------- |
| beaver-nest-be | 19320 | F# / Giraffe REST API backend |
| beaver-nest-fe | 19310 | Next.js 16 frontend (Phase 8) |

## Quick Start

```bash
# Start the backend (no .env file required today)
npm run beaver-nest:dev

# Restart with a fresh build
npm run beaver-nest:dev:restart
```

## Environment Variables

No required environment variables today. The backend runs the health endpoint without
configuration and is stateless.

## CI Variant

`docker-compose.ci.yml` is used in GitHub Actions for E2E tests. It overrides only what differs
from the default compose file (making `BEAVER_NEST_BE_PORT` and `BEAVER_NEST_BE_CORS_ORIGINS` explicit).

## beaver-nest-fe

The `beaver-nest-fe` service is active in both compose files, alongside `beaver-nest-be`. It was
commented out until `apps/beaver-nest-fe/` was scaffolded — see
[learnings.md](../../../plans/done/2026-07-31__baseerah-repo-reset/learnings.md) for the rationale.

## Behavior & Architecture

See [specs/apps/beaver-nest/system-context/README.md](../../../specs/apps/beaver-nest/system-context/README.md)
for the C4 system context.
