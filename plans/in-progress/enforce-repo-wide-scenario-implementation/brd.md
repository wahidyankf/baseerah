# Business Requirements — Enforce Repo-Wide Gherkin Scenario Implementation

## Business Goal

Make every Gherkin scenario in the repo a **guaranteed-executed, guaranteed-passing** test at each level
it claims — across all apps and libs, for unit/integration/e2e — so that a green quality gate is honest:
it means the specified behaviour was actually verified, not merely documented and marked.

## Why Now

The repo's specs are Diátaxis-grade and its `behavior-coverage` mechanism is well-designed, but it
enforces **traceability** (a `@covers` marker exists) rather than **execution** (the test ran and
passed). The rhino-cli audit proved the cost: **121 of 228 scenarios (53%) silently skipped** while every
gate was green. The same class of hole exists at every tier repo-wide (Jest/Vitest `.skip`/`.todo`,
Playwright `test.skip`, F# ignored tests) and `@covers` adoption is **rhino-cli-only (0 markers elsewhere)**.
Once the rhino-cli plan proves the pattern, generalizing it is the highest-leverage integrity work
available: it converts the whole spec tree from aspirational to enforced.

## Impact & Affected Roles

| Role                   | Impact                                                                                      |
| ---------------------- | ------------------------------------------------------------------------------------------- |
| Repo maintainer (solo) | A green `specs:coverage` becomes trustworthy repo-wide; unimplemented behaviour can't hide. |
| AI coding agents       | Feature-change-completeness (specs+tests) is mechanically enforced, not convention-only.    |
| CI / quality gates     | Skipped/absent tests fail; "covered" requires a real passing test at each declared tier.    |

## Business Success Metrics

- **Zero silently-skipped scenarios repo-wide** — every tier fails on skip/only/todo (observable: a
  planted `.skip`/undefined step reddens CI in each tool). [Judgment call: derived from the user directive
  "all behaviour should be implemented"]
- **`@covers` adoption = 100% of eligible projects** — every project in `coverage.projects` has its
  scenarios level-tagged and `@covers`-marked (observable: `specs behavior-coverage validate` passes
  non-vacuously; audit shows markers in every eligible app/lib, not just rhino-cli).
- **Central runtime cross-check live** — `behavior-coverage` fails unless each `@covers` scenario actually
  executed and passed (observable: a scenario whose test is skipped fails the gate even though its marker
  exists).

## Business-Scope Non-Goals

- Not authoring new behaviour; not changing app/validator logic (except writing the real test a scenario
  demands).
- Not unifying app sets across repos (each repo enforces its own apps/libs; only the rhino-cli engine is
  byte-identical).
- No time estimates.

## Business Risks

| Risk                                                                                           | Severity | Mitigation                                                                                                                                                                                                                                                                                                                     |
| ---------------------------------------------------------------------------------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Turning on fail-on-skip / the cross-check surfaces a large backlog of unimplemented scenarios. | High     | Expected. Phase 0 sizes it; the rollout is batched per-project per-phase. **No defer, no shortcut** (Decision 4): each batch fully implements its scenarios before its gate — no `@wip`, no skip. A scenario that can't pass has its behaviour built or is removed as an invalid spec. The backlog is worked down, not parked. |
| The central runtime cross-check must parse multiple tools' run outputs (brittle).              | Medium   | Prefer each tool's machine-readable reporter (JSON) over scraping; TDD the ingester per tier; rhino-cli's own suite is the first proving consumer.                                                                                                                                                                             |
| Engine change re-introduces rhino-cli source drift.                                            | Medium   | Single canonical edit in public, propagated verbatim; guarded by the dependency plan's byte-identity boundary + golden-master.                                                                                                                                                                                                 |
