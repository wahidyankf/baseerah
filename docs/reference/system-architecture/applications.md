---
title: Applications & Containers
description: Application inventory and C4 Level 2 container diagram
category: reference
tags:
  - architecture
  - applications
  - c4-model
created: 2025-11-29
---

# Applications & Containers

Application inventory and C4 Level 2 container diagram for the Baseerah platform.

> **2026 Baseerah repo reset**: every prior app (`ose-www`, `ose-be`, `ayokoding-www`,
> `ayokoding-cli`, `ose-cli`, `crane-cli`, `organiclever-www`, `organiclever-app-web`,
> `organiclever-be`, `wahidyankf-www`, and their `-e2e` counterparts) was deleted. `rhino-cli` is
> the sole surviving app; `baseerah-fe` and `baseerah-be` are planned but not yet scaffolded. See
> [monorepo-structure.md](../monorepo-structure.md) and the
> [baseerah-repo-reset plan](../../../plans/in-progress/baseerah-repo-reset/README.md).

## Applications Inventory

### CLI Tools

#### rhino-cli

- **Purpose**: Repository management and automation
- **Language**: Rust
- **Build Command**: `nx build rhino-cli`
- **Location**: `apps/rhino-cli/`
- **Status**: Active development

### Planned Applications (not yet scaffolded)

#### baseerah-fe

- **Purpose**: Baseerah frontend
- **Technology**: Next.js 16 (App Router) + TypeScript
- **Planned Dev Port**: 19310
- **Location**: `apps/baseerah-fe/` (does not exist yet)
- **Expected Dependencies**: `web-ui`, `web-ui-token`, `baseerah-contracts`, `rhino-cli`

#### baseerah-be

- **Purpose**: Baseerah backend REST API
- **Technology**: Likely F# + Giraffe + ASP.NET 10 (framework TBD pending backend tech-stack decision)
- **Planned Port**: 19320
- **Location**: `apps/baseerah-be/` (does not exist yet)
- **Expected Dependencies**: `baseerah-contracts`, `rhino-cli`

#### baseerah-fe-e2e / baseerah-be-e2e

- **Purpose**: Playwright FE E2E and HTTP-driven BE E2E suites for `baseerah-fe` and
  `baseerah-be` respectively
- **Location**: `apps/baseerah-fe-e2e/`, `apps/baseerah-be-e2e/` (do not exist yet)

## C4 Level 2: Container Diagram

Shows the high-level technical building blocks (containers) of the system. In C4 terminology, a "container" is a deployable/executable unit (web app, database, file system, etc.), not a Docker container.

**Current state:**

```mermaid
graph LR
    subgraph "CLI Tools"
        RHINO[rhino-cli<br/>Rust CLI]
    end

    subgraph "Shared Infrastructure"
        LIBS[Libs<br/>rust-commons, web-ui]
        NX[Nx Workspace<br/>Build Orchestration]
    end

    NX -.->|Manages| RHINO
    NX -.->|Manages| LIBS

    style RHINO fill:#2a9d8f,stroke:#264653,color:#ffffff
    style LIBS fill:#457b9d,stroke:#1d3557,color:#ffffff
    style NX fill:#6a4c93,stroke:#22223b,color:#ffffff
```

**Planned Baseerah product stack** (not yet scaffolded):

```mermaid
graph LR
    subgraph "Baseerah Product (planned)"
        FE[baseerah-fe<br/>Next.js App]
        BE[baseerah-be<br/>Backend API]
    end

    subgraph "E2E Test Suites (planned)"
        FE_E2E[baseerah-fe-e2e<br/>Playwright FE E2E]
        BE_E2E[baseerah-be-e2e<br/>BE E2E]
    end

    subgraph "Shared Infrastructure (current)"
        LIBS[web-ui, web-ui-token]
        RHINO[rhino-cli]
    end

    FE_E2E -.->|Tests| FE
    BE_E2E -.->|Tests| BE
    FE -.->|Calls| BE
    FE -.->|Imports| LIBS
    FE -.->|Spec validation| RHINO
    BE -.->|Spec validation| RHINO

    style FE fill:#0077b6,stroke:#03045e,color:#ffffff
    style BE fill:#e76f51,stroke:#9d0208,color:#ffffff
    style FE_E2E fill:#457b9d,stroke:#1d3557,color:#ffffff
    style BE_E2E fill:#457b9d,stroke:#1d3557,color:#ffffff
    style LIBS fill:#457b9d,stroke:#1d3557,color:#ffffff
    style RHINO fill:#2a9d8f,stroke:#264653,color:#ffffff
```

## Application Interactions

**Current:**

- `rhino-cli`: Repository management automation, managed by the Nx workspace
- Shared libraries (`rust-commons`, `web-ui`, `web-ui-token`) may be imported at build time via
  `@open-sharia-enterprise/[lib-name]`; no app currently consumes them

**Planned (once `baseerah-fe` and `baseerah-be` are scaffolded):**

- `baseerah-fe` calls `baseerah-be` over HTTP per the `baseerah-contracts` OpenAPI spec
- `baseerah-fe` imports `web-ui` and `web-ui-token` for UI components and design tokens
- Both `baseerah-fe` and `baseerah-be` declare `rhino-cli` as an `implicitDependency` for spec
  validation
- All applications are managed by the Nx workspace
