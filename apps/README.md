# Apps Folder

## Purpose

The `apps/` directory contains **deployable application projects** (executables). These are the final artifacts that can be run, deployed, and served to end users.

## Naming Convention

Apps follow the naming pattern: **`{domain}-{part}`**

Where `{part}` describes the role and technology stack:

| Part pattern            | Examples                                              | Description                              |
| ----------------------- | ----------------------------------------------------- | ---------------------------------------- |
| `be-{lang}-{framework}` | `be-golang-gin`, `be-java-springboot`, `be-ts-effect` | Backend service                          |
| `fe-{lang}-{framework}` | `fe-ts-nextjs`, `fe-dart-flutterweb`                  | Frontend application                     |
| `fs-{lang}-{framework}` | `fs-ts-nextjs`                                        | Fullstack application (FE + BE combined) |
| `cli`                   | `ayokoding-cli`, `rhino-cli`, `ose-cli`               | CLI tool                                 |
| `web`                   | `ayokoding-www`, `ose-www`                            | Web platform (content site)              |
| `{role}-e2e`            | `be-e2e`, `fe-e2e`, `organiclever-app-web-e2e`        | E2E test project for the named role      |
| `be` / `fe`             | `organiclever-be`, `organiclever-app-web`             | Simple single-technology projects        |

