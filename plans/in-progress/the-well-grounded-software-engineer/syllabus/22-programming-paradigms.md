# 22 · Programming Paradigms (By Example, Python \*\* survey)

**prd row**: Pass 2 · Depth, Design & Craft · By Example · Python \*\* · Learn 122 / Drill 222 · Nvim-ready
Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: a **survey** of the major paradigms and how to choose among them, anchored in Python
(`**`) with other languages shown illustratively. Functional programming has its own deep topic
([`23-functional-programming`](./23-functional-programming.md)); the concurrency-oriented paradigms deepen
in Pass 4 (CSP → Go, actor → Elixir). This topic's job is fluency in _matching paradigm to problem_.

## Why this exists · the big idea

- **The problem before the solution**: most engineers default to one paradigm and bend every problem to
  it — the wrong paradigm makes easy problems hard and hides the shape the problem actually has.
- **Keep-this-if-you-forget-everything**: a paradigm is a set of _constraints that buy a property_ (purity
  buys reasoning, objects buy encapsulation, logic buys search) — match the problem's grain, don't fight it.
- **Big ideas touched**: `abstraction-and-its-cost` (each paradigm is a lens with a bill attached),
  `taming-state` (paradigms differ most in how they treat mutable state — this is the real fault line).

## Prerequisites

- **Prior topics**: [topic 4 Just Enough Python](./04-just-enough-python.md),
  [topic 8 Object-Oriented Programming Essentials](./08-object-oriented-programming-essentials.md) (the OO
  paradigm), and [topic 21 Object-Oriented Design & Patterns](./21-object-oriented-design-and-patterns.md);
  functional style is cross-referenced forward to [topic 23](./23-functional-programming.md).
- **Tools & environment**: a macOS/Linux terminal; **Python 3.x** (all runnable examples); illustrative
  snippets in other languages are read-only (no extra toolchain required to follow).
- **Assumed knowledge**: comfortable writing Python in both procedural and OO styles; can read a small
  snippet in an unfamiliar language with explanation.

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: the paradigm taxonomy (imperative/procedural, OO, functional, logic,
  event-driven/reactive, dataflow) is a stable decades-old classification with no material redefinition.
  Any illustrative non-Python snippet must be re-checked against current language versions once actually
  authored (none exist in this pre-authoring file yet). (general CS canon)

## Items

- Imperative & procedural; structured programming.
- Object-oriented (cross-reference the OOP topics).
- Functional (cross-reference `functional-programming`); declarative vs imperative.
- Logic programming (Prolog-style intuition); constraint programming (survey).
- Event-driven & reactive; dataflow.
- Choosing a paradigm: matching paradigm to problem; multi-paradigm languages.

## Tensions & trade-offs — when NOT to reach for this

- **No paradigm is universal**: functional purity shines for transformation and logic but fights hardware
  and I/O; OO models entities well but drowns a simple script in ceremony; logic/constraint programming is
  magic for search and dead weight everywhere else. Loyalty to one paradigm is the failure mode.
- **Multi-paradigm is not paradigm soup**: mixing freely _inside_ one boundary — mutable OO objects threaded
  through a nominally "functional" pipeline — collects the costs of both and the benefits of neither. Choose a
  paradigm per boundary, not per line.
- **When NOT to care**: for a 50-line script the paradigm question is noise; reach for whatever is fastest to
  write. Paradigm choice earns its weight only once a system is big enough to have a dominant axis of change.

## Lineage — why it beat the alternative

- Each paradigm is a historical reaction to a specific pain the prior one couldn't hold. Structured programming
  (Dijkstra, "Go To Considered Harmful", 1968) beat goto-spaghetti by constraining control flow; OO
  (Simula/Smalltalk) rose to tame large mutable-state systems by bundling state with behavior; functional
  (Lisp → ML → Haskell) answered the reasoning-and-concurrency crisis by removing shared mutable state;
  reactive/dataflow answered UIs and streams. So the durable skill is not allegiance to one paradigm but
  reading _which pain a problem has_ — a judgment that feeds straight into
  [`23-functional-programming`](./23-functional-programming.md) and the concurrency paradigms of Pass 4
  ([`61-csp-style-concurrency`](./61-csp-style-concurrency.md), [`63-actor-model-concurrency`](./63-actor-model-concurrency.md)).

## Worked examples

Colocated under `programming-paradigms/learning/code/`; the same problem solved multiple ways for direct
contrast (DD-20/DD-30).

- **four-ways** — solve one small problem (word-frequency count) imperative, OO, functional, and
  declarative — same output, contrasted trade-offs.
- **declarative-vs-imperative** — express a rule set imperatively vs declaratively; show a reactive /
  event-driven counter.
- **decision-table** — a table mapping problem shapes to fitting paradigms, with reasoning.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: pick one non-trivial small problem and implement it in **four paradigms** (imperative, OO,
  functional, declarative/reactive) producing identical output, then write a decision record arguing which
  paradigm best fits the problem and why — a runnable, side-by-side paradigm comparison.
- **Concepts exercised**: [ ] imperative/procedural solution [ ] OO solution [ ] functional solution
  [ ] declarative or reactive solution [ ] a reasoned paradigm-selection decision.
- **Ordered steps**:
  1. `.../learning/capstone/code/` — define the problem + a shared test asserting the expected output.
     Verify the test exists and fails against empty implementations.
  2. Implement the imperative and OO versions. Verify both pass the shared test.
  3. Implement the functional and declarative/reactive versions. Verify both pass the shared test.
  4. `decision.md` — argue the best-fit paradigm with trade-offs (readability, testability, change-cost).
     Verify each claim references the concrete code.
- **Acceptance criteria**: all four implementations pass the identical test; the decision record is
  grounded in the code, not generic prose; trade-offs are concrete.
- **Done bar**: runnable end-to-end + web-verified.

## Read more

**Books**

- **Structure and Interpretation of Computer Programs** — Harold Abelson & Gerald Jay Sussman (1985; 2nd ed. 1996). Canonical MIT text teaching procedural, functional, object-oriented, and logic paradigms through a single Lisp substrate. <https://mitp-content-server.mit.edu/books/content/sectbyfn/books_pres_0/6515/sicp.zip/full-text/book/book.html>
- **Concepts, Techniques, and Models of Computer Programming** — Peter Van Roy & Seif Haridi (2004). Comprehensive graduate text organizing all major paradigms by their underlying computational models. <https://webperso.info.ucl.ac.be/~pvr/VanRoyHaridi2003-book.pdf>
- **Seven Languages in Seven Weeks** — Bruce Tate (2010). Practical survey spanning imperative, object-oriented, functional, and logic paradigms across seven languages.

**Papers & articles**

- **Programming Paradigms for Dummies: What Every Programmer Should Know** — Peter Van Roy (2009). Widely cited taxonomy of roughly thirty paradigms and how they relate. <https://webperso.info.ucl.ac.be/~pvr/VanRoyChapter.pdf>
- **Can Programming Be Liberated from the von Neumann Style? A Functional Style and Its Algebra of Programs** — John Backus (1978). Backus's ACM Turing Award lecture critiquing imperative programming and motivating the functional paradigm. <https://dl.acm.org/doi/10.1145/359576.359579>

---

← Previous: [21 · Object-Oriented Design & Patterns](./21-object-oriented-design-and-patterns.md) · Next: [23 · Functional Programming](./23-functional-programming.md) →
