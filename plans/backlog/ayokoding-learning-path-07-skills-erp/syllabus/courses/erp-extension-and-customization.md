# ERP Extension and Customization (By Example)

**Course ID**: `erp-extension-and-customization` · **Format**: By Example · **Language**: SQL (schema-extension patterns, illustrative only).

**Short summary**: EAV vs JSONB vs generated schema, custom fields, upgrade-safety trade-offs

**Scope note**: the deep treatment of the extensibility axis first introduced in course 2 — a genuine
open design question [Repo-grounded — domain-research grounding, Part 2], not a solved problem with
one right answer. Authorable in Stage A. License-aware (DD-15).

## Why this exists · the big idea

- **The problem before the solution**: every real ERP deployment eventually needs a custom field or
  table the base schema does not provide, and the three common approaches trade off differently
  against query performance, upgrade safety, and schema clarity.
- **Keep-this-if-you-forget-everything**: there is no universally correct extensibility approach — the
  right choice depends on how often the schema changes, how much ad-hoc querying is needed, and how
  disruptive an upgrade migration is allowed to be.
- **Big ideas touched**: `extensibility-as-genuine-tradeoff`, not a solved problem;
  `upgrade-safety-as-a-first-class-constraint`.

## Prerequisites

- **Prior topics**: [`erp-module-map-and-architecture`](./erp-module-map-and-architecture.md).
- **Cross-domain prerequisites**: `sql-essentials` (existing library).
- **Assumed knowledge**: course 2's extensibility-axis introduction; `sql-essentials`'s relational
  modeling vocabulary.

## Accuracy notes (pending A4 verification)

> The Phase 1.2a confirmation pass has not yet run for this course.

- The EAV/JSONB/generated-schema trade-off is `[Repo-grounded]` in the domain-research grounding file
  as a genuine open design axis; no single approach is asserted as universally correct.

## Concepts

- **co-01 · eav-pattern** — entity-attribute-value: custom fields stored as rows in a generic
  key-value table, queryable without a schema migration.
- **co-02 · eav-tradeoffs** — flexible and migration-free, but query performance and type safety
  degrade as custom-field volume grows.
- **co-03 · jsonb-pattern** — custom fields stored in a single JSON/JSONB column on the base table.
- **co-04 · jsonb-tradeoffs** — better locality than EAV, indexable in modern databases, but still
  weaker type/constraint enforcement than a native column.
- **co-05 · generated-schema-pattern** — custom fields become real, migration-generated columns on the
  base table.
- **co-06 · generated-schema-tradeoffs** — full type safety and query performance, but every
  customization requires a migration, directly affecting upgrade safety.
- **co-07 · upgrade-safety** — how easily a base-schema upgrade can proceed without breaking existing
  customizations, evaluated per approach.
- **co-08 · custom-field-vs-custom-table** — a single custom field differs from a whole custom
  table/entity, and the three approaches apply differently to each.

## Worked examples

Illustrative SQL fragments only (no runnable application, per the course's own scope; this is
schema-shape illustration, not a system to run). Every example cites the `co-NN` it exercises.

### Beginner

- **ex-01 · eav-fragment** — sketch a minimal EAV table shape (`entity_id, attribute, value`) for one
  custom field. (co-01)
- **ex-02 · jsonb-fragment** — sketch a JSONB column holding the same custom field. (co-03)

### Intermediate

- **ex-03 · generated-schema-fragment** — sketch a migration-generated column for the same custom
  field, with a type constraint the other two approaches cannot enforce natively. (co-05, co-06)
- **ex-04 · query-performance-contrast** — describe, without benchmarking code, why a query filtering
  on the custom field is progressively cheaper across EAV → JSONB → generated schema. (co-02, co-04,
  co-06)

### Advanced

- **ex-05 · upgrade-safety-scenario** — given a base-schema upgrade that renames a column, analyze the
  impact on a customization implemented under each of the three approaches. (co-07)

## Synthesis exercise — intra-topic

> Analysis and design only — never build, implement, or deploy a system (`A6`).

- **Goal**: given a customization requirement (one new field, one new small entity), choose an
  extensibility approach for each and justify the choice against query-performance, upgrade-safety,
  and type-safety criteria.
- **Concepts exercised**: [ ] all three patterns (co-01, co-03, co-05) [ ] their trade-offs (co-02,
  co-04, co-06) [ ] upgrade safety (co-07).
- **Ordered steps**: 1) state the requirement; 2) evaluate each approach against the three criteria; 3) choose and justify.
- **Acceptance criteria**: the justification references at least two of the three criteria concretely,
  not just "it's more flexible".
- **Done bar**: a written design with illustrative schema sketches, no runnable migration, no system
  built.

## Read more

> Populated during the Phase 1.2a `web-researcher` confirmation pass (`A12`) — coverage-check
> citations only.

## In which paths

- `skills/conventional-erp` — Stage A, course 22 of 27.
- `skills/sharia-erp` — Stage A, course 22 of 30.

---

← Back to the [syllabus index](../README.md)
