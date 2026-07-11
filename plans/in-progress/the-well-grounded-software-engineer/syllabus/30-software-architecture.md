# 30 · Software Architecture (Annotated-concept, Python \*)

**prd row**: Pass 3 · Build for the Real World · Annotated-concept · Python \* · Learn 130 / Drill 230 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: architectural styles and the trade-off thinking behind them — layered, hexagonal
(ports-and-adapters), functional core/imperative shell, monolith vs microservices — plus quality
attributes, boundaries/modularity, C4 documentation, and evolutionary architecture. Event-driven has its
own topic ([`33-event-driven-architecture`](./33-event-driven-architecture.md)); tactical DDD is
catalogued here and taught deeply in [`31-domain-driven-design`](./31-domain-driven-design.md). `*`:
Python where code appears, else annotated C4 diagrams.

## Prerequisites

- **Prior topics**: [topic 16 Object-Oriented Design & Patterns](./16-object-oriented-design-and-patterns.md)
  (coupling/cohesion, DIP), [topic 18 Functional Programming](./18-functional-programming.md) (functional
  core/imperative shell), and a built app from Pass 1/3 (e.g.
  [topic 28 Backend at Scale](./28-backend-at-scale.md)) to reason about.
- **Tools & environment**: a macOS/Linux terminal; **Python 3.x** for the ports-and-adapters code; a
  Markdown/Mermaid editor for C4 diagrams and ADRs (Neovim per DD-17).
- **Assumed knowledge**: dependency inversion and interfaces (topic 16); the idea of separating a domain
  core from I/O (topic 18); reading a system diagram.

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: the C4 model (Context/Container/Component/Code, Simon Brown), hexagonal /
  ports-and-adapters (Cockburn 2005), strangler-fig (Fowler), and fitness functions (_Building Evolutionary
  Architectures_, 2017) are stable unchanged canonical terminology — no revision. (c4model.com)
- 2026-07-12 — verified (evolving-discourse flag): microservices-vs-monolith is a genuinely evolving debate,
  not a settled fact (e.g. Amazon Prime Video's 2023 move away from microservices). The file's trade-off
  framing is fine; content drafted here should reflect the current "boring / modular-monolith-first"
  counter-narrative, not an unqualified "microservices are the scale answer." (industry sources)

## Items

- Architectural styles catalogue: layered, hexagonal / ports-and-adapters (first-class style here),
  functional core/imperative shell, event-driven (own topic), microservices vs monolith.
- Quality attributes: modifiability, scalability, availability, performance, security — trade-off thinking.
- Boundaries & modularity: coupling/cohesion, dependency direction, DIP at scale; bounded contexts + DDD
  tactical patterns catalogued here, taught deeply in `domain-driven-design`.
- Cross-cutting concerns: config, logging, error handling, transactions across boundaries.
- Documentation: the C4 model, ADRs, diagrams-as-communication.
- Evolutionary architecture: fitness functions, strangler-fig migration.

## Worked examples

Colocated under `software-architecture/learning/`; annotated C4 Mermaid diagrams + a runnable
ports-and-adapters Python example (DD-20/DD-30).

- **modular-refactor** — a monolith → modular-boundaries refactor, shown as before/after C4-style Mermaid
  diagrams.
- **ports-and-adapters** — a domain core isolated from an interchangeable adapter (runnable Python: swap
  adapter without touching the core).
- **adr-tradeoff** — an ADR trade-off analysis for one significant decision (e.g. sync vs async
  integration).

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: take a small tangled service and re-architect it to ports-and-adapters with a functional core:
  isolate the domain from infrastructure behind ports, provide two interchangeable adapters (e.g. SQL vs
  in-memory), document the before/after with C4 diagrams, and record an ADR for the key trade-off — a
  runnable proof that the core is infrastructure-free.
- **Concepts exercised**: [ ] ports-and-adapters boundary [ ] functional core / imperative shell [ ] DIP
  (core depends on abstractions) [ ] two interchangeable adapters [ ] C4 before/after diagrams [ ] an ADR.
- **Ordered steps**:
  1. `.../learning/capstone/code/` — the tangled baseline + a characterization test. Verify the test pins
     current behavior.
  2. Extract ports (interfaces) and move the domain into an infrastructure-free core. Verify the core
     imports no I/O module and the test stays green.
  3. Provide two adapters (SQL + in-memory) behind the same port. Verify swapping the adapter changes no
     core code and both pass the test.
  4. `architecture.md` — before/after C4 (context + container + component) Mermaid + an ADR for the key
     decision. Verify the diagrams match the code and the ADR states context/decision/consequences.
- **Acceptance criteria**: the domain core is provably free of infrastructure imports; adapters are
  interchangeable without core edits; C4 diagrams and the ADR match the implementation.
- **Done bar**: runnable end-to-end (adapter swap) + produces the C4/ADR artifacts + web-verified.

---

← Previous: [29 · Advanced Frontend](./29-advanced-frontend.md) · Next: [31 · Domain-Driven Design](./31-domain-driven-design.md) →
