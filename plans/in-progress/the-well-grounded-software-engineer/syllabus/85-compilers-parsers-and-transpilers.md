# 85 · Compilers, Parsers & Transpilers (By Example, F# †)

**prd row**: Pass 4 · Concurrency & Systems · By Example · F# † · Learn 185 / Drill 285 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: how a language processor works front-to-back — lexing, parsing (to an AST), semantic
analysis, and either interpreting/evaluating or emitting code — by building a small language end-to-end in
**F#**. The ML family is the natural home for this work: discriminated unions model an AST directly,
exhaustive pattern matching walks it safely, and parser combinators (FParsec) express a grammar as
composable functions. **Motivation (DD-16)**: in the AI-assisted era, compilers/type-checkers/linters are
your **guardrails** — understanding how they parse and reason about code makes you a sharper reader and
reviewer of both hand-written and AI-generated code. As the **last subject topic of Pass 4**, this file also
anchors the two Pass-4 concurrency capstones (`capstone-concurrency-and-systems` and
`capstone-concurrency-showdown`), which integrate the pass's concurrency + systems-depth threads (the
whole-journey `capstone-lead-at-altitude` now anchors at the journey's true close,
[`90-site-reliability-engineering`](./90-site-reliability-engineering.md)).

## Why this exists · the big idea

- **The problem before the solution**: every language, type-checker, linter, and transpiler you use is a
  black box until you've built one — and in the AI-assisted era those tools are your guardrails, so not
  understanding how they parse and reason about code makes you a weaker reviewer of both human and machine
  output. This topic opens the box.
- **Keep-this-if-you-forget-everything**: a language processor is a pipeline — source → tokens → AST →
  analysis → interpret or emit — and once you've built the pipeline once, every compiler, linter, and
  transpiler stops being magic.
- **Big ideas touched**: `layering-and-leaks` — a compiler is layering made literal, each stage
  transforming one representation into the next, with errors leaking upward from the layer that first
  noticed them; `abstraction-and-its-cost` — the AST is the abstraction the whole back-end is built on, and
  modelling it as a discriminated union walked by exhaustive matching is where the language's shape is
  captured (and where a missing case would leak).

## Prerequisites

- **Prior topics**: [topic 83 Just Enough F#](./83-just-enough-fsharp.md) (the implementation language —
  discriminated unions, records, pattern matching, pipelines),
  [topic 19 Computer Science Foundations](./19-computer-science-foundations.md) (trees, recursion, grammars),
  and [topic 84 Type Systems](./84-type-systems.md) (ADTs + pattern matching make an AST + evaluator natural
  — the immediately-prior topic).
