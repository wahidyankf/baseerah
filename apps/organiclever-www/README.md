# organiclever-www

Next.js 16 marketing website for the OrganicLever productivity platform.

## Quick Start

```bash
nx dev organiclever-www   # http://localhost:3200
```

## Commands

| Nx target                                | What it does                       |
| ---------------------------------------- | ---------------------------------- |
| `nx dev organiclever-www`                | Dev server (localhost:3200)        |
| `nx build organiclever-www`              | Production build                   |
| `nx run organiclever-www:test:quick`     | Unit tests + coverage + DDD checks |
| `nx run organiclever-www:test:unit`      | Unit tests only                    |
| `nx run organiclever-www:specs:coverage` | Gherkin spec coverage              |
| `nx run organiclever-www:lint`           | Lint with oxlint + ESLint          |
| `nx run organiclever-www:typecheck`      | TypeScript type check              |

## Tech Stack

- **Framework**: Next.js 16 (App Router)
- **Language**: TypeScript
- **Styling**: Tailwind CSS v4 + `@open-sharia-enterprise/web-ui`
- **Deployment**: Vercel via `prod-organiclever-www` branch
- **Dev port**: 3200

## Related

- [organiclever-www-fe-e2e](../organiclever-www-fe-e2e/README.md) — Playwright FE E2E tests
- [organiclever-www-be-e2e](../organiclever-www-be-e2e/README.md) — Playwright BE E2E slot (placeholder)
- [specs/apps/organiclever/](../../specs/apps/organiclever/README.md) — full spec tree
