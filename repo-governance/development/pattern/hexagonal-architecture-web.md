---
title: Hexagonal Architecture — Web Apps
description: Hexagonal architecture specialization for Next.js web apps — feature context modules, Effect.ts ports, application barrel rule, and adapter placement
category: explanation
subcategory: development
tags:
  - architecture
  - hexagonal
  - nextjs
  - effect-ts
  - web
created: 2026-05-26
---

# Hexagonal Architecture — Web Apps

Next.js web apps apply hexagonal architecture through feature context modules. Each module under `contexts/<name>/`
owns its domain, application, infrastructure, and presentation layers. The `contexts/` directory name follows the
Effect.ts `Context.Tag` naming convention — it does **not** represent DDD bounded contexts. DDD applies only to
backend apps; see [Hexagonal Architecture + DDD — Backend Apps](./hexagonal-architecture-be.md).

## Principles Implemented/Respected

- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**: Effect.ts
  `Context.Tag` defines each outbound port as a named, typed interface. Callers declare their dependencies
  explicitly; no global singletons or ambient module state.

- **[Pure Functions Over Side Effects](../../principles/software-engineering/pure-functions.md)**: Domain and
  application logic run as pure Effect pipelines. Side effects (API calls, localStorage) are pushed into
  infrastructure adapters and composed at the presentation boundary.

- **[Simplicity Over Complexity](../../principles/general/simplicity-over-complexity.md)**: The application barrel
  rule (`application/index.ts`) gives presentation and cross-context callers a single, stable import surface. Renaming
  or reorganising internals does not break callers.

- **[Immutability Over Mutability](../../principles/software-engineering/immutability.md)**: Domain value objects are
  immutable. State management in presentation layers uses immutable update patterns (spread, `produce`).

## Conventions Implemented/Respected

- **[Functional Programming Practices](./functional-programming.md)**: Application and domain layers use pure
  functions and immutable data structures.

## Overview

Web apps organise feature concerns into context modules. Each context module is self-contained: it owns its business
rules (domain), orchestration (application), external connections (infrastructure), and UI (presentation). Cross-context
calls go through the application barrel of the target context — never directly into domain or infrastructure.

The `contexts/` directory name arises from Effect.ts, which uses `Context.Tag` to model typed service dependencies.
This naming is a framework convention, not a DDD concept. Web apps do not apply DDD bounded-context isolation rules.

## Directory Layout

```
src/
└── contexts/
    └── <name>/
        ├── domain/            # Business entities, value objects, pure rules
        ├── application/
        │   └── index.ts       # Sole public API surface for this context
        ├── infrastructure/    # Outbound adapters (tRPC client, fetch, localStorage)
        └── presentation/      # Server Components, Client Components, Server Actions
```

| Layer          | Path                              | Exports                                     |
| -------------- | --------------------------------- | ------------------------------------------- |
| Domain         | `contexts/<name>/domain/`         | Entities, value objects, domain error types |
| Application    | `contexts/<name>/application/`    | `index.ts` barrel only                      |
| Infrastructure | `contexts/<name>/infrastructure/` | Internal — not imported by other contexts   |
| Presentation   | `contexts/<name>/presentation/`   | React components, Server Actions            |

## Layer Responsibilities

### domain/ — Domain Layer

- Business entities and value objects (immutable, equality by value)
- Pure validation and transformation functions
- Domain error types (no HTTP status codes, no React types)

### application/ — Application Layer

- Use-case functions that orchestrate domain objects and call outbound ports
- Outbound port definitions as Effect.ts `Context.Tag` services
- `index.ts` barrel: the sole export surface (see Application Barrel Rule below)

### infrastructure/ — Outbound Adapters

- Concrete `Context.Tag` implementations (tRPC client, REST fetch wrappers, localStorage)
- Effect.ts `Layer` definitions wiring implementations to tags
- Never imported by other contexts or by presentation directly

### presentation/ — Inbound Adapters

- Next.js Server Components and Client Components
- Server Actions
- Route handlers (`route.ts`)
- Imports from `application/index.ts` only — never from `domain/` or `infrastructure/` directly

## Port Pattern

Effect.ts `Context.Tag` serves as the port definition mechanism. The application layer declares what it needs; the
infrastructure layer provides the implementation; the presentation layer (or a root `Layer`) wires them together.

