# 32 · System Design (Annotated-concept, Python \*)

**prd row**: Pass 3 · Build for the Real World · Annotated-concept · Python \* · Learn 132 / Drill 232 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: designing systems at scale — the building blocks (load balancing, caching, sharding,
replication, queues, CDNs), the estimation/trade-off method, and worked case studies (URL shortener,
rate limiter, news feed). `*`: Python where a component is demonstrated runnably (e.g. a rate limiter,
a consistent-hashing ring), else annotated architecture diagrams. Single-service scaling depth is
[`28-backend-at-scale`](./28-backend-at-scale.md).

## Prerequisites

- **Prior topics**: [topic 28 Backend at Scale](./28-backend-at-scale.md) (services, queues, caching),
  [topic 21 Advanced Networking](./21-advanced-networking.md) (latency, DNS, load balancing), and
  [topic 23 Advanced SQL](./23-advanced-sql-and-query-performance.md) (indexes, replication, sharding).
- **Tools & environment**: a macOS/Linux terminal; **Python 3.x** for the runnable component demos; a
  Markdown/Mermaid editor for the architecture diagrams + capacity estimates (Neovim per DD-17).
- **Assumed knowledge**: how a single service scales (topic 28); back-of-the-envelope arithmetic; reading a
  latency/throughput number.

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: no version/license-sensitive claims here. Capacity-estimation numbers (latency
  ladder, QPS/storage rules of thumb), consistent hashing, token-bucket/sliding-window rate limiting, and
  the canonical case studies (URL shortener, news feed) are evergreen CS fundamentals unchanged in years —
  nothing time-sensitive to correct.

## Items

- The method: requirements → capacity estimation → API → data model → high-level design → bottlenecks →
  trade-offs.
- Building blocks: load balancing, caching layers, sharding/partitioning, replication, message queues,
  CDNs.
- Consistency & availability: CAP/PACELC in system terms; leader/follower; quorum.
- Reliability at scale: redundancy, failover, graceful degradation, backpressure.
- Case studies (annotated + partly runnable): URL shortener, rate limiter, news feed.
- Communicating a design: diagrams, capacity numbers, explicit trade-offs.

## Worked examples

Colocated under `system-design/learning/`; annotated Mermaid architecture + runnable Python component demos
(DD-20/DD-30).

- **rate-limiter** — a token-bucket / sliding-window rate limiter in Python, tested for its limit behavior.
- **consistent-hashing** — a consistent-hashing ring demonstrating minimal key movement on node add/remove.
- **url-shortener-design** — an annotated end-to-end design (API + data model + capacity estimate +
  bottleneck analysis).

## Capstone spec — intra-topic (subject → design artifact + runnable components)

- **Goal**: produce a complete system design for one non-trivial system (e.g. a news feed or URL
  shortener) — requirements, capacity estimation, API, data model, a high-level architecture diagram, and
  a trade-off/bottleneck analysis — and back it with two runnable Python components (a rate limiter and a
  consistent-hashing ring) that prove the load-shedding and partitioning mechanics.
- **Concepts exercised**: [ ] capacity estimation (QPS/storage/bandwidth) [ ] API + data model design
  [ ] a high-level architecture diagram [ ] a runnable rate limiter [ ] a runnable consistent-hashing ring
  [ ] an explicit trade-off/bottleneck analysis.
- **Ordered steps**:
  1. `.../learning/capstone/design.md` — requirements + capacity estimate + API + data model + a Mermaid
     architecture. Verify the capacity numbers are arithmetic-checked and the diagram matches the API.
  2. `.../learning/capstone/code/rate_limiter.py` — a token-bucket limiter with tests. Verify it admits up
     to the limit and rejects beyond it.
  3. `.../learning/capstone/code/hashing.py` — a consistent-hashing ring with tests. Verify adding/removing
     a node moves only a bounded fraction of keys.
  4. `design.md` trade-off section — bottlenecks + failure modes + graceful degradation. Verify each
     trade-off names what is gained and what is given up.
- **Acceptance criteria**: the design has checked capacity numbers, a coherent API + data model + diagram,
  two runnable components with passing tests, and an explicit trade-off analysis.
- **Done bar**: design artifact complete + components runnable + web-verified.

---

← Previous: [31 · Domain-Driven Design](./31-domain-driven-design.md) · Next: [33 · Event-Driven Architecture](./33-event-driven-architecture.md) →
