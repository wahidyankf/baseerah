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

Application inventory and C4 Level 2 container diagram for the BeaverNest platform.

> **2026 BeaverNest repo reset**: every prior app (`ose-www`, `ose-be`, `ayokoding-www`,
> `ayokoding-cli`, `ose-cli`, `crane-cli`, `organiclever-www`, `organiclever-app-web`,
> `organiclever-be`, `wahidyankf-www`, and their `-e2e` counterparts) was deleted, then `beaver-nest-fe`
> and `beaver-nest-be` were scaffolded as the BeaverNest product's hello-world skeleton. See
> [monorepo-structure.md](../monorepo-structure.md) and the
> [baseerah-repo-reset plan](../../../plans/done/2026-07-31__baseerah-repo-reset/README.md).

## Applications Inventory

### CLI Tools

#### rhino-cli

- **Purpose**: Repository management and automation
- **Language**: Rust
- **Build Command**: `nx build rhino-cli`
- **Location**: `apps/rhino-cli/`
- **Status**: Active development

### BeaverNest Product Apps

#### beaver-nest-fe

- **Purpose**: BeaverNest frontend
- **Technology**: Next.js 16 (App Router) + TypeScript
- **Dev Port**: 19310
- **Location**: `apps/beaver-nest-fe/`
- **Dependencies**: `web-ui`, `web-ui-token`, `beaver-nest-contracts`

#### beaver-nest-be

- **Purpose**: BeaverNest backend REST API
- **Technology**: F# + Giraffe + ASP.NET
- **Port**: 19320
- **Location**: `apps/beaver-nest-be/`
- **Dependencies**: `beaver-nest-contracts`, `rhino-cli`

#### beaver-nest-fe-e2e / beaver-nest-be-e2e

- **Purpose**: Playwright FE E2E and HTTP-driven BE E2E suites for `beaver-nest-fe` and
  `beaver-nest-be` respectively
- **Location**: `apps/beaver-nest-fe-e2e/`, `apps/beaver-nest-be-e2e/`

## C4 Level 2: Container Diagram

Shows the high-level technical building blocks (containers) of the system. In C4 terminology, a "container" is a deployable/executable unit (web app, database, file system, etc.), not a Docker container.

**Current state:**

```mermaid
graph LR
    subgraph "CLI Tools"
        RHINO[rhino-cli<br/>Rust CLI]
    end

    subgraph "BeaverNest Product"
        FE[beaver-nest-fe<br/>Next.js App]
        BE[beaver-nest-be<br/>Backend API]
    end

    subgraph "E2E Test Suites"
        FE_E2E[beaver-nest-fe-e2e<br/>Playwright FE E2E]
        BE_E2E[beaver-nest-be-e2e<br/>BE E2E]
    end

    subgraph "Shared Infrastructure"
        LIBS[Libs: web-ui, rust-commons]
        NX[Nx Workspace<br/>Build Orchestration]
    end

    FE_E2E -->|Tests| FE
    BE_E2E -->|Tests| BE
    FE -->|Calls| BE
    FE -->|Imports| LIBS
    BE -->|Spec validation| RHINO
    NX -.->|Manages| RHINO
    NX -.->|Manages| LIBS

    style RHINO fill:#2a9d8f,stroke:#264653,color:#ffffff
    style FE fill:#0077b6,stroke:#03045e,color:#ffffff
    style BE fill:#e76f51,stroke:#9d0208,color:#ffffff
    style FE_E2E fill:#457b9d,stroke:#1d3557,color:#ffffff
    style BE_E2E fill:#457b9d,stroke:#1d3557,color:#ffffff
    style LIBS fill:#457b9d,stroke:#1d3557,color:#ffffff
    style NX fill:#6a4c93,stroke:#22223b,color:#ffffff
```

## Application Interactions

- `rhino-cli`: Repository management automation, managed by the Nx workspace
- `beaver-nest-fe` calls `beaver-nest-be` over HTTP per the `beaver-nest-contracts` OpenAPI spec
- `beaver-nest-fe` imports `web-ui` and `web-ui-token` for UI components and design tokens
- `beaver-nest-be` declares `rhino-cli` as an `implicitDependency` for spec validation;
  `beaver-nest-fe` invokes `rhino-cli` via a raw `cargo run` command in its `specs:*` targets instead
- All applications are managed by the Nx workspace
