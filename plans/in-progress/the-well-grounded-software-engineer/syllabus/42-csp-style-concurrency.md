# 42 · CSP-Style Concurrency (By Example, Go †)

**prd row**: Pass 4 · Concurrency & Systems · By Example · Go † · Learn 142 / Drill 242 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the CSP (communicating-sequential-processes) concurrency model in Go — goroutines,
channels, `select`, `sync`, `context`/cancellation, pipelines, worker pools, and the race detector. Sets
up the deliberate contrast with the actor model in
[`44-actor-model-concurrency`](./44-actor-model-concurrency.md). Deepens the concepts from
[`19-concurrency-and-parallelism`](./19-concurrency-and-parallelism.md).

## Prerequisites

- **Prior topics**: [topic 41 Just Enough Go](./41-just-enough-go.md) (the language + a channel preview) and
  [topic 19 Concurrency & Parallelism](./19-concurrency-and-parallelism.md) (races, deadlocks, the shared-
  state hazards CSP avoids).
- **Tools & environment**: a macOS/Linux terminal; the **Go toolchain** with the **race detector**
  (`go test -race`); Neovim/VSCode with Go LSP (DD-17).
- **Assumed knowledge**: Go syntax + goroutines/channels at a preview level (topic 41); what a race
  condition and a deadlock are (topic 19).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: channels, `select`, `sync` (Mutex/WaitGroup/Once), `context` cancellation,
  `go test -race`, and fan-in/fan-out + worker-pool patterns are long-stable Go concurrency primitives —
  nothing in Go 1.26 changes this surface. No corrections.

## Items

- The CSP model: communicating sequential processes; goroutines; channels (buffered/unbuffered).
- `select`; `sync` primitives (Mutex/WaitGroup/Once); `context` & cancellation.
- Pipelines and fan-in / fan-out; worker pools.
- The race detector (`go test -race`); common concurrency bugs.
- Contrast with the actor model (`actor-model-concurrency`).

## Worked examples

Colocated under `csp-style-concurrency/learning/code/`; each runnable + race-checked (DD-20/DD-30).

- **beginner** — a goroutine + channel hand-off.
- **intermediate** — a pipeline with fan-out / fan-in.
- **advanced** — a worker pool with `context` cancellation, verified clean with the race detector.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: build a concurrent work processor in Go — a bounded worker pool draining a channel pipeline
  (fan-out / fan-in), coordinated with `select` + `context` cancellation and `sync` primitives, that
  shuts down cleanly and passes `go test -race` with no data races — a demonstrably correct CSP design.
- **Concepts exercised**: [ ] goroutines + channels [ ] a fan-out/fan-in pipeline [ ] a bounded worker pool
  [ ] `select` + `context` cancellation [ ] `sync` coordination [ ] a race-clean `go test -race`.
- **Ordered steps**:
  1. `.../learning/capstone/code/` — a channel pipeline (producer → workers → collector). Verify all items
     flow through and the counts reconcile.
  2. Bound it with a worker pool + `select` + `context` cancellation. Verify a cancel signal stops all
     workers promptly with no goroutine leak.
  3. `go test -race`. Verify the suite passes with **no** race-detector warnings.
- **Acceptance criteria**: the pipeline processes every item; cancellation shuts workers down cleanly with
  no leak; `go test -race` reports zero data races.
- **Done bar**: runnable end-to-end + race-clean + web-verified.

---

← Previous: [41 · Just Enough Go](./41-just-enough-go.md) · Next: [43 · Just Enough Elixir](./43-just-enough-elixir.md) →
