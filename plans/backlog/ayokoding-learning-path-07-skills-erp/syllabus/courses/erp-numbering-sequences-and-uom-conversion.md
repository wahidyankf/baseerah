# ERP Numbering Sequences and UoM Conversion (Annotated-concept)

**Course ID**: `erp-numbering-sequences-and-uom-conversion` · **Format**: Annotated-concept · **Language**: — (domain, no code).

**Short summary**: Number-range objects, gapless vs gapped sequences, unit-of-measure conversion

**Scope note**: two cross-cutting-spine mechanics that share a course because both are quiet sources
of downstream error when mishandled — document numbering sequences and unit-of-measure conversion.
License-aware (DD-15).

## Why this exists · the big idea

- **The problem before the solution**: a numbering gap can be a compliance question (some
  jurisdictions require gapless invoice numbering) and a UoM conversion error can silently understate
  or overstate a quantity by orders of magnitude.
- **Keep-this-if-you-forget-everything**: numbering and UoM conversion look like small mechanical
  details but each carries a real failure mode with downstream consequences.
- **Big ideas touched**: `gapless-vs-gapped-as-a-jurisdictional-question`; `conversion-error-propagation`.

## Prerequisites

- **Prior topics**: [`erp-module-map-and-architecture`](./erp-module-map-and-architecture.md).
- **Cross-domain prerequisites**: none.
- **Assumed knowledge**: course 3's module-map vocabulary.

## Accuracy notes (pending A4 verification)

> The Phase 1.2a confirmation pass has not yet run for this course.

- Whether a specific jurisdiction requires gapless invoice numbering is `[Needs Verification]` —
  stated as a general compliance concern, not a specific jurisdictional citation, in this course.

## Concepts

- **co-01 · number-range-object** — the mechanism that assigns the next sequential number to a new
  document.
- **co-02 · gapless-sequence** — a sequence with no skipped numbers, often a compliance requirement.
- **co-03 · gapped-sequence** — a sequence that may legitimately skip numbers (e.g. on a cancelled
  draft), acceptable in most non-fiscal document types.
- **co-04 · numbering-across-multi-entity** — how numbering ranges are scoped per legal entity in a
  multi-entity system (preview of course 25).
- **co-05 · uom-conversion-table** — the table mapping one unit of measure to another for the same
  item (e.g. box to each).
- **co-06 · rounding-in-conversion** — rounding rules applied during UoM conversion, and why they
  matter more than they look.
- **co-07 · cross-uom-error-propagation** — how a conversion error compounds across a multi-step
  process (receive in one UoM, issue in another).
- **co-08 · numbering-as-audit-signal** — numbering gaps and sequence integrity as an input to the
  audit-trail treatment (deep dive: course 9).

## Worked examples

Prose-based worked scenarios (no runnable code). Every example cites the `co-NN` it exercises.

### Beginner

- **ex-01 · number-range-assignment** — given a sequence of document creations and cancellations,
  assign numbers and classify the result as gapless or gapped. (co-01, co-02, co-03)
- **ex-02 · uom-conversion-lookup** — given a conversion table, convert a quantity from one UoM to
  another. (co-05)

### Intermediate

- **ex-03 · rounding-error-demonstration** — given a conversion with a non-integer ratio, show how
  rounding at each step produces a different final quantity than rounding once at the end. (co-06,
  co-07)
- **ex-04 · multi-entity-numbering-scope** — given two legal entities issuing invoices, verify their
  number ranges do not collide. (co-04)

## Synthesis exercise — intra-topic

> Analysis and design only — never build, implement, or deploy a system (`A6`).

- **Goal**: design a numbering scheme and a UoM conversion table for a new item family, and
  demonstrate a rounding-error scenario and its downstream consequence.
- **Concepts exercised**: [ ] number ranges (co-01–co-03) [ ] UoM conversion (co-05) [ ] rounding
  propagation (co-06, co-07).
- **Ordered steps**: 1) design the numbering scheme; 2) design the conversion table; 3) construct a
  rounding-error scenario; 4) quantify its downstream effect.
- **Acceptance criteria**: the numbering scheme states gapless vs gapped explicitly; the rounding
  scenario shows a measurable discrepancy.
- **Done bar**: a written design and worked example, no code, no system built.

## Read more

> Populated during the Phase 1.2a `web-researcher` confirmation pass (`A12`) — coverage-check
> citations only.

## In which paths

- `skills/conventional-erp` — Stage A, course 8 of 27.
- `skills/sharia-erp` — Stage A, course 8 of 30.

---

← Back to the [syllabus index](../README.md)
