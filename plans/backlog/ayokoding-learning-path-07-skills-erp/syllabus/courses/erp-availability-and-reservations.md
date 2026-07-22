# ERP Availability and Reservations (By Example)

**Course ID**: `erp-availability-and-reservations` · **Format**: By Example · **Language**: — (domain, worked scenarios, no application code).

**Short summary**: ATP calculation, reservation types and priority, allocation under scarcity

**Scope note**: closes the production-planning cluster — available-to-promise (ATP) calculation and
reservations, one of the domain's hard parts [Repo-grounded — domain-research grounding, Part 2].
Transitively requires `inventory-and-cogs-accounting` via its two ERP prerequisites.
License-aware (DD-15).

## Why this exists · the big idea

- **The problem before the solution**: promising a delivery date to a customer requires knowing what
  is actually available to sell — not just on-hand stock, but on-hand minus reservations plus
  confirmed incoming supply.
- **Keep-this-if-you-forget-everything**: a reservation removes quantity from ATP the moment it is
  created, whether or not the reserved order ever actually ships.
- **Big ideas touched**: `atp-as-a-computed-view`, not a stored field; `allocation-under-scarcity` —
  who gets the last unit when multiple reservations compete.

## Prerequisites

- **ERP prereqs**: [`inventory-and-warehouse-management`](./inventory-and-warehouse-management.md),
  [`production-planning-and-mrp`](./production-planning-and-mrp.md).
- **Accounting prereqs**: `inventory-and-cogs-accounting` (transitive via courses 14 and 18).
- **Assumed knowledge**: course 14's stock-type vocabulary; course 18's planned-order vocabulary.

## Accuracy notes (pending A4 verification)

> The Phase 1.2a confirmation pass has not yet run for this course.

- Worked ATP examples use an originally-authored dataset throughout.
- Concept co-08 is placed on domain-reasoning grounds rather than sourced from the grounding research,
  and is `[Needs Verification]` pending the Phase 1.2a coverage pass.

## Concepts

- **co-01 · atp-calculation** — on-hand plus confirmed incoming supply minus existing reservations,
  computed at the moment of the query.
- **co-02 · reservation** — a quantity earmarked against a specific demand, removed from ATP
  immediately upon creation.
- **co-03 · reservation-priority** — a rule determining which reservation wins when total demand
  exceeds available supply.
- **co-04 · allocation-under-scarcity** — the mechanism (e.g. pro-rata, priority-based) for splitting
  insufficient supply across competing reservations.
- **co-05 · atp-vs-on-hand** — ATP is a computed view, not a stored quantity — it changes as
  reservations and confirmed supply change, without any physical stock movement.
- **co-06 · reservation-release** — releasing a reservation returns its quantity to ATP.
- **co-07 · atp-conflict** — two near-simultaneous orders both querying ATP and both receiving a
  "yes", when only one can actually be fulfilled — a race condition analogous to course 16's
  concurrency treatment.
- **co-08 · atp-horizon-and-promise-date** — an ATP query answers "how much, by when": confirmed
  supply arriving after the requested date does not make a quantity promisable on that date, so the
  answer is a date-qualified quantity rather than a single number, and the horizon bounds how far
  forward incoming supply may be counted at all.

## Worked examples

Prose-based worked scenarios with originally-authored data (no runnable code). Every example cites
the `co-NN` it exercises.

### Beginner

- **ex-01 · atp-calculation-basic** — given on-hand stock, confirmed incoming supply, and existing
  reservations, compute ATP. (co-01)
- **ex-02 · reservation-creation-effect** — create a new reservation and show ATP decreases
  immediately. (co-02)

### Intermediate

- **ex-03 · allocation-under-scarcity** — given total demand exceeding available supply, apply an
  allocation rule and show the resulting split. (co-03, co-04)
- **ex-04 · reservation-release-effect** — release a reservation and show ATP increases accordingly.
  (co-06)

### Advanced

- **ex-05 · atp-conflict-demonstration** — given two near-simultaneous orders each querying ATP against
  the same limited stock, show how both can receive a "yes" without a locking mechanism, echoing
  course 16's concurrency-race treatment. (co-07)
- **ex-06 · promise-date-from-later-supply** — given on-hand stock insufficient for an order and a
  confirmed receipt dated later in the horizon, determine the quantity promisable on the requested
  date and the earliest date the full quantity becomes promisable. (co-01, co-05, co-08)

## Synthesis exercise — intra-topic

> Analysis and design only — never build, implement, or deploy a system (`A6`).

- **Goal**: given a scarce-supply scenario with three competing orders, compute ATP, apply an
  allocation rule, and write an analysis of how an ATP-conflict race could still occur and what
  detection or prevention approach (conceptual, not implemented) would address it.
- **Concepts exercised**: [ ] ATP calculation (co-01) [ ] allocation under scarcity (co-03, co-04) [ ]
  ATP conflict (co-07).
- **Ordered steps**: 1) compute ATP; 2) apply the allocation rule across the three orders; 3) describe
  an ATP-conflict scenario; 4) propose a conceptual prevention approach.
- **Acceptance criteria**: the allocation is arithmetically correct and respects the stated priority
  rule; the conflict scenario is realistic and its prevention approach is coherent.
- **Done bar**: a written analysis, no code, no system built.

## Read more

> Populated during the Phase 1.2a `web-researcher` confirmation pass (`A12`) — coverage-check
> citations only.

## In which paths

- `skills/conventional-erp` — Stage B, course 20 of 27.
- `skills/sharia-erp` — Stage B, course 20 of 30.

---

← Back to the [syllabus index](../README.md)
