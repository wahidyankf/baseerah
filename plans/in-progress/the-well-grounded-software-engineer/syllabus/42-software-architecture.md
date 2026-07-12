# 42 · Software Architecture (Annotated-concept, Python \*)

**prd row**: Pass 3 · Build for the Real World · Annotated-concept · Python \* · Learn 142 / Drill 242 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: architectural styles and the trade-off thinking behind them — layered, hexagonal
(ports-and-adapters), functional core/imperative shell, monolith vs microservices — plus quality
attributes, boundaries/modularity, C4 documentation, and evolutionary architecture. Event-driven has its
own topic ([`45-event-driven-architecture`](./45-event-driven-architecture.md)); tactical DDD is
catalogued here and taught deeply in [`43-domain-driven-design`](./43-domain-driven-design.md). `*`:
Python where code appears, else annotated C4 diagrams.

## Why this exists · the big idea

- **The problem before the solution**: past a certain size, a system's cost is dominated not by any one
  module but by how the modules depend on each other — the wrong boundaries make every change ripple.
- **Keep-this-if-you-forget-everything**: architecture is the deliberate placement of boundaries so that
  things that change together live together and things that don't are decoupled — you are buying
  changeability, and every boundary costs indirection.
- **Big ideas touched**: `coupling-vs-cohesion` (the fundamental lever), `layering-and-leaks`
  (layered/hexagonal styles are about honest boundaries that don't leak), `abstraction-and-its-cost`
  (each boundary buys isolation and charges indirection).

## Prerequisites

- **Prior topics**: [topic 21 Object-Oriented Design & Patterns](./21-object-oriented-design-and-patterns.md)
  (coupling/cohesion, DIP), [topic 23 Functional Programming](./23-functional-programming.md) (functional
  core/imperative shell), and a built app from Pass 1/3 (e.g.
  [topic 39 Backend at Scale](./39-backend-at-scale.md)) to reason about.
- **Tools & environment**: a macOS/Linux terminal; **Python 3.x** for the ports-and-adapters code; a
  Markdown/Mermaid editor for C4 diagrams and ADRs (Neovim per DD-17).
- **Assumed knowledge**: dependency inversion and interfaces (topic 21); the idea of separating a domain
  core from I/O (topic 23); reading a system diagram.

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

## Tensions & trade-offs — when NOT to reach for this

- **Microservices vs monolith**: microservices buy independent deploy/scale and charge network failure,
  distributed debugging, and operational overhead. The 2020s counter-narrative (modular-monolith-first;
  Amazon Prime Video's 2023 re-consolidation) is that most teams pay the distributed tax without needing it.
  Start modular-monolith; split only at a proven scaling or team-boundary seam.
- **Hexagonal everywhere**: ports-and-adapters buys testability and swappable infra and charges indirection.
  Wrapping a trivial CRUD app in ports is ceremony — apply it where the core is genuinely worth protecting.
- **When NOT to invest**: for a small, short-lived, or well-understood system, heavy architecture is
  speculative generality (YAGNI). Architecture earns its cost at scale, longevity, and team size.

## Lineage — why it beat the alternative

- Architecture patterns are reactions to the pain of large-system _change_. Layered architecture answered
  spaghetti; hexagonal (Cockburn 2005) answered business logic entangled with frameworks and the DB;
  microservices (2010s) answered monolith deploy-coupling at org scale; evolutionary architecture / fitness
  functions (2017) answered big-upfront-design failing against change. Each generation traded one rigidity
  for a new cost, and the pendulum is mid-swing back toward modular monoliths — so read the _current_
  pressure rather than cargo-culting the last era's answer. The tactical detail lives in
  [`43-domain-driven-design`](./43-domain-driven-design.md); the runtime-decoupled style in
  [`45-event-driven-architecture`](./45-event-driven-architecture.md).

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

## Read more

**Books**

- **Software Architecture in Practice** — Len Bass, Paul Clements, Rick Kazman (4th ed., 2021). The foundational SEI textbook defining quality attributes and architecture as an engineering discipline.
- **Documenting Software Architectures: Views and Beyond** — Paul Clements et al. (2nd ed., 2010). The standard reference for the multi-view approach to architecture documentation.
- **Fundamentals of Software Architecture** — Mark Richards, Neal Ford (2020). Widely adopted modern survey of architectural styles and architectural thinking.
- **Clean Architecture** — Robert C. Martin (2017). Influential synthesis of dependency-inversion-centric architecture principles.

**Papers & articles**

- **Architectural Blueprints — The "4+1" View Model of Software Architecture** — Philippe Kruchten (1995), IEEE Software. Introduced the multi-view approach to describing architecture that underlies most modern architecture documentation practice. <https://dl.acm.org/doi/10.1109/52.469759>
- **Documenting Architecture Decisions** — Michael Nygard (2011). The blog post that originated the Architecture Decision Record (ADR) format now standard across the industry. <https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions>

---

← Previous: [41 · API Design](./41-api-design.md) · Next: [43 · Domain-Driven Design](./43-domain-driven-design.md) →