- **Tools & environment**: the **.NET SDK** (`dotnet`) on a current LTS, which ships the F# compiler and FSI;
  **FParsec** (the F# parser-combinator library) to contrast with a hand-written recursive-descent parser; a
  test project via `dotnet test` (xUnit/Expecto); Neovim/VSCode with the F# LSP (Ionide, DD-17).
- **Assumed knowledge**: F# discriminated unions + pattern matching and recursion (topic 83); trees + grammar
  intuition (topic 19); sum types / pattern matching as a way to shape an AST (topic 84).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: the pipeline (lexer → parser/AST → semantic analysis → interpret/emit),
  recursive-descent + Pratt parsing (precedence-climbing operator parsing), tree-walking interpreter +
  environments/scopes, and transpilation are evergreen/unchanged. Modelling the AST as a discriminated union
  and walking it with exhaustive pattern matching is idiomatic ML-family compiler practice.
- 2026-07-12 — verified (to verify at authoring time): the F# toolchain is correctly left version-unpinned —
  pin the exact **.NET SDK** and **FParsec** versions at authoring time (both were stable and current in the
  sweep, but treat the specific numbers as "to verify"). FParsec's combinator API and F#'s DU/active-pattern
  surface are stable across recent releases. (fsprojects.github.io/FParsec)

## Items

- The pipeline: source → lexer/tokens → parser/AST → semantic analysis → interpret or emit.
- Lexing: tokenizing input; handling whitespace/comments/errors — often folded into FParsec's token parsers.
- Parsing: grammars, recursive descent, precedence (Pratt parsing), and the parser-combinator alternative
  (FParsec) building an AST as a discriminated union.
- Evaluation: a tree-walking interpreter over the AST, with pattern matching and environments/scopes.
- Transpilation: emitting target code (e.g. to Python or JS) instead of interpreting.
- **The guardrail lens (DD-16)**: how type-checkers/linters use the same front-end to catch errors — why
  understanding this makes you a better reviewer of AI-generated code.

## Worked examples

Colocated under `compilers-parsers-and-transpilers/learning/code/`; F# + `dotnet test` (DD-20/DD-30). The
AST is a discriminated union; walks are exhaustive `match` expressions.

- **beginner** — a lexer that tokenizes a small expression language into a token DU (+ tests).
- **intermediate** — a parser building an AST discriminated union with correct precedence, done two ways: a
  hand-written recursive-descent/Pratt parser and an FParsec combinator version (+ tests).
- **advanced** — a tree-walking interpreter that evaluates the AST via pattern matching; a transpiler variant
  that emits target code instead.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: build a small but complete language processor in F# — lexer → parser (a hand-written
  recursive-descent/Pratt parser, with an FParsec combinator variant for contrast) → an AST discriminated
  union → **both** a tree-walking interpreter that evaluates programs **and** a transpiler that emits
  equivalent target code — fully covered by `dotnet test`, demonstrating the front-end that every
  compiler/type-checker/linter shares.
- **Concepts exercised**: [ ] a lexer/tokenizer with error handling [ ] a recursive-descent/Pratt parser with
  correct precedence [ ] an FParsec combinator parser for contrast [ ] an AST as a discriminated union [ ] a
  tree-walking interpreter with scopes/environments via pattern matching [ ] a transpiler emitting target
  code [ ] `dotnet test` coverage of each stage.
- **Ordered steps**:
  1. `.../learning/capstone/code/Lexer.fs` — tokenize the source language into a token DU. Verify tests cover
     tokens, whitespace/comments, and a lexer error.
  2. `Parser.fs` — a recursive-descent/Pratt parser → AST discriminated union with correct operator
     precedence, plus an FParsec variant. Verify precedence-sensitive expressions parse to the right tree, and
     that both parsers agree (tests).
  3. `Interpreter.fs` — evaluate the AST with scopes via exhaustive pattern matching; `Transpiler.fs` — emit
     equivalent target code. Verify the interpreter produces correct results and the transpiled output, when
     run, matches the interpreter.
- **Acceptance criteria**: the full pipeline works; precedence is correct; the recursive-descent and FParsec
  parsers agree; interpreter results and transpiler output agree; `dotnet test` covers each stage; the
  guardrail framing is stated.
- **Done bar**: runnable end-to-end + tests green + web-verified.

<!-- Inter-topic capstone spec block: this file (last subject topic of Pass 4) anchors the Pass-4 boundary capstones -->

## Capstone spec — inter-topic: capstone-concurrency-and-systems (Pass-4 boundary)

> **Weight**: `capstone-concurrency-and-systems/_index.md` = **955** (section root, after Pass 4 / topic 85).
> Kind: **subject → full runnable**. Integrates the pass's concurrency + systems-depth topics.

- **Goal**: build a **concurrent, systems-aware, observable service** that ties Pass 4 together — a
  work-processing service using a real concurrency model (CSP-Go **or** actor-Elixir), backed by a
  systems-level component, containerized, and instrumented with SRE golden signals + an SLO — demonstrating
  that concurrency, systems depth, and reliability compose into one operable system.
- **Concepts integrated**: [ ] a concurrency model in anger (Go CSP: goroutines/channels/`context`
  [topic 61] **or** Elixir actors: GenServer/supervision [topic 63]) [ ] a systems-level component (a C
  primitive / memory-aware data path [topics 74/77/79] **or** a justified equivalent) [ ] containerized +
  orchestrated deployment [topic 50, Pass 3] [ ] SRE instrumentation: four golden signals + an SLI/SLO +
  error budget [topic 90] [ ] a symptom-based alert + dashboard.
