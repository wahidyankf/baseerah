# Sharia Ledger System Architecture (By Example)

**Course ID**: `sharia-ledger-system-architecture` · **Format**: By Example. **NEW course, replacing
`capstone-sharia-compliant-ledger` (A6).** Sharia-only (`sharia-accounting` manifest).

**Short summary**: How a Sharia-compliant ledger system extends #19's general-ledger architecture —
contract-type-aware posting, Zakah fund isolation, and sukuk-holder reporting — described, not built.

**Scope note**: architecture only, extending #19's general architecture with Sharia-specific
concerns. This course describes the additional structure a Sharia-compliant system needs; it does not
ask the reader to build, scaffold, or extend one (A6). No separate linked SWE edge — the
`backend-essentials` edge already reaches this course through #19.

## Why this exists · the big idea

- **The problem before the solution**: #19 architected a general ledger correctly, but a
  Sharia-compliant system needs more than a correctly architected conventional ledger with Islamic
  contract types bolted on — it needs Zakah tracked as its own fund (#22), sukuk-holder positions
  reported distinctly from conventional liabilities (#23), and contract-type classification enforced
  structurally, not left to a data-entry convention someone might forget.
- **Keep-this-if-you-forget-everything**: a Sharia-compliant architecture must make it structurally
  hard to post a disguised-loan transaction as if it were a compliant trade — the substance test from
  #21 has to live in the system's posting rules, not only in a training document.
- **Big ideas touched**: `subledger-to-ledger-integrity`, extended to Sharia-specific subledgers (a
  Zakah fund, a sukuk-holder register) — and `form-vs-substance`, this time as an architectural
  enforcement question rather than a classification exercise: how does the system itself make a
  substance violation visible rather than merely possible to avoid.

## Prerequisites

- **Prior courses**: `islamic-contract-modeling-for-systems` (#21),
  `general-ledger-system-architecture` (#19).
- **Assumed knowledge**: #19's architectural patterns (posting engine, subledger integration,
  reconciliation architecture), #21's contract-type mechanics, #22's Zakah computation, #23's sukuk
  structures.

## Accuracy notes

- Sharia-specific ledger-architecture patterns are domain-standard software-architecture reasoning
  applied to the `[Verified]` AAOIFI contract types and standards already cited in #20–#23
  `[Judgment call — no single canonical textbook source for the architecture patterns themselves]`.
- The profit-sharing-account-holder subledger (co-08) is architectural reasoning extending #21's
  `[Verified]` Mudaraba contract type; the balance-sheet presentation such balances receive under any
  particular standards ecosystem is **not** asserted here `[Needs Verification]` pending the Phase 1
  coverage pass.
- **Relationship to `DD-15`**: where this course names real software, the same precedent from #19
  applies — permissively licensed examples (e.g. Apache Fineract, whose configurable product
  framework is flexible enough to model Islamic-finance products `[Needs Verification]`) are named
  directly; copyleft projects are described behaviourally only, per
  [tech-docs §Licensing](../../tech-docs.md#licensing-and-ip-compliance-a8).

## Concepts

- **co-01 · contract-type-aware-posting** — extending #19's posting engine so each entry carries an
  explicit contract-type classification (Murabaha, Mudaraba, Musharaka, Salam, Istisnaa, or
  conventional), rather than inferring type from account codes alone.
- **co-02 · substance-validation-at-posting-time** — structurally checking a transaction's
  asset-risk-transfer and asset-backing characteristics (#21's `form-vs-substance` test) at the point
  of posting, not only in a later review.
- **co-03 · zakah-fund-isolation** — architecting #22's Zakah obligation as a structurally separate
  fund/ledger, never a sub-account of the general tax-liability structure.
- **co-04 · sukuk-holder-register** — a subledger tracking sukuk-holder positions and their
  asset/usufruct-based distributions, architecturally distinct from a conventional bondholder
  liability register.
- **co-05 · sharia-board-audit-trail** — architecting the system so a Sharia supervisory board's
  product/transaction approvals are recorded and linked to the transactions they govern, extending
  #15's audit-trail concept to a Sharia-specific governance layer.
- **co-06 · dual-manifest-shared-architecture** — the same posting-engine and reconciliation
  architecture (#19) serves both a conventional-only deployment and a Sharia-compliant deployment; the
  Sharia-specific layers (co-01 through co-05) are additive, not a fork of the core architecture.
- **co-07 · reconciliation-across-sharia-subledgers** — extending #19's reconciliation architecture so
  the Zakah fund and sukuk-holder register each tie back to their own control accounts, exactly as
  #19 required for conventional subledgers.
- **co-08 · profit-sharing-account-holder-subledger** — balances placed with the institution under a
  profit-sharing arrangement (#21's Mudaraba) are architecturally neither a fixed-return liability nor
  owners' equity, so they need their own subledger carrying each holder's share of profit actually
  realised — a structure a conventional deposit subledger, which accrues a contracted return
  regardless of outcome, cannot represent.

## Worked examples

### Beginner

- **ex-01 · tag-a-posting-with-contract-type** — sketch (on paper, no code) how a journal entry carries
  an explicit contract-type field alongside its accounts and amounts — verify the field distinguishes
  a Murabaha entry from a conventional loan entry at the data level. (co-01)
- **ex-02 · sketch-the-zakah-fund-boundary** — sketch the Zakah fund as a structurally separate ledger
  from the general tax-liability account — verify no shared account code between the two. (co-03)

### Intermediate

- **ex-03 · design-a-substance-check** — design (on paper) a posting-time check that would have caught
  #21's ex-09 disguised-loan failure (a murabaha-labelled entry with no genuine asset-risk transfer) —
  verify the check's criteria matches #21's co-08 asset-risk-transfer-timing test.
  (co-02, `form-vs-substance`)
- **ex-04 · design-the-sukuk-holder-register** — design a subledger structure for sukuk-holder
  positions, distinct from #19's conventional liability subledger — verify it can represent an
  asset/usufruct-based distribution rather than a fixed-interest schedule. (co-04)
- **ex-05 · link-a-sharia-board-approval** — design how a Sharia board's approval record links to the
  specific product or transaction type it governs — verify a transaction posted under an unapproved
  contract type would be structurally flagged. (co-05)
- **ex-06 · reconcile-the-zakah-fund** — design a reconciliation check comparing the Zakah fund's
  computed balance (per #22) against its control account — verify the check would catch #22's ex-07
  zakah-folded-into-tax failure. (co-07)

### Advanced

- **ex-07 · extend-19-not-fork-it** — compare a hypothetical forked Sharia-only architecture against
  this course's additive-layer design — verify the additive design reuses #19's posting engine and
  reconciliation architecture unchanged, while a fork would duplicate both. (co-06)
- **ex-08 · missing-substance-check-failure** — a system with contract-type tagging (co-01) but no
  posting-time substance validation (co-02) — verify a disguised-loan transaction can still be tagged
  "Murabaha" and posted without structural objection, and name the missing architectural control that
  would prevent it. (co-02, silent-failure, `form-vs-substance`)
- **ex-09 · contrast-two-holder-subledger-shapes** — on paper, contrast a conventional deposit
  subledger (a per-holder balance plus a contracted return accrued each period regardless of outcome)
  against a profit-sharing account-holder subledger (a per-holder balance plus a share of profit only
  once realised) — verify the conventional shape has no field in which "profit not yet realised" can
  be represented, and name the reconciliation check (co-07) each shape would need. (co-06, co-07,
  co-08)

## Applied synthesis (no build — A6)

On paper, extend #19's applied-synthesis subledger design (a new expense-reimbursement subledger) with
one Sharia-specific concern: design how a new Sukuk-holder register would integrate into the same
posting engine and reconciliation architecture, naming its control account, its substance-validation
check, and its Sharia-board audit-trail linkage. Verify the design reuses #19's architecture rather
than forking it. No system is built, no code is scaffolded — the synthesis is the architectural design
document itself.

## Read more

- **Apache Fineract** (Apache-2.0) — a real, permissively licensed reference implementation whose
  configurable product framework is flexible enough to model Islamic-finance products
  `[Needs Verification]` — Islamic-banking support is **not** asserted here as a documented Fineract
  feature; named nominatively per `DD-15`, never quoted from.
- **Designing Data-Intensive Applications** — Martin Kleppmann (O'Reilly). Cited nominatively, as in
  #19, for the reconciliation and architectural-invariant reasoning applied here to Sharia-specific
  subledgers.

## In which paths

- `sharia-accounting` — Stage 3 · Full competence, including how to architect (not build) a
  Sharia-compliant ledger. Terminal course for this manifest.

---

← Back to the [syllabus index](../README.md)
