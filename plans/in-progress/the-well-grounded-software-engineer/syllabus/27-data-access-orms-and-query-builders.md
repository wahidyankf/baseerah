# 27 · Data Access: ORMs & Query Builders (By Example, Python †)

**prd row**: Pass 2 · Depth, Design & Craft · By Example · Python † · Learn 127 / Drill 227 · Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: the three-tier spectrum from raw SQL → query builder → full ORM, the trade-offs each
tier buys, the N+1 problem, the identity-map and unit-of-work patterns, migrations, and when each tier
is the right call. `†`: fully type-annotated Python (DD-34) over the DB-API driver. The build-your-own
tier — reconstructing a minimal ORM so it stops being magic — is
[`28-build-your-own-orm-and-query-builder`](./28-build-your-own-orm-and-query-builder.md).

## Why this exists · the big idea

- **The problem before the solution**: hand-writing SQL and mapping every result row to an object by
  hand is tedious and error-prone; the object-relational impedance mismatch — objects have identity,
  references, and inheritance while tables have rows, keys, and joins — bred a generation of
  boilerplate mapping code.
- **Keep-this-if-you-forget-everything**: an ORM is an abstraction over SQL, not a replacement for
  understanding it — it buys productivity on CRUD and charges you the moment the query matters, so know
  which tier you're on and what it hides.
- **Big ideas touched**: `abstraction-and-its-cost` (each tier hides more SQL for more leverage, and
  the hidden SQL leaks as the N+1 and the accidental full-table scan), `coupling-vs-cohesion` (a
  repository/data-mapper layer keeps persistence concerns cohesive and decoupled from domain logic
  instead of scattering SQL through the codebase).

## Prerequisites

- **Prior topics**: [topic 10 SQL Essentials](./10-sql-essentials.md) and
  [topic 26 Advanced SQL & Query Performance](./26-advanced-sql-and-query-performance.md).
- **Tools & environment**: a macOS/Linux terminal; **Python 3.x** (fully type-annotated); a local SQL
  database; the DB-API driver (PEP 249), a query builder, and an ORM library, each CVE-clean and
  pinned; Neovim/VSCode with the Python LSP (DD-17).
- **Assumed knowledge**: writing joins and reading an `EXPLAIN` plan (topics 10, 26); indexes and how
  they change a query plan (topic 26); reading a typed Python module (topic 04).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: the patterns taught here (Active Record, Data Mapper, Unit of Work, Identity
  Map, Lazy Load, the N+1 problem) are stable, named in Fowler's PoEAA, and correctly unpinned. PEP 249
  (Python DB-API v2.0) remains the standard low-level driver contract.
- 2026-07-12 — verified: specific ORM/query-builder library names and versions move over time — keep
  the shipped text pattern-first and library-agnostic, and re-verify any named library at authoring
  time.

## Items

- The data-access spectrum: raw SQL/driver → query builder → full ORM, and what each tier buys and
  hides.
- **Tier 1 — raw SQL** over the DB-API driver (PEP 249): full control, manual row→object mapping, no
  magic; fully type-annotated (DD-34).
- **Tier 2 — a query builder**: composable, parameterized queries as data — no hand-concatenated
  strings and no full object graph.
- **Tier 3 — a full ORM**: Active Record vs Data Mapper, the identity map, the unit of work, and lazy
  loading.
- The N+1 query problem: how an ORM invisibly fans one query out into hundreds, and how eager loading
  fixes it.
- Migrations: evolving a schema under version control, forward and rollback, and zero-downtime
  patterns.
- Choosing a tier per use case: reporting/analytics vs CRUD vs hot paths.

## Tensions & trade-offs — when NOT to reach for this

- **The ORM hides SQL until it can't**: the abstraction is productive for CRUD but leaks on the queries
  that matter — the N+1, the accidental full-table scan, the query you can't express — and then you
  need exactly the SQL you were avoiding (topic 26).
- **The query builder is often the sweet spot**: a full ORM buys identity map and change tracking you
  may not need; a query builder gives composition and injection safety without the object-graph
  machinery. Reach for the ORM when the domain is genuinely object-shaped, not by reflex.
- **When NOT**: analytics, reporting, and bulk operations are set-oriented — forcing them through an
  ORM's row-object model is slow and awkward, so drop to SQL for those.

