# 18 · Functional Programming (By Example, Python)

**prd row**: Pass 2 · Solidify the Core · By Example · Python · Learn 118 / Drill 218 · Nvim-ready Yes ·
VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: functional programming as an everyday discipline in Python — purity, immutability,
higher-order functions, composition, and the algebraic error-handling patterns (`Option`/`Result`), plus
a **gentle, practical** first exposure to functors/monoids/monads as code patterns. The deeper,
law-checking treatment lives in [`57-type-systems`](./57-type-systems.md); the functional-core /
imperative-shell idea recurs across the whole curriculum.

## Prerequisites

- **Prior topics**: [topic 04 Just Enough Python](./04-just-enough-python.md) (functions, closures,
  comprehensions, generators); [topic 06 Data Structures & Algorithms Essentials](./06-data-structures-and-algorithms-essentials.md)
  for the data being transformed; contrasts against [topic 07 OOP](./07-object-oriented-programming-essentials.md).
- **Tools & environment**: a macOS/Linux terminal; **Python 3.x** (`functools`, `itertools` from the
  stdlib); `pytest` for the purity/refactor examples.
- **Assumed knowledge**: writing Python functions and comprehensions; the idea of a side effect; basic
  generators/iterators.

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: `functools.reduce`/`partial`, `itertools`, and generator/`yield` semantics
  (PEP 342/380) are stable unchanged stdlib APIs, no recent breaking changes. The gentle functor/monad
  framing is a pedagogical choice, not a hard fact — safe as long as content stays practical and avoids
  overclaiming category-theory rigor (file already scopes it correctly). (docs.python.org)

## Items

- Pure functions & referential transparency; side effects & where to push them (functional core /
  imperative shell).
- Immutability & persistent data; first-class & higher-order functions; closures.
- `map`/`filter`/`reduce`; comprehensions; function composition; currying & partial application.
- Recursion & tail-call intuition; laziness / generators.
- Algebraic thinking: `Option`/`Maybe`, `Result`/`Either` error handling; functors/monads (gentle,
  practical framing).
- **Applied category theory (gentle introduction)**: functors, monoids, and monads named and shown as
  everyday code patterns (mappable containers, combinable values, chainable effects) — never abstract math.
- Managing state functionally; reducing shared mutable state.

## Worked examples

Colocated under `functional-programming/learning/code/`; each a runnable pure/impure contrast (DD-20/DD-30).

- **beginner** — refactor a loop to `map`/`filter`/`reduce`; pure vs impure versions of one function.
- **intermediate** — compose small functions into a pipeline; `Option`/`Result` for safe parsing; closures
  for configurable behavior.
- **advanced** — a functional-core/imperative-shell restructuring of a stateful task; a lazy generator
  pipeline; a `Maybe`/`Either` chain replacing exception control flow.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: build a small data-processing tool (e.g. a log/CSV analyzer) as a **functional core +
  imperative shell**: pure transformation pipeline (`map`/`filter`/`reduce` + composition), `Result`-based
  error handling instead of exceptions, immutable data, and a thin I/O shell — with property tests
  asserting purity/invariants.
- **Concepts exercised**: [ ] pure functions + referential transparency [ ] composition pipeline
  [ ] `Result`/`Option` error handling [ ] immutability [ ] functional core / imperative shell split
  [ ] a functor/monoid pattern used in earnest.
- **Ordered steps**:
  1. `.../learning/capstone/code/core.py` — pure parse→transform→aggregate functions, no I/O. Verify a
     `pytest` suite (incl. a Hypothesis invariant) passes with no mocking needed.
  2. `shell.py` — the imperative shell reading a file and calling the core. Verify it produces the report
     end to end from the CLI.
  3. Replace exception control flow with a `Result`/`Either` chain. Verify malformed rows yield a
     collected error result, not a crash.
  4. Show one functor/monoid pattern (e.g. combining partial aggregates monoidally). Verify the combined
     result equals the whole-in-one-pass result.
- **Acceptance criteria**: the core is pure and tested without mocks; errors are values not exceptions; the
  shell is the only place with I/O; the tool runs end to end.
- **Done bar**: runnable end-to-end + web-verified.

---

← Previous: [17 · Programming Paradigms](./17-programming-paradigms.md) · Next: [19 · Concurrency & Parallelism (Core)](./19-concurrency-and-parallelism.md) →
