# 26 · NoSQL Databases (By Example, Python †)

**prd row**: Pass 3 · Build for the Real World · By Example · Python † · Learn 126 / Drill 226 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the non-relational families — key-value, document, wide-column — when to pick each, how to
model access-pattern-first, and the CAP/PACELC trade-offs, accessed from Python. Graph databases are their
own topic ([`27-graph-databases`](./27-graph-databases.md)). License-awareness (DD-15) is treated as a
real engineering step. Relational depth is [`23-advanced-sql-and-query-performance`](./23-advanced-sql-and-query-performance.md).

## Prerequisites

- **Prior topics**: [topic 08 SQL Essentials](./08-sql-essentials.md) (the relational model these contrast
  against) and [topic 04 Just Enough Python](./04-just-enough-python.md);
  [topic 23 Advanced SQL](./23-advanced-sql-and-query-performance.md) sharpens the modeling contrast.
- **Tools & environment**: a macOS/Linux terminal; **Python 3.x** with pinned CVE-clean drivers; local
  instances (Docker fine) of **Valkey** (BSD) or Redis, a document store (MongoDB — note SSPL), and a
  wide-column store (Cassandra — Apache-2.0); each product's **license** checked before use.
- **Assumed knowledge**: relational schema + CRUD (topic 08); running a local service; basic Python
  driver use.

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified (all five license claims confirmed against primary sources): **Redis** — tri-license
  RSALv2/SSPLv1/**AGPLv3** since Redis 8 (2025-05-01, re-adding an OSI-approved option); **Valkey** —
  **BSD-3-Clause** (Linux Foundation fork, March 2024); **MongoDB** — **SSPLv1** (unchanged since Oct 2018,
  not OSI-approved); **Cassandra** — **Apache-2.0** (unchanged); **ScyllaDB** — **source-available**
  (Enterprise 2025.1+; Open Source 6.2.x is the final AGPL release). CAP (Brewer 2000) + PACELC (Abadi 2010)
  framing stable. (redis.io/blog/agplv3 / valkey.io / mongodb.com / cassandra.apache.org / scylladb.com)

## Items

- NoSQL families: key-value, document, wide-column (graph is its own topic); when to pick each.
- Modeling for NoSQL: denormalization, aggregates, access-pattern-first design.
- CAP / PACELC trade-offs; eventual consistency; tunable consistency levels.
- Hands-on stores with license-awareness (DD-15): Redis (AGPLv3 since Redis 8) / Valkey (BSD fork); a
  document store — MongoDB (SSPL); wide-column — Cassandra (Apache-2.0) / ScyllaDB (source-available).
  "Read the license" is treated as a real engineering step.
- Access from Python: drivers, CRUD, indexing in a document store.

## Worked examples

Colocated under `nosql-databases/learning/code/`; each runnable from Python against a local store
(DD-20/DD-30).

- **beginner** — key-value CRUD (Redis/Valkey) from Python; a document insert + query.
- **intermediate** — model the same domain relationally vs as documents; an access-pattern-driven schema.
- **advanced** — a wide-column data model; a tunable-consistency trade-off demonstrated.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: model one domain across the NoSQL families — a key-value cache/session store (Valkey), a
  document model driven by real access patterns (MongoDB), and a wide-column table (Cassandra) — from
  Python, with a written CAP/PACELC + license rationale for each choice.
- **Concepts exercised**: [ ] key-value CRUD [ ] access-pattern-first document modeling [ ] a wide-column
  data model [ ] a CAP/PACELC trade-off stated per store [ ] a license check recorded per store.
- **Ordered steps**:
  1. `.../learning/capstone/code/kv.py` — session/cache CRUD against Valkey/Redis. Verify set/get/expire
     round-trips from the CLI.
  2. `doc.py` — a document schema shaped by two named access patterns + an index. Verify each query is
     index-served and returns expected data.
  3. `wide.py` — a wide-column model (partition + clustering keys) for a time-series/feed access pattern.
     Verify a partition query returns ordered rows.
  4. `rationale.md` — per store, state the CAP/PACELC position and the license (with the actual license
     name). Verify each choice is justified by the access pattern.
- **Acceptance criteria**: all three stores are exercised from Python with correct results; each modeling
  choice is justified by an access pattern; the license of each product is named and checked.
- **Done bar**: runnable end-to-end + web-verified.

---

← Previous: [25 · Project Management](./25-project-management.md) · Next: [27 · Graph Databases](./27-graph-databases.md) →
