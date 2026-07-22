# Chart of Accounts and Data Modeling (By Example)

**Course ID**: `chart-of-accounts-and-data-modeling` · **Format**: By Example.

**Short summary**: Turning a chart of accounts into a normalised schema that enforces double-entry at
the data layer.

**Scope note**: account types, numbering schemes, parent/child hierarchies, journal-header-vs-line
table design, and constraint enforcement — the material `legacy/business/accounting.md` never had
(it stops at bookkeeping-for-humans and never touches data modelling). Applies `sql-essentials`'
general relational-modelling skills to one domain rather than re-teaching them.

## Why this exists · the big idea

- **The problem before the solution**: a systems builder's first real question after #1 is "what does
  this look like as tables?" — left unanswered, the reader guesses, and every later course inherits
  whatever guess they made.
- **Keep-this-if-you-forget-everything**: the double-entry invariant belongs in the schema, not only
  in application code — a check constraint or an equivalent data-layer guarantee catches what a
  forgotten `if` statement would silently let through.
- **Big ideas touched**: `subledger-to-ledger-integrity` — this is the course where systems
  architecture and accounting mechanics first meet as one subject, not two.

## Prerequisites

- **Prior courses**: `accounting-foundations` (#1).
- **Linked (SWE)**: `sql-essentials` — declared in frontmatter, never walked into `courseOrder`; this
  course applies its relational-modelling skills, it does not re-teach them.
- **Assumed knowledge**: #1's mental model; basic relational-database vocabulary (table, foreign key)
  from the linked prerequisite.

## Accuracy notes

- Ledger schema-design patterns (header/line separation, parent/child account hierarchies) are stable
  domain-standard reasoning `[Judgment call — the specific schema shown is originally authored, not
sourced from any textbook or vendor system]`.

## Concepts

- **co-01 · chart-of-accounts-as-schema** — an account type, number, and hierarchy position expressed
  as table columns rather than a paper list.
- **co-02 · numbering-scheme** — numbering ranges are conventional, not standardised, across
  organisations; a scheme is a design choice, not a lookup.
- **co-03 · parent-child-hierarchy** — accounts roll up into summary parents, supporting both
  detail-level posting and summary-level reporting.
- **co-04 · journal-header-vs-lines** — a journal entry is one header row with many line rows, a
  one-to-many structure, not a single flat table.
- **co-05 · normalisation-tradeoffs-for-ledgers** — where normalising helps (avoiding update anomalies
  in account metadata) and where a ledger's append-only nature changes the usual tradeoffs.
- **co-06 · double-entry-invariant-enforcement** — enforcing "debits = credits per journal entry" at
  the data layer, not only in application code.
- **co-07 · check-constraint-vs-application-layer** — a database constraint fails loudly and always;
  application-layer enforcement is bypassable by any code path that forgets to call it.
- **co-08 · computed-vs-stored-balances** — deriving an account balance on read versus maintaining a
  cached running balance, and the correctness/performance tradeoff each makes.
- **co-09 · rollup-queries** — summing child-account balances into a parent total as an applied
  `sql-essentials` skill, not a new concept.
- **co-10 · original-chart-authorship** — no public-domain chart of accounts exists anywhere
  `[Verified, 2026-07-22 grounding run]`; every chart in this corpus is originally authored, never
  copied from a textbook, standard, or vendor system.

## Worked examples

### Beginner

- **ex-01 · design-the-account-table** — design a table for accounts (id, number, name, type, parent)
  — verify a three-level hierarchy round-trips through the design. (co-01, co-02, co-03)
- **ex-02 · design-journal-header-and-lines** — design separate header and line tables for a journal
  entry — verify a two-line entry stores correctly across both tables. (co-04)
- **ex-03 · classify-a-numbering-range** — assign number ranges to the five account types — verify no
  range overlaps another. (co-02)

### Intermediate

- **ex-04 · enforce-balance-by-constraint** — write a check that a journal entry's lines sum to zero
  (debits positive, credits negative, or an equivalent convention) — verify an unbalanced entry is
  rejected. (co-06, co-07)
- **ex-05 · compare-enforcement-layers** — the same unbalanced-entry defect caught by a data-layer
  constraint versus missed by application code that forgot to validate — verify which one catches it.
  (co-07)
- **ex-06 · computed-balance-query** — derive an account's balance by summing its posted lines — verify
  the computed figure against a known correct total. (co-08)
- **ex-07 · cached-balance-tradeoff** — maintain a stored running balance updated on each posting —
  verify it matches the computed figure from ex-06, and name one way it could drift if updates are
  missed. (co-08)
- **ex-08 · rollup-a-parent-account** — sum three child accounts into a parent's rollup total — verify
  the rollup matches independent addition. (co-03, co-09)

### Advanced

- **ex-09 · multi-level-rollup** — a four-level hierarchy rolled up to the top — verify every
  intermediate level's total is internally consistent with its own children. (co-03, co-09)
- **ex-10 · schema-defends-against-one-big-table** — contrast the header/line/account design against a
  single flat table holding every field — verify the flat design cannot express a multi-line entry
  without duplication or nulls. (co-01, co-04, co-05)

## Applied synthesis (no build — A6)

Design and walk an originally-authored small-business chart of accounts end to end: assign account
types and numbers, place two accounts under a shared parent, and post one multi-line journal entry
against it by hand. Verify the entry balances, the parent rollup is correct, and no account number or
naming pattern is copied from any textbook, standard, or vendor system. No schema is implemented in
running code — the synthesis is the design walkthrough itself.

## Read more

- **Database System Concepts** — Silberschatz, Korth & Sudarshan (McGraw-Hill). The standard relational
  database textbook; cited nominatively for the normalisation theory this course applies.
- **IFRS Foundation — IFRS Taxonomy** (ifrs.org). The IFRS Foundation's own structured account/element
  taxonomy is a real, publicly documented example of accounts modelled as data; named nominatively as
  corroboration, never transcribed.

## In which paths

- `conventional-accounting` — Stage 1 · The first ramp boundary — a working, correctly balancing
  ledger by course 3.
- `sharia-accounting` — Stage 1 · same boundary; identical course.

---

← Back to the [syllabus index](../README.md)
