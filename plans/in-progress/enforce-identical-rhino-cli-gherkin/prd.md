# Product Requirements — Enforce Identical, Fully-Enforcing rhino-cli Gherkin

## Product Overview

Make the rhino-cli Gherkin behaviour tree (`specs/apps/rhino/behavior/rhino-cli/gherkin/`) a single
canonical, fully-executing, byte-identical specification across `ose-public`, `ose-primer`, and
`ose-infra`. Every leaf rhino-cli command owns at least one scenario that **actually runs and passes**;
no scenario is skipped-by-data; the tree is identical everywhere; and an anti-drift gate keeps it that
way.

This is a **CLI/tooling + specs** plan (no web UI, no HTTP API) — the UI-design-funnel and the
Rule-15/Rule-16 live-tester retests do **not** apply.

## Personas

- **Petra (Platform maintainer, solo)** — owns all three repos; needs one behaviour spec to reason
  about and confidence that a green gate means behaviour was truly checked.
- **Aria (AI coding agent)** — executes work across the three repos; needs identical commands, behaviour,
  and specs so cross-repo work is deterministic.
- **Devin (Downstream template user of `ose-primer`)** — clones the public template; needs it to ship the
  same enforced governance as the source repo.

## User Stories

- **US-1** — As Petra, I want every rhino-cli behaviour scenario to actually execute, so a green suite
  proves the behaviour was checked rather than skipped.
- **US-2** — As Aria, I want the Gherkin tree byte-identical across all three repos, so behaviour I rely
  on in one repo is guaranteed in the others.
- **US-3** — As Petra, I want every leaf rhino-cli command to own at least one enforcing scenario, so no
  command ships unspecified.
- **US-4** — As Devin, I want `ose-primer`'s rhino-cli behaviour spec to equal the source repo's, so the
  template's governance is trustworthy.
- **US-5** — As Petra, I want the Gherkin tree inside the rhino-cli identity gate, so it can never
  silently drift again.

## Acceptance Criteria (Gherkin)

### AC-1 — No behaviour is skipped-by-data (US-1)

```gherkin
Scenario: The rhino-cli cucumber suite executes every scenario in the canonical repo
  Given the rhino-cli cucumber suite in ose-public
  When a developer runs "cargo test --release -p rhino-cli"
  Then every cucumber binary reports zero skipped scenarios
  And the overall test run exits with code 0
```

### AC-2 — Every leaf command owns an executing scenario (US-3)

```gherkin
Scenario: Each leaf rhino-cli command has at least one enforcing scenario
  Given the full rhino-cli leaf-command census from Phase 0
  When each leaf command is mapped to the canonical .feature tree
  Then every leaf command resolves to at least one scenario that executed and passed
  And no leaf command is left with only skipped or absent coverage
```

### AC-3 — Previously-unbound feature dirs execute (US-1)

```gherkin
Scenario: The ddd, git, specs, and test-coverage feature dirs run under a cucumber binary
  Given the 4 feature dirs that were bound to no test binary before this plan
  When the rhino-cli cucumber suite runs
  Then scenarios under ddd, git, specs, and test-coverage are executed by a cucumber binary
  And none of them remain unexecuted spec-only files
```

### AC-4 — Gherkin tree is byte-identical across all three repos (US-2, US-4)

```gherkin
Scenario: The Gherkin behaviour tree is identical in all three repos
  Given the canonical Gherkin tree in ose-public after de-hollowing
  When the tree is compared against ose-primer and ose-infra
  Then all .feature files and behaviour-tree README.md files are byte-identical across the three repos
  And no file is present in one repo and missing or different in another
```

### AC-5 — rhino-cli source stays byte-identical (US-2)

```gherkin
Scenario: The rhino-cli source remains byte-identical after the step-def edits propagate
  Given the de-hollow edits to apps/rhino-cli/tests/*.rs and the regenerated golden-master in ose-public
  When apps/rhino-cli is compared across the three repos, excluding untracked build artifacts
  Then apps/rhino-cli is byte-identical across ose-public, ose-primer, and ose-infra
  And each repo's rhino-cli golden-master test passes
```

