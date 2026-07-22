# Treasury and Cash Management (By Example)

**Course ID**: `treasury-and-cash-management` · **Format**: By Example.

**Short summary**: Cash positioning, forecasting, and bank reconciliation — where AP's and AR's
subledgers meet as one cash picture.

**Scope note**: cash position, short-term forecasting, and bank reconciliation — built directly on
course \#6's payables and course \#7's receivables cycles, not a re-teaching of either.

## Why this exists · the big idea

- **The problem before the solution**: AP (#6) and AR (#7) each maintain their own subledger of
  obligations, but neither alone answers "how much cash will we actually have next week?" — that
  question requires combining both, plus timing assumptions neither subledger states outright.
- **Keep-this-if-you-forget-everything**: a cash forecast is an estimate built from payment-timing
  assumptions on top of AP and AR data, not a fact read directly off the ledger — treating a forecast
  as certain is the fastest way to be surprised by a liquidity shortfall the underlying data could have
  warned about.
- **Big ideas touched**: `subledger-to-ledger-integrity` — cash management is where AP's and AR's
  separate subledgers are combined into one operational picture — and `estimation-under-uncertainty` —
  every forecast beyond the current bank balance is an estimate, not a measurement.

## Prerequisites

- **Prior courses**: `accounts-payable-and-procure-to-pay` (#6),
  `accounts-receivable-and-order-to-cash` (#7).
- **Assumed knowledge**: #6's payables cycle, #7's receivables cycle.

## Accuracy notes

- Cash-position, forecasting, and bank-reconciliation mechanics are stable, widely taught domain
  knowledge with no dynamic component to re-verify at authoring `[Verified — stable, non-dynamic
domain fact]`.
- The restricted-cash distinction (co-08) is domain reasoning about what co-01's cash position may and
  may not aggregate; the specific presentation any one standard requires is not asserted here
  `[Needs Verification]` pending the Phase 1 coverage pass.

## Concepts

- **co-01 · cash-position** — the current, actual cash balance across all bank accounts at a point in
  time.
- **co-02 · cash-forecasting** — projecting future cash position from expected AP disbursements and AR
  collections, plus timing assumptions.
- **co-03 · bank-reconciliation** — reconciling the book cash balance against the bank statement's
  balance, accounting for outstanding checks, deposits in transit, and bank fees.
- **co-04 · float** — the timing gap between when a payment is recorded in the books and when it
  actually clears the bank.
- **co-05 · cash-pooling** — combining cash balances across multiple entities or accounts to optimize
  overall liquidity, relevant once #13's multi-entity structure is in play.
- **co-06 · short-term-investment-classification** — classifying excess cash placed in short-term
  instruments (and the accounting distinction between cash equivalents and short-term investments).
- **co-07 · working-capital** — current assets minus current liabilities, the standard liquidity
  measure that AP and AR feed directly into.
- **co-08 · restricted-cash** — cash the entity holds but cannot freely spend because a contract or a
  legal requirement reserves it (an escrow, a compensating balance, a customer security deposit); it
  belongs in the cash position of co-01 only when reported separately, because a single aggregate
  "cash" figure presents reserved balances as though they were available to meet obligations.

## Worked examples

### Beginner

- **ex-01 · state-the-cash-position** — sum three bank accounts' balances into one cash position —
  verify the total against independent addition. (co-01)
- **ex-02 · reconcile-a-bank-statement** — reconcile a book cash balance against a bank statement with
  one outstanding check and one deposit in transit — verify the reconciled balance matches both sides.
  (co-03, co-04)

### Intermediate

- **ex-03 · build-a-13-week-forecast** — build a simple cash forecast from a set of AP due dates and AR
  expected collection dates over several weeks — verify the projected ending balance each week. (co-02)
- **ex-04 · stress-a-forecast-assumption** — change one collection-timing assumption in ex-03's
  forecast (a large receivable slips two weeks) — verify how the projected cash position changes, and
  identify which week first shows a shortfall. (co-02)
- **ex-05 · compute-working-capital** — compute working capital from a stated set of current assets and
  current liabilities including AP and AR balances — verify the figure and its sign. (co-07)
- **ex-06 · classify-a-short-term-investment** — classify a 60-day term deposit as a cash equivalent or
  a short-term investment against a stated maturity threshold — verify the classification. (co-06)

### Advanced

- **ex-07 · pool-cash-across-two-entities** — combine two entities' cash positions into a pooled
  liquidity view — verify the pooled total and identify which entity, viewed alone, would appear
  short. (co-05)
- **ex-08 · stale-forecast-assumption-failure** — a cash forecast built on a collection-timing
  assumption that stopped being true (customers began paying slower) but was never updated — verify the
  book cash balance and bank reconciliation both still balance perfectly while the forecast
  systematically overstates future cash, and name the observable signal (actual collections
  consistently lagging the forecast) that would reveal it. (co-02, silent-failure)
- **ex-09 · separate-restricted-from-available-cash** — a cash position that includes an escrow balance
  the entity may not draw on — verify the bank reconciliation of ex-02 still ties on the aggregate
  figure, then recompute ex-03's forecast with the escrow excluded and identify which week's projected
  shortfall the aggregate figure had concealed. (co-01, co-02, co-08)

## Applied synthesis (no build — A6)

Build a short cash forecast by hand from a stated set of AP and AR items with payment-timing
assumptions, reconcile the starting book balance against a bank statement, and compute working
capital from the same underlying data. Verify the forecast's ending balance, the reconciled balance,
and the working-capital figure against independent recomputation. No system is built — the synthesis
is the hand-worked forecast and reconciliation.

## Read more

- **Corporate Cash Management, Excess Liquidity, and Cash Flow Uncertainty** — general treasury
  management literature; cited nominatively as a domain reference, not transcribed.
- **Treasury Management: The Practitioner's Guide** — Steven M. Bragg (Wiley). A standard treasury
  management reference; cited nominatively.

## In which paths

- `conventional-accounting` — Stage 2 · Most conventional systems a mid-size company runs, plus how to
  architect (not build) a ledger system.
- `sharia-accounting` — Stage 2 · same; the shared spine both paths cover identically.

---

← Back to the [syllabus index](../README.md)
