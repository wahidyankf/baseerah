# Accounts Receivable and Order-to-Cash (By Example)

**Course ID**: `accounts-receivable-and-order-to-cash` · **Format**: By Example.

**Short summary**: The order-to-cash (O2C) cycle and the estimation mechanics — aging, the allowance
for doubtful accounts — it requires.

**Scope note**: order through cash application, and the first course in this corpus to teach a number
that is deliberately not exact. O2C's mirror to #6's P2P. #5 decides **when** revenue is recognised;
this course covers the operational collection cycle after recognition.

## Why this exists · the big idea

- **The problem before the solution**: not every receivable is collected, and a system that assumes
  every invoice will be paid in full overstates its assets from day one.
- **Keep-this-if-you-forget-everything**: an allowance is a defensible estimate reported as a real
  number on the balance sheet — treating it as optional, or as "we'll true it up later," misstates
  collectible assets in the meantime.
- **Big ideas touched**: `estimation-under-uncertainty` — the allowance for doubtful accounts is this
  corpus's first number that is honestly, deliberately not exact, and the choice of estimation method
  is a real economic-consequence decision.

## Prerequisites

- **Prior courses**: `journal-entries-and-posting-mechanics` (#4),
  `accrual-accounting-and-revenue-recognition` (#5).
- **Assumed knowledge**: #4's posting mechanics, #5's recognition timing.

## Accuracy notes

- The allowance method and aging mechanics are stable, widely taught domain knowledge with no dynamic
  component to re-verify at authoring `[Verified — stable, non-dynamic domain fact]`.
- The write-off-against-the-allowance mechanic (co-08) is domain reasoning completing co-03's
  allowance cycle rather than a claim sourced from this plan's grounding file `[Needs Verification]`
  pending the Phase 1 coverage pass.

## Concepts

- **co-01 · order-to-cash-cycle** — order, fulfilment, invoice, collection, cash application, as a
  document chain mirroring #6's P2P.
- **co-02 · accounts-receivable-aging** — grouping open invoices by how long they have been
  outstanding.
- **co-03 · allowance-for-doubtful-accounts** — an estimated reserve against receivables expected to
  go uncollected, reported as a contra-asset.
- **co-04 · allowance-method-vs-direct-write-off** — the allowance method estimates uncollectibility
  in advance; direct write-off waits until a specific account is known bad — and is not
  GAAP/IFRS-compliant treatment for material balances.
- **co-05 · cash-application** — matching an incoming payment to its open invoice(s).
- **co-06 · unapplied-cash** — a payment received but not yet matched to an invoice; a persistent
  unapplied-cash balance is a signal worth investigating.
- **co-07 · credit-memo** — a document reducing a customer's balance (return, pricing correction) and
  its effect on the aging schedule.
- **co-08 · write-off-against-the-allowance** — removing a specific account confirmed uncollectible by
  charging it against the allowance already estimated in co-03 rather than against expense, so the
  write-off itself changes neither the net receivable nor that period's income — the loss was already
  recognised when the allowance was estimated.

## Worked examples

### Beginner

- **ex-01 · walk-the-o2c-chain** — trace one sale from order through cash application across all five
  stages — verify each stage references the prior one. (co-01)
- **ex-02 · build-an-aging-schedule** — bucket five open invoices by days outstanding — verify each
  invoice lands in the correct age bucket. (co-02)

### Intermediate

- **ex-03 · estimate-an-allowance** — apply an aging-based percentage to each bucket from ex-02 to
  estimate the allowance for doubtful accounts — verify the total allowance against an independent
  recomputation. (co-03)
- **ex-04 · allowance-vs-direct-write-off** — the same uncollectible account treated once under the
  allowance method and once under direct write-off — verify the allowance method recognises the
  expected loss earlier, in the period of the sale rather than the period of confirmed default. (co-04)
- **ex-05 · apply-a-payment** — apply an incoming payment to two open invoices for the same customer —
  verify both invoices' open balances decrease correctly. (co-05)
- **ex-06 · track-unapplied-cash** — a payment received with no matching invoice reference — verify it
  is held as unapplied cash rather than incorrectly applied to the wrong invoice. (co-06)
- **ex-07 · post-a-credit-memo** — issue a credit memo for a partial return — verify the customer's
  aging schedule reflects the reduced balance. (co-07)

### Advanced

- **ex-08 · full-o2c-cycle-with-aging** — a full cycle for three customers with different payment
  timing — verify the resulting aging schedule and allowance estimate. (co-01–co-03)
- **ex-09 · understated-allowance-failure** — estimate the allowance using a stale aging percentage
  that understates expected losses — verify the balance sheet still balances while collectible assets
  are overstated, and name the observable signal (actual write-offs consistently exceeding the
  allowance) that would reveal it. (co-03, silent-failure)
- **ex-10 · write-off-a-confirmed-bad-account** — write one confirmed uncollectible invoice off
  against ex-03's allowance, then contrast it with the same write-off charged straight to expense —
  verify the allowance route leaves net receivables and period income unchanged while the
  direct-to-expense route recognises the loss a second time. (co-03, co-04, co-08)

## Applied synthesis (no build — A6)

Build one customer's aging schedule by hand from a set of open invoices, estimate the allowance for
doubtful accounts from it, apply an incoming partial payment, and issue one credit memo. Verify the
aging schedule, the allowance estimate, and the resulting balance are each independently
recomputable. No system is built — the synthesis is the hand-worked schedule and its updates.

## Read more

- **Intermediate Accounting** — Kieso, Weygandt & Warfield (Wiley). Cited nominatively for a fuller
  treatment of the allowance method and receivables aging.

## In which paths

- `conventional-accounting` — Stage 2 · Most conventional systems a mid-size company runs, plus how to
  architect (not build) a ledger system.
- `sharia-accounting` — Stage 2 · same; the shared spine both paths cover identically.

---

← Back to the [syllabus index](../README.md)
