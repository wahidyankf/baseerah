# ERP Document Lifecycle and State Machines (Annotated-concept)

**Course ID**: `erp-document-lifecycle-and-state-machines` · **Format**: Annotated-concept · **Language**: — (domain, no code).

**Short summary**: Document status as a state machine; cancellation vs reversal vs correction

**Scope note**: the first cross-cutting-spine course proper — every transactional document (purchase
order, sales order, goods receipt, invoice) is a state machine, and the state machine's design
determines what "cancel", "reverse", or "correct" actually means. Architecture in the
domain-driven-design sense, never implementation. License-aware (DD-15).

## Why this exists · the big idea

- **The problem before the solution**: without an explicit state machine, "cancel" and "reverse" get
  conflated, and a posted document can silently revert to draft — corrupting the audit trail (course 9) before it is even built.
- **Keep-this-if-you-forget-everything**: a posted document is never mutated in place; correction
  happens by a new document that references and offsets the original.
- **Big ideas touched**: `document-as-aggregate-root` — status transitions are the aggregate's
  invariants (domain-driven-design tie-in); `history-preservation-over-mutation`.

## Prerequisites

- **Prior topics**: [`erp-module-map-and-architecture`](./erp-module-map-and-architecture.md).
- **Cross-domain prerequisites**: `domain-driven-design` (existing library).
- **Assumed knowledge**: DDD's aggregate/invariant vocabulary from `domain-driven-design`.

## Accuracy notes (pending A4 verification)

> The Phase 1.2a confirmation pass has not yet run for this course.

- State names (draft, released, posted, cancelled) are generic across the industry `[Judgment call]`,
  not tied to a specific vendor's terminology.

## Concepts

- **co-01 · header-line-status-independence** — status fields exist at header and line level
  independently (a partially-delivered order).
- **co-02 · status-as-explicit-field** — status is modeled as an explicit field, never inferred from
  other data.
- **co-03 · draft-state** — a document not yet committed to downstream effects.
- **co-04 · released-approved-state** — a document that has passed an approval gate but not yet posted.
- **co-05 · posted-state** — a document whose effects (e.g. GL postings) are committed.
- **co-06 · cancelled-state** — a document cancelled before it had downstream effects.
- **co-07 · valid-transitions** — the state machine enforces which transitions are legal, preventing a
  posted document from reverting to draft.
- **co-08 · document-as-aggregate-root** — the document's status transitions are its invariants, in
  domain-driven-design terms.
- **co-09 · reversal-vs-cancellation** — reversal creates a new document offsetting a posted one's
  effects, preserving history; cancellation undoes a document before it has effects.
- **co-10 · correction-avoids-mutation** — why ERPs generally avoid mutating a posted document in
  place.
- **co-11 · document-flow-chains** — quote → order → delivery → invoice as a linked chain, each
  document referencing its predecessor.
- **co-12 · broken-chain-as-integrity-failure** — an invoice with no traceable source order is a
  data-integrity failure, not a cosmetic one.

## Worked examples

Prose-based worked scenarios (no runnable code). Every example cites the `co-NN` it exercises.

### Beginner

- **ex-01 · state-machine-sketch** — given a purchase order's lifecycle, sketch its states and valid
  transitions. (co-03–co-07)
- **ex-02 · reversal-vs-cancel-classify** — given five scenarios, classify each as needing a
  cancellation or a reversal. (co-09)

### Intermediate

- **ex-03 · chain-trace** — given a quote-to-invoice document set, trace the reference chain and
  identify a missing link. (co-11, co-12)
- **ex-04 · invalid-transition-detect** — given a log of status changes, identify the one that violates
  the state machine (a posted document reverting to draft). (co-02, co-07)
- **ex-05 · aggregate-invariant-mapping** — map a document's status transitions onto DDD's aggregate
  invariant concept, citing `domain-driven-design`. (co-08)

## Synthesis exercise — intra-topic

> Analysis and design only — never build, implement, or deploy a system (`A6`).

- **Goal**: design a state machine (states + valid transitions) for a new document type (e.g. a
  return-merchandise authorization), and specify what "cancel" vs "reverse" means for it.
- **Concepts exercised**: [ ] states (co-03–co-06) [ ] valid transitions (co-07) [ ] reversal vs
  cancellation (co-09).
- **Ordered steps**: 1) list the document's states; 2) draw valid transitions; 3) define cancel and
  reverse operations for it.
- **Acceptance criteria**: no invalid transition is implied; cancel and reverse are distinctly
  defined and non-overlapping.
- **Done bar**: a written/diagrammed state machine, no code, no system stood up.

## Read more

> Populated during the Phase 1.2a `web-researcher` confirmation pass (`A12`) — coverage-check
> citations only.

## In which paths

- `skills/conventional-erp` — Stage A, course 4 of 27.
- `skills/sharia-erp` — Stage A, course 4 of 30.

---

← Back to the [syllabus index](../README.md)
