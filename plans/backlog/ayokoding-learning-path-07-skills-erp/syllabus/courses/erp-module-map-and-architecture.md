# ERP Module Map and Architecture (Annotated-concept)

**Course ID**: `erp-module-map-and-architecture` · **Format**: Annotated-concept · **Language**: — (domain, no code).

**Short summary**: The module map, cross-module process flows, the open-source landscape

**Scope note**: names the module families (finance, controlling, materials management, sales,
production, quality management, human capital) and the process flows that cross them, and introduces
the open-source landscape nominatively so later worked examples can reference "how ERPNext models X"
without re-explaining what ERPNext is each time. License-aware (DD-15).

## Why this exists · the big idea

- **The problem before the solution**: a module-by-module treatment alone produces a reader who can
  name parts but not explain how a transaction crosses them — this course fixes the map before any
  process course goes deep on one flow.
- **Keep-this-if-you-forget-everything**: naming the _flow_ (procure-to-pay, order-to-cash) is a more
  useful unit of architectural reasoning than naming the _module_ it happens to touch.
- **Big ideas touched**: `cross-module-process-flows`; `nominative-open-source-reference` — the
  licence-aware posture that lets this corpus cite real systems without reproducing their content
  (see [tech-docs.md §Licensing and IP Compliance](../../tech-docs.md#licensing-and-ip-compliance-a8)).

## Prerequisites

- **Prior topics**: [`erp-conceptual-data-model`](./erp-conceptual-data-model.md).
- **Cross-domain prerequisites**: none.
- **Assumed knowledge**: course 2's master-data and header/line vocabulary.

## Accuracy notes (pending A4 verification)

> The Phase 1.2a confirmation pass has not yet run for this course.

- Licence claims for the open-source landscape are `[Repo-grounded]` against the domain-research
  grounding and `tech-docs.md`'s licensing table. Metasfresh's licence is **GPLv2**
  `[Web-cited: metasfresh/metasfresh LICENSE.md — https://raw.githubusercontent.com/metasfresh/metasfresh/master/LICENSE.md ; accessed 2026-07-22]`;
  its commercial offering is paid support on the same GPL code, **not** a separate proprietary
  edition. The "GPLv2/GPLv3" characterization is grounded in the LICENSE.md's own "version 2 of the
  License, or (at your option) any later version" clause (GPL-2.0-or-later, so GPLv3 use is permitted)
  `[Web-cited: metasfresh/metasfresh LICENSE.md — https://raw.githubusercontent.com/metasfresh/metasfresh/master/LICENSE.md ; accessed 2026-07-22]`;
  Wikipedia's infobox likewise records the licence as "GPLv2/GPLv3"
  `[Web-cited: Wikipedia — Metasfresh — https://en.wikipedia.org/wiki/Metasfresh ; accessed 2026-07-22]`.
  Described behaviourally only, never with copied code.
- Concept co-13 is placed on domain-reasoning grounds rather than sourced from the grounding research,
  and remains `[Needs Verification]` pending the Phase 1.2a coverage pass **as a module-family naming
  claim**. Its prior open question — whether covering quality management at its touchpoints is
  sufficient, or whether the field would expect it as a module family in its own right — is **settled
  in favour of the latter** (DD-36): QM now carries its own course at reading position 21, and co-13
  is the map-level pointer to it rather than the corpus's whole treatment of it.

## Concepts

- **co-01 · finance-controlling-fi-co** — the accounting-facing module family (deep dive: courses 5-9,
  13).
- **co-02 · materials-management-mm** — the procurement/inventory-facing module family (deep dive:
  courses 10, 12, 14-16).
- **co-03 · sales-distribution-sd** — the sales-facing module family (deep dive: course 11).
- **co-04 · production-planning-pp** — the manufacturing-facing module family (deep dive: courses 17-20).
- **co-05 · human-capital-management-hcm** — the people-facing module family (deep dive: course 24).
- **co-06 · generic-role-names** — FI/CO/MM/SD/PP/HCM are generic industry role names, not any single
  vendor's proprietary terms.
- **co-07 · procure-to-pay-flow** — the P2P cross-module flow at a glance (deep dive: course 10).
- **co-08 · order-to-cash-flow** — the O2C cross-module flow at a glance (deep dive: course 11).
- **co-09 · record-to-report-flow** — the R2R cross-module flow at a glance (deep dive: course 13).
- **co-10 · hire-to-retire-flow** — the H2R cross-module flow at a glance (deep dive: course 24).
- **co-11 · open-source-landscape-nominative** — Odoo, ERPNext, Tryton, Apache OFBiz, Dolibarr,
  iDempiere named as reference points, licence noted per project, never in a course title or path
  segment.
- **co-12 · enterprise-it-boundary** — what sits outside the ERP boundary (CRM, standalone WMS, BI/data
  warehouse, e-commerce front end) at a glance (deep dive: course 23).
- **co-13 · quality-management-qm** — the inspection-and-release module family, named here because it
  sits _inside_ the boundary co-12 draws rather than outside it like CRM, standalone WMS and BI: a
  goods receipt can be routed to an inspection hold instead of straight into unrestricted stock, and a
  usage decision then releases or rejects it. Its mechanics land on the stock-type model (course 14)
  and its rejections on the procurement exception flow (course 12) — deep dive: course 21.

## Worked examples

Prose-based worked scenarios (no runnable code). Every example cites the `co-NN` it exercises.

### Beginner

- **ex-01 · module-family-classification** — given ten business events, classify each into its module
  family. (co-01–co-05)
- **ex-02 · flow-vs-module-contrast** — trace one order through P2P, noting which module each step
  belongs to versus the flow name that actually explains the sequence. (co-07)

### Intermediate

- **ex-03 · nominative-reference-drafting** — write one sentence describing "how ERPNext models a stock
  ledger entry" that stays nominative (no reproduction of ERPNext's own docs). (co-11)
- **ex-04 · it-boundary-scoping** — given a system landscape diagram, identify which boxes are inside
  the ERP boundary and which are not. (co-12)
- **ex-05 · qm-boundary-placement** — given a goods receipt that must pass inspection before it can be
  sold, place the inspection step inside the ERP boundary and explain what distinguishes it from the
  CRM, standalone WMS and BI systems co-12 places outside. (co-02, co-12, co-13)

## Synthesis exercise — intra-topic

> Analysis and design only — never build, implement, or deploy a system (`A6`).

- **Goal**: given a business scenario spanning three module families, diagram (in prose or an
  originally-drawn Mermaid diagram, never a vendor screenshot) the cross-module flow it represents.
- **Concepts exercised**: [ ] module families (co-01–co-05) [ ] a named flow (co-07–co-10).
- **Ordered steps**: 1) identify the module families touched; 2) name the flow; 3) sequence the steps
  across modules.
- **Acceptance criteria**: the flow name matches a real cross-module flow (P2P/O2C/R2R/H2R); the
  sequence is internally consistent.
- **Done bar**: a written/diagrammed analysis, no code, no system stood up.

## Read more

> Populated during the Phase 1.2a `web-researcher` confirmation pass (`A12`) — coverage-check
> citations only.

## In which paths

- `skills/conventional-erp` — Stage A, course 3 of 27.
- `skills/sharia-erp` — Stage A, course 3 of 30.

---

← Back to the [syllabus index](../README.md)
