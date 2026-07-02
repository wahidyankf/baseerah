# PRD — Unify rhino-cli, SDLC & Repo Structure (Second Pass)

## Product Overview

A configuration-plus-source product: the convergence edits to three repos' `apps/rhino-cli` (Rust
source, `Cargo.toml`, `Cargo.lock`, `project.json`), hooks (`.husky/*`), workflows
(`.github/workflows/*`), `repo-config.yml`, per-project `project.json` targets, and the reference/
governance docs — driving all three to one byte-identical structure (`apps/rhino-cli` identical with
zero carve-outs; only app/language set and the CI runner label legitimately differ).

Unlike the first plan (which standardized _wiring_ and the rhino-cli _target set_), this pass makes
the **rhino-cli source itself** identical and wires its **cucumber-rs BDD harness** in all three
repos. rhino-cli source changes follow TDD (RED/GREEN/REFACTOR) with companion `specs/` Gherkin; pure
config/wiring edits are verified by running the affected target/hook, not a TDD code cycle.

## Personas

- **Maya, the maintainer** — operates all three repos; wants one mental model that now reaches into the
  tool's own source, and frictionless copy-not-translate propagation.
- **Theo, the agent** (`ci-checker`, `swe-rust-dev`) — needs one canonical rhino-cli + gate standard to
  validate and edit each repo against.
- **Sam, the downstream consumer** — clones `ose-primer`; inherits an identical, cucumber-covered tool.

## User Stories

- As Maya, I want `apps/rhino-cli` to be byte-identical across all three repos so a fix I make in one
  is a literal copy into the others, not a re-port.
- As Maya, I want rhino-cli's own behaviour cucumber-covered in every repo so the coverage-enforcing
  tool is itself spec-covered everywhere, not just in primer.
- As Maya, I want every gate invoked through the identical mechanism in all three repos so a green
  check means the same thing everywhere (no `npx nx` vs `cargo run` divergence).
- As Theo, I want every Nx project to wire `namedInputs.specs` so a specs-only change is caught at
  pre-push/PR, not silently deferred to main-ci.
- As Theo, I want the audit grounded in the working tree, not in stale "done" notes, so drift from the
  standard is detectable mechanically.
- As Sam, I want `apps/rhino-cli` to be identical everywhere with no carve-outs, and the only divergence
  (app/language set + the CI runner label) to be an explicit short list, so the template stays lean and
  the identity claim is honest.

## Scope

### In Scope

- **Same surface as the first plan** (rhino-cli command set + verb-last naming, Nx target names +
  contents, per-project mandatory targets, specs C4 structure, unified `repo-config.yml`, harness
  binding coverage, canonical GitHub CI, CRON test+deploy shape, worktree-agnostic guardrails).
- **rhino-cli source identity**: converge `src/`, `Cargo.toml`, `Cargo.lock`, `project.json` to one
  canonical form **100% byte-identical across repos, zero carve-outs**. Synthesize canonical in
  ose-public; relicense infra's CLI to MIT (Decision 3).
- **cucumber-rs harness**: adopt primer's wired harness (`tests/*.rs`, fixtures, golden-master,
  `specs/apps/rhino/behavior/rhino-cli/gherkin/*.feature`) as canonical; present + passing in all 3.
- **Full `namedInputs.specs` rollout** on every Nx project in all 3 repos.
- **Missing mandatory targets** added to the 5 infra projects; `coverage.projects` registry completed.
- **Repo-specific behaviour driven from `repo-config.yml`** (env-validation scan paths, domain/ddd
  areas) so `src/` AND every `project.json` command string are byte-identical (Decision 5).
- **Latent-bug fixes**: `.opencode/agent/`→`.opencode/agents/` trigger path; public PR-gate
  `gherkin-cardinality` step; stale `specs/libs/golang-commons` orphan removed in public;
  `repo-config.yml` header-comment canonicalized.
- **SDLC mechanism convergence (zero `⚠️`)**: infra hooks/CI converge to direct `cargo run` +
  lint-staged + lower-kebab workflow names + missing jobs added; `*.cs/.clj/.dart` format mechanism
  unified across repos.
- **Governance/docs convergence**: reference docs + governance conventions + `AGENTS.md` sections kept
  identical across repos.

### Out of Scope

- App-set / language-set unification; validator logic changes.
- New automated parity-enforcement tooling (possible future follow-up).

## Acceptance Criteria (Gherkin)

```gherkin
Feature: rhino-cli source is byte-identical across the three repos

  Scenario: The Rust source directories are identical
    Given apps/rhino-cli/src in ose-public, ose-primer, and ose-infra
    When `diff -rq` is run between any two of them
    Then it reports no differing files and no only-in-one files

  Scenario: Cargo manifests and lockfile are byte-identical
    Given apps/rhino-cli/Cargo.toml and Cargo.lock in all three repos
    When they are diffed pairwise
    Then there are no differences (infra's CLI is relicensed to MIT)
    And the dependency set and versions (including cucumber 0.23.0) are identical

  Scenario: project.json target commands are byte-identical
    Given apps/rhino-cli/project.json in all three repos
    When the targets are diffed pairwise
    Then the target key set and every command string are identical
    And there are no carve-out inputs — env-validation scan paths are read from repo-config.yml, not hard-coded per repo
```

