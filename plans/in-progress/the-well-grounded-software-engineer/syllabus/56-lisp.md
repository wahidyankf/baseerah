# 56 · Lisp (By Example, Scheme + Clojure †)

**prd row**: Pass 4 · Concurrency & Systems · By Example · Scheme + Clojure † · Learn 156 /
Drill 256 · Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the Lisp idea — code-as-data (homoiconicity), s-expressions, the read-eval loop, and macros
that let the language extend itself. Taught in **Scheme from scratch** (minimal, pedagogical), with a
**Clojure sidebar** showing the same ideas on a modern hosted JVM Lisp. Deliberately stretches the mental
model established by the FP thread ([`18-functional-programming`](./18-functional-programming.md)).

- **License note (DD-15/DD-21)**: **Racket** (a batteries-included Scheme) is Apache-2.0/MIT; **Clojure** is
  EPL-1.0. Both OSS and runnable with no paid account (DD-20).

## Prerequisites

- **Prior topics**: [topic 18 Functional Programming](./18-functional-programming.md) (recursion, higher-
  order functions, immutability) and [topic 17 Programming Paradigms](./17-programming-paradigms.md)
  (the paradigm-survey context — Lisp as its own family blending functional style with metaprogramming).
- **Tools & environment**: **Racket** (or another Scheme, e.g. Guile) for the from-scratch track; a
  **Clojure** toolchain (`clojure`/`deps.edn`) + JDK for the sidebar; Neovim/VSCode with a Lisp/REPL
  integration (DD-17).
- **Assumed knowledge**: recursion + higher-order functions + immutability as a habit (topic 18); comfort
  moving between programming paradigms (topic 17).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified (licenses, exact match): **Racket = Apache-2.0/MIT at your option**
  (download.racket-lang.org/license.html); **Clojure = EPL-1.0** (clojure.org/community/license). Both are
  fully free/open, runnable with no paid account. `syntax-rules` (Scheme, hygienic) vs `defmacro` (Clojure,
  unhygienic) contrast is correct standard terminology; GNU Guile remains an actively-maintained Scheme.
- 2026-07-12 — verified (minor copyedit for the content maker): "Racket (a batteries-included Scheme)" is
  fine but Racket now positions itself as a Lisp-family language _descended from_ Scheme (R7RS via
  `#lang r7rs`, not the default) — consider "a batteries-included Scheme descendant" for precision.

## Items

- S-expressions & homoiconicity: code is data; the reader; `quote`/`eval`.
- The core: `define`, `lambda`, recursion, lists, `cons`/`car`/`cdr`, higher-order functions.
- The read-eval loop as the primary workflow.
- Macros: extending the language; `syntax-rules` (Scheme, hygienic) — the payoff feature.
- **Clojure sidebar**: the same ideas on the JVM — persistent data structures, `defmacro`, hosted interop.

## Worked examples

Colocated under `lisp/learning/code/`; Scheme (Racket) primary + Clojure sidebar (DD-20/DD-30).

- **beginner** — recursion + list processing in Scheme; the same in Clojure (sidebar).
- **intermediate** — higher-order functions + a small interpreter-flavoured example (code-as-data).
- **advanced** — a `syntax-rules` macro that adds a new control form; the Clojure `defmacro` equivalent.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: use Lisp's code-as-data nature to build something the language couldn't do out of the box — a
  `syntax-rules` macro (Scheme) that introduces a new control/binding form, exercised by a small
  recursive, list-processing program — then reproduce the core idea in the **Clojure sidebar** with
  `defmacro`, showing homoiconicity across two Lisps.
- **Concepts exercised**: [ ] s-expressions + `quote`/`eval` (code-as-data) [ ] recursion + `cons`/`car`/
  `cdr` list processing [ ] higher-order functions [ ] a hygienic `syntax-rules` macro adding a new form
  [ ] the Clojure `defmacro` equivalent (sidebar).
- **Ordered steps**:
  1. `.../learning/capstone/code/main.rkt` — a recursive list-processing program using higher-order
     functions. Verify it runs in Racket and produces the expected output.
  2. Add a `syntax-rules` macro introducing a new control/binding form, used by the program. Verify the
     macro expands correctly (check with the macro stepper) and the program still runs.
  3. `sidebar.clj` — reproduce the core idea in Clojure with `defmacro`. Verify it runs on the Clojure
     toolchain and mirrors the Scheme behaviour.
- **Acceptance criteria**: recursion + list processing + higher-order functions work; the macro adds a
  genuinely new form and expands hygienically; the Clojure sidebar reproduces the idea; both run.
- **Done bar**: runnable end-to-end (Racket + Clojure) + web-verified.

---

← Previous: [55 · System Programming](./55-system-programming.md) · Next: [57 · Type Systems](./57-type-systems.md) →
