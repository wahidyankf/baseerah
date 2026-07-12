# 78 · Just Enough Rust (Primer §, Rust †)

**prd row**: Pass 4 · Concurrency & Systems · Primer § · Rust † · Learn 178 / Drill 278 · Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: `§` primer — **just enough Rust** to be productive in modern systems programming
([`79-modern-system-programming`](./79-modern-system-programming.md)). `cargo`, an intuition for
ownership/borrowing/lifetimes, the type system, `Result`/`Option`, traits, and pattern matching. `†`:
Rust, run and built with the `cargo` toolchain.

## Why this exists · the big idea

- **The problem before the solution**: Rust's ownership model is the one genuinely new idea most engineers
  meet here, and trying to learn it while also wrestling concurrency, FFI, and `unsafe` at once is how people
  bounce off the language — so the primer isolates the language core first.
- **Keep-this-if-you-forget-everything**: in Rust every value has exactly one owner, and the borrow checker
  enforces that at compile time — fighting the borrow checker early is normal, and the moment its rules click
  is the moment Rust starts feeling productive rather than obstructive.
- **Big ideas touched**: `taming-state` (ownership and borrowing are a compile-time discipline for who may
  read and who may mutate — the language's whole approach to shared mutable state), `abstraction-and-its-cost`
  (traits and `Result`/`Option` give expressive, zero-cost abstractions — the cost is paid up front in
  explicitness the compiler demands).

## Prerequisites

- **Prior topics**: [topic 8 Object-Oriented Programming Essentials](./08-object-oriented-programming-essentials.md)
  (interfaces map onto traits, and composition-over-inheritance is Rust's default) and
  [topic 20 Computer Architecture](./20-computer-architecture.md) (stack vs heap and the memory model that
  ownership is really about).
- **Tools & environment**: a macOS/Linux/Windows terminal; the **Rust toolchain** (`cargo`, `rustc`) pinned
  to a current stable; Neovim/VSCode with the Rust LSP (rust-analyzer, DD-17).
- **Assumed knowledge**: interfaces and composition (topic 08); stack-vs-heap and the memory hierarchy
  (topic 20); running a CLI build tool (topic 05).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: keep the Rust toolchain at "a current stable" in shipped text — `cargo`
  (`new`/`run`/`build`/`test`), ownership/borrowing/lifetimes, `Result`/`Option`, traits, generics, and
  pattern matching are stable, settled language surface. Rust ships on a six-week cadence, so a pinned
  version number would go stale fast; the language core here does not.
- 2026-07-12 — verified: the primer stays on the standard library and language core, so no third-party
  crate version is claimed — nothing to re-pull beyond the toolchain itself.

## Items

- The `cargo` CLI from the terminal: `new` / `run` / `build` / `test`, and adding a dependency.
- Ownership, borrowing (`&`/`&mut`), and a working intuition for lifetimes.
- The type system: structs, enums, generics, and traits (Rust's interface mechanism).
- `Result` and `Option` for fallible and optional values, with `?` for propagation.
- Pattern matching with `match` and `if let`, exhaustively over enums.

## Worked examples

Colocated under `just-enough-rust/learning/code/`; each runnable via `cargo` (DD-20/DD-30).

- **beginner** — a `cargo run` program with structs, enums, and a `match`.
- **intermediate** — a function that returns a `Result`, propagated with `?`, plus an `Option` handled with
  pattern matching.
- **advanced** — a small generic function or struct constrained by a trait, showing ownership vs borrowing
  at a call site.

## Capstone spec — intra-topic (primer → light consolidation)

- **Goal**: build a small Rust program that exercises the primer's surface — structs/enums, a trait, a
  generic, `Result`/`Option` with `?`, and exhaustive pattern matching — runnable via `cargo run` plus a
  `cargo test`, with the borrow checker satisfied, proving readiness for modern systems programming.
- **Concepts exercised**: [ ] structs + enums [ ] a trait implemented for a type [ ] a generic function or
  struct [ ] `Result`/`Option` + `?` [ ] exhaustive `match` [ ] a `cargo test`.
- **Ordered steps**:
  1. `.../learning/capstone/code/` — a program modeling data with structs and enums and a `match` over the
     enum. Verify `cargo run` produces the expected output with an exhaustive match.
  2. Add a trait and implement it for a type, and a fallible function returning `Result` propagated with
     `?`. Verify the trait dispatches and the error path returns an `Err` cleanly.
  3. Add a generic constrained by the trait and a `cargo test`. Verify borrow-checker-clean compilation and
     that the test passes.
- **Acceptance criteria**: structs/enums and exhaustive matching work; the trait and generic compile and
  dispatch; `Result`/`Option` + `?` handle success and failure; the code is borrow-checker-clean and
  `cargo test` passes.
- **Done bar**: runnable end-to-end + web-verified.

## Read more

**Books**

- **The Rust Programming Language**, 2nd ed. — Steve Klabnik, Carol Nichols, with the Rust community. The
  official book and canonical entry point to Rust, including ownership and borrowing; maintained by the Rust
  project itself. <https://doc.rust-lang.org/book/>
- **Programming Rust**, 2nd ed. — Jim Blandy, Jason Orendorff, Leonora F. S. Tindall (2021). The deep,
  systems-oriented O'Reilly treatment of Rust's ownership and type system.

**Papers & articles**

- **Rust By Example** — The Rust Project. Official companion of runnable examples reinforcing
  ownership/borrowing and core syntax. <https://doc.rust-lang.org/rust-by-example/>

---

← Previous: [77 · System Programming](./77-system-programming.md) · Next: [79 · Modern System Programming](./79-modern-system-programming.md) →
