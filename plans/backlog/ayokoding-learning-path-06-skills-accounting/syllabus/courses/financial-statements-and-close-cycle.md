# Financial Statements and Close Cycle (By Example)

**Course ID**: `financial-statements-and-close-cycle` · **Format**: By Example.

**Short summary**: The three financial statements, how they interlock, and the close cycle that
produces them for a single entity.

**Scope note**: balance sheet, income statement, and cash flow statement derivation from a trial
balance, plus the close cycle as a state machine — external/financial reporting only, not internal
management reporting (that is #8's subject). This is the course that unblocks ERP's record-to-report
capability — see [tech-docs §Stage-signal contract](../../tech-docs.md#stage-signal-contract-the-plan-07-handoff-stage-granularity).

## Why this exists · the big idea

- **The problem before the solution**: "a balancing ledger" is not yet "the three statements a
  business actually needs." Without this course the ramp's Stage-1 promise — a working ledger by
  course 3 — would stop one step short of anything a real reader recognises as finished output.
- **Keep-this-if-you-forget-everything**: closing a period is a state transition, not a formality —
  once locked, a period should reject new postings, and what "closing the books" changes is precise
  and mechanical, not vague.
- **Big ideas touched**: `silent-failure` — this course previews the theme with a period-end example
  (an adjusting entry booked to the wrong period, trial balance still foots) without yet requiring the
  formal worked demonstration, which begins at #4 per
  [tech-docs DD-609](../../tech-docs.md#design-decisions).

## Prerequisites

- **Prior courses**: `chart-of-accounts-and-data-modeling` (#2).
- **Assumed knowledge**: #1 and #2's mental models and schema.

## Accuracy notes

- Statement-derivation mechanics and the close-cycle state machine are stable, universally taught
  domain knowledge with no dynamic component to re-verify at authoring `[Verified — stable,
non-dynamic domain fact]`.

## Concepts

- **co-01 · balance-sheet** — the position statement: what the entity owns, owes, and the residual
  equity, at a point in time.
- **co-02 · income-statement** — the performance statement: revenue less expenses over a period.
- **co-03 · cash-flow-statement** — the liquidity statement: cash movements over a period, distinct
  from accrual-basis income.
- **co-04 · statement-interlock** — net income flows into retained earnings, which flows into the
  balance sheet — the three statements are one system, not three independent reports.
- **co-05 · close-cycle-state-machine** — open period → adjusting entries → trial balance →
  statements → period lock, as an explicit sequence of states.
- **co-06 · period-lock** — a locked period rejects new postings; correcting a locked period requires
  a new entry in the current period, not an edit to the past.
- **co-07 · adjusting-entries-mechanical-intro** — accruals and deferrals introduced here at a
  mechanical level; revisited in depth at #5.
- **co-08 · statement-derivation** — the mechanical path from a trial balance to all three statements,
  not just their definitions.
- **co-09 · balancing-vs-correct-preview** — a trial balance can foot while an adjusting entry was
  booked to the wrong period — the theme previewed again, formalised starting at #4.

## Worked examples

### Beginner

- **ex-01 · derive-the-balance-sheet** — derive a balance sheet from a given trial balance — verify
  Assets = Liabilities + Equity holds on the derived statement. (co-01, co-08)
- **ex-02 · derive-the-income-statement** — derive an income statement from the same trial balance —
  verify net income equals revenue minus expenses. (co-02, co-08)
- **ex-03 · trace-net-income-to-equity** — trace ex-02's net income into retained earnings, then into
  ex-01's balance sheet — verify the balance sheet still balances after the trace. (co-04)

### Intermediate

- **ex-04 · derive-the-cash-flow-statement** — derive a simple cash flow statement from the same period
  — verify it reconciles to the change in the cash account on the balance sheet. (co-03, co-04)
- **ex-05 · post-an-adjusting-entry** — post one accrual and one deferral adjusting entry before
  closing — verify both statements change correctly after the adjustment. (co-07)
- **ex-06 · close-and-lock-the-period** — walk the close-cycle state machine from open to locked —
  verify a new posting attempt against the locked period is rejected. (co-05, co-06)
- **ex-07 · correct-a-locked-period** — correct an error discovered after lock via a new current-period
  entry — verify the locked period's statements are unchanged and the correction lands in the open
  period. (co-06)

### Advanced

- **ex-08 · full-close-cycle** — run a complete close from open period through all three statements to
  lock — verify every statement derives correctly and the period rejects postings once locked. (co-01–
  co-08)
- **ex-09 · wrong-period-adjustment** — an adjusting entry correctly balanced but booked one period
  late — verify the trial balance still foots while both periods' income statements are wrong, and
  name the observable signal (a restated prior period with no disclosure) that would reveal it. (co-09)

## Applied synthesis (no build — A6)

Take one populated trial balance through the full close-cycle state machine by hand: post two
adjusting entries, derive all three statements, verify their interlock, and lock the period. Verify
the balance sheet balances, net income traces correctly into equity, and a postlock posting attempt is
rejected on paper. No system is built — the synthesis is the hand-worked close itself.

## Read more

- **Financial Accounting** — Weygandt, Kimmel & Kieso (Wiley). Cited nominatively for a fuller
  treatment of statement derivation and the close cycle.
- **IFRS Foundation — IAS 1 Presentation of Financial Statements** (ifrs.org). The IFRS Foundation
  publishes its **own** free teaching materials for classroom use by recognised institutions under
  attribution and non-commercial terms; **the Standards text itself still requires a separate licence
  to reproduce** `[Verified]` (see
  [tech-docs §Licensing](../../tech-docs.md#licensing-and-ip-compliance-a8)); named nominatively, no
  clause text reproduced.

## In which paths

- `conventional-accounting` — Stage 1 · The first ramp boundary — a working, correctly balancing
  ledger and all three statements by course 3.
- `sharia-accounting` — Stage 1 · same boundary; identical course.

---

← Back to the [syllabus index](../README.md)
