# 49 · Just Enough C# (Primer §, C# †)

**prd row**: Pass 4 · Concurrency & Systems · Primer § · C# † · Learn 149 / Drill 249 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: `§` primer — **just enough C#** to be productive in
[`50-windows-app-development`](./50-windows-app-development.md). The `dotnet` CLI, syntax/types, nullable
reference types, properties, records, a LINQ intro, classes/interfaces, and an `async`/`await` _preview_.

## Prerequisites

- **Prior topics**: [topic 07 Object-Oriented Programming Essentials](./07-object-oriented-programming-essentials.md)
  (classes/interfaces) and general typed-language fluency (any of Kotlin/Swift/TypeScript from earlier
  primers transfers).
- **Tools & environment**: a macOS/Linux/Windows machine; the **.NET SDK** (`dotnet`), pinned to a current
  LTS; Neovim/VSCode with the C# LSP (DD-17).
- **Assumed knowledge**: classes/interfaces (topic 07); nullable-vs-non-null thinking (topics 11/45);
  running a CLI build tool (topic 05).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: keep "current LTS" unpinned in shipped text. Current .NET LTS is **.NET 10**
  (Nov 2025, supported to Nov 2028); next is **.NET 11** (STS, ~Nov 2026) — after that "current LTS" is
  still .NET 10 but a newer STS coexists, so re-pull at authoring time. `dotnet` CLI (`new`/`run`/`build`/
  `test`), nullable reference types, properties, records, LINQ, `async`/`await` are current/unchanged.
  (dotnet.microsoft.com/platform/support/policy/dotnet-core)

## Items

- The `dotnet` CLI from the terminal: `new` / `run` / `build` / `test`.
- Syntax & types; nullable reference types; properties; records; a LINQ intro.
- Classes / interfaces; the `async` / `await` **preview** (depth in `windows-app-development`).

## Worked examples

Colocated under `just-enough-csharp/learning/code/`; each runnable via `dotnet` (DD-20/DD-30).

- **beginner** — a `dotnet run` console program.
- **intermediate** — records + a LINQ query.
- **advanced** — an `async`/`await` preview.

## Capstone spec — intra-topic (primer → light consolidation)

- **Goal**: build a small C# console app that exercises the primer's surface — nullable reference types,
  records, a LINQ query over a collection, an interface, and a single `async`/`await` call — runnable via
  `dotnet run` + a `dotnet test`, proving readiness for Windows app development.
- **Concepts exercised**: [ ] nullable reference types [ ] records [ ] a LINQ query [ ] an interface
  [ ] an `async`/`await` call [ ] a `dotnet test`.
- **Ordered steps**:
  1. `.../learning/capstone/code/` — a console app using records + a LINQ query with nullable-aware code.
     Verify `dotnet run` produces the expected output.
  2. Add an interface + an implementation. Verify dispatch works.
  3. Add an `async` method + an `await` call + a `dotnet test`. Verify the async path completes and the test
     passes.
- **Acceptance criteria**: records, LINQ, and nullable handling work; the interface dispatches; the
  `async`/`await` call completes; `dotnet test` passes.
- **Done bar**: runnable end-to-end + web-verified.

---

← Previous: [48 · iOS App Development](./48-ios-app-development.md) · Next: [50 · Windows App Development](./50-windows-app-development.md) →
