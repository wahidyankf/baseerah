---
title: Monorepo Structure Reference
description: Complete reference for the Nx monorepo structure, folder organization, and file formats
category: reference
tags:
  - nx
  - monorepo
  - architecture
  - structure
created: 2025-11-29
---

# Monorepo Structure Reference

Complete reference for the Nx monorepo structure, folder organization, and file formats.

## Overview

This project uses **Nx** as a monorepo build system with a plugin-free "vanilla Nx" approach. The Nx monorepo consists of two main folders:

- `apps/` - Deployable applications
- `libs/` - Reusable libraries (flat structure with language prefixes)

## Root Structure

```
open-sharia-enterprise/
├── apps/                      # Deployable applications (Nx monorepo)
├── libs/                      # Reusable libraries (Nx monorepo, flat structure)
├── docs/                      # Documentation (Diátaxis framework)
├── plans/                     # Project planning documents
├── .claude/                   # Claude Code configuration
├── infra/                     # Infrastructure configurations
│   ├── dev/                  # Local development Docker Compose files per service
│   │   └── [service]/        # docker-compose.yml for local dev environment
│   └── k8s/                  # Kubernetes deployments
├── specs/                     # Gherkin acceptance specs, C4 diagrams, and OpenAPI contracts
│   ├── apps/                  # Per-app specs (C4-aware five-folder layout)
│   │   └── [domain]/         # e.g. rhino/, beaver-nest/ (planned)
│   │       ├── product/      # PM-first content (overview, roadmap)
│   │       ├── system-context/ # C4 L1 — system boundary diagram
│   │       ├── containers/   # C4 L2 — runtime containers + OpenAPI contracts
│   │       ├── components/   # C4 L3 — internal structure (be/, web/, cli/)
│   │       ├── ddd/          # Domain-Driven Design artefacts (when adopted)
│   │       └── behavior/     # Gherkin feature files (be/, web/, cli/)
│   │           └── [surface]/gherkin/[domain]/ # e.g. behavior/<product>-cli/gherkin/system/
│   └── libs/                  # Per-library specs
├── .husky/                    # Git hooks
├── .nx/                       # Nx cache (gitignored)
├── node_modules/              # Dependencies (gitignored)
├── nx.json                    # Nx workspace configuration
├── tsconfig.base.json         # Base TypeScript configuration
├── package.json               # Workspace manifest with npm workspaces
├── package-lock.json          # Dependency lock file
├── .dockerignore              # Docker build context exclusions (web app)
├── .nxignore                  # Files to exclude from Nx processing
├── .gitignore                 # Git ignore rules
├── commitlint.config.js       # Commit message validation
├── CLAUDE.md                  # Claude Code guidance
└── README.md                  # Project README
```

## Apps Folder (`apps/`)

### Purpose

Contains deployable application projects (executables).

### Location

`apps/` at repository root

### Organization

Flat structure - all apps at the same level, no subdirectories.

### Naming Convention

`[domain]-[type]`

**Current Apps** (as of the 2026 BeaverNest repo reset, see
[baseerah-repo-reset plan](../../plans/done/2026-07-31__baseerah-repo-reset/README.md)):

- `rhino-cli` - Repository management CLI (Rust application). Ported from Go 2026-05-23 (predecessor Go source recoverable from git history).
- `beaver-nest-fe` - BeaverNest Vite CSR client (local development port 19310)
- `beaver-nest-fe-e2e` - Playwright FE E2E tests for beaver-nest-fe
- `beaver-nest-be` - BeaverNest F#/Giraffe/ASP.NET same-origin runtime (local API port 19320; combined port 19300)
- `beaver-nest-be-e2e` - E2E tests for beaver-nest-be

Every other app previously listed here (`ose-www`, `ose-be`, `ayokoding-www`, `ayokoding-cli`, `ose-cli`, `crane-cli`, `organiclever-www`, `organiclever-app-web`, `organiclever-be`, `wahidyankf-www`, and their `-e2e` counterparts) was deleted by the BeaverNest repo reset. The structural examples below use generic `apps/<app-name>/` placeholders except where `rhino-cli` illustrates a real, currently-scaffolded app.

### App Structure (Vite CSR Application — e.g. `beaver-nest-fe`)

```
apps/<app-name>/
├── src/                       # Source code (App Router)
├── public/                    # Static assets
├── next.config.mjs            # Next.js configuration
├── project.json               # Nx project configuration
├── vercel.json                # Deployment configuration (if deployed via Vercel)
└── README.md                  # App documentation
```

A Next.js app may also add `.storybook/` (Storybook) and a `Dockerfile` (containerized deployment) depending on its deployment target.

### App Structure (Rust CLI Application — rhino-cli)

