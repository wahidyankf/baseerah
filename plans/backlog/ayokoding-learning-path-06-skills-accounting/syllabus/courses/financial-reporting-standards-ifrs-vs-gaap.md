# Financial Reporting Standards: IFRS vs. GAAP (Annotated-concept)

**Course ID**: `financial-reporting-standards-ifrs-vs-gaap` · **Format**: Annotated-concept.

**Short summary**: Where IFRS and US GAAP converge, where they genuinely diverge, and why a systems
builder must model the divergence rather than assume one universal standard.

**Scope note**: this course is the corpus's headline treatment of `standard-plurality` — a landscape
and judgment framework, not a mechanism the reader executes. It names the specific, stable divergences
already introduced piecemeal in earlier courses (LIFO at #10, lease classification detail at #11) and
adds the ones not yet covered, without re-deriving any of them from scratch.

## Why this exists · the big idea

- **The problem before the solution**: a systems builder who assumes "there is one correct number"
  will misdesign any system meant to report under more than one standard, or to a cross-listed
  audience — the correct architecture models the standard as a first-class dimension, not an
  afterthought.
- **Keep-this-if-you-forget-everything**: divergence is not sloppiness on either standard's part — it
  reflects different, defensible judgment calls (e.g. principles-based vs. more rule-detailed
  guidance) that a systems builder must surface, never silently resolve by picking one.
- **Big ideas touched**: `standard-plurality` — this course is where the theme, seeded at #5 and #10,
  becomes the explicit subject.

## Prerequisites

- **Prior courses**: `accrual-accounting-and-revenue-recognition` (#5),
  `lease-and-intangible-asset-accounting` (#11).
- **Assumed knowledge**: #5's revenue-recognition model, #11's lease classification.

## Accuracy notes

- The IASB/FASB convergence project (the 2002 Norwalk Agreement and its aftermath) and the resulting
  full alignment of the five-step revenue model are documented history
  `[Web-cited: IFRS Foundation — Norwalk Agreement (2002 IASB/FASB Memorandum of Understanding) — https://www.ifrs.org/content/dam/ifrs/around-the-world/mous/norwalk-agreement-2002.pdf ; accessed 2026-07-22]`.
- The LIFO prohibition under IFRS (co-03) restates #10's already-`[Verified]` fact.
- The lessee single-model-vs-dual-model lease divergence (co-04), the inventory-writedown-reversal
  divergence (co-05), and the development-cost-capitalisation divergence (co-06) are well-documented,
  stable cross-standard differences, with the relevant standards (IFRS 16 vs. ASC 842 for leases,
  IAS 2 for inventory-writedown reversal, IAS 38 for development-cost capitalisation) named
  nominatively and no clause text reproduced per A8
  `[Verified — stable, well-documented cross-standard divergences; no standard text sourced]`.
- "Principles-based vs. rules-based" (co-07) is a common characterization of the two standards, not a
  precise technical claim, and is presented as such.

## Concepts

- **co-01 · two-standard-setters** — the IASB issues IFRS; the FASB issues US GAAP; neither has
  authority over the other, and a reporting entity's jurisdiction (or listing requirements) determines
  which applies.
- **co-02 · convergence-history** — the IASB and FASB pursued an explicit convergence project from the
  early 2000s; some areas (the five-step revenue model, #5) fully aligned, others did not.
- **co-03 · lifo-divergence** — LIFO is GAAP-permitted, IFRS-forbidden (restates #10's `[Verified]`
  fact at the corpus's explicit standard-plurality course).
- **co-04 · lease-model-divergence** — IFRS 16 uses a single on-balance-sheet lessee model with no
  operating/finance distinction; ASC 842 retains the operating/finance distinction for lessees even
  though both are on-balance-sheet, producing different expense patterns for an otherwise identical
  lease.
- **co-05 · inventory-writedown-reversal-divergence** — IFRS permits reversing a prior
  lower-of-cost-or-net-realisable-value writedown if value recovers; GAAP prohibits reversal.
- **co-06 · development-cost-capitalisation-divergence** — IFRS requires capitalising qualifying
  development costs once specific criteria are met; GAAP generally expenses research and development
  as incurred, with narrower exceptions (e.g. certain software costs, #18's subject).
- **co-07 · principles-based-vs-rules-based-characterization** — IFRS is commonly characterized as
  more principles-based, GAAP as more detailed and rules-based; a common characterization, not a
  precise technical boundary, and useful mainly as a lens for why new situations get resolved
  differently under each.
- **co-08 · dual-reporting-need** — a cross-listed or dual-jurisdiction entity may need to reconcile
  or dual-report figures, and the divergences above are exactly where that reconciliation work
  concentrates.

## Tensions & trade-offs — when a systems builder must model the divergence explicitly

- **One ledger, two standards**: an entity reporting under both frameworks (e.g. a US subsidiary of an
  IFRS parent) cannot simply pick one set of rules — the same underlying transaction data must support
  two different, individually correct reporting views, which pushes the standard itself into the data
  model rather than treating it as a report-time formatting choice.
- **When NOT to hard-code one standard**: any system whose readers might ever be dual-reporting,
  cross-listed, or acquired by an entity under the other standard should not assume its chosen standard
  is permanent — hard-coding GAAP-only or IFRS-only logic into core posting rules (rather than into a
  reporting layer) is the architecture mistake this course exists to prevent.

## Worked examples

Grouped by theme; no fixed Beginner/Intermediate/Advanced bands (Annotated-concept). Every example
cites the `co-NN` it exercises, and a "verify" clause means recompute by hand against a stated figure.

### Theme A · Naming the divergences

- **ex-01 · lifo-under-each-standard** — determine whether a stated inventory-costing method is
  available to a GAAP-reporting entity and to an IFRS-reporting entity — verify the answer differs
  between the two. (co-03)
- **ex-02 · lease-expense-pattern-contrast** — compute a lessee's period expense for the identical
  lease under IFRS 16's single model and under ASC 842's finance-lease classification — verify the two
  produce different period-by-period expense patterns despite an identical underlying lease. (co-04)
- **ex-03 · writedown-reversal-contrast** — an inventory item written down, then its net realisable
  value recovers — verify the reversal is permitted under IFRS and prohibited under GAAP for the
  identical fact pattern. (co-05)

### Theme B · Where the divergence changes a system's design

- **ex-04 · design-a-dual-standard-flag** — sketch (at a conceptual level, no code) how a system could
  carry a per-entity or per-report reporting-standard flag that changes which of co-03–co-06's rules
  apply at reporting time — verify the flag's effect is isolated to the reporting layer, not scattered
  through posting logic. (co-08, `subledger-to-ledger-integrity`)
- **ex-05 · reconciliation-schedule** — build a reconciliation schedule showing the same entity's net
  income under both standards, with the LIFO and lease divergences as the two reconciling items —
  verify the reconciliation ties both figures back to the same underlying transactions. (co-08)

### Theme C · The silent failure — a plausible number produced under the wrong standard

- **ex-06 · reversal-taken-under-the-wrong-standard** — a US-GAAP-reporting entity writes inventory
  down, its net realisable value later recovers, and the recovery is reversed the way co-05 records
  IFRS as permitting — verify every entry balances, the trial balance foots, and the restored carrying
  amount is entirely plausible on its face (it never rises above original cost), so nothing in the
  arithmetic marks the figure as inadmissible under the standard the entity actually reports under;
  name the observable signal (a writedown reversal appearing in a filing prepared on a US GAAP basis)
  that would reveal it, and state which layer the reversal rule belongs in per this course's
  tensions section. (co-05, co-08, silent-failure)

## Applied synthesis (no build — A6)

Take one entity's set of transactions (including one LIFO-eligible inventory decision and one lease)
by hand through both an IFRS-reporting and a GAAP-reporting lens, producing two divergent net-income
figures and a reconciliation schedule explaining the difference. Verify the reconciliation accounts
for exactly the divergences named in this course. No system is built — the synthesis is the two
hand-worked reporting views and their reconciliation.

## Read more

- **IFRS Foundation — IFRS/GAAP comparison resources** (ifrs.org). The IFRS Foundation publishes its
  **own** free teaching materials for classroom use by recognised institutions under attribution and
  non-commercial terms; **the Standards text itself still requires a separate licence to reproduce**
  `[Verified]`; named nominatively, no clause text reproduced.
- **FASB Accounting Standards Codification** (fasb.org). FASB's codification is closed copyright; named
  nominatively only.
- **International Financial Reporting Standards vs. US GAAP** — Ernst & Young / KPMG comparison guides
  (publicly available, published by the respective firms). Named nominatively as a corroboration
  source for coverage-checking under A12, never transcribed.

## In which paths

- `conventional-accounting` — Stage 2 · Most conventional systems a mid-size company runs, plus how to
  architect (not build) a ledger system.
- `sharia-accounting` — Stage 2 · same; the shared spine both paths cover identically.

---

← Back to the [syllabus index](../README.md)
