# ERP Foundations and History (Annotated-concept)

**Course ID**: `erp-foundations-and-history` · **Format**: Annotated-concept · **Language**: — (domain, no code).

**Short summary**: Why integrated ERP emerged, what it actually does, this corpus's own scope

**Scope note**: orients the reader before any architecture detail lands — the shift from siloed
departmental software to one integrated system of record, the module map and cross-cutting spine
previewed at a glance, and an explicit statement of what this corpus teaches (architecture and domain
reasoning to build-founding depth, `A6`) and does not (installing, operating, evaluating, or selecting
a system, `A7`). License-aware (DD-15).

## Why this exists · the big idea

- **The problem before the solution**: departmental software kept its own ledger, its own inventory
  count, its own customer list; reconciling them by hand was slow and error-prone. An ERP's core
  promise is one system of record that removes the reconciliation step.
- **Keep-this-if-you-forget-everything**: "single source of truth" is a property the architecture has
  to earn — through the cross-cutting spine (courses 4-9) — not a fact delivered by installing
  software.
- **Big ideas touched**: `integration-as-architecture` — courses 4-9 show the actual mechanisms that
  make integration real, not just a marketing claim; `read-reason-design-not-operate` — this corpus's
  honest scope boundary (DD-29), stated here first.

## Prerequisites

- **Prior topics**: none — this is the corpus's entry point.
- **Cross-domain prerequisites**: none.
- **Assumed knowledge**: none.

## Accuracy notes (pending A4 verification)

> The Phase 1.2a `web-researcher` confirmation pass (see
> [tech-docs.md §Verification status](../../tech-docs.md#verification-status-carried-forward-a4)) has
> not yet run for this course. Claims below are domain reasoning grounded in the corpus's own research
> file and must be re-verified, not restated as fact, before this course is finalized.

- Module framing throughout is conceptual; no claim depends on a specific vendor figure or market
  statistic.
- The "single source of truth as an earned property" framing is `[Needs Verification]` — domain
  reasoning, not directly sourced from the grounding research.

## Concepts

<!-- co-NN · concept enumeration. Floor >= 8 (DD-35, Annotated-concept). Each example below cites the co-NN it exercises. -->

- **co-01 · integration-promise** — one system of record across finance, logistics, and people,
  replacing siloed departmental software.
- **co-02 · reconciliation-elimination** — the specific pain integration removes: manual cross-checking
  between disconnected ledgers and records.
- **co-03 · transaction-processing-scope** — an ERP processes transactions across procurement, sales,
  inventory, finance, and people — not just one department's records.
- **co-04 · subledger-to-gl-preview** — a first-pass mention of the subledger-to-GL relationship
  (deep dive: course 6) as the architectural crux underlying "integration".
- **co-05 · real-time-vs-batch-preview** — real-time vs batch processing as a design axis that recurs
  throughout the corpus (deep dive: course 6).
- **co-06 · build-founding-depth** — this corpus teaches domain knowledge and architecture deep enough
  to found an implementation; it never asks the reader to build one (`A6`).
- **co-07 · no-buyer-content** — this corpus contains no evaluation, selection, or
  implementation-methodology material (`A7`).
- **co-08 · two-path-relationship** — `skills/conventional-erp` and `skills/sharia-erp` share the same
  27-course foundation; `sharia-erp` is not an add-on assuming the conventional path (`A10`/`A11`).
- **co-09 · single-source-of-truth-as-earned** — integration is a property the cross-cutting spine
  delivers, not an automatic consequence of installing software.
- **co-10 · module-map-preview** — the module families (finance, logistics, production, people) named
  here at a glance, detailed in course 3.

## Worked examples

Prose-based worked scenarios (no runnable code — this is a domain, not a language, course). Every
example cites the `co-NN` concept(s) it exercises.

### Beginner

- **ex-01 · siloed-vs-integrated-contrast** — a short scenario contrasting a manual month-end
  reconciliation across three disconnected spreadsheets with the same close in an integrated system —
  identify where the manual version can silently diverge. (co-01, co-02)
- **ex-02 · transaction-scope-map** — given a list of ten business events (hire an employee, receive a
  shipment, pay a vendor, and so on), classify which ERP module family each belongs to. (co-03, co-10)
- **ex-03 · scope-boundary-self-check** — given five candidate course topics, mark which belong in this
  corpus (architecture, domain reasoning) and which do not (installation steps, vendor comparison
  criteria) — verify against `A6`/`A7`. (co-06, co-07)

### Intermediate

- **ex-04 · single-source-of-truth-failure-mode** — a scenario where "integration" is claimed but the
  cross-cutting spine (course 4-9 concerns) is missing — identify what actually breaks. (co-09)
- **ex-05 · two-path-scoping-check** — given a hypothetical reader who wants only Sharia-compliant ERP
  knowledge, explain why they still need the shared 27-course foundation. (co-08)

## Synthesis exercise — intra-topic

> Analysis and design only — never build, implement, or deploy a system (`A6`).

- **Goal**: given a short business narrative (a mid-size distributor moving off spreadsheets), identify
  which ERP module families and cross-cutting concerns are implicated, and state — in writing — which
  of this corpus's scope boundaries (A6/A7) apply.
- **Concepts exercised**: [ ] integration promise (co-01) [ ] transaction scope (co-03) [ ] build/buy
  scope boundary (co-06, co-07).
- **Ordered steps**: 1) read the narrative; 2) list every module family it touches; 3) name the
  cross-cutting concern most at risk of a silent failure; 4) state explicitly what this corpus would
  and would not teach the reader to do about it.
- **Acceptance criteria**: the module list is complete against the narrative; the named cross-cutting
  concern is a real course-4-9 topic, not invented; the scope statement correctly excludes
  installation/evaluation content.
- **Done bar**: a written analysis, no code, no system stood up.

## Read more

> Populated during the Phase 1.2a `web-researcher` confirmation pass (`A12`) — coverage-check
> citations only, never a structure source. Left unpopulated here to avoid asserting an unverified
> citation.

## In which paths

- `skills/conventional-erp` — Stage A, course 1 of 27.
- `skills/sharia-erp` — Stage A, course 1 of 30.

---

← Back to the [syllabus index](../README.md)
