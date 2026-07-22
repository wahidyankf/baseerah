# Order-to-Cash Systems (By Example)

**Course ID**: `order-to-cash-systems` · **Format**: By Example · **Language**: — (domain, worked scenarios, no application code).

**Short summary**: The quote→SO→delivery→invoice chain, credit management, customer master

**Scope note**: the second core transaction-cycle course (mirrors procure-to-pay, course 10) — the
order-to-cash chain from quote through invoice, with credit management as its central control.
Exception handling (partials, tolerances, returns) is deferred to course 12. License-aware (DD-15).

## Why this exists · the big idea

- **The problem before the solution**: shipping goods to a customer who cannot pay, or invoicing a
  quantity that was never actually delivered, are both real risks — credit management and the
  quote→SO→delivery→invoice chain exist to prevent them.
- **Keep-this-if-you-forget-everything**: the delivery document, not the sales order, is what should
  drive the invoice quantity — invoicing off the order alone risks billing for goods never shipped.
- **Big ideas touched**: `chain-of-referencing-documents` (mirrors course 10); `credit-as-a-gate`,
  distinct from the three-way match's post-hoc verification.

## Prerequisites

- **Prior topics**: [`erp-subledger-to-gl-architecture`](./erp-subledger-to-gl-architecture.md).
- **Cross-domain prerequisites**: none.
- **Assumed knowledge**: course 6's subledger/GL vocabulary; course 5's account-determination
  vocabulary (revenue recognition account specifically).

## Accuracy notes (pending A4 verification)

> The Phase 1.2a confirmation pass has not yet run for this course.

- Worked O2C example uses an originally-authored customer, item, and pricing dataset.

## Concepts

- **co-01 · quote** — a non-binding offer to a customer, the chain's optional entry point.
- **co-02 · sales-order** — the committed order from a customer, referencing the quote where one
  exists.
- **co-03 · delivery-document** — the recorded shipment, referencing the sales order — the quantity
  that should drive invoicing.
- **co-04 · customer-invoice** — the bill to the customer, referencing the delivery (not the order
  directly).
- **co-05 · credit-management** — a gate checked before order confirmation or delivery release,
  distinct from a post-hoc match.
- **co-06 · customer-master** — the customer's identity, credit limit, and payment terms, referenced
  by every document in the chain.
- **co-07 · revenue-recognition-touchpoint** — where the O2C chain hands off to revenue-recognition
  accounting (a preview; deep accounting treatment sits in the accounting corpus, not here).
- **co-08 · order-vs-delivery-quantity-mismatch** — why invoicing off the order instead of the
  delivery risks billing for undelivered goods.
- **co-09 · credit-hold-as-control-event** — a credit hold is not an error to bypass; it is the
  control working as designed.

## Worked examples

Prose-based worked scenarios with originally-authored data (no runnable code). Every example cites
the `co-NN` it exercises.

### Beginner

- **ex-01 · quote-to-so-trace** — trace a quote into its resulting sales order, verifying the
  reference link. (co-01, co-02)
- **ex-02 · delivery-to-invoice-trace** — post a delivery for the ex-01 order and generate its invoice
  from the delivery quantity, not the order quantity. (co-03, co-04)

### Intermediate

- **ex-03 · credit-hold-trigger** — given a customer whose order would exceed their credit limit,
  verify the order is held rather than confirmed. (co-05, co-09)
- **ex-04 · order-vs-delivery-mismatch** — given a partial delivery, show what happens if the invoice
  is generated from the order quantity instead of the delivery quantity. (co-08)

### Advanced

- **ex-05 · end-to-end-o2c-trace** — trace one sales scenario fully from quote through invoice, citing
  every concept above in sequence. (co-01–co-08)

## Synthesis exercise — intra-topic

> Analysis and design only — never build, implement, or deploy a system (`A6`).

- **Goal**: design a full O2C scenario (quote through invoice) for a new customer relationship,
  including a deliberate credit-hold event and its resolution path.
- **Concepts exercised**: [ ] the quote→SO→delivery→invoice chain (co-01–co-04) [ ] credit management
  (co-05) [ ] credit hold as control (co-09).
- **Ordered steps**: 1) draft the quote and sales order; 2) set a credit limit that the order would
  exceed; 3) show the credit hold triggers; 4) describe the resolution path (never an
  implementation); 5) complete delivery and invoice once resolved.
- **Acceptance criteria**: the chain's references are all correct; the credit hold is correctly
  triggered and attributed.
- **Done bar**: a written worked scenario, no code, no system built.

## Read more

> Populated during the Phase 1.2a `web-researcher` confirmation pass (`A12`) — coverage-check
> citations only.

## In which paths

- `skills/conventional-erp` — Stage A, course 11 of 26.
- `skills/sharia-erp` — Stage A, course 11 of 29.

---

← Back to the [syllabus index](../README.md)
