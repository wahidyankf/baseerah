<!-- Knowledge Capture running log — append entries during execution. -->
<!-- Triage every entry (or record the explicit "none" escape) before archival. -->

# Learnings: e2e-coverage-rule-feature-skip-fixme-gap

## Triage (terminal): No generalizable learning surfaced

No durable, generalizable learning surfaced during execution. The change was a bounded extension
of the existing `e2e-coverage` gap detector — lifting `@skip`/`@fixme` special-tag detection from
`Scenario Outline` level to also cover `Rule:`/`Feature:`-level tags one AST level up — delivered
under the established TDD + 3-repo byte-identity workflow with no new cross-cutting insight. Merged
via PR #76 (`e21f7a212`, 2026-07-19). Per the
[Knowledge Capture Convention](../../../repo-governance/development/quality/knowledge-capture.md),
this explicit "none" record is the terminal KC state for this plan.
