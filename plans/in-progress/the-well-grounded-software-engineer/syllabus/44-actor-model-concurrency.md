# 44 · Actor-Model Concurrency (By Example, Elixir †)

**prd row**: Pass 4 · Concurrency & Systems · By Example · Elixir † · Learn 144 / Drill 244 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the actor concurrency model on the BEAM — processes, message passing, mailboxes,
`GenServer` for stateful processes, supervision trees + "let it crash", OTP applications — and the explicit
contrast with CSP ([`42-csp-style-concurrency`](./42-csp-style-concurrency.md)). License-aware (DD-15).

## Prerequisites

- **Prior topics**: [topic 43 Just Enough Elixir](./43-just-enough-elixir.md) (the language + a process
  preview), [topic 42 CSP-Style Concurrency](./42-csp-style-concurrency.md) (the model to contrast), and
  [topic 19 Concurrency & Parallelism](./19-concurrency-and-parallelism.md).
- **Tools & environment**: a macOS/Linux terminal; **Elixir/OTP** + `mix` + `iex`, pinned to a current
  stable release; Neovim/VSCode (DD-17).
- **Assumed knowledge**: Elixir syntax + a `spawn`/`send`/`receive` preview (topic 43); channels/CSP for the
  contrast (topic 42).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: `GenServer`/supervisor APIs and OTP application structure have no breaking changes;
  current with Elixir 1.20 / OTP 27-29.
- 2026-07-12 — verified: **Akka is on BSL 1.1** (source-available; production use needs a Lightbend
  commercial license). **Apache Pekko** — the ASF community fork of Akka 2.6.x — is **Apache-2.0** and is
  the current JVM open-source option (DD-21 clean). Nuance: BSL's rolling 3-year Change Date converts each
  _specific_ Akka release to Apache-2.0 eventually, but all current/new Akka releases stay BSL going
  forward, so "Akka moved to BSL" is the correct steady-state framing. (akka.io/bsl-license-faq / github.com/apache/pekko)

## Items

- The actor model: processes, message passing, mailboxes; `spawn` / `send` / `receive`.
- `GenServer` for stateful processes; supervision trees & "let it crash".
- OTP applications; registries.
- License-aware note (DD-15): Erlang/Elixir OTP is the primary vehicle; **Apache Pekko** (Apache-2.0) is the
  current JVM open-source option — **not Akka** (which moved to BSL).
- Contrast with CSP (`csp-style-concurrency`): shared-nothing message passing vs channels.

## Worked examples

Colocated under `actor-model-concurrency/learning/code/`; each runnable via `mix`/`iex` (DD-20/DD-30).

- **beginner** — `spawn` / `send` / `receive` message passing.
- **intermediate** — a `GenServer` stateful process.
- **advanced** — a supervised worker tree that recovers from a crash.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: build a small fault-tolerant OTP system — a `GenServer` holding state, a supervision tree that
  restarts a crashing worker under a chosen strategy ("let it crash"), and a registry — that demonstrably
  recovers from an induced crash without losing the supervised service, plus a written CSP-vs-actor
  contrast.
- **Concepts exercised**: [ ] `spawn`/`send`/`receive` [ ] a stateful `GenServer` [ ] a supervision tree +
  restart strategy [ ] crash recovery ("let it crash") [ ] a registry [ ] a CSP-vs-actor contrast note.
- **Ordered steps**:
  1. `.../learning/capstone/code/` — a `GenServer` managing state with a clean client API. Verify state
     updates + reads round-trip through messages.
  2. Put it under a supervisor with a restart strategy + a registry. Verify the supervisor starts the tree
     and the process is discoverable by name.
  3. Induce a crash. Verify the supervisor restarts the worker and the service stays available.
  4. `contrast.md` — actor vs CSP (shared-nothing messaging vs channels; supervision vs explicit
     coordination). Verify the contrast names a concrete trade-off each way.
- **Acceptance criteria**: the `GenServer` manages state correctly; the supervision tree restarts a crashed
  worker with no loss of service; the registry resolves the process; the CSP contrast is concrete.
- **Done bar**: runnable end-to-end (survives an induced crash) + web-verified.

---

← Previous: [43 · Just Enough Elixir](./43-just-enough-elixir.md) · Next: [45 · Just Enough Kotlin](./45-just-enough-kotlin.md) →
