# ERP Procurement and Fulfillment Exceptions (By Example)

**Course ID**: `erp-procurement-and-fulfillment-exceptions` · **Format**: By Example · **Language**: — (domain, worked scenarios, no application code).

**Short summary**: Partial receipts/invoices, tolerances, returns — both directions

**Scope note**: the exception-handling course for both P2P (course 10) and O2C (course 11) — partial
receipts, partial invoices, over/under delivery tolerances, and returns processing in both
directions. This is one of the two scope-boundary-risk courses in the catalog: it stays scoped to
ERP-specific tolerance/match mechanics and never drifts into general contract-terms negotiation.
License-aware (DD-15).

## Why this exists · the big idea

- **The problem before the solution**: the happy-path chains in courses 10 and 11 assume everything
  arrives exactly as ordered — real procurement and fulfillment rarely do, and the exception paths
  are where most operational friction concentrates.
- **Keep-this-if-you-forget-everything**: a tolerance is a deliberately configured allowance, not a
  bug — the question is always "was this within the configured tolerance," never "did it match
  exactly."
- **Big ideas touched**: `tolerance-as-configuration`; `returns-as-a-mirrored-chain` — a return
  reverses the original chain's document flow rather than inventing a new one.

## Prerequisites

- **ERP prereqs**: [`procure-to-pay-systems`](./procure-to-pay-systems.md),
  [`order-to-cash-systems`](./order-to-cash-systems.md).
- **Cross-domain prerequisites**: none.
- **Assumed knowledge**: courses 10 and 11's chain vocabulary (PR/PO/GR/IR and quote/SO/delivery/
  invoice).

## Accuracy notes (pending A4 verification)

> The Phase 1.2a confirmation pass has not yet run for this course.

- Worked exception examples use originally-authored data throughout.

## Concepts

- **co-01 · partial-receipt** — a goods receipt for less than the ordered quantity, leaving the order
  open for a subsequent receipt.
- **co-02 · partial-invoice** — an invoice covering less than the full order or delivery, mirroring a
  partial receipt.
- **co-03 · over-delivery-tolerance** — a configured percentage or quantity above which excess
  delivery is rejected rather than accepted.
- **co-04 · under-delivery-tolerance** — a configured percentage or quantity below which a short
  delivery is still accepted as complete.
- **co-05 · three-way-match-with-tolerance** — the three-way match (course 10) evaluated against
  configured tolerances rather than exact equality.
- **co-06 · vendor-return-rma** — a return-to-vendor flow, referencing the original purchase chain.
- **co-07 · customer-return-rma** — a return-from-customer flow, referencing the original sales
  chain.
- **co-08 · returns-as-mirrored-chain** — a return reverses the original document flow's direction
  rather than inventing a new one.
- **co-09 · scope-boundary-tolerance-mechanics** — this course's own scope: ERP-specific
  tolerance/match configuration, explicitly distinct from general contract-terms negotiation (which
  this corpus does not teach).

## Worked examples

Prose-based worked scenarios with originally-authored data (no runnable code). Every example cites
the `co-NN` it exercises.

### Beginner

- **ex-01 · partial-receipt-trace** — given an order for 100 units received in two shipments of 60 and
  40, trace both goods receipts against the one order. (co-01)
- **ex-02 · tolerance-configuration-read** — given a tolerance table, determine whether a 5% over-
  delivery is accepted or rejected. (co-03, co-04)

### Intermediate

- **ex-03 · tolerance-match-pass** — given a delivery within tolerance, verify the three-way match
  passes despite a quantity that is not exactly equal. (co-05)
- **ex-04 · tolerance-match-fail** — given a delivery outside tolerance, verify the match fails and
  the exception is flagged. (co-05)
- **ex-05 · vendor-return-trace** — trace a vendor return referencing the original purchase order and
  goods receipt. (co-06, co-08)

### Advanced

- **ex-06 · customer-return-trace** — trace a customer return referencing the original sales order and
  delivery, including its effect on the customer invoice. (co-07, co-08)
- **ex-07 · scope-boundary-self-check** — given five candidate topics (tolerance configuration,
  contract-terms negotiation, vendor SLA drafting, return workflow design, price-negotiation
  strategy), mark which belong in this course and which fall outside its scope. (co-09)

## Synthesis exercise — intra-topic

> Analysis and design only — never build, implement, or deploy a system (`A6`).

- **Goal**: design a tolerance and returns policy for a new product line, and write worked examples
  demonstrating a within-tolerance pass, an out-of-tolerance fail, and a return in each direction.
- **Concepts exercised**: [ ] tolerances (co-03, co-04) [ ] tolerance-match evaluation (co-05) [ ]
  returns in both directions (co-06, co-07, co-08).
- **Ordered steps**: 1) set tolerance thresholds; 2) write a within-tolerance scenario; 3) write an
  out-of-tolerance scenario; 4) write both return directions.
- **Acceptance criteria**: tolerance thresholds are explicit; both pass and fail scenarios are
  correctly evaluated against them; both return directions correctly mirror their original chains.
- **Done bar**: a written worked example set, no code, no system built.

## Read more

> Populated during the Phase 1.2a `web-researcher` confirmation pass (`A12`) — coverage-check
> citations only.

## In which paths

- `skills/conventional-erp` — Stage A, course 12 of 26.
- `skills/sharia-erp` — Stage A, course 12 of 29.

---

← Back to the [syllabus index](../README.md)
