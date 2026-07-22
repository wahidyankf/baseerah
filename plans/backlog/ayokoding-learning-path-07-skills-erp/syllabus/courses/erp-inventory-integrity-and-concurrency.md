# ERP Inventory Integrity and Concurrency (By Example)

**Course ID**: `erp-inventory-integrity-and-concurrency` · **Format**: By Example · **Language**: — (domain, worked scenarios, no application code).

**Short summary**: Negative stock, backdated transactions, concurrent stock-movement races

**Scope note**: the third and final inventory hard-parts course [Repo-grounded — domain-research
grounding, Part 2] — closes Stage B's inventory cluster, and Dangerous 2 (see
[tech-docs.md §Landing content requirements](../../tech-docs.md#landing-content-requirements-what-plan-03-cannot-infer)).
License-aware (DD-15).

## Why this exists · the big idea

- **The problem before the solution**: real warehouses do not process transactions in a tidy,
  sequential order — receipts and issues happen concurrently, corrections arrive backdated, and stock
  sometimes goes negative before anyone notices.
- **Keep-this-if-you-forget-everything**: negative stock, backdating, and concurrency are not edge
  cases to design around later — they are the domain's normal operating conditions and must be
  designed for from the start.
- **Big ideas touched**: `negative-stock-as-a-policy-choice`; `backdating-reopens-costing`;
  `concurrency-as-a-normal-condition-not-an-edge-case`.

## Prerequisites

- **ERP prereqs**: [`inventory-and-warehouse-management`](./inventory-and-warehouse-management.md).
- **Accounting prereqs**: `inventory-and-cogs-accounting` (transitive via course 14).
- **Assumed knowledge**: course 14's stock-type vocabulary; course 15's costing-method vocabulary.

## Accuracy notes (pending A4 verification)

> The Phase 1.2a confirmation pass has not yet run for this course.

- Worked integrity/concurrency examples use an originally-authored dataset throughout.

## Concepts

- **co-01 · negative-stock** — an issue posted against insufficient on-hand quantity, and the policy
  question of whether it is allowed.
- **co-02 · negative-stock-costing-consequence** — under FIFO or moving-average, negative stock has no
  well-defined cost layer to consume — a genuine costing ambiguity.
- **co-03 · backdated-transaction** — a transaction posted with an effective date earlier than
  transactions already processed.
- **co-04 · backdating-reopens-costing** — a backdated receipt can retroactively change the cost layers
  already consumed by later issues.
- **co-05 · stock-concurrency-race** — two movements against the same item processed at nearly the
  same time, each unaware of the other's effect.
- **co-06 · lost-update-scenario** — a specific concurrency race where one movement's effect overwrites
  another's, understating or overstating the resulting quantity.
- **co-07 · locking-vs-optimistic-concurrency** — two general strategies (not implementation detail)
  for preventing a lost update, at a conceptual level.
- **co-08 · discrepancy-detection** — how a stock-count discrepancy surfaces after a concurrency race
  or an unresolved negative-stock condition.

## Worked examples

Prose-based worked scenarios with originally-authored data (no runnable code). Every example cites
the `co-NN` it exercises.

### Beginner

- **ex-01 · negative-stock-scenario** — given an issue larger than on-hand quantity, show the resulting
  negative balance and state the policy question it raises. (co-01)

### Intermediate

- **ex-02 · negative-stock-costing-ambiguity** — given a negative-stock scenario followed by a late
  receipt, show why the cost layer to assign to the earlier issue is ambiguous. (co-02)
- **ex-03 · backdated-transaction-trace** — given a backdated receipt inserted after later issues were
  already costed, show which prior costings are now inconsistent. (co-03, co-04)

### Advanced

- **ex-04 · concurrency-race-demonstration** — given two near-simultaneous issues against the same
  item, show how a naive read-then-write sequence produces a lost update. (co-05, co-06)
- **ex-05 · concurrency-strategy-contrast** — contrast, conceptually, how locking and optimistic
  concurrency would each prevent the ex-04 lost update, without specifying an implementation. (co-07)
- **ex-06 · discrepancy-detection-design** — design a stock-count reconciliation check that would
  surface the ex-04 lost update after the fact. (co-08)

## Synthesis exercise — intra-topic

> Analysis and design only — never build, implement, or deploy a system (`A6`).

- **Goal**: given a scenario combining a negative-stock event, a backdated transaction, and a
  concurrency race, write an analysis identifying each hazard and a detection check for each.
- **Concepts exercised**: [ ] negative stock (co-01, co-02) [ ] backdating (co-03, co-04) [ ]
  concurrency (co-05, co-06) [ ] detection (co-08).
- **Ordered steps**: 1) identify each hazard in the scenario; 2) explain its consequence; 3) propose a
  detection check for each.
- **Acceptance criteria**: all three hazards are correctly identified and distinguished; detection
  checks are concrete and specific to each hazard.
- **Done bar**: a written analysis, no code, no system built.

## Read more

> Populated during the Phase 1.2a `web-researcher` confirmation pass (`A12`) — coverage-check
> citations only.

## In which paths

- `skills/conventional-erp` — Stage B, course 16 of 27. **Dangerous 2 ⚡ boundary — inventory
  hard-parts cluster complete.**
- `skills/sharia-erp` — Stage B, course 16 of 30. **Dangerous 2 ⚡ boundary.**

---

← Back to the [syllabus index](../README.md)
