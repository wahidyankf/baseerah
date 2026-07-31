# Baseerah Local Development Infrastructure

Docker Compose setup for running the Baseerah stack locally.

## Services

| Service     | Port  | Description                   |
| ----------- | ----- | ----------------------------- |
| baseerah-be | 19320 | F# / Giraffe REST API backend |
| baseerah-fe | 19310 | Next.js 16 frontend (Phase 8) |

## Quick Start

```bash
# Start the backend (no .env file required today)
npm run baseerah:dev

# Restart with a fresh build
npm run baseerah:dev:restart
```

## Environment Variables

No required environment variables today. The backend runs the health endpoint without
configuration and is stateless.

## CI Variant

`docker-compose.ci.yml` is used in GitHub Actions for E2E tests. It overrides only what differs
from the default compose file (making `BASEERAH_BE_PORT` and `BASEERAH_BE_CORS_ORIGINS` explicit).

## baseerah-fe

The `baseerah-fe` service is commented out in both compose files until Phase 8 creates
`apps/baseerah-fe/`. See [learnings.md](../../../plans/in-progress/baseerah-repo-reset/learnings.md)
for the rationale.

## Behavior & Architecture

See [specs/apps/baseerah/system-context/README.md](../../../specs/apps/baseerah/system-context/README.md)
for the C4 system context.
