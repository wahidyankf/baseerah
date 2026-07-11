# 17 · Programming Paradigms (By Example, Python \*\* survey)

**prd row**: Pass 2 · Solidify the Core · By Example · Python \*\* · Learn 117 / Drill 217 · Nvim-ready
Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: a **survey** of the major paradigms and how to choose among them, anchored in Python
(`**`) with other languages shown illustratively. Functional programming has its own deep topic
([`18-functional-programming`](./18-functional-programming.md)); the concurrency-oriented paradigms deepen
in Pass 4 (CSP → Go, actor → Elixir). This topic's job is fluency in _matching paradigm to problem_.

## Prerequisites

- **Prior topics**: [topic 04 Just Enough Python](./04-just-enough-python.md),
  [topic 07 Object-Oriented Programming Essentials](./07-object-oriented-programming-essentials.md) (the OO
  paradigm), and [topic 16 Object-Oriented Design & Patterns](./16-object-oriented-design-and-patterns.md);
  functional style is cross-referenced forward to [topic 18](./18-functional-programming.md).
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

---

← Previous: [16 · Object-Oriented Design & Patterns](./16-object-oriented-design-and-patterns.md) · Next: [18 · Functional Programming](./18-functional-programming.md) →
