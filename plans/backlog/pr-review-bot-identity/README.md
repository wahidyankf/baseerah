# PR-Review Bot Identity (Restore `REQUEST_CHANGES`)

> **Status**: Backlog — filed by the Knowledge Capture phase of
> [`parallel-orchestration-shared-machine-governance`](../../done/) (merged as `60d53119b`).
>
> **Delivery Mode**: `worktree-to-pr` (repo default)

`pr-review-maker` structurally cannot post a `REQUEST_CHANGES` review, so every blocking review it
posts lands with GitHub review STATE `COMMENT`. Any consumer that gates on review STATE reads a
blocked PR as unblocked. This plan provisions a distinct posting identity to close the gap.

## Context

`gh` authenticates as the **PR author** under the repository's current identity posture, and
GitHub rejects `REQUEST_CHANGES` on one's own pull request. The consequence is not cosmetic:

- A CRITICAL, blocking finding posts as review STATE `COMMENT`.
- GitHub's own "changes requested" signal never fires.
- Branch-protection rules, dashboards, or any future automation keyed on review STATE will
  classify the PR as unblocked while a CRITICAL finding sits open on it.

The current mitigation is documentation-only: blocking status is carried in the finding's
severity label (`CRITICAL` / `HIGH`) in the comment body, and consumers are instructed to parse
severity from comment text rather than trust the STATE field. This is recorded in
[`pr-review-quality-gate.md` §GitHub Reviews API Mechanics](../../../repo-governance/workflows/pr/pr-review-quality-gate.md)
and in [`.claude/agents/pr-review-maker.md`](../../../.claude/agents/pr-review-maker.md).
A documented workaround is not the same as a fixed gate.

## Scope

**In scope**:

- Provision a dedicated GitHub App or CI-scoped bot identity with minimal write scope — create
  review, reply to review comment, resolve review thread; nothing broader.
- Wire `pr-review-maker` to authenticate as that identity so `REQUEST_CHANGES` becomes available
  for blocking findings.
- Reinstate `REQUEST_CHANGES` in the workflow and agent definitions, and remove the
  parse-severity-from-text workaround language once the STATE field is trustworthy.
- Decide whether the AI-attribution footer stays once a distinct identity makes authorship
  self-evident.

**Out of scope**: `pr-review-fixer`'s commit-push identity. That is governed by the
[Git Identity Guardrail](../../../repo-governance/development/workflow/git-identity-from-global-config.md)
and is a separate concern from `gh`/GitHub-API posting identity.

## Acceptance Criteria

```gherkin
Feature: Blocking PR reviews carry a blocking review STATE

  Scenario: A review containing a CRITICAL finding
    Given pr-review-maker authenticates as the dedicated bot identity
    And the review contains at least one CRITICAL severity finding
    When the review is submitted via the GitHub Reviews API
    Then the API call succeeds
    And the resulting review STATE is REQUEST_CHANGES

  Scenario: A review containing only MEDIUM and LOW findings
    Given pr-review-maker authenticates as the dedicated bot identity
    And the review contains no CRITICAL or HIGH findings
    When the review is submitted
    Then the resulting review STATE is COMMENT

  Scenario: The bot identity cannot exceed its review scope
    Given the dedicated bot identity's token
    When a push to a repository branch is attempted with that token
    Then the operation is rejected
```

The third scenario is the security control — an identity provisioned with broad repo-write would
satisfy the first two scenarios while quietly granting far more than review posting.

## Dependencies

Requires an organization-level GitHub App installation or a CI-scoped token, which is an
infrastructure action outside the code repositories. Confirm availability before scheduling.