### AC-6 — The two stale primer files are reconciled to real behaviour (US-3, US-4)

```gherkin
Scenario: Renamed-command behaviours are re-expressed against current commands
  Given primer's stale env/env-validate.feature and repo-governance-gherkin-keyword-cardinality.feature
  When the canonical tree is finalized
  Then the env-validate and gherkin-cardinality behaviours each own an executing scenario under the current command name
  And the two stale files no longer exist under their pre-union names in any repo
```

### AC-7 — Anti-drift gate covers the Gherkin tree (US-5)

```gherkin
Scenario: The rhino-cli identity boundary explicitly includes the Gherkin tree
  Given the SDLC gate standard and the multi-repo parity workflow
  When a maintainer consults the rhino-cli byte-identity boundary
  Then the Gherkin tree path is listed inside the boundary
  And the parity workflow has a verification step asserting the tree is identical across the three repos
```

### AC-8 — Each repo passes its own full gate (US-1)

```gherkin
Scenario: Every repo is green on its own pre-push gate after propagation
  Given the changes applied to a repo in its phase
  When that repo's full pre-push gate runs
  Then typecheck, lint, and the rhino-cli test suite pass with zero skipped scenarios
  And CI on main is green after push
```

### AC-9 — Feature-dir names match their command group (US-3)

```gherkin
Scenario: Every gherkin feature dir maps to its command group
  Given the renamed feature-dir mapping from Phase 0 (docs->md, agents->harness, plus any other mismatch)
  When the gherkin tree and the tests/*.rs feature_dir bindings are inspected
  Then each feature dir name matches the rhino-cli command group it exercises
  And no feature dir retains a legacy name that mismatches its command group
```

### AC-10 — repo-config schema gate runs at pre-commit + PR + main, not pre-push (US-1, US-5)

```gherkin
Scenario: A malformed repo-config.yml is rejected at commit, PR, and main
  Given a repo-config.yml containing an unknown or misspelled key in any of the three repos
  When the config is staged for commit, or a PR runs, or main-ci runs
  Then rhino-cli repo-config validate rejects it at pre-commit (staged-gated), the PR quality gate, and the main quality gate
  And the step is absent from .husky/pre-push and byte-identical across all three repos at the three retained points
```

### AC-11 — The behaviour suite runs in the pre-push gate at test:unit (US-1)

```gherkin
Scenario: The mocked behaviour suite runs inside test:quick
  Given rhino-cli test:unit rewired to the in-process mocked behaviour suite
  When a developer runs the pre-push gate (nx affected -t test:quick)
  Then the rhino-cli behaviour scenarios execute at the unit tier with mocked I/O
  And test:integration still runs the temp-fixture binary-spawn suite as the heavier tier
```

### AC-12 — An unimplemented scenario fails the build (US-1)

```gherkin
Scenario: A skipped or undefined cucumber step reddens the build
  Given the cucumber harness configured with fail_on_skipped
  When a scenario contains a step with no matching step definition
  Then the test run exits non-zero and names the offending scenario
  And no scenario can silently skip while the suite reports success
```

## Product Scope

**In:** de-hollowing (step-vocab alignment in `tests/*.rs`), wiring the 4 unbound dirs, gap-fill
features for uncovered leaf commands, byte-identical Gherkin `.feature` + behaviour-`README.md` across
three repos, golden-master regeneration, the anti-drift gate (SDLC boundary doc + parity-workflow step).

**Out:** validator logic changes; C4 architecture docs unification; new runtime drift-detection tooling;
app/language-set divergence; `repo-config.yml` data values.

## Product Risks

| Risk                                                                                                  | Mitigation                                                                                                                                                 |
| ----------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| De-hollowing reveals a scenario whose described behaviour and the current command genuinely disagree. | Treat as a real finding: fix the validator or correct the scenario to the intended behaviour, documented in the phase notes — never re-skip to stay green. |
| A gap-fill scenario duplicates an existing one under a different name.                                | Phase 0 command-census ↔ feature map dedupes before authoring; reuse the existing scenario.                                                                |
| Golden-master churn masks a real regression.                                                          | Regenerate only after the suite is green; review the golden-master diff for intent before freezing.                                                        |
