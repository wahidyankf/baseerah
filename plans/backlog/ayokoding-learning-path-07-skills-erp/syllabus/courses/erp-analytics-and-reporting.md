# ERP Analytics and Reporting (By Example)

**Course ID**: `erp-analytics-and-reporting` · **Format**: By Example · **Language**: SQL (illustrative extraction patterns, no runnable pipeline).

**Short summary**: Operational reporting vs BI extraction, ERP-specific CDC, embedded vs external analytics

**Scope note**: the second of the two scope-boundary-risk courses (DD-10) — stays scoped to
**ERP-specific** change-data-capture and delta-extraction patterns, explicitly distinct from
`data-engineering`'s general CDC scope. Closes Stage B and `conventional-erp` (Dangerous 3).
License-aware (DD-15).

## Why this exists · the big idea

- **The problem before the solution**: an ERP's own transactional reports answer "what happened", but
  cross-period trend analysis and dashboards typically need data extracted into a separate analytical
  store — and extracting it correctly requires ERP-specific knowledge of which tables change and how.
- **Keep-this-if-you-forget-everything**: operational reporting reads live transactional data; BI
  extraction reads a copy — the two have different freshness, performance, and query-complexity
  trade-offs.
- **Big ideas touched**: `extraction-as-a-distinct-concern-from-reporting`; `erp-specific-vs-general-cdc-scope`
  — this course's own boundary, stated explicitly.

## Prerequisites

- **ERP prereqs**: [`record-to-report-systems`](./record-to-report-systems.md).
- **Cross-domain prerequisites**: `data-engineering`, `analytics-and-experimentation`,
  `advanced-sql-and-query-performance` (all existing library).
- **Assumed knowledge**: the three existing-library courses' own pipeline, experimentation, and
  query-tuning vocabulary; course 13's trial-balance/R2R vocabulary.

## Accuracy notes (pending A4 verification)

> The Phase 1.2a confirmation pass has not yet run for this course.

- Illustrative SQL fragments use an originally-authored schema; no extraction query is lifted from any
  reference implementation.

## Concepts

- **co-01 · operational-reporting** — a report reading live transactional data directly, favoring
  freshness over query complexity.
- **co-02 · bi-extraction** — a periodic copy of transactional data into a separate analytical store,
  favoring query complexity and isolation from operational load.
- **co-03 · change-data-capture-erp-specific** — identifying which ERP tables changed since the last
  extraction, scoped to ERP-specific change markers (status transitions, posting timestamps) rather
  than a general CDC mechanism.
- **co-04 · delta-extraction** — extracting only changed records since the last run, rather than a
  full reload.
- **co-05 · embedded-analytics** — analytics rendered inside the ERP's own interface, reading live or
  near-live data.
- **co-06 · external-warehouse-analytics** — analytics rendered from a separate data warehouse fed by
  extraction.
- **co-07 · extraction-consistency** — an extraction must capture a consistent snapshot (e.g. not mid-
  posting), or downstream analytics will misreport.
- **co-08 · scope-boundary-vs-data-engineering** — this course's own boundary: ERP-specific CDC and
  delta-extraction mechanics, explicitly distinct from `data-engineering`'s general-purpose pipeline
  treatment.

## Worked examples

Illustrative SQL fragments and prose scenarios (no runnable pipeline). Every example cites the
`co-NN` it exercises.

### Beginner

- **ex-01 · operational-vs-bi-contrast** — given a request for "sales this week" versus "sales trend
  over three years", classify each as better served by operational reporting or BI extraction. (co-01,
  co-02)
- **ex-02 · delta-extraction-sketch** — sketch an illustrative query selecting only records changed
  since a given timestamp. (co-04)

### Intermediate

- **ex-03 · erp-specific-cdc-identify** — given an ERP table's status-transition history (course 4),
  identify which field changes should trigger inclusion in a delta extraction. (co-03)
- **ex-04 · extraction-consistency-failure** — given an extraction run mid-posting, show how a
  half-posted document produces an inconsistent analytical snapshot. (co-07)

### Advanced

- **ex-05 · embedded-vs-external-tradeoff** — given a reporting requirement, decide between embedded
  analytics and external-warehouse analytics, justifying the choice. (co-05, co-06)
- **ex-06 · scope-boundary-self-check** — given five candidate topics (ERP-specific CDC design,
  general streaming-pipeline architecture, delta-extraction scheduling, dashboard visualization
  design, general data-warehouse modeling), mark which belong in this course and which belong in
  `data-engineering` instead. (co-08)

## Synthesis exercise — intra-topic

> Analysis and design only — never build, implement, or deploy a system (`A6`).

- **Goal**: design a delta-extraction scheme for one ERP subject area (e.g. sales orders), including
  an ERP-specific CDC marker and a consistency safeguard.
- **Concepts exercised**: [ ] CDC (co-03) [ ] delta extraction (co-04) [ ] extraction consistency
  (co-07).
- **Ordered steps**: 1) identify the CDC marker; 2) design the delta-extraction query (illustrative
  SQL); 3) design the consistency safeguard.
- **Acceptance criteria**: the CDC marker is a real ERP-specific field (e.g. a status or posting
  timestamp); the consistency safeguard would actually prevent a mid-posting snapshot.
- **Done bar**: a written design with illustrative SQL, no runnable pipeline, no system built.

## Read more

> Populated during the Phase 1.2a `web-researcher` confirmation pass (`A12`) — coverage-check
> citations only.

## In which paths

- `skills/conventional-erp` — Stage B, course 26 of 26. **Dangerous 3 ⚡ — `conventional-erp` ENDS
  HERE.**
- `skills/sharia-erp` — Stage B, course 26 of 29.

---

← Back to the [syllabus index](../README.md)
