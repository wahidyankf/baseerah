# Accounts Payable and Procure-to-Pay (By Example)

**Course ID**: `accounts-payable-and-procure-to-pay` · **Format**: By Example.

**Short summary**: The procure-to-pay (P2P) document chain and the accounting entries it produces.

**Scope note**: purchase requisition through payment as a document chain, the three-way match, accrued
liabilities, and payment-term discounts — the accounting entries the cycle produces, not the ERP
workflow or approval-routing that generates them (a separate, ERP-owned systems-architecture concern).

## Why this exists · the big idea

- **The problem before the solution**: P2P is the first of the two operational cycles (the other is
  order-to-cash, #7) that most systems this corpus's readers will actually build touch directly — get
  the three-way match wrong and a system can pay for goods it never received while every number still
  balances.
- **Keep-this-if-you-forget-everything**: a balancing ledger is not a control — the three-way match is
  the control, and skipping it is a control failure even when the books still balance.
- **Big ideas touched**: `form-vs-substance` — a purchase order and an invoice are the **form**; a
  completed three-way match verifies the economic **substance** (goods actually received) before a
  liability is booked as payable.

## Prerequisites

- **Prior courses**: `journal-entries-and-posting-mechanics` (#4).
- **Assumed knowledge**: #4's posting mechanics.

## Accuracy notes

- The P2P cycle and three-way match are standard, widely used domain terminology with no dynamic
  component to re-verify at authoring `[Verified — stable, non-dynamic domain fact]`.
- The goods-received-not-invoiced clearing pattern (co-08) is domain reasoning about how co-02's match
  is carried in the accounts between receipt and invoice, not a claim sourced from this plan's
  grounding file `[Needs Verification]` pending the Phase 1 coverage pass.

## Concepts

- **co-01 · procure-to-pay-cycle** — purchase requisition, purchase order, goods receipt, invoice,
  payment, as a document chain.
- **co-02 · three-way-match** — matching the purchase order, the goods receipt, and the invoice before
  a liability is approved for payment.
- **co-03 · match-exception** — a mismatch between any two of the three documents (quantity, price)
  that must be resolved before payment, not silently accepted.
- **co-04 · accrued-liability** — goods or services received but not yet invoiced, recognised as a
  liability at period end regardless of invoice timing.
- **co-05 · payment-terms** — the agreed timing and conditions for payment (e.g. net-30), and how they
  shape when a liability is settled.
- **co-06 · early-payment-discount** — the accounting for a discount taken (or missed) for paying
  before the due date.
- **co-07 · liability-without-match** — booking a liability from an invoice alone, without a completed
  three-way match, as this course's headline control failure.
- **co-08 · goods-received-not-invoiced-clearing** — the intermediate account a goods receipt credits
  and the matching supplier invoice later debits, so a receipt still awaiting its invoice and an
  invoice still awaiting its receipt each leave a residual balance that names which side of the match
  is missing.

## Worked examples

### Beginner

- **ex-01 · walk-the-p2p-chain** — trace one purchase from requisition through payment across all five
  document stages — verify each stage's document references the prior one. (co-01)
- **ex-02 · post-a-purchase-order** — record a purchase order committing to a future receipt — verify
  no liability is booked yet at this stage. (co-01)
- **ex-03 · post-a-goods-receipt** — record receipt of the ordered goods — verify quantity received
  matches the purchase order. (co-01, co-02)

### Intermediate

- **ex-04 · perform-a-three-way-match** — match a purchase order, goods receipt, and invoice that agree
  on quantity and price — verify the match passes and the liability is booked. (co-02)
- **ex-05 · catch-a-match-exception** — an invoice priced higher than the purchase order — verify the
  mismatch is flagged and the liability is held pending resolution rather than booked as invoiced.
  (co-03)
- **ex-06 · accrue-an-unbilled-receipt** — goods received at period end with no invoice yet — verify an
  accrued liability is booked and later reversed when the real invoice arrives. (co-04)
- **ex-07 · take-an-early-payment-discount** — pay within the discount window — verify the discount
  reduces the recorded expense rather than being treated as separate income. (co-05, co-06)
- **ex-08 · miss-an-early-payment-discount** — pay after the discount window — verify the full invoice
  amount is paid with no discount applied. (co-05, co-06)

### Advanced

- **ex-09 · full-p2p-cycle-with-accrual** — a purchase spanning a period boundary: order, receipt
  before period end, invoice after — verify the accrual at period end and its reversal on invoice
  receipt net to the correct total liability. (co-01, co-04)
- **ex-10 · liability-booked-without-match** — book a liability directly from an invoice, skipping the
  three-way match, where the goods were never actually received — verify the trial balance still
  foots while the entity has paid for goods it does not have, and name the control (the three-way
  match) that would have caught it. (co-07, silent-failure)
- **ex-11 · reconcile-the-received-not-invoiced-balance** — at period end one receipt has arrived with
  no invoice yet and one invoice has arrived with no matching receipt — verify the clearing account's
  residual balance decomposes into exactly those two items, and that neither is visible from the
  accounts-payable balance alone. (co-02, co-04, co-08)

## Applied synthesis (no build — A6)

Trace one purchase from requisition through payment by hand, including one deliberate quantity
mismatch between the goods receipt and the invoice. Verify the mismatch is caught by the three-way
match before payment, and separately verify what would have happened — a liability booked for goods
not received — had the match been skipped. No system is built — the synthesis is the two hand-traced
outcomes and their comparison.

## Read more

- **Accounting Information Systems** — Romney & Steinbart (Pearson). A standard textbook covering the
  procure-to-pay cycle and internal controls including the three-way match; cited nominatively.

## In which paths

- `conventional-accounting` — Stage 2 · Most conventional systems a mid-size company runs, plus how to
  architect (not build) a ledger system.
- `sharia-accounting` — Stage 2 · same; the shared spine both paths cover identically.

---

← Back to the [syllabus index](../README.md)
