# 43 · Just Enough Elixir (Primer §, Elixir †)

**prd row**: Pass 4 · Concurrency & Systems · Primer § · Elixir † · Learn 143 / Drill 243 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: `§` primer — **just enough Elixir** to be productive in
[`44-actor-model-concurrency`](./44-actor-model-concurrency.md). Immutable data, pattern matching, the pipe
operator, functions/modules, recursion, and a `spawn`/`send`/`receive` _preview_ only (process depth
belongs to topic 44).

## Prerequisites

- **Prior topics**: [topic 18 Functional Programming](./18-functional-programming.md) (immutability,
  pure functions, recursion over loops) and [topic 04 Just Enough Python](./04-just-enough-python.md) (a
  contrasting first language).
- **Tools & environment**: a macOS/Linux terminal; **Elixir** + `mix` + `iex`, pinned to a current stable
  release (note the Erlang/Elixir license posture, DD-15); Neovim/VSCode (DD-17).
- **Assumed knowledge**: immutability + recursion (topic 18); running a REPL + a CLI build tool
  (topics 04/05).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: keep the version unpinned in shipped text. Current stable is **Elixir v1.20**
  (1.20.2 with OTP 29 released 2026-06-23; requires OTP 27+). `spawn`/`send`/`receive` preview syntax and
  `mix`/`iex` usage are unchanged. Re-pull the exact version at authoring time.
- 2026-07-12 — verified: **both Erlang/OTP (since OTP 18.0) and Elixir are Apache License 2.0** — permissive
  open source, not source-available (DD-21 clean). (github.com/erlang/otp/blob/master/LICENSE.txt)

## Items

- `iex` and `mix` from the CLI; immutable data; pattern matching; the pipe operator (`|>`).
- Functions & modules; recursion (no mutable loops).
- The process **preview**: `spawn` / `send` / `receive` at a glance (depth in `actor-model-concurrency`).

## Worked examples

Colocated under `just-enough-elixir/learning/code/`; each runnable via `iex`/`mix` (DD-20/DD-30).

- **beginner** — `iex` exploration + a `mix` script.
- **intermediate** — pattern matching + a pipeline (`|>`).
- **advanced** — a module with recursion + a spawned-process preview.

## Capstone spec — intra-topic (primer → light consolidation)

- **Goal**: build a small `mix` program that exercises the primer's surface — immutable data, pattern
  matching, the pipe operator, a module with recursion, and a single `spawn`/`send`/`receive` message
  hand-off — proving readiness for actor-model concurrency.
- **Concepts exercised**: [ ] a `mix` project [ ] pattern matching [ ] the pipe operator [ ] recursion
  (no mutable loop) [ ] a `spawn`/`send`/`receive` hand-off.
- **Ordered steps**:
  1. `.../learning/capstone/code/` — a module transforming data through a `|>` pipeline with pattern
     matching. Verify the pipeline produces the expected value.
  2. Add a recursive function (e.g. a fold/aggregate) with no mutable loop. Verify it returns the correct
     result on a known input.
  3. Add a `spawn`ed process with `send`/`receive`. Verify the message round-trips and the program exits
     cleanly.
- **Acceptance criteria**: the pipeline + pattern matching work; the recursion is correct; the process
  hand-off round-trips.
- **Done bar**: runnable end-to-end + web-verified.

---

← Previous: [42 · CSP-Style Concurrency](./42-csp-style-concurrency.md) · Next: [44 · Actor-Model Concurrency](./44-actor-model-concurrency.md) →
