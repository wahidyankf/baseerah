# Demand and Supply Planning (Annotated-concept)

**Course ID**: `demand-and-supply-planning` · **Format**: Annotated-concept · **Language**: — (domain, no code).

**Short summary**: Forecasting inputs, supply horizons, safety stock, planning hierarchy

**Scope note**: sits above MRP's mechanical netting (course 18) — where the demand MRP consumes
actually comes from, and how a planning hierarchy moves between aggregate and detailed views.
Transitively requires `inventory-and-cogs-accounting` via course 18. License-aware (DD-15).

## Why this exists · the big idea

- **The problem before the solution**: MRP (course 18) is only as good as the demand it is fed — if
  the forecast is wrong or the safety-stock policy is mismatched to real demand variability, netting
  will faithfully compute the wrong answer.
- **Keep-this-if-you-forget-everything**: safety stock exists to absorb forecast error, not to pad
  inventory arbitrarily — sizing it requires knowing how uncertain the forecast actually is.
- **Big ideas touched**: `forecast-consumption` — actual orders reduce a forecast rather than adding
  to it; `planning-hierarchy` — aggregate plans disaggregate into detailed ones, not the reverse.

## Prerequisites

- **Prior topics**: [`production-planning-and-mrp`](./production-planning-and-mrp.md).
- **Cross-domain prerequisites**: none.
- **Assumed knowledge**: course 18's demand-source and netting vocabulary.

## Accuracy notes (pending A4 verification)

> The Phase 1.2a confirmation pass has not yet run for this course.

- Forecasting-method names (moving average, exponential smoothing) are treated at concept depth only;
  no specific vendor's forecasting-algorithm implementation is described.
- Concept co-08 is placed on domain-reasoning grounds rather than sourced from the grounding research,
  and is `[Needs Verification]` pending the Phase 1.2a coverage pass.

## Concepts

- **co-01 · forecast-as-demand-input** — a statistical estimate of future demand, one of MRP's demand
  sources (course 18).
- **co-02 · forecast-consumption** — an actual order reduces the remaining forecast for its period
  rather than adding to total demand.
- **co-03 · safety-stock** — a buffer quantity sized to absorb forecast error, not an arbitrary
  padding.
- **co-04 · reorder-point** — the on-hand level that triggers a new replenishment order, distinct from
  MRP's period-by-period netting.
- **co-05 · planning-horizon** — how far into the future a plan extends, and why longer horizons carry
  more forecast uncertainty.
- **co-06 · planning-hierarchy** — an aggregate plan (product family, region) disaggregates into
  detailed plans (individual SKU, individual location).
- **co-07 · demand-variability-and-buffer-sizing** — safety stock sizing depends on how variable actual
  demand is relative to the forecast, not a fixed rule of thumb.
- **co-08 · master-production-schedule** — the committed period-by-period build/buy quantities for
  finished items, sitting between the aggregate plan (co-06) and MRP's netting run (course 18): it
  states what is actually promised for production, not what the forecast merely predicted, and it is
  the schedule a rough-cut capacity check (course 18) is run against.

## Worked examples

Prose-based worked scenarios (no runnable code). Every example cites the `co-NN` it exercises.

### Beginner

- **ex-01 · forecast-consumption-trace** — given a period's forecast and an incoming order, show the
  remaining forecast after consumption. (co-01, co-02)
- **ex-02 · reorder-point-vs-mrp-contrast** — given the same item under a reorder-point policy and
  under MRP netting (course 18), contrast when each would trigger a replenishment. (co-04)

### Intermediate

- **ex-03 · safety-stock-sizing** — given two items with different demand variability but the same
  average demand, explain why they warrant different safety-stock levels. (co-03, co-07)
- **ex-04 · planning-hierarchy-disaggregation** — given an aggregate product-family plan, disaggregate
  it into individual SKU-level plans. (co-06)
- **ex-05 · forecast-to-mps-to-mrp-trace** — given an aggregate forecast and its disaggregation,
  derive the master production schedule for one finished item, then identify which figure MRP (course 18) actually consumes as demand and which figure it never sees. (co-01, co-06, co-08)

## Synthesis exercise — intra-topic

> Analysis and design only — never build, implement, or deploy a system (`A6`).

- **Goal**: design a demand-and-supply plan for a new product family, including forecast consumption
  logic, a safety-stock policy justified by stated demand variability, and a two-level planning
  hierarchy.
- **Concepts exercised**: [ ] forecast consumption (co-02) [ ] safety stock (co-03, co-07) [ ]
  planning hierarchy (co-06).
- **Ordered steps**: 1) state the forecast and consumption rule; 2) size safety stock with reasoning; 3) disaggregate the aggregate plan.
- **Acceptance criteria**: the safety-stock size is justified by a stated variability figure, not
  arbitrary; disaggregation sums correctly back to the aggregate.
- **Done bar**: a written plan, no code, no system built.

## Read more

> Populated during the Phase 1.2a `web-researcher` confirmation pass (`A12`) — coverage-check
> citations only.

## In which paths

- `skills/conventional-erp` — Stage B, course 19 of 27.
- `skills/sharia-erp` — Stage B, course 19 of 30.

---

← Back to the [syllabus index](../README.md)
