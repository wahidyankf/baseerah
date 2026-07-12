# 8 · Object-Oriented Programming Essentials (By Example, Python)

**prd row**: Pass 1 · Core Foundations · By Example · Python · Learn 108 / Drill 208 · Nvim-ready
Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the **usable slice** of OOP — enough to model a domain cleanly. SOLID, design patterns,
and deeper design go to [`21-object-oriented-design-and-patterns`](./21-object-oriented-design-and-patterns.md)
(split-and-interleave, DD-11).

## Why this exists · the big idea

- **The problem before the solution**: once data and the code that changes it drift apart, invariants
  break silently — anyone can put an object into an invalid state from anywhere.
- **Keep-this-if-you-forget-everything**: bundle state with the operations that guard it, and expose
  behavior, not fields — an object is a small guarantee about what stays true.
- **Big ideas touched**: `taming-state` (encapsulation contains mutable state behind an invariant),
  `coupling-vs-cohesion` (a well-shaped object is cohesive and narrowly coupled to the rest).

## Prerequisites

- **Prior topics**: [topic 4 Just Enough Python](./04-just-enough-python.md) (classes, functions,
  modules).
- **Tools & environment**: a macOS/Linux terminal; **Python 3.x** with `pytest` in a `venv`.
- **Assumed knowledge**: reading/writing basic Python including the one-line class preview from topic 04;
  no prior OOP background required.

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: `@dataclass` options current — `frozen` (raises on field assignment), `slots`
  (since 3.10, drops per-instance `__dict__`), `eq` (generates `__eq__`); when `eq` and `frozen` are both
  True, `__hash__` is auto-generated. Overriding `__eq__` without `__hash__` sets `__hash__ = None`
  (unhashable). `abc.ABC`/`abstractmethod` mechanics stable. (docs.python.org dataclasses/datamodel)

## Items

- **Classes, objects, fields, methods**, `__init__`, `self`, identity vs equality.
- **Four pillars introduced**: encapsulation, inheritance, polymorphism, abstraction.
- `__eq__` / `__hash__` / `__repr__`; `@dataclass` value objects.
- **Composition vs inheritance intro**; duck typing; abstract base classes intro.

## Worked examples

Colocated under `object-oriented-programming-essentials/learning/code/`; runnable + `pytest` (DD-20).

- **beginner** — a `BankAccount` with an encapsulated balance + invariants; equality/hash.
- **intermediate** — a small class hierarchy showing polymorphism; a dataclass value object.
- **advanced** — refactor a naive inheritance into composition; a duck-typed interface.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: model a small domain (e.g. a library or a payments ledger) as a clean object model —
  encapsulated invariants, a polymorphic operation, a `@dataclass` value object, and one composition
  refactor — as a runnable, tested package.
- **Concepts exercised**: [ ] encapsulated invariant enforced in `__init__`/setters [ ] polymorphism via
  a shared method across subclasses/duck types [ ] `@dataclass` value object with `__eq__`/`__hash__`
  [ ] composition over inheritance [ ] an `abc.ABC` interface.
- **Ordered steps**:
  1. `.../learning/capstone/code/domain/` — a value object (`@dataclass(frozen=True)`) + an entity with
     an invariant. Verify `pytest` rejects invalid construction.
  2. Add an `abc.ABC` interface with ≥2 implementations exercised polymorphically. Verify a single
     call-site handles all implementations.
  3. Refactor one naive inheritance chain into composition. Verify behavior unchanged (tests still green).
- **Acceptance criteria**: `pytest` green; invariants cannot be violated; the polymorphic call-site is
  implementation-agnostic; value object equality/hash behave correctly.
- **Done bar**: runnable end-to-end + web-verified.

## Read more

**Books**

- **Design Patterns: Elements of Reusable Object-Oriented Software** — Gamma, Helm, Johnson, Vlissides (1994, "Gang of Four"). Canonical catalog of 23 OO patterns and the shared vocabulary still used industry-wide.
- **Object-Oriented Analysis and Design with Applications** — Grady Booch (3rd ed., 2007). Foundational OO analysis/design methodology from a UML co-creator.
- **Effective Java** — Joshua Bloch (3rd ed., 2018). Java-specific but its item-based OO-design tradeoffs (composition vs inheritance, immutability, interfaces) are widely cited.

**Papers & articles**

- **"A Behavioral Notion of Subtyping"** — Liskov, Wing (1994, ACM TOPLAS). The formal paper defining the Liskov Substitution Principle. <https://www.cs.cmu.edu/~wing/publications/LiskovWing94.pdf>
- **"The Early History of Smalltalk"** — Alan Kay (1993, ACM SIGPLAN Notices). Kay's own account of the origin of "object-oriented." <https://dl.acm.org/doi/10.1145/155360.155364>

---

← Previous: [7 · Data Structures & Algorithms Essentials](./07-data-structures-and-algorithms-essentials.md) · Next: [9 · Project Management](./09-project-management.md) →
