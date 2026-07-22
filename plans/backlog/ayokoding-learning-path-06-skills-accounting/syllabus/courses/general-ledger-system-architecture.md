# General Ledger System Architecture (By Example)

**Course ID**: `general-ledger-system-architecture` · **Format**: By Example. **NEW course, replacing
`capstone-build-a-general-ledger-system` (A6).**

**Short summary**: How a general ledger system is architected — the posting engine, subledger
integration, and period-close orchestration — described, not built.

**Scope note**: architecture only. This course describes how a ledger system is structured in detail;
it does not ask the reader to build, scaffold, or extend one (A6). It carries the same
`backend-essentials` (SWE) linked cross-domain prerequisite the deleted capstone carried.

## Why this exists · the big idea

- **The problem before the solution**: every course from #1 through #18 has taught a piece of what a
  ledger system must do; none of them has stepped back to show how those pieces fit into one
  architecture. Without this course, a reader finishes the shared spine with nineteen separate
  mechanisms and no picture of the system they compose.
- **Keep-this-if-you-forget-everything**: the general ledger is the point where every subledger (AP,
  AR, fixed assets, payroll) converges — architecting that convergence correctly, so no subledger can
  post an unbalanced or untraceable entry into the GL, is the single highest-leverage architectural
  decision in the whole system.
- **Big ideas touched**: `subledger-to-ledger-integrity` — this course's headline architectural
  concern, generalising every prior course's individual subledger into one coherent system view.

## Prerequisites

