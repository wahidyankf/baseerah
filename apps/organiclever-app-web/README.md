# organiclever-app-web

Next.js 16 frontend for the OrganicLever life journal — local-first productivity tracker
with PGlite (Postgres-WASM, IndexedDB-backed) for in-browser data storage.

> **Status: Pre-Alpha.** Data model may change between versions without migration.

## Quick Start

```bash
nx dev organiclever-app-web   # http://localhost:3202
```

## Commands

| Nx target                                      | What it does                             |
| ---------------------------------------------- | ---------------------------------------- |
| `nx dev organiclever-app-web`                  | Dev server (localhost:3202)              |
| `nx build organiclever-app-web`                | Production build                         |
| `nx run organiclever-app-web:test:quick`       | Unit tests + coverage (70%) + DDD checks |
| `nx run organiclever-app-web:test:unit`        | Unit tests only                          |
| `nx run organiclever-app-web:test:integration` | Integration tests                        |
| `nx run organiclever-app-web:specs:coverage`   | Gherkin spec coverage                    |
| `nx run organiclever-app-web:lint`             | Lint with oxlint + ESLint                |
| `nx run organiclever-app-web:typecheck`        | TypeScript type check                    |

## Environment Variables

| Variable              | Scope       | Required | Description                                         |
| --------------------- | ----------- | -------- | --------------------------------------------------- |
| `ORGANICLEVER_BE_URL` | Server-only | No       | Backend base URL probed by `/system/status/be` only |

## Deployment

- **Staging**: served by Vercel from the `stag-organiclever-app-web` branch, which the
  scheduled `organiclever-app-test-local-deploy-stag.yml` workflow force-pushes from `main`
  after the local-stack test gate passes. The staging URL is kept **private** (Vercel
  Deployment Protection) — it lives only in the `organiclever-app-staging` GitHub
  Environment var `WEB_BASE_URL`, never in a tracked file.
- **Production**: `prod-organiclever-app-web` → `app.organiclever.com`. Production CD is
  **deferred** — no production-promotion workflow exists yet.
- **Deployer agent**: [`apps-organiclever-app-web-deployer`](../../.claude/agents/apps-organiclever-app-web-deployer.md).

## Project Layout

```
apps/organiclever-app-web/src/
├── app/          # Next.js App Router — thin page wrappers only
├── contexts/     # Bounded-context implementations (journal, routine, stats, …)
├── shared/       # Cross-context utilities (PgliteService, format-relative-time)
└── test/         # Vitest-cucumber step implementations
```

## Tech Stack

- **Next.js 16** — App Router, Server Components
- **PGlite** — Postgres-WASM, IndexedDB-backed; local-first, no backend required
- **XState v5** — UI shell and workout-session FSMs
- **Effect TS** — typed functional effects in infrastructure layer
- **Tailwind CSS v4** — utility-first CSS
- **`@open-sharia-enterprise/web-ui`** — shared component library

## Behavior & Architecture

| Artifact                     | Location                                                                                                                        |
| ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| Bounded-context architecture | [specs/…/components/app-web/architecture.md](../../specs/apps/organiclever/components/app-web/architecture.md)                  |
| Routes and screens           | [specs/…/components/app-web/routes-and-screens.md](../../specs/apps/organiclever/components/app-web/routes-and-screens.md)      |
| Design system                | [specs/…/components/app-web/design-system.md](../../specs/apps/organiclever/components/app-web/design-system.md)                |
| Ubiquitous language          | [specs/…/ddd/ubiquitous-language/](../../specs/apps/organiclever/ddd/ubiquitous-language/README.md)                             |
| Gherkin specs                | [specs/…/behavior/organiclever-app-web/gherkin/](../../specs/apps/organiclever/behavior/organiclever-app-web/gherkin/README.md) |

## Related

- [organiclever-app-web-e2e](../organiclever-app-web-e2e/README.md) — Playwright E2E tests
- [specs/apps/organiclever/](../../specs/apps/organiclever/README.md) — full spec tree
