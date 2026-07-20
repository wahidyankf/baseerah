# Mermaid State-Diagram Label Render-Clipping WARN Rule

> **Status**: Backlog — filed by the Knowledge Capture phase of
> [`parallel-orchestration-shared-machine-governance`](../../done/) (merged as `60d53119b`).
>
> **Delivery Mode**: `worktree-to-pr` (repo default)
>
> **Boundary note**: this touches `apps/rhino-cli/**`, which is required to be **byte-identical**
> across `ose-public`, `ose-primer`, and `ose-infra` per the
> [SDLC Gate Standard](../../../docs/reference/sdlc-gate-standard.md#rhino-cli-byte-identity-boundary).
> Execution is a coordinated three-repo change plus companion Gherkin under
> `specs/apps/rhino/behavior/rhino-cli/gherkin/**`.

`stateDiagram-v2` edge labels can clip in GitHub's renderer. The diagram is syntactically valid,
passes every text-based validator, and is silently wrong as displayed. This plan adds a
**WARN-level** heuristic to `rhino-cli md mermaid validate` — with a threshold derived
empirically, not assumed.

## Context

A diagram can be **source-correct and render-wrong**. No text-based validator can observe this,
because the defect exists only in the rendered output. The existing `md mermaid validate` label
rule (≤ 30 raw characters) is a proxy that neither catches nor explains the real failure.

**Character count does not predict clipping.** Observed during the originating plan:

| Label length | Rendered result |
| ------------ | --------------- |
| 30 chars     | **clipped**     |
| 33 chars     | **clipped**     |
| 40 chars     | rendered fine   |

Clipping depends on glyph widths and diagram layout, not raw length. Any threshold asserted
without measuring rendered output is fiction.

## Why WARN, Never FAIL

Measured blast radius across this repo's `stateDiagram` edge labels:

| Label length band | Count   |
| ----------------- | ------- |
| > 40 chars        | 31      |
| 31–40 chars       | 202     |
| 26–30 chars       | 983     |
| ≤ 25 chars        | ~11,800 |

A failing gate on a heuristic threshold would block roughly 1,200 existing labels on a defect the
gate cannot actually detect. The rule must classify as **⚠️ Warn** (message emitted, exit 0),
never **❌ Fail**.

## Scope

**In scope**:

- Derive the threshold **empirically**: render a calibration set of `stateDiagram-v2` labels
  across a length sweep, observe which clip in GitHub's renderer, and characterize the predictor
  (likely rendered glyph width, not character count).
- Add the rule to `rhino-cli md mermaid validate` at WARN severity only.
- Record the derivation method and the calibration data in the plan's `tech-docs.md`, so a future
  maintainer can re-derive the threshold when the renderer changes.
- Update
  [`diagrams.md` §Render-Fidelity Caveat](../../../repo-governance/conventions/formatting/diagrams.md)
  to point at the shipped rule.

**Out of scope**:

- Any FAIL-level enforcement.
- Bulk-rewriting the ~1,200 existing labels in the warn bands. Warnings surface them; remediation
  is opportunistic.
- Flowchart and sequence-diagram labels — the calibration is state-diagram-specific until
  measured otherwise.

## Acceptance Criteria

```gherkin
Feature: State-diagram labels at render-clipping risk are warned about

  Scenario: A label above the empirically derived threshold
    Given a markdown file containing a stateDiagram-v2 transition label above the threshold
    When rhino-cli md mermaid validate runs against that file
    Then a warning naming the file, line, and label is emitted
    And the command exits 0

  Scenario: A label below the threshold produces no warning
    Given a markdown file whose stateDiagram-v2 labels are all below the threshold
    When rhino-cli md mermaid validate runs against that file
    Then no render-clipping warning is emitted
    And the command exits 0

  Scenario: The rule never fails the build
    Given a markdown file containing a stateDiagram-v2 label far above the threshold
    When rhino-cli md mermaid validate runs against that file
    Then the command exits 0

  Scenario: The threshold is documented as empirically derived
    Given the plan's tech-docs.md
    When a maintainer reads the threshold rationale
    Then it cites the calibration set and the observed clipping results
    And it does not assert a raw character count as the cause
```

The second scenario is the falsifiability control in the negative direction — a rule that warned
unconditionally would pass the first and third scenarios and fail this one.
