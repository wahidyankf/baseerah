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

`apps/beaver-nest-be/.env.example` is the sole backend environment-key owner. Local development
uses an explicit loopback listener on port `19320`; the backend's default process contract is
loopback `127.0.0.1:19300`. Production-only host publication and durable bind-source values remain
empty placeholders in the template and are never committed here.

## CI Variant

`docker-compose.ci.yml` is used in GitHub Actions for E2E tests. It explicitly configures the
container listener as `0.0.0.0:19320`, a disposable `/tmp/beaver-nest` SQLite directory, and a
finite busy timeout; it does not introduce a second environment-key template.

## beaver-nest-fe

The `beaver-nest-fe` service is active in both compose files, alongside `beaver-nest-be`. It was
commented out until `apps/beaver-nest-fe/` was scaffolded — see
[learnings.md](../../../plans/done/2026-07-31__baseerah-repo-reset/learnings.md) for the rationale.

## Behavior & Architecture

See [specs/apps/beaver-nest/system-context/README.md](../../../specs/apps/beaver-nest/system-context/README.md)
for the C4 system context.
