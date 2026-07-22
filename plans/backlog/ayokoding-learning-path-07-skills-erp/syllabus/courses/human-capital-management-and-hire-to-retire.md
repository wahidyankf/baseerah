# Human Capital Management and Hire-to-Retire (Annotated-concept)

**Course ID**: `human-capital-management-and-hire-to-retire` · **Format**: Annotated-concept · **Language**: — (domain, no code).

**Short summary**: The H2R process chain, organizational/position master data, payroll touchpoints

**Scope note**: opens the enterprise-scale cluster (with courses 25-27) — the hire-to-retire flow and
the organizational master data it depends on. Requires `payroll-and-tax-accounting-essentials` from
the accounting corpus. License-aware (DD-15).

## Why this exists · the big idea

- **The problem before the solution**: HCM is easy to treat as "just another module", but its GL
  integration (payroll posting) and its organizational master data (positions, org units) have their
  own structure distinct from the finance/logistics modules covered so far.
- **Keep-this-if-you-forget-everything**: payroll is where HCM's process data (hours worked,
  attendance) becomes accounting data (a GL posting) — the same subledger-to-GL pattern from course 6,
  applied to a new domain.
- **Big ideas touched**: `organizational-master-data-as-its-own-hierarchy`; `payroll-as-a-subledger`.

## Prerequisites

- **ERP prereqs**: [`erp-module-map-and-architecture`](./erp-module-map-and-architecture.md).
- **Accounting prereqs**: `payroll-and-tax-accounting-essentials` (from
  `ayokoding-learning-path-06-skills-accounting`).
- **Assumed knowledge**: course 3's module-map vocabulary; course 6's subledger-to-GL pattern.

## Accuracy notes (pending A4 verification)

> The Phase 1.2a confirmation pass has not yet run for this course.

- The accounting-side course id `payroll-and-tax-accounting-essentials` is as named in
  `ayokoding-learning-path-06-skills-accounting`'s own in-flight rewrite as of 2026-07-22.
- Concept co-08 is placed on domain-reasoning grounds rather than sourced from the grounding research,
  and is `[Needs Verification]` pending the Phase 1.2a coverage pass.

## Concepts

- **co-01 · hire-to-retire-flow** — the H2R process chain: hire, onboard, compensate, develop,
  offboard.
- **co-02 · position-master-data** — a defined role in the org structure, distinct from the person who
  currently occupies it.
- **co-03 · organizational-unit** — a department or team in the org hierarchy, the structure positions
  attach to.
- **co-04 · employee-master-data** — the person, distinct from the position they occupy (a position
  can be vacant).
- **co-05 · time-and-attendance-data-flow** — hours worked or attendance recorded, feeding payroll as
  an input.
- **co-06 · payroll-as-subledger** — payroll processing produces a GL posting, following the same
  subledger-to-GL pattern as course 6, applied to labor cost.
- **co-07 · position-vs-employee-separation** — why separating the position from its occupant matters
  (e.g. for headcount planning and reorganizations).
- **co-08 · retroactive-payroll-adjustment** — a change effective in a period already paid (a backdated
  raise, a corrected timesheet) is recomputed for that period but posts the difference in the current
  open one, because a closed period (course 7) is not reopened to absorb it — the same posting-date
  versus effective-date distinction the finance modules carry, applied to labor cost.

## Worked examples

Prose-based worked scenarios (no runnable code). Every example cites the `co-NN` it exercises.

### Beginner

- **ex-01 · h2r-flow-trace** — given a new hire's first month, trace which H2R stage each event
  belongs to. (co-01)
- **ex-02 · position-vs-employee-contrast** — given a vacant position later filled by a new employee,
  show how position and employee master data relate. (co-02, co-04, co-07)

### Intermediate

- **ex-03 · org-hierarchy-read** — given an organizational chart, identify each position's
  organizational unit. (co-03)
- **ex-04 · payroll-posting-trace** — given a period's time-and-attendance data, trace it through to a
  payroll GL posting, citing the subledger-to-GL pattern from course 6. (co-05, co-06)
- **ex-05 · retroactive-adjustment-trace** — given a raise backdated into a period that has already
  been paid and closed, compute the difference for that period and identify the period its posting
  actually lands in. (co-06, co-08)

## Synthesis exercise — intra-topic

> Analysis and design only — never build, implement, or deploy a system (`A6`).

- **Goal**: design the position/organizational-unit structure for a new department, and trace one
  employee's time-and-attendance data through to its payroll GL posting.
- **Concepts exercised**: [ ] position vs employee (co-02, co-04) [ ] org hierarchy (co-03) [ ]
  payroll as subledger (co-06).
- **Ordered steps**: 1) design the org unit and its positions; 2) assign an employee; 3) record
  time-and-attendance data; 4) trace it to a payroll posting.
- **Acceptance criteria**: position and employee are correctly separated; the payroll posting
  correctly follows the subledger-to-GL pattern.
- **Done bar**: a written design and trace, no code, no system built.

## Read more

> Populated during the Phase 1.2a `web-researcher` confirmation pass (`A12`) — coverage-check
> citations only.

## In which paths

- `skills/conventional-erp` — Stage B, course 24 of 27.
- `skills/sharia-erp` — Stage B, course 24 of 30.

---

← Back to the [syllabus index](../README.md)