```
apps/rhino-cli/
├── src/                       # Source code
│   ├── commands/              # CLI command handlers
│   ├── domain/                # Domain logic
│   ├── application/           # Application services
│   ├── infrastructure/        # Adapters (I/O, HTTP)
│   ├── cli.rs                 # CLI argument parsing
│   ├── lib.rs                 # Library root
│   └── main.rs                # Entry point
├── tests/                     # Integration tests
├── target/                    # Build output (gitignored)
├── Cargo.toml                 # Rust package manifest
├── rust-toolchain.toml        # Pinned Rust toolchain
├── project.json               # Nx project configuration
└── README.md                  # App documentation
```

### App Structure (F#/Giraffe Application — planned, e.g. `beaver-nest-be`)

```
apps/<app-name>/
├── src/                       # Source code (F# modules)
├── tests/                     # Test suites (unit/, integration/)
├── Dockerfile                 # Production multi-stage build
├── .dockerignore              # Docker build context exclusions
├── *.fsproj                   # F# project file
├── project.json               # Nx project configuration
└── README.md                  # App documentation
```

### App Characteristics

- **Consumers** - Apps import and use libs, don't export for reuse
- **Isolated** - Apps should NOT import from other apps
- **Deployable** - Each app is independently deployable
- **Specific** - Contains app-specific logic and configuration
- **Entry Points** - Has clear entry point (index.ts, main.ts, etc.)

## Libs Folder (`libs/`)

### Purpose

Contains reusable library packages.

### Location

`libs/` at repository root

### Organization

**Flat structure** - All libraries at the same level, no nested scopes.

### Naming Convention

`[language-prefix]-[name]`

**Language Prefixes**:

- `ts-` - TypeScript (e.g., `web-ui`, `web-ui-token`)
- `rust-` - Rust (e.g., `rust-commons`)
- `fsharp-` - F# (future; `fsharp-crane-core` was deleted 2026 alongside `crane-cli`, its sole consumer, by the BeaverNest repo reset)
- `java-` - Java (future)
- `kt-` - Kotlin (future)
- `py-` - Python (future)

**Current Libraries**:

- `rust-commons` - Shared Rust utilities (link-checking, HTTP)
- `web-ui` - Shared React component library (shadcn/ui patterns, Radix UI primitives, Tailwind CSS)
- `web-ui-token` - Shared design tokens for `web-ui`

**Examples** (planned):

- `ts-utils` - TypeScript utility functions
- `ts-components` - Reusable React components
- `ts-hooks` - Custom React hooks
- `ts-api` - API client libraries
- `ts-validators` - Data validation functions

### Library Structure (TypeScript)

```
libs/ts-utils/
├── src/
│   ├── index.ts               # Public API (barrel export)
│   └── lib/                   # Implementation
│       ├── greet.ts           # Feature implementation
│       └── greet.test.ts      # Unit tests
├── dist/                      # Build output (gitignored)
│   ├── index.js               # Compiled JavaScript
│   ├── index.d.ts             # Type definitions
│   └── lib/                   # Compiled lib files
├── project.json               # Nx project configuration
├── tsconfig.json              # TypeScript configuration
├── tsconfig.build.json        # Build-specific TS config
├── package.json               # Library metadata and dependencies
└── README.md                  # Library documentation
```

### Library Characteristics

- **Polyglot-Ready** - Designed for multiple languages (TypeScript now, Java/Kotlin/Python future)
- **Flat Structure** - All libs at same level, no nested scopes
- **Reusable** - Designed to be imported by apps and other libs
- **Focused** - Each lib has single, clear purpose
- **Public API** - Exports controlled through `index.ts` (barrel export)
- **Testable** - Can be tested independently

### Current Scope

Rust (`rust-commons`) and TypeScript (`web-ui`, `web-ui-token`) libraries. No F# library currently exists;
`fsharp-crane-core` was deleted by the BeaverNest repo reset (it was `crane-cli`-specific, not generic).

## Nx Monorepo Projects (`apps/` and `libs/`)

