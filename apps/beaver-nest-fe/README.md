# beaver-nest-fe

Next.js 16 hello-world frontend for BeaverNest. Renders one route, `/`, showing the product name
and a greeting fetched live from `beaver-nest-be`.

## Quick Start

1. Install dependencies: `npm install`
2. Copy env: `cp .env.example .env.local`
3. Start `beaver-nest-be` (see `apps/beaver-nest-be/README.md`) or the full stack:
   `npm run beaver-nest:dev`
4. Start dev server: `nx dev beaver-nest-fe`
5. Open: <http://localhost:19310>

## Commands

| Command                            | Description                    |
| ---------------------------------- | ------------------------------ |
| `nx dev beaver-nest-fe`            | Dev server (port 19310)        |
| `nx build beaver-nest-fe`          | Production build               |
| `nx run beaver-nest-fe:test:quick` | Unit tests + coverage          |
| `nx run beaver-nest-fe:typecheck`  | TypeScript typecheck           |
| `nx run beaver-nest-fe:lint`       | oxlint                         |
| `nx run beaver-nest-fe:codegen`    | Generate TS types from OpenAPI |

## Tech Stack

- **Next.js 16** — App Router, React 19
- **TypeScript** — Strict mode
- **Tailwind v4** — Styling
- **@open-sharia-enterprise/web-ui** — Shared component library
- **Vitest** — Unit testing

## Related

- [beaver-nest-be](../beaver-nest-be/README.md) — the backend this page fetches its greeting from
