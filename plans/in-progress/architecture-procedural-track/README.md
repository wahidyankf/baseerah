# Architecture Procedural Track

Adds a third paradigm track — **`in-procedural-by-example`** — to the software architecture content on ayokoding-web. Sibling of the existing `in-oop-by-example` and `in-fp-by-example` tracks. Covers Go (canonical), Rust (typestate-flavoured second), and C (sidebar where canonical literature exists — FSM only).

## Status

**Scaffolding complete (2026-05-20).** Overview pages and `_index.md` indexes for all five top-level architecture topics (patterns-and-principles, ddd, hexagonal, fsm, cases) are live with full paradigm framing, language-fit reasoning, and authoritative citations. Full beginner / intermediate / advanced example content authoring is pending — see [delivery.md](./delivery.md) for the rollout checklist.

## Documents

- [brd.md](./brd.md) — business rationale (audience reach, paradigm-honesty argument)
- [prd.md](./prd.md) — product requirements with Gherkin acceptance criteria
- [tech-docs.md](./tech-docs.md) — content structure, language-tab matrix, citation matrix
- [delivery.md](./delivery.md) — step-by-step authoring checklist (~15 deep tutorial files, 5 already-scaffolded overviews)

## Scope Summary

**Five new `in-procedural-by-example/` directories**:

| Topic                 | Path                                                 | Primary language                   | Secondary              | C?            |
| --------------------- | ---------------------------------------------------- | ---------------------------------- | ---------------------- | ------------- |
| Patterns & Principles | `patterns-and-principles/in-procedural-by-example/`  | Go                                 | Rust                   | No            |
| DDD                   | `domain-driven-design-ddd/in-procedural-by-example/` | Go                                 | Rust                   | No            |
| Hexagonal             | `hexagonal-architecture/in-procedural-by-example/`   | Go (strongest fit)                 | Rust                   | No            |
| FSM                   | `finite-state-machine-fsm/in-procedural-by-example/` | Rust (typestate), Go (looplab/fsm) | C (Samek)              | Yes (sidebar) |
| Cases                 | `cases/in-procedural/`                               | Go (chi/database/sql)              | Rust (axum/sqlx/tokio) | No            |

**Three paradigm-axis updates already shipped (overview-level)**:

- 5 FP overviews now carry a `## Rust as an FP-Adjacent Member — With Concept Adjustments` section enumerating six concept adjustments (ownership/affine types, no HKT, `?` operator vs monadic bind, async/Future vs Async monad, no persistent shared structures by default, traits ≈ typeclasses minus HKT).
- 5 OOP overviews now carry a `## Where Go Fits (Partial)` or `## Where Go and C Fit` section (FSM) pointing to the procedural track for the canonical formulation.
- Parent `_index.md` and `overview.md` for software-architecture now expose three paradigm tracks instead of two.

## Author Estimate (Pending Tier Content)

- 5 topics × 3 tiers (beginner/intermediate/advanced) = 15 deep tutorial files.
- Following the existing OOP/FP track template: each tier file is 5,000–10,000 lines with 1.0–2.25 annotation density.
- Annotations split across Go (canonical), Rust where the idiom changes, occasionally C (FSM only).

This is a multi-session deliverable. Rollout under [delivery.md](./delivery.md).

## Authority Anchors

- **Pike** — [Go at Google: Language Design in the Service of Software Engineering](https://go.dev/talks/2012/splash.article) (2012 SPLASH keynote)
- **Boyle** — [_Domain-Driven Design with Golang_](https://www.oreilly.com/library/view/domain-driven-design-with/9781804613450/) (Packt, 2022)
- **Three Dots Labs** — [DDD + CQRS + Clean Architecture in Go](https://threedots.tech/post/ddd-cqrs-clean-architecture-combined/)
- **Cockburn** — [Hexagonal Architecture](https://alistair.cockburn.us/hexagonal-architecture) (2005)
- **Hoverbear** — [Pretty State Machine Patterns in Rust](https://hoverbear.org/blog/rust-state-machine-pattern/)
- **Crichton** — [Type-Driven API Design in Rust](https://willcrichton.net/rust-api-type-patterns/typestate.html)
- **Samek** — [_Practical UML Statecharts in C/C++_](https://www.routledge.com/Practical-UML-Statecharts-in-CC-Event-Driven-Programming-for-Embedded-Systems/Samek/p/book/9780750687065) (Routledge, 2nd ed. 2008)
- **looplab/fsm** — [github.com/looplab/fsm](https://github.com/looplab/fsm) (v1.0.3, May 2025; 3.4k stars; Apache 2.0)
- **Blandy, Orendorff & Tindall** — [_Programming Rust_, 3rd ed.](https://www.oreilly.com/library/view/programming-rust-3rd/9781098176228/) (O'Reilly, 2023) — Ch. 10 (Enums), Ch. 11 (Traits)
- **GAT stabilisation post** (Matsakis & Huey) — [blog.rust-lang.org/2022/10/28/gats-stabilization](https://blog.rust-lang.org/2022/10/28/gats-stabilization/) — explicit "not full-blown higher-kinded polymorphism"

## Trigger

User requested addition of Go / Rust / C to the architecture-track content "where appropriate" with web-research-maker-validated paradigm fit and conceptual adjustments to FP teaching where Rust does not collapse one-to-one.
