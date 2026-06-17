---
title: Functional Core / Imperative Shell — Web Apps
description: The architecture pattern for Next.js web apps — every feature module splits into a pure functional core and an effectful imperative shell under src/features/<name>/{core,shell}/
category: explanation
subcategory: development
tags:
  - architecture
  - functional-core-imperative-shell
  - nextjs
  - functional-programming
  - web
created: 2026-06-17
---

# Functional Core / Imperative Shell — Web Apps

Next.js web apps in this repo organise every feature as a **functional core / imperative shell** module under
`src/features/<name>/`. Each module splits into exactly two zones: a pure `core/` that holds all logic and decisions,
and an effectful `shell/` that performs IO, renders UI, and wires the framework. The shell calls the core; the core
never reaches back.

This is **not** hexagonal architecture and **not** DDD. There are no ports, no adapters, no `Context.Tag` services,
no `domain`/`application`/`infrastructure`/`presentation` layering, and no application barrel rule. Web apps render
content and orchestrate a thin amount of IO; the two-zone core/shell split is the minimum viable structure that keeps
the logic pure and testable. The ports-and-adapters hexagonal pattern is reserved for **backend** services — see
[Hexagonal Architecture + DDD — Backend Apps](./hexagonal-architecture-be.md).

## Principles Implemented/Respected

- **[Pure Functions Over Side Effects](../../principles/software-engineering/pure-functions.md)**: The functional core
  is pure — every decision, transformation, validation, and derivation lives in functions with no IO and no side
  effects. Effects are pushed to the imperative shell at the edge.

- **[Simplicity Over Complexity](../../principles/general/simplicity-over-complexity.md)**: Two zones, not four
  layers. No port interfaces or dependency-injection wiring are introduced for their own sake; the shell imports the
  core directly.

- **[Immutability Over Mutability](../../principles/software-engineering/immutability.md)**: Core data is immutable;
  shell state uses immutable update patterns (spread, `produce`).

- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**: The directory a file
  lives in declares its nature. A file under `core/` is provably pure; a file under `shell/` is where effects are
  allowed. No ambient inference required.

## Conventions Implemented/Respected

- **[Functional Programming Practices](./functional-programming.md)**: The core uses pure functions and immutable data
  structures throughout.

## Directory Layout

```
src/
└── features/
    └── <name>/
        ├── core/      # PURE: logic, decisions, validation, transforms, schemas, types, constant data
        └── shell/     # EFFECTFUL: React components, hooks, fs/network/tRPC, Server Actions, route handlers, wiring
```

| Zone  | Path                     | Holds                                                                            |
| ----- | ------------------------ | -------------------------------------------------------------------------------- |
| Core  | `features/<name>/core/`  | Pure functions, immutable data, plain types/interfaces, zod schemas, data tables |
| Shell | `features/<name>/shell/` | React components, DOM hooks, fs readers, repositories, tRPC routers, route.ts    |

A feature that has no pure logic (UI-only) has only a `shell/`. A feature that has no effects has only a `core/`.
Create only the zones a feature actually needs — do not add empty placeholder directories or barrels.

## Zone Responsibilities

### core/ — Functional Core

- Pure functions: validation, transformation, derivation, ranking, formatting, calculation
- Immutable value types and plain TypeScript interfaces
- zod schemas (pure data validation)
- Constant data tables (e.g. i18n translation maps, static datasets)
- Shared interfaces that the shell implements (e.g. a repository interface) live here so the shell can depend on the
  core without the core ever depending on the shell

The core is fully unit-testable without a Next.js runtime, a DOM, a filesystem, or a network.

### shell/ — Imperative Shell

- Next.js Server Components and Client Components (`.tsx`)
- React hooks that touch the DOM or browser globals
- Filesystem readers, content repositories, search-index generators
- tRPC routers, tRPC init, root router (server wiring)
- Next.js middleware, route handlers (`route.ts`), Server Actions
- Any code performing IO, network, or framework wiring

The shell stays thin: it gathers inputs, calls the core for decisions, and applies the results as effects.

## The Dependency Rule

```
shell/  --imports-->  core/      ALLOWED
core/   --imports-->  shell/     FORBIDDEN
```

`core/` MUST NOT import any of: `react`, `react-dom`, `next`, `next/*`, node builtins (`fs`, `path`, `node:*`),
`@trpc/server` router/init wiring, any HTTP/DB/`fetch` client, or browser globals — not even as types. If a file under
`core/` needs one of those, it belongs in `shell/`. `core/` may import other `core/` modules (pure to pure). `shell/`
may freely import its own and sibling `core/`.

### Forbidden imports

| Zone     | Forbidden                                                                                  |
| -------- | ------------------------------------------------------------------------------------------ |
| `core/`  | `react`, `react-dom`, `next`, `next/*`, `fs`/`path`/`node:*`, `@trpc/server`, `fetch`/HTTP |
| `shell/` | Business decisions that belong in the core (extract pure logic to `core/` and call it)     |

Verify core purity with:

```bash
rg -n "from ['\"](react|react-dom|next|node:|fs|path|@trpc/server)" apps/<app>/src/features/*/core
```

This must return nothing.

## Next.js Construct Placement

Next.js framework constructs are effects and belong in `shell/`:

| Construct                   | Placement | Notes                                                           |
| --------------------------- | --------- | --------------------------------------------------------------- |
| Server Components           | `shell/`  | Call `core/` functions directly for any logic                   |
| Client Components           | `shell/`  | Hold UI state; delegate decisions to `core/`                    |
| Server Actions              | `shell/`  | Thin wrappers that call `core/` and return serialisable values  |
| Route handlers (`route.ts`) | `shell/`  | Validate input (zod schema from `core/`), call `core/`, respond |
| Middleware                  | `shell/`  | Framework wiring                                                |
| tRPC routers / init         | `shell/`  | Server wiring                                                   |

## Reference Implementations

All three Next.js content apps follow this pattern identically:

- `apps/ose-www/` — content/landing/search/seo/rss-feed features, each split into `core/` (parsers, schemas, builders)
  and `shell/` (fs repositories, tRPC routers, React components)
- `apps/ayokoding-www/` — content/i18n/navigation/search features in the same split
- `apps/wahidyankf-www/` — portfolio features; pure CV/project/search data and helpers in `core/`, React UI in
  `shell/`

## Related

- **[Hexagonal Architecture](./hexagonal-architecture.md)** — Core dependency-rule idea shared with the backend pattern
- **[Hexagonal Architecture + DDD — Backend Apps](./hexagonal-architecture-be.md)** — The ports-and-adapters / DDD
  pattern used by backend services; explains why web apps deliberately use the simpler core/shell split instead
- **[Functional Programming Practices](./functional-programming.md)** — Pure-function and immutability conventions the
  core depends on
