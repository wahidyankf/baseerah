# Sharia-Compliant ERP Design (Annotated-concept)

**Course ID**: `sharia-compliant-erp-design` · **Format**: Annotated-concept · **Language**: — (domain, no code).

**Short summary**: Jurisdictional pluggability, configurable chart of accounts, contract-aware flows

**Scope note**: opens Stage C — `sharia-erp`'s exclusive extension. The engineering lesson is
**jurisdictional pluggability**: the chart of accounts, recognition rules, and disclosure set are
configuration, not hardcoded constants, because no single Sharia-compliance standard is universal
[Repo-grounded — structural claim; cell-level jurisdictional detail is `[Unverified]`]. Requires
`islamic-contract-modeling-for-systems` and `sharia-accounting-and-aaoifi-standards`.
License-aware (DD-15).

## Why this exists · the big idea

- **The problem before the solution**: three structurally different jurisdictional models coexist
  (AAOIFI/Bahrain, PSAK Syariah/Indonesia, MFRS + BNM policy/Malaysia) — a system hardcoded to one
  model cannot serve a business operating across jurisdictions.
- **Keep-this-if-you-forget-everything**: pluggability is the design answer to jurisdictional
  plurality — the chart of accounts, recognition rules, and disclosure set must be configuration.
- **Big ideas touched**: `jurisdictional-pluggability-as-the-design-principle`; `contract-type-aware-document-flow`
  — Murabaha/Ijarah/Musharaka each imply a different document-flow variant (deep dive: course 28).

## Prerequisites

- **ERP prereqs**: [`multi-company-and-multi-currency-erp`](./multi-company-and-multi-currency-erp.md).
- **Accounting prereqs**: `islamic-contract-modeling-for-systems`, `sharia-accounting-and-aaoifi-standards`
  (from `ayokoding-learning-path-06-skills-accounting`).
- **Assumed knowledge**: course 24's multi-entity structure vocabulary (a jurisdictional model is
  configured per legal entity, analogous to a fiscal-year variant).

## Accuracy notes (pending A4 verification)

> The Phase 1.2a confirmation pass has not yet run for this course.

- **The whole jurisdictional-model table is `[Unverified]`** pending the primary-source
  re-verification pass named in `tech-docs.md` — AAOIFI FAS numbers, PSAK Syariah series, and MASB/BNM
  positioning are all carried forward from the grounding file's own status, not restated as verified
  fact here.
- The structural claim (three coexisting models, none universal) is independent of the cell-level
  detail and does not itself require re-verification, per `tech-docs.md` DD-12.
- Indonesian PSAK numbering is `[Needs Verification]` — sources show both a "PSAK 59 / SIFAS 101-109"
  generation and a "PSAK 101-110" series; not asserted as a specific number here.

## Concepts

- **co-01 · jurisdictional-plurality** — AAOIFI/Bahrain, PSAK Syariah/Indonesia, and MFRS + BNM
  policy/Malaysia coexist as structurally different models, none universal.
- **co-02 · jurisdictional-pluggability** — the chart of accounts, recognition rules, and disclosure
  set are modeled as configuration, selected per legal entity, not hardcoded.
- **co-03 · configurable-chart-of-accounts** — a chart of accounts structure that can vary by
  jurisdiction without a schema change, applying course 21's extensibility-axis reasoning.
- **co-04 · configurable-disclosure-set** — which disclosures a financial statement must carry, varied
  by jurisdictional configuration.
- **co-05 · contract-type-awareness** — a document flow that varies its shape based on the underlying
  contract type (Murabaha, Ijarah, Musharaka), previewed here.
- **co-06 · profit-sharing-hook** — a configuration point for profit-sharing calculation logic
  (Mudaraba/Musharaka), detailed in course 29.
- **co-07 · zakat-hook** — a configuration point for zakat calculation, detailed in course 29.

## Worked examples

Prose-based worked scenarios (no runnable code). Every example cites the `co-NN` it exercises.

### Beginner

- **ex-01 · jurisdictional-model-classify** — given three sample transactions each following a
  different jurisdictional model's disclosure requirement, classify each by model. (co-01)
- **ex-02 · configurable-coa-sketch** — sketch how a single chart-of-accounts structure could vary its
  labeling by jurisdiction without a schema change. (co-03)

### Intermediate

- **ex-03 · contract-type-flow-preview** — given a Murabaha transaction and a conventional sale, note
  where the document flow would need to diverge (full treatment: course 28). (co-05)
- **ex-04 · disclosure-configuration-trace** — given two jurisdictions with different disclosure
  requirements for the same transaction type, show how the configuration would differ. (co-04)

## Synthesis exercise — intra-topic

> Analysis and design only — never build, implement, or deploy a system (`A6`).

- **Goal**: design the jurisdictional-configuration scheme (chart of accounts, disclosure set) for a
  business operating in two of the three named jurisdictional models.
- **Concepts exercised**: [ ] jurisdictional plurality (co-01) [ ] pluggability (co-02) [ ]
  configurable chart of accounts and disclosure (co-03, co-04).
- **Ordered steps**: 1) name the two jurisdictional models; 2) design the configurable chart-of-
  accounts structure; 3) design the configurable disclosure set.
- **Acceptance criteria**: the design does not hardcode either jurisdiction's specifics into the base
  schema; both models are genuinely representable through configuration.
- **Done bar**: a written design, no code, no system built.

## Read more

> Populated during the Phase 1.2a `web-researcher` confirmation pass (`A12`) — coverage-check
> citations only.

## In which paths

- `skills/sharia-erp` only — Stage C, course 27 of 29.

---

← Back to the [syllabus index](../README.md)
