# 10 · SQL Essentials (By Example, SQL + Python †)

**prd row**: Pass 1 · Core Foundations · By Example · SQL + Python † (SQLite) · Learn 108 / Drill
208 · Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the **usable slice** — schema design + core queries + safe access from Python via SQLite,
all from the CLI. Window functions, CTEs, indexing strategy, and isolation levels are deferred to
[`26-advanced-sql-and-query-performance`](./26-advanced-sql-and-query-performance.md) (DD-11). SQLite is
public-domain (Tier-1, DD-21).

## Why this exists · the big idea

- **The problem before the solution**: application data outlives the process that made it and must be
  queried, related, and kept consistent — an in-memory structure cannot do that.
- **Keep-this-if-you-forget-everything**: declare _what_ result you want and let the engine decide _how_
  to get it; the relational model separates your intent from the storage machinery.
- **Big ideas touched**: `mechanism-vs-policy` — SQL is declarative policy (the result you want), the
  query planner is the mechanism (how it is fetched); normalization keeps one fact in one place.

## Prerequisites

- **Prior topics**: [topic 4 Just Enough Python](./04-just-enough-python.md) (Python drives DB access).
- **Tools & environment**: a macOS/Linux terminal; **SQLite** (`sqlite3 --version`, bundled with Python's
  `sqlite3` module); **Python 3.x** with `pytest` in a `venv`. `psql`/PostgreSQL only for the
  cross-reference note (not required).
- **Assumed knowledge**: reading/writing basic Python; no prior SQL or database background required.

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: current SQLite **3.53.3** (2026-06-26); **public-domain** (no license needed).
  Python `sqlite3` parameterized queries support `?` (qmark, sequence param) and `:name` (named, dict
  param) — both current. Note the SQLite version bundled with a given Python build varies; phrase the
  topic to read `sqlite3.sqlite_version` at runtime rather than asserting a fixed bundled version.
  (sqlite.org / docs.python.org)

## Items

- **Relational model**: tables, rows, primary/foreign keys, basic constraints, normalization (1NF–3NF)
  intuition.
- **Core SQL**: `SELECT`/`WHERE`/`ORDER BY`/`LIMIT`; `INSERT`/`UPDATE`/`DELETE`; inner & left `JOIN`;
  `GROUP BY` + `COUNT`/`SUM`/`AVG`.
- **DDL basics**: `CREATE TABLE`, column types.
- **Access from Python**: `sqlite3`/driver, parameterized queries (SQL-injection avoidance), a
  transaction with commit/rollback.
- **Running a DB from the CLI** (`sqlite3`/`psql`) — no GUI.

## Worked examples

Colocated under `sql-essentials/learning/code/`; `.sql` scripts run via `sqlite3` + a Python access
script (DD-20/DD-30).

- **beginner** — design a normalized 2–3-table schema; `SELECT`/`INSERT`/`UPDATE`; an inner `JOIN`; a
  parameterized query from Python.
- **intermediate** — a `GROUP BY` + `HAVING` aggregation report; a transaction that rolls back on error.
- **advanced** — a migration that adds a column safely; basic N+1 avoidance.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: design and populate a small normalized SQLite database (3–4 tables) and ship a Python data
  access layer with parameterized queries, a reporting aggregation, and a transaction that rolls back on
  error — runnable from the CLI end-to-end.
- **Concepts exercised**: [ ] 3NF schema with PK/FK constraints [ ] `CREATE TABLE` DDL [ ] joins +
  `GROUP BY`/`HAVING` report [ ] parameterized queries (no string interpolation) [ ] commit/rollback
  transaction [ ] a safe additive migration.
- **Ordered steps**:
  1. `.../learning/capstone/code/schema.sql` + `seed.sql` — apply via `sqlite3 app.db < schema.sql`.
     Verify `.tables` lists all tables with FKs.
  2. `dal.py` — parameterized CRUD + a `GROUP BY` report function. Verify `pytest` on a seeded fixture DB.
  3. A transaction that partially fails and rolls back. Verify the DB is unchanged after the failure.
  4. `migrate_add_column.sql` — an additive column with a default. Verify existing rows still valid.
- **Acceptance criteria**: `pytest` green; the report matches hand-computed expected values; the rollback
  leaves no partial write; no query uses string interpolation (injection-safe).
- **Done bar**: runnable end-to-end + web-verified.

## Read more

**Books**

- **Database System Concepts** — Silberschatz, Korth, Sudarshan (7th ed., 2019). Standard textbook on the relational model, SQL, and database system design.
- **SQL and Relational Theory** — C.J. Date (3rd ed., 2015). Rigorous treatment of the relational model underlying SQL, clarifying where SQL diverges from theory.
- **Joe Celko's SQL for Smarties: Advanced SQL Programming** — Joe Celko (5th ed., 2014). Classic advanced-technique reference from an SQL-89/92 standards co-author.

**Papers & articles**

- **"A Relational Model of Data for Large Shared Data Banks"** — E.F. Codd (1970, CACM). Foundational paper introducing the relational model. <https://dl.acm.org/doi/10.1145/362384.362685>

---

← Previous: [9 · Project Management](./09-project-management.md) · Next: [11 · Backend Essentials](./11-backend-essentials.md) →
