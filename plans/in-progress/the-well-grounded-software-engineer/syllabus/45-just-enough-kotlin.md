# 45 · Just Enough Kotlin (Primer §, Kotlin †)

**prd row**: Pass 4 · Concurrency & Systems · Primer § · Kotlin † · Learn 145 / Drill 245 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: `§` primer — **just enough Kotlin** to be productive in
[`46-android-app-development`](./46-android-app-development.md). The toolchain, syntax, null-safety,
`val`/`var`, data classes, functions/lambdas, collections, classes/interfaces, and a coroutine _preview_.

## Prerequisites

- **Prior topics**: [topic 07 Object-Oriented Programming Essentials](./07-object-oriented-programming-essentials.md)
  (classes/interfaces) and general typed-language fluency —
  [topic 11 Just Enough TypeScript](./11-just-enough-typescript.md) helps for null-safety intuition.
- **Tools & environment**: a macOS/Linux terminal; **Kotlin** (`kotlinc`) + Gradle (`./gradlew`), pinned to
  a current stable release; a JDK; Neovim/VSCode (DD-17).
- **Assumed knowledge**: classes + interfaces (topic 07); nullable-vs-non-null thinking (topic 11);
  running a build tool (topic 05).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: keep the version unpinned in shipped text. Current stable is **Kotlin 2.4.0**
  (~June 2026; 2.4.20 planned Sept 2026, 2.5.0 Dec 2026). Null-safety (`?`, `?:`, `!!`), data classes,
  `val`/`var`, and coroutine-preview syntax are unchanged. Re-pull the exact version at authoring time.
  (kotlinlang.org/docs/releases.html)

## Items

- `kotlinc` / Gradle (`./gradlew`) from the CLI.
- Syntax; null-safety (`?`, `?:`, `!!`); `val` / `var`; data classes; functions & lambdas; collections.
- Classes / interfaces; the coroutine **preview** (depth in `android-app-development`).

## Worked examples

Colocated under `just-enough-kotlin/learning/code/`; each runnable via the Kotlin CLI/Gradle (DD-20/DD-30).

- **beginner** — a runnable Kotlin program via the CLI.
- **intermediate** — data classes + null-safety.
- **advanced** — a coroutine preview.

## Capstone spec — intra-topic (primer → light consolidation)

- **Goal**: build a small Kotlin CLI that exercises the primer's surface — null-safety, data classes,
  lambdas over collections, an interface, and a single coroutine — with a Gradle/`kotlinc` build, proving
  readiness for Android development.
- **Concepts exercised**: [ ] null-safety (`?`/`?:`) [ ] data classes [ ] lambdas over collections
  [ ] an interface [ ] a coroutine.
- **Ordered steps**:
  1. `.../learning/capstone/code/` — a program using data classes + collection lambdas + null-safe access.
     Verify it builds and produces the expected output.
  2. Add an interface + an implementation. Verify polymorphic dispatch works.
  3. Add a single coroutine (e.g. a suspended computation). Verify it runs to completion and returns its
     value.
- **Acceptance criteria**: null-safety, data classes, and collection lambdas work; the interface dispatches;
  the coroutine completes.
- **Done bar**: runnable end-to-end + web-verified.

---

← Previous: [44 · Actor-Model Concurrency](./44-actor-model-concurrency.md) · Next: [46 · Android App Development](./46-android-app-development.md) →
