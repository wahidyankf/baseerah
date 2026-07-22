# Fixed Assets and Depreciation (By Example)

**Course ID**: `fixed-assets-and-depreciation` · **Format**: By Example.

**Short summary**: Owned tangible-asset accounting — capitalisation, depreciation, disposal, and
impairment.

**Scope note**: owned tangible assets only; leased assets and intangibles are #11's subject. This is
the corpus's first recurring, systematic estimate (as opposed to #7's one-off allowance estimate), and
a direct prerequisite for #11's lease treatment.

## Why this exists · the big idea

- **The problem before the solution**: a purchase that benefits several future periods should not be
  expensed entirely in the period it is bought — but every method for spreading that cost forward is
  an estimate, not a measurement, and the choice of method changes reported profit in every affected
  period.
- **Keep-this-if-you-forget-everything**: depreciation is the systematic, period-by-period recognition
  of an estimate — how fast an asset is actually consumed — not an arbitrary bookkeeping ritual.
- **Big ideas touched**: `estimation-under-uncertainty` — depreciation is this corpus's first
  **recurring** estimate (#7's allowance was a one-off estimate); the method chosen must match how the
  asset is actually consumed, or every period's expense is smoothed or lumped incorrectly.

## Prerequisites

- **Prior courses**: `chart-of-accounts-and-data-modeling` (#2).
- **Assumed knowledge**: #2's schema.

## Accuracy notes

- Depreciation methods (straight-line, declining-balance, units-of-production) are stable, widely
  taught domain knowledge with no dynamic component to re-verify at authoring `[Verified — stable,
non-dynamic domain fact]`.

## Concepts

- **co-01 · capitalisation-threshold** — the decision to record a purchase as an asset (spread over
  future periods) rather than expense it immediately, and its balance-sheet consequence.
- **co-02 · straight-line-depreciation** — spreading an asset's depreciable cost evenly across its
  useful life.
- **co-03 · declining-balance-depreciation** — depreciating a larger portion of cost in earlier
  periods, modelling faster early consumption.
- **co-04 · units-of-production-depreciation** — depreciating in proportion to actual usage (units
  produced, hours run) rather than time elapsed.
- **co-05 · fixed-asset-subledger** — the asset register: useful life, salvage value, and accumulated
  depreciation as a contra-account.
- **co-06 · accumulated-depreciation-as-contra-account** — a contra-asset account that reduces an
  asset's carrying value without altering its original cost.
- **co-07 · disposal-gain-or-loss** — the difference between an asset's sale proceeds and its carrying
  value at disposal, recognised as a gain or loss.
- **co-08 · impairment-conceptual** — a conceptual-level trigger for writing an asset down when its
  carrying value exceeds its recoverable value, ahead of its normal depreciation schedule.

## Worked examples

### Beginner

- **ex-01 · capitalise-vs-expense** — decide whether a purchase (a $50 tool vs. a $50,000 machine)
  should be capitalised or expensed — verify the decision against the capitalisation threshold. (co-01)
- **ex-02 · build-a-straight-line-schedule** — build a straight-line depreciation schedule for one
  asset over its useful life — verify each period's expense is identical and the schedule ends at
  salvage value. (co-02)

### Intermediate

- **ex-03 · build-a-declining-balance-schedule** — build a declining-balance schedule for the same
  asset — verify total depreciation across the useful life matches ex-02's total, though the
  period-by-period split differs. (co-03)
- **ex-04 · build-a-units-of-production-schedule** — build a units-of-production schedule for an asset
  with variable annual usage — verify each period's expense tracks actual usage rather than time.
  (co-04)
- **ex-05 · maintain-the-asset-register** — record an asset's cost, useful life, salvage value, and
  running accumulated depreciation in a subledger — verify the subledger's total ties to the general
  ledger's accumulated depreciation balance. (co-05, co-06)
- **ex-06 · dispose-of-an-asset** — sell a partially depreciated asset — verify the gain or loss on
  disposal against the asset's carrying value at the sale date. (co-07)

### Advanced

- **ex-07 · impairment-writedown** — an asset whose recoverable value has fallen below its carrying
  value — verify the writedown at a conceptual level and how it changes the depreciation base going
  forward. (co-08)
- **ex-08 · mismatched-method-failure** — depreciate a heavily-used-early asset on a straight-line
  schedule instead of declining-balance or units-of-production — verify the trial balance still foots
  while early-period expense is understated and late-period expense is overstated relative to actual
  consumption, and name the observable signal (disposal losses concentrated late in life) that would
  reveal it. (co-02, co-03, silent-failure)

## Applied synthesis (no build — A6)

Build a depreciation schedule for one asset under two different methods by hand, then dispose of the
asset partway through its life and compute the resulting gain or loss under each schedule. Verify both
schedules' total depreciation matches, and identify which method is defensible for a stated
consumption pattern. No system is built — the synthesis is the two hand-worked schedules and the
disposal calculation.

## Read more

- **Intermediate Accounting** — Kieso, Weygandt & Warfield (Wiley). Cited nominatively for a fuller
  treatment of depreciation methods and impairment.

## In which paths

- `conventional-accounting` — Stage 2 · Most conventional systems a mid-size company runs, plus how to
  architect (not build) a ledger system.
- `sharia-accounting` — Stage 2 · same; the shared spine both paths cover identically.

---

← Back to the [syllabus index](../README.md)
