# 44 · System Design (Annotated-concept, Python \*)

**prd row**: Pass 3 · Build for the Real World · Annotated-concept · Python \* · Learn 144 / Drill 244 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: designing systems at scale — the building blocks (load balancing, caching, sharding,
replication, queues, CDNs), the estimation/trade-off method, and worked case studies (URL shortener,
rate limiter, news feed). `*`: Python where a component is demonstrated runnably (e.g. a rate limiter,
a consistent-hashing ring), else annotated architecture diagrams. Single-service scaling depth is
[`39-backend-at-scale`](./39-backend-at-scale.md).

## Why this exists · the big idea

- **The problem before the solution**: "build a system that handles millions of users" has no single right
  answer — every building block (cache, shard, queue, replica) relieves one bottleneck by creating another,
  and the skill is choosing under uncertainty.
- **Keep-this-if-you-forget-everything**: start from the numbers — estimate load, find the bottleneck, reach
  for the specific block that relieves it, and say out loud what each choice gives up. Design is trade-off,
  not a checklist of components.
- **Big ideas touched**: `consistency-latency-throughput` (the axes every decision moves along),
  `correctness-vs-pragmatism` (capacity estimation is deliberate approximation, not precision),
  `abstraction-and-its-cost` (every building block buys scale and charges operational complexity).

## Prerequisites

- **Prior topics**: [topic 39 Backend at Scale](./39-backend-at-scale.md) (services, queues, caching),
  [topic 29 Advanced Networking](./29-advanced-networking.md) (latency, DNS, load balancing), and
  [topic 26 Advanced SQL](./26-advanced-sql-and-query-performance.md) (indexes, replication, sharding).
- **Tools & environment**: a macOS/Linux terminal; **Python 3.x** for the runnable component demos; a
  Markdown/Mermaid editor for the architecture diagrams + capacity estimates (Neovim per DD-17).
- **Assumed knowledge**: how a single service scales (topic 39); back-of-the-envelope arithmetic; reading a
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

## Tensions & trade-offs — when NOT to reach for this

- **Estimation is a bet, not a prediction**: back-of-envelope numbers guide the design but are wrong by
  design. Treating them as precise (over-provisioning for imagined scale) wastes money and complexity;
  ignoring them entirely designs blind. Estimate to _find the bottleneck_, not to forecast the future.
- **Every block cuts both ways**: a cache adds staleness, a shard adds cross-shard queries and rebalancing,
  a queue adds eventual consistency and ordering headaches, a replica adds replication lag. No building
  block only helps.
- **When NOT to scale**: a single well-tuned Postgres serves further than most designs admit. Reach for
  sharding, a CDN, or multi-region when a _measured_ limit forces it, not because the diagram looks bigger.

## Lineage — why it beat the alternative

- The system-design canon crystallized when the big web companies published how they scaled — Dynamo (2007;
  eventual consistency + consistent hashing), MapReduce, and the CAP theorem (Brewer 2000) formalizing that
  you cannot have consistency, availability, and partition tolerance all at once. It became interview ritual
  because it compresses decades of scaling scars into a repeatable method: numbers → bottleneck → trade-off.
  The durable lesson isn't the specific blocks but the discipline of making the trade-off explicit — the same
  judgment [`39-backend-at-scale`](./39-backend-at-scale.md) applies to one service and
  [`42-software-architecture`](./42-software-architecture.md) applies to boundaries.

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

## Read more

**Books**

- **Designing Data-Intensive Applications** — Martin Kleppmann (2017). The definitive modern text connecting distributed systems theory to practical large-scale system design.
- **System Design Interview: An Insider's Guide** — Alex Xu (2020). The most widely used practical primer for the system-design-interview canon.
- **Web Scalability for Startup Engineers** — Artur Ejsmont (2015). Practical treatment of scaling patterns (load balancing, caching, sharding) for growing systems.

**Papers & articles**

- **MapReduce: Simplified Data Processing on Large Clusters** — Jeffrey Dean, Sanjay Ghemawat (2004), OSDI. The paper that popularized the batch-processing model underlying much of large-scale system design. <https://research.google/pubs/mapreduce-simplified-data-processing-on-large-clusters/>
- **The Google File System** — Sanjay Ghemawat, Howard Gobioff, Shun-Tak Leung (2003), SOSP. Canonical paper describing the distributed storage design that inspired HDFS and much of big-data infrastructure. <https://research.google/pubs/the-google-file-system/>
- **Dynamo: Amazon's Highly Available Key-value Store** — Giuseppe DeCandia et al. (2007), SOSP. Foundational paper behind eventually-consistent, partitioned key-value stores (Cassandra, Riak, DynamoDB). <https://www.allthingsdistributed.com/files/amazon-dynamo-sosp2007.pdf>

---

← Previous: [43 · Domain-Driven Design](./43-domain-driven-design.md) · Next: [45 · Event-Driven Architecture](./45-event-driven-architecture.md) →
