# ERP Subledger-to-GL Architecture (By Example)

**Course ID**: `erp-subledger-to-gl-architecture` · **Format**: By Example · **Language**: — (domain, worked scenarios, no application code).

**Short summary**: The architectural crux — subledger vs GL responsibilities and reconciliation

**Scope note**: **the architectural crux of the whole corpus** [Repo-grounded — domain-research
grounding, Part 2]: how sales, purchasing, and inventory subledgers post into the general ledger, and
what reconciliation guarantees hold. Every downstream process course (P2P, O2C, R2R, inventory)
assumes this architecture. License-aware (DD-15).

## Why this exists · the big idea

- **The problem before the solution**: without a clear subledger/GL boundary, a direct posting can
  bypass the subledger's detail entirely — and the resulting break is invisible to a trial balance.
- **Keep-this-if-you-forget-everything**: the GL is designed to never need the subledger's line-item
  detail to balance; a control account is the mechanical proof that the two stay tied together.
- **Big ideas touched**: `control-account-as-proof` — reconciliation is not a courtesy check, it is the
  only thing that proves subledger detail sums to the GL; `real-time-vs-batch-posting` — a recurring
  design axis across the corpus.

## Prerequisites

- **Prior topics**: [`erp-posting-rules-and-account-determination`](./erp-posting-rules-and-account-determination.md).
- **Cross-domain prerequisites**: none.
- **Assumed knowledge**: course 5's account-determination and posting-key vocabulary.

## Accuracy notes (pending A4 verification)

> The Phase 1.2a confirmation pass has not yet run for this course.

- Worked reconciliation example uses an originally-authored dataset; no figures are drawn from any
  reference implementation's sample data.

## Concepts

- **co-01 · subledger-responsibility** — accounts receivable, accounts payable, inventory as the
  detailed transaction record.
- **co-02 · gl-responsibility** — the GL as the summarized, always-reconcilable view.
- **co-03 · gl-independence-from-detail** — the GL is designed to never need the subledger's line-item
  detail to balance.
- **co-04 · real-time-posting** — each subledger transaction posts to the GL immediately.
- **co-05 · batch-posting** — subledger transactions accumulate and post to the GL on a schedule.
- **co-06 · real-time-vs-batch-tradeoff** — real-time simplifies reconciliation but couples subledger
  performance to GL availability.
- **co-07 · control-account** — the GL account a subledger's balance must always tie to.
- **co-08 · blocked-direct-posting** — a direct posting to a control account, bypassing the subledger,
  is normally blocked.
- **co-09 · reconciliation-as-proof** — reconciliation mechanically proves subledger detail sums to the
  control-account balance.
- **co-10 · reconciliation-break** — a subledger posting that bypasses the control-account link, and
  how it manifests.
- **co-11 · silent-vs-visible-failure** — a reconciliation break is invisible to a trial balance but
  visible to a subledger reconciliation report — the concrete mechanism behind the domain's
  characteristic silent-failure framing.

## Worked examples

Prose-based worked scenarios with originally-authored data (no runnable code). Every example cites the
`co-NN` it exercises.

### Beginner

- **ex-01 · subledger-gl-role-split** — given a sample transaction, identify which parts are the
  subledger's job and which are the GL's. (co-01, co-02)
- **ex-02 · control-account-lookup** — given a subledger and its control account, verify the balances
  currently tie. (co-07)

### Intermediate

- **ex-03 · real-time-vs-batch-trace** — trace the same transaction under a real-time posting design
  and a batch posting design — verify both eventually tie. (co-04, co-05, co-06)
- **ex-04 · reconciliation-mechanics** — given subledger line items and a control-account balance,
  perform the reconciliation and verify they sum correctly. (co-09)

### Advanced

- **ex-05 · reconciliation-break-injection** — introduce one subledger posting that bypasses the
  control-account link — verify the control-account balance no longer equals the subledger's summed
  detail. (co-10)
- **ex-06 · silent-failure-demonstration** — show the ex-05 break is invisible to a trial balance but
  caught by a subledger reconciliation report — verify both checks' outcomes explicitly. (co-11)

## Synthesis exercise — intra-topic

> Analysis and design only — never build, implement, or deploy a system (`A6`).

- **Goal**: design a control-account reconciliation scheme for a new subledger (e.g. a fixed-asset
  register), and demonstrate — with a worked example — how a reconciliation break occurs and evades a
  simple balance check.
- **Concepts exercised**: [ ] subledger/GL split (co-01, co-02) [ ] control account (co-07) [ ]
  reconciliation break (co-10, co-11).
- **Ordered steps**: 1) define the subledger and its control account; 2) write a correct worked
  reconciliation; 3) introduce a break; 4) show the trial balance is unaffected; 5) show the
  reconciliation report catches it.
- **Acceptance criteria**: the break is a genuine bypass, not an imbalance; the reconciliation report
  demonstrably catches what the trial balance misses.
- **Done bar**: a written worked example with an originally-authored dataset, no code, no system built.

## Read more

> Populated during the Phase 1.2a `web-researcher` confirmation pass (`A12`) — coverage-check
> citations only.

## In which paths

- `skills/conventional-erp` — Stage A, course 6 of 26.
- `skills/sharia-erp` — Stage A, course 6 of 29.

---

← Back to the [syllabus index](../README.md)
