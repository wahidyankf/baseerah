# 68 · Just Enough Dart (Primer §, Dart †)

**prd row**: Pass 4 · Concurrency & Systems · Primer § · Dart † · Learn 168 / Drill 268 · Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: `§` primer — **just enough Dart** to be productive in
[`69-hybrid-app-development`](./69-hybrid-app-development.md). The `dart` / `pub` CLI, sound null safety,
syntax and types, `async`/`await` with `Future`s and `Stream`s, and classes/mixins. `†`: Dart, run and
built with the `dart` toolchain.

## Why this exists · the big idea

- **The problem before the solution**: you cannot learn Flutter and Dart at the same time without drowning
  — the widget model deserves full attention, so the language it is written in has to already be muscle
  memory.
- **Keep-this-if-you-forget-everything**: Dart is a familiar C-family, null-safe, statically typed language
  with first-class async — if you know a typed OO language, most of it transfers; the parts worth deliberate
  practice are sound null safety and `Future`/`Stream` async.
- **Big ideas touched**: `taming-state` (async/await, `Future`s, and `Stream`s are Dart's structured way to
  handle state that arrives over time without callback tangles), `abstraction-and-its-cost` (sound null
  safety is a compile-time abstraction that eliminates a whole class of null errors — at the cost of forcing
  you to be explicit about what can be absent).

## Prerequisites

- **Prior topics**: [topic 8 Object-Oriented Programming Essentials](./08-object-oriented-programming-essentials.md)
  (classes, interfaces, inheritance) and [topic 13 Just Enough TypeScript](./13-just-enough-typescript.md)
  (static types and null-vs-non-null thinking transfer directly).
- **Tools & environment**: a macOS/Linux/Windows machine; the **Dart SDK** (`dart`, `pub`) pinned to a
  current stable (it ships with Flutter, so a Flutter install also provides it); Neovim/VSCode with the Dart
  LSP (DD-17).
- **Assumed knowledge**: classes/interfaces/inheritance (topic 08); static types and nullability (topic
  13); running a CLI build/run tool (topic 05).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: keep the Dart SDK at "a current stable" in shipped text — sound null safety, the
  `dart` CLI (`create`/`run`/`test`), `pub` package management, `async`/`await` with `Future`/`Stream`, and
  classes/mixins are stable, settled language surface. Dart releases on a moving cadence, so a pinned number
  would go stale fast.
- 2026-07-12 — verified: no third-party package version is claimed in the body — the primer stays on the
  standard library and language core, so there is no version to re-pull beyond the SDK itself.

## Items

- The `dart` / `pub` CLI from the terminal: `create` / `run` / `test`, and adding a package.
- Syntax & types; sound null safety (`?`, `!`, `late`, and null-aware operators).
- Classes, interfaces, and mixins; named/optional/required parameters.
- Collections and generics; the core-library types you will meet in Flutter.
- `async` / `await` with `Future`s, and `Stream`s for values over time.

## Worked examples

Colocated under `just-enough-dart/learning/code/`; each runnable via `dart` (DD-20/DD-30).

- **beginner** — a `dart run` console program with null-safe types.
- **intermediate** — a class with a mixin plus a small generic collection.
- **advanced** — an `async`/`await` call returning a `Future`, and consuming a `Stream`.

## Capstone spec — intra-topic (primer → light consolidation)

- **Goal**: build a small Dart console program that exercises the primer's surface — null-safe types, a
  class with a mixin, a generic collection, and an `async`/`await` call over a `Future` (plus a `Stream`),
  runnable via `dart run` and a `dart test`, proving readiness for Flutter.
- **Concepts exercised**: [ ] sound null safety [ ] a class + a mixin [ ] a generic collection [ ] an
  `async`/`await` + `Future` [ ] consuming a `Stream` [ ] a `dart test`.
- **Ordered steps**:
  1. `.../learning/capstone/code/` — a console program using null-safe types, a class with a mixin, and a
     generic collection. Verify `dart run` produces the expected output.
  2. Add an `async` function returning a `Future` and `await` it. Verify the async path completes with the
     awaited value.
  3. Add a `Stream` consumer and a `dart test`. Verify the stream is consumed in order and the test passes.
- **Acceptance criteria**: null-safe code compiles and runs; the mixin and generic collection work; the
  `Future` completes and the `Stream` is consumed in order; `dart test` passes.
- **Done bar**: runnable end-to-end + web-verified.

## Read more

**Papers & articles**

- **A tour of the Dart language** — official (dart.dev). The canonical language primer maintained by the
  Dart team. <https://dart.dev/language>
- **Dart documentation** — official (dart.dev). The authoritative documentation hub, including core
  libraries. <https://dart.dev/docs>
- **Dart language specification** — official (dart.dev). The formal specification of the language.
  <https://dart.dev/resources/language/spec>

---

← Previous: [67 · iOS App Development](./67-ios-app-development.md) · Next: [69 · Hybrid App Development](./69-hybrid-app-development.md) →
