# 61 · CSP-Style Concurrency (By Example, Go †)

**prd row**: Pass 4 · Concurrency & Systems · By Example · Go † · Learn 161 / Drill 261 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the CSP (communicating-sequential-processes) concurrency model in Go — goroutines,
channels, `select`, `sync`, `context`/cancellation, pipelines, worker pools, and the race detector. Sets
up the deliberate contrast with the actor model in
[`63-actor-model-concurrency`](./63-actor-model-concurrency.md). Deepens the concepts from
[`24-concurrency-and-parallelism`](./24-concurrency-and-parallelism.md).

## Why this exists · the big idea

- **The problem before the solution**: shared mutable state across threads breeds races and deadlocks that
  are nearly impossible to reproduce or reason about — CSP answers with a discipline where goroutines never
  share memory; they hand values across channels.
- **Keep-this-if-you-forget-everything**: "don't communicate by sharing memory; share memory by
  communicating" — make the channel the synchronization point and whole classes of races disappear.
- **Big ideas touched**: `taming-state` — channels contain state by transferring ownership across a
  boundary instead of sharing it; `determinism-vs-emergence` — pipelines and worker pools compose into
  predictable dataflow, yet scheduling and cancellation add emergent timing you must design for (hence the
  race detector).

## Prerequisites

- **Prior topics**: [topic 60 Just Enough Go](./60-just-enough-go.md) (the language + a channel preview) and
  [topic 24 Concurrency & Parallelism](./24-concurrency-and-parallelism.md) (races, deadlocks, the shared-
  state hazards CSP avoids).
- **Tools & environment**: a macOS/Linux terminal; the **Go toolchain** with the **race detector**
  (`go test -race`); Neovim/VSCode with Go LSP (DD-17).
- **Assumed knowledge**: Go syntax + goroutines/channels at a preview level (topic 60); what a race
  condition and a deadlock are (topic 24).

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

## Read more

**Books**

- **The Go Programming Language** — Alan A. A. Donovan & Brian W. Kernighan (2015). The definitive Go reference book, with a canonical treatment of goroutines, channels, and `select`.
- **Communicating Sequential Processes** — C. A. R. Hoare (1985). Hoare's own book-length formalization of CSP, freely distributed by the author's estate/collaborators. <http://www.usingcsp.com/cspbook.pdf>

**Papers & articles**

- **Communicating Sequential Processes** — C. A. R. Hoare, _Communications of the ACM_ 21(8) (1978). The original paper that introduced CSP, the formal model underlying Go's concurrency primitives. <https://www.cs.cmu.edu/~crary/819-f09/Hoare78.pdf>
- **Share Memory By Communicating** — Andrew Gerrand, The Go Blog (2010). The Go team's canonical articulation of "don't communicate by sharing memory; share memory by communicating." <https://go.dev/blog/codelab-share>
- **Concurrency Is Not Parallelism** — Rob Pike (2012). Foundational talk by a Go co-creator distinguishing concurrent design from parallel execution. <https://go.dev/talks/2012/waza.slide>
- **Effective Go** — The Go Authors (official documentation). Canonical guidance on idiomatic goroutine and channel usage. <https://go.dev/doc/effective_go>

---

← Previous: [60 · Just Enough Go](./60-just-enough-go.md) · Next: [62 · Just Enough Elixir](./62-just-enough-elixir.md) →
