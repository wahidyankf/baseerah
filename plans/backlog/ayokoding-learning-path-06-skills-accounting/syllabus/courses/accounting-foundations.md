# Accounting Foundations (By Example)

**Course ID**: `accounting-foundations` · **Format**: By Example.

**Short summary**: The accounting equation, double-entry mechanics, and how one transaction becomes a
balanced ledger.

**Scope note**: the accounting equation, debit/credit mechanics, T-accounts, journal-vs-ledger, and a
first trial balance — for a systems builder, not a bookkeeper. Mines `legacy/business/accounting.md`'s
running example and narrative sequencing (DD-626), discarding its small-business-owner framing. No
schema design (that is #2's subject) and no systems-level posting mechanics — batches, reversals,
suspense accounts (that is #4's).

## Why this exists · the big idea

- **The problem before the solution**: every downstream course assumes the reader can look at a
  business event and see its two effects without hesitation. Get this wrong once and every downstream
  number is wrong in a way nothing mechanically catches.
- **Keep-this-if-you-forget-everything**: a transaction is not recorded until it has touched at least
  two accounts by equal amounts, one debited and one credited — that symmetry is the whole mechanism,
  and it is also the whole limitation.
- **Big ideas touched**: `silent-failure` — this course makes the corpus's **first statement** of the
  theme ("a balancing trial balance is necessary but not sufficient for correctness"), without yet
  naming a concrete worked example; the first worked silent-failure example begins at #4, per
  [tech-docs DD-609](../../tech-docs.md#design-decisions), because a hand-built single-entity ledger at
  this stage fails loudly (it does not balance) rather than silently.

## Prerequisites

- **Prior courses**: none — the entry point for a reader with neither accounting nor SQL background.
- **Assumed knowledge**: arithmetic only.

## Accuracy notes

- The accounting equation and double-entry mechanics are centuries-old, stable domain knowledge with
  no dynamic component to re-verify at authoring `[Verified — stable, non-dynamic domain fact]`.
- The running example's narrative sequencing is mined from `legacy/business/accounting.md`
  `[Repo-grounded, 34.2 KB]` per DD-626; no paragraph is reproduced verbatim.

## Concepts

1. **co-01 · accounting-equation** — Assets = Liabilities + Equity, restated as a systems invariant:
   every recorded event has two effects that must offset.
2. **co-02 · double-entry** — every transaction touches at least two accounts, one debited and one
   credited, by equal amounts.
3. **co-03 · debit-credit-mechanics** — for each account type, which side increases it and which
   decreases it; "debit" and "credit" are positional labels, not judgments.
4. **co-04 · account-types** — Asset, Liability, Equity, Revenue, Expense, and each type's normal
   balance side.
5. **co-05 · t-accounts** — the two-column visualization of one account's debits and credits, a visual
   aid rather than a data structure to implement literally.
6. **co-06 · journal-vs-ledger** — the journal records transactions chronologically; the ledger
   reorganizes them by account.
7. **co-07 · trial-balance** — the summary check that total debits equal total credits across every
   account.
8. **co-08 · balancing-is-not-correctness** — a trial balance can foot while an individual posting is
   substantively wrong (see the big-idea bullet above) — stated here, demonstrated concretely starting
   at #4.
9. **co-09 · chart-of-accounts-preview** — accounts need a stable naming/numbering scheme, taught in
   full in #2.
10. **co-10 · accounting-period-preview** — transactions are grouped into a bounded period for
    reporting, taught in full in #3.
11. **co-11 · equity-as-residual** — equity is what remains after liabilities are subtracted from
    assets, not a bank account with its own independent balance.
12. **co-12 · ledger-as-append-only** — entries are conceptually never edited in place; a correction is
    a new offsetting entry, not an edit to history.

## Worked examples

### Beginner

- **ex-01 · record-a-cash-sale** — record a cash sale of a stated amount in T-accounts — verify debits
  equal credits. (co-02, co-03)
- **ex-02 · record-an-expense-payment** — record a rent payment — verify the equation still balances
  after posting. (co-01, co-03)
- **ex-03 · classify-five-accounts** — classify Cash, Accounts Payable, Revenue, Rent Expense, and
  Owner's Equity by type — verify each account's normal balance side against co-04's rule. (co-04)
- **ex-04 · build-a-t-account** — post three transactions to one Cash T-account — verify the running
  balance after each posting. (co-05)
- **ex-05 · journal-entry-basic** — write the journal entry for a loan received in cash — verify the
  debit and credit lines net to the loan amount. (co-02, co-06)

### Intermediate

- **ex-06 · multi-account-transaction** — an equipment purchase paid partly in cash and partly on
  account — verify three accounts are touched and the equation still balances. (co-01, co-02)
- **ex-07 · post-to-ledger** — post five journal entries to their ledger accounts — verify each
  account's ending balance by hand. (co-06)
- **ex-08 · build-a-trial-balance** — sum every account balance from ex-07 into a trial balance —
  verify total debits equal total credits. (co-07)
- **ex-09 · reversed-entry-catch** — an entry posted with debit and credit sides reversed — verify the
  trial balance does **not** balance, and that this is a case the mechanical check does catch. (co-03,
  co-07)
- **ex-10 · correction-as-new-entry** — correct a wrongly posted amount by a new offsetting entry
  rather than editing the original — verify the ledger's history is unchanged and the correction nets
  to the right ending balance. (co-12)

### Advanced

- **ex-11 · five-transaction-sequence** — a sequence of five transactions spanning all five account
  types — verify the resulting trial balance by independent recomputation. (co-01–co-07)
- **ex-12 · equity-as-residual-check** — after ex-11's five transactions, recompute equity two ways —
  directly from its own account, and as Assets minus Liabilities — verify both methods agree. (co-11)

## Applied synthesis (no build — A6)

Trace one week of transactions (a cash sale, an expense payment, and a loan draw) by hand from journal
entry through T-accounts to a trial balance. Verify the trial balance balances and that every
account's ending balance matches an independent manual recomputation. No system is built — the
synthesis is the hand-worked trace itself.

## Read more

- **Financial Accounting** — Weygandt, Kimmel & Kieso (Wiley). A widely used introductory financial
  accounting textbook, cited nominatively for a reader who wants a fuller treatment.
- **Summa de Arithmetica, Geometria, Proportioni et Proportionalita** — Luca Pacioli (1494). The first
  published description of double-entry bookkeeping, cited for historical context only.

## In which paths

- `conventional-accounting` — Stage 1 · The first ramp boundary — a working, correctly balancing
  ledger by course 3.
- `sharia-accounting` — Stage 1 · same boundary; identical course.

---

← Back to the [syllabus index](../README.md)
