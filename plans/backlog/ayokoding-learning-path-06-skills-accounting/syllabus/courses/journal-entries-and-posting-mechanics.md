# Journal Entries and Posting Mechanics (By Example)

**Course ID**: `journal-entries-and-posting-mechanics` · **Format**: By Example. **NEW course (A9)**.

**Short summary**: Posting rules at systems scale — batch vs. real-time posting, safe correction, and
suspense accounts.

**Scope note**: the systems-mechanics bridge between manual bookkeeping (#1) and every course that
follows — the material "three courses to a balancing ledger" implies but the original catalog never
taught as its own subject. See
[tech-docs §What changed](../../tech-docs.md#what-changed-from-the-original-twenty-course-single-path-catalog-and-why).

## Why this exists · the big idea

- **The problem before the solution**: a reader who can hand-post one entry (#1) and design a schema
  for it (#2) still cannot answer "how does a system safely correct a mistake it already posted?" —
  every downstream course assumes this question is already settled.
- **Keep-this-if-you-forget-everything**: a correct system rarely deletes a posted entry — it reverses
  it — because an editable ledger cannot support an audit trail, and this corpus's silent-failure
  theme depends on entries staying visible even when wrong.
- **Big ideas touched**: `silent-failure` — this is the **first course carrying the formal "what still
  balances while being wrong" section**, per
  [tech-docs DD-609](../../tech-docs.md#design-decisions); every course from here through #24 carries
  one.

## Prerequisites

- **Prior courses**: `financial-statements-and-close-cycle` (#3).
- **Assumed knowledge**: #1–#3's mental model, schema, and close cycle.

## Accuracy notes

- Posting-mechanics patterns (batch/real-time, reversing entries, suspense accounts) are
  domain-standard software/accounting-integration knowledge `[Judgment call — no single canonical
textbook source; the described patterns are cross-checked conceptually against ledger-cli's publicly
documented posting model, BSD-3-Clause, described behaviourally per A8's preference for permissive
references, no code reproduced]`. This course's placement is an A9 addition — concepts not directly
  sourced from the seeding research are flagged `[Needs Verification]` pending the Phase 1
  `web-researcher` coverage pass.

## Concepts

1. **co-01 · batch-vs-real-time-posting** — entries posted in scheduled batches versus posted
   immediately as they occur, and the consistency tradeoffs of each.
2. **co-02 · posting-date-vs-transaction-date** — the two dates an entry carries, and why conflating
   them breaks period integrity.
3. **co-03 · voiding-vs-reversing** — voiding removes an entry as if it never happened; reversing posts
   an equal-and-opposite entry, preserving the audit trail — a correct system reverses.
4. **co-04 · reversing-entry-pattern** — the standard correction mechanism: a new entry that offsets
   the original, both remaining visible.
5. **co-05 · suspense-account** — a holding account for entries that cannot yet be classified
   correctly; a persistently nonzero balance is a signal, not a feature.
6. **co-06 · clearing-a-suspense-account** — the mechanics of moving a suspense balance to its correct
   final account once known.
7. **co-07 · recurring-entry-templates** — mechanics for entries that repeat on a schedule
   (depreciation, accruals) without duplicating logic each period.
8. **co-08 · posting-mistake-vs-recognition-mistake** — a posting-mechanics error (wrong period, wrong
   reversal) is a different failure class from a recognition-timing error (#5's subject) — both are
   silent, but they are caught by different controls.

## Worked examples

### Beginner

- **ex-01 · batch-post-three-entries** — post three entries as one batch — verify all three land with
  the same posting date. (co-01)
- **ex-02 · real-time-post-one-entry** — post a single entry immediately as it occurs — verify its
  posting date matches its transaction date. (co-01, co-02)
- **ex-03 · separate-posting-and-transaction-dates** — post an entry with a transaction date one day
  before its posting date — verify both dates are stored and neither overwrites the other. (co-02)

### Intermediate

- **ex-04 · reverse-a-wrong-entry** — reverse an incorrectly posted entry with an offsetting entry —
  verify both entries remain visible and the net effect is zero. (co-03, co-04)
- **ex-05 · void-vs-reverse-contrast** — the same mistake corrected once by voiding and once by
  reversing — verify only the reversal preserves an audit trail of the original mistake. (co-03)
- **ex-06 · route-through-suspense** — post an unclassifiable receipt to a suspense account, then clear
  it once its correct account is known — verify the suspense balance returns to zero. (co-05, co-06)
- **ex-07 · recurring-template-entry** — generate three months of a recurring depreciation entry from
  one template — verify each month's entry matches the template's amount and only its date differs.
  (co-07)

### Advanced

- **ex-08 · reversal-in-the-wrong-period** — reverse a correct entry but post the reversal to the wrong
  period — verify the trial balance still foots while the prior period's ending balance is now
  restated with no disclosure. (co-08, silent-failure)
- **ex-09 · stale-suspense-balance** — a suspense account with a balance unchanged for several periods
  — verify the trial balance foots while the stale balance signals an uninvestigated classification
  gap. (co-05, silent-failure)

## Applied synthesis (no build — A6)

Trace one posting mistake through its full correction lifecycle by hand: post an entry, discover it is
wrong, reverse it with an offsetting entry, and re-post the correct version — once landing the reversal
in the correct period, once (deliberately) in the wrong one. Verify the correct-period trace leaves no
observable trace of the error beyond the visible reversal, and the wrong-period trace produces a
silently restated prior period. No system is built — the synthesis is the two hand-traced corrections.

## Read more

- **ledger-cli** (BSD-3-Clause) — a real, permissively licensed command-line double-entry accounting
  tool; named nominatively as a publicly documented posting model this course's patterns are
  cross-checked against, never quoted from.
- **Accounting Information Systems** — Romney & Steinbart (Pearson). A standard textbook covering
  posting controls and suspense-account practice; cited nominatively.

## In which paths

- `conventional-accounting` — Stage 2 · Most conventional systems a mid-size company runs, plus how to
  architect (not build) a ledger system.
- `sharia-accounting` — Stage 2 · same; the shared spine both paths cover identically.

---

← Back to the [syllabus index](../README.md)
