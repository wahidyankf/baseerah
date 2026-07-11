# 33 · Event-Driven Architecture (By Example, Python)

**prd row**: Pass 3 · Build for the Real World · By Example · Python · Learn 133 / Drill 233 · Nvim-ready
Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: designing around events — pub/sub, event sourcing, CQRS, the outbox pattern, dead-letter
queues, idempotent consumers, and sagas for distributed workflows — as runnable Python. The event-driven
_style_ is catalogued in [`30-software-architecture`](./30-software-architecture.md); domain events come
from [`31-domain-driven-design`](./31-domain-driven-design.md); the messaging basics from
[`28-backend-at-scale`](./28-backend-at-scale.md).

## Prerequisites

- **Prior topics**: [topic 19 Concurrency & Parallelism](./19-concurrency-and-parallelism.md) (async
  processing, ordering), [topic 28 Backend at Scale](./28-backend-at-scale.md) (queues, idempotent
  consumers), and [topic 31 Domain-Driven Design](./31-domain-driven-design.md) (domain events).
- **Tools & environment**: a macOS/Linux terminal; **Python 3.x** with a pinned CVE-clean broker client;
  a local broker or stream (Valkey/Redis Streams or an in-process bus is fine); a SQL DB for the outbox.
- **Assumed knowledge**: what a message queue is + why idempotency matters (topic 28); domain events
  (topic 31); async processing (topic 19).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: pattern terminology (event sourcing, CQRS, outbox pattern, choreography-vs-
  orchestration sagas, DLQ, idempotent consumers, at-least-once delivery) is stable and matches current
  industry usage (Microsoft Cloud Design Patterns catalog, Confluent event-driven guides). No corrections.
- 2026-07-12 — verified: the "Valkey/Redis Streams or an in-process bus" hedge is itself accurate —
  Valkey is the Linux Foundation community fork after Redis's 2024 relicensing (SSPL/RSALv2); keep the
  hedge (aligns with DD-21 Tier-1-OSS default).

## Items

- Events vs commands vs messages; pub/sub; event choreography vs orchestration.
- Event sourcing: state as an event log; rebuilding state by replay; snapshots.
- CQRS: separating the write model from read models.
- Reliability patterns: the outbox pattern (atomic write + publish), idempotent consumers, dead-letter
  queues.
- Sagas: managing a distributed workflow with compensating actions.
- Ordering, delivery guarantees (at-least-once), and their consequences.

## Worked examples

Colocated under `event-driven-architecture/learning/code/`; each runnable against a local broker/bus
(DD-20/DD-30).

- **beginner** — a pub/sub example; an idempotent consumer that dedups a redelivered message.
- **intermediate** — an event-sourced aggregate: append events, rebuild state by replay, add a snapshot.
- **advanced** — the outbox pattern (atomic DB write + publish) + a saga with a compensating action + a DLQ
  for poison messages.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: build an event-driven slice of a domain (e.g. order → payment → fulfillment) using event
  sourcing for the write model, a CQRS read model rebuilt from events, the outbox pattern for reliable
  publish, idempotent consumers, a saga with a compensating action, and a dead-letter queue for poison
  messages — all runnable and tested against a local broker.
- **Concepts exercised**: [ ] pub/sub [ ] event sourcing (append + replay + snapshot) [ ] a CQRS read model
  [ ] the outbox pattern [ ] an idempotent consumer [ ] a saga + compensation [ ] a DLQ.
- **Ordered steps**:
  1. `.../learning/capstone/code/` — an event-sourced aggregate: append events + rebuild state by replay.
     Verify replay reconstructs identical state and a snapshot speeds it up.
  2. Add a CQRS read model projected from the event stream. Verify the read model matches the write
     model's state after processing.
  3. Add the outbox pattern (atomic write + relay publish) + an idempotent consumer. Verify a crash between
     write and publish still delivers, and a redelivery processes once.
  4. Add a saga with a compensating action + a DLQ. Verify a downstream failure triggers compensation and a
     poison message lands in the DLQ.
- **Acceptance criteria**: event replay is deterministic; the read model stays consistent; no lost or
  double-applied messages under redelivery; the saga compensates on failure; poison messages reach the DLQ.
- **Done bar**: runnable end-to-end + web-verified.

---

← Previous: [32 · System Design](./32-system-design.md) · Next: [34 · Containers & Orchestration](./34-containers-and-orchestration.md) →