**Language abbreviations** (`{lang}`): `ts` (TypeScript), `golang` (Go), `java` (Java), `kt` (Kotlin),
`py` (Python), `rs` (Rust), `cs` (C#), `fs` (F#), `clj` (Clojure), `dart` (Dart), `ex` (Elixir).

**Framework abbreviations** (`{framework}`): `nextjs`, `gin`, `springboot`, `ktor`, `fastapi`, `axum`,
`aspnetcore`, `giraffe`, `pedestal`, `phoenix`, `vertx`, `effect`, `tanstack-start`, `flutterweb`.

### Current Apps

- `ose-www` - OSE Platform website ([oseplatform.com](https://oseplatform.com)) - Next.js 16 content platform (TypeScript, tRPC)
- `ose-www-be-e2e` - Playwright BE E2E tests for ose-www tRPC API
- `ose-www-fe-e2e` - Playwright FE E2E tests for ose-www UI
- `ayokoding-www` - AyoKoding educational platform ([ayokoding.com](https://ayokoding.com)) - Next.js 16 fullstack content platform (TypeScript, tRPC)
- `ayokoding-www-be-e2e` - Playwright BE E2E tests for ayokoding-www tRPC API
- `ayokoding-www-fe-e2e` - Playwright FE E2E tests for ayokoding-www UI
- `ayokoding-cli` - AyoKoding CLI tool for link validation - Go application
- `crane-cli` - Content Retrieval And Normalization Engine CLI for PDF-to-Markdown pipeline - F# application
- `rhino-cli` - Repository management CLI tools - Rust application (ported from Go 2026-05-23)
- `ose-cli` - OSE Platform CLI tool for link validation - Go application
- `organiclever-app-web` - OrganicLever app frontend (www.organiclever.com) - Next.js app (port 3202)
- `organiclever-be` - OrganicLever backend API (F#/Giraffe) - F# application (port 8202)
- `organiclever-app-web-e2e` - FE E2E tests for organiclever-app-web - Playwright (browser testing)
- `organiclever-be-e2e` - BE E2E tests for organiclever-be - Playwright (API testing)
- `wahidyankf-www` - Wahidyankf personal portfolio ([www.wahidyankf.com](https://www.wahidyankf.com)) - Next.js 16 app (port 3201)
- `wahidyankf-www-fe-e2e` - FE E2E tests for wahidyankf-www - Playwright-BDD with axe-core

## Application Characteristics

- **Consumers** - Apps import and use libs, but don't export anything for reuse
- **Isolated** - Apps should NOT import from other apps
- **Deployable** - Each app is independently deployable
- **Specific** - Contains app-specific logic and configuration
- **Entry Points** - Has clear entry points (index.ts, main.ts, etc.)

## App Structure Examples

### Next.js App (ose-www)

```
apps/ose-www/
├── content/                 # Markdown content files
├── src/                     # Application source code
│   ├── app/                 # Next.js App Router pages
│   ├── components/          # Reusable React components
│   └── lib/                 # Utility functions and helpers
├── public/                  # Static assets
├── next.config.ts           # Next.js configuration
├── tsconfig.json            # TypeScript configuration
├── vercel.json              # Deployment configuration
├── project.json             # Nx project configuration
└── README.md                # App documentation
```

### Go CLI Application (Current)

```
apps/ayokoding-cli/
├── cmd/                     # CLI commands
├── internal/                # Internal packages
├── dist/                    # Build output (gitignored)
├── main.go                  # Entry point
├── go.mod                   # Go module definition
├── project.json             # Nx project configuration
└── README.md                # App documentation
```

```
apps/rhino-cli/
├── cmd/                     # CLI commands
├── internal/                # Internal packages
├── dist/                    # Build output (gitignored)
├── main.go                  # Entry point
├── go.mod                   # Go module definition
├── project.json             # Nx project configuration
└── README.md                # App documentation
```

```
apps/ose-cli/
├── internal/                # Internal packages (links/)
├── cmd/                     # CLI commands
├── dist/                    # Build output (gitignored)
├── main.go                  # Entry point
├── go.mod                   # Go module definition
├── project.json             # Nx project configuration
└── README.md                # App documentation
```

### Playwright E2E Test App (Current)

```
apps/organiclever-be-e2e/
├── playwright.config.ts         # Playwright configuration (baseURL, reporters)
├── package.json                 # Pinned @playwright/test dependency
├── tsconfig.json                # TypeScript config (extends workspace base)
├── project.json                 # Nx configuration
├── tests/
│   ├── e2e/                     # Feature-grouped specs
│   └── utils/                   # Shared request utilities
└── README.md                    # App documentation
```

### Next.js Application (Current)

```
apps/organiclever-app-web/
├── src/
│   ├── app/                    # Next.js App Router pages
│   │   ├── dashboard/          # Dashboard route
│   │   ├── login/              # Login route
│   │   ├── api/                # API route handlers
│   │   ├── layout.tsx          # Root layout
│   │   └── page.tsx            # Root page
│   ├── components/             # Reusable React components
│   │   └── ui/                 # shadcn-ui component library
│   ├── contexts/               # Shared React contexts
│   ├── data/                   # JSON data files
│   └── lib/                    # Utility functions and helpers
├── public/                     # Static assets
├── components.json             # shadcn-ui configuration
├── next.config.mjs             # Next.js configuration
├── tailwind.config.ts          # TailwindCSS configuration
├── tsconfig.json               # TypeScript configuration
├── vercel.json                 # Vercel deployment configuration
├── project.json                # Nx project configuration
└── README.md                   # App documentation
```

### Future App Types

Kotlin, Python apps will have language-specific structures and tooling.

## Nx Configuration (project.json)

Each app must have a `project.json` file with Nx configuration.

**Next.js App Example** (`ose-www`):

```json
{
  "name": "ose-www",
  "sourceRoot": "apps/ose-www/src",
  "projectType": "application",
  "targets": {
    "dev": {
      "command": "next dev --port 3100",
      "options": {
        "cwd": "apps/ose-www"
      }
    },
    "build": {
      "command": "next build",
      "options": {
        "cwd": "apps/ose-www"
      },
      "outputs": ["{projectRoot}/.next"],
      "cache": true
    },
    "start": {
      "command": "next start --port 3100",
      "options": {
        "cwd": "apps/ose-www"
      }
    }
  },
  "tags": ["type:app", "platform:nextjs", "lang:ts", "domain:ose-platform"]
}
```

**Note**: This repository uses vanilla Nx (no plugins), so all targets use `command` (shorthand for `nx:run-commands`) to run standard build tools directly (Next.js, Go, etc.).

## How to Add a New App

See the how-to guide: `docs/how-to/add-new-app.md` (to be created)

## Importing from Libraries

Apps can import from any library in `libs/` using path mappings:

```typescript
// Future TypeScript apps will use path mappings like:
import { utils } from "@open-sharia-enterprise/ts-utils";
import { Button } from "@open-sharia-enterprise/ts-components";
```

Path mappings are configured in the workspace `tsconfig.base.json` file.

**Note**: Currently there are no libraries in `libs/`. Libraries will be created as shared functionality is identified.

## Running Apps

Use Nx commands to run apps:

```bash
# Development mode (Next.js)
nx dev ose-www
nx dev organiclever-app-web
nx dev ayokoding-www

# Build for production
nx build ose-www
nx build ayokoding-www
nx build ayokoding-cli
nx build rhino-cli
nx build organiclever-app-web

# Run CLI applications
nx run rhino-cli

# Clean build artifacts
nx clean ose-www

# Run E2E tests for organiclever-app-web (organiclever-app-web must be running first)
nx run organiclever-app-web-e2e:test:e2e

# Run API E2E tests (backend must be running first)
nx run organiclever-be-e2e:test:e2e
```

## Deployment Branches

Vercel-deployed apps use dedicated production branches (deployment-only — never commit directly):

| Branch                  | Production URL                                        | App                  |
| ----------------------- | ----------------------------------------------------- | -------------------- |
| `prod-ayokoding-web`    | [ayokoding.com](https://ayokoding.com)                | ayokoding-www        |
| `prod-ose-web`          | [oseplatform.com](https://oseplatform.com)            | ose-www              |
| `prod-organiclever-web` | [www.organiclever.com](https://www.organiclever.com/) | organiclever-app-web |

**ayokoding-www**: Deploy by force-pushing `main` to the production branch:

```bash
git push origin main:prod-ayokoding-web --force
```

**ose-www**: Deployed automatically by scheduled GitHub Actions
workflow (`ose-www-test-local-deploy-prod.yml`) running at 6 AM and 6 PM
WIB. The workflow detects changes scoped to the app directory before building and deploying.
Trigger on-demand from the GitHub Actions UI (set `force_deploy=true` to skip change detection).

**organiclever-app-web**: Deploy by force-pushing `main` to the production branch:

```bash
git push origin main:prod-organiclever-web --force
```

Use the corresponding deployer agent (e.g. `apps-organiclever-web-deployer`) for guided deployment.

## Language Support

Currently:

- **Go** (CLI tools) - ayokoding-cli, ose-cli
- **Rust** (CLI tools) - rhino-cli
- **TypeScript/Next.js** (web applications) - ose-www, organiclever-app-web, ayokoding-www
- **F#** (CLI tools, backend API) - crane-cli, organiclever-be
- **TypeScript/Playwright** (E2E testing) - organiclever-app-web-e2e, organiclever-be-e2e

Future: Kotlin, Python apps (each language will have language-specific structure and tooling)
