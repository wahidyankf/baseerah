---
title: "Overview"
weight: 10000003
date: 2026-05-20T00:00:00+07:00
draft: false
description: "Procedural variant — Hexagonal Architecture in Go (canonical, structural typing makes Go arguably the strongest fit for ports-and-adapters), with Rust formulations where ownership reshapes port design. C is out-of-scope (no canonical hexagonal literature for C)."
tags: ["hexagonal-architecture", "ports-and-adapters", "tutorial", "by-example", "procedural", "go", "rust"]
---

**Hexagonal Architecture is the architecture topic with the strongest fit for Go on this site.** Alistair Cockburn's original 2005 definition is explicitly language-agnostic — ports are interfaces, adapters are implementations, the domain depends on nothing. Go's structural typing (a type satisfies an interface by having the methods, with no `implements` declaration) is arguably a _better_ fit for hexagonal than Java's nominal `implements` because adding a new adapter requires only matching the method set — there is no declaration coupling to the port.

This is an **in-progress track**. The overview and paradigm framing on this page are stable. Full beginner / intermediate / advanced example content rolls out under the [architecture-procedural-track plan](https://github.com/wahidyankf/ose-public/tree/main/plans/in-progress/architecture-procedural-track).

## Why Procedural Wins at Hexagonal

The three forces that make hexagonal Go-friendly:

1. **Structural interface satisfaction.** A Go interface is just a method set; any type with those methods satisfies it. Adding a Postgres adapter for a `PurchaseOrderRepository` port requires only writing the methods — no annotation, no declaration. In Java you must write `implements PurchaseOrderRepository`; in Go that coupling does not exist.

2. **Interface segregation by default.** Idiomatic Go interfaces are tiny — one to three methods is normal, ten is suspicious. This naturally produces the **small, focused ports** hexagonal architecture demands. The opposite tendency (fat interfaces) is the OOP failure mode.

3. **Composition root is plain code.** The composition root in `main.go` is a sequence of constructor calls — `repo := postgres.NewRepo(db); svc := app.NewService(repo, clock)`. No `@Configuration`, no reflection container, no DI annotations. The wiring is step-debuggable plain Go.

Authoritative references for hexagonal in Go:

- Alistair Cockburn — [Hexagonal Architecture](https://alistair.cockburn.us/hexagonal-architecture) (2005) — canonical original; language-agnostic by design.
- Matías Varela — [Hexagonal Architecture in Go](https://medium.com/@matiasvarela/hexagonal-architecture-in-go-cfd4e436faa3) — widely cited Go-specific introductory treatment.
- buarki — [Hexagonal Architecture / Ports And Adapters: Clarifying Key Concepts Using Go](https://dev.to/buarki/hexagonal-architectureports-and-adapters-clarifying-key-concepts-using-go-14oo) — detailed Go-specific walkthrough.
- Three Dots Labs — [DDD + CQRS + Clean Architecture in Go](https://threedots.tech/post/ddd-cqrs-clean-architecture-combined/) — production-grade Go reference combining hexagonal with DDD.
- Matthew Boyle — [_Domain-Driven Design with Golang_](https://www.oreilly.com/library/view/domain-driven-design-with/9781804613450/) (Packt, 2022) — has dedicated hexagonal chapters.

## Rust's Hexagonal Twist — Ownership-Driven Port Design

Rust shares Go's "no inheritance, polymorphism via traits not classes" stance, but adds **ownership** as a port-design concern. The choice between `fn save(self, agg: PurchaseOrder)` (consuming) versus `fn save(&self, agg: &PurchaseOrder)` (shared borrow) versus `fn save(&mut self, agg: &mut PurchaseOrder)` (exclusive borrow) is a port-design decision **no other language asks the architect to make**.

- **Consume-on-call ports** force the caller to commit — useful for "submit this aggregate; you cannot use it again" semantics.
- **Borrow-on-call ports** allow caller retention — useful for read-only or query-shaped ports.
- **Exclusive-borrow ports** allow caller-controlled mutation — useful for adapters that update aggregates in place.

Rust adapters typically implement `#[async_trait] trait Repository` with `Arc<dyn Repository + Send + Sync>` at the composition root for shared ownership across tokio's async runtime.

## What This Tutorial Will Cover

**Go (canonical) idioms:**

- **Three-zone structure** as Go package layout: `domain/`, `app/` (services + port interfaces), `adapter/{in,out}/` (HTTP, repository, messaging adapters).
- **Output ports as small interfaces** — `PurchaseOrderRepository`, `Clock`, `EventPublisher`, `SupplierNotifier` — defined in `app/` package, implemented in `adapter/out/`.
- **In-memory test adapters** — a struct with a `map[ID]Entity` field, four methods, no mocking framework.
- **HTTP primary adapter** with [chi](https://github.com/go-chi/chi) or [echo](https://github.com/labstack/echo) routing; CLI primary adapter via `cobra`.
- **Composition root in `main.go`** — explicit constructor wiring; environment-based adapter selection (`if cfg.UseInMemoryRepo { repo = mem.NewRepo() } else { repo = postgres.NewRepo(db) }`).
- **Cross-context wiring** — multiple bounded contexts as sibling packages; cross-context events flow through `EventPublisher`.
- **Retry / circuit-breaker** as decorator structs that wrap a port implementation.
- **Outbox pattern** at the repository-adapter level, atomic with domain state writes.

**Rust formulations** where ownership reshapes the port design:

- **`#[async_trait]` traits as ports** — async port methods on traits; `Arc<dyn Trait + Send + Sync>` at the composition root.
- **`sqlx` repository adapters** — compile-time-checked SQL with async port methods.
- **`tower` middleware** as decorator stack — retry, timeout, circuit-breaker as composable `Layer`s.
- **`axum` HTTP primary adapter** — typed routing with extractor traits.
- **Move-on-call port design** — for ports that take ownership of an aggregate (e.g., final-state archival ports).

## What This Tutorial Does NOT Cover

- **C**: no canonical hexagonal literature exists for C. Hexagonal patterns require interface-like polymorphism that C approximates only via function-pointer tables — the formulation would be original tutorial content with no citation anchor.
- **Framework setup**: Go module init, Rust `cargo new`, dependency management.
- **DDD tactical pattern depth**: see the [DDD in Procedural track](/en/learn/software-engineering/software-architecture/domain-driven-design-ddd/in-procedural-by-example) for tactical building blocks; this track focuses on the structural boundary.

## Running Domain

Same `procurement-platform-be` Procure-to-Pay domain as the OOP and FP tracks. The bounded contexts (purchasing, supplier, receiving, invoicing, payments, murabaha-finance) and cross-context events (`PurchaseOrderIssued`, `GoodsReceived`, `InvoiceMatched`, `PaymentDisbursed`) are identical.

## Sibling Tutorials

- [Hexagonal Architecture By Example in OOP](/en/learn/software-engineering/software-architecture/hexagonal-architecture/in-oop-by-example/overview) — Java 21+ / Spring Boot 4 (canonical), Kotlin, C#, TypeScript / NestJS.
- [Hexagonal Architecture By Example in FP](/en/learn/software-engineering/software-architecture/hexagonal-architecture/in-fp-by-example/overview) — F# / Giraffe (canonical), Clojure, TypeScript, Haskell.

## Rollout Plan

Full beginner / intermediate / advanced example content authoring tracks under [`plans/in-progress/architecture-procedural-track/`](https://github.com/wahidyankf/ose-public/tree/main/plans/in-progress/architecture-procedural-track).

## Structure of Each Example (Planned)

1. **Brief Explanation** — what hexagonal concept the example demonstrates (2-3 sentences).
2. **Optional Diagram** — Mermaid diagram for zones, port / adapter boundaries.
3. **Heavily Annotated Code** — parallel tabs: Go (canonical), Rust where the idiom changes. `// =>` annotations at 1.0–2.25 comment lines per code line.
4. **Key Takeaway** — the structural boundary principle (1-2 sentences).
5. **Why It Matters** — real-world production impact (50-100 words).
