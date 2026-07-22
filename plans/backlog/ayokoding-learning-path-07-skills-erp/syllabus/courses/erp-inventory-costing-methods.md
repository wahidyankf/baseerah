# ERP Inventory Costing Methods (By Example)

**Course ID**: `erp-inventory-costing-methods` · **Format**: By Example · **Language**: — (domain, worked scenarios, no application code).

**Short summary**: FIFO/moving-average/standard cost, revaluation, and a costing-mismatch failure

**Scope note**: the second inventory hard-parts course — one of the domain's genuine hard parts
[Repo-grounded — domain-research grounding, Part 2]: costing method choice and its revaluation
behaviour. License-aware (DD-15).

## Why this exists · the big idea

- **The problem before the solution**: two businesses can process the identical physical transactions
  and report different COGS and inventory values, purely because they chose different costing
  methods — and a mismatch between the configured method and the business's actual practice produces
  a silent misstatement.
- **Keep-this-if-you-forget-everything**: a costing method is a valuation policy, not a physical fact
  about the inventory — the same units can be valued three different ways depending on which method
  is configured.
- **Big ideas touched**: `costing-method-as-policy-not-fact`; `revaluation-as-a-distinct-posting-event`.

## Prerequisites

- **ERP prereqs**: [`inventory-and-warehouse-management`](./inventory-and-warehouse-management.md).
- **Accounting prereqs**: `inventory-and-cogs-accounting` (transitive via course 14).
- **Assumed knowledge**: course 14's stock-type and valuation-touchpoint vocabulary.

## Accuracy notes (pending A4 verification)

> The Phase 1.2a confirmation pass has not yet run for this course.

- Worked costing examples use an originally-authored dataset throughout.

## Concepts

- **co-01 · fifo-costing** — first-in-first-out: the oldest cost layer is consumed first.
- **co-02 · moving-average-costing** — a single weighted-average cost, recalculated on every receipt.
- **co-03 · standard-costing** — a fixed, predetermined cost, with variance posted separately.
- **co-04 · cost-layer** — a costing method's internal record of "this quantity at this cost", present
  in FIFO, absent in moving-average.
- **co-05 · revaluation** — a distinct posting event that adjusts inventory value when a cost changes
  after the fact.
- **co-06 · variance-posting** — under standard costing, the difference between standard and actual
  cost, posted to a variance account.
- **co-07 · method-choice-consequence** — the same physical transactions produce different COGS and
  inventory values under different costing methods.
- **co-08 · costing-mismatch-silent-failure** — a costing method configured inconsistently with actual
  business practice (e.g. standard costing with no variance review process) produces a silent
  misstatement.

## Worked examples

Prose-based worked scenarios with originally-authored data (no runnable code). Every example cites
the `co-NN` it exercises.

### Beginner

- **ex-01 · fifo-layer-trace** — given three receipts at different costs and one issue, compute the
  issue's cost under FIFO. (co-01, co-04)
- **ex-02 · moving-average-recompute** — given the same three receipts, compute the moving-average
  cost after each. (co-02)

### Intermediate

- **ex-03 · method-comparison** — given the identical receipt/issue sequence, compute COGS under
  FIFO, moving-average, and standard costing, and compare the three results. (co-01–co-03, co-07)
- **ex-04 · standard-cost-variance** — given a standard cost and an actual purchase price, compute and
  post the variance. (co-03, co-06)

### Advanced

- **ex-05 · revaluation-event** — given a cost change after goods are already in stock, compute the
  revaluation entry. (co-05)
- **ex-06 · costing-mismatch-demonstration** — given standard costing configured with no variance
  review process, show how a growing unreviewed variance balance constitutes a silent misstatement.
  (co-08)

## Synthesis exercise — intra-topic

> Analysis and design only — never build, implement, or deploy a system (`A6`).

- **Goal**: given one item's full transaction history, compute its cost under all three methods, and
  write a short analysis of which method fits the item's actual price volatility and why a mismatched
  choice would misstate results.
- **Concepts exercised**: [ ] all three costing methods (co-01–co-03) [ ] method-choice consequence
  (co-07) [ ] costing mismatch (co-08).
- **Ordered steps**: 1) compute cost under each method; 2) compare the three; 3) recommend a method
  with reasoning; 4) describe a mismatch scenario and its consequence.
- **Acceptance criteria**: all three computations are internally consistent; the recommendation cites
  a concrete reason (e.g. price volatility).
- **Done bar**: a written worked example, no code, no system built.

## Read more

> Populated during the Phase 1.2a `web-researcher` confirmation pass (`A12`) — coverage-check
> citations only.

## In which paths

- `skills/conventional-erp` — Stage B, course 15 of 27.
- `skills/sharia-erp` — Stage B, course 15 of 30.

---

← Back to the [syllabus index](../README.md)
