# 35 · Graph Databases (By Example, Cypher + Python †)

**prd row**: Pass 3 · Build for the Real World · By Example · Cypher + Python † · Learn 135 / Drill 235 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the property-graph model and Cypher — nodes/relationships/properties, traversal, and the
problems where a graph beats relational (recommendations, fraud, knowledge graphs) — accessed from Python.
`†`: Cypher is the query language (note GQL = ISO/IEC 39075:2024). Sits beside the other non-relational
families in [`34-nosql-databases`](./34-nosql-databases.md).

## Why this exists · the big idea

- **The problem before the solution**: when the relationships between entities are the real question
  (who-knows-whom, paths, recommendations), a relational store answers with join explosions that get
  exponentially slower with each hop.
- **Keep-this-if-you-forget-everything**: when connections are first-class data, model them as first-class —
  a graph makes a k-hop traversal cost roughly k, where the relational equivalent multiplies.
- **Big ideas touched**: `consistency-latency-throughput` (deep-traversal performance is the win),
  `coupling-vs-cohesion` (the domain here _is_ connectedness itself).

## Prerequisites

- **Prior topics**: [topic 10 SQL Essentials](./10-sql-essentials.md) (the relational model to contrast),
  [topic 34 NoSQL Databases](./34-nosql-databases.md) (the non-relational framing), and
  [topic 4 Just Enough Python](./04-just-enough-python.md).
- **Tools & environment**: a macOS/Linux terminal; a local **Neo4j** (or GQL-compatible) instance (Docker
  fine; check the edition license); **Python 3.x** with a pinned CVE-clean driver; the Cypher shell.
- **Assumed knowledge**: relational joins (to feel the contrast); basic Python driver use; the idea of
  entities and relationships.

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: **GQL = ISO/IEC 39075:2024** (published 2024-04-12, first new ISO DB query-language
  standard since SQL/1987, Cypher-inspired, SQL's sibling). **Neo4j Community Edition = GPLv3** (not
  AGPLv3 — confirmed against the official `neo4j/neo4j` repo; secondary sources conflate this with Enterprise
  Edition's AGPLv3 history). (iso.org/standard/76120 / github.com/neo4j/neo4j)
- 2026-07-12 — verified (CORRECTION, version-sensitive): Neo4j moved to **calendar versioning** (2025.x,
  2026.x) and now ships **two parallel Cypher versions** — **Cypher 5** (frozen, bug-fixes only) and
  **Cypher 25** (evolving; default for new databases from Neo4j 2026.02). Content must state which Cypher
  version its examples target rather than assuming a single unversioned "Cypher." (neo4j.com/docs/cypher-manual)

## Items

- The property-graph model: nodes, relationships, properties, labels.
- Cypher: `MATCH`/`CREATE`/`MERGE`, patterns, `WHERE`, `RETURN`; note GQL = ISO/IEC 39075:2024.
- Graph traversal & queries: variable-length paths, shortest path, neighborhoods.
- When a graph DB beats relational: deeply connected data — recommendations, fraud, knowledge graphs.
- Access from Python: a Neo4j-style driver; loading & querying.
- Modeling: graph vs relational for the same problem.

## Worked examples

Colocated under `graph-databases/learning/code/`; Cypher + a Python driver, each runnable (DD-20/DD-30).

- **beginner** — create nodes/relationships and query them with Cypher.
- **intermediate** — a variable-length traversal (friends-of-friends); a shortest-path query.
- **advanced** — model a recommendation / knowledge-graph domain and contrast the relational version.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: build a small recommendation/knowledge-graph over a property graph — load a domain (people +
  items + relationships), answer real graph questions (neighborhoods, variable-length paths, shortest
  path, a recommendation), from Python — and contrast the equivalent relational query to show why the
  graph wins.
- **Concepts exercised**: [ ] property-graph modeling (nodes/rels/labels) [ ] Cypher `MATCH`/`MERGE`
  [ ] a variable-length traversal [ ] a shortest-path query [ ] a recommendation query [ ] a graph-vs-SQL
  contrast.
- **Ordered steps**:
  1. `.../learning/capstone/code/load.py` — load the domain via Cypher `MERGE`. Verify node/relationship
     counts match the dataset.
  2. `queries.cypher` + `run.py` — neighborhood + friends-of-friends (variable-length) queries. Verify
     results match a hand-checked small case.
  3. `recommend.py` — a "people who X also Y" recommendation + a shortest-path query. Verify sensible,
     verifiable output.
  4. `contrast.md` — the equivalent relational query with its join explosion; explain the graph advantage.
     Verify the contrast is concrete (query text + why).
- **Acceptance criteria**: the graph loads correctly; every query returns verifiable results; the
  recommendation is sensible; the relational contrast is concrete and justified.
- **Done bar**: runnable end-to-end + web-verified.

## Read more

**Books**

- **Graph Databases: New Opportunities for Connected Data** — Ian Robinson, Jim Webber, Emil Eifrem (2015, 2nd ed.). Free, canonical introduction to property graph modeling and the graph database landscape. <https://graphdatabases.com/>
- **Graph Algorithms: Practical Examples in Apache Spark and Neo4j** — Mark Needham & Amy E. Hodler (2019). Standard reference on classic graph algorithms — PageRank, community detection, centrality — applied to graph databases.

**Papers & articles**

- **Cypher: An Evolving Query Language for Property Graphs** — Nadime Francis, Alastair Green, Paolo Guagliardo, Leonid Libkin, et al. (2018). Paper formalizing the semantics of Cypher, the query language later standardized as part of ISO GQL. <https://dl.acm.org/doi/10.1145/3183713.3190657>
- **Apache TinkerPop / Gremlin Reference Documentation** — Apache Software Foundation (continually maintained). The canonical specification and reference for the Gremlin graph traversal language used across many graph databases. <https://tinkerpop.apache.org/docs/current/reference/>

---

← Previous: [34 · NoSQL Databases](./34-nosql-databases.md) · Next: [36 · Database Internals & Storage Engines](./36-database-internals-and-storage-engines.md) →
