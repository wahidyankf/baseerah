# 23 · Functional Programming (By Example, Python)

**prd row**: Pass 2 · Depth, Design & Craft · By Example · Python · Learn 123 / Drill 223 · Nvim-ready Yes ·
VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: functional programming as an everyday discipline in Python — purity, immutability,
higher-order functions, composition, and the algebraic error-handling patterns (`Option`/`Result`), plus
a **gentle, practical** first exposure to functors/monoids/monads as code patterns. The deeper,
law-checking treatment lives in [`84-type-systems`](./84-type-systems.md); the functional-core /
imperative-shell idea recurs across the whole curriculum.

## Why this exists · the big idea

- **The problem before the solution**: shared mutable state is the root of the hardest bugs — action at a
  distance, failures you can't reproduce, and a codebase you're afraid to change because nothing is local.
- **Keep-this-if-you-forget-everything**: push side effects to the edges and keep a pure core — code that
  only maps inputs to outputs is code you can test, reason about, and parallelize without fear.
- **Big ideas touched**: `taming-state` (the central move — quarantine state and effects), `determinism-vs-emergence`
  (purity buys deterministic, replayable behavior), `abstraction-and-its-cost` (immutability costs allocations).

## Prerequisites

- **Prior topics**: [topic 4 Just Enough Python](./04-just-enough-python.md) (functions, closures,
  comprehensions, generators); [topic 7 Data Structures & Algorithms Essentials](./07-data-structures-and-algorithms-essentials.md)
  for the data being transformed; contrasts against [topic 8 OOP](./08-object-oriented-programming-essentials.md).
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

## Tensions & trade-offs — when NOT to reach for this

- **Purity vs the machine**: immutability allocates, and a tight numeric loop or a huge in-place buffer is a
  place an imperative core is honestly faster — insisting on purity there is dogma, not engineering.
- **Monad-all-the-things**: the algebraic patterns (`Result`/`Option`, functors, monads) buy composability
  and charge indirection plus a learning tax; a `Result` chain three layers deep can read _worse_ than an
  early `raise`. Reach for them where error-as-value genuinely simplifies, not everywhere.
- **When NOT to use it**: a fundamentally stateful, mutation-heavy domain (a game loop, a physics sim, a
  device driver) fights the paradigm head-on. The move is functional-core / imperative-shell to _quarantine_
  the state, not a crusade to abolish it.

## Lineage — why it beat the alternative

- FP traces to Church's lambda calculus (1930s) — a model of computation as pure function application that
  predates stored-program machines. It stayed academic (Lisp 1958, ML, Haskell 1990) until multicore and
  distributed systems made _shared mutable state_ the industry's dominant pain: the property FP had all
  along — referential transparency — became the practical answer to concurrency and testability. That is why
  "reduce shared mutable state" now surfaces inside mainstream OO languages (immutable records, `map`/`filter`,
  `Optional`). The lesson is not purity-as-religion but that _controlling where state and effects live_ is the
  leverage — the same functional-core / imperative-shell split this repo is built on, and the ground
  [`84-type-systems`](./84-type-systems.md) later makes rigorous.

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

## Read more

**Books**

- **Structure and Interpretation of Computer Programs** — Harold Abelson & Gerald Jay Sussman (1985; 2nd ed. 1996). Teaches functional-programming fundamentals — closures, higher-order functions, recursion — as its core method. <https://mitp-content-server.mit.edu/books/content/sectbyfn/books_pres_0/6515/sicp.zip/full-text/book/book.html>
- **Learn You a Haskell for Great Good!** — Miran Lipovača (2011). The most widely used friendly introduction to pure functional programming and Haskell's type system. <https://learnyouahaskell.com/>
- **Programming in Haskell** — Graham Hutton (2007; 2nd ed. 2016). Rigorous, widely adopted undergraduate functional programming textbook.
- **Functional Programming in Scala** — Paul Chiusano & Rúnar Bjarnason (2014). Canonical text teaching algebraic data types and pure-function design in a hybrid OO/FP language.

**Papers & articles**

- **Why Functional Programming Matters** — John Hughes (1989). Classic paper arguing that higher-order functions and lazy evaluation are what make functional programming modular. <https://www.cs.kent.ac.uk/people/staff/dat/miranda/whyfp90.pdf>
- **Out of the Tar Pit** — Ben Moseley & Peter Marks (2006). Influential essay diagnosing software complexity and proposing a functional-relational remedy. <https://curtclifton.net/papers/MoseleyMarks06a.pdf>

---

← Previous: [22 · Programming Paradigms](./22-programming-paradigms.md) · Next: [24 · Concurrency & Parallelism](./24-concurrency-and-parallelism.md) →
