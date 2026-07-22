# Accrual Accounting and Revenue Recognition (By Example)

**Course ID**: `accrual-accounting-and-revenue-recognition` · **Format**: By Example.

**Short summary**: When revenue is recognised under the modern five-step model, as distinct from when
cash changes hands.

**Scope note**: cash-vs-accrual basis, the five-step revenue recognition model, variable consideration,
expense-side matching, and the same recognition question asked of obligations whose amount or timing is
uncertain — provisions and contingencies. This course decides **when** revenue is recognised and
**when** an uncertain obligation becomes a recognised liability rather than a note disclosure; #7
covers the operational cash-collection cycle around it.

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
  rare case of alignment this corpus notes precisely because divergence is more often the norm. Also
  `estimation-under-uncertainty` — a provision puts a single reported number on an obligation whose
  amount nobody yet knows, which is the same discipline #7's allowance and #11's impairment apply to
  the asset side.

## Prerequisites

- **Prior courses**: `journal-entries-and-posting-mechanics` (#4).
- **Assumed knowledge**: #4's posting mechanics.

## Accuracy notes

- The ASC 606 / IFRS 15 five-step model's structure is stable, widely documented domain knowledge
  `[Judgment call — the model's structure is cited generically; no clause or paragraph is reproduced
from either standard per A8]`.
- Provisions and contingencies (co-09 through co-12) are cited by standard name only — IAS 37 under
  IFRS, ASC 450 under US GAAP — with no clause text, threshold wording, or numbering layout reproduced,
  per A8. The recognition ladder as taught here, and the statement that the two frameworks' likelihood
  wording is not identical, are domain reasoning rather than claims sourced from this plan's grounding
  file `[Needs Verification]` pending the Phase 1 coverage pass; neither framework's actual threshold
  wording is stated, and no threshold is presented as the universal one.

## Concepts

- **co-01 · cash-basis-vs-accrual-basis** — cash basis records a transaction when cash moves; accrual
  basis records it when the economic event occurs.
- **co-02 · five-step-model-overview** — identify the contract; identify performance obligations;
  determine the transaction price; allocate the price; recognise revenue as obligations are satisfied
  — restated in original words, no standard text reproduced.
- **co-03 · performance-obligation** — a promise in a contract that is distinct enough to account for
  separately.
- **co-04 · transaction-price-allocation** — splitting one contract price across multiple performance
  obligations.
- **co-05 · variable-consideration** — a price that depends on a future outcome (bonuses, refunds,
  discounts) and how it is estimated for recognition purposes.
- **co-06 · multi-period-contract-recognition** — recognising revenue over a performance period rather
  than at a single point, for a contract that spans periods.
- **co-07 · matching-principle** — expenses are recognised in the same period as the revenue they
  helped generate, the revenue side's mirror image.
- **co-08 · expense-accrual** — recognising an expense before cash is paid, when the obligating event
  has already occurred.
- **co-09 · provision** — co-08 extended to an obligation whose timing or amount is uncertain: a past
  event has already created it, an outflow is expected, and the amount can be estimated confidently
  enough to report a single figure.
- **co-10 · recognition-ladder-for-uncertain-obligations** — the same obligation is recognised on the
  balance sheet, disclosed in the notes only, or left out entirely, and which rung it lands on is
  decided by how likely the outflow is rather than by how large it would be. The likelihood wording is
  not identical between IFRS and US GAAP, so identical facts can land on different rungs under the two
  frameworks (see the Accuracy note above for what is and is not asserted here).
- **co-11 · contingent-asset-asymmetry** — an uncertain inflow is not treated as the mirror of an
  uncertain outflow: an expected gain waits for more certainty than an expected loss does, so the two
  parties to one disputed claim do not account for it symmetrically.
- **co-12 · provision-remeasurement** — a provision is re-estimated each period as information
  improves, with the change taken through the current period's profit rather than by restating the
  entry that first recognised it.

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
- **ex-10 · place-three-obligations-on-the-ladder** — expected warranty claims on goods already sold, a
  lawsuit whose outcome is genuinely open, and a regulatory claim nobody expects to succeed — verify
  each lands on a different rung (recognised, disclosed only, neither) and that the deciding factor is
  the likelihood of outflow, not the size of the amount at stake. (co-09, co-10)
- **ex-11 · remeasure-a-provision** — re-estimate ex-10's warranty provision in the following period as
  actual claim experience arrives — verify the change lands in that period's profit and that the
  original recognition entry is left standing rather than rewritten. (co-09, co-12)
- **ex-12 · both-sides-of-one-dispute** — the same claim seen from the defendant's side (an expected
  outflow) and the claimant's side (an expected inflow) — verify the two are not recognised on the same
  terms, and state which party reports a number and which reports only a description. (co-11)
- **ex-13 · unprovided-obligation-failure** — an obligation whose outflow is expected but whose amount
  is awkward to estimate, left off the balance sheet and out of the notes entirely — verify every
  posted entry is correct and the trial balance foots while liabilities are understated, and name the
  observable signal (a settlement paid in a later period against no provision ever recognised) that
  would reveal it. (co-09, co-10, silent-failure)

## Applied synthesis (no build — A6)

Take one multi-period, multi-obligation contract by hand through the full five-step model to a
period-by-period recognition schedule, then compare it against a naive "recognise on invoice"
treatment of the same contract. Verify the two schedules diverge, and identify exactly which period's
income statement the naive treatment misstates. No system is built — the synthesis is the two
hand-worked schedules and their comparison.

## Read more

- **IFRS Foundation — IFRS 15 Revenue from Contracts with Customers** (ifrs.org). The IFRS Foundation
  publishes its **own** free teaching materials for classroom use by recognised institutions under
  attribution and non-commercial terms; **the Standards text itself still requires a separate licence
  to reproduce** `[Verified]`; named nominatively, no clause text reproduced.
- **FASB Accounting Standards Codification — Topic 606** (fasb.org). FASB's codification is closed
  copyright (see [tech-docs §Licensing](../../tech-docs.md#licensing-and-ip-compliance-a8)); named
  nominatively only.

## In which paths

- `conventional-accounting` — Stage 2 · Most conventional systems a mid-size company runs, plus how to
  architect (not build) a ledger system.
- `sharia-accounting` — Stage 2 · same; the shared spine both paths cover identically.

---

← Back to the [syllabus index](../README.md)
