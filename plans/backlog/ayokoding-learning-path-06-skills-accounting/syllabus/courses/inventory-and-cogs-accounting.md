# Inventory and COGS Accounting (By Example)

**Course ID**: `inventory-and-cogs-accounting` · **Format**: By Example.

**Short summary**: Inventory costing methods and their COGS consequences.

**Scope note**: costing methods and the inventory-to-COGS flow. Systems architecture for inventory
(negative stock, reservations, backdated transactions) is a separate, ERP-owned systems-architecture
concern this course does not teach.

## Why this exists · the big idea

- **The problem before the solution**: costing-method choice is one of the corpus's clearest
  "plausible and wrong" examples — two valid methods, applied to the identical transaction set,
  produce two different, individually consistent, and mutually incompatible sets of numbers.
- **Keep-this-if-you-forget-everything**: FIFO, weighted-average, and specific identification are all
  defensible; picking one is a real economic-consequence decision, not a bookkeeping preference, and
  switching methods mid-stream is itself a disclosure event.
- **Big ideas touched**: `standard-plurality` — **LIFO is IFRS-forbidden but GAAP-permitted**, a
  genuine, stable cross-standard divergence a systems builder must model explicitly rather than assume
  away.

## Prerequisites

- **Prior courses**: `chart-of-accounts-and-data-modeling` (#2), `managerial-and-cost-accounting` (#8).
- **Assumed knowledge**: #2's schema, #8's cost-classification vocabulary.

## Accuracy notes

- FIFO/weighted-average and the IFRS LIFO prohibition are stable, well-documented domain facts with no
  dynamic component to re-verify at authoring `[Verified — stable, non-dynamic domain fact]`.

## Concepts

1. **co-01 · fifo-costing** — first-in-first-out: the earliest-purchased units are assumed sold first.
2. **co-02 · weighted-average-costing** — a single average cost per unit, recomputed as new purchases
   arrive.
3. **co-03 · specific-identification** — tracking the actual cost of each specific unit sold, used
   where units are individually distinguishable.
4. **co-04 · lifo-and-the-ifrs-prohibition** — last-in-first-out is GAAP-permitted but IFRS-forbidden —
   a genuine, stable cross-standard divergence.
5. **co-05 · perpetual-inventory-system** — COGS and inventory balances update with every transaction.
6. **co-06 · periodic-inventory-system** — COGS is computed only at period end from a physical count,
   not per transaction.
7. **co-07 · cogs-derivation** — the mechanical path from a chosen costing method to a computed COGS
   figure.
8. **co-08 · lower-of-cost-or-net-realisable-value** — writing inventory down below its costed value
   when its net realisable value has fallen below cost.

## Worked examples

### Beginner

- **ex-01 · fifo-cost-a-sale** — cost one sale under FIFO from three purchase lots at different prices
  — verify the resulting COGS and ending inventory value. (co-01, co-07)
- **ex-02 · weighted-average-cost-a-sale** — cost the same sale under weighted-average — verify the
  resulting COGS differs from ex-01's FIFO result. (co-02, co-07)

### Intermediate

- **ex-03 · specific-identification-cost-a-sale** — cost a sale of individually serial-numbered units
  under specific identification — verify COGS matches the actual cost of the units sold, not an
  averaged or FIFO figure. (co-03)
- **ex-04 · lifo-under-gaap** — cost the same sale set under LIFO for a US-GAAP-reporting entity —
  verify the result, and separately verify this method is unavailable to an IFRS-reporting entity.
  (co-04)
- **ex-05 · perpetual-vs-periodic** — record the same set of purchases and sales once under a perpetual
  system and once under a periodic system — verify COGS is computed after every sale in the perpetual
  case and only at period end in the periodic case. (co-05, co-06)
- **ex-06 · lcnrv-writedown** — write inventory down when its net realisable value falls below its
  costed value — verify the writedown amount and its effect on COGS. (co-08)

### Advanced

- **ex-07 · same-transactions-three-methods** — cost the identical purchase-and-sale sequence under
  FIFO, weighted-average, and (where permitted) LIFO — verify all three produce different, internally
  consistent COGS and ending-inventory figures from the same transactions. (co-01, co-02, co-04)
- **ex-08 · method-inconsistent-with-physical-flow-failure** — apply FIFO costing to inventory that
  physically moves last-in-first-out (e.g. a bulk pile drawn from the top) — verify the trial balance
  still foots while COGS and ending inventory are both wrong in offsetting ways that never trip a
  balance check. (co-01, silent-failure)

## Applied synthesis (no build — A6)

Cost the identical set of purchases and one sale under FIFO and weighted-average by hand, and identify
which choice would be unavailable to the same entity if it reported under IFRS instead of US GAAP.
Verify COGS and ending inventory under each method against independent recomputation. No system is
built — the synthesis is the two hand-worked costings and the standard-divergence identification.

## Read more

- **Intermediate Accounting** — Kieso, Weygandt & Warfield (Wiley). Cited nominatively for a fuller
  treatment of inventory costing methods.
- **IFRS Foundation — IAS 2 Inventories** (ifrs.org). IFRS Foundation carries an explicit
  free-educational-use carve-out; named nominatively for the LIFO prohibition, no clause text
  reproduced.

## In which paths

- `conventional-accounting` — Stage 2 · Most conventional systems a mid-size company runs, plus how to
  architect (not build) a ledger system.
- `sharia-accounting` — Stage 2 · same; the shared spine both paths cover identically.

---

← Back to the [syllabus index](../README.md)