```gherkin
Feature: rhino-cli's own behaviour is cucumber-covered in every repo

  Scenario: The cucumber harness runs in each repo
    Given apps/rhino-cli in ose-public, ose-primer, and ose-infra
    When `cargo test` runs in each repo
    Then the cucumber [[test]] suites execute and pass in all three
    And the tests/*.rs harness files are identical across repos
    And the specs/apps/rhino/behavior/rhino-cli/gherkin tree is identical across repos

  Scenario: A new rhino-cli behaviour lands with a scenario
    Given a change to rhino-cli behaviour
    When the change is committed
    Then a companion .feature scenario exists and is covered by a step definition
```

```gherkin
Feature: SDLC gate mechanism is identical (zero mechanism divergence)

  Scenario: Every gate uses the identical invocation mechanism
    Given the .husky hooks and .github workflows in all three repos
    When a shared gate (env-guard, bindings, naming, vendor-audit, instruction-size, specs) is inspected
    Then it is invoked through the identical mechanism in all three (direct `cargo run`, not `npx nx run rhino-cli:*` or `npm run`)
    And tool-lint (shellcheck/hadolint/actionlint) runs via lint-staged in all three, not inline shell
    And only documented infra-only IaC steps differ

  Scenario: The parity table has no warning rows
    Given the Phase 5 cross-repo parity table
    When it is inspected
    Then every mechanics row is ✅
    And no row is marked ⚠️ (functionally-equivalent mechanism divergence)
```

```gherkin
Feature: Canonical workflow names and jobs are identical

  Scenario: Canonical workflows exist with identical names and job skeletons
    Given .github/workflows in all three repos
    Then pr-quality-gate.yml, validate-env.yml, main-ci.yml, and deps-audit.yml exist with lower-kebab names
    And no validate-markdown.yml / markdown-validate.yml exists
    And pr-quality-gate.yml runs gherkin-cardinality in its specs-gate in all three
    And main-ci.yml has standalone compat-min-version and env-validate jobs in all three
    And ose-infra's workflow `name:` values are lower-kebab consistent with public and primer
```

```gherkin
Feature: namedInputs.specs is wired on every project

  Scenario: Every Nx project wires the specs input
    Given every project.json in each repo
    When namedInputs.specs presence is counted
    Then the count equals the total project count in that repo
    And a specs-only change marks the owning project affected at pre-push and the PR gate
```

```gherkin
Feature: Every project declares the mandatory targets

  Scenario: No project is missing a mandatory target
    Given every direct child of apps/ or libs/ registered with Nx in each repo
    When its project.json targets are inspected
    Then test:unit, test:integration, test:e2e, test:quick, lint, typecheck, test:coverage, the specs:* targets, deps:audit, and compat:min-version are all present (echo where N/A)
    And the 5 previously-missing ose-infra projects declare deps:audit and compat:min-version
```

```gherkin
Feature: repo-config.yml is byte-identical modulo per-repo data

  Scenario: The schema and header comment are identical
    Given repo-config.yml in all three repos
    When the top-level keys and the header comment block are diffed
    Then the schema keys and the header comment block are byte-identical
    And only the per-repo data values (harness list is identical; domain-areas / env globs differ per repo) vary

  Scenario: Repo-specific behaviour is data-driven, not hard-coded
    Given rhino-cli's repo-specific behaviour (env globs, domain/ddd areas)
    When rhino-cli runs
    Then it reads that behaviour from repo-config.yml, not from source hard-coded per repo
```

```gherkin
Feature: Latent validator bugs are fixed

  Scenario: The agent-naming validator fires
    Given an agent file renamed to an invalid suffix
    When the naming validator runs (triggered on .opencode/agents/ changes)
    Then it detects the invalid name and fails
    And no trigger path references the singular .opencode/agent/

  Scenario: The stale orphan spec is gone
    Given ose-public
    When `find specs -type d -name gherkin -not -path '*/behavior/*'` runs
    Then it returns nothing
    And specs/libs/golang-commons no longer exists in ose-public
```

```gherkin
Feature: The audit is grounded in reality, not stale notes

  Scenario: Phase 0 re-audits against the working tree
    Given the first plan's delivery.md "done" notes
    When Phase 0 runs
    Then the current-state matrices are recomputed from the working tree (diff/jq/grep)
    And every delivery item cites a concrete verification command, not a prior "done" note
```

```gherkin
Feature: Convergence introduces no regressions

  Scenario: Each repo stays green after convergence
    Given a converged repo
    When its affected pre-push gate and PR quality-gate run on a no-op change
    Then all gates pass
    And rhino-cli's unit + cucumber suites pass
    And no previously-passing gate is removed without a divergence-policy entry
```

```gherkin
Feature: Legitimate divergence is preserved

  Scenario: Only app/language-set divergence remains
    Given the converged repos
    When the divergence policy is applied
    Then ose-infra retains its terraform/ansible/yamllint gates and self-hosted runner label
    And each repo retains only the per-app deploy CRONs for apps it ships
    And apps/rhino-cli is byte-identical across all three repos with no carve-outs
    And the only sanctioned divergence is app/language set, IaC gates, and the runner label (CI-workflow layer)
    And these are recorded in the divergence policy, not flagged as drift
```

## Product Risks

- **Risk: the infra rhino-cli regeneration introduces subtle behaviour changes.** Mitigation: the
  canonical carries the cucumber + unit + golden-master suites; infra must pass all three post-port.
- **Risk: data-driving env-validation paths regresses infra's IaC env scanning.** Mitigation: infra's
  IaC scan paths move to `repo-config.yml` and are asserted by a `.feature` scenario + the Phase 5
  diff matrix (which must show zero `apps/rhino-cli` differences).
- **Risk: pulling primer's testcoverage/cucumber into public expands public's rhino-cli surface.**
  Mitigation: synthesis is gated by public's own suites; the golden-master is refreshed deliberately.
