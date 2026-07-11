# 47 · Just Enough Swift (Primer §, Swift †)

**prd row**: Pass 4 · Concurrency & Systems · Primer § · Swift † · Learn 147 / Drill 247 ·
Nvim-ready Partial · VSCode-ready Partial. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: `§` primer — **just enough Swift** to be productive in
[`48-ios-app-development`](./48-ios-app-development.md), taught from the `swift` REPL / `swiftc` CLI
**before** the Xcode-bound topic. Syntax, optionals, structs vs classes, enums with associated values,
protocols, closures, and an `async`/`await` _preview_.

## Prerequisites

- **Prior topics**: [topic 07 Object-Oriented Programming Essentials](./07-object-oriented-programming-essentials.md) (types,
  classes) and [topic 45 Just Enough Kotlin](./45-just-enough-kotlin.md) (null-safety/optionals intuition
  transfers).
- **Tools & environment**: a **macOS** machine (Swift toolchain; Xcode not yet required); the `swift` REPL
  - `swiftc` from the CLI; Neovim/VSCode (DD-17). (Linux Swift works for the CLI examples.)
- **Assumed knowledge**: classes/structs + types (topic 07); optional/nullable thinking (topic 45).

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

---

← Previous: [46 · Android App Development](./46-android-app-development.md) · Next: [48 · iOS App Development](./48-ios-app-development.md) →
