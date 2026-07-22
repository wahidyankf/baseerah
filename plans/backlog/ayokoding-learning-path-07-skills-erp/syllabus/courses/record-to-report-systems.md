# Record-to-Report Systems (By Example)

**Course ID**: `record-to-report-systems` · **Format**: By Example · **Language**: — (domain, worked scenarios, no application code).

**Short summary**: Subledger closing into GL, trial balance/statement generation, intercompany basics

**Scope note**: the third core transaction-cycle course, and the first Stage B course — record-to-
report ties the P2P and O2C chains' subledger postings to a period close and financial statement
generation. Carries the **hard accounting edge**: this course requires
`financial-statements-and-close-cycle` from `ayokoding-learning-path-06-skills-accounting`, because
subledger-to-GL posting is meaningless without a balanced ledger. License-aware (DD-15).

## Why this exists · the big idea

- **The problem before the solution**: P2P and O2C each produce subledger postings in isolation — R2R
  is where they converge into a single period's financial statements, and where a subledger break
  (course 6) would first become visible to a preparer.
- **Keep-this-if-you-forget-everything**: a financial statement is the GL's summarized output; every
  number on it traces back through a control account to specific subledger transactions.
- **Big ideas touched**: `convergence-of-subledgers`; `intercompany-elimination-preview` (full
  multi-entity treatment in course 24).

## Prerequisites

- **ERP prereqs**: [`erp-subledger-to-gl-architecture`](./erp-subledger-to-gl-architecture.md),
  [`erp-fiscal-calendar-and-period-close`](./erp-fiscal-calendar-and-period-close.md).
- **Accounting prereqs (HARD)**: `financial-statements-and-close-cycle` (from
  `ayokoding-learning-path-06-skills-accounting`) — subledger-to-GL posting is meaningless without a
  balanced ledger; this is the hard edge in the 06→07 dependency (see
  [tech-docs.md §The 06→07 dependency edge](../../tech-docs.md#the-0607-dependency-edge-stage-granularity-not-course-numbers)).
- **Assumed knowledge**: courses 6 and 7's subledger/GL and period-close vocabulary.

## Accuracy notes (pending A4 verification)

> The Phase 1.2a confirmation pass has not yet run for this course.

- The accounting-side course id `financial-statements-and-close-cycle` is as named in
  `ayokoding-learning-path-06-skills-accounting`'s own in-flight rewrite as of 2026-07-22; see the
  cross-plan coordination risk noted in `tech-docs.md`.
- Concepts co-07 through co-10 are placed on domain-reasoning grounds rather than sourced from the
  grounding research, and are `[Needs Verification]` pending the Phase 1.2a coverage pass.

## Concepts

- **co-01 · subledger-convergence** — how P2P (accounts payable) and O2C (accounts receivable)
  subledger postings both feed the same period's GL.
- **co-02 · trial-balance-generation** — the GL's account balances, summed and presented for review
  before statement generation.
- **co-03 · financial-statement-touchpoint** — where R2R hands off to the income statement/balance
  sheet — the accounting corpus's own deep treatment, not duplicated here.
- **co-04 · close-dependency-on-subledgers** — the period cannot close (course 7) until subledgers are
  reconciled (course 6).
- **co-05 · intercompany-transaction-preview** — a transaction between two related legal entities,
  requiring elimination at consolidation (full treatment: course 24).
- **co-06 · r2r-as-integration-point** — R2R is where every other process course's postings ultimately
  land, making it the natural place to verify the whole corpus's posting logic end to end.
- **co-07 · control-account-reconciliation** — the check that proves convergence actually happened: a
  GL control account's balance must equal the sum of its subledger's open items, and a difference
  between the two is a break to be located rather than a rounding artifact to accept.
- **co-08 · subledger-to-gl-summarization-level** — how many subledger lines collapse into one GL
  posting (per document, per day, per account per day), and what each choice costs in traceability
  back from a statement figure to the originating transaction.
- **co-09 · period-boundary-cut-off** — deciding which period a transaction belongs to when its
  document date and its posting date fall either side of the boundary; a cut-off error moves a figure
  between two adjacent periods rather than losing it, which is why it survives a balanced trial
  balance.
- **co-10 · gl-only-entry-at-close** — accruals and provisions recorded directly in the GL with no
  originating subledger transaction: the deliberate exception to co-01's convergence story, and the
  reason a trial balance is never fully explained by the subledgers alone.

## Worked examples

Prose-based worked scenarios with originally-authored data (no runnable code). Every example cites
the `co-NN` it exercises.

### Beginner

- **ex-01 · subledger-to-trial-balance-trace** — given a set of P2P and O2C postings for one period,
  build the resulting trial balance. (co-01, co-02)

### Intermediate

- **ex-02 · close-blocked-by-open-subledger** — given a subledger with an unreconciled item, show the
  period close is blocked until it resolves. (co-04)
- **ex-03 · intercompany-transaction-flag** — given a transaction between two related entities,
  identify it as needing elimination and explain why (without performing the full consolidation,
  deferred to course 24). (co-05)

### Advanced

- **ex-04 · end-to-end-r2r-trace** — trace one full period's P2P and O2C postings through close to a
  trial balance, citing every earlier course's mechanics that contributed to it. (co-01–co-04)
- **ex-05 · summarization-level-tradeoff** — given the same period's subledger postings summarized per
  document and then per account per day, contrast what each level lets a preparer trace back to and
  what it hides. (co-08)
- **ex-06 · control-account-break-diagnosis** — given a control account whose balance does not equal
  its subledger's open-item total, locate the difference and classify it as a cut-off error, a
  GL-only entry, or a genuine subledger break. (co-07, co-09, co-10)

## Synthesis exercise — intra-topic

> Analysis and design only — never build, implement, or deploy a system (`A6`).

- **Goal**: given one period's worth of P2P and O2C transactions (including one intercompany
  transaction and one unreconciled subledger item), produce the close sequence and trial balance,
  explaining why the close is initially blocked.
- **Concepts exercised**: [ ] subledger convergence (co-01) [ ] trial balance (co-02) [ ] close
  dependency (co-04) [ ] intercompany preview (co-05) [ ] control-account reconciliation (co-07)
  [ ] GL-only entries at close (co-10).
- **Ordered steps**: 1) list the period's transactions; 2) attempt the close; 3) show it is blocked by
  the unreconciled item; 4) resolve it; 5) record one accrual as a GL-only entry; 6) produce the trial
  balance; 7) reconcile each control account to its subledger.
- **Acceptance criteria**: the block is correctly attributed to the unreconciled item; the trial
  balance reflects all transactions correctly; each control account reconciles to its subledger, with
  the accrual accounted for as a GL-only entry rather than treated as a break.
- **Done bar**: a written worked example, no code, no system built.

## Read more

> Populated during the Phase 1.2a `web-researcher` confirmation pass (`A12`) — coverage-check
> citations only.

## In which paths

- `skills/conventional-erp` — Stage B, course 13 of 26.
- `skills/sharia-erp` — Stage B, course 13 of 29.

---

← Back to the [syllabus index](../README.md)
