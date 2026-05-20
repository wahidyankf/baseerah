# Tech Docs — Architecture Procedural Track

## File Tree (Stage 1 — Shipped)

```
apps/ayokoding-web/content/en/learn/software-engineering/software-architecture/
├── _index.md                                       (UPDATED — adds procedural links to 5 sections)
├── overview.md                                     (UPDATED — exposes three paradigm tracks)
│
├── patterns-and-principles/
│   ├── _index.md                                   (UPDATED — adds in-procedural link)
│   ├── in-fp-by-example/overview.md                (UPDATED — adds Rust concept adjustments)
│   ├── in-oop-by-example/overview.md               (UPDATED — adds Go partial-fit note)
│   └── in-procedural-by-example/                   (NEW)
│       ├── _index.md                               (NEW)
│       └── overview.md                             (NEW — substantive paradigm framing)
│
├── domain-driven-design-ddd/
│   ├── _index.md                                   (UPDATED — adds in-procedural link)
│   ├── in-fp-by-example/overview.md                (UPDATED — adds Rust concept adjustments)
│   ├── in-oop-by-example/overview.md               (UPDATED — adds Go partial-fit note)
│   └── in-procedural-by-example/                   (NEW)
│       ├── _index.md                               (NEW)
│       └── overview.md                             (NEW)
│
├── hexagonal-architecture/
│   ├── _index.md                                   (UPDATED — adds in-procedural link)
│   ├── in-fp-by-example/overview.md                (UPDATED — adds Rust concept adjustments)
│   ├── in-oop-by-example/overview.md               (UPDATED — adds Go strong-fit note)
│   └── in-procedural-by-example/                   (NEW)
│       ├── _index.md                               (NEW)
│       └── overview.md                             (NEW)
│
├── finite-state-machine-fsm/
│   ├── _index.md                                   (UPDATED — adds in-procedural link)
│   ├── in-fp-by-example/overview.md                (UPDATED — adds Rust + typestate note)
│   ├── in-oop-by-example/overview.md               (UPDATED — adds Go/C native idioms)
│   └── in-procedural-by-example/                   (NEW)
│       ├── _index.md                               (NEW)
│       └── overview.md                             (NEW — covers Rust/Go/C native idioms)
│
└── cases/
    ├── _index.md                                   (UPDATED — adds in-procedural link)
    ├── in-fp/overview.md                           (UPDATED — adds Rust adjustments)
    ├── in-oop/overview.md                          (UPDATED — adds Go/Rust sibling pointer)
    └── in-procedural/                              (NEW)
        ├── _index.md                               (NEW)
        └── overview.md                             (NEW)
```

## Language-Tab Matrix (Planned for Stage 2 Tier Content)

| Topic                   | Tier         | Go                                            | Rust                               | C                                      |
| ----------------------- | ------------ | --------------------------------------------- | ---------------------------------- | -------------------------------------- |
| patterns-and-principles | beginner     | canonical                                     | for trait-based examples           | —                                      |
| patterns-and-principles | intermediate | canonical                                     | for `?`-operator + Result examples | —                                      |
| patterns-and-principles | advanced     | canonical                                     | for systems-level examples         | —                                      |
| ddd                     | beginner     | canonical (Boyle Ch. 2 style)                 | for typestate aggregates           | —                                      |
| ddd                     | intermediate | canonical (Three Dots Labs reference)         | for ownership-driven aggregates    | —                                      |
| ddd                     | advanced     | canonical (Three Dots Labs CQRS)              | for typestate state machines       | —                                      |
| hexagonal               | beginner     | canonical (chi + database/sql)                | for trait-based ports              | —                                      |
| hexagonal               | intermediate | canonical (composition root in main.go)       | for `Arc<dyn Trait>` composition   | —                                      |
| hexagonal               | advanced     | canonical (production wiring)                 | for axum + sqlx + tokio            | —                                      |
| fsm                     | beginner     | looplab/fsm                                   | typestate canonical                | —                                      |
| fsm                     | intermediate | looplab/fsm + workflow callbacks              | typestate w/ guards                | Samek function-pointer table (sidebar) |
| fsm                     | advanced     | looplab/fsm + hierarchical (workaround)       | typestate + nested                 | Samek hierarchical statecharts         |
| cases                   | beginner     | canonical (chi)                               | axum tab                           | —                                      |
| cases                   | intermediate | canonical (database/sql + outbox)             | sqlx + tokio                       | —                                      |
| cases                   | advanced     | canonical (production wiring + observability) | axum production wiring             | —                                      |

## Citation Matrix