```typescript
// contexts/tasks/application/ports.ts  — port definition (application layer)
import { Context, Effect } from "effect";

export interface TaskRepository {
  findById(id: string): Effect.Effect<Task | null, TaskNotFoundError>;
  save(task: Task): Effect.Effect<void, PersistenceError>;
}

export const TaskRepository = Context.GenericTag<TaskRepository>("TaskRepository");
```

```typescript
// contexts/tasks/infrastructure/trpc-task-repository.ts  — port implementation
import { Layer } from "effect";
import { TaskRepository } from "../application/ports";

export const TrpcTaskRepositoryLive = Layer.succeed(TaskRepository, {
  findById: (id) => {
    /* tRPC call */
  },
  save: (task) => {
    /* tRPC call */
  },
});
```

## Application Barrel Rule

`application/index.ts` is the sole public API surface for a context. Presentation layers and other contexts import
only from this barrel.

```typescript
// contexts/tasks/application/index.ts  — barrel (only public export)
export { createTask } from "./use-cases/create-task";
export { completeTask } from "./use-cases/complete-task";
export type { CreateTaskInput, TaskView } from "./types";
// Do NOT re-export domain internals or infrastructure types
```

```typescript
// PASS: presentation imports from barrel
import { createTask } from "@/contexts/tasks/application";

// FAIL: presentation reaches into domain directly
import { Task } from "@/contexts/tasks/domain/task";

// FAIL: presentation reaches into infrastructure
import { TrpcTaskRepositoryLive } from "@/contexts/tasks/infrastructure/trpc-task-repository";
```

The barrel boundary means that renaming, splitting, or reorganising files inside `application/`, `domain/`, or
`infrastructure/` does not break any presentation component as long as the barrel's public surface stays stable.

## Next.js Adapter Placement

Next.js-specific constructs are inbound adapters and belong in `presentation/`:

| Construct                   | Placement       | Notes                                                                         |
| --------------------------- | --------------- | ----------------------------------------------------------------------------- |
| Server Components           | `presentation/` | May call `application/index.ts` use-cases directly                            |
| Client Components           | `presentation/` | Communicate with server through Server Actions or tRPC                        |
| Server Actions              | `presentation/` | Thin wrappers that call `application/index.ts` and return serialisable values |
| Route handlers (`route.ts`) | `presentation/` | Validate input, call application, map errors to HTTP responses                |

Server Components and Server Actions must not import from `domain/` directly. All access goes through the application
barrel so that business logic is testable without a Next.js runtime.

## Forbidden Imports

| Layer             | Forbidden                                                        |
| ----------------- | ---------------------------------------------------------------- |
| `domain/`         | React, Next.js, tRPC, Effect `Layer`, any HTTP client, `fetch`   |
| `application/`    | React, Next.js, `next/headers`, concrete infrastructure modules  |
| `infrastructure/` | React, Next.js UI primitives, business logic (move to `domain/`) |
| `presentation/`   | `domain/` (must go through barrel), `infrastructure/` directly   |

## Reference Implementation

`organiclever-web` is the canonical reference implementation of this pattern in the monorepo. Its `contexts/`
directory demonstrates the barrel rule, Effect.ts port definitions, and Server Action adapter placement.

## Exemptions

Trivially-small static content sites with no IO ports, no business rules, and no
framework-level service dependencies MAY use a flat `src/features/<name>/` layout
instead of the hexagonal `contexts/<name>/{domain,application,infrastructure,presentation}/`
layout. This exemption applies when:

- The site renders static or near-static content with no data-mutation flows.
- No outbound port interfaces are needed (no repositories, no API clients).
- No business invariants need guarding.

When using the flat layout, each feature directory (`src/features/<name>/`) should be
self-contained and import only from sibling features or shared libraries — never from
`src/contexts/`.

**Documented example**: `apps/wahidyankf-web/` (personal portfolio) uses `src/features/`
because it has no IO ports and no business rules.

## Related

- **[Hexagonal Architecture](./hexagonal-architecture.md)** — Core pattern, dependency rule, and layer definitions
- **[Hexagonal Architecture + DDD — Backend Apps](./hexagonal-architecture-be.md)** — DDD bounded contexts for
  backend services; explains why `contexts/` in web apps is different from bounded contexts
