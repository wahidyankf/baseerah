# 63 · Actor-Model Concurrency (By Example, Elixir †)

**prd row**: Pass 4 · Concurrency & Systems · By Example · Elixir † · Learn 163 / Drill 263 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the actor concurrency model on the BEAM — processes, message passing, mailboxes,
`GenServer` for stateful processes, supervision trees + "let it crash", OTP applications — and the explicit
contrast with CSP ([`61-csp-style-concurrency`](./61-csp-style-concurrency.md)). License-aware (DD-15).

## Why this exists · the big idea

- **The problem before the solution**: one unhandled error can corrupt shared state and take down a whole
  system — the actor model isolates state inside shared-nothing processes and supervises them so a failure
  is contained, not catastrophic.
- **Keep-this-if-you-forget-everything**: "let it crash" — don't defensively guard every process; isolate
  state per actor and let a supervisor restart a failed one back to a known-good state.
- **Big ideas touched**: `taming-state` — each actor owns its state privately, reachable only by message,
  so there is nothing to share and nothing to corrupt; `determinism-vs-emergence` — system reliability
  emerges from supervision trees and restart strategies, not from any single process being perfect.

## Prerequisites

- **Prior topics**: [topic 62 Just Enough Elixir](./62-just-enough-elixir.md) (the language + a process
  preview), [topic 61 CSP-Style Concurrency](./61-csp-style-concurrency.md) (the model to contrast), and
  [topic 24 Concurrency & Parallelism](./24-concurrency-and-parallelism.md).
- **Tools & environment**: a macOS/Linux terminal; **Elixir/OTP** + `mix` + `iex`, pinned to a current
  stable release; Neovim/VSCode (DD-17).
- **Assumed knowledge**: Elixir syntax + a `spawn`/`send`/`receive` preview (topic 62); channels/CSP for the
  contrast (topic 61).

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

## Read more

**Books**

- **Programming Erlang: Software for a Concurrent World**, 2nd ed. — Joe Armstrong (2013, Pragmatic Bookshelf). The canonical Erlang/actor-model text by one of Erlang's creators.
- **Designing for Scalability with Erlang/OTP** — Francesco Cesarini & Steve Vinoski (2016, O'Reilly). The standard reference on building fault-tolerant, supervision-tree-based systems in production.

**Papers & articles**

- **A Universal Modular ACTOR Formalism for Artificial Intelligence** — Carl Hewitt, Peter Bishop, Richard Steiger, _Proc. 3rd IJCAI_ (1973). The founding paper that introduced the actor model. <http://ijcai.org/Proceedings/73/Papers/027B.pdf>
- **Making Reliable Distributed Systems in the Presence of Software Errors** — Joe Armstrong, PhD thesis, Royal Institute of Technology (2003). Defines OTP's "let it crash" philosophy and supervision trees; officially hosted by erlang.org. <https://erlang.org/download/armstrong_thesis_2003.pdf>
- **OTP Design Principles** — Erlang/OTP official System Documentation. The authoritative reference for workers, supervisors, and supervision trees. <https://www.erlang.org/doc/system/design_principles.html>

---

← Previous: [62 · Just Enough Elixir](./62-just-enough-elixir.md) · Next: [64 · Just Enough Kotlin](./64-just-enough-kotlin.md) →
