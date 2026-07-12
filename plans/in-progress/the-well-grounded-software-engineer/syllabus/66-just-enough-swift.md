# 66 · Just Enough Swift (Primer §, Swift †)

**prd row**: Pass 4 · Concurrency & Systems · Primer § · Swift † · Learn 166 / Drill 266 ·
Nvim-ready Partial · VSCode-ready Partial. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: `§` primer — **just enough Swift** to be productive in
[`67-ios-app-development`](./67-ios-app-development.md), taught from the `swift` REPL / `swiftc` CLI
**before** the Xcode-bound topic. Syntax, optionals, structs vs classes, enums with associated values,
protocols, closures, and an `async`/`await` _preview_.

## Why this exists · the big idea

- **The problem before the solution**: iOS in topic 67 is Xcode-bound and dense — learning Swift's value
  semantics, optionals, and concurrency from the plain `swiftc` CLI first strips away the IDE so the
  language itself is what you actually learn.
- **Keep-this-if-you-forget-everything**: Swift defaults to value types (structs) — copies don't alias, so
  shared mutable state is opt-in (a class) rather than the default you fight.
- **Big ideas touched**: `taming-state` — value semantics and optionals make mutation and absence explicit
  instead of ambient; `abstraction-and-its-cost` — protocols and enums-with-associated-values buy
  expressive modeling you pay for in language surface.

## Prerequisites

- **Prior topics**: [topic 8 Object-Oriented Programming Essentials](./08-object-oriented-programming-essentials.md) (types,
  classes) and [topic 64 Just Enough Kotlin](./64-just-enough-kotlin.md) (null-safety/optionals intuition
  transfers).
- **Tools & environment**: a **macOS** machine (Swift toolchain; Xcode not yet required); the `swift` REPL
  - `swiftc` from the CLI; Neovim/VSCode (DD-17). (Linux Swift works for the CLI examples.)
- **Assumed knowledge**: classes/structs + types (topic 08); optional/nullable thinking (topic 64).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: keep the version unpinned in shipped text. Current stable is **Swift 6.3**
  (2026-03-24; 6.4 in beta — re-pull at authoring time). `swift` REPL / `swiftc`, optionals, structs-vs-
  classes, enums-with-associated-values, protocols, closures, and `async`/`await` are unchanged. Swift is
  open source and cross-platform (Swift 6.3 even shipped an official Android SDK), so the Linux-CLI framing
  holds. (swift.org/blog)

## Items

- The `swift` REPL and `swiftc` from the CLI (before the Xcode-bound topic).
- Syntax; optionals; structs vs classes; enums with associated values; protocols; closures.
- The `async` / `await` **preview** (depth in `ios-app-development`).

## Worked examples

Colocated under `just-enough-swift/learning/code/`; each runnable via `swift`/`swiftc` (DD-20/DD-30).

- **beginner** — a runnable Swift script via the CLI.
- **intermediate** — optionals + enums with associated values.
- **advanced** — a protocol + an `async`/`await` preview.

## Capstone spec — intra-topic (primer → light consolidation)

- **Goal**: build a small Swift CLI program that exercises the primer's surface — optionals, an enum with
  associated values, a protocol + conformance, closures, and a single `async`/`await` call — runnable via
  `swiftc`, proving readiness for iOS development.
- **Concepts exercised**: [ ] optionals (safe unwrapping) [ ] an enum with associated values [ ] a protocol
  - conformance [ ] a closure [ ] an `async`/`await` call.
- **Ordered steps**:
  1. `.../learning/capstone/code/` — a program modeling a small domain with an enum (associated values) +
     optionals. Verify it compiles with `swiftc` and handles the nil case safely.
  2. Add a protocol + a conforming type + a closure-based transform. Verify polymorphic dispatch + the
     closure work.
  3. Add an `async` function + an `await` call. Verify it runs to completion and returns its value.
- **Acceptance criteria**: optionals, the enum, the protocol, and the closure work; the `async`/`await`
  call completes.
- **Done bar**: runnable end-to-end + web-verified.

## Read more

**Books**

- **The Swift Programming Language** — Apple Inc. / Swift core team, official book (CC BY 4.0). The canonical, continuously maintained, free-and-open Swift primer published alongside the language itself. <https://docs.swift.org/swift-book/documentation/the-swift-programming-language/>

**Papers & articles**

- **Swift.org Documentation** — Swift core team, official. The authoritative hub for language, toolchain, and evolution documentation. <https://www.swift.org/documentation/>

---

← Previous: [65 · Android App Development](./65-android-app-development.md) · Next: [67 · iOS App Development](./67-ios-app-development.md) →
