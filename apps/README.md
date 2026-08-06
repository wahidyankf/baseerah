# Apps Folder

## Purpose

The `apps/` directory contains **deployable application projects** (executables). These are the final artifacts that can be run, deployed, and served to end users.

## Naming Convention

Apps follow the naming pattern: **`[domain]-[type]`** — see
[Monorepo Structure Reference](../docs/reference/monorepo-structure.md) for the full tier vocabulary
(`[domain]-www`, `[domain]-app-web`, `[domain]-be`, `[domain]-fe`) and language/framework
abbreviations.

### Current Apps

- `rhino-cli` - Repository management CLI tools - Rust application (ported from Go 2026-05-23)
- `beaver-nest-be` - BeaverNest backend REST API - F#/Giraffe/ASP.NET application (port 19320)
- `beaver-nest-be-e2e` - HTTP-driven E2E tests for `beaver-nest-be` - Playwright
- `beaver-nest-fe` - BeaverNest frontend - Next.js 16 App Router application (port 19310)
- `beaver-nest-fe-e2e` - Playwright FE E2E tests for `beaver-nest-fe`

Every prior app (`ose-www`, `ose-be`, `ayokoding-www`, `ayokoding-cli`, `ose-cli`, `crane-cli`,
`organiclever-www`, `organiclever-app-web`, `organiclever-be`, `wahidyankf-www`, and their `-e2e`
counterparts) was deleted by the 2026 BeaverNest repo reset — see the
[baseerah-repo-reset plan](../plans/done/2026-07-31__baseerah-repo-reset/README.md).

## Application Characteristics

- **Consumers** - Apps import and use libs, but don't export anything for reuse
- **Isolated** - Apps should NOT import from other apps
- **Deployable** - Each app is independently deployable
- **Specific** - Contains app-specific logic and configuration
- **Entry Points** - Has clear entry points (index.ts, main.ts, etc.)

## App Structure Examples

### Rust CLI Application (`rhino-cli`)

```
apps/rhino-cli/
├── src/                     # Rust source (main.rs, application/, domain/, infra/)
├── tests/                   # Integration tests + fixtures
├── target/                  # Build output (gitignored)
├── Cargo.toml               # Rust package manifest
├── project.json             # Nx project configuration
└── README.md                # App documentation
```

### F# Backend Application (`beaver-nest-be`)

```
apps/beaver-nest-be/
├── src/BeaverNestBe/           # F# source (Program.fs, WebApp.fs)
├── tests/                    # Unit + integration test projects
├── generated-contracts/      # Generated from beaver-nest-contracts OpenAPI spec (gitignored)
├── Dockerfile
├── project.json              # Nx project configuration
└── README.md                 # App documentation
```

### Next.js Application (`beaver-nest-fe`)

```
apps/beaver-nest-fe/
├── src/
│   ├── app/                      # Next.js App Router pages
│   ├── components/               # Reusable React components
│   └── generated-contracts/      # Generated from beaver-nest-contracts OpenAPI spec (gitignored)
├── public/                       # Static assets
├── next.config.ts                # Next.js configuration
├── tsconfig.json                 # TypeScript configuration
├── Dockerfile
├── project.json                  # Nx project configuration
└── README.md                     # App documentation
```

### Playwright E2E Test App (`beaver-nest-be-e2e`, `beaver-nest-fe-e2e`)

```
apps/beaver-nest-be-e2e/
├── playwright.config.ts         # Playwright configuration (baseURL, reporters)
├── package.json                 # Pinned @playwright/test dependency
├── tsconfig.json                # TypeScript config (extends workspace base)
├── project.json                 # Nx configuration
├── tests/
│   ├── e2e/                     # Feature-grouped specs
│   └── utils/                   # Shared request utilities
└── README.md                    # App documentation
```

## Nx Configuration (project.json)

Each app must have a `project.json` file with Nx configuration.

**Note**: This repository uses vanilla Nx (no plugins), so all targets use `command` (shorthand for `nx:run-commands`) to run standard build tools directly (Next.js, dotnet, cargo, etc.).

## How to Add a New App

See the how-to guide: [`docs/how-to/add-new-app.md`](../docs/how-to/add-new-app.md)

## Importing from Libraries

Apps can import from any library in `libs/` using path mappings:

```typescript
import { Button } from "@open-sharia-enterprise/web-ui";
import { colors } from "@open-sharia-enterprise/web-ui-token";
```

Path mappings are configured in the workspace `tsconfig.base.json` file.

## Running Apps

Use Nx commands to run apps:

```bash
# Development mode
nx dev beaver-nest-fe
nx run beaver-nest-be:dev

# Build for production
nx build beaver-nest-fe
nx build beaver-nest-be
nx build rhino-cli

# Run CLI applications
nx run rhino-cli

# Run E2E tests (target app must be running first)
nx run beaver-nest-fe-e2e:test:e2e
nx run beaver-nest-be-e2e:test:e2e
```

## Deployment Branches

`beaver-nest-fe` and `beaver-nest-be` have deployer agents and CI caller workflows wired but dormant — no
`prod-beaver-nest-fe`/`stag-beaver-nest-be` deploy target is provisioned yet. See
[apps-beaver-nest-fe-deployer](../.claude/agents/apps-beaver-nest-fe-deployer.md),
[apps-beaver-nest-be-deployer](../.claude/agents/apps-beaver-nest-be-deployer.md), and the
[beaver-nest-first-deploy](../plans/ideas/q2-not-urgent-important/beaver-nest-first-deploy.md) idea brief for the deferred
provisioning work.

## Language Support

Currently:

- **Rust** (CLI tools) - `rhino-cli`
- **F#** (backend API) - `beaver-nest-be`
- **TypeScript/Next.js** (frontend) - `beaver-nest-fe`
- **TypeScript/Playwright** (E2E testing) - `beaver-nest-be-e2e`, `beaver-nest-fe-e2e`
