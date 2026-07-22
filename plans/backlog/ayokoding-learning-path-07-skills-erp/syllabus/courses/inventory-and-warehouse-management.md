# Inventory and Warehouse Management (By Example)

**Course ID**: `inventory-and-warehouse-management` · **Format**: By Example · **Language**: — (domain, worked scenarios, no application code).

**Short summary**: Stock types, movement types, warehouse structures, valuation touchpoints

**Scope note**: opens the inventory hard-parts cluster (with courses 15, 16) — stock types, goods
movements, and warehouse structures, ending at the valuation touchpoint that feeds COGS. Requires
`inventory-and-cogs-accounting` from the accounting corpus. License-aware (DD-15).

## Why this exists · the big idea

- **The problem before the solution**: "how much stock do we have" sounds like a single number, but
  it is really several — on-hand, available, in-transit, reserved, inspection-blocked — and conflating
  them is a common source of operational error.
- **Keep-this-if-you-forget-everything**: every stock movement is a typed event (receipt, issue,
  transfer) that both changes a quantity and, eventually, posts a value — the two are linked but not
  identical.
- **Big ideas touched**: `movement-type-as-a-typed-event`; `valuation-as-a-downstream-consequence-of-movement`.

## Prerequisites

- **ERP prereqs**: [`erp-subledger-to-gl-architecture`](./erp-subledger-to-gl-architecture.md).
- **Accounting prereqs**: `inventory-and-cogs-accounting` (from
  `ayokoding-learning-path-06-skills-accounting`).
- **Assumed knowledge**: course 6's subledger/GL vocabulary.

## Accuracy notes (pending A4 verification)

> The Phase 1.2a confirmation pass has not yet run for this course.

- The accounting-side course id `inventory-and-cogs-accounting` is as named in
  `ayokoding-learning-path-06-skills-accounting`'s own in-flight rewrite as of 2026-07-22.
- Concept co-08 and the inspection-blocked stock type added to co-01 are placed on domain-reasoning
  grounds rather than sourced from the grounding research, and are `[Needs Verification]` pending the
  Phase 1.2a coverage pass.

## Concepts

- **co-01 · stock-types** — on-hand, available, in-transit, reserved, and inspection-blocked as
  distinct, coexisting quantities for the same item.
- **co-02 · movement-type** — a typed event (receipt, issue, transfer) that changes a stock quantity.
- **co-03 · goods-receipt-into-stock** — the inventory-side effect of a procurement goods receipt
  (course 10), distinct from its accounting posting.
- **co-04 · goods-issue** — a movement that reduces stock, typically tied to a sales delivery (course 11) or internal consumption.
- **co-05 · stock-transfer** — a movement between two storage locations with no change in ownership.
- **co-06 · storage-location-and-bin** — the warehouse structure a movement is scoped to.
- **co-07 · valuation-touchpoint** — where a stock movement's quantity change becomes a value change
  feeding COGS (deep dive: course 15).
- **co-08 · inspection-blocked-stock-and-usage-decision** — quality management's touchpoint on the
  inventory model: a goods receipt can be routed to an inspection hold rather than straight into
  unrestricted stock, where it counts as on-hand but not as available, until a usage decision either
  releases it to unrestricted stock or rejects it back to the supplier (exception handling: course
  12). This is the concrete reason on-hand and available are separate stock types rather than
  synonyms (deep dive: course 21).

## Worked examples

Prose-based worked scenarios with originally-authored data (no runnable code). Every example cites
the `co-NN` it exercises.

### Beginner

- **ex-01 · stock-type-classification** — given a scenario with on-order, received, and reserved
  quantities, classify each into its correct stock type. (co-01)
- **ex-02 · movement-type-trace** — given a goods receipt and a subsequent goods issue, identify each
  movement's type. (co-02, co-03, co-04)

### Intermediate

- **ex-03 · stock-transfer-trace** — trace a transfer between two storage locations, verifying total
  on-hand quantity is unchanged. (co-05, co-06)
- **ex-04 · valuation-touchpoint-identify** — given a goods issue, identify where its quantity change
  becomes a value posted to COGS. (co-07)
- **ex-05 · inspection-hold-and-usage-decision** — given a goods receipt routed to inspection, state
  the item's on-hand and available quantities before the usage decision, after a release, and after a
  rejection instead. (co-01, co-03, co-08)

## Synthesis exercise — intra-topic

> Analysis and design only — never build, implement, or deploy a system (`A6`).

- **Goal**: design the stock-type and movement-type scheme for a new item family, and write a worked
  example tracing one item from receipt through issue, including its valuation touchpoint.
- **Concepts exercised**: [ ] stock types (co-01) [ ] movement types (co-02) [ ] valuation touchpoint
  (co-07) [ ] inspection hold and usage decision (co-08).
- **Ordered steps**: 1) define the stock types tracked, including the inspection-blocked type; 2)
  define movement types; 3) trace one item's full lifecycle through an inspection hold and its usage
  decision; 4) mark the valuation touchpoint.
- **Acceptance criteria**: stock types are mutually distinct and correctly used; inspection-blocked
  quantity is counted as on-hand but not as available; the valuation touchpoint is correctly placed.
- **Done bar**: a written design and worked example, no code, no system built.

## Read more

> Populated during the Phase 1.2a `web-researcher` confirmation pass (`A12`) — coverage-check
> citations only.

## In which paths

- `skills/conventional-erp` — Stage B, course 14 of 27.
- `skills/sharia-erp` — Stage B, course 14 of 30.

---

← Back to the [syllabus index](../README.md)
