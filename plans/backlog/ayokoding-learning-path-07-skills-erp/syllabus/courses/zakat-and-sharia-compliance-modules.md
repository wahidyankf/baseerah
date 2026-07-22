# Zakat and Sharia Compliance Modules (Annotated-concept)

**Course ID**: `zakat-and-sharia-compliance-modules` · **Format**: Annotated-concept · **Language**: — (domain, no code).

**Short summary**: Zakat calculation architecture, Sharia-board workflow, compliance reporting

**Scope note**: closes Stage C and `sharia-erp` (Dangerous 4) — zakat calculation module architecture,
Sharia-board approval workflow touchpoints, and compliance reporting distinct from conventional
statutory reporting, tying back to course 27's jurisdictional-configuration design.
License-aware (DD-15).

## Why this exists · the big idea

- **The problem before the solution**: zakat calculation and Sharia-board approval are compliance
  concerns with no conventional-ERP analogue — treating them as an afterthought bolted onto a
  conventional module produces a design that cannot actually satisfy them.
- **Keep-this-if-you-forget-everything**: zakat calculation depends on the same jurisdictional
  configuration (course 27) as the chart of accounts — it is not a fixed formula, it is a configured
  calculation.
- **Big ideas touched**: `zakat-as-configured-calculation`, not a hardcoded formula;
  `sharia-board-as-an-approval-workflow-actor`, distinct from a conventional approval hierarchy.

## Prerequisites

- **ERP prereqs**: [`erp-security-and-controls`](./erp-security-and-controls.md),
  [`sharia-compliant-erp-design`](./sharia-compliant-erp-design.md).
- **Assumed knowledge**: course 25's role/authorization-object vocabulary; course 27's
  jurisdictional-pluggability and profit-sharing/zakat hooks.

## Accuracy notes (pending A4 verification)

> The Phase 1.2a confirmation pass has not yet run for this course.

- Zakat calculation-method specifics (which assets are zakatable, the applicable nisab and rate) are
  `[Unverified]` pending the primary-source re-verification pass named in `tech-docs.md`; this course
  teaches the calculation as a **configured, jurisdiction-dependent module**, not a specific fixed
  formula.
- Concepts co-07 and co-08 are placed on domain-reasoning grounds rather than sourced from the
  grounding research, and are `[Needs Verification]` pending the Phase 1.2a coverage pass.

## Concepts

- **co-01 · zakat-calculation-module** — a configurable calculation over zakatable assets, distinct
  per jurisdictional configuration (course 27).
- **co-02 · zakatable-asset-classification** — which asset types are included in a zakat calculation,
  itself a configuration point rather than a universal fixed list.
- **co-03 · sharia-board-approval-workflow** — an approval actor distinct from a conventional approval
  hierarchy (course 25's role concepts), specific to Sharia-compliance sign-off.
- **co-04 · sharia-board-as-workflow-participant** — modeling the Sharia board as a participant in an
  approval-workflow state machine (course 4's state-machine concept, applied).
- **co-05 · compliance-reporting-distinct-from-statutory** — Sharia-compliance reporting has its own
  disclosure requirements, distinct from conventional statutory financial reporting (course 26's
  reporting concepts, applied to a new report type).
- **co-06 · jurisdictional-configuration-recap** — zakat and compliance-reporting modules both draw on
  the same jurisdictional-configuration mechanism introduced in course 27, closing the corpus's
  Sharia-specific design arc.
- **co-07 · zakat-valuation-date-as-configuration** — the point in time a zakat calculation reads
  balances at is itself configured against the fiscal-calendar variant (course 7), not fixed by the
  module; two entities on different calendar variants therefore strike the same calculation on
  different balances. `[Needs Verification]`
- **co-08 · compliance-evidence-trail** — every zakat figure and every Sharia-board decision must trace
  back to the transactions and approvals that produced it, reusing course 9's audit-trail and
  change-tracking mechanics rather than a parallel compliance log that could drift from them.

## Worked examples

Prose-based worked scenarios (no runnable code). Every example cites the `co-NN` it exercises.

### Beginner

- **ex-01 · zakatable-asset-classify** — given a sample balance sheet, classify which line items would
  be zakatable under a given jurisdictional configuration. (co-01, co-02)
- **ex-02 · sharia-board-workflow-trace** — trace a transaction requiring Sharia-board approval
  through its workflow states. (co-03, co-04)

### Intermediate

- **ex-03 · compliance-report-contrast** — given the same period's data, contrast what a
  Sharia-compliance report discloses versus what a conventional statutory report discloses. (co-05)
- **ex-04 · jurisdictional-recap-trace** — given a business reconfigured from one jurisdictional model
  to another (course 27), trace how its zakat calculation and compliance report both change as a
  result. (co-06)
- **ex-05 · valuation-date-sensitivity** — given the same set of balances and two entities configured
  on different fiscal-calendar variants (course 7), show that the zakat calculation reads a different
  set of balances in each, without asserting any one jurisdiction's rate or nisab. (co-01, co-07)
- **ex-06 · zakat-figure-traceback** — given a reported zakat figure, trace it back through the
  classified assets and the Sharia-board approval that produced it, using course 9's change-tracking
  record. (co-02, co-03, co-08)

## Synthesis exercise — intra-topic

> Analysis and design only — never build, implement, or deploy a system (`A6`).

- **Goal**: design a zakat-calculation module and a Sharia-board approval workflow for a sample
  business, and produce a compliance report distinct from its conventional statutory report.
- **Concepts exercised**: [ ] zakat calculation (co-01, co-02) [ ] Sharia-board workflow (co-03, co-04)
  [ ] compliance reporting (co-05).
- **Ordered steps**: 1) classify zakatable assets; 2) compute the zakat calculation; 3) design the
  approval workflow; 4) produce the compliance report.
- **Acceptance criteria**: the zakat calculation is traceable to the classified assets; the workflow
  correctly includes the Sharia board as a distinct participant; the compliance report is genuinely
  distinct from a conventional statutory report.
- **Done bar**: a written design, no code, no system built.

## Read more

> Populated during the Phase 1.2a `web-researcher` confirmation pass (`A12`) — coverage-check
> citations only.

## In which paths

- `skills/sharia-erp` only — Stage C, course 29 of 29. **Dangerous 4 ⚡ — `sharia-erp` ENDS HERE.**

---

← Back to the [syllabus index](../README.md)
