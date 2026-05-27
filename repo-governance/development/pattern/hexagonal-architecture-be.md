---
title: Hexagonal Architecture + DDD — Backend Apps
description: Hexagonal architecture with DDD bounded contexts for backend apps — Rust/Axum directory layouts, language-specific idioms, and inter-context isolation rules
category: explanation
subcategory: development
tags:
  - architecture
  - hexagonal
  - ddd
  - rust
  - backend
created: 2026-05-26
---

# Hexagonal Architecture + DDD — Backend Apps

Backend apps combine hexagonal architecture with Domain-Driven Design (DDD) bounded contexts. Each bounded context
lives under `contexts/<name>/` and owns its hexagonal layers independently. DDD applies **only** to backend apps
(`organiclever-be`, `ose-app-be`). Web apps use `contexts/` as an Effect.ts naming convention, not DDD — see
[Hexagonal Architecture — Web Apps](./hexagonal-architecture-web.md).

## Principles Implemented/Respected

- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**: Bounded context
  boundaries are directory boundaries. Inter-context dependencies cross through application interfaces only —
  never through shared domain types.

- **[Pure Functions Over Side Effects](../../principles/software-engineering/pure-functions.md)**: Domain layers
  contain pure business rules. Infrastructure layers contain all I/O. Error mapping (domain error → HTTP response)
  happens at the `api/http/` boundary, not inside the domain.

- **[Simplicity Over Complexity](../../principles/general/simplicity-over-complexity.md)**: Each bounded context is
  independently deployable in principle. Keeping contexts isolated prevents a change in one domain from cascading
  through the entire codebase.

- **[Immutability Over Mutability](../../principles/software-engineering/immutability.md)**: Domain entities are
  immutable value types. Infrastructure adapters translate mutable external representations at the boundary.

## Conventions Implemented/Respected

- **[Functional Programming Practices](./functional-programming.md)**: Domain and application layers use pure
  functions and immutable data.
- **[OpenAPI Contract-First Development](./openapi-contract-first.md)**: The `api/http/` inbound adapter layer
  implements handlers generated from or validated against the OpenAPI spec.

## Overview

Each bounded context encapsulates a coherent subdomain. Within the context, hexagonal layering keeps business logic
isolated from delivery mechanisms (HTTP, future GraphQL, MCP) and infrastructure (database, external HTTP).

The `api/` directory groups all inbound transport adapters. Today `api/http/` is the only transport; `api/graphql/`
and `api/mcp/` are reserved for future transports. All transport-specific code stays inside `api/<transport>/`.

## Directory Layout

### Rust/Axum — `organiclever-be`

```
src/
├── contexts/
│   ├── <name>/
│   │   ├── domain/            # Entities, value objects, domain errors
│   │   ├── application/       # Use-cases, inbound ports, outbound port traits
│   │   ├── infrastructure/    # Outbound adapter implementations (SQLx repos, HTTP clients)
│   │   └── api/
│   │       └── http/          # Axum handlers, request/response types, error mapping
│   └── shared/
│       └── infrastructure/    # Cross-context shared infrastructure (DB pool, migrations)
└── main.rs                    # Composition root — wires Axum router + dependency graph
```

| Layer           | Path                              | Contents                                                      |
| --------------- | --------------------------------- | ------------------------------------------------------------- |
| Domain          | `contexts/<n>/domain/`            | Entities, value objects, `DomainError` enum                   |
| Application     | `contexts/<n>/application/`       | Use-case functions, port traits (`trait TaskRepository`)      |
| Infrastructure  | `contexts/<n>/infrastructure/`    | `SqlxTaskRepository`, external HTTP clients                   |
| Inbound adapter | `contexts/<n>/api/http/`          | Axum handlers, `From<DomainError> for ApiError`, request DTOs |
| Shared infra    | `contexts/shared/infrastructure/` | DB pool, migration runner, shared middleware                  |

## Rust-Specific

### Traits as Ports

Outbound ports are Rust `trait` definitions in the application layer. Infrastructure crates provide concrete
`struct` implementations.

```rust
// contexts/tasks/application/ports.rs  — outbound port (application layer)
use async_trait::async_trait;
use crate::contexts::tasks::domain::{Task, TaskId, DomainError};

#[async_trait]
pub trait TaskRepository: Send + Sync {
    async fn find_by_id(&self, id: TaskId) -> Result<Option<Task>, DomainError>;
    async fn save(&self, task: &Task) -> Result<(), DomainError>;
}
```

### `#[async_trait]` vs Native Async Trait

- Use `#[async_trait]` only when the trait requires **dynamic dispatch** (`dyn TaskRepository`).
- For **static dispatch** (generics, `impl TaskRepository`), use native `async fn in trait` (stable from MSRV 1.88).
  Do not add `#[async_trait]` where dynamic dispatch is not needed.

```rust
// PASS: static dispatch — no #[async_trait] needed (MSRV 1.88+)
pub async fn create_task<R: TaskRepository>(repo: &R, input: CreateTaskInput) -> Result<Task, AppError> {
    // ...
}

// PASS: dynamic dispatch — #[async_trait] required
pub async fn create_task(repo: &dyn TaskRepository, input: CreateTaskInput) -> Result<Task, AppError> {
    // ...
}
```

### Error Mapping at the API Boundary

Domain errors must not contain HTTP status codes. The `api/http/` layer owns the translation.

```rust
// contexts/tasks/domain/errors.rs  — domain errors (no HTTP types)
#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("task not found: {0}")]
    NotFound(TaskId),
    #[error("task is already completed")]
    AlreadyCompleted,
}

// contexts/tasks/api/http/errors.rs  — translation at API boundary
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use crate::contexts::tasks::domain::DomainError;

pub struct ApiError(DomainError);

impl From<DomainError> for ApiError {
    fn from(e: DomainError) -> Self { ApiError(e) }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match &self.0 {
            DomainError::NotFound(_) => StatusCode::NOT_FOUND,
            DomainError::AlreadyCompleted => StatusCode::CONFLICT,
        };
        (status, self.0.to_string()).into_response()
    }
}
```

## DDD Integration

### Bounded Context Isolation

Contexts communicate through application layer interfaces only. Shared domain types between contexts create coupling
and are forbidden.

```
PASS: ContextA.Application.IOrderService calls ContextB.Application.IInventoryService
FAIL: ContextA.Domain.Order references ContextB.Domain.InventoryItem
```

### Shared Infrastructure

Cross-context infrastructure (database connection pool, migration runner, shared middleware) lives in
`contexts/shared/infrastructure/`. Shared infrastructure must not contain business logic.

### Anti-Corruption Layer

When a context must integrate with a legacy system or external API that speaks a different domain language, place an
anti-corruption layer in `infrastructure/` of the consuming context. Translate external types to domain types at
the boundary.

## Forbidden Imports

| Layer             | Forbidden                                                                                  |
| ----------------- | ------------------------------------------------------------------------------------------ |
| `domain/`         | `axum`, `sqlx`, `reqwest`, `tokio::fs`, HTTP status types, `serde_json` (HTTP-specific)    |
| `application/`    | `axum`, `sqlx`, concrete infrastructure structs, HTTP types                                |
| `infrastructure/` | `axum`, HTTP response types, business logic                                                |
| `api/http/`       | Direct DB driver calls (must go through outbound port), other context's `domain/` directly |

## Related

- **[Hexagonal Architecture](./hexagonal-architecture.md)** — Core pattern, dependency rule, and layer definitions
- **[OpenAPI Contract-First Development](./openapi-contract-first.md)** — How the OpenAPI spec governs the
  `api/http/` inbound adapter boundary
