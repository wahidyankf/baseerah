# ERP Conceptual Data Model (Annotated-concept)

**Course ID**: `erp-conceptual-data-model` · **Format**: Annotated-concept · **Language**: — (domain, no code).

**Short summary**: Master data types, the header/line document shape, multi-entity accommodation

**Scope note**: the shared vocabulary of master data and the header/line document pattern almost every
ERP transaction reuses, plus a first introduction to the extensibility axis (deep dive: course 21).
Foundation for course 4's document-lifecycle treatment and course 17's BOM/routing treatment.
License-aware (DD-15).

## Why this exists · the big idea

- **The problem before the solution**: without a shared vocabulary for "master data" and "document",
  every later course would have to redefine its own terms — this course fixes them once.
- **Keep-this-if-you-forget-everything**: almost every ERP transaction — purchase order, sales order,
  invoice, goods movement — reuses the same header/line shape; recognizing that shape is the fastest
  way to understand an unfamiliar document type.
- **Big ideas touched**: `master-data-as-shared-identity` — a duplicate or malformed master record
  propagates errors into every transaction referencing it; `extensibility-as-open-axis` — EAV vs JSONB
  vs generated schema, introduced here, resolved in course 21.

## Prerequisites

- **Prior topics**: [`erp-foundations-and-history`](./erp-foundations-and-history.md).
- **Cross-domain prerequisites**: none.
- **Assumed knowledge**: course 1's integration-promise framing.

## Accuracy notes (pending A4 verification)

> The Phase 1.2a confirmation pass has not yet run for this course.

- The extensibility axis (EAV vs JSONB vs generated schema) is `[Repo-grounded]` in the domain-research
  grounding file, Part 2; its full trade-off analysis is deferred to course 21.
- Master-data and header/line terminology is generic across the industry `[Judgment call]`, not tied
  to any single vendor's proprietary field names.

## Concepts

- **co-01 · item-material-master** — the shared identity of a thing bought, made, or sold, referenced by
  every transaction that touches it.
- **co-02 · business-partner-master** — customer and vendor as often the same underlying entity type,
  distinguished by role rather than a separate schema.
- **co-03 · gl-account-master** — the account master as the target of every posting (deep dive: course
  5).
- **co-04 · cost-center-profit-center** — organizational dimensions attached to a transaction
  independent of the GL account itself.
- **co-05 · master-data-governance** — why a duplicate or malformed master record propagates errors
  into every transaction that references it.
- **co-06 · header-line-pattern** — header fields (partner, dates, currency, status) vs line fields
  (item, quantity, price, account).
- **co-07 · totals-and-rounding** — totals and rounding rules differ at the header level vs the line
  level, and the two must reconcile.
- **co-08 · company-code-legal-entity** — the organizational dimension that scopes a document to one
  legal entity (deep dive: course 24).
- **co-09 · multi-currency-fields** — currency fields at document, line, and local-reporting level,
  present even in a single-currency deployment.
- **co-10 · extensibility-axis-introduced** — EAV, JSONB/schemaless columns, and generated schema as
  three approaches to extending a data model, a genuine open design axis.

## Worked examples

Prose-based worked scenarios (no runnable code). Every example cites the `co-NN` it exercises.

### Beginner

- **ex-01 · master-data-identify** — given a sample purchase order, identify every master-data
  reference it makes (item, vendor, GL account). (co-01, co-02, co-03)
- **ex-02 · header-line-split** — given a sample invoice, split its fields into header vs line. (co-06)
- **ex-03 · duplicate-master-consequence** — trace how a duplicate vendor master record produces two
  separate payment histories for the same real vendor. (co-05)

### Intermediate

- **ex-04 · totals-rounding-mismatch** — a scenario where line-level rounding sums to a different total
  than header-level rounding — identify the discrepancy and its size. (co-07)
- **ex-05 · multi-entity-field-audit** — given a single-entity deployment's document schema, identify
  which fields exist "for later" multi-entity readiness. (co-08, co-09)
- **ex-06 · extensibility-tradeoff-preview** — given a requirement to add a custom field, sketch how EAV,
  JSONB, and generated schema would each represent it, without choosing yet. (co-10)

## Synthesis exercise — intra-topic

> Analysis and design only — never build, implement, or deploy a system (`A6`).

- **Goal**: given a new document type's business requirement (e.g. a return-merchandise
  authorization), design its master-data references and header/line field layout on paper.
- **Concepts exercised**: [ ] master data types (co-01, co-02, co-03) [ ] header/line pattern (co-06)
  [ ] totals/rounding (co-07).
- **Ordered steps**: 1) list the master-data types the new document must reference; 2) draft header
  fields; 3) draft line fields; 4) state the totals/rounding rule at each level.
- **Acceptance criteria**: every referenced master-data type is named; header/line split is correct
  per the pattern; a rounding rule is stated for both levels.
- **Done bar**: a written design, no schema created, no code.

## Read more

> Populated during the Phase 1.2a `web-researcher` confirmation pass (`A12`) — coverage-check
> citations only.

## In which paths

- `skills/conventional-erp` — Stage A, course 2 of 26.
- `skills/sharia-erp` — Stage A, course 2 of 29.

---

← Back to the [syllabus index](../README.md)
