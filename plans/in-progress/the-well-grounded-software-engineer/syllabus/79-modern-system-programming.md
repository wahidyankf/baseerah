# 79 · Modern System Programming (By Example, Rust †)

**prd row**: Pass 4 · Concurrency & Systems · By Example · Rust † · Learn 179 / Drill 279 · Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: systems programming without the footguns — ownership as a compile-time memory-safety
strategy, fearless concurrency, zero-cost abstractions, and FFI across the C boundary. This is the modern
counterpart to the C systems topic ([`77-system-programming`](./77-system-programming.md)): the same
low-level control, but with the classes of bug that dominate C — use-after-free, data races, buffer
overruns — moved from runtime crashes to compile errors. The usable language slice is the prerequisite
[`78-just-enough-rust`](./78-just-enough-rust.md). `†`: Rust, driving `cargo` and building native binaries.

## Why this exists · the big idea

- **The problem before the solution**: systems languages gave you full control over memory and hardware and
  charged for it in the worst currency — use-after-free, double-free, buffer overflows, and data races that
  compile cleanly, ship, and then corrupt memory or open security holes in production. Decades of C/C++ CVEs
  are overwhelmingly this one category.
- **Keep-this-if-you-forget-everything**: Rust's borrow checker enforces, at compile time, that memory has
  exactly one owner and that shared access is either many-readers or one-writer but never both — so the
  bugs that plague manual memory management become programs that simply do not compile. Safety is a property
  the compiler proves, not a discipline you hope you maintained.
- **Big ideas touched**: `taming-state` (ownership and borrowing are a static discipline for mutable shared
  state — aliasing and mutation cannot coexist, which is exactly what makes data races unrepresentable),
  `mechanism-vs-policy` (zero-cost abstractions and the `unsafe` boundary separate the safe machinery you
  build on from the small, audited places where you take manual control of the mechanism).

## Prerequisites

- **Prior topics**: [topic 78 Just Enough Rust](./78-just-enough-rust.md) (ownership/borrowing intuition,
  the type system, `Result`/`Option`, traits, pattern matching) and [topic 76 Windows OS](./76-windows-os.md)
  (the OS-level view of memory, threads, and system calls that systems code sits on).
- **Tools & environment**: a macOS/Linux/Windows terminal; the **Rust toolchain** (`cargo`, `rustc`) pinned
  to a current stable; a C compiler/toolchain available for the FFI examples; Neovim/VSCode with the Rust
  LSP (rust-analyzer, DD-17).
- **Assumed knowledge**: ownership/borrowing and lifetimes at intuition level, traits, and `Result`/`Option`
  (topic 78); processes/threads and system calls (topics 76/77); the memory hierarchy and stack-vs-heap
  (topic 20).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: the core model is stable and correctly left version-unpinned — ownership,
  borrowing, lifetimes, `Send`/`Sync` and the "fearless concurrency" guarantees, zero-cost abstractions, and
  `unsafe`/FFI across the C ABI are settled Rust semantics, not fast-moving surface. Keep the toolchain at
  "a current stable" in shipped text.
- 2026-07-12 — verified (GAP for plan owner): async runtimes and specific crates (an async executor, an FFI
  helper crate) are referenced by role, not pinned — re-verify exact crate names/versions once the worked
  examples are drafted, and keep the standard-library-first framing (threads/channels before an async
  runtime).

## Items

- Ownership as a memory strategy: move semantics, borrowing (`&`/`&mut`), lifetimes, and why the borrow
  checker rejects aliasing-plus-mutation.
- Fearless concurrency: threads, channels, `Arc`/`Mutex`, and how `Send`/`Sync` make data races a compile
  error rather than a runtime lottery.
- Zero-cost abstractions: iterators, traits, and generics that compile down to the same code you would have
  written by hand.
- Error handling at systems level: `Result`, `?`, and clear failure paths without exceptions.
- `unsafe` and its contract: what `unsafe` actually permits, why it exists, and how to keep it small and
  audited behind a safe API.
- FFI: calling C from Rust and exposing Rust to C across the C ABI, and the ownership rules at the boundary.

## Tensions & trade-offs — when NOT to reach for this

- **The borrow checker is a learning tax**: patterns that are trivial in a GC'd or manually-managed
  language (self-referential structures, shared mutable graphs, some callback designs) fight the borrow
  checker and require rethinking. For a team without the time to climb that curve, a memory-safe GC language
  may ship the same feature faster.
- **`unsafe` is not a safety escape valve to reach for casually**: dropping into `unsafe` to silence the
  compiler reintroduces exactly the bugs Rust exists to prevent — now in code the compiler no longer checks.
  Every `unsafe` block is a manual proof obligation; if you find yourself writing many, the design is
  usually wrong.
