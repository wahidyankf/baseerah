# 21 · Object-Oriented Design & Patterns (By Example, Python)

**prd row**: Pass 2 · Depth, Design & Craft · By Example · Python · Learn 121 / Drill 221 · Nvim-ready Yes ·
VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: deep object-oriented **design** — SOLID, coupling/cohesion, and the essential Gang-of-Four
patterns — each taught as a code smell → refactor. The OO **mechanics** (classes, inheritance,
polymorphism) are prerequisites from
[`08-object-oriented-programming-essentials`](./08-object-oriented-programming-essentials.md); this topic
is about designing well with them. Domain modeling at scale continues in
[`43-domain-driven-design`](./43-domain-driven-design.md).

## Why this exists · the big idea

- **The problem before the solution**: OO mechanics let you build classes; they don't stop you building a
  rigid tangle where every change ripples outward. Design is what keeps a growing system **soft** —
  changeable without fear.
- **Keep-this-if-you-forget-everything**: depend on abstractions, not concretions, and put each
  responsibility where change is isolated — most patterns are just named tactics for that one move.
- **Big ideas touched**: `coupling-vs-cohesion` (the core lens), `abstraction-and-its-cost` (an interface
  buys pluggability and charges indirection), `taming-state` (encapsulation as a state-containment
  strategy).

## Prerequisites

- **Prior topics**: [topic 8 Object-Oriented Programming Essentials](./08-object-oriented-programming-essentials.md)
  (classes, inheritance, polymorphism, composition) and [topic 4 Just Enough Python](./04-just-enough-python.md).
- **Tools & environment**: a macOS/Linux terminal; **Python 3.x**; `pytest` to lock refactors behind tests.
- **Assumed knowledge**: writing Python classes; the difference between composition and inheritance;
  reading/writing a basic unit test (topic 15 helps but is not required).

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

## Tensions & trade-offs — when NOT to reach for this

- **Pattern vs YAGNI**: a strategy or factory earns its indirection only when a second variant exists or is
  imminent; applied to a single case it is speculative generality — extra classes that hide straight-line
  logic behind ceremony.
- **Inheritance vs composition**: inheritance couples a subclass to its superclass's internals (the fragile
  base class); reach for it only for genuine substitutable is-a hierarchies, and prefer composition
  otherwise — the default, not the fallback.
- **When NOT to use it**: a small script, a one-off, or a stable spec with no real axis of change. SOLID
  and patterns are insurance against change; insurance you don't need is pure cost, and over-applied they
  make a codebase _harder_ to read, not easier.

## Lineage — why it beat the alternative

- The GoF catalogue (1994) named patterns that kept recurring in C++/Smalltalk codebases; SOLID (Robert
  Martin, 2000s) distilled the principles underneath them. Both were a reaction to inheritance-heavy 1990s
  OO producing rigid, fragile hierarchies (the "you wanted a banana, you got a gorilla holding the whole
  jungle" problem). The lesson is not the 23 patterns as a checklist but the **pressure** that produced
  them: unmanaged coupling makes systems ossify. So the durable skill is reading coupling and cohesion and
  judging which tactic — or none — relieves it; that same judgment carries forward into
  [`43-domain-driven-design`](./43-domain-driven-design.md) and [`42-software-architecture`](./42-software-architecture.md).

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

## Read more

**Books**

- **Design Patterns: Elements of Reusable Object-Oriented Software** — Erich Gamma, Richard Helm, Ralph Johnson, John Vlissides (1994). The original "Gang of Four" catalog of 23 patterns that defined the vocabulary of object-oriented design.
- **Object-Oriented Software Construction** — Bertrand Meyer (1997, 2nd ed.). Foundational text on OO design principles; introduced Design by Contract and the Open/Closed Principle. <https://bertrandmeyer.com/wp-content/upLoads/OOSC2.pdf>
- **Refactoring: Improving the Design of Existing Code** — Martin Fowler (1999; 2nd ed. 2018). Canonical catalog of code smells and refactorings for evolving object-oriented designs safely.
- **Agile Software Development: Principles, Patterns, and Practices** — Robert C. Martin (2002). Introduced the SOLID principles alongside worked object-oriented design case studies.
- **Head First Design Patterns** — Eric Freeman & Elisabeth Robson (2004; 2nd ed. 2020). The most widely used accessible introduction to the GoF patterns.

**Papers & articles**

- **Design Principles and Design Patterns** — Robert C. Martin (2000). The original paper naming the design principles later branded as SOLID. <https://staff.cs.utu.fi/~jounsmed/doos_06/material/DesignPrinciplesAndPatterns.pdf>

---

← Previous: [20 · Computer Architecture](./20-computer-architecture.md) · Next: [22 · Programming Paradigms](./22-programming-paradigms.md) →
