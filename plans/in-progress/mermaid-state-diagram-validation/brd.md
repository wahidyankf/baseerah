# Business Requirements — Mermaid State Diagram Validation (ose-public)

## Business Goal

Extend the existing automated Mermaid width/label discipline to cover state diagrams, and unify
the rhino-cli validator across all three sibling repos so the rule set cannot silently diverge
again.

## Business Rationale

The repo invests in `validate-mermaid` to keep diagrams readable on mobile viewports and in narrow
documentation columns. That investment is undermined when a whole diagram family — state diagrams —
escapes the check entirely. A `stateDiagram-v2 direction LR` chain of 11 states sails through the
gate today while an equivalent flowchart would be blocked. [Repo-grounded:
`apps/rhino-cli/src/internal/mermaid.rs:342-356` returns count `0` for non-flowchart headers]

Three repos (ose-public, ose-primer, ose-infra) each carry an independently evolved copy of the
validator. Patching state support into three diverged codebases independently would deepen the
drift. Unifying onto one fresh design first, then adding state support once, removes the drift risk
at its root.

## Business Impact

Pain points addressed:

- State diagrams render too wide on mobile and in PDF exports without any automated guard.
- Three diverged validator copies make every future Mermaid-rule change a three-way manual
  reconciliation — error-prone and easy to forget. [Repo-grounded: brief survey table — public
  monolith 1757 lines, infra monolith 1583 lines version-behind, primer modular split]

Expected benefits:

- State diagrams obey the same render-width discipline as flowcharts. _Judgment call: the rule is
  identical to the flowchart rule already trusted, so the readability benefit transfers directly._
- A single shared module design plus a machine-checked golden corpus make future parity automatic
  rather than manual.

## Affected Roles

This is a solo-maintainer repo; the maintainer wears several hats here:

- **Tooling maintainer** — owns the rhino-cli validator refactor.
- **Documentation author** — benefits from state diagrams being held to the same standard as
  flowcharts.
- **Governance maintainer** — propagates the new rule into `repo-governance` and re-syncs bindings.

## Business-Level Success Metrics

- Every state diagram that exceeds 4 nodes on a rank or 30 characters in a label is flagged by
  `validate:mermaid` (observable: run the target; over-wide fixtures report `width_exceeded`).
- Zero `width_exceeded` / `label_too_long` state-diagram violations remain repo-wide after cleanup
  (observable: full-repo scan is clean).
- The golden corpus produces byte-identical violation output across all three repos (observable:
  same fixtures, same expected JSON committed in each repo).
- Flowchart behavior is unchanged (observable: every pre-existing flowchart unit test stays green).

## Business-Scope Non-Goals

- No validation of other diagram families (sequence, class, ER, gitGraph).
- No change to where or when the gate runs (pre-commit staged scan + CI, unchanged).
- No relaxation of the existing width/label thresholds.

## Business Risks and Mitigations

- **Risk**: The unify refactor silently changes flowchart behavior. **Mitigation**: Phase A is a
  pure behavior-preserving refactor gated on all existing flowchart tests staying green before any
  state code is added.
- **Risk**: The three repos drift again. **Mitigation**: A shared golden corpus is the hard,
  machine-checked parity lock; the identical module design is the structural lock.
- **Risk**: Repo-wide cleanup touches `plans/done/` history. **Mitigation**: This is an explicit,
  recorded D-CLEAN choice for maximum hygiene; edits are diagram-only and reviewed in the cleanup
  phase.
