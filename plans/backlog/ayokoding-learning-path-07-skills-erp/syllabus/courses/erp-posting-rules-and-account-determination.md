# ERP Posting Rules and Account Determination (By Example)

**Course ID**: `erp-posting-rules-and-account-determination` · **Format**: By Example · **Language**: — (domain, worked scenarios, no application code).

**Short summary**: How a business transaction becomes an accounting entry, and how it silently fails

**Scope note**: account determination is where a business transaction becomes an accounting entry —
the mechanism that decides which GL accounts a goods receipt or an invoice posts to. Getting this
wrong is a silent failure: the trial balance still balances even when a transaction posts to the wrong
account. Precedes course 6's subledger-to-GL architecture treatment. License-aware (DD-15).

## Why this exists · the big idea

- **The problem before the solution**: a misconfigured account-determination rule produces a
  misclassification, not an imbalance — the trial balance still balances, so this class of error is
  invisible to the most obvious check.
- **Keep-this-if-you-forget-everything**: "which account" (determination) and "which side" (posting
  key) are deliberately separate mechanisms, not redundant ones.
- **Big ideas touched**: `silent-failure-by-design` — the domain's characteristic failure mode, named
  concretely here for the first time; `configuration-over-hardcoding` — table-driven determination is
  what makes account rules configurable per business.

## Prerequisites

- **Prior topics**: [`erp-document-lifecycle-and-state-machines`](./erp-document-lifecycle-and-state-machines.md).
- **Cross-domain prerequisites**: none.
- **Assumed knowledge**: course 4's document state-machine vocabulary (posted state in particular).

## Accuracy notes (pending A4 verification)

> The Phase 1.2a confirmation pass has not yet run for this course.

- Worked examples use an originally-authored chart of accounts and transaction dataset
  (safe-authoring rule 4 — no chart of accounts is lifted from any reference implementation).

## Concepts

- **co-01 · determination-as-lookup** — determination is a lookup keyed by transaction type,
  material/item category, and organizational dimension (company code, plant).
- **co-02 · condition-tables** — a table-driven design, not hardcoded account numbers, is what makes
  account determination configurable per business.
- **co-03 · gr-posting** — goods receipt posts inventory account debit, GR/IR clearing account credit.
- **co-04 · ir-posting** — invoice receipt posts GR/IR clearing debit, vendor payable credit.
- **co-05 · posting-key** — the mechanism fixing debit/credit side independent of account
  determination.
- **co-06 · separation-of-which-account-and-which-side** — a deliberate design choice, not redundancy.
- **co-07 · misconfigured-determination** — a condition-table entry pointing at the wrong GL account.
- **co-08 · trial-balance-blindness** — why the trial balance still balances even when a
  misclassification has occurred.
- **co-09 · detection-in-practice** — how misclassification is actually caught: account-level review,
  not automated balancing checks.
- **co-10 · account-determination-vs-subledger** — how determination output feeds the subledger-to-GL
  relationship (deep dive: course 6).

## Worked examples

Prose-based worked scenarios with originally-authored data (no runnable code, no lifted chart of
accounts). Every example cites the `co-NN` it exercises.

### Beginner

- **ex-01 · determination-table-read** — given a condition table, look up the GL account for a sample
  transaction. (co-01, co-02)
- **ex-02 · gr-ir-trace** — trace one procurement transaction through both the GR and IR postings.
  (co-03, co-04)

### Intermediate

- **ex-03 · posting-key-vs-determination** — given a transaction, separately identify its posting key
  (side) and its determined account, and explain why they are set independently. (co-05, co-06)
- **ex-04 · misconfiguration-injection** — given a condition table with one deliberately wrong entry,
  identify which transaction posts incorrectly. (co-07)
- **ex-05 · trial-balance-still-balances** — show, with the misconfigured entry from ex-04, that the
  trial balance still balances — verify the debit/credit totals are unaffected by the
  misclassification. (co-08)

### Advanced

- **ex-06 · detection-method-design** — design an account-level review check that would have caught
  ex-04's misconfiguration, and explain why a balance check alone would not. (co-09)

## Synthesis exercise — intra-topic

> Analysis and design only — never build, implement, or deploy a system (`A6`).

- **Goal**: design a condition-table-driven account-determination scheme for a new transaction type,
  and write a worked example demonstrating a misconfiguration and its silent-failure consequence.
- **Concepts exercised**: [ ] condition tables (co-01, co-02) [ ] misconfiguration (co-07) [ ] trial
  balance blindness (co-08) [ ] detection (co-09).
- **Ordered steps**: 1) design the condition table; 2) write a correct worked posting; 3) introduce one
  misconfigured entry; 4) show the trial balance is unaffected; 5) propose a detection check.
- **Acceptance criteria**: the misconfiguration produces a real misclassification, not an imbalance;
  the detection check would actually catch it.
- **Done bar**: a written worked example with an originally-authored dataset, no code, no system built.

## Read more

> Populated during the Phase 1.2a `web-researcher` confirmation pass (`A12`) — coverage-check
> citations only.

## In which paths

- `skills/conventional-erp` — Stage A, course 5 of 27.
- `skills/sharia-erp` — Stage A, course 5 of 30.

---

← Back to the [syllabus index](../README.md)
