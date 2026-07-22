# ERP Security and Controls (Annotated-concept)

**Course ID**: `erp-security-and-controls` · **Format**: Annotated-concept · **Language**: — (domain, no code).

**Short summary**: RBAC/authorization objects, segregation of duties, ERP-specific COSO/SOX mapping

**Scope note**: one of the two scope-boundary-risk courses in the catalog (DD-10) — stays scoped to
**ERP-specific** RBAC and segregation-of-duties mechanics, explicitly distinct from
`it-governance-grc`'s general GRC scope. Requires `audit-controls-and-compliance` from the accounting
corpus. License-aware (DD-15).

## Why this exists · the big idea

- **The problem before the solution**: a user who can both create a vendor and approve a payment to
  that vendor has the technical means to commit fraud undetected — segregation of duties exists to
  make that combination of access structurally impossible, not just discouraged by policy.
- **Keep-this-if-you-forget-everything**: an SoD conflict is a property of a _combination_ of access
  rights, not any single right in isolation — a role review must check combinations, not just
  individual permissions.
- **Big ideas touched**: `sod-as-a-combinatorial-property`; `erp-specific-vs-general-grc-scope` — this
  course's own boundary, stated explicitly.

## Prerequisites

- **ERP prereqs**: [`erp-module-map-and-architecture`](./erp-module-map-and-architecture.md).
- **Cross-domain prerequisites**: `security-essentials` (existing library).
- **Accounting prereqs**: `audit-controls-and-compliance` (from
  `ayokoding-learning-path-06-skills-accounting`).
- **Assumed knowledge**: `security-essentials`'s general authorization vocabulary; course 9's
  audit-trail-as-control preview.

## Accuracy notes (pending A4 verification)

> The Phase 1.2a confirmation pass has not yet run for this course.

- The accounting-side course id `audit-controls-and-compliance` is as named in
  `ayokoding-learning-path-06-skills-accounting`'s own in-flight rewrite as of 2026-07-22.
- COSO/SOX control-point terminology is treated at concept depth. The COSO _Internal Control—Integrated
  Framework_ and Sarbanes-Oxley Section 404 (management assessment of internal control over financial
  reporting) are cited nominatively only — no framework text, component list, or principle numbering is
  reproduced here (A8)
  `[Web-cited: COSO — Internal Control—Integrated Framework — https://www.coso.org/guidance-on-ic ; accessed 2026-07-22]`.
  Any claim about what a specific COSO component or principle prescribes stays `[Needs Verification]`
  and is not asserted.
- Concept co-08 is placed on domain-reasoning grounds rather than sourced from the grounding research,
  and is `[Needs Verification]` pending the Phase 1.2a coverage pass.

## Concepts

- **co-01 · authorization-object** — a fine-grained permission unit an ERP's RBAC scheme is built
  from, more granular than a coarse "role" label.
- **co-02 · role-composition** — a role is a bundle of authorization objects assigned to a user; the
  bundle, not any single object, determines what combinations of access a user holds.
- **co-03 · segregation-of-duties-sod** — a design rule preventing one user from holding two
  authorization objects whose combination enables fraud or error (e.g. create-vendor +
  approve-payment).
- **co-04 · sod-conflict-matrix** — a structured way to enumerate which authorization-object
  combinations constitute an SoD conflict.
- **co-05 · sod-detection-vs-prevention** — a conflict can be prevented at role-design time or
  detected after the fact via a review; both are real, distinct controls.
- **co-06 · coso-sox-control-point-erp-specific** — where COSO/SOX-style control points map onto
  specific ERP mechanisms (approval workflows, three-way match, period-close locks), scoped to how
  this maps within an ERP specifically.
- **co-07 · scope-boundary-vs-it-governance-grc** — this course's own boundary: ERP-specific
  authorization-object and SoD mechanics, explicitly distinct from `it-governance-grc`'s
  general-purpose GRC framework treatment.
- **co-08 · privileged-and-emergency-access** — a bounded temporary elevation that deliberately breaks
  an SoD rule so an incident can be resolved, paired with its compensating control: every action taken
  under the elevation is recorded (course 9's change-tracking) and reviewed afterwards, which is what
  distinguishes it from simply granting the conflicting combination permanently.

## Worked examples

Prose-based worked scenarios (no runnable code). Every example cites the `co-NN` it exercises.

### Beginner

- **ex-01 · authorization-object-identify** — given a list of user permissions, identify the
  authorization objects each represents. (co-01)
- **ex-02 · role-composition-read** — given a role definition, list the authorization objects it
  bundles. (co-02)

### Intermediate

- **ex-03 · sod-conflict-detect** — given a role holding both create-vendor and approve-payment
  authorization objects, flag the SoD conflict. (co-03, co-04)
- **ex-04 · sod-prevention-vs-detection-design** — given the ex-03 conflict, design both a
  role-design-time prevention and a post-hoc detection review. (co-05)

### Advanced

- **ex-05 · coso-control-mapping** — given three ERP mechanisms (approval workflow, three-way match,
  period-close lock), map each to the COSO/SOX-style control point it satisfies. (co-06)
- **ex-06 · scope-boundary-self-check** — given five candidate topics (SoD matrix design, general
  enterprise risk framework selection, ERP authorization-object modeling, vendor risk scoring,
  approval-workflow design), mark which belong in this course and which belong in
  `it-governance-grc` instead. (co-07)
- **ex-07 · emergency-access-review** — given an emergency elevation that granted both create-vendor
  and approve-payment to one user for a single day, design the after-the-fact review that determines
  whether the conflicting combination was actually exercised, and name the recorded evidence it reads.
  (co-03, co-05, co-08)

## Synthesis exercise — intra-topic

> Analysis and design only — never build, implement, or deploy a system (`A6`).

- **Goal**: design an authorization-object and role scheme for a procurement department, including an
  SoD conflict matrix and both a prevention and a detection control.
- **Concepts exercised**: [ ] authorization objects (co-01) [ ] role composition (co-02) [ ] SoD (co-03,
  co-04, co-05).
- **Ordered steps**: 1) list authorization objects; 2) compose roles; 3) build the SoD conflict
  matrix; 4) design one prevention and one detection control.
- **Acceptance criteria**: the SoD matrix correctly flags at least one realistic conflict; prevention
  and detection controls are distinct and both address the flagged conflict.
- **Done bar**: a written design, no code, no system built.

## Read more

> Populated during the Phase 1.2a `web-researcher` confirmation pass (`A12`) — coverage-check
> citations only.

## In which paths

- `skills/conventional-erp` — Stage B, course 26 of 27.
- `skills/sharia-erp` — Stage B, course 26 of 30.

---

← Back to the [syllabus index](../README.md)
