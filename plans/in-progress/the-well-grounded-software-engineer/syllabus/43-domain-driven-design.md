# 43 · Domain-Driven Design (By Example, Python)

**prd row**: Pass 3 · Build for the Real World · By Example · Python · Learn 143 / Drill 243 · Nvim-ready
Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: tactical + strategic DDD as runnable code — entities, value objects, aggregates,
repositories, domain events, bounded contexts, context maps, and anti-corruption layers. The catalogue
entry lives in [`42-software-architecture`](./42-software-architecture.md); this is the deep, hands-on
teaching of it. Domain events connect forward to
[`45-event-driven-architecture`](./45-event-driven-architecture.md).

## Why this exists · the big idea

- **The problem before the solution**: code that models database tables instead of the business drifts from
  how domain experts think — the translation tax shows up as bugs, miscommunication, and rules enforced in
  the wrong place.
- **Keep-this-if-you-forget-everything**: name the code after the domain and put each invariant inside a
  single consistency boundary (an aggregate) so the rule has exactly one home and cannot be violated from
  outside.
- **Big ideas touched**: `coupling-vs-cohesion` (bounded contexts draw the seams), `taming-state`
  (an aggregate root is a consistency boundary quarantining invariant-protected state),
  `correctness-vs-pragmatism` (DDD pays off on complex domains and is overkill on simple ones).

## Prerequisites

- **Prior topics**: [topic 21 Object-Oriented Design & Patterns](./21-object-oriented-design-and-patterns.md)
  (encapsulation, invariants), [topic 42 Software Architecture](./42-software-architecture.md) (bounded
  contexts, ports), and [topic 8 Object-Oriented Programming Essentials](./08-object-oriented-programming-essentials.md).
- **Tools & environment**: a macOS/Linux terminal; **Python 3.x** with a pinned CVE-clean test runner;
  Neovim/VSCode (DD-17). No DB required for the core — persistence is behind a repository port.
- **Assumed knowledge**: classes/invariants (topic 08/16); the idea of a domain core separated from I/O
  (topic 42); writing a unit test.

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

## Tensions & trade-offs — when NOT to reach for this

- **Ceremony vs simplicity**: value objects, aggregates, repositories, and ACLs are a lot of scaffolding; on
  a CRUD app with no real invariants they add indirection and buy nothing. DDD's own answer is to apply the
  tactical patterns only where the domain is genuinely complex.
- **Aggregate boundaries**: too-large aggregates kill concurrency (everything locks the root); too-small ones
  can't protect their invariant. Vernon's "small aggregates, reference others by identity, eventual
  consistency across them" is a hard-won balance, not a default to reach for blindly.
- **When NOT to use it**: a generic/technical subdomain (a mailer, a PDF exporter) needs no ubiquitous
  language or bounded context — buy or use a library. Spend the modeling effort on the _core_ domain that
  differentiates the business.

## Lineage — why it beat the alternative

- DDD (Evans 2003) reacted to two failures: anemic data-model code that scattered business rules across
  services, and the analysis-paralysis of trying to model an entire enterprise at once. Its move was to
  align code with the domain's language and to divide the model into bounded contexts so each stays
  internally consistent — the same "boundaries so things that change together stay together" idea as
  [`42-software-architecture`](./42-software-architecture.md), aimed at the domain. Vernon's _Implementing
  DDD_ (2013) added the tactical rules of thumb. The domain events modeled here become the backbone of
  [`45-event-driven-architecture`](./45-event-driven-architecture.md).

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

## Read more

**Books**

- **Domain-Driven Design: Tackling Complexity in the Heart of Software** — Eric Evans (2003). The original book that coined DDD and its vocabulary (bounded context, aggregate, ubiquitous language).
- **Implementing Domain-Driven Design** — Vaughn Vernon (2013). The standard practical/tactical companion showing how to apply Evans's strategic patterns in real codebases.
- **Domain-Driven Design Distilled** — Vaughn Vernon (2016). Concise, widely recommended on-ramp to strategic DDD concepts.

**Papers & articles**

- **Domain-Driven Design Reference** — Eric Evans (2015). Free, author-published summary of every pattern and definition from the original book, under Creative Commons. <https://www.domainlanguage.com/wp-content/uploads/2016/05/DDD_Reference_2015-03.pdf>
- **Bounded Context** — Martin Fowler (2014). The widely cited bliki explanation of DDD's central strategic-design concept. <https://martinfowler.com/bliki/BoundedContext.html>

---

← Previous: [42 · Software Architecture](./42-software-architecture.md) · Next: [44 · System Design](./44-system-design.md) →
