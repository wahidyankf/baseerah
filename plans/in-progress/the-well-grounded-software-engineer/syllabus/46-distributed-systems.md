# 46 · Distributed Systems (By Example, Python †)

**prd row**: Pass 3 · Build for the Real World · By Example · Python † · Learn 146 / Drill 246 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: why distributed systems are hard — CAP/PACELC, consensus (Paxos/Raft intuition),
logical clocks, replication, quorums, and CRDTs — taught as the failure modes that appear the moment
one machine becomes many. The point is judgment about trade-offs, not a from-scratch consensus
engine; that build lives in [`88-build-your-own-raft`](./88-build-your-own-raft.md). `†`: Python,
fully type-annotated (DD-34) — every snippet carries type hints in the mypy-clean spirit.

## Why this exists · the big idea

- **The problem before the solution**: on one machine, a write either happens or it doesn't. Add a
  network and everything you assumed breaks — messages arrive late, out of order, twice, or never; a
  node that looks dead is just slow; and no one has a shared "now". Reasoning that was trivial in
  process becomes a minefield of partial failure.
- **Keep-this-if-you-forget-everything**: in a distributed system the network is unreliable and there
  is no global clock, so you cannot have perfect consistency, availability, and partition tolerance at
  once — you choose which guarantee to relax, on purpose, per operation.
- **Big ideas touched**: `consistency-latency-throughput` (CAP/PACELC is exactly this trilemma —
  under a partition choose consistency or availability, and even without one, consistency costs
  latency), `determinism-vs-emergence` (correct global behaviour — agreement, ordering, convergence —
  has to _emerge_ from unreliable local message-passing, since no node sees the whole truth).

## Prerequisites

- **Prior topics**: [topic 12 Networking Essentials](./12-networking-essentials.md) (packets, latency,
  timeouts, why the network lies) and [topic 44 System Design](./44-system-design.md) (replication,
  partitioning, and the scaling context these guarantees serve).
- **Tools & environment**: a macOS/Linux terminal; **Python** at a recent stable release with type
  hints and `mypy`; the ability to run several communicating processes locally (asyncio or multiple
  processes) with simulated message delay/loss/reordering; Neovim/VSCode with the Python LSP (DD-17).
- **Assumed knowledge**: timeouts, retries, and network latency (topic 12); replication and sharding
  at a design level (topic 44); running concurrent tasks in Python (topics 04/24).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: the theory here is settled and correctly version-unpinned — the CAP theorem
  (Gilbert–Lynch proof), Lamport's logical/vector clocks, Paxos, and **Raft** as the most widely
  implemented production consensus algorithm are all stable, foundational results, not moving targets.
- 2026-07-12 — verified: **PACELC** (Abadi) is correctly presented as the refinement of CAP that also
  accounts for the latency-vs-consistency trade-off in the _absence_ of a partition; keep CAP and
  PACELC together rather than treating CAP alone as complete. CRDTs are stable as a convergence
  strategy for AP designs.

## Items

- The eight fallacies and the two hard facts: the network is unreliable and there is no global clock —
  everything else follows.
- CAP and PACELC: what a partition forces you to give up, and the latency-vs-consistency choice even
  when the network is healthy.
- Time without a clock: logical clocks and vector clocks; the happens-before relation and causal
  ordering.
- Replication and quorums: leader-based vs leaderless, read/write quorums (R + W > N), and read-repair.
- Consensus intuition: what Paxos guarantees and why Raft (leader election + log replication)
  reformulated it to be understandable and implementable.
- Convergence without coordination: CRDTs and eventual consistency for the AP side of the trade-off.

## Tensions & trade-offs — when NOT to reach for this

- **Distribution is a cost, not a feature**: consensus, quorums, and replication add latency, failure
  modes, and operational burden. A single well-backed-up node with a fast restore is simpler and often
  correct — reach for a distributed protocol only when the availability or scale requirement genuinely
  forces it.
- **Strong consistency isn't free and isn't always needed**: linearizable consensus costs a round trip
  and stalls under partition; many workloads are fine with causal or eventual consistency and a CRDT.
  Paying for strong consistency where the domain tolerates staleness is latency you burn for nothing.
- **Rolling your own consensus is a trap in production**: the algorithms are subtle and the failure
  cases are adversarial. Build one to _understand_ it (topic 88), but in production adopt a proven
  implementation (etcd/Consul/a database that embeds Raft) rather than hand-writing the protocol.

## Lineage — why it beat the alternative

