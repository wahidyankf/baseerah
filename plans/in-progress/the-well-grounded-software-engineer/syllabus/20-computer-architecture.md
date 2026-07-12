# 20 · Computer Architecture (By Example, C †)

**prd row**: Pass 2 · Depth, Design & Craft · By Example · C † · Learn 120 / Drill 220 · Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: how the machine actually runs your code, in the CS:APP "program in the machine's
terms" model — the memory hierarchy and caches, the cost of a cache miss, virtual memory,
integer/float representation, endianness, and why data layout dominates performance. `†`: examples in
C (with a little assembly to read), where memory layout and representation are visible rather than
hidden. Builds on [`19-computer-science-foundations`](./19-computer-science-foundations.md).

## Why this exists · the big idea

- **The problem before the solution**: reasoning about a flat, uniform memory and a CPU that runs one
  instruction at a time stopped predicting performance once caches, pipelines, and virtual memory
  arrived — the same big-O algorithm now runs an order of magnitude apart depending on how it touches
  memory.
- **Keep-this-if-you-forget-everything**: memory is a hierarchy and the CPU is fast only when it hits
  cache — sequential, cache-friendly access to compact data beats a "clever" algorithm that chases
  pointers, because a cache miss costs hundreds of cycles.
- **Big ideas touched**: `layering-and-leaks` (this is the layer just under your language — its cache,
  page, and word-size behavior leaks upward as performance you must explain), `abstraction-and-its-cost`
  (the "flat memory, one instruction at a time" abstraction is convenient and wrong; the cost it hides
  is exactly the 100× gap between cache hit and miss).

## Prerequisites

- **Prior topics**: [topic 4 Just Enough Python](./04-just-enough-python.md) and
  [topic 19 Computer Science Foundations](./19-computer-science-foundations.md).
- **Tools & environment**: a macOS/Linux terminal; a C toolchain (a recent stable `clang`/`gcc`); a
  profiler/`perf`-style tool to measure cache and cycle behavior; optionally a disassembler to read
  emitted assembly; Neovim/VSCode with the C LSP (DD-17).
