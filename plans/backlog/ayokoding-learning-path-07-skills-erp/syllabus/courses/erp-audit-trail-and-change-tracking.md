# ERP Audit Trail and Change Tracking (Annotated-concept)

**Course ID**: `erp-audit-trail-and-change-tracking` · **Format**: Annotated-concept · **Language**: — (domain, no code).

**Short summary**: Change-history capture, reversal vs deletion semantics, audit trail as a control

**Scope note**: closes Stage A's cross-cutting-spine sequence — how an ERP captures who changed what
and when, and why a reversal preserves history while a deletion would not. Introduces the audit trail
as a control, with the deep SOX/COSO treatment deferred to course 26. License-aware (DD-15).

## Why this exists · the big idea

- **The problem before the solution**: without change tracking, a dispute over "who changed this and
  when" has no answer — and worse, a deletion can erase the evidence a reversal would have preserved.
- **Keep-this-if-you-forget-everything**: reversal, not deletion, is how a posted document's effects
  are undone — the original record stays, offset by a new one.
- **Big ideas touched**: `who-what-when-capture`; `audit-trail-as-a-control`, previewed here and
  detailed in course 26's RBAC/SoD/COSO treatment.

## Prerequisites

- **Prior topics**: [`erp-document-lifecycle-and-state-machines`](./erp-document-lifecycle-and-state-machines.md).
- **Cross-domain prerequisites**: none.
- **Assumed knowledge**: course 4's reversal-vs-cancellation-vs-correction vocabulary.

## Accuracy notes (pending A4 verification)

> The Phase 1.2a confirmation pass has not yet run for this course.

- The deep SOX/COSO control-mapping treatment is deferred to course 26 and not duplicated here; this
  course previews "audit trail as a control" conceptually only.

## Concepts

- **co-01 · change-document** — a record capturing what changed on a master or transactional record.
- **co-02 · who-what-when-capture** — user, field, old value, new value, and timestamp as the minimum
  captured set.
- **co-03 · reversal-preserves-history** — a reversal is a new document; the original stays visible.
- **co-04 · deletion-erases-evidence** — why deletion of a posted document is normally disallowed.
- **co-05 · change-document-vs-transaction-history** — change documents track master-data edits;
  transaction history tracks document lifecycle events — related but distinct records.
- **co-06 · audit-trail-as-control-preview** — the audit trail is not just a log; it is a control an
  auditor relies on (deep dive: course 26).
- **co-07 · immutable-posted-record** — once posted, a document's core fields are not editable in
  place, tying back to course 4's correction-avoids-mutation concept.
- **co-08 · retention-and-accessibility** — the change record must remain accessible for the retention
  period relevant to the business, not just exist somewhere.

## Worked examples

Prose-based worked scenarios (no runnable code). Every example cites the `co-NN` it exercises.

### Beginner

- **ex-01 · change-document-read** — given a change document, identify who changed what field and
  when. (co-01, co-02)
- **ex-02 · reversal-vs-deletion-contrast** — given a posted invoice needing correction, contrast what
  a reversal preserves versus what a deletion would erase. (co-03, co-04)

### Intermediate

- **ex-03 · change-document-vs-transaction-history** — given a scenario with both a master-data edit
  and a document status change, correctly attribute each to the right record type. (co-05)
- **ex-04 · immutability-violation-detect** — given a log showing a posted document's core field
  edited in place, flag the violation. (co-07)

## Synthesis exercise — intra-topic

> Analysis and design only — never build, implement, or deploy a system (`A6`).

- **Goal**: design the change-document capture scheme for a new master-data type, and write a worked
  example contrasting a correct reversal with an incorrect in-place edit.
- **Concepts exercised**: [ ] who-what-when capture (co-02) [ ] reversal preserves history (co-03) [ ]
  immutable posted record (co-07).
- **Ordered steps**: 1) list the fields the change document must capture; 2) write a correct reversal
  scenario; 3) write an incorrect in-place-edit scenario; 4) explain why the second violates the
  audit trail.
- **Acceptance criteria**: the change document captures user/field/old/new/timestamp; the two
  scenarios are clearly distinguished.
- **Done bar**: a written design and worked example, no code, no system built.

## Read more

> Populated during the Phase 1.2a `web-researcher` confirmation pass (`A12`) — coverage-check
> citations only.

## In which paths

- `skills/conventional-erp` — Stage A, course 9 of 27.
- `skills/sharia-erp` — Stage A, course 9 of 30.

---

← Back to the [syllabus index](../README.md)
