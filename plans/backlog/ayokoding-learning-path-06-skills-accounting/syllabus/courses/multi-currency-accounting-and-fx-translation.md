# Multi-Currency Accounting and FX Translation (By Example)

**Course ID**: `multi-currency-accounting-and-fx-translation` · **Format**: By Example. **NEW course
(A9)**.

**Short summary**: Recording transactions in a foreign currency and translating foreign-entity
financial statements into a reporting currency.

**Scope note**: functional currency, transaction-level FX gain/loss, and statement-level translation —
a real domain gap the original catalog named consolidation but never taught the FX mechanics
consolidation depends on. See
[tech-docs §What changed](../../tech-docs.md#what-changed-from-the-original-twenty-course-single-path-catalog-and-why).

## Why this exists · the big idea

- **The problem before the solution**: #13's consolidation cannot be taught honestly without this
  course — combining a foreign subsidiary's statements into a group's requires first putting every
  number into the same currency, and the mechanics for doing that are not obvious from single-currency
  accounting.
- **Keep-this-if-you-forget-everything**: a monetary item (cash, a receivable) is remeasured at the
  current rate every period; a nonmonetary item (inventory, fixed assets) generally is not — treating
  them the same is the most common FX mistake.
- **Big ideas touched**: `standard-plurality` — ASC 830 (US GAAP) and IAS 21 (IFRS) govern the same
  transaction-vs-translation distinction with closely aligned but not identical mechanics, and a
  systems builder must model functional-currency determination explicitly rather than assume one
  universal rate.

## Prerequisites

- **Prior courses**: `financial-statements-and-close-cycle` (#3).
- **Assumed knowledge**: #3's statement derivation.

## Accuracy notes

- Functional-currency determination and the current-rate/temporal translation mechanics are stable,
  widely documented domain knowledge `[Judgment call — the mechanics are cited generically under both
ASC 830 and IAS 21; no clause or paragraph is reproduced from either standard per A8]`. This course's
  placement is an A9 addition — concepts not directly sourced from the seeding research are flagged
  `[Needs Verification]` pending the Phase 1 `web-researcher` coverage pass.

## Concepts

1. **co-01 · functional-currency** — the currency of the primary economic environment an entity
   operates in, determined by facts, not by choice.
2. **co-02 · transaction-currency-vs-functional-currency** — a transaction denominated in a currency
   other than the entity's functional currency, recorded at the spot rate on the transaction date.
3. **co-03 · fx-gain-or-loss-realized** — the gain or loss recognised when a foreign-currency-
   denominated item is settled at a different rate than it was recorded.
4. **co-04 · fx-gain-or-loss-unrealized** — the gain or loss recognised at period end on an open
   foreign-currency-denominated monetary item, before settlement.
5. **co-05 · monetary-vs-nonmonetary-items** — monetary items (cash, receivables, payables) are
   remeasured at the current rate each period; nonmonetary items (inventory, fixed assets) generally
   remain at their historical rate.
6. **co-06 · current-rate-translation-method** — translating an entire foreign entity's statements at
   the current rate for the balance sheet and an average rate for the income statement, when the
   foreign operation's functional currency is not the reporting currency.
7. **co-07 · cumulative-translation-adjustment** — the equity account that absorbs the translation
   difference produced by the current-rate method, rather than flowing through net income.
8. **co-08 · rate-selection-by-item-type** — different account types use different rates (spot,
   average, historical) even within the same translation, and choosing consistently is what makes a
   translation defensible.

## Worked examples

### Beginner

- **ex-01 · record-a-foreign-currency-sale** — record a sale invoiced in a foreign currency at the spot
  rate on the transaction date — verify the recorded amount in the reporting currency. (co-02)
- **ex-02 · settle-at-a-different-rate** — settle ex-01's invoice when the spot rate has moved — verify
  the realized FX gain or loss recognised at settlement. (co-03)

### Intermediate

- **ex-03 · remeasure-an-open-receivable** — remeasure ex-01's receivable at period end, before
  settlement, using the current rate — verify the unrealized FX gain or loss recognised. (co-04, co-05)
- **ex-04 · classify-monetary-vs-nonmonetary** — classify cash, a receivable, inventory, and a fixed
  asset as monetary or nonmonetary — verify each classification determines whether it is remeasured at
  period end. (co-05)
- **ex-05 · translate-a-balance-sheet** — translate a foreign subsidiary's balance sheet into the
  reporting currency using the current-rate method — verify every line item's translated value. (co-06)
- **ex-06 · translate-an-income-statement** — translate the same subsidiary's income statement using an
  average rate for the period — verify net income translates consistently with the balance sheet
  figures. (co-06, co-08)

### Advanced

- **ex-07 · compute-the-cta** — compute the cumulative translation adjustment that reconciles ex-05 and
  ex-06's translated statements to a balanced translated balance sheet — verify the CTA lands in
  equity, not net income. (co-07)
- **ex-08 · wrong-rate-for-item-type-failure** — translate a nonmonetary inventory balance at the
  current rate instead of its historical rate — verify the translated balance sheet still balances
  (because the CTA absorbs the difference) while the inventory figure itself is wrong, and name the
  observable signal (an unexplained CTA swing with no corresponding operational change) that would
  reveal it. (co-05, co-08, silent-failure)

## Applied synthesis (no build — A6)

Record one foreign-currency transaction from origination through settlement by hand, computing both
the realized gain/loss at settlement and, separately, what the unrealized gain/loss would have been
had the period ended before settlement. Verify both figures against independent recomputation at the
stated rates. No system is built — the synthesis is the two hand-worked FX calculations.

## Read more

- **IFRS Foundation — IAS 21 The Effects of Changes in Foreign Exchange Rates** (ifrs.org). IFRS
  Foundation carries an explicit free-educational-use carve-out; named nominatively, no clause text
  reproduced.
- **FASB Accounting Standards Codification — Topic 830** (fasb.org). FASB's codification is closed
  copyright; named nominatively only.

## In which paths

- `conventional-accounting` — Stage 2 · Most conventional systems a mid-size company runs, plus how to
  architect (not build) a ledger system.
- `sharia-accounting` — Stage 2 · same; the shared spine both paths cover identically.

---

← Back to the [syllabus index](../README.md)
