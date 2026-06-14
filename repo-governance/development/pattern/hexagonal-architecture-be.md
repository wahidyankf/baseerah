---
title: Hexagonal Architecture + DDD — Backend Apps
description: Hexagonal architecture with DDD bounded contexts for backend apps — F#/Giraffe directory layouts, language-specific idioms, and inter-context isolation rules
category: explanation
subcategory: development
tags:
  - architecture
  - hexagonal
  - ddd
  - fsharp
  - backend
created: 2026-05-26
---

# Hexagonal Architecture + DDD — Backend Apps

Backend apps combine hexagonal architecture with Domain-Driven Design (DDD) bounded contexts. Each bounded context
lives under `contexts/<name>/` and owns its hexagonal layers independently. DDD applies **only** to backend apps
(`organiclever-be`, `ose-be`). Web apps use `contexts/` as an Effect.ts naming convention, not DDD — see
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

### F#/Giraffe — `organiclever-be` / `ose-be`

```
src/
├── Contexts/
│   ├── <Name>/
│   │   ├── Domain/            # Entities, value objects, domain errors
│   │   ├── Application/       # Use-cases, inbound ports, outbound port interfaces
│   │   ├── Infrastructure/    # Outbound adapter implementations (EF Core repos, HTTP clients)
│   │   └── Api/
│   │       └── Http/          # Giraffe handlers, request/response types, error mapping
│   └── Shared/
│       └── Infrastructure/    # Cross-context shared infrastructure (DB context, migrations)
└── Program.fs                 # Composition root — wires Giraffe router + dependency graph
```

| Layer           | Path                              | Contents                                                            |
| --------------- | --------------------------------- | ------------------------------------------------------------------- |
| Domain          | `Contexts/<N>/Domain/`            | Entities, value objects, `DomainError` discriminated union          |
| Application     | `Contexts/<N>/Application/`       | Use-case functions, port interfaces (`type ITaskRepository`)        |
| Infrastructure  | `Contexts/<N>/Infrastructure/`    | `EfCoreTaskRepository`, external HTTP clients                       |
| Inbound adapter | `Contexts/<N>/Api/Http/`          | Giraffe handlers, `DomainError → HttpHandler` mapping, request DTOs |
| Shared infra    | `Contexts/Shared/Infrastructure/` | DB context, DbUp migration runner, shared middleware                |

## F#-Specific

### Interfaces as Ports

Outbound ports are F# `interface` definitions in the application layer. Infrastructure modules provide
concrete implementations that depend on EF Core or other infrastructure concerns.

```fsharp
// Contexts/Tasks/Application/Ports.fs  — outbound port (application layer)
module Contexts.Tasks.Application.Ports

open Contexts.Tasks.Domain

type ITaskRepository =
    abstract member FindById : TaskId -> Async<Task option>
    abstract member Save : Task -> Async<unit>
```

### Dependency Injection via ASP.NET 10

Application services receive port interfaces through ASP.NET 10 constructor injection. Infrastructure
implementations are registered in `Program.fs` and never referenced directly by application or domain
modules.

```fsharp
// Program.fs  — wire infrastructure implementations to application ports
builder.Services.AddScoped<ITaskRepository, EfCoreTaskRepository>()
```

### Error Mapping at the API Boundary

Domain errors must not contain HTTP status codes. The `Api/Http/` layer owns the translation to
Giraffe `HttpHandler` responses.

```fsharp
// Contexts/Tasks/Domain/Errors.fs  — domain errors (no HTTP types)
module Contexts.Tasks.Domain.Errors

type DomainError =
    | NotFound of TaskId
    | AlreadyCompleted

// Contexts/Tasks/Api/Http/Errors.fs  — translation at API boundary
module Contexts.Tasks.Api.Http.Errors

open Giraffe
open Contexts.Tasks.Domain.Errors

let toHttpHandler (error: DomainError) : HttpHandler =
    match error with
    | NotFound taskId -> RequestErrors.notFound (text (sprintf "task not found: %A" taskId))
    | AlreadyCompleted -> RequestErrors.conflict (text "task is already completed")
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

| Layer             | Forbidden                                                                                    |
| ----------------- | -------------------------------------------------------------------------------------------- |
| `Domain/`         | `Giraffe`, `EntityFrameworkCore`, `HttpContext`, HTTP status types, serialisation attributes |
| `Application/`    | `Giraffe`, `EntityFrameworkCore`, concrete infrastructure types, HTTP types                  |
| `Infrastructure/` | `Giraffe`, HTTP response types, business logic                                               |
| `Api/Http/`       | Direct DB driver calls (must go through outbound port), other context's `Domain/` directly   |

## Related

- **[Hexagonal Architecture](./hexagonal-architecture.md)** — Core pattern, dependency rule, and layer definitions
- **[OpenAPI Contract-First Development](./openapi-contract-first.md)** — How the OpenAPI spec governs the
  `api/http/` inbound adapter boundary
