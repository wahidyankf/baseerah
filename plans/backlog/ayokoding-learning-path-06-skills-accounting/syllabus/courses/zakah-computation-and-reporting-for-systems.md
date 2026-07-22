# Zakah Computation and Reporting for Systems (By Example)

**Course ID**: `zakah-computation-and-reporting-for-systems` · **Format**: By Example. **NEW course
(A9)**. Sharia-only (`sharia-accounting` manifest).

**Short summary**: Computing and reporting Zakah as its own obligation, distinct from tax.

**Scope note**: Zakah base computation, the nisab threshold, and Zakah reporting — a real domain gap
the original catalog never taught despite AAOIFI FAS 9 being `[Verified]` in the seeding research. See
[tech-docs §What changed](../../tech-docs.md#what-changed-from-the-original-twenty-course-single-path-catalog-and-why).

## Why this exists · the big idea

- **The problem before the solution**: a systems builder unfamiliar with Islamic finance might fold
  Zakah into a general "tax liability" account, treating it as a jurisdictional tax variant. It is not
  — Zakah is a religious obligation with its own base, its own threshold, and its own reporting
  requirement, computed alongside tax, never as a line item within it.
- **Keep-this-if-you-forget-everything**: Zakah is computed and reported as its own obligation, on its
  own base (zakatable assets above the nisab threshold), never folded into a tax computation even when
  both are computed for the same entity in the same period.
- **Big ideas touched**: `standard-plurality` — Zakah sits alongside, not inside, a jurisdiction's tax
  regime, and a system must model it as a structurally distinct obligation rather than a tax-code
  variant — and `form-vs-substance` — Zakah's obligation arises from asset ownership and wealth
  accumulation, a substance test, not from a transaction or income event the way most taxes are
  triggered.

## Prerequisites

- **Prior courses**: `islamic-contract-modeling-for-systems` (#21).
- **Assumed knowledge**: #21's contract types, several of which (Mudaraba, Musharaka) produce the kind
  of asset holdings a Zakah computation draws from.

## Accuracy notes

- `[Verified]` AAOIFI FAS 9 governs Zakah accounting and is this course's anchor standard. The specific
  nisab threshold value and zakatable-asset classification rules are jurisdiction- and
  institution-specific in practice `[Judgment call — this course teaches the computation pattern
generically; no specific numeric nisab value is asserted as universal, and any such value is flagged
`[Needs Verification]`pending the Phase 1 coverage pass]`.

## Concepts

1. **co-01 · zakah-as-distinct-obligation** — Zakah is a religious wealth-based obligation, structurally
   separate from tax, computed on its own base and reported on its own line.
2. **co-02 · nisab-threshold** — the minimum level of zakatable wealth below which no Zakah obligation
   arises; a threshold concept, not a flat-rate-on-everything rule.
3. **co-03 · zakatable-assets** — the categories of assets subject to Zakah (e.g. cash, trade
   inventory, certain investments), distinct from the entity's full asset base.
4. **co-04 · zakah-base-computation** — computing the net zakatable base (zakatable assets less
   qualifying deductions) that the Zakah rate is applied to.
5. **co-05 · zakah-liability-recognition** — recognising the computed Zakah obligation as a liability
   in the period it is determined, following the same recognition discipline #5 taught for revenue.
6. **co-06 · zakah-fund-disclosure** — reporting Zakah separately in the financial statements (often as
   a dedicated disclosure or fund), never merged into a general tax-expense line.
7. **co-07 · entity-vs-individual-zakah-payer** — an institution's own Zakah obligation (as a payer) is
   distinct from its role, where applicable, in facilitating individual shareholders' or depositors'
   own Zakah obligations — the two are not the same computation.

## Worked examples

### Beginner

- **ex-01 · classify-zakatable-assets** — from a stated balance sheet, classify which assets are
  zakatable and which are not — verify the classification against co-03's category definitions. (co-03)
- **ex-02 · apply-the-nisab-threshold** — given a computed zakatable-asset total, determine whether it
  clears a stated nisab threshold — verify no obligation arises below the threshold. (co-02)

### Intermediate

- **ex-03 · compute-the-zakah-base** — compute a net zakatable base from ex-01's classified assets less
  stated qualifying deductions — verify the base figure. (co-04)
- **ex-04 · recognise-the-zakah-liability** — recognise the computed Zakah obligation from ex-03 as a
  liability in the correct period — verify it is recognised in the period determined, not deferred.
  (co-05)
- **ex-05 · separate-zakah-from-tax-computation** — compute both a tax liability and a Zakah liability
  for the same entity and period from overlapping underlying data — verify the two are reported as
  separate line items, neither absorbed into the other. (co-01, co-06)

### Advanced

- **ex-06 · distinguish-entity-and-individual-zakah** — for an institution facilitating individual
  depositors' Zakah alongside its own institutional Zakah obligation — verify the two computations use
  different bases and are not merged into one figure. (co-07)
- **ex-07 · zakah-folded-into-tax-failure** — a system that computes Zakah correctly but reports it as
  a line item within the tax-expense account instead of its own disclosure — verify every individual
  figure is numerically correct while the statement's presentation misrepresents Zakah as a tax
  variant, and name the observable signal (no separate Zakah disclosure despite a nonzero computed
  obligation) that would reveal it. (co-06, silent-failure)

## Applied synthesis (no build — A6)

Take one entity's balance sheet by hand through zakatable-asset classification, nisab-threshold
testing, base computation, and liability recognition, producing a Zakah figure reported separately from
that same entity's tax liability for the same period. Verify the Zakah base, the liability recognised,
and that neither figure is merged with the other. No system is built — the synthesis is the hand-worked
computation and the separated reporting.

## Read more

- **AAOIFI — FAS 9: Zakah** (aaoifi.com). Named nominatively as this course's anchor standard; no
  standard text reproduced (AAOIFI is free to read but treated as closed to reproduction).
- **Fiqh az-Zakat** — Yusuf al-Qaradawi (English translation widely available). A standard reference
  work on Zakah jurisprudence; named nominatively for the reader who wants the doctrinal foundation
  this course does not itself adjudicate.

## In which paths

- `sharia-accounting` — Stage 3 · Full competence, including how to architect (not build) a
  Sharia-compliant ledger.

---

← Back to the [syllabus index](../README.md)
