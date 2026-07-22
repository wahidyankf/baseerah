# ERP Fiscal Calendar and Period Close (Annotated-concept)

**Course ID**: `erp-fiscal-calendar-and-period-close` · **Format**: Annotated-concept · **Language**: — (domain, no code).

**Short summary**: Fiscal year variants, period locks, the close checklist, year-end carry-forward

**Scope note**: the cross-cutting-spine course covering how an ERP models time for accounting
purposes — fiscal year variants, period statuses, and the mechanics of closing a period so that
posting to it is blocked. Sets up course 13's record-to-report treatment. License-aware (DD-15).

## Why this exists · the big idea

- **The problem before the solution**: without period locks, a transaction dated into a closed period
  can post silently, corrupting a financial statement that was already reported as final.
- **Keep-this-if-you-forget-everything**: closing a period is a status change with teeth — it blocks
  posting, it doesn't just mark a date as "done".
- **Big ideas touched**: `period-as-a-gate`, `year-end-carry-forward-as-a-distinct-mechanism` from
  ordinary period close.

## Prerequisites

- **Prior topics**: [`erp-subledger-to-gl-architecture`](./erp-subledger-to-gl-architecture.md).
- **Cross-domain prerequisites**: none.
- **Assumed knowledge**: course 6's control-account and posting vocabulary.

## Accuracy notes (pending A4 verification)

> The Phase 1.2a confirmation pass has not yet run for this course.

- Fiscal-year variant names (calendar, 4-4-5, custom) are generic across the industry `[Judgment
call]`.

## Concepts

- **co-01 · fiscal-year-variants** — calendar-year, 4-4-5, and custom fiscal-year structures.
- **co-02 · period-status** — open, closing, closed as an explicit, enforced status per period.
- **co-03 · posting-lock** — a closed period blocks new postings dated into it.
- **co-04 · close-checklist-mechanics** — the ordered set of steps (subledger close, reconciliation,
  adjustment entries, lock) a period close typically follows.
- **co-05 · adjustment-entries** — entries posted specifically during the close window, distinct from
  ordinary transactional postings.
- **co-06 · year-end-carry-forward** — balances rolled from one fiscal year into the next, a distinct
  mechanism from ordinary period close.
- **co-07 · reopening-a-period** — the (normally exceptional, tightly controlled) act of reopening a
  closed period, and why it is dangerous.
- **co-08 · fiscal-calendar-and-multi-entity** — different legal entities may run different fiscal-year
  variants concurrently (preview of course 25).

## Worked examples

Prose-based worked scenarios (no runnable code). Every example cites the `co-NN` it exercises.

### Beginner

- **ex-01 · fiscal-year-variant-compare** — compare a calendar-year and a 4-4-5 fiscal year for the
  same business, noting where period boundaries land differently. (co-01)
- **ex-02 · posting-lock-demonstration** — attempt (on paper) to post a transaction dated into a closed
  period — verify it is blocked. (co-02, co-03)

### Intermediate

- **ex-03 · close-checklist-walkthrough** — walk a sample period through subledger close,
  reconciliation, adjustment entries, and lock, in order. (co-04, co-05)
- **ex-04 · year-end-carry-forward-trace** — trace a balance-sheet account's balance from the last day
  of one fiscal year into the first day of the next. (co-06)

## Synthesis exercise — intra-topic

> Analysis and design only — never build, implement, or deploy a system (`A6`).

- **Goal**: design a period-close checklist for a new legal entity, including the posting-lock
  sequencing and the year-end carry-forward step.
- **Concepts exercised**: [ ] period status (co-02) [ ] posting lock (co-03) [ ] close checklist
  (co-04) [ ] carry-forward (co-06).
- **Ordered steps**: 1) list close steps in order; 2) mark where the posting lock takes effect; 3)
  state which accounts carry forward and how.
- **Acceptance criteria**: the lock step comes after reconciliation, not before; carry-forward is
  distinguished from ordinary posting.
- **Done bar**: a written checklist, no code, no system built.

## Read more

> Populated during the Phase 1.2a `web-researcher` confirmation pass (`A12`) — coverage-check
> citations only.

## In which paths

- `skills/conventional-erp` — Stage A, course 7 of 27.
- `skills/sharia-erp` — Stage A, course 7 of 30.

---

← Back to the [syllabus index](../README.md)