| Source                                                   | Used in topics                                         | Citation                                                                                                                                                                            |
| -------------------------------------------------------- | ------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Pike — Go at Google (SPLASH 2012)                        | All Go-content topics + OOP overview note              | [go.dev/talks/2012/splash.article](https://go.dev/talks/2012/splash.article)                                                                                                        |
| Boyle — _Domain-Driven Design with Golang_ (Packt, 2022) | ddd procedural, ddd OOP overview note                  | [oreilly.com/library/view/domain-driven-design-with/9781804613450/](https://www.oreilly.com/library/view/domain-driven-design-with/9781804613450/)                                  |
| Three Dots Labs                                          | ddd procedural, hexagonal procedural, cases procedural | [threedots.tech/post/ddd-cqrs-clean-architecture-combined/](https://threedots.tech/post/ddd-cqrs-clean-architecture-combined/)                                                      |
| Cockburn — Hexagonal Architecture (2005)                 | hexagonal procedural, hexagonal OOP overview note      | [alistair.cockburn.us/hexagonal-architecture](https://alistair.cockburn.us/hexagonal-architecture)                                                                                  |
| Hoverbear — Pretty State Machine Patterns in Rust        | fsm procedural, fsm FP overview Rust adjustment note   | [hoverbear.org/blog/rust-state-machine-pattern/](https://hoverbear.org/blog/rust-state-machine-pattern/)                                                                            |
| Crichton — Type-Driven API Design in Rust                | fsm procedural typestate examples                      | [willcrichton.net/rust-api-type-patterns/typestate.html](https://willcrichton.net/rust-api-type-patterns/typestate.html)                                                            |
| Samek — _Practical UML Statecharts in C/C++_             | fsm procedural C sidebar                               | [routledge.com/Practical-UML-Statecharts-in-CC](https://www.routledge.com/Practical-UML-Statecharts-in-CC-Event-Driven-Programming-for-Embedded-Systems/Samek/p/book/9780750687065) |
| looplab/fsm                                              | fsm procedural Go examples                             | [github.com/looplab/fsm](https://github.com/looplab/fsm)                                                                                                                            |
| Blandy, Orendorff & Tindall — _Programming Rust_ 3rd ed. | All Rust-content topics + FP overview adjustments      | [oreilly.com/library/view/programming-rust-3rd/9781098176228/](https://www.oreilly.com/library/view/programming-rust-3rd/9781098176228/)                                            |
| GAT stabilisation post (Matsakis & Huey)                 | FP overview no-HKT adjustment                          | [blog.rust-lang.org/2022/10/28/gats-stabilization/](https://blog.rust-lang.org/2022/10/28/gats-stabilization/)                                                                      |
| Rust Reference — Enum types                              | FP overview enum-as-ADT adjustment                     | [doc.rust-lang.org/reference/types/enum.html](https://doc.rust-lang.org/reference/types/enum.html)                                                                                  |
| Rust By Example — `?` operator                           | FP overview `?`-vs-monadic-bind adjustment             | [doc.rust-lang.org/rust-by-example/std/result/question_mark.html](https://doc.rust-lang.org/rust-by-example/std/result/question_mark.html)                                          |
| without.boats — Ownership                                | FP overview ownership/affine-types adjustment          | [without.boats/blog/ownership/](https://without.boats/blog/ownership/)                                                                                                              |

## Style Decisions

**Overview length**: ~500–800 lines each. Substantive enough to stand on its own without tier content; not so long that it duplicates what the tier files will cover.

**Voice**: Same as existing OOP / FP overviews — direct, prerequisite-aware, citation-heavy. Open with a "Want to…?" hook where the existing tracks use that pattern.

**Cross-track linking**: Each track overview links to its two siblings. Each tier file (Stage 2) will cross-link to same-numbered examples in sibling tracks where possible.

**Frontmatter weight ordering**: OOP = `…001`, FP = `…002`, Procedural = `…003`. Cases follows the existing ordering: in-fp (weight 10000002), in-oop (weight 10000011), in-procedural (weight 10000012).

**Sharia procurement extension**: The `murabaha-finance` bounded context is the optional sidebar across all three tracks. Procedural track includes it for parity.

## Anti-Goals (Explicitly Out of Scope)

1. **C beyond FSM**. No canonical literature exists for DDD-in-C or hexagonal-in-C. Adding would mean fabricating tutorial content.
2. **Rust as OOP-track tab**. Rust rejects inheritance; force-fitting would teach the wrong instincts.
3. **Procedural patterns track for the by-concept ("`overview.md`") layer**. The by-concept content is paradigm-agnostic; only the `*-by-example` layers get paradigm tracks.
4. **Java / C# in the procedural track**. Java and C# are inheritance-bearing; they live in the OOP track.
5. **Haskell in the procedural track**. Haskell is pure FP; it lives in the FP track.
