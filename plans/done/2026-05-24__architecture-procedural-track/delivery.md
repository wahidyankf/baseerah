# Delivery — Architecture Procedural Track

## Stage 1 — Overview Scaffolding (Shipped 2026-05-20)

- [x] Research paradigm-fit for Go / Rust / C via web-researcher
- [x] Read all 5 FP overview.md files (patterns-and-principles, ddd, hexagonal, fsm, cases)
- [x] Read all 5 OOP overview.md files
- [x] Read parent \_index.md + overview.md
- [x] Read 5 topic \_index.md files
- [x] Update 5 FP overviews with "Rust as an FP-Adjacent Member — With Concept Adjustments" section
  - [x] patterns-and-principles/in-fp-by-example/overview.md
  - [x] domain-driven-design-ddd/in-fp-by-example/overview.md
  - [x] hexagonal-architecture/in-fp-by-example/overview.md
  - [x] finite-state-machine-fsm/in-fp-by-example/overview.md
  - [x] cases/in-fp/overview.md
- [x] Update 5 OOP overviews with Go partial-fit / language-reach note
  - [x] patterns-and-principles/in-oop-by-example/overview.md
  - [x] domain-driven-design-ddd/in-oop-by-example/overview.md
  - [x] hexagonal-architecture/in-oop-by-example/overview.md (strong fit)
  - [x] finite-state-machine-fsm/in-oop-by-example/overview.md (Go + C + Rust native idioms)
  - [x] cases/in-oop/overview.md (procedural sibling pointer)
- [x] Create 5 in-procedural-by-example directories with \_index.md + overview.md
  - [x] patterns-and-principles/in-procedural-by-example/
  - [x] domain-driven-design-ddd/in-procedural-by-example/
  - [x] hexagonal-architecture/in-procedural-by-example/
  - [x] finite-state-machine-fsm/in-procedural-by-example/
  - [x] cases/in-procedural/
- [x] Update parent \_index.md (3 paradigm tracks per topic)
- [x] Update parent overview.md (3-track umbrella framing)
- [x] Update 5 topic \_index.md files with procedural sub-link
- [x] Create plan documents (README, brd, prd, tech-docs, delivery)
- [x] Lint + format all markdown
- [x] Commit + push to origin main
- [x] Deploy to prod-ayokoding-web

## Stage 2 — Tier Content Rollout (Shipped 2026-05-24)

### Patterns and Principles (in-procedural-by-example)

- [x] beginner.md — Go canonical for Examples 1–28 with Rust trait/`?`-operator tabs where the idiom changes
- [x] intermediate.md — Go canonical for Examples 29–57 with Rust adjustments for state/strategy/observer
- [x] advanced.md — Go canonical for Examples 58–77 with Rust adjustments for reactive/observability/saga

### DDD (in-procedural-by-example)

- [x] beginner.md — Tactical patterns in Go (Boyle Ch. 2 style); Rust typestate-aggregate adjustments
- [x] intermediate.md — Integration patterns in Go (Three Dots Labs reference); Rust ownership-driven aggregates
- [x] advanced.md — Strategic patterns in Go; Rust + murabaha-finance Sharia procurement extension

### Hexagonal Architecture (in-procedural-by-example)

- [x] beginner.md — Three zones, structural interface satisfaction, in-memory adapter, composition root in main.go
- [x] intermediate.md — Adapter swap, integration test seam, Anti-Corruption Layer, multi-context wiring
- [x] advanced.md — Production wiring, retry/circuit-breaker decorators, observability adapter, outbox pattern

### Finite State Machine (in-procedural-by-example)

- [x] beginner.md — Rust typestate canonical + Go looplab/fsm side-by-side for PurchaseOrder lifecycle
- [x] intermediate.md — Invoice state machine, three-way match guards; Rust typestate w/ phantom types
- [x] advanced.md — Hierarchical states (Rust nested typestate, Go workaround, C Samek statecharts), parallel regions, saga

### Cases (in-procedural)

- [x] beginner.md — Guides 1–6: one context = one hexagon in Go and Rust
- [x] intermediate.md — Guides 7–14: Postgres adapter, in-memory test adapter, event publisher, cross-context ACL
- [x] advanced.md — Guides 15–27: full production wiring, observability, retry, outbox, deployment topology

### Stage 2 Acceptance Criteria

- [x] All 15 tier files exist and follow the five-part example format
- [x] Annotation density 1.0–2.25 comment lines per code line per tab in every file
- [x] Go is canonical in every topic except FSM (Rust typestate canonical, Go secondary)
- [x] Rust appears as a tab in every topic
- [x] C appears only in FSM intermediate / advanced (Samek sidebar)
- [x] Running domain is `procurement-platform-be` (parity with OOP and FP tracks)
- [x] Cross-track example numbering parity maintained where applicable
- [x] All examples lint clean (markdownlint + prettier)
- [x] Authority citations match the citation matrix in tech-docs.md
- [x] Mermaid diagrams follow accessible-color-palette convention

## Notes

- **Stage 2 is multi-session**. Each tier file is 5,000–10,000 annotated lines; authoring across all 15 files spans many working sessions.
- **Order recommendation for Stage 2**: hexagonal-procedural beginner first (Go's strongest fit, fastest to write), then fsm-procedural beginner (Rust typestate canonical idiom is highest-value content), then ddd-procedural beginner, then proceed by tier across all topics.
- **Spawn agent help**: by-example-maker agent in apps-ayokoding-web-by-example-maker is the right delegate for each tier file once the structure is set.
