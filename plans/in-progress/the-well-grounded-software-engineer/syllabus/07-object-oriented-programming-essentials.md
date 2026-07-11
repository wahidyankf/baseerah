# 07 · Object-Oriented Programming Essentials (By Example, Python)

**prd row**: Pass 1 · First Working Software · By Example · Python · Learn 107 / Drill 207 · Nvim-ready
Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the **usable slice** of OOP — enough to model a domain cleanly. SOLID, design patterns,
and deeper design go to [`16-object-oriented-design-and-patterns`](./16-object-oriented-design-and-patterns.md)
(split-and-interleave, DD-11).

## Prerequisites

- **Prior topics**: [topic 04 Just Enough Python](./04-just-enough-python.md) (classes, functions,
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

---

← Previous: [06 · Data Structures & Algorithms Essentials](./06-data-structures-and-algorithms-essentials.md) · Next: [08 · SQL Essentials](./08-sql-essentials.md) →
