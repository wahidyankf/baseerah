---
title: Project Dependency Graph
description: Complete reference for Nx project dependencies, implicit dependencies, and workspace-level spec inputs
category: reference
tags:
  - nx
  - dependencies
  - architecture
  - monorepo
created: 2026-03-22
---

# Project Dependency Graph

Complete reference for how projects depend on each other in the Nx monorepo.
Run `nx graph` to visualize this interactively.

> **Note**: The polyglot demo apps (`a-demo-be-*`, `a-demo-fe-*`, `a-demo-fs-ts-nextjs`) and
> their contract/spec infrastructure were extracted to
> [ose-primer](https://github.com/wahidyankf/ose-primer) on 2026-04-18. That repository is the
> authoritative reference for polyglot showcase dependency patterns.
>
> **2026 Baseerah repo reset**: every app except `rhino-cli` was deleted (see
> [monorepo-structure.md](./monorepo-structure.md)). The diagrams and tables below reflect the
> current, much smaller dependency graph, followed by the graph the [`baseerah-repo-reset`
> plan](../../plans/in-progress/baseerah-repo-reset/README.md) expects once `baseerah-fe` and
> `baseerah-be` are scaffolded.

## Dependency Mechanisms

Nx tracks project relationships through three mechanisms:

### 1. `implicitDependencies` (Project-Level)

Declared in `project.json`. When the dependency project changes, `nx affected`
flags the dependent project for re-testing.

```json
"implicitDependencies": ["rhino-cli"]
```

### 2. `dependsOn` (Task-Level)

Declared per target in `project.json`. Controls execution order — the dependency
task runs before the dependent task.

### 3. `inputs` with `{workspaceRoot}` (File-Level)

Declared per target. When matched files change, the target's cache is
invalidated and `nx affected` flags the project.

```json
"inputs": [
  "default",
  "{workspaceRoot}/specs/apps/rhino/**/*.feature"
]
```

## Visual Dependency Graph

**Current apps and libs:**

```mermaid
graph TD
  %% Current app
  RC[rhino-cli]

  %% Current libs
  RSC[rust-commons]
  WUI[web-ui]
  WUT[web-ui-token]

  %% Edges
  WUI --> WUT

  classDef lib fill:#029E73,stroke:#016B4E,color:#FFFFFF
  classDef cli fill:#DE8F05,stroke:#A56A04,color:#FFFFFF

  class RSC,WUI,WUT lib
  class RC cli
```

`rhino-cli` and `rust-commons` currently have no dependency edges to other projects — their
former consumers (`ayokoding-cli`, `ose-cli`, and the deleted content sites) were removed by the
Baseerah repo reset.

**Planned Baseerah product stack** (not yet scaffolded — per
[tech-docs.md § Dependencies](../../plans/in-progress/baseerah-repo-reset/tech-docs.md)):

```mermaid
graph TD
  %% Planned E2E tests (top level)
  FEE2E[baseerah-fe-e2e]
  BEE2E[baseerah-be-e2e]

  %% Planned apps
  FE[baseerah-fe]
  BE[baseerah-be]

  %% Planned contracts project
  BC[baseerah-contracts]

  %% Existing shared projects the planned apps are expected to use
  WUI[web-ui]
  WUT[web-ui-token]
  RC[rhino-cli]

  %% Edges
  FEE2E -.-> FE
  BEE2E -.-> BE
  FE -.-> WUI
  FE -.-> WUT
  FE -.-> BC
  BE -.-> BC
  FE -.-> RC
  BE -.-> RC

  classDef planned fill:#CA9161,stroke:#977048,color:#FFFFFF
  classDef current fill:#029E73,stroke:#016B4E,color:#FFFFFF
  classDef cli fill:#DE8F05,stroke:#A56A04,color:#FFFFFF

  class FE,BE,BC,FEE2E,BEE2E planned
  class WUI,WUT current
  class RC cli
```

**Legend**:

- Green: Libraries (current)
- Orange: CLI tools (current)
- Brown, dashed edges: Planned Baseerah projects (not yet scaffolded)

## Shared Infrastructure Projects

### rhino-cli

**Location**: `apps/rhino-cli/`

Repository management CLI used by most projects for spec coverage (`rhino-cli specs coverage`)
and other validation tasks.

- **Dependents**: None currently — every prior consumer app was deleted by the 2026 Baseerah
  repo reset. Once scaffolded, `baseerah-fe` and `baseerah-be` are each expected to declare
  `rhino-cli` as an `implicitDependency` for spec validation.
- **Mechanism**: `implicitDependencies`
- **Own dependency**: None (self-contained Rust application with only Rust crate dependencies)
- **Note**: rhino-cli was ported from Go to Rust (2026-05-23).

### rust-commons

**Location**: `libs/rust-commons/`

Shared Rust utilities (link-checking, HTTP utilities). Created 2026-05-25 to
consolidate logic shared by `ose-cli` and `ayokoding-cli` after their Go-to-Rust migration.
Both consumer CLIs were later deleted by the 2026 Baseerah repo reset.

- **Dependents**: None currently.
- **Mechanism**: Cargo workspace `path` dependency

### web-ui / web-ui-token

**Location**: `libs/web-ui/`, `libs/web-ui-token/`

Shared React component library (`web-ui`) and its design-token package (`web-ui-token`).
`web-ui` depends on `web-ui-token` via an npm workspace `package.json` dependency.
No app currently consumes either lib; `baseerah-fe` is the planned consumer once scaffolded.

## Project Dependency Table

### Current Projects

| Project      | Dependencies            | Spec Inputs                     |
| ------------ | ----------------------- | ------------------------------- |
| rhino-cli    | (none — self-contained) | rhino-cli/\* (test:integration) |
| rust-commons | (none)                  | rust-commons/\* (test:unit)     |
| web-ui       | web-ui-token            | web-ui/\* (test:unit)           |
| web-ui-token | (none)                  | web-ui-token/\* (test:unit)     |

### Planned Projects (not yet scaffolded)

| Project            | Expected Dependencies                               | Notes                                             |
| ------------------ | --------------------------------------------------- | ------------------------------------------------- |
| baseerah-contracts | (none)                                              | OpenAPI contract source consumed by fe and be     |
| baseerah-be        | baseerah-contracts, rhino-cli                       | Framework TBD pending backend tech-stack decision |
| baseerah-fe        | web-ui, web-ui-token, baseerah-contracts, rhino-cli | Next.js 16 App Router, planned port 19310         |
| baseerah-be-e2e    | baseerah-be                                         | Planned                                           |
| baseerah-fe-e2e    | baseerah-fe                                         | Planned                                           |

See [tech-docs.md § Dependencies](../../plans/in-progress/baseerah-repo-reset/tech-docs.md) for
the source of this table. That document also records the resolved verdict (delete) on the
conditional `libs/fsharp-crane-core` dependency it once considered for `baseerah-be` — the lib
was `crane-cli`-specific and was deleted alongside `crane-cli`.

## Spec Directory Mapping

All Gherkin specs and API contracts live under `specs/` and are consumed via
`{workspaceRoot}` inputs.

| Spec Directory                   | Consumed By              | Targets                            |
| -------------------------------- | ------------------------ | ---------------------------------- |
| `specs/apps/rhino/`              | rhino-cli                | test:integration                   |
| `specs/libs/web-ui/`             | web-ui                   | test:unit                          |
| `specs/libs/web-ui-token/`       | web-ui-token             | test:unit                          |
| `specs/apps/baseerah/` (planned) | baseerah-fe, baseerah-be | test:integration (once scaffolded) |

## Related Documentation

- [Monorepo Structure Reference](./monorepo-structure.md) - Folder organization and file formats
- [Nx Configuration Reference](./nx-configuration.md) - Workspace configuration options
- [Nx Target Standards](../../repo-governance/development/infra/nx-targets.md) - Canonical target names and caching rules
- [Three-Level Testing Standard](../../repo-governance/development/quality/three-level-testing-standard.md) - Unit, integration, and E2E testing requirements
- [Code Coverage Reference](./code-coverage.md) - Coverage measurement and tools
