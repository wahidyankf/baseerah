# Islamic Contract-Based Transaction Flows (By Example)

**Course ID**: `islamic-contract-based-transaction-flows` · **Format**: By Example · **Language**: — (domain, worked scenarios, no application code).

**Short summary**: Murabaha, Ijarah, and Musharaka/Mudaraba flow variants, contrasted with conventional

**Scope note**: the worked-example deep dive on course 28's contract-type-awareness concept — how
Murabaha, Ijarah, and Musharaka/Mudaraba each reshape the P2P/O2C document flow from courses 10-11.
License-aware (DD-15).

## Why this exists · the big idea

- **The problem before the solution**: a conventional P2P/O2C flow assumes a simple sale — a Murabaha
  cost-plus sale, an Ijarah lease, and a Musharaka profit-sharing arrangement each require a
  differently shaped document chain, and treating them as "a sale with extra fields" misses real
  structural differences.
- **Keep-this-if-you-forget-everything**: the contract type determines the document flow's shape, not
  just its labels — an Ijarah flow has recurring lease-payment documents a simple sale flow does not.
- **Big ideas touched**: `contract-type-determines-flow-shape`, applying course 4's document-chain
  concept to jurisdiction-plural content.

## Prerequisites

- **ERP prereqs**: [`procure-to-pay-systems`](./procure-to-pay-systems.md),
  [`order-to-cash-systems`](./order-to-cash-systems.md),
  [`sharia-compliant-erp-design`](./sharia-compliant-erp-design.md).
- **Assumed knowledge**: courses 10-11's chain vocabulary; course 28's jurisdictional-pluggability and
  contract-type-awareness vocabulary.

## Accuracy notes (pending A4 verification)

> The Phase 1.2a confirmation pass has not yet run for this course.

- Contract-mechanics claims carry the same `[Unverified]` status as course 28's jurisdictional table
  pending the primary-source re-verification pass; worked examples use originally-authored data and
  do not assert a specific jurisdiction's numeric detail as fact.
- What **is** corroborated (2026-07-22, coverage level only): the recognised Islamic-contract set —
  murabaha, ijarah, musharakah, mudarabah, salam, istisna and wakala — and the structural mechanics
  of each are covered by the AAOIFI Shari'ah Standards
  `[Web-cited: AAOIFI — https://aaoifi.com/?lang=en ; accessed 2026-07-22]`
  `[Web-cited: AAOIFI — Shari'ah Standards — https://aaoifi.com/shariaa-standards/?lang=en ; accessed 2026-07-22]`.
  Individual Shari'ah-Standard numbers remain `[Needs Verification]` and are not asserted; AAOIFI (not
  IFSB, which is prudential) is the contract-mechanics authority cited here.
- Concepts co-08 and co-09 are placed on domain-reasoning grounds rather than sourced from the
  grounding research, and are `[Needs Verification]` pending the Phase 1.2a coverage pass; both are
  framed as questions the document flow must answer under its jurisdictional configuration, not as a
  prescribed treatment.

## Concepts

- **co-01 · murabaha-flow** — a cost-plus sale: the seller discloses cost and markup, structurally
  distinct from a conventional sale's undisclosed margin.
- **co-02 · ijarah-flow** — a lease arrangement, producing recurring lease-payment documents rather
  than a single sale document.
- **co-03 · musharaka-mudaraba-flow** — a profit-sharing arrangement, producing periodic
  profit-distribution documents rather than a fixed-price sale.
- **co-04 · disclosed-cost-plus-markup** — Murabaha's structural requirement that cost and markup be
  disclosed, distinct from conventional pricing.
- **co-05 · recurring-payment-document-chain** — Ijarah's lease-payment documents, each referencing
  the master lease agreement.
- **co-06 · profit-distribution-document** — Musharaka/Mudaraba's periodic distribution, referencing
  the underlying profit-sharing agreement and an agreed ratio.
- **co-07 · conventional-vs-sharia-flow-contrast** — the same underlying business event (financing an
  asset purchase) produces structurally different document flows depending on contract type.
- **co-08 · overdue-amount-treatment-divergence** — a conventional flow's interest-accruing
  late-payment charge has no like-for-like counterpart here, so an overdue receivable must be modeled
  by whatever the applicable jurisdictional configuration (course 28) prescribes; the divergence is
  structural — a different document, not a renamed field on the same one. `[Needs Verification]`
- **co-09 · early-settlement-and-remaining-profit** — settling a Murabaha or Ijarah obligation ahead of
  schedule raises the question of what happens to the not-yet-earned portion of the disclosed markup
  or the remaining lease term, which the document flow must represent explicitly rather than leave
  implicit in a balance. `[Needs Verification]`

## Worked examples

Prose-based worked scenarios with originally-authored data (no runnable code). Every example cites
the `co-NN` it exercises.

### Beginner

- **ex-01 · murabaha-flow-trace** — trace a Murabaha transaction's document flow, noting where cost
  and markup are disclosed. (co-01, co-04)
- **ex-02 · ijarah-flow-trace** — trace an Ijarah transaction's document flow, noting the recurring
  lease-payment documents. (co-02, co-05)

### Intermediate

- **ex-03 · musharaka-flow-trace** — trace a Musharaka transaction's document flow, noting the
  periodic profit-distribution documents. (co-03, co-06)
- **ex-04 · conventional-vs-murabaha-contrast** — given the same asset-financing business event,
  contrast its conventional-sale document flow with its Murabaha document flow. (co-07)

### Advanced

- **ex-05 · three-way-flow-contrast** — given one underlying financing need, sketch how it would be
  structured under Murabaha, Ijarah, and Musharaka respectively, noting the structural difference each
  time. (co-01–co-03, co-07)
- **ex-06 · overdue-amount-flow-divergence** — given the same overdue receivable under a conventional
  sale flow and under a Murabaha flow, contrast how each flow's documents represent the overdue
  amount, treating the Murabaha side as configuration-driven rather than asserting any one
  jurisdiction's prescribed treatment as universal. (co-07, co-08)
- **ex-07 · early-settlement-trace** — given a Murabaha obligation settled ahead of schedule, trace
  which documents in the flow must change and identify where the not-yet-earned portion of the
  disclosed markup is represented. (co-01, co-04, co-09)

## Synthesis exercise — intra-topic

> Analysis and design only — never build, implement, or deploy a system (`A6`).

- **Goal**: given one business financing scenario, design its document flow under two different
  contract types (e.g. Murabaha and Ijarah), and write a contrast explaining the structural
  difference.
- **Concepts exercised**: [ ] two contract-type flows (co-01, co-02, or co-03) [ ] flow-shape
  contrast (co-07).
- **Ordered steps**: 1) design the flow under the first contract type; 2) design it under the second; 3) write the structural contrast.
- **Acceptance criteria**: each flow is structurally distinct, not a relabeled copy of the other; the
  contrast names the concrete structural difference.
- **Done bar**: a written worked example, no code, no system built.

## Read more

> Populated during the Phase 1.2a `web-researcher` confirmation pass (`A12`) — coverage-check
> citations only.

## In which paths

- `skills/sharia-erp` only — Stage C, course 29 of 30.

---

← Back to the [syllabus index](../README.md)
