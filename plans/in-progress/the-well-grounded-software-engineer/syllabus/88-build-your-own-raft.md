# 88 · Build Your Own Raft / Replicated KV (By Example, Go †)

**prd row**: Pass 5 · Internals & Lead at Altitude · By Example · Go † · Learn 188 / Drill 288 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: demystify consensus by building it — a Raft implementation (leader election + log
replication) driving a small replicated key-value store, exercised under deliberate failure injection
(dropped messages, partitions, crashed nodes). This is the build-your-own tier of
[`46-distributed-systems`](./46-distributed-systems.md): that topic gave the CAP/consensus intuition; here
you make Raft real. `†`: Go, chosen for its goroutines/channels and `net/rpc` — the concurrency model maps
cleanly onto Raft's timers, RPCs, and per-peer state.

## Why this exists · the big idea

- **The problem before the solution**: keeping several machines agreeing on one ordered history — through
  crashes, delays, and network splits — is deceptively hard, and hand-rolled "just have a leader" schemes
  quietly lose or duplicate data; consensus algorithms exist because the obvious approaches are subtly wrong.
- **Keep-this-if-you-forget-everything**: Raft reduces consensus to an understandable core — elect exactly
  one leader per term, replicate an append-only log to a majority, and only apply an entry once it is safely
  on a quorum. A majority that agrees on a prefix of the log is the whole game.
- **Big ideas touched**: `consistency-latency-throughput` (a write must reach a quorum before it commits —
  that round-trip is the latency price of strong consistency), `determinism-vs-emergence` (correct global
  behaviour — a single consistent log — has to emerge from independent nodes exchanging messages with no
  shared clock).

## Prerequisites

- **Prior topics**: [topic 46 Distributed Systems](./46-distributed-systems.md) (CAP/PACELC, consensus
  intuition, logical clocks, quorums — the theory this topic implements) and
  [topic 60 Just Enough Go](./60-just-enough-go.md) (goroutines, channels, and RPC for the concurrency model).
- **Tools & environment**: a macOS/Linux terminal; **Go** on a recent stable toolchain; the standard library
  (`net/rpc` or gRPC, `testing`, `time`) plus a way to simulate an unreliable network (a message-dropping/
  delaying test harness); Neovim/VSCode with the Go LSP (`gopls`, DD-17).
- **Assumed knowledge**: the consensus problem and why quorums matter (topic 46); Go concurrency —
  goroutines, channels, `select`, timers (topic 60); writing tests that inject failure (topic 15).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: Raft's structure — terms, leader election with randomized election timeouts,
  `AppendEntries`/`RequestVote` RPCs, log matching, and the commit rule (replicate to a majority, then apply)
  — is stable and matches the Ongaro–Ousterhout paper; correctly left version-unpinned. The Go standard
  library surface (`net/rpc`, `testing`, `time`) used here is evergreen.
- 2026-07-12 — verified (SCOPE note for plan owner): implement the core (leader election + log replication +
  a replicated KV state machine on top) and treat log compaction/snapshotting and dynamic membership changes
  as clearly-labelled stretch goals — they are part of full Raft but not needed to demonstrate consensus. The
  MIT 6.5840 labs are a well-trodden reference for exactly this scoping. (raft.github.io; pdos.csail.mit.edu/6.824)

## Items

- The Raft state model: terms, roles (follower/candidate/leader), and per-peer state.
- Leader election: randomized election timeouts, `RequestVote`, and split-vote avoidance.
- Log replication: `AppendEntries`, the log-matching property, and the commit index.
- Applying to a state machine: driving a replicated key-value store from the committed log.
- Failure injection: dropped/delayed messages, partitions, and crashed/restarted nodes — proving safety and
  liveness.
- Persistence & stretch: persisting term/vote/log across restarts; snapshotting and membership change as
  labelled extensions.

## Worked examples

Colocated under `build-your-own-raft/learning/code/`; Go (`go test`) with a message-dropping/partitioning
test harness (DD-20/DD-30). Correctness is proven under injected failure, not just the happy path.

- **beginner** — leader election among 3–5 nodes: exactly one leader per term, and a new election after the
  leader is killed; verify under `go test`.
- **intermediate** — log replication: a client write reaches a quorum and commits; a lagging follower catches
  up; verify logs converge.
- **advanced** — drive a replicated KV store through partitions and node restarts: verify no committed write
  is lost and all live nodes converge to one consistent log.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: build a working Raft — leader election, log replication, and persistence — that drives a
  replicated key-value store across a cluster of nodes, and prove it keeps a single consistent, durable log
  under dropped messages, partitions, and node restarts, all exercised by `go test` with a failure-injecting
  harness.
- **Concepts exercised**: [ ] terms + roles + per-peer state [ ] leader election with randomized timeouts
  [ ] `AppendEntries` log replication + commit rule [ ] a replicated KV state machine [ ] persistence across
  restarts [ ] failure injection (drops/partitions/crashes) [ ] `go test` coverage of safety + liveness.
- **Ordered steps**:
  1. `.../learning/capstone/code/raft/` — the Raft node with terms, roles, and `RequestVote`/`AppendEntries`
     RPCs; a test harness that can drop/delay/partition messages. Verify exactly one leader is elected per
     term and a re-election follows a leader crash (`go test`).
  2. Implement log replication + the commit index and persist term/vote/log. Verify a committed entry reaches
     a quorum, a lagging follower catches up, and state survives a restart (`go test`).
  3. `.../kv/` — a replicated key-value store applying the committed log. Verify client reads/writes are
     linearizable under partitions and restarts, and all live nodes converge to one log (`go test`).
- **Acceptance criteria**: one leader per term; committed entries never lost or reordered; followers converge;
  the KV store stays consistent under injected failures; persisted state survives restart; `go test` covers
  safety and liveness.
- **Done bar**: runnable end-to-end + correct under failure injection + tests green + web-verified.

## Read more

**Books**

- **Designing Data-Intensive Applications** — Martin Kleppmann (2017). Provides the replication and consensus
  context that underpins Raft-based systems.

**Papers & articles**

- **In Search of an Understandable Consensus Algorithm (Extended Version)** — Diego Ongaro, John Ousterhout
  (2014). THE Raft paper; the primary source for the algorithm. <https://raft.github.io/raft.pdf>
- **Consensus: Bridging Theory and Practice** — Diego Ongaro (PhD dissertation, 2014). The full formal
  treatment and proofs behind Raft, by its co-creator, including snapshotting and membership change.
  <https://web.stanford.edu/~ouster/cgi-bin/papers/OngaroPhD.pdf>
- **Paxos Made Simple** — Leslie Lamport (2001). The canonical predecessor consensus paper that motivates why
  Raft was designed for understandability. <https://www.microsoft.com/en-us/research/publication/paxos-made-simple/>
- **The Raft Consensus Algorithm** — Diego Ongaro et al. Official companion site with an interactive
  visualization widely used to build intuition before implementing Raft. <https://raft.github.io/>
- **MIT 6.5840 (6.824) Distributed Systems** — MIT PDOS. Free graduate course whose labs have students
  implement Raft and a replicated KV store from scratch. <https://pdos.csail.mit.edu/6.824/>

---

← Previous: [87 · Build Your Own Database](./87-build-your-own-database.md) · Next: [89 · Platform Engineering & Developer Experience](./89-platform-engineering-and-devex.md) →
