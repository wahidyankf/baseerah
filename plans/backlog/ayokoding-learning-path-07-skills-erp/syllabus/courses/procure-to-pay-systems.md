# Procure-to-Pay Systems (By Example)

**Course ID**: `procure-to-pay-systems` · **Format**: By Example · **Language**: — (domain, worked scenarios, no application code).

**Short summary**: The PR→PO→GR→IR chain, three-way match, vendor master, worked P2P example

**Scope note**: the first of the two core transaction-cycle courses (with order-to-cash, course 11) —
the procure-to-pay chain from purchase requisition through invoice receipt, with the three-way match
as its central control. Exception handling (partials, tolerances, returns) is deferred to course 12.
License-aware (DD-15).

## Why this exists · the big idea

- **The problem before the solution**: paying for goods that were never received, or receiving goods
  never ordered, are both real risks — the PR→PO→GR→IR chain and its three-way match exist
  specifically to prevent them.
- **Keep-this-if-you-forget-everything**: the three-way match ties the purchase order, the goods
  receipt, and the invoice together — payment happens only when all three agree.
- **Big ideas touched**: `chain-of-referencing-documents` (course 4's document-chain concept, applied);
  `control-through-matching` rather than through approval alone.

## Prerequisites

- **Prior topics**: [`erp-subledger-to-gl-architecture`](./erp-subledger-to-gl-architecture.md).
- **Cross-domain prerequisites**: none.
- **Assumed knowledge**: course 6's subledger/GL and control-account vocabulary; course 5's
  account-determination vocabulary (GR/IR clearing account specifically).

## Accuracy notes (pending A4 verification)

> The Phase 1.2a confirmation pass has not yet run for this course.

- Worked P2P example uses an originally-authored vendor, item, and pricing dataset.

## Concepts

- **co-01 · purchase-requisition** — the internal request that initiates a procurement, before a
  vendor is committed.
- **co-02 · purchase-order** — the committed order to a specific vendor, referencing the requisition.
- **co-03 · goods-receipt** — the recorded arrival of ordered goods, referencing the purchase order.
- **co-04 · invoice-receipt** — the vendor's invoice, referencing the purchase order and/or goods
  receipt.
- **co-05 · three-way-match** — PO, GR, and invoice quantities/prices must agree (within tolerance,
  deep dive: course 12) before payment is released.
- **co-06 · vendor-master** — the vendor's identity, payment terms, and banking details, referenced by
  every document in the chain.
- **co-07 · payment-terms** — net-due dates, early-payment discounts, and how they interact with the
  invoice-receipt date.
- **co-08 · gr-ir-clearing-account** — the intermediate account that ties the goods-receipt posting to
  the invoice-receipt posting (from course 5).
- **co-09 · match-failure-as-control-event** — a three-way-match failure is not an error to suppress;
  it is the control working as designed.

## Worked examples

Prose-based worked scenarios with originally-authored data (no runnable code). Every example cites
the `co-NN` it exercises.

### Beginner

- **ex-01 · pr-to-po-trace** — trace a purchase requisition into its resulting purchase order,
  verifying the reference link. (co-01, co-02)
- **ex-02 · gr-posting-trace** — post a goods receipt for the ex-01 order, verifying the GR/IR clearing
  entry from course 5. (co-03, co-08)

### Intermediate

- **ex-03 · three-way-match-pass** — given matching PO, GR, and invoice quantities/prices, verify the
  match passes and payment can proceed. (co-04, co-05)
- **ex-04 · three-way-match-fail** — given an invoice quantity that does not match the GR, verify the
  match fails and payment is blocked. (co-05, co-09)
- **ex-05 · payment-terms-application** — given payment terms with an early-payment discount, compute
  the payable amount at two different payment dates. (co-07)

### Advanced

- **ex-06 · end-to-end-p2p-trace** — trace one procurement scenario fully from requisition through
  payment, citing every concept above in sequence. (co-01–co-08)

## Synthesis exercise — intra-topic

> Analysis and design only — never build, implement, or deploy a system (`A6`).

- **Goal**: design a full P2P scenario (requisition through payment) for a new vendor relationship,
  including a deliberate three-way-match failure and its resolution path.
- **Concepts exercised**: [ ] the PR→PO→GR→IR chain (co-01–co-04) [ ] three-way match (co-05) [ ]
  match failure as control (co-09).
- **Ordered steps**: 1) draft the requisition and PO; 2) draft the GR; 3) draft an invoice with one
  intentional mismatch; 4) show the match fails; 5) describe the resolution path (never an
  implementation).
- **Acceptance criteria**: the chain's references are all correct; the mismatch is realistic; the
  match failure is correctly attributed.
- **Done bar**: a written worked scenario, no code, no system built.

## Read more

> Populated during the Phase 1.2a `web-researcher` confirmation pass (`A12`) — coverage-check
> citations only.

## In which paths

- `skills/conventional-erp` — Stage A, course 10 of 27.
- `skills/sharia-erp` — Stage A, course 10 of 30.

---

← Back to the [syllabus index](../README.md)
