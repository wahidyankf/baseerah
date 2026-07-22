# Accrual Accounting and Revenue Recognition (By Example)

**Course ID**: `accrual-accounting-and-revenue-recognition` · **Format**: By Example.

**Short summary**: When revenue is recognised under the modern five-step model, as distinct from when
cash changes hands.

**Scope note**: cash-vs-accrual basis, the five-step revenue recognition model, variable consideration,
and expense-side matching — this course decides **when** revenue is recognised; #7 covers the
operational cash-collection cycle around it.

## Why this exists · the big idea

- **The problem before the solution**: systems builders default to cash-basis intuitions ("charge the
  card, record the revenue") that hold for a single-period, single-delivery sale and break for
  anything else — a multi-period contract, a bundled deliverable, a subscription.
  This is the corpus's first fully worked instance of "the trial balance foots and the income
  statement is wrong."
- **Keep-this-if-you-forget-everything**: revenue is recognised as performance obligations are
  satisfied, not as cash arrives and not as a contract is signed.
- **Big ideas touched**: `silent-failure` (the formal worked demonstration below) and
  `standard-plurality` — ASC 606 (US GAAP) and IFRS 15 converged to the same five-step structure, a
  rare case of alignment this corpus notes precisely because divergence is more often the norm.

## Prerequisites

- **Prior courses**: `journal-entries-and-posting-mechanics` (#4).
- **Assumed knowledge**: #4's posting mechanics.

## Accuracy notes

- The ASC 606 / IFRS 15 five-step model's structure is stable, widely documented domain knowledge
  `[Judgment call — the model's structure is cited generically; no clause or paragraph is reproduced
from either standard per A8]`.

## Concepts

1. **co-01 · cash-basis-vs-accrual-basis** — cash basis records a transaction when cash moves; accrual
   basis records it when the economic event occurs.
2. **co-02 · five-step-model-overview** — identify the contract; identify performance obligations;
   determine the transaction price; allocate the price; recognise revenue as obligations are satisfied
   — restated in original words, no standard text reproduced.
3. **co-03 · performance-obligation** — a promise in a contract that is distinct enough to account for
   separately.
4. **co-04 · transaction-price-allocation** — splitting one contract price across multiple performance
   obligations.
5. **co-05 · variable-consideration** — a price that depends on a future outcome (bonuses, refunds,
   discounts) and how it is estimated for recognition purposes.
6. **co-06 · multi-period-contract-recognition** — recognising revenue over a performance period rather
   than at a single point, for a contract that spans periods.
7. **co-07 · matching-principle** — expenses are recognised in the same period as the revenue they
   helped generate, the revenue side's mirror image.
8. **co-08 · expense-accrual** — recognising an expense before cash is paid, when the obligating event
   has already occurred.

## Worked examples

### Beginner

- **ex-01 · cash-vs-accrual-same-transaction** — record one sale under both cash basis and accrual
  basis — verify the two produce different period placements for the same transaction. (co-01)
- **ex-02 · identify-performance-obligations** — split a bundled contract (product plus one year of
  support) into its distinct performance obligations — verify each obligation is separately
  identifiable. (co-03)
- **ex-03 · allocate-a-transaction-price** — allocate one contract price across two obligations from
  ex-02 by relative standalone value — verify the allocated amounts sum to the original price. (co-04)

### Intermediate

- **ex-04 · recognise-over-a-performance-period** — recognise a twelve-month support obligation's
  allocated price evenly across twelve periods — verify each period's recognised amount and that the
  total across all twelve equals the allocation. (co-06)
- **ex-05 · estimate-variable-consideration** — estimate expected revenue for a contract with a
  volume-based rebate — verify the estimate is updated when actual volume becomes known. (co-05)
- **ex-06 · accrue-an-expense** — accrue a utility expense for services received but not yet billed —
  verify the accrual matches the correct period regardless of invoice timing. (co-07, co-08)
- **ex-07 · match-revenue-and-cost** — match a project's recognised revenue against its incurred cost
  in the same period — verify the matched figures both land in the same reporting period. (co-07)

### Advanced

- **ex-08 · five-step-full-walkthrough** — apply all five steps to one multi-period, multi-obligation
  contract from identification through period-by-period recognition — verify the resulting schedule.
  (co-02–co-06)
- **ex-09 · full-recognition-at-signing-failure** — recognise a twelve-month subscription's full value
  at contract signing instead of over the performance period — verify the trial balance still foots
  while every period's income statement is wrong, and name the observable signal (a revenue spike at
  signing with none in later periods) that would reveal it. (co-06, silent-failure)

## Applied synthesis (no build — A6)

Take one multi-period, multi-obligation contract by hand through the full five-step model to a
period-by-period recognition schedule, then compare it against a naive "recognise on invoice"
treatment of the same contract. Verify the two schedules diverge, and identify exactly which period's
income statement the naive treatment misstates. No system is built — the synthesis is the two
hand-worked schedules and their comparison.

## Read more

- **IFRS Foundation — IFRS 15 Revenue from Contracts with Customers** (ifrs.org). IFRS Foundation
  carries an explicit free-educational-use carve-out; named nominatively, no clause text reproduced.
- **FASB Accounting Standards Codification — Topic 606** (fasb.org). FASB's codification is closed
  copyright (see [tech-docs §Licensing](../../tech-docs.md#licensing-and-ip-compliance-a8)); named
  nominatively only.

## In which paths

- `conventional-accounting` — Stage 2 · Most conventional systems a mid-size company runs, plus how to
  architect (not build) a ledger system.
- `sharia-accounting` — Stage 2 · same; the shared spine both paths cover identically.

---

← Back to the [syllabus index](../README.md)