- **Assumed knowledge**: reading and running a small C program (topic 19); binary/number
  representation and complexity intuition (topic 19); reading a typed script to drive experiments
  (topic 04).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: the model taught here (memory hierarchy, cache lines, virtual memory,
  two's-complement integers, IEEE-754 floats, endianness, data-layout effects) is long-settled and
  correctly left version-unpinned. CS:APP remains the canonical programmer's-eye reference.
- 2026-07-12 — verified: exact cache sizes, line sizes, and miss penalties are microarchitecture- and
  vendor-specific — the file describes them as order-of-magnitude and hardware-dependent rather than
  asserting one CPU's numbers.

## Items

- The memory hierarchy: registers → L1/L2/L3 cache → DRAM → disk, and the latency gap between each
  level.
- Caches in practice: cache lines, spatial and temporal locality, and the cost of a cache miss.
- Virtual memory: pages, the TLB, and how the address you see maps to physical memory.
- Number representation: two's-complement integers, overflow, and IEEE-754 floating point and its
  rounding traps.
- Endianness and alignment: byte order on the wire and in memory, struct padding, and portability
  hazards.
- Data layout for performance: array-of-structs vs struct-of-arrays, and why layout beats micro-tuning
  in hot loops.

## Tensions & trade-offs — when NOT to reach for this

- **Cache-tuning is premature for most code**: data-layout tuning pays off in hot loops and tight
  kernels, but rewriting readable code for cache lines before a profile proves it's the bottleneck
  trades clarity for imagined speed.
- **The model is itself an abstraction**: modern out-of-order, superscalar CPUs with speculative
  execution defeat back-of-envelope reasoning — mechanical-sympathy intuition must be _checked_ against
  a profiler (topic 16), not trusted blind.
- **Portability vs exploiting the machine**: endianness, word size, and alignment assumptions bake in
  hardware details; code that depends on them is fast on one target and broken on another. Reach for
  the machine's specifics only where the win is measured and the target is fixed.

## Lineage — why it beat the alternative

- Programmers once reasoned about a flat, uniform memory and a CPU that executed one instruction at a
  time — a model that stopped predicting performance once caches, pipelines, and virtual memory arrived
  and the "memory wall" (CPU speed outpacing DRAM latency) opened. The CS:APP "program in the machine's
  terms" view won because it explains the gaps the flat model can't: why the same algorithm runs an
  order of magnitude faster with a cache-friendly layout, and why a float comparison can lie. This
  mechanical-sympathy foundation feeds the systems-programming topics — [`78-just-enough-rust`](./78-just-enough-rust.md)
  lists it as a prerequisite — and gives [`16-debugging-and-profiling`](./16-debugging-and-profiling.md)
  the vocabulary to read a profile instead of guessing.

## Worked examples

Colocated under `computer-architecture/learning/code/`; each is a small C program you compile, run,
and measure to make an invisible cost visible (DD-20/DD-30).

- **beginner** — inspect integer and float representation and endianness: print the bytes of a value,
  trigger an overflow, and show a float that doesn't compare equal.
- **intermediate** — measure a cache miss: traverse the same data with cache-friendly vs
  cache-hostile access patterns and time the difference.
- **advanced** — restructure a hot loop from array-of-structs to struct-of-arrays (or blocked
  traversal) and demonstrate the speedup with a profiler, tying the gain back to cache behavior.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: take one small numeric/data-processing kernel and make it measurably faster by changing
  only how it touches memory — proving that data layout, not cleverness, drove the win — while
  demonstrating representation and endianness hazards along the way.
- **Concepts exercised**: [ ] integer/float representation + an overflow/rounding hazard [ ] endianness
  inspection [ ] a measured cache-miss cost [ ] a data-layout transformation (AoS→SoA or blocking)
  [ ] a before/after profile [ ] a written explanation tying the speedup to the memory hierarchy.
- **Ordered steps**:
  1. `.../learning/capstone/code/repr.c` — print bytes of int/float values, force an overflow, and
     show a non-equal float compare. Verify the output matches the documented representation.
  2. `.../cache.c` — the kernel with a cache-hostile layout, timed. Verify the miss cost is
     reproducible across runs.
  3. Restructure to a cache-friendly layout and re-measure with a profiler. Verify a documented
     speedup attributable to cache behavior, with identical results.
  4. `.../explanation.md` — tie the numbers to the memory hierarchy. Verify the explanation matches the
     measured profile.
- **Acceptance criteria**: representation/endianness hazards are demonstrated; the layout change
  produces a measured, reproducible speedup with unchanged results; the explanation is grounded in the
  profile.
- **Done bar**: runnable end-to-end + web-verified.

## Read more

**Books**

- **Computer Organization and Design: The Hardware/Software Interface (RISC-V ed.)** — Patterson,
  Hennessy (2nd RISC-V ed., 2020). Standard bridge from digital logic to ISAs, pipelining, and the
  memory hierarchy.
- **Computer Systems: A Programmer's Perspective (CS:APP)** — Bryant, O'Hallaron (3rd ed., 2015).
  Canonical programmer's-eye view of machine representation, linking, and the memory hierarchy.
  <https://csapp.cs.cmu.edu/>
- **Computer Architecture: A Quantitative Approach** — Hennessy, Patterson (6th ed., 2017).
  Graduate-level, data-driven reference for ILP, pipelining, and memory-system trade-offs.

**Papers & articles**

- **"The Case for the Reduced Instruction Set Computer"** — Patterson, Ditzel (1980, ACM SIGARCH).
  Landmark paper founding the RISC design philosophy. <https://dl.acm.org/doi/10.1145/641914.641917>
- **What Every Programmer Should Know About Memory** — Ulrich Drepper (2007). Detailed explanation of
  cache hierarchies and NUMA on real hardware.
  <https://people.freebsd.org/~lstewart/articles/cpumemory.pdf>

---

← Previous: [19 · Computer Science Foundations](./19-computer-science-foundations.md) · Next: [21 · Object-Oriented Design & Patterns](./21-object-oriented-design-and-patterns.md) →