- Distributed-systems theory grew from trying to make many unreliable machines behave like one
  reliable one. Lamport's 1978 logical clocks gave ordering without a shared clock; the CAP theorem
  (conjectured by Brewer, proved by Gilbert and Lynch) named the fundamental trade-off; Paxos proved
  consensus was possible but was famously hard to understand, so Raft (2014) re-expressed the same
  guarantees around an understandable leader-and-log model — which is why etcd, Consul, and
  CockroachDB adopted it. The winner wasn't a single protocol but the discipline of choosing your
  consistency model per operation. This feeds the hands-on build in
  [`88-build-your-own-raft`](./88-build-your-own-raft.md) and the scaling context in
  [topic 44 System Design](./44-system-design.md).

## Worked examples

Colocated under `distributed-systems/learning/code/`; each runnable as several communicating local
processes/tasks with injectable delay/loss, every Python snippet fully type-annotated and `mypy`-clean
(DD-20/DD-30/DD-34).

- **beginner** — implement a Lamport logical clock and a vector clock across a few message-passing
  processes; show two events' causal vs concurrent relationship.
- **intermediate** — a leaderless quorum key-value store (R + W > N) with read-repair; demonstrate a
  stale read when a quorum is not met and its repair when it is.
- **advanced** — a Raft-style leader election over a simulated lossy network (heartbeats, timeouts,
  term voting); inject a partition and observe re-election and log convergence.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: build a small replicated key-value store over a simulated unreliable network that
  demonstrates the trade-off explicitly — a quorum-based (AP-leaning) mode with read-repair and a
  leader-elected (CP-leaning) mode — with injectable delay, loss, and partition, and a test that
  proves the consistency behaviour of each mode.
- **Concepts exercised**: [ ] logical/vector clocks [ ] read/write quorums (R + W > N) [ ] read-repair
  [ ] leader election [ ] partition injection [ ] a consistency-behaviour assertion per mode.
- **Ordered steps**:
  1. `.../learning/capstone/code/clocks.py` — vector clocks tagging every write. Verify concurrent vs
     causally-ordered writes are correctly classified; `mypy` clean.
  2. `.../learning/capstone/code/quorum.py` — a leaderless quorum store with read-repair. Verify that
     `W + R > N` yields the latest value and that a sub-quorum can observe a stale read.
  3. `.../learning/capstone/code/raft.py` — leader election + log replication. Verify a single leader
     is elected per term and followers converge on the leader's log.
  4. `.../learning/capstone/code/partition_test.py` — inject a partition against both modes. Verify the
     CP mode blocks/loses availability while the AP mode stays available but may diverge then converge.
- **Acceptance criteria**: clocks classify causality correctly; quorum reads/writes obey R + W > N;
  leader election is stable; the partition test demonstrates each mode's advertised consistency/
  availability behaviour; all Python is type-annotated and `mypy`-clean.
- **Done bar**: runnable end-to-end + web-verified.

## Read more

**Books**

- **Designing Data-Intensive Applications** — Martin Kleppmann (2017). Arguably the single most cited
  modern reference for replication, consistency, and partitioning trade-offs.
- **Distributed Systems: Principles and Paradigms** — Andrew S. Tanenbaum, Maarten van Steen (2nd ed.,
  2007). Long-standing academic textbook covering the core theory of distributed systems.

**Papers & articles**

- **Time, Clocks, and the Ordering of Events in a Distributed System** — Leslie Lamport (1978). The
  foundational paper introducing logical clocks and the happened-before relation.
  <https://lamport.azurewebsites.net/pubs/time-clocks.pdf>
- **Paxos Made Simple** — Leslie Lamport (2001). The clearest author-written explanation of the Paxos
  consensus algorithm. <https://lamport.azurewebsites.net/pubs/paxos-simple.pdf>
- **In Search of an Understandable Consensus Algorithm (Extended Version)** — Diego Ongaro, John
  Ousterhout (2014), USENIX ATC. Introduced Raft, now the most widely implemented consensus algorithm
  in production systems (etcd, Consul, CockroachDB). <https://raft.github.io/raft.pdf>
- **CAP Twelve Years Later: How the "Rules" Have Changed** — Eric Brewer (2012), IEEE Computer.
  Brewer's own retrospective clarifying and correcting common misreadings of the CAP theorem.

---

← Previous: [45 · Event-Driven Architecture](./45-event-driven-architecture.md) · Next: [47 · Advanced Frontend](./47-advanced-frontend.md) →
