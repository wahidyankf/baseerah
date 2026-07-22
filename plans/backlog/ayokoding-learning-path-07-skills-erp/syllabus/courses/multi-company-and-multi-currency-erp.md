# Multi-Company and Multi-Currency ERP (By Example)

**Course ID**: `multi-company-and-multi-currency-erp` · **Format**: By Example · **Language**: — (domain, worked scenarios, no application code).

**Short summary**: Company-code structures, intercompany elimination, currency translation

**Scope note**: the enterprise-scale cluster's core course — multi-entity and multi-currency
structures, closing the loop opened by course 13's intercompany preview. Requires
`consolidation-and-multi-entity-accounting` from the accounting corpus. License-aware (DD-15).

## Why this exists · the big idea

- **The problem before the solution**: a single-entity, single-currency mental model breaks down the
  moment a business operates through more than one legal entity or transacts in more than one
  currency — both require structure the earlier courses' examples deliberately simplified away.
- **Keep-this-if-you-forget-everything**: an intercompany transaction must be eliminated at
  consolidation, or the group's combined financial statements double-count revenue and expense that
  never left the group.
- **Big ideas touched**: `company-code-as-a-legal-boundary`; `translation-vs-transaction-currency` —
  two distinct currency concerns often conflated.

## Prerequisites

- **ERP prereqs**: [`record-to-report-systems`](./record-to-report-systems.md).
- **Accounting prereqs**: `consolidation-and-multi-entity-accounting` (from
  `ayokoding-learning-path-06-skills-accounting`).
- **Assumed knowledge**: course 13's intercompany-transaction preview; course 2's multi-currency-field
  introduction.

## Accuracy notes (pending A4 verification)

> The Phase 1.2a confirmation pass has not yet run for this course.

- The accounting-side course id `consolidation-and-multi-entity-accounting` is as named in
  `ayokoding-learning-path-06-skills-accounting`'s own in-flight rewrite as of 2026-07-22.

## Concepts

- **co-01 · company-code-structure** — the organizational unit representing one legal entity within a
  multi-entity ERP deployment.
- **co-02 · intercompany-transaction** — a transaction between two related legal entities, requiring
  elimination at consolidation (deep dive from course 13's preview).
- **co-03 · elimination-entry** — the accounting adjustment that removes an intercompany transaction's
  effect from consolidated financial statements.
- **co-04 · transaction-currency** — the currency a specific document is recorded in.
- **co-05 · translation-currency** — the currency a document's amount is converted to for group-level
  reporting, distinct from transaction currency.
- **co-06 · currency-revaluation** — a periodic adjustment for balance-sheet items held in a foreign
  currency, as exchange rates move.
- **co-07 · multi-entity-close-sequencing** — why individual entities must close (course 7) before a
  group consolidation can run.

## Worked examples

Prose-based worked scenarios with originally-authored data (no runnable code). Every example cites
the `co-NN` it exercises.

### Beginner

- **ex-01 · company-code-assignment** — given a group with three legal entities, assign a sample
  transaction to its correct company code. (co-01)
- **ex-02 · transaction-vs-translation-currency** — given a transaction recorded in one currency and
  reported at group level in another, distinguish the two currency roles. (co-04, co-05)

### Intermediate

- **ex-03 · intercompany-transaction-trace** — given a sale from one group entity to another, trace it
  through both entities' books before elimination. (co-02)
- **ex-04 · elimination-entry-construction** — construct the elimination entry that removes ex-03's
  transaction from the consolidated statements. (co-03)

### Advanced

- **ex-05 · currency-revaluation-computation** — given a foreign-currency balance-sheet item and a
  rate change, compute the revaluation adjustment. (co-06)
- **ex-06 · multi-entity-close-sequencing-trace** — given three entities on different close schedules,
  determine the earliest possible date for group consolidation. (co-07)

## Synthesis exercise — intra-topic

> Analysis and design only — never build, implement, or deploy a system (`A6`).

- **Goal**: design a two-entity group structure with one intercompany transaction and one
  foreign-currency balance, and produce the elimination entry and revaluation adjustment.
- **Concepts exercised**: [ ] company codes (co-01) [ ] intercompany elimination (co-02, co-03) [ ]
  currency translation/revaluation (co-04–co-06).
- **Ordered steps**: 1) define the two entities and currencies; 2) record the intercompany
  transaction; 3) construct the elimination entry; 4) compute the revaluation adjustment.
- **Acceptance criteria**: the elimination entry correctly zeroes out the intercompany effect at group
  level; the revaluation is arithmetically correct.
- **Done bar**: a written worked example, no code, no system built.

## Read more

> Populated during the Phase 1.2a `web-researcher` confirmation pass (`A12`) — coverage-check
> citations only.

## In which paths

- `skills/conventional-erp` — Stage B, course 24 of 26.
- `skills/sharia-erp` — Stage B, course 24 of 29.

---

← Back to the [syllabus index](../README.md)