- **When the ecosystem or hard-real-time constraints say otherwise**: an existing C/C++ codebase, a platform
  with only a C toolchain, or a hard-real-time context where you cannot tolerate any allocation may make Rust
  the wrong or premature choice. Rust wins the safety argument, not every argument.

## Lineage — why it beat the alternative

- Systems programming lived on C and C++ for decades: unmatched control and performance, paid for with a
  standing epidemic of memory-safety bugs that industry data repeatedly ties to the majority of critical
  CVEs. The alternatives each gave something up — garbage-collected languages removed the bug class but added
  a runtime and unpredictable pauses unacceptable for kernels, drivers, and hot paths. Rust's bet was that
  an ownership type system could prove memory and thread safety at compile time with no runtime cost,
  keeping C-level control while deleting the bug class — a bet now validated by its adoption in operating
  systems, browsers, and infrastructure. The safe-systems instincts, concurrency model, and FFI boundary
  built here are the toolkit you carry into any low-level work, and they contrast directly with the
  manual-discipline model of [`77-system-programming`](./77-system-programming.md).

## Worked examples

Colocated under `modern-system-programming/learning/code/`; each runnable via `cargo` (DD-20/DD-30).

- **beginner** — an ownership/borrowing exercise: a program the borrow checker rejects, and the corrected
  version, with the compiler error explained.
- **intermediate** — a fearlessly-concurrent pipeline: worker threads communicating over channels, or shared
  state behind `Arc<Mutex<_>>`, that the compiler proves free of data races.
- **advanced** — an FFI bridge: call a C function from Rust (and/or expose a Rust function to C), handling
  ownership across the boundary, with the only `unsafe` block small and wrapped behind a safe API.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: build one small systems tool in Rust that exercises the safety story end to end — an ownership-
  correct core, a concurrent component the compiler proves race-free, a zero-cost abstraction over its data,
  and an FFI call across the C boundary whose single `unsafe` block is contained behind a safe API.
- **Concepts exercised**: [ ] ownership/borrowing/lifetimes correct by construction [ ] threads + channels
  or `Arc<Mutex<_>>` with `Send`/`Sync` [ ] a zero-cost iterator/trait abstraction [ ] `Result`/`?` error
  handling [ ] one FFI call across the C ABI [ ] a small audited `unsafe` block behind a safe wrapper.
- **Ordered steps**:
  1. `.../learning/capstone/code/core/` — the ownership-correct core logic with `Result`-based error paths.
     Verify `cargo build` compiles with no borrow-checker warnings and `cargo test` passes.
  2. `.../learning/capstone/code/concurrent/` — add a concurrent component (threads + channels or shared
     state behind `Arc<Mutex<_>>`). Verify it compiles (the type system proving `Send`/`Sync`) and runs
     without a data race under repeated execution.
  3. `.../learning/capstone/code/abstract/` — express the data handling through a zero-cost
     iterator/trait abstraction. Verify behavior is unchanged and the tests still pass.
  4. `.../learning/capstone/code/ffi/` — call a C function across the boundary, handling ownership, with the
     `unsafe` block small and wrapped in a safe API. Verify the FFI call returns the expected result and the
     `unsafe` surface is minimal and documented.
- **Acceptance criteria**: the core is borrow-checker-clean; the concurrent component is provably race-free
  and runs repeatably; the abstraction is zero-cost with unchanged behavior; the FFI call works with
  ownership handled and `unsafe` confined behind a safe wrapper.
- **Done bar**: runnable end-to-end + web-verified.

## Read more

**Books**

- **Rust for Rustaceans** — Jon Gjengset (2021). The canonical intermediate/advanced Rust book for
  engineers moving from language basics into idiomatic systems code, traits, and unsafe boundaries.
- **Rust Atomics and Locks** — Mara Bos (2023). Authoritative treatment of low-level concurrency primitives
  in Rust, written by the former Rust library team lead; free online. <https://marabos.nl/atomics/>

**Papers & articles**

- **Writing an OS in Rust** — Philipp Oppermann. Widely-used, free blog series building a minimal x86-64
  kernel in Rust from bare metal up. <https://os.phil-opp.com/>
- **The Rustonomicon** — The Rust Project. Official reference for unsafe Rust internals, required reading
  for systems-level Rust work. <https://doc.rust-lang.org/nomicon/>

---

← Previous: [78 · Just Enough Rust](./78-just-enough-rust.md) · Next: [80 · Just Enough Java](./80-just-enough-java.md) →