- **Ordered steps**:
  1. `capstone-concurrency-and-systems/code/` — a concurrent work-processing service in Go (CSP) **or**
     Elixir (actors), with a bounded worker pool / supervised workers and graceful shutdown. Verify it
     processes a concurrent workload with no race (Go `-race`) / clean supervision (Elixir) and shuts down
     gracefully.
  2. Add a systems-level component (or a justified equivalent) and containerize the service. Verify the
     container builds and runs the full workload.
  3. Instrument the four golden signals + an SLI/SLO + error budget; add a symptom-based alert + dashboard.
     Verify the signals expose under load, the SLO alert fires on violation, and the dashboard reflects it.
- **Acceptance criteria**: the concurrency model is used correctly (race-free / properly supervised); the
  service is containerized; golden signals + SLO + alert + dashboard all work; graceful shutdown holds.
- **Done bar**: runnable end-to-end + observable + web-verified.

## Capstone spec — inter-topic: capstone-concurrency-showdown (cross-cutting)

> **Weight**: `capstone-concurrency-showdown/_index.md` = **956** (section root, after Pass 4 / topic 85).
> Kind: **subject → full runnable + comparison artifact**. A deliberate CSP-vs-actor head-to-head.

- **Goal**: solve the **same** concurrent problem twice — once with **CSP-style Go**
  (goroutines/channels/`select`/`context`) and once with the **actor-model Elixir/OTP**
  (GenServer/supervision/"let it crash") — then write a grounded comparison of the two paradigms on the same
  workload: how each handles coordination, backpressure, failure/supervision, and observability.
- **Concepts integrated**: [ ] the same problem in Go CSP [topic 61] and Elixir actors [topic 63] [ ] channel
  coordination + `select` + `context` cancellation (Go) [ ] GenServer + supervision trees + "let it crash"
  (Elixir) [ ] backpressure + failure handling contrasted [ ] a decision write-up: when each model fits.
- **Ordered steps**:
  1. `capstone-concurrency-showdown/go/` — solve the chosen concurrent problem (e.g. a fan-out/fan-in
     pipeline with cancellation + backpressure) in Go. Verify it runs `-race`-clean and handles cancellation
     - a failing worker.
  2. `.../elixir/` — solve the identical problem with GenServer + a supervision tree. Verify it runs and a
     crashing worker is supervised/restarted without taking down the system.
  3. `comparison.md` — contrast the two on coordination, backpressure, failure/supervision, testability, and
     observability, with a concrete "when to reach for which" recommendation grounded in the two
     implementations. Verify each claim points at real behaviour in the two codebases.
- **Acceptance criteria**: both implementations solve the same problem correctly (Go race-free; Elixir
  supervised); the comparison is concrete and evidence-backed, not generic; the recommendation is justified.
- **Done bar**: both runnable end-to-end + comparison artifact + web-verified.

## Read more

**Books**

- **Compilers: Principles, Techniques, and Tools** ("The Dragon Book") — Alfred V. Aho, Monica S. Lam, Ravi
  Sethi, Jeffrey D. Ullman (2nd ed., 2006). The most iconic, field-defining compiler-construction textbook.
- **Engineering a Compiler** — Keith D. Cooper, Linda Torczon (3rd ed., 2022). Rigorous, modern treatment of
  compiler construction, optimization, and code generation.
- **Modern Compiler Implementation in ML** — Andrew W. Appel (1998). Influential, implementation-focused
  compilers text using ML — the closest classic to this topic's F# approach.
- **Crafting Interpreters** — Robert Nystrom (2021). Widely adopted, hands-on guide building a tree-walking
  interpreter and a bytecode VM from scratch; free online. <https://craftinginterpreters.com/>

---

← Previous: [84 · Type Systems](./84-type-systems.md) · Next: [86 · Build Your Own Git](./86-build-your-own-git.md) →
