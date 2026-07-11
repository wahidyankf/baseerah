# 23 · Advanced SQL & Query Performance (By Example, SQL + Python †)

**prd row**: Pass 2 · Solidify the Core · By Example · SQL + Python † (PostgreSQL) · Learn 123 / Drill 223 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: deep SQL and the performance engineering around it — advanced query features, ACID and
isolation internals, indexing, query planning (`EXPLAIN`), and the N+1/denormalization/partitioning
trade-offs. Basics are the prerequisite [`08-sql-essentials`](./08-sql-essentials.md); PostgreSQL is the
teaching engine (`†` platform-mandated for `EXPLAIN ANALYZE`/MVCC realism).

## Prerequisites

- **Prior topics**: [topic 08 SQL Essentials](./08-sql-essentials.md) (schema, CRUD, joins, transactions,
  parameterized queries) and [topic 04 Just Enough Python](./04-just-enough-python.md) for the DAL side;
  [topic 09 Backend Essentials](./09-backend-essentials.md) provides the N+1 scenario.
- **Tools & environment**: a macOS/Linux terminal; a local **PostgreSQL** (pinned, CVE-clean); the `psql`
  CLI; **Python 3.x** with a pinned driver for the N+1 example; a seed dataset large enough for `EXPLAIN`
  to matter.
- **Assumed knowledge**: writing `SELECT`/`JOIN`/`INSERT`, transactions, and a parameterized query from
  topic 08; reading a table schema.

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28). **Re-check the PG major at authoring.**

- 2026-07-12 — verified (CORRECTION, version-sensitive): latest stable is **PostgreSQL 18** (18.4);
  **PG 19 Beta 1** released 2026-06-04, GA targeted Sept 2026 — pin content to PG 18 (or 19 if GA lands
  first). **PG 18 changed `EXPLAIN ANALYZE`**: buffer stats now show **automatically by default** (explicit
  `BUFFERS` no longer required; restore old behavior with `EXPLAIN (ANALYZE, BUFFERS OFF)`), and
  `EXPLAIN ... VERBOSE` gained WAL/CPU/per-row-average stats. Reflect this in the body. (postgresql.org
  news / neon.com/postgresql/postgresql-18)
- 2026-07-12 — verified: window functions, recursive CTEs (`WITH RECURSIVE`), set operations, and
  MVCC isolation-level behavior (Read Committed default, Repeatable Read, Serializable via SSI) are stable
  unchanged across recent PostgreSQL releases. (postgresql.org/docs/current)

## Items

- Advanced SQL: subqueries, CTEs (incl. recursive), window functions, set operations.
- Transactions & ACID deep; isolation levels; locking & MVCC.
- Indexing: B-tree/hash indexes, composite & covering indexes, when indexes hurt.
- Query planning: `EXPLAIN` / `EXPLAIN ANALYZE`, reading a plan, table statistics.
- Performance: N+1 diagnosis & fix, denormalization trade-offs, partitioning, connection pooling.
- OLTP vs OLAP intuition; schema evolution / migrations at scale.

## Worked examples

Colocated under `advanced-sql-and-query-performance/learning/code/`; each runnable against a seeded
PostgreSQL (DD-20/DD-30).

- **beginner** — a window-function report; a recursive CTE over a tree.
- **intermediate** — add an index and read `EXPLAIN` before/after; diagnose & fix an N+1 from the app side.
- **advanced** — reproduce & fix an isolation-level anomaly; a partitioning/denormalization trade-off
  worked out with measurements.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: take a seeded PostgreSQL database + a Python access path and make it fast and correct: write a
  reporting query with window functions/CTEs, diagnose an N+1 and a missing-index slow query with
  `EXPLAIN ANALYZE` and fix both, and reproduce then resolve an isolation-level anomaly — with
  before/after plans and timings.
- **Concepts exercised**: [ ] window functions + a recursive CTE [ ] reading `EXPLAIN ANALYZE`
  [ ] an index that changes the plan [ ] N+1 diagnosis + fix [ ] an isolation-level anomaly reproduced +
  resolved [ ] before/after measurements.
- **Ordered steps**:
  1. `.../learning/capstone/code/seed.sql` — schema + a dataset large enough for plans to differ. Verify
     the seed loads and row counts are as expected.
  2. `report.sql` — a window-function + recursive-CTE report. Verify it returns the correct aggregate on
     the seed.
  3. Capture a slow query's `EXPLAIN ANALYZE`, add the right index, re-capture. Verify the plan changes
     (seq scan → index) and time drops; fix the app-side N+1 and show query-count before/after.
  4. `anomaly.md` + scripts — reproduce a non-repeatable-read/write-skew anomaly, then fix it with the
     correct isolation level/locking. Verify the anomaly occurs before and is gone after.
- **Acceptance criteria**: the report is correct; the index measurably changes the plan and timing; the N+1
  is eliminated (fewer queries); the anomaly is demonstrably reproduced and resolved.
- **Done bar**: runnable end-to-end (against seeded PostgreSQL) + web-verified.

---

← Previous: [22 · Software Engineering Practices](./22-software-engineering-practices.md) · Next: [24 · Software Product Engineering](./24-software-product-engineering.md) →
