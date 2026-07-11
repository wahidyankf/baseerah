# 16 · Object-Oriented Design & Patterns (By Example, Python)

**prd row**: Pass 2 · Solidify the Core · By Example · Python · Learn 116 / Drill 216 · Nvim-ready Yes ·
VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: deep object-oriented **design** — SOLID, coupling/cohesion, and the essential Gang-of-Four
patterns — each taught as a code smell → refactor. The OO **mechanics** (classes, inheritance,
polymorphism) are prerequisites from
[`07-object-oriented-programming-essentials`](./07-object-oriented-programming-essentials.md); this topic
is about designing well with them. Domain modeling at scale continues in
[`31-domain-driven-design`](./31-domain-driven-design.md).

## Prerequisites

- **Prior topics**: [topic 07 Object-Oriented Programming Essentials](./07-object-oriented-programming-essentials.md)
  (classes, inheritance, polymorphism, composition) and [topic 04 Just Enough Python](./04-just-enough-python.md).
- **Tools & environment**: a macOS/Linux terminal; **Python 3.x**; `pytest` to lock refactors behind tests.
- **Assumed knowledge**: writing Python classes; the difference between composition and inheritance;
  reading/writing a basic unit test (topic 13 helps but is not required).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: SOLID + the GoF pattern catalogue are unchanged canon since 1994 (no revision).
  `typing.Protocol` (PEP 544, 3.8+) remains the current idiomatic structural-typing mechanism for
  strategy-style duck typing; `functools` (`wraps`, `partial`, decorator factories) remains the current
  stdlib decorator idiom — no deprecation or replacement. (docs.python.org / GoF canon)

## Items

- SOLID principles, each shown as a code smell → refactor.
- Law of Demeter; cohesion & coupling; composition over inheritance (deep).
- Essential design patterns: strategy, factory, observer, adapter, decorator, singleton (+ its costs),
  template method, command.
- Refactoring to patterns; common anti-patterns; when **not** to reach for a pattern.
- Immutability and value vs reference semantics at design scale.

## Worked examples

Colocated under `object-oriented-design-and-patterns/learning/code/`; each a runnable before→after refactor
with a locking test (DD-20/DD-30).

- **beginner** — strategy pattern for pluggable pricing; factory for object creation.
- **intermediate** — observer for events; adapter wrapping a foreign API; decorator for cross-cutting
  behavior.
- **advanced** — refactor an inheritance hierarchy to composition applying SOLID; a small domain model
  applying several patterns cohesively.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: take a deliberately smelly small system (e.g. an order/pricing engine) and re-engineer it
  applying SOLID and a coherent set of patterns (strategy + factory + observer + decorator), keeping a
  test suite green through every refactor step — ending with a clean, extensible design.
- **Concepts exercised**: [ ] each SOLID principle applied to a real smell [ ] composition over inheritance
  [ ] strategy + factory + observer + decorator used cohesively [ ] an anti-pattern removed [ ] tests
  green through every step.
- **Ordered steps**:
  1. `.../learning/capstone/code/` — the smelly baseline + a `pytest` suite pinning current behavior.
     Verify the suite passes against the baseline.
  2. Refactor to SOLID (SRP/OCP first): extract responsibilities, invert a dependency. Verify tests stay
     green after each move.
  3. Introduce strategy (pluggable pricing) + factory (creation) + observer (events) + decorator
     (cross-cutting). Verify a new pricing rule/event can be added without editing existing classes (OCP).
  4. Document the before→after design with a Mermaid class diagram. Verify the diagram matches the code.
- **Acceptance criteria**: behavior is unchanged (suite green throughout); extending the system needs no
  edits to closed classes; each applied principle/pattern is justified in prose.
- **Done bar**: runnable end-to-end + web-verified.

---

← Previous: [15 · Computer Science Foundations](./15-computer-science-foundations.md) · Next: [17 · Programming Paradigms](./17-programming-paradigms.md) →