- **Prior courses**: `chart-of-accounts-and-data-modeling` (#2),
  `financial-statements-and-close-cycle` (#3).
- **Linked (SWE)**: `backend-essentials` — declared in frontmatter, never walked into `courseOrder`;
  carries forward the same linked edge the deleted `capstone-build-a-general-ledger-system` carried
  (DD-607).
- **Assumed knowledge**: #2's schema, #3's close cycle, and every subledger course's mechanics
  (#4–#18) as the material this course architecturally unifies.

## Accuracy notes

- General-ledger architecture patterns (posting engine, subledger integration, close orchestration)
  are domain-standard software-architecture reasoning
  `[Judgment call — no single canonical textbook source; patterns are cross-checked conceptually against permissively-licensed reference implementations]`
  per `DD-15`, see the reference-implementation-landscape concept below.
- **Relationship to `DD-15` (License-aware technology choices, inherited via plan 02's corpus)** — this
  course names real accounting software; per `DD-15`'s precedent, ledger-cli (BSD-3-Clause) and Apache
  Fineract (Apache-2.0) are named as permissively-licensed examples a reader could study directly;
  GnuCash (GPLv2+), hledger (GPLv3), and Beancount (GPL-2.0-only) are described behaviourally only,
  never quoted from, per this plan's own eleven safe-authoring rules (see
  [tech-docs §Licensing](../../tech-docs.md#licensing-and-ip-compliance-a8)).

## Concepts

- **co-01 · posting-engine-architecture** — the component responsible for accepting, validating, and
  committing journal entries — the architectural home for #4's posting mechanics.
- **co-02 · subledger-integration-pattern** — how AP, AR, fixed assets, and payroll subledgers post
  summarised or detailed entries into the GL, and where the boundary between subledger detail and GL
  summary sits.
- **co-03 · idempotent-posting** — designing the posting engine so that retrying a failed post cannot
  double-post the same transaction — a reliability property, not a bookkeeping rule.
- **co-04 · period-close-orchestration** — the system-level sequencing of subledger closes, adjusting
  entries, and the GL lock, extending #3's close-cycle state machine to a multi-subledger system.
- **co-05 · audit-trail-as-architecture-property** — #4's "never delete, always reverse" pattern
  implemented as a system property (append-only storage, immutable posted entries) rather than a
  convention someone might forget.
- **co-06 · reconciliation-architecture** — the system components that continuously verify each
  subledger's total ties to its corresponding GL control account.
- **co-07 · multi-entity-architecture** — how a system built for one entity extends to support #13's
  multi-entity consolidation without duplicating the posting engine per entity.
- **co-08 · batch-vs-event-driven-posting-architecture** — architecting for scheduled batch posting,
  real-time event-driven posting, or a hybrid, and the consistency/latency tradeoffs of each,
  extending #4's batch-vs-real-time concept to the system level.
- **co-09 · reference-implementation-landscape** — the licensing posture of real, named
  general-ledger-adjacent open-source projects, per `DD-15`: ledger-cli and Apache Fineract
  (permissive, safe to study directly); GnuCash, hledger, and Beancount (copyleft, described
  behaviourally only).

## Worked examples

### Beginner

- **ex-01 · sketch-the-posting-engine-boundary** — sketch (on paper, no code) the posting engine's
  inputs (a validated journal entry) and outputs (a committed, immutable ledger entry) — verify the
  sketch names both the validation step and the immutability guarantee. (co-01, co-05)
- **ex-02 · map-a-subledger-to-a-control-account** — map #6's AP subledger to its GL control account —
  verify every AP subledger entry's summary lands in exactly one control account. (co-02, co-06)

### Intermediate

- **ex-03 · design-for-idempotent-retry** — design (on paper) how a posting request that times out
  mid-commit can be safely retried without double-posting — verify the design names the mechanism
  (e.g. an idempotency key) that prevents duplication. (co-03)
- **ex-04 · sequence-a-multi-subledger-close** — sequence the order in which AP, AR, fixed assets, and
  payroll subledgers must close relative to the GL lock — verify the sequence prevents a late subledger
  post from landing in an already-locked period. (co-04)
- **ex-05 · design-a-reconciliation-check** — design a reconciliation check that compares a subledger's
  total against its GL control account balance — verify the check would have caught #6's ex-10
  liability-booked-without-match failure. (co-06)
- **ex-06 · choose-batch-vs-event-driven** — for a stated business (high transaction volume, needs
  near-real-time cash visibility) choose batch or event-driven posting architecture — verify the choice
  against the stated latency requirement, not a default preference. (co-08)

### Advanced

- **ex-07 · design-the-multi-entity-posting-path** — extend ex-01's posting engine sketch to support
  #13's multi-entity structure without duplicating the engine per entity — verify the design shares
  the posting engine while keeping each entity's ledger data isolated. (co-07)
- **ex-08 · reconciliation-gap-failure** — a system with a subledger-to-GL integration but no automated
  reconciliation check — verify each individual subledger and the GL each balance internally while a
  subledger-to-GL drift (caused by a missed integration post) goes undetected, and name the
  architectural fix (co-06's reconciliation check). (co-02, co-06, silent-failure)

## Applied synthesis (no build — A6)

On paper, architect the posting path for one new subledger (choose: a simple expense-reimbursement
subledger) integrating into the GL — naming its control account, its idempotency mechanism, its close
sequencing relative to the existing subledgers, and its reconciliation check. Verify the design answers
all four questions concretely and does not silently assume any of them away. No system is built, no
code is scaffolded — the synthesis is the architectural design document itself.

## Read more

- **Designing Data-Intensive Applications** — Martin Kleppmann (O'Reilly). Cited nominatively for
  idempotency, event-driven architecture, and reconciliation-pattern reasoning applied here to a
  ledger system specifically.
- **ledger-cli** (BSD-3-Clause) and **Apache Fineract** (Apache-2.0) — real, permissively licensed
  reference implementations named nominatively per `DD-15`; never quoted from.

## In which paths

- `conventional-accounting` — Stage 2 · Most conventional systems a mid-size company runs, plus how to
  architect (not build) a ledger system. Terminal course for this manifest.
- `sharia-accounting` — Stage 2 · same; the shared spine both paths cover identically. Continues into
  Stage 3.

---

← Back to the [syllabus index](../README.md)
