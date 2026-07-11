# 36 · Data Engineering (Annotated-concept, Python)

**prd row**: Pass 3 · Build for the Real World · Annotated-concept · Python · Learn 136 / Drill 236 ·
Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: moving and shaping data at rest — batch vs streaming, ETL/ELT pipelines, the medallion
(bronze/silver/gold) layering, dimensional modeling (star schema), data quality, and orchestration — as
runnable Python against local files/DB. Operational SQL depth is
[`23-advanced-sql-and-query-performance`](./23-advanced-sql-and-query-performance.md); the AI/RAG data
path is [`37-creating-ai-powered-apps`](./37-creating-ai-powered-apps.md).

## Prerequisites

- **Prior topics**: [topic 08 SQL Essentials](./08-sql-essentials.md) + [topic 23 Advanced SQL](./23-advanced-sql-and-query-performance.md)
  (the warehouse target, star schema, window functions), and [topic 04 Just Enough Python](./04-just-enough-python.md).
- **Tools & environment**: a macOS/Linux terminal; **Python 3.x** with pinned CVE-clean data libs
  (a dataframe lib + a local analytical engine such as DuckDB); a local SQL DB; sample CSV/JSON datasets.
- **Assumed knowledge**: SQL joins + aggregation (topic 08); window functions + EXPLAIN (topic 23); Python
  functions + files (topic 04).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: medallion architecture (bronze/silver/gold), ETL vs ELT, Kimball-style
  dimensional modeling (star schema, SCDs), and DAG-based orchestration are evergreen data-engineering
  vocabulary, unchanged in current (2026) usage (Databricks medallion terminology, dbt ELT framing). No
  version/license-sensitive claims to correct.

## Items

- Batch vs streaming; ETL vs ELT; the modern data-stack shape (sources → ingest → transform → serve).
- Pipelines: idempotent, incremental, backfill-safe transforms; partitioning.
- The medallion architecture: bronze (raw) → silver (cleaned/conformed) → gold (serving) layers.
- Dimensional modeling: facts vs dimensions, star schema, slowly-changing dimensions.
- Data quality: schema validation, null/dup/range checks, freshness; contracts.
- Orchestration concepts: DAGs, scheduling, retries, lineage (annotated; a tiny runnable DAG in Python).

## Worked examples

Colocated under `data-engineering/learning/code/`; each runnable in Python against local data (DD-20/DD-30).

- **beginner** — an idempotent ETL step: read raw CSV → clean/validate → write a silver table; re-run
  produces no duplicates.
- **intermediate** — a star schema (fact + dimensions) built from silver; a gold aggregate served via SQL.
- **advanced** — a small orchestrated DAG (extract → transform → quality-check → load) with retries + a
  data-quality gate that fails the run on a bad batch.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: build a complete local data pipeline — ingest raw source files to a bronze layer, clean and
  conform to a silver layer, model a star schema and serve gold aggregates, all wrapped in a small
  orchestrated DAG with retries and a data-quality gate — idempotent and backfill-safe, verified end to
  end in Python.
- **Concepts exercised**: [ ] idempotent + incremental ingest (bronze) [ ] cleaning/conforming (silver)
  [ ] a star schema (facts + dimensions) [ ] gold serving aggregates [ ] data-quality checks that fail a
  bad batch [ ] a small orchestrated DAG with retries.
- **Ordered steps**:
  1. `.../learning/capstone/code/ingest.py` — raw files → bronze, idempotent. Verify a re-run adds no
     duplicate rows.
  2. `transform.py` — bronze → silver (typed, deduped, validated) → a star schema. Verify facts join to
     every dimension and row counts reconcile.
  3. `serve.sql` / `serve.py` — gold aggregates from the star schema. Verify a serving query matches a
     hand-computed expected total.
  4. `pipeline.py` — a DAG wiring the steps with retries + a quality gate. Verify a deliberately bad batch
     fails the quality gate and does not reach gold.
- **Acceptance criteria**: the pipeline is idempotent + backfill-safe; the star schema reconciles; gold
  aggregates are correct; a bad batch is caught by the quality gate before serving.
- **Done bar**: runnable end-to-end + web-verified.

---

← Previous: [35 · Cloud & IaC](./35-cloud-and-iac.md) · Next: [37 · Creating AI-Powered Apps](./37-creating-ai-powered-apps.md) →
