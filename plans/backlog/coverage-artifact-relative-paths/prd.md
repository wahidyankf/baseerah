# Product Requirements: Coverage Artifact Relative Paths

## Persona

An engineer or AI agent running `test:quick`/`test:coverage` on a lib with a git-tracked .NET
coverage artifact, from any checkout path — needs the coverage run to produce zero unrelated diff
regardless of that path.

## User Story

As an engineer or AI agent running the coverage suite from any checkout path, I want the tracked
coverage artifact to stay path-independent, so that running tests never dirties the tree with a
diff unrelated to any actual code change.

## Product Scope

**Candidate scope, pending the Phase 1 investigation** (see `tech-docs.md` and `delivery.md`'s
Phase 1): `libs/fsharp-crane-core/tests/unit/coverage.json` at minimum; expand to any other
`libs/*`/`apps/*` project found to track the same class of artifact.

## Acceptance Criteria (Gherkin)

```gherkin
Feature: Path-independent coverage artifacts

  Scenario: Running the coverage suite from a different checkout path
    Given a lib with a git-tracked coverage artifact previously generated from checkout path A
    When the coverage suite runs from a different checkout path B
    Then "git status" reports no diff in the coverage artifact

  Scenario: Coverage artifact is gitignored instead
    Given a lib whose coverage artifact is gitignored per the Phase 1 decision
    When the coverage suite runs from any checkout path
    Then the regenerated artifact never appears in "git status"
```

## Non-Goals

- Does not change `fsharp-crane-core`'s coverage threshold or test content.
- Does not retroactively rewrite existing coverage history.
