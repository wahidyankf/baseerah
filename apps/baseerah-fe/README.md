# baseerah-fe

Next.js 16 hello-world frontend for Baseerah. Renders one route, `/`, showing the product name
and a greeting fetched live from `baseerah-be`.

## Quick Start

1. Install dependencies: `npm install`
2. Copy env: `cp .env.example .env.local`
3. Start `baseerah-be` (see `apps/baseerah-be/README.md`) or the full stack:
   `npm run baseerah:dev`
4. Start dev server: `nx dev baseerah-fe`
5. Open: <http://localhost:19310>

## Commands

| Command                         | Description                    |
| ------------------------------- | ------------------------------ |
| `nx dev baseerah-fe`            | Dev server (port 19310)        |
| `nx build baseerah-fe`          | Production build               |
| `nx run baseerah-fe:test:quick` | Unit tests + coverage          |
| `nx run baseerah-fe:typecheck`  | TypeScript typecheck           |
| `nx run baseerah-fe:lint`       | oxlint                         |
| `nx run baseerah-fe:codegen`    | Generate TS types from OpenAPI |

## Tech Stack

- **Next.js 16** — App Router, React 19
- **TypeScript** — Strict mode
- **Tailwind v4** — Styling
- **@open-sharia-enterprise/web-ui** — Shared component library
- **Vitest** — Unit testing

## Related

- [baseerah-be](../baseerah-be/README.md) — the backend this page fetches its greeting from
