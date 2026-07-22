# Consolidation and Multi-Entity Accounting (By Example)

**Course ID**: `consolidation-and-multi-entity-accounting` · **Format**: By Example.

**Short summary**: Combining a parent and its subsidiaries' financial statements into one set of group
statements.

**Scope note**: consolidation scope, intercompany elimination, and non-controlling interest — for a
group of entities already each individually reporting under #3's mechanics and, where foreign,
already translated per #12.

## Why this exists · the big idea

- **The problem before the solution**: a parent and its subsidiaries each keep their own books; simply
  adding the statements together double-counts every intercompany sale, loan, and profit — consolidation
  is not addition, it is addition minus elimination.
- **Keep-this-if-you-forget-everything**: every intercompany balance and every unrealized intercompany
  profit must be eliminated before the group's statements are meaningful — an unremoved intercompany
  receivable/payable pair, or unrealized profit still sitting in ending inventory, overstates the
  group.
- **Big ideas touched**: `subledger-to-ledger-integrity` — extended here to entity level: the
  relationship between each subsidiary's ledger and the group's consolidated ledger is exactly the
  subledger-to-GL relationship this corpus has taught throughout, one level up.

## Prerequisites

- **Prior courses**: `financial-statements-and-close-cycle` (#3),
  `chart-of-accounts-and-data-modeling` (#2), `multi-currency-accounting-and-fx-translation` (#12).
- **Assumed knowledge**: #3's statement derivation, #2's schema, #12's translation mechanics for
  foreign subsidiaries.

## Accuracy notes

- Consolidation mechanics (control test, intercompany elimination, non-controlling interest) are
  stable, widely taught domain knowledge `[Judgment call — the mechanics are cited generically; no
clause or paragraph is reproduced from any standard per A8]`.

## Concepts

1. **co-01 · parent-subsidiary-structure** — a parent controls one or more subsidiaries, each with its
   own separately maintained books.
2. **co-02 · consolidation-scope** — determining which entities are included in the group's
   consolidated statements, based on control, not mere ownership percentage.
3. **co-03 · intercompany-balance-elimination** — removing intercompany receivables/payables and
   intercompany revenue/expense pairs so the group's statements reflect only external transactions.
4. **co-04 · unrealized-intercompany-profit-elimination** — removing profit on an intercompany sale
   that has not yet been resold to an external party (e.g. still sitting in the buyer's ending
   inventory).
5. **co-05 · non-controlling-interest** — the portion of a partially-owned subsidiary's equity and
   income attributable to owners outside the parent.
6. **co-06 · consolidation-worksheet** — the working paper that combines each entity's trial balance and
   applies elimination entries to produce the consolidated result.
7. **co-07 · goodwill-on-consolidation** — the excess of the purchase price for a subsidiary over the
   fair value of its identifiable net assets, recognised at acquisition (ties to #11's goodwill
   concept at group level).

## Worked examples

### Beginner

- **ex-01 · determine-consolidation-scope** — given three entities with different ownership/control
  levels, determine which are consolidated — verify the scope decision against the control test rather
  than ownership percentage alone. (co-01, co-02)
- **ex-02 · simple-add-across-two-entities** — sum a parent's and a wholly-owned subsidiary's trial
  balances with no intercompany activity — verify the naive sum is already correct in this
  no-elimination case. (co-01)

### Intermediate

- **ex-03 · eliminate-an-intercompany-balance** — a parent's receivable from, and a subsidiary's payable
  to, the same intercompany loan — verify both are eliminated and the consolidated balance sheet no
  longer shows either. (co-03)
- **ex-04 · eliminate-intercompany-revenue-and-expense** — an intercompany service fee recorded as
  revenue by one entity and expense by another — verify both are eliminated from the consolidated
  income statement. (co-03)
- **ex-05 · eliminate-unrealized-intercompany-profit** — a parent sells inventory to a subsidiary at a
  markup, and the subsidiary has not yet resold it externally — verify the unrealized profit is
  eliminated from consolidated inventory and income. (co-04)
- **ex-06 · compute-non-controlling-interest** — a subsidiary 80%-owned by the parent — verify the 20%
  non-controlling interest's share of the subsidiary's equity and income, both computed separately from
  the parent's own equity. (co-05)

### Advanced

- **ex-07 · full-consolidation-worksheet** — build a consolidation worksheet combining a parent and one
  subsidiary with both an intercompany loan and unrealized intercompany inventory profit — verify the
  consolidated statements after all eliminations. (co-01–co-06)
- **ex-08 · unremoved-elimination-failure** — consolidate two entities while forgetting to eliminate one
  intercompany receivable/payable pair — verify the consolidated trial balance still foots (both sides
  of the omitted elimination are individually balanced) while the group's total assets and liabilities
  are simultaneously overstated by the same amount. (co-03, silent-failure)

## Applied synthesis (no build — A6)

Consolidate a parent and one wholly-owned subsidiary by hand, including one intercompany loan and one
unrealized intercompany inventory profit, producing a full consolidation worksheet and the resulting
group balance sheet. Verify the consolidated statement balances and that neither eliminated item
appears in the group total. No system is built — the synthesis is the hand-worked worksheet.

## Read more

- **Advanced Accounting** — Hoyle, Schaefer & Doupnik (McGraw-Hill). A standard textbook on
  consolidation mechanics and intercompany elimination; cited nominatively.

## In which paths

- `conventional-accounting` — Stage 2 · Most conventional systems a mid-size company runs, plus how to
  architect (not build) a ledger system.
- `sharia-accounting` — Stage 2 · same; the shared spine both paths cover identically.

---

← Back to the [syllabus index](../README.md)
