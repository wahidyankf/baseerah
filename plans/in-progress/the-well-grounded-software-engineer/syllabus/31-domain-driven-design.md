# 31 · Domain-Driven Design (By Example, Python)

**prd row**: Pass 3 · Build for the Real World · By Example · Python · Learn 131 / Drill 231 · Nvim-ready
Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: tactical + strategic DDD as runnable code — entities, value objects, aggregates,
repositories, domain events, bounded contexts, context maps, and anti-corruption layers. The catalogue
entry lives in [`30-software-architecture`](./30-software-architecture.md); this is the deep, hands-on
teaching of it. Domain events connect forward to
[`33-event-driven-architecture`](./33-event-driven-architecture.md).

## Prerequisites

- **Prior topics**: [topic 16 Object-Oriented Design & Patterns](./16-object-oriented-design-and-patterns.md)
  (encapsulation, invariants), [topic 30 Software Architecture](./30-software-architecture.md) (bounded
  contexts, ports), and [topic 07 Object-Oriented Programming Essentials](./07-object-oriented-programming-essentials.md).
- **Tools & environment**: a macOS/Linux terminal; **Python 3.x** with a pinned CVE-clean test runner;
  Neovim/VSCode (DD-17). No DB required for the core — persistence is behind a repository port.
- **Assumed knowledge**: classes/invariants (topic 07/16); the idea of a domain core separated from I/O
  (topic 30); writing a unit test.

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: DDD terminology (entities, value objects, aggregates/aggregate roots,
  repositories, domain events, ubiquitous language, bounded contexts, context maps, anti-corruption layer)
  is unchanged canon from Evans (2003) + Vernon's _Implementing DDD_ (2013). Vernon's four aggregate-design
  rules of thumb (protect true invariants inside consistency boundaries; small aggregates; reference other
  aggregates by identity; update others via eventual consistency) remain the canonical reference.
  (archi-lab.io / learn.microsoft.com anti-corruption-layer)

## Items

- Tactical patterns: entities, value objects, aggregates + aggregate roots, invariants, domain services.
- Repositories: the domain-facing persistence port (implementation stays out of the domain).
- Domain events: modeling meaningful state changes as first-class events.
- Ubiquitous language: naming the code after the domain.
- Strategic design: bounded contexts, context maps, anti-corruption layers (ACL).
- When DDD pays off (and when it is overkill).

## Worked examples

Colocated under `domain-driven-design/learning/code/`; each runnable + unit-tested (DD-20/DD-30).

- **beginner** — a value object (equality by value, immutable) + an entity (identity) with an invariant.
- **intermediate** — an aggregate root enforcing an invariant across child entities; a repository port.
- **advanced** — two bounded contexts talking through an anti-corruption layer; a domain event emitted on a
  state change.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: model one non-trivial domain (e.g. orders/inventory) with proper DDD tactical patterns —
  value objects, an aggregate root that enforces its invariants, a repository port with an in-memory
  adapter, domain events on key transitions — and split it into two bounded contexts connected by an
  anti-corruption layer, all unit-tested with the domain core free of infrastructure.
- **Concepts exercised**: [ ] value objects (immutable, value equality) [ ] an aggregate root + invariant
  [ ] a repository port + in-memory adapter [ ] domain events [ ] two bounded contexts [ ] an ACL
  translating between them.
- **Ordered steps**:
  1. `.../learning/capstone/code/domain/` — value objects + an entity with identity. Verify value equality
     and an invariant rejection via unit tests.
  2. Add an aggregate root enforcing an invariant across children + a repository port. Verify the invariant
     holds through the root and the port has an in-memory adapter.
  3. Emit domain events on key transitions. Verify the right event fires with the right payload on each
     transition.
  4. Split into two bounded contexts + an ACL translating one context's model to the other. Verify the ACL
     maps correctly and neither context leaks the other's model.
- **Acceptance criteria**: aggregate invariants cannot be violated through the root; the domain core imports
  no infrastructure; events fire correctly; the ACL isolates the two contexts; all unit tests green.
- **Done bar**: runnable end-to-end + web-verified.

---

← Previous: [30 · Software Architecture](./30-software-architecture.md) · Next: [32 · System Design](./32-system-design.md) →
