# ose-app-web

Next.js 16 frontend for OSE Application (Governance, Risk, and Compliance) platform.

## Quick Start

1. Install dependencies: `npm install`
2. Copy env: `cp .env.example .env.local`
3. Start dev server: `nx dev ose-app-web`
4. Open: <http://localhost:3300>

## Commands

| Command                         | Description                    |
| ------------------------------- | ------------------------------ |
| `nx dev ose-app-web`            | Dev server (port 3300)         |
| `nx build ose-app-web`          | Production build               |
| `nx run ose-app-web:test:quick` | Unit tests + coverage          |
| `nx run ose-app-web:typecheck`  | TypeScript typecheck           |
| `nx run ose-app-web:lint`       | ESLint + oxlint                |
| `nx run ose-app-web:codegen`    | Generate TS types from OpenAPI |

## Tech Stack

- **Next.js 16** — App Router, React 19
- **TypeScript** — Strict mode
- **tRPC** — Type-safe API calls to ose-be
- **Tailwind v4** — Styling
- **@open-sharia-enterprise/web-ui** — Shared component library
- **Vitest** — Unit testing
- **Storybook** — Component development

## Deployment

- **Staging**: served by Vercel from the `stag-ose-app-web` branch, which the scheduled
  `ose-app-test-local-deploy-stag.yml` workflow force-pushes from `main` after the
  local-stack test gate passes. The staging URL is kept **private** (Vercel Deployment
  Protection) — it lives only in the `ose-app-staging` GitHub Environment var
  `WEB_BASE_URL`, never in a tracked file.
- **Production**: `prod-ose-app-web` → `app.oseplatform.com`. Production CD is **deferred** —
  no production-promotion workflow exists yet.
- **Deployer agent**: [`apps-ose-app-web-deployer`](../../.claude/agents/apps-ose-app-web-deployer.md).

## Related

- [ose-be](../ose-be/) — F#/Giraffe backend API
- [ose-be-e2e](../ose-be-e2e/) — BE E2E tests
- [ose-app-web-e2e](../ose-app-web-e2e/) — FE E2E tests
- [specs/apps/ose](../../specs/apps/ose/) — DDD specs and behavior
