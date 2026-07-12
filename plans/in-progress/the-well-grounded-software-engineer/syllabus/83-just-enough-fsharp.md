# 83 · Just Enough F# (Primer §, F# †)

**prd row**: Pass 4 · Concurrency & Systems · Primer § · F# † · Learn 183 / Drill 283 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: `§` primer — **just enough F#** to be productive in
[`85-compilers-parsers-and-transpilers`](./85-compilers-parsers-and-transpilers.md). The `dotnet` F#
toolchain, let-bindings and immutability by default, discriminated unions + records, exhaustive pattern
matching, the pipeline (`|>`) style, and the type system. F# is the ML-family language on .NET — its
algebraic data types and pattern matching are exactly what make an AST and an evaluator natural, which is
why it precedes the compilers topic.

## Why this exists · the big idea

- **The problem before the solution**: modelling a compiler's AST — or any domain with many shapes and
  states — in a language without sum types means classes, visitors, and runtime checks for the cases you
  forgot. F# is here to give you algebraic data types and exhaustive matching, so the shapes are exact and
  a missing case is a compile-time warning.
- **Keep-this-if-you-forget-everything**: with immutability by default and discriminated unions matched
  exhaustively, illegal states become hard to build and forgotten cases impossible to ignore — the
  compiler carries the load.
- **Big ideas touched**: `taming-state` — immutability by default and let-bindings drop mutable shared
  state as the default, so data flows through `|>` pipelines instead of being mutated in place;
  `abstraction-and-its-cost` — discriminated unions and records model a domain precisely (the AST the
  compilers topic needs), at the cost of learning to think in types and pattern matches.

## Prerequisites

- **Prior topics**: [topic 23 Functional Programming](./23-functional-programming.md) (immutability, pure
  functions, higher-order functions, recursion) and
  [topic 8 Object-Oriented Programming Essentials](./08-object-oriented-programming-essentials.md) (F# is
  functional-first but interoperates with .NET's object model).
- **Tools & environment**: a macOS/Linux/Windows machine; the **.NET SDK** (`dotnet`), pinned to a current
  LTS, which ships the F# compiler and FSI (the F# Interactive REPL); Neovim/VSCode with the F# LSP
  (Ionide/FSAutoComplete, DD-17). Keep the SDK version unpinned in prose — re-pull at authoring time.
- **Assumed knowledge**: functional fundamentals — immutability, first-class functions, recursion (topic 23);
  types and interfaces from any prior typed language (topic 08); running a CLI build/run tool (topic 05).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: keep ".NET SDK current LTS" unpinned in shipped text. The F# language surface used
  here — let-bindings, immutability by default, discriminated unions, records, active patterns, exhaustive
  match, the `|>` pipeline, and `Option`/`Result` — is stable and evergreen. `dotnet` subcommands
  (`new`/`run`/`build`/`test`/`fsi`) are current/unchanged. (learn.microsoft.com/dotnet/fsharp)
- 2026-07-12 — verified: the F# toolchain (dotnet SDK, and FParsec for the compilers topic that follows) is
  correctly left version-unpinned here; note the exact SDK and FParsec versions as "to verify" at authoring
  time.

## Items

- The `dotnet` F# toolchain: `dotnet new`, `run`, `build`, `test`, and `dotnet fsi` for the REPL.
- Let-bindings, immutability by default, functions, and the significant-whitespace syntax.
- Discriminated unions + records — algebraic data types for modelling; the shape an AST is built from.
- Exhaustive pattern matching and active patterns; the compiler warns on missing cases.
- The pipeline (`|>`) and function composition; idiomatic data-transformation style.
- The type system: type inference, `Option`/`Result` for total functions, and light OO interop with .NET.

## Worked examples

Colocated under `just-enough-fsharp/learning/code/`; each runnable via `dotnet` (DD-20/DD-30).

- **beginner** — a `dotnet run` console program using let-bindings and a pipeline.
- **intermediate** — a discriminated union + a record, consumed by an exhaustive `match`.
- **advanced** — a small recursive DU (e.g. an expression tree) folded/evaluated with pattern matching — a
  direct rehearsal for the compilers topic.

## Capstone spec — intra-topic (primer → light consolidation)

- **Goal**: build a small F# console app that exercises the primer's surface — immutable let-bindings, a
  discriminated union + a record, exhaustive pattern matching, the `|>` pipeline, and an `Option`/`Result`
  return — runnable via `dotnet run` with a `dotnet test`, proving readiness for the compilers topic.
- **Concepts exercised**: [ ] immutable let-bindings [ ] a discriminated union [ ] a record [ ] exhaustive
  pattern matching [ ] the `|>` pipeline [ ] `Option`/`Result` [ ] a `dotnet test`.
- **Ordered steps**:
  1. `.../learning/capstone/code/` — a console app defining a discriminated union + record and transforming
     data through a `|>` pipeline. Verify `dotnet run` produces the expected output.
  2. Add an exhaustive `match` over the union and a function returning `Result`. Verify the compiler reports
     no missing-case warning and the error path returns `Error`, not an exception.
  3. Add a small recursive DU (an expression tree) with a recursive evaluator + a `dotnet test`. Verify the
     evaluator returns correct results and the test passes.
- **Acceptance criteria**: immutability and the pipeline style work; the union/record and exhaustive match
  compile warning-free; `Option`/`Result` handles the error path; the recursive evaluator is correct;
  `dotnet test` passes.
- **Done bar**: runnable end-to-end + web-verified.

## Read more

**Books**

- **Domain Modeling Made Functional** — Scott Wlaschin (2018). Canonical guide to applying F#'s type system
  and functional idioms to domain-driven design.
- **Get Programming with F#** — Isaac Abraham (2018). Widely recommended step-by-step primer for .NET
  developers learning F#.
- **Stylish F#** — Kit Eason (2018). Focused guide to idiomatic, elegant F# style for working engineers.

**Papers & articles**

- **F# for Fun and Profit** — Scott Wlaschin. The most widely cited free resource for practical,
  ML-family functional-first F# idioms. <https://fsharpforfunandprofit.com/>

---

← Previous: [82 · Lisp](./82-lisp.md) · Next: [84 · Type Systems](./84-type-systems.md) →
