# 24 · Concurrency & Parallelism (Core) (By Example, Python)

**prd row**: Pass 2 · Depth, Design & Craft · By Example · Python · Learn 124 / Drill 224 · Nvim-ready Yes ·
VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the **core concurrency model** every engineer needs — threads vs processes vs async,
synchronization, races and deadlocks, message passing, and parallel decomposition — in Python, including
the GIL and free-threaded CPython. The two alternative styles get their own Pass-4 topics:
CSP → [`61-csp-style-concurrency`](./61-csp-style-concurrency.md) (Go) and the actor model →
[`63-actor-model-concurrency`](./63-actor-model-concurrency.md) (Elixir). Those three are integrated in
the `capstone-concurrency-showdown` inter-topic capstone.

## Why this exists · the big idea

- **The problem before the solution**: one thing at a time is simple but slow and unresponsive; the moment
  two things run at once, shared state corrupts and bugs stop being reproducible.
- **Keep-this-if-you-forget-everything**: don't share mutable state; when you must, protect it — almost
  every concurrency bug is a shared-state bug wearing a costume.
- **Big ideas touched**: `taming-state` (the whole discipline is containing state two things can touch),
  `determinism-vs-emergence` (interleavings turn deterministic code into emergent, order-dependent behavior).

## Prerequisites

- **Prior topics**: [topic 4 Just Enough Python](./04-just-enough-python.md);
  [topic 7 Data Structures & Algorithms Essentials](./07-data-structures-and-algorithms-essentials.md)
  (queues, the producer/consumer shape); [topic 23 Functional Programming](./23-functional-programming.md)
  for the "reduce shared mutable state" mindset.
- **Tools & environment**: a macOS/Linux terminal; **Python 3.x** with `threading`, `multiprocessing`,
  `asyncio`, `concurrent.futures` (stdlib); optionally the free-threaded `3.14t` build for the GIL demo.
- **Assumed knowledge**: writing Python functions and loops; running a script from the CLI; the idea that
  two things running at once can interfere.

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: free-threaded CPython is real and correctly pinned — PEP 703 (no-GIL design,
  accepted Oct 2023) + PEP 779 ("supported status" criteria, accepted). As of **Python 3.14** the
  free-threaded build is officially **supported** (Phase II), **not yet the default** (Phase III pending).
  Binaries ship as `python3.14t` / `3.14.0t` (ABI tag `cp314t`; macOS opt-in checkbox, Windows
  `py install 3.14t`). Single-threaded overhead ≈5–10% depending on platform — cite if the body quantifies
  it. `asyncio` / `concurrent.futures` APIs current. (peps.python.org/pep-0779 / docs.python.org /
  py-free-threading.github.io)

## Items

- Concurrency vs parallelism; processes vs threads vs async; the GIL and its implications, and
  free-threaded CPython (PEP 703/779, the `3.14t` build; officially supported since 3.14).
- Synchronization: locks/mutexes, semaphores, condition variables; deadlock/livelock/starvation.
- Race conditions & data races; atomicity; memory-visibility intuition.
- Message passing & queues; producer/consumer; thread/process pools.
- `async`/`await` & event loops; cooperative vs preemptive scheduling.
- Parallel decomposition: map-reduce style, work stealing (concept), Amdahl's-law intuition.
- Forward pointers: CSP style (`csp-style-concurrency`, Go) and the actor model
  (`actor-model-concurrency`, Elixir).

## Worked examples

Colocated under `concurrency-and-parallelism/learning/code/`; each runnable and reproducible (DD-20/DD-30).

- **beginner** — threads vs `asyncio` for I/O-bound work; a shared-counter race and its lock fix.
- **intermediate** — producer/consumer with a bounded queue; a thread/process pool over a CPU/I/O workload.
- **advanced** — reproduce & resolve a deadlock; `asyncio.gather` over many I/O tasks; process-based
  parallelism to sidestep the GIL for CPU-bound work.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: build a concurrent work processor (e.g. a URL/file fetch-and-aggregate pipeline) three ways —
  thread pool, `asyncio`, and process pool — measure them against a serial baseline on I/O-bound and
  CPU-bound workloads, and demonstrate a race + its synchronized fix and a deadlock + its resolution.
- **Concepts exercised**: [ ] thread pool [ ] `asyncio` event loop [ ] process pool (GIL sidestep)
  [ ] a race condition + lock fix [ ] a reproduced-and-resolved deadlock [ ] Amdahl's-law reasoning on the
  measured speedups.
- **Ordered steps**:
  1. `.../learning/capstone/code/serial.py` — the serial baseline + a timing harness. Verify it produces
     the correct aggregate and a baseline time.
  2. `pool_threads.py` + `async_run.py` — thread-pool and `asyncio` versions for the I/O-bound workload.
     Verify each matches the baseline result and is faster on I/O.
  3. `pool_process.py` — process-pool version for the CPU-bound workload. Verify it beats threads on CPU
     work and explain why (GIL).
  4. `race_demo.py` — a shared-counter race, then the lock fix; a deadlock, then its resolution. Verify the
     unsafe version is observably wrong and the fixed version is correct/terminating.
- **Acceptance criteria**: all variants produce the identical correct aggregate; measured speedups match
  the expected pattern (async/threads win I/O, processes win CPU); the race and deadlock are demonstrably
  fixed; speedups are explained with Amdahl's-law intuition.
- **Done bar**: runnable end-to-end + web-verified.

## Read more

**Books**

- **The Art of Multiprocessor Programming** — Maurice Herlihy & Nir Shavit (2008; revised ed. 2012). The standard graduate text on concurrent data structures, synchronization, and memory models.
- **Java Concurrency in Practice** — Brian Goetz, Tim Peierls, Joshua Bloch, Joseph Bowbeer, David Holmes, Doug Lea (2006). Canonical practitioner's guide to safe concurrent programming and the Java Memory Model.
- **Seven Concurrency Models in Seven Weeks** — Paul Butcher (2014). Practical survey of threads, actors, CSP, STM, and dataflow concurrency models.

**Papers & articles**

- **Is Parallel Programming Hard, And, If So, What Can You Do About It?** — Paul E. McKenney (continually updated). Free, encyclopedic handbook of parallel-programming techniques from a Linux kernel maintainer. <https://mirrors.edge.kernel.org/pub/linux/kernel/people/paulmck/perfbook/perfbook.html>
- **Time, Clocks, and the Ordering of Events in a Distributed System** — Leslie Lamport (1978). Founding paper of logical clocks and the happens-before relation underpinning distributed concurrency. <https://dl.acm.org/doi/10.1145/359545.359563>
- **The Problem with Threads** — Edward A. Lee (2006). Influential argument that nondeterministic thread interleaving is the wrong default concurrency model. <https://www2.eecs.berkeley.edu/Pubs/TechRpts/2006/EECS-2006-1.pdf>

---

← Previous: [23 · Functional Programming](./23-functional-programming.md) · Next: [25 · Advanced Algorithms](./25-advanced-algorithms.md) →
