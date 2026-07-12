# 64 · Just Enough Kotlin (Primer §, Kotlin †)

**prd row**: Pass 4 · Concurrency & Systems · Primer § · Kotlin † · Learn 164 / Drill 264 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: `§` primer — **just enough Kotlin** to be productive in
[`65-android-app-development`](./65-android-app-development.md). The toolchain, syntax, null-safety,
`val`/`var`, data classes, functions/lambdas, collections, classes/interfaces, and a coroutine _preview_.

## Why this exists · the big idea

- **The problem before the solution**: Android in topic 65 leans on null-safety and coroutines from the
  first line — this primer makes Kotlin's type system and concurrency preview familiar before the
  platform's complexity lands on top of them.
- **Keep-this-if-you-forget-everything**: Kotlin makes null a compile-time decision, not a runtime
  surprise — the type `T?` forces you to handle absence exactly where it can occur.
- **Big ideas touched**: `taming-state` — nullability is a state hazard the type system contains before it
  becomes an NPE; `abstraction-and-its-cost` — data classes and coroutines buy concise expression over
  machinery you stop seeing (and occasionally must see through).

## Prerequisites

- **Prior topics**: [topic 8 Object-Oriented Programming Essentials](./08-object-oriented-programming-essentials.md)
  (classes/interfaces) and general typed-language fluency —
  [topic 13 Just Enough TypeScript](./13-just-enough-typescript.md) helps for null-safety intuition.
- **Tools & environment**: a macOS/Linux terminal; **Kotlin** (`kotlinc`) + Gradle (`./gradlew`), pinned to
  a current stable release; a JDK; Neovim/VSCode (DD-17).
- **Assumed knowledge**: classes + interfaces (topic 08); nullable-vs-non-null thinking (topic 13);
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

## Read more

**Books**

- **Kotlin in Action** — Dmitry Jemerov & Svetlana Isakova (2017, Manning). Written by JetBrains engineers; the classic Kotlin primer.

**Papers & articles**

- **Kotlin documentation** — JetBrains, official (kotlinlang.org). The authoritative, continuously updated language reference. <https://kotlinlang.org/docs/home.html>
- **Kotlin tour** — official (kotlinlang.org). The official guided primer to Kotlin fundamentals. <https://kotlinlang.org/docs/kotlin-tour-welcome.html>
- **Kotlin language specification** — official (kotlinlang.org). The formal specification of the language. <https://kotlinlang.org/spec/>

---

← Previous: [63 · Actor-Model Concurrency](./63-actor-model-concurrency.md) · Next: [65 · Android App Development](./65-android-app-development.md) →
