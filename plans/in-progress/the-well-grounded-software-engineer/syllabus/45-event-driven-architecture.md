# 45 · Event-Driven Architecture (By Example, Python)

**prd row**: Pass 3 · Build for the Real World · By Example · Python · Learn 145 / Drill 245 · Nvim-ready
Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: designing around events — pub/sub, event sourcing, CQRS, the outbox pattern, dead-letter
queues, idempotent consumers, and sagas for distributed workflows — as runnable Python. The event-driven
_style_ is catalogued in [`42-software-architecture`](./42-software-architecture.md); domain events come
from [`43-domain-driven-design`](./43-domain-driven-design.md); the messaging basics from
[`39-backend-at-scale`](./39-backend-at-scale.md).

## Why this exists · the big idea

- **The problem before the solution**: when services call each other synchronously, one slow or down
  dependency stalls the whole chain, and every caller is bound to the callee's availability and shape.
- **Keep-this-if-you-forget-everything**: turn state changes into events others react to — this decouples
  producers from consumers in time and space, but you trade immediate consistency and simple debugging for
  eventual consistency and at-least-once delivery you must design around.
- **Big ideas touched**: `coupling-vs-cohesion` (events decouple producer from consumer), `taming-state`
  (event sourcing makes the append-only log the source of truth), `consistency-latency-throughput`
  (you buy availability and throughput with eventual consistency).

## Prerequisites

- **Prior topics**: [topic 24 Concurrency & Parallelism](./24-concurrency-and-parallelism.md) (async
  processing, ordering), [topic 39 Backend at Scale](./39-backend-at-scale.md) (queues, idempotent
  consumers), and [topic 43 Domain-Driven Design](./43-domain-driven-design.md) (domain events).
- **Tools & environment**: a macOS/Linux terminal; **Python 3.x** with a pinned CVE-clean broker client;
  a local broker or stream (Valkey/Redis Streams or an in-process bus is fine); a SQL DB for the outbox.
- **Assumed knowledge**: what a message queue is + why idempotency matters (topic 39); domain events
  (topic 43); async processing (topic 24).

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
- Broker mechanics contrasted: a log-based broker (Kafka — partitions, offsets, replayable retention) vs a
  queue broker (RabbitMQ — exchanges, bindings, acks), and which delivery/ordering guarantees each gives.

## Tensions & trade-offs — when NOT to reach for this

- **The eventual-consistency tax**: decoupling via events means the read model lags, "did it work?" has no
  synchronous answer, and debugging spans logs across services. A synchronous call is simpler and
  correct-now — use events only when the decoupling is worth losing that.
- **Event sourcing is not free**: replayable logs and audit history are powerful, but schema evolution of
  old events, snapshotting, and rebuild time are real burdens. Most systems want plain state plus a few
  domain events, not full event sourcing.
- **When NOT to use it**: a simple request/response CRUD flow gains nothing from a broker and loses its
  straight-line debuggability. Reach for EDA when you have genuine async workflows, multiple independent
  consumers, or audit/replay needs.

## Lineage — why it beat the alternative

- EDA generalized from message-queue middleware (JMS; the enterprise-integration patterns of Hohpe & Woolf, 2003) and from the CQRS + event-sourcing work (Fowler, Young, ~2010) that answered high-throughput domains
  where the write and read shapes diverged. Kafka (2011) made durable, replayable logs cheap and normalized
  "the log as source of truth." Each step traded synchronous simplicity for decoupling and scale — so adopt
  the piece whose decoupling you actually need. It builds on the domain events of
  [`43-domain-driven-design`](./43-domain-driven-design.md) and the messaging basics of
  [`39-backend-at-scale`](./39-backend-at-scale.md).

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

## Read more

**Books**

- **Designing Event-Driven Systems** — Ben Stopford (2018). Free O'Reilly-published primer connecting Kafka, event sourcing, and CQRS into a coherent architecture story. <https://www.confluent.io/resources/ebook/designing-event-driven-systems/>
- **Building Event-Driven Microservices** — Adam Bellemare (2020). Standard modern treatment of stream-based, event-first microservice architecture.
- **Enterprise Integration Patterns** — Gregor Hohpe, Bobby Woolf (2003). The classic pattern catalog for asynchronous messaging that predates and underlies most event-driven architecture vocabulary.

**Papers & articles**

- **Event Sourcing** — Martin Fowler (2005). The widely cited article that named and popularized the event-sourcing pattern. <https://martinfowler.com/eaaDev/EventSourcing.html>
- **CQRS** — Martin Fowler (2011). Canonical explanation of Command Query Responsibility Segregation and its relationship to event sourcing. <https://martinfowler.com/bliki/CQRS.html>
- **Kafka: a Distributed Messaging System for Log Processing** — Jay Kreps, Neha Narkhede, Jun Rao (2011), NetDB. The original paper describing Kafka's log-centric design, now the reference architecture for event streaming.

---

← Previous: [44 · System Design](./44-system-design.md) · Next: [46 · Distributed Systems](./46-distributed-systems.md) →
