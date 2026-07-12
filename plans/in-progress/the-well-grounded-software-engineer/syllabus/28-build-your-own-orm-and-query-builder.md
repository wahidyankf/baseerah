# 28 · Build Your Own ORM & Query Builder (By Example, Python †)

**prd row**: Pass 2 · Depth, Design & Craft · By Example · Python † · Learn 128 / Drill 228 · Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the build-your-own tier of
[`27-data-access-orms-and-query-builders`](./27-data-access-orms-and-query-builders.md) — implement a
minimal ORM and query builder so the tier above stops being magic. You build a fluent query builder,
row→object mapping, an identity map, a unit of work, and lazy loading. `†`: fully type-annotated
Python (DD-34) over the DB-API driver.

## Why this exists · the big idea

- **The problem before the solution**: an ORM feels like magic until it surprises you — a silent N+1,
  a stale object, a write that didn't persist — and you can't debug what you can't picture, so the
  fastest way to demystify the abstraction is to rebuild its core.
- **Keep-this-if-you-forget-everything**: an ORM is a handful of small, comprehensible mechanisms — a
  query built as data, rows mapped to typed objects, one object per identity, changes tracked and
  flushed in a single transaction — none of which is magic once you've written it.
- **Big ideas touched**: `abstraction-and-its-cost` (building the abstraction yourself makes its cost
  concrete — you feel exactly what lazy loading buys and what it charges when it fans into an N+1).

## Prerequisites

- **Prior topics**: [topic 27 Data Access: ORMs & Query Builders](./27-data-access-orms-and-query-builders.md).
- **Tools & environment**: a macOS/Linux terminal; **Python 3.x** (fully type-annotated); a local
  SQLite (or equivalent) database reached through the standard DB-API driver (PEP 249); a test runner;
  Neovim/VSCode with the Python LSP (DD-17).
- **Assumed knowledge**: the three data-access tiers and the patterns they use — identity map, unit of
  work, lazy load, the N+1 (topic 27); parameterized SQL and joins (topics 10, 26); reading and writing
  typed Python (topic 04).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: the patterns being rebuilt (query object, row mapper, identity map, unit of
  work, lazy load) are stable, named in Fowler's PoEAA, and correctly unpinned; the first-person
  SQLAlchemy architecture account (AOSA vol. II) remains the canonical reference for how a production
  Python ORM is actually structured.
- 2026-07-12 — verified: PEP 249 (Python DB-API v2.0) is the current standard driver contract the
  hand-built layer sits on; no version to pin.

## Items

- A fluent, composable **query builder** that emits parameterized SQL as data — no string
  concatenation.
- **Row→object mapping**: turning result rows into typed domain objects and back.
- An **identity map**: guaranteeing one in-memory object per primary key within a session.
- A **unit of work**: tracking new/dirty/deleted objects and flushing them in one transaction.
- **Lazy loading**: deferring a relationship's load until first access — and where it bites (the N+1
  you now cause yourself).
- Wiring it together over a real DB-API driver (PEP 249), fully type-annotated (DD-34).

## Worked examples

Colocated under `build-your-own-orm/learning/code/`; each component is built and unit-tested against a
local SQLite database, fully type-annotated Python (DD-20/DD-30/DD-34).

- **beginner** — the query builder: `select(...).where(...).order_by(...)` composing down to
  parameterized SQL plus bound parameters.
- **intermediate** — row→object mapping plus an identity map that returns the same instance for a
  repeated primary key.
- **advanced** — a unit of work that batches inserts/updates/deletes into one transaction, plus a
  lazy-loaded relationship that demonstrates the N+1 it can cause.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: build a working miniature ORM — query builder, mapper, identity map, unit of work, lazy
  loading — over the DB-API, and use it to run the same customers/orders scenario from topic 27,
  proving you can rebuild the abstraction you were using.
- **Concepts exercised**: [ ] a fluent query builder emitting parameterized SQL [ ] row→object mapping
  [ ] an identity map [ ] a unit of work with a single-transaction flush [ ] lazy loading [ ] fully
  type-annotated Python.
- **Ordered steps**:
  1. `.../learning/capstone/code/query_builder.py` — the builder. Verify a composed query produces the
     expected SQL string and bound parameters, with no interpolation.
  2. `.../mapper.py` + `.../identity_map.py` — mapping and the identity map. Verify loading the same
     primary key twice returns the identical object instance.
  3. `.../unit_of_work.py` — track and flush changes. Verify a session with mixed new/dirty/deleted
     objects commits in one transaction and rolls back atomically on error.
  4. `.../lazy.py` — a lazy relationship. Verify it loads on first access and demonstrate the N+1 it
     can cause.
- **Acceptance criteria**: the builder emits safe parameterized SQL; the identity map de-duplicates by
  key; the unit of work flushes atomically; lazy loading defers correctly and the N+1 is observable.
- **Done bar**: runnable end-to-end + web-verified.

## Read more

**Books**

- **Patterns of Enterprise Application Architecture** — Martin Fowler et al. (2002). The design
  blueprint — Active Record, Data Mapper, Unit of Work, Identity Map, Query Object, Lazy Load — for
  implementing an ORM from scratch.

**Papers & articles**

- **SQLAlchemy** (chapter) — Michael Bayer, in _The Architecture of Open Source Applications, Volume
  II_ (2012). First-person account by SQLAlchemy's creator of how a production-grade Python ORM and
  query builder is actually architected. <https://aosabook.org/en/v2/sqlalchemy.html>
- **PEP 249 — Python Database API Specification v2.0** — Marc-André Lemburg (1999). The DB-API contract
  any hand-built ORM or query builder must sit on top of. <https://peps.python.org/pep-0249/>
- **OrmHate** — Martin Fowler (2012). Essential framing of what a hand-built ORM must solve, and why
  the problem is harder than it looks. <https://martinfowler.com/bliki/OrmHate.html>

---

← Previous: [27 · Data Access: ORMs & Query Builders](./27-data-access-orms-and-query-builders.md) · Next: [29 · Advanced Networking](./29-advanced-networking.md) →
