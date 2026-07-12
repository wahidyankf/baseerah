# 84 · Type Systems (By Example, OCaml + Haskell + F# †)

**prd row**: Pass 4 · Concurrency & Systems · By Example · OCaml + Haskell + F# † · Learn 184 /
Drill 284 · Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: what a strong static type system buys you — algebraic data types, sum/product types,
exhaustive pattern matching, parametric polymorphism, Hindley-Milner inference, and typeclasses/modules —
taught in the **ML family from scratch** (OCaml as the workhorse, Haskell for purity/typeclasses), with an
**F# sidebar** connecting it back to the .NET topics. Closes with **applied category theory** (functor/
monad intuition per Bartosz Milewski, CC-licensed) — practical, not abstract.

- **License note (DD-15/DD-21)**: OCaml (LGPL-with-linking-exception), Haskell/GHC (BSD-3), F# (MIT). All
  OSS, runnable with no paid account (DD-20). Milewski's _Category Theory for Programmers_ is
  Creative-Commons.

## Why this exists · the big idea

- **The problem before the solution**: whole categories of bug — nulls, unhandled cases, wrong-shape data
  — are things a program can express and then fail on at runtime. A strong static type system exists to
  make those states unrepresentable, so the class of error is caught before the program ever runs.
- **Keep-this-if-you-forget-everything**: the compiler is a proof assistant — make illegal states
  unrepresentable and let exhaustive matching prove you've handled every case, and a large class of runtime
  bugs simply cannot occur.
- **Big ideas touched**: `correctness-vs-pragmatism` — types shift the dial toward provable-before-run,
  with the compiler proving totality and shape in exchange for some flexibility; `abstraction-and-its-cost`
  — parametric polymorphism, typeclasses/functors, and monads generalize code safely, but they leak the
  theory you must learn to wield them.

## Prerequisites

- **Prior topics**: [topic 23 Functional Programming](./23-functional-programming.md) (immutability, pure
  functions, HOFs), [topic 22 Programming Paradigms](./22-programming-paradigms.md) (typed-functional as one
  paradigm among several), and [topic 13 Just Enough TypeScript](./13-just-enough-typescript.md)
  (a first taste of static types — generics, unions — before the ML-family deep dive).
- **Tools & environment**: an **OCaml** toolchain (opam/dune), a **Haskell** toolchain (GHCup/`ghc`/`cabal`
  or `stack`), and a **.NET** SDK for the F# sidebar; Neovim/VSCode with the respective LSPs (DD-17).
- **Assumed knowledge**: FP fundamentals — recursion, HOFs, immutability (topic 23); paradigm fluency
  (topic 22); basic static-typing exposure — generics, unions (topic 13).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified (licenses, exact match): **OCaml = LGPL-with-linking-exception** (SPDX
  OCaml-LGPL-linking-exception), **GHC/Haskell = BSD-3**, **F# = MIT** (dotnet/fsharp). Milewski's
  _Category Theory for Programmers_ is **CC BY-SA 4.0** (more precise than "Creative-Commons").
- 2026-07-12 — verified: OCaml (opam/dune) and Haskell (GHCup / `ghc` + `cabal` or `stack`) toolchains are
  current; Hindley-Milner inference, typeclasses-vs-modules/functors, and functor/applicative/monad are
  evergreen PL theory, unchanged.

## Items

- Algebraic data types: sum types + product types; making illegal states unrepresentable.
- Exhaustive pattern matching; the compiler as a proof assistant for total coverage.
- Parametric polymorphism (generics) + Hindley-Milner type inference.
- Typeclasses (Haskell) vs modules/functors (OCaml) — two takes on ad-hoc polymorphism.
- **F# sidebar**: the same ADTs + pattern matching on .NET (ties to the C#/.NET thread).
- **Applied category theory**: functor/applicative/monad as practical patterns (Milewski, CC) — the "why"
  behind `map`/`bind`, grounded, not abstract.

## Worked examples

Colocated under `type-systems/learning/code/`; OCaml primary + Haskell + F# sidebar (DD-20/DD-30).

- **beginner** — model a domain with ADTs + exhaustive pattern matching in OCaml (illegal states
  unrepresentable).
- **intermediate** — parametric polymorphism + inference; the same domain in Haskell with a typeclass.
- **advanced** — a functor/monad-flavoured abstraction (e.g. an `Option`/`Result` pipeline) with the
  category-theory intuition spelled out; the F# sidebar version.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: model a small domain so that **illegal states are unrepresentable** — algebraic data types +
  exhaustive pattern matching in OCaml, a Haskell version using a typeclass for ad-hoc polymorphism, and an
  F# sidebar — then build a `map`/`bind` pipeline over an `Option`/`Result`-style type with the functor/
  monad intuition written up plainly, showing types as a correctness tool.
- **Concepts exercised**: [ ] ADTs (sum + product) making illegal states unrepresentable [ ] exhaustive
  pattern matching (compiler-checked total coverage) [ ] parametric polymorphism + HM inference [ ] a
  typeclass (Haskell) vs a module/functor (OCaml) [ ] a functor/monad `map`/`bind` pipeline with grounded
  category-theory intuition [ ] the F# sidebar.
- **Ordered steps**:
  1. `.../learning/capstone/code/domain.ml` — model the domain with ADTs + exhaustive matching in OCaml.
     Verify the compiler rejects a non-exhaustive match and accepts the total one.
  2. `Domain.hs` — the Haskell version using a typeclass; `domain.fs` — the F# sidebar. Verify both compile
     and reproduce the OCaml behaviour.
  3. Add a `map`/`bind` pipeline over an `Option`/`Result`-style type + `intuition.md` explaining the
     functor/monad pattern (Milewski, CC). Verify the pipeline runs and the write-up ties the abstraction to
     the concrete code.
- **Acceptance criteria**: illegal states are unrepresentable; pattern matching is exhaustive (compiler-
  proven); the `map`/`bind` pipeline works; the category-theory intuition is concrete, not hand-wavy; all
  three versions (OCaml, Haskell, F#) compile.
- **Done bar**: runnable end-to-end (OCaml + Haskell + F#) + web-verified.

## Read more

**Books**

- **Types and Programming Languages** — Benjamin C. Pierce (2002). The definitive graduate-level textbook on type theory; the standard reference across the field.
- **Practical Foundations for Programming Languages**, 2nd ed. — Robert Harper (2016). Rigorous, comprehensive treatment of programming language semantics and type systems.
- **Real World OCaml**, 2nd ed. — Yaron Minsky, Anil Madhavapeddy, Jason Hickey (2022). The canonical practical guide to OCaml and ML-family typed functional programming; open access. <https://realworldocaml.org/>
- **Programming in Haskell**, 2nd ed. — Graham Hutton (2016). Widely adopted, rigorous introduction to Haskell and its type system.
- **Learn You a Haskell for Great Good!** — Miran Lipovača (2011). The most widely recommended accessible introduction to Haskell's type system; free online. <https://learnyouahaskell.github.io/>
- **Software Foundations** — Benjamin C. Pierce et al. Free, machine-checked introduction to programming language theory and type systems using a proof assistant. <https://softwarefoundations.cis.upenn.edu/>

---

← Previous: [83 · Just Enough F#](./83-just-enough-fsharp.md) · Next: [85 · Compilers, Parsers & Transpilers](./85-compilers-parsers-and-transpilers.md) →
