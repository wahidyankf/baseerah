# 16 · Debugging & Profiling (By Example, Python + native †)

**prd row**: Pass 1 · Core Foundations · By Example · Python + native † · Learn 116 / Drill 216 · Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: finding and fixing what the tests didn't catch — interactive debuggers (breakpoints,
watches, stepping), sampling versus instrumenting profilers, flame graphs, and a systematic bisection
method. `†`: fully type-annotated Python examples (DD-34) plus a native-profiler pass to see costs the
interpreter hides. Builds directly on [`15-software-testing`](./15-software-testing.md) — tests tell
you _that_ something is wrong; this topic is _where_ and _why_.

## Why this exists · the big idea

- **The problem before the solution**: a failing test tells you something is wrong but not where or
  why; `print`-driven guessing scales poorly, and optimizing by hunch tunes the wrong 90% while the
  real bottleneck sits untouched.
- **Keep-this-if-you-forget-everything**: debugging is a search — form a hypothesis, change one thing,
  observe, halve the space; performance work is measure-first, because the hot spot is almost never
  where you would have guessed.
- **Big ideas touched**: `layering-and-leaks` (a bug or a hot spot usually lives at a seam — your code,
  the runtime, the OS, the CPU cache — and a profiler is how you see through the layer),
  `determinism-vs-emergence` (the hardest bugs are emergent — races, heisenbugs, load-dependent
  slowdowns — reproducible only by controlling the interaction, not the line).

## Prerequisites

- **Prior topics**: [topic 4 Just Enough Python](./04-just-enough-python.md),
  [topic 5 Just Enough Bash](./05-just-enough-bash.md), and
  [topic 15 Software Testing](./15-software-testing.md).
- **Tools & environment**: a macOS/Linux terminal; an interactive debugger (`pdb`/`debugpy` for
  Python, `gdb`/`lldb` for native); a sampling profiler and an instrumenting one (`cProfile`-style); a
  flame-graph renderer; Neovim/VSCode with DAP debugger integration (DD-17).
- **Assumed knowledge**: reading a stack trace and writing a failing test (topic 15); driving CLI
  tools (topic 05); reading a typed Python module (topic 04).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: the concepts (breakpoint/watch/step debugging, sampling vs instrumenting
  profilers, flame graphs, bisection) are tool-independent and stable; tool names are left generic and
  version-unpinned on purpose. Python's `pdb`/`cProfile` are standard-library and current; native
  `perf`/`gdb`/`lldb` behavior is stable across recent releases.
- 2026-07-12 — verified: flame graphs (Brendan Gregg's visualization) remain the standard way to read
  a profile; there is no version to pin.

## Items

- Interactive debugging: breakpoints, conditional breakpoints, watches, and stepping (into/over/out)
  in Python and in a native debugger.
- Reading the state: the call stack, frames, and inspecting or altering variables at a breakpoint.
- Sampling vs instrumenting profilers: what each measures, their overhead, and when to use which.
- Flame graphs and hot spots: reading a profile to find where time (or allocation) actually goes.
- Systematic bisection: `git bisect` for regressions, and delta-debugging a failing input down to a
  minimal reproducer.
- Hard cases: reproducing races and heisenbugs, and profiling under realistic load rather than a toy
  input.

## Worked examples

Colocated under `debugging-and-profiling/learning/code/`; each is a seeded bug or slow function you
diagnose from the CLI, fully type-annotated Python (DD-20/DD-30/DD-34) with one native pass.

- **beginner** — set a conditional breakpoint to catch a bad value mid-loop; inspect the frame to find
  the wrong assumption.
- **intermediate** — profile a slow function two ways (sampling + instrumenting), render a flame graph,
  fix the real hot spot, and confirm the speedup.
- **advanced** — `git bisect` a regression to its introducing commit, delta-debug a large failing input
  to a minimal reproducer, then take one native-profiler pass to see a cost the interpreter hid.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: take a repo with one seeded correctness bug and one seeded performance bug and resolve both
  by method — bisect and minimize the failing case, fix it with a regression test; profile and fix the
  hot path with a before/after measurement.
- **Concepts exercised**: [ ] a debugger session (breakpoints/watch/step) [ ] `git bisect` to the
  offending commit [ ] delta-debugging to a minimal input [ ] a sampling + instrumenting profile
  [ ] a flame-graph read [ ] a documented before/after speedup.
- **Ordered steps**:
  1. `.../learning/capstone/code/` — reproduce the correctness bug; `git bisect` to the introducing
     commit and minimize the failing input. Verify the minimal case still fails.
  2. Fix the bug with a debugger-guided change plus a regression test. Verify the test fails before the
     fix and passes after.
  3. Profile the slow path (sampling then instrumenting), render a flame graph, and identify the hot
     spot. Verify both profilers agree on the hot spot.
  4. Fix the hot spot and re-measure. Verify a documented before/after improvement with no test
     regressions.
- **Acceptance criteria**: the regression is bisected and covered by a failing→passing test; the hot
  spot is identified from a profile (not a guess) and measurably improved.
- **Done bar**: runnable end-to-end + web-verified.

## Read more

**Books**

- **Why Programs Fail: A Guide to Systematic Debugging** — Andreas Zeller (2005). First comprehensive
  treatment of debugging as a systematic, teachable discipline.
- **Debugging: The 9 Indispensable Rules** — David J. Agans (2002). Concise, tool-agnostic heuristics
  for isolating faults.
- **Systems Performance** — Brendan Gregg (2nd ed., 2020). Standard reference for methodical
  performance analysis and profiling on Linux and in the cloud.

**Papers & articles**

- **What Every Programmer Should Know About Memory** — Ulrich Drepper (2007). Canonical explanation of
  cache and memory-hierarchy behavior for making sense of profiler output.
  <https://people.freebsd.org/~lstewart/articles/cpumemory.pdf>

---

← Previous: [15 · Software Testing](./15-software-testing.md) · Next: [17 · Security Essentials](./17-security-essentials.md) →
