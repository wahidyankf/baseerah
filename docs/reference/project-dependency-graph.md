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
  "{workspaceRoot}/specs/apps/organiclever-web/**/*.feature"
]
```

## Visual Dependency Graph

**Rust CLI ecosystem and content sites:**

```mermaid
graph TD
  %% Content sites (top level)
  AKW[ayokoding-web]
  OPW[ose-web]
  WKF[wahidyankf-web]

  %% CLI tools
  AKC[ayokoding-cli]
  OPC[ose-cli]
  RC[rhino-cli]

  %% Rust lib
  RSC[rust-commons]

  %% Content site → CLI
  AKW --> AKC
  OPW --> OPC

  %% CLI → shared libs
  AKC --> RSC
  AKC --> RC
  OPC --> RSC
  OPC --> RC

  classDef lib fill:#029E73,stroke:#016B4E,color:#FFFFFF
  classDef cli fill:#DE8F05,stroke:#A56A04,color:#FFFFFF
  classDef site fill:#CC78BC,stroke:#9A5A8E,color:#FFFFFF

  class RSC lib
  class RC,AKC,OPC cli
  class AKW,OPW,WKF site
```

**OrganicLever product stack:**

```mermaid
graph TD
  %% E2E tests (top level)
  OLFE2E[organiclever-web-e2e]
  OLBE2E[organiclever-be-e2e]

  %% Apps
  OLF[organiclever-web]
  OLB[organiclever-be]

  %% Shared
  OLC[organiclever-contracts]
  RC[rhino-cli]

  %% Edges
  OLFE2E --> OLF
  OLBE2E --> OLB
  OLF --> OLC
  OLF --> RC
  OLB --> OLC

  classDef cli fill:#DE8F05,stroke:#A56A04,color:#FFFFFF
  classDef product fill:#CA9161,stroke:#977048,color:#FFFFFF
  classDef e2e fill:#0173B2,stroke:#01537F,color:#FFFFFF

  class RC cli
  class OLF,OLB,OLC product
  class OLFE2E,OLBE2E e2e
```

**Legend**:

- Green: Libraries
- Orange: CLI tools
- Purple: Web sites
- Brown: OrganicLever product apps
- Blue: E2E tests

## Shared Infrastructure Projects

### rhino-cli

**Location**: `apps/rhino-cli/`

Repository management CLI used by most projects for coverage validation
(`test-coverage validate`) and spec coverage (`spec-coverage validate`).

- **Dependents**: CLI tools, libs, content platforms, organiclever-web
- **Mechanism**: `implicitDependencies`
- **Own dependency**: None (self-contained Rust application with only Rust crate dependencies)
- **Note**: rhino-cli was ported from Go to Rust (2026-05-23).

### rust-commons

**Location**: `libs/rust-commons/`

Shared Rust utilities (link-checking, HTTP utilities). Created 2026-05-25 to
consolidate logic shared by `ose-cli` and `ayokoding-cli` after their Go-to-Rust migration.

- **Dependents**: `ose-cli`, `ayokoding-cli`
- **Mechanism**: Cargo workspace `path` dependency

## Project Dependency Table

### Content Platforms

| Project        | Dependencies  | Spec Inputs |
| -------------- | ------------- | ----------- |
| ayokoding-web  | ayokoding-cli | (none)      |
| ose-web        | ose-cli       | (none)      |
| wahidyankf-web | (none)        | (none)      |

### OrganicLever

| Project                | Dependencies                      | Spec Inputs                                 |
| ---------------------- | --------------------------------- | ------------------------------------------- |
| organiclever-contracts | (none)                            | (self — project root is spec dir)           |
| organiclever-web       | rhino-cli, organiclever-contracts | organiclever-web/\* (test:integration)      |
| organiclever-be        | organiclever-contracts            | organiclever-be/\* (test:integration)       |
| organiclever-web-e2e   | organiclever-web                  | organiclever-web/\* (typecheck, test:quick) |
| organiclever-be-e2e    | organiclever-be                   | organiclever-be/\* (typecheck, test:quick)  |

### CLI Tools

| Project       | Dependencies            | Spec Inputs                         |
| ------------- | ----------------------- | ----------------------------------- |
| ayokoding-cli | rust-commons, rhino-cli | ayokoding-cli/\* (test:integration) |
| ose-cli       | rust-commons, rhino-cli | ose-cli/\* (test:integration)       |
| rhino-cli     | (none — self-contained) | rhino-cli/\* (test:integration)     |

### Libraries

| Project      | Dependencies | Spec Inputs                 |
| ------------ | ------------ | --------------------------- |
| rust-commons | (none)       | rust-commons/\* (test:unit) |

## Spec Directory Mapping

All Gherkin specs and API contracts live under `specs/` and are consumed via
`{workspaceRoot}` inputs.

| Spec Directory                                  | Consumed By                            | Targets                                 |
| ----------------------------------------------- | -------------------------------------- | --------------------------------------- |
| `specs/apps/organiclever/containers/contracts/` | organiclever-web, organiclever-be      | codegen                                 |
| `specs/apps/organiclever-web/`                  | organiclever-web, organiclever-web-e2e | test:integration, typecheck, test:quick |
| `specs/apps/rhino/`                             | rhino-cli                              | test:integration                        |
| `specs/apps/ayokoding/`                         | ayokoding-cli, ayokoding-web           | test:integration                        |
| `specs/apps/ose/`                               | ose-cli, ose-web                       | test:integration                        |

## Related Documentation

- [Monorepo Structure Reference](./monorepo-structure.md) - Folder organization and file formats
- [Nx Configuration Reference](./nx-configuration.md) - Workspace configuration options
- [Nx Target Standards](../../repo-governance/development/infra/nx-targets.md) - Canonical target names and caching rules
- [Three-Level Testing Standard](../../repo-governance/development/quality/three-level-testing-standard.md) - Unit, integration, and E2E testing requirements
- [Code Coverage Reference](./code-coverage.md) - Coverage measurement and tools