**Purpose**: Integrated projects (TypeScript, Rust, F#) that benefit from shared tooling and workspace integration.

**Characteristics**:

- Managed by Nx workspace configuration
- Integrated build system with task caching and orchestration
- Shared TypeScript configuration (`tsconfig.base.json`)
- Workspace path mappings (`@open-sharia-enterprise/*`)
- Cross-project dependencies supported
- Unified testing and linting commands
- Affected detection (`nx affected -t build`, `nx affected -t test:quick`)
- Dependency graph visualization (`nx graph`)

**When to use**:

- TypeScript applications and libraries
- Projects that share code with other monorepo projects
- Projects that benefit from task caching
- Projects that need unified build/test/lint workflows

**Examples**:

- Next.js frontend applications
- F#/Giraffe backend services
- Rust CLI tools
- Reusable Rust, F#, and TypeScript libraries

## File Format Reference

### `project.json` (Nx Configuration)

Location: `apps/[app-name]/project.json` or `libs/[lib-name]/project.json`

**Next.js App Example** (illustrative — no Next.js app is scaffolded yet; shape matches the planned `beaver-nest-fe`):

```json
{
  "name": "app-name",
  "sourceRoot": "apps/app-name",
  "projectType": "application",
  "targets": {
    "dev": {
      "executor": "nx:run-commands",
      "options": {
        "command": "next dev --port 3100",
        "cwd": "apps/app-name"
      }
    },
    "build": {
      "executor": "nx:run-commands",
      "options": {
        "command": "next build",
        "cwd": "apps/app-name"
      },
      "outputs": ["{projectRoot}/.next"]
    }
  },
  "tags": ["type:app", "platform:nextjs", "lang:ts", "domain:beaver-nest"]
}
```

**TypeScript Library Example**:

```json
{
  "name": "ts-utils",
  "sourceRoot": "libs/ts-utils/src",
  "projectType": "library",
  "targets": {
    "build": {
      "executor": "nx:run-commands",
      "options": {
        "command": "tsc -p libs/ts-utils/tsconfig.build.json",
        "cwd": "."
      },
      "outputs": ["{projectRoot}/dist"]
    },
    "typecheck": {
      "executor": "nx:run-commands",
      "options": {
        "command": "tsc --noEmit -p libs/ts-utils/tsconfig.json",
        "cwd": "."
      }
    },
    "test:quick": {
      "executor": "nx:run-commands",
      "options": {
        "command": "tsc --noEmit -p libs/ts-utils/tsconfig.json && node --import tsx --test libs/ts-utils/src/**/*.test.ts",
        "cwd": "."
      }
    },
    "test:unit": {
      "executor": "nx:run-commands",
      "options": {
        "command": "node --import tsx --test libs/ts-utils/src/**/*.test.ts",
        "cwd": "."
      }
    },
    "lint": {
      "executor": "nx:run-commands",
      "options": {
        "command": "echo 'Linting not configured yet'",
        "cwd": "."
      }
    }
  }
}
```

**Target names follow [Nx Target Standards](../../repo-governance/development/infra/nx-targets.md)**: Use `test:quick` for the mandatory pre-push gate, `test:unit` for isolated unit tests. Avoid generic `test` targets.

**Fields**:

- `name` - Project name (used by Nx CLI)
- `sourceRoot` - Source code location
- `projectType` - `"application"` or `"library"`
- `targets` - Nx tasks (build, test, lint, etc.)
- `executor` - Always `"nx:run-commands"` (no plugins)
- `command` - Shell command to execute
- `cwd` - Working directory for command
- `outputs` - Cache output locations
- `dependsOn` - Task dependencies
- `tags` - Project classification (see [Tag Convention](#tag-convention) below)

### Tag Convention

All projects use a standard four-dimension tag scheme:

| Dimension   | Values                                       | Required                 | Purpose                 |
| ----------- | -------------------------------------------- | ------------------------ | ----------------------- |
| `type:`     | `app`, `lib`, `e2e`                          | Yes                      | Project kind            |
| `platform:` | `cli`, `nextjs`, `spring-boot`, `playwright` | For apps/e2e             | Framework/runtime       |
| `lang:`     | `rust`, `ts`, `dotnet`                       | Where source code exists | Primary language        |
| `domain:`   | `beaver-nest`, `tooling`, `ui`               | Yes                      | Business/product domain |

**Notes**:

- Rust libs omit `platform:` — they have no framework, only `lang:rust`
- Use `domain:tooling` for generic dev utilities not tied to a product domain

### `tsconfig.json` (TypeScript Configuration)

**App Example**:

```json
{
  "extends": "../../tsconfig.base.json",
  "compilerOptions": {
    "jsx": "preserve",
    "allowJs": true,
    "esModuleInterop": true,
    "allowSyntheticDefaultImports": true,
    "strict": true,
    "forceConsistentCasingInFileNames": true,
    "noEmit": true,
    "incremental": true,
    "module": "esnext",
    "moduleResolution": "bundler",
    "resolveJsonModule": true,
    "isolatedModules": true,
    "plugins": [
      {
        "name": "next"
      }
    ]
  },
  "include": ["**/*", ".next/types/**/*.ts"],
  "exclude": ["node_modules"]
}
```

**Library Example**:

```json
{
  "extends": "../../tsconfig.base.json",
  "compilerOptions": {
    "module": "ESNext",
    "moduleResolution": "node",
    "declaration": true
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules", "dist", "**/*.test.ts"]
}
```

**Key Points**:

- Always extends `../../tsconfig.base.json`
- Workspace path mappings inherited from base config
- Project-specific options only

### App Configuration Files

**Rust Apps** use `Cargo.toml` for dependency management:

```toml
# apps/rhino-cli/Cargo.toml
[package]
name = "rhino-cli"
version = "0.1.0"
edition = "2024"
rust-version = "1.88"
```

**TypeScript/Next.js Apps** use `package.json`:

```json
{
  "name": "@open-sharia-enterprise/[app-name]",
  "version": "0.1.0",
  "private": true,
  "scripts": {
    "dev": "next dev",
    "build": "next build",
    "start": "next start"
  }
}
```

**Library Example**:

```json
{
  "name": "@open-sharia-enterprise/ts-utils",
  "version": "0.1.0",
  "private": true,
  "main": "./dist/index.js",
  "types": "./dist/index.d.ts",
  "devDependencies": {
    "tsx": "^4.0.0"
  }
}
```

**Naming**:

- Scope: `@open-sharia-enterprise`
- Apps: `@open-sharia-enterprise/[app-name]`
- Libs: `@open-sharia-enterprise/[lib-name]`

## Dependency Rules

### Import Patterns

**Note**: Rust/F# apps do not use TypeScript path mappings. These patterns apply to TypeScript/Next.js apps.

**Apps importing libs** (TypeScript apps):

```typescript
// In apps/<app-name>/app/page.tsx
import { formatDate } from "@open-sharia-enterprise/ts-utils";
```

**Libs importing other libs**:

```typescript
// In libs/ts-components/src/index.ts
import { formatDate } from "@open-sharia-enterprise/ts-utils";
```

### Rules

1. **Apps can import from any lib**
2. **Libs can import from other libs**
3. **No circular dependencies** (A → B → A is prohibited)
4. **Apps should NOT import from other apps**
5. **Language boundaries exist** (TypeScript libs can't directly import Rust/F# libs)

### Monitoring Dependencies

```bash
# View full dependency graph
nx graph

# View specific project dependencies
nx graph --focus=rhino-cli

# View affected projects
nx affected:graph
```

## Path Mappings

Configured in `tsconfig.base.json`:

```json
{
  "compilerOptions": {
    "baseUrl": ".",
    "paths": {
      "@open-sharia-enterprise/ts-*": ["libs/ts-*/src/index.ts"]
    }
  }
}
```

**Pattern**: `@open-sharia-enterprise/[language-prefix]-[name]`

**Examples**:

- `@open-sharia-enterprise/ts-utils`
- `@open-sharia-enterprise/ts-components`
- `@open-sharia-enterprise/ts-hooks`

## Port Allocation

| App                                           | Port    | Status  |
| --------------------------------------------- | ------- | ------- |
| `beaver-nest-fe` (Vite CSR local development) | `19310` | Current |
| `beaver-nest-be` (F#/Giraffe local API)       | `19320` | Current |
| Combined BeaverNest runtime                   | `19300` | Current |

**Rule**: BeaverNest deliberately allocates outside every port band the sibling repos occupy, since all
four repos (`ose-public`, `ose-primer`, `ose-private`, and this repo) can run concurrently on one
development machine:

- `3000-3401` — sibling Next.js/web app ports
- `8000-8302` — sibling backend service ports
- `4222-4224` — NATS
- `5432-5438` — PostgreSQL
- `6006` — Storybook
- `6379` — Redis
- `9090-9411`, `14250`, `14268`, `16686`, `24224` — observability stack (Prometheus, Jaeger, Fluentd)

`19310`/`19320` were verified free across all three sibling repos
(`rg -n '19310|19320'` across `ose-public`, `ose-primer`, and `ose-private` returns no matches) before
being adopted. See [tech-docs Decision 5](../../plans/done/2026-07-31__baseerah-repo-reset/tech-docs.md#decision-5--f--giraffe-backend-on-19320-nextjs-16-frontend-on-19310)
for the full rationale.

## Build Outputs

### Apps

- **Rust**: `apps/[app-name]/target/` (compiled binaries)
- **Next.js**: `apps/[app-name]/.next/`
- **F#/.NET**: `apps/[app-name]/bin/`

### Libraries

- **TypeScript**: `libs/ts-[name]/dist/`

All build outputs are gitignored.

## Related Documentation

- [Nx Target Standards](../../repo-governance/development/infra/nx-targets.md) - Canonical target names, mandatory targets per project type, caching rules, and build output conventions
- [How to Add New App](../how-to/add-new-app.md)
- [How to Add New Library](../how-to/add-new-lib.md)
- [How to Run Nx Commands](../how-to/run-nx-commands.md)
- [Nx Configuration Reference](./nx-configuration.md)
