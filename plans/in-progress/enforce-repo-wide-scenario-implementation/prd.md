# Product Requirements — Enforce Repo-Wide Gherkin Scenario Implementation

## Product Overview

Generalize rhino-cli's proven enforcement pattern to every eligible app and lib: level-tag + `@covers`
every scenario, make every test tier fail-on-skip, and upgrade `rhino-cli specs behavior-coverage` to a
runtime cross-check so "covered" requires a real test that executed and passed. CLI/tooling + config +
test-wiring plan — **no web UI, no HTTP API surface is added**, so the UI-design-funnel and Rule-15/16
live-tester retests do not apply (individual per-project rollout batches that touch a UI app's tests do
not change its UI).

## Personas

- **Petra (Platform maintainer, solo)** — wants a green `specs:coverage` to mean "all specified behaviour
  was verified", repo-wide.
- **Aria (AI coding agent)** — wants feature-change-completeness enforced mechanically so it can't ship a
  scenario without a real passing test.

## User Stories

- **US-1** — As Petra, I want every test tier to fail when a test is skipped/only/todo, so no behaviour
  silently goes unverified.
- **US-2** — As Petra, I want `behavior-coverage` to fail unless each `@covers` scenario actually ran and
  passed, so a marker alone can't satisfy the gate.
- **US-3** — As Petra, I want every eligible app/lib to carry level tags + `@covers` markers, so coverage
  is enforced everywhere, not just rhino-cli.
- **US-4** — As Aria, I want the enforcement identical in mechanism across the three repos (engine
  byte-identical), so cross-repo work is deterministic.

## Acceptance Criteria (Gherkin)

### AC-1 — Every tier fails on skip (US-1)

```gherkin
Scenario: A skipped or todo test reddens the tier it lives in
  Given a project configured with per-tier fail-on-skip for unit, integration, and e2e
  When a test is marked skip/only/todo or a cucumber step is undefined
  Then that tier's run exits non-zero and names the offending test
  And CI does not report success while any scenario is unexecuted
```

### AC-2 — behavior-coverage requires a real passing test (US-2)

```gherkin
Scenario: A marked-but-unexecuted scenario fails the central gate
  Given a scenario with a valid @covers marker whose covering test is skipped at runtime
  When rhino-cli specs behavior-coverage validate runs with the runtime cross-check
  Then the gate fails and names the scenario as marked-but-not-executed
  And the gate passes only when every @covers scenario executed and passed at each declared level
```

### AC-3 — @covers + level tags adopted repo-wide (US-3)

```gherkin
Scenario: Every eligible project carries level tags and @covers markers
  Given the coverage.projects registry in repo-config.yml
  When the rollout is complete
  Then every eligible app and lib has its scenarios level-tagged and @covers-marked
  And specs behavior-coverage validate passes non-vacuously for each of them
```

### AC-4 — Engine change is byte-identical across the three repos (US-4)

```gherkin
Scenario: The runtime cross-check engine is identical in all three repos
  Given the upgraded behavior-coverage command in apps/rhino-cli
  When apps/rhino-cli is compared across ose-public, ose-primer, ose-infra
  Then the engine source is byte-identical across the three repos
  And each repo's rhino-cli golden-master and suite pass
```

### AC-5 — The gate is wired into pre-push and CI (US-1, US-2)

```gherkin
Scenario: The runtime cross-check runs in the standard gates
  Given the upgraded specs:coverage target
  When a developer runs the pre-push gate or CI runs
  Then the runtime cross-check executes for every affected project
  And a marked-but-unexecuted scenario blocks the push and the merge
```

## Product Scope

**In:** level-tag + `@covers` rollout across eligible apps/libs; per-tier fail-on-skip (cucumber/Jest/
Vitest/Playwright/F#); `behavior-coverage` runtime cross-check; pre-push + CI wiring; engine propagated
byte-identical to the three repos.

**Out:** authoring new behaviour; app/validator logic changes; UI changes; rhino-cli's own de-hollow
(dependency plan).

## Product Risks

| Risk                                                               | Mitigation                                                                                                                                                                                                                                                                              |
| ------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| A tool has no clean machine-readable reporter for the cross-check. | Prefer JSON reporters; fall back to that tool's fail-on-skip alone for that tier, documented.                                                                                                                                                                                           |
| Large unimplemented-scenario backlog stalls the rollout.           | Batch per-project per-phase and work it down. **No defer, no shortcut** (Decision 4): each batch fully implements its scenarios before its gate — no `@wip`, no skip, no partial. A scenario that can't pass has its behaviour built or is removed as an invalid spec (with rationale). |