## Lineage — why it beat the alternative

- The object-relational impedance mismatch spawned mountains of hand-written mapping code that was
  tedious and bug-prone. Fowler's _Patterns of Enterprise Application Architecture_ (2002) named the
  patterns — Active Record, Data Mapper, Unit of Work, Identity Map — that ORMs then productized, while
  the DB-API contract (PEP 249) gave Python a uniform driver layer to build on. The ORM won for
  CRUD-heavy applications by collapsing the boilerplate; it is precisely the abstraction whose cost the
  next topic makes concrete by rebuilding it —
  [`28-build-your-own-orm-and-query-builder`](./28-build-your-own-orm-and-query-builder.md) — and it
  rests on the query-performance foundation of
  [`26-advanced-sql-and-query-performance`](./26-advanced-sql-and-query-performance.md).

## Worked examples

Colocated under `data-access/learning/code/`; the same "list orders with their customer" query
implemented at each tier, runnable against a local DB, fully type-annotated Python (DD-20/DD-30/DD-34).

- **beginner** — Tier 1: the query in raw parameterized SQL over the DB-API driver, with manual typed
  row→object mapping.
- **intermediate** — Tier 2 then Tier 3: the same query via a query builder and then via an ORM;
  trigger an N+1 and then fix it with eager loading.
- **advanced** — an identity-map/unit-of-work write path plus a forward + rollback migration.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: implement the same small domain (customers/orders) across all three tiers — raw SQL, query
  builder, ORM — expose and fix an N+1, and ship a reversible migration, so each tier's trade-off is
  demonstrated rather than asserted.
- **Concepts exercised**: [ ] raw SQL via the DB-API (PEP 249) [ ] a query builder [ ] an ORM with
  identity map + unit of work [ ] an N+1 reproduced then fixed with eager loading [ ] a forward +
  rollback migration [ ] fully type-annotated Python.
- **Ordered steps**:
  1. `.../learning/capstone/code/tier1_sql.py` — the query in raw parameterized SQL with typed row
     mapping. Verify results match a known fixture and no string interpolation is used.
  2. `.../tier2_builder.py` and `.../tier3_orm.py` — the same query via builder and ORM. Verify
     identical results and capture the emitted SQL for each.
  3. Reproduce an N+1 on the ORM path, then fix it with eager loading. Verify the query count drops
     from N+1 to a small constant.
  4. `.../migrations/` — a forward migration plus its rollback. Verify migrate-then-rollback returns
     the schema to its starting state.
- **Acceptance criteria**: all three tiers return identical correct results; the N+1 is measurably
  eliminated; the migration applies and rolls back cleanly; the Python is fully type-annotated.
- **Done bar**: runnable end-to-end + web-verified.

## Read more

**Books**

- **Patterns of Enterprise Application Architecture** — Martin Fowler, with David Rice, Matthew
  Foemmel, Edward Hieatt, Robert Mee, Randy Stafford (2002). Named and codified the Active Record, Data
  Mapper, Unit of Work, and Identity Map patterns underlying every modern ORM.

**Papers & articles**

- **OrmHate** — Martin Fowler (2012). Canonical defense-and-critique of ORMs, addressing the
  object-relational impedance mismatch directly. <https://martinfowler.com/bliki/OrmHate.html>
- **Active Record** — Martin Fowler, _PoEAA_ online catalog (2002). Canonical definition of the Active
  Record pattern used by Rails and the Django ORM.
  <https://martinfowler.com/eaaCatalog/activeRecord.html>
- **Data Mapper** — Martin Fowler, _PoEAA_ online catalog (2002). Canonical definition of the Data
  Mapper pattern used by Hibernate, Doctrine, and SQLAlchemy's ORM layer.
  <https://martinfowler.com/eaaCatalog/dataMapper.html>
- **PEP 249 — Python Database API Specification v2.0** — Marc-André Lemburg (1999). The standard
  low-level interface that Python ORMs and query builders are built on top of.
  <https://peps.python.org/pep-0249/>

---

← Previous: [26 · Advanced SQL & Query Performance](./26-advanced-sql-and-query-performance.md) · Next: [28 · Build Your Own ORM & Query Builder](./28-build-your-own-orm-and-query-builder.md) →
