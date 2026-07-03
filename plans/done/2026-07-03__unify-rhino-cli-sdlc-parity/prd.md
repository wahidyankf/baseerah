# PRD — Unify rhino-cli, SDLC & Repo Structure (Second Pass)

## Product Overview

A configuration-plus-source product: the convergence edits to three repos' `apps/rhino-cli` (Rust
source, `Cargo.toml`, `Cargo.lock`, `project.json`, `LICENSE`), hooks (`.husky/*`), workflows
(`.github/workflows/*`), `repo-config.yml`, per-project `project.json` targets, and the reference/
governance docs — driving all three to one byte-identical structure (`apps/rhino-cli` identical with
zero carve-outs; only app/language set and the CI runner label legitimately differ).

Unlike the first plan (which standardized _wiring_ and the rhino-cli _target set_), this pass makes
the **rhino-cli source itself** identical — as the **union** of all three repos' command surfaces —
and wires its **cucumber-rs BDD harness** (migrated to `0.23.0`) with a **reconciled-union `.feature`
tree** in all three repos. rhino-cli source changes follow TDD (RED/GREEN/REFACTOR) with companion
`specs/` Gherkin; pure config/wiring edits are verified by running the affected target/hook, not a TDD
code cycle.

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
- As Maya, I want the primer→public→primer round trip to never regress primer, so making public the
  canonical seat is safe even though primer is the more-evolved tree today.
- As Theo, I want every Nx project to wire `namedInputs.specs` so a specs-only change is caught at
  pre-push/PR, not silently deferred to main-ci.
- As Theo, I want all three `repo-config.yml` files to share one key set (enforced by a gate) so the
  byte-identical source can read config keys without a missing-key runtime break in one repo.
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
- **rhino-cli source identity**: converge `src/`, `Cargo.toml`, `Cargo.lock`, `project.json`, and an
  `apps/rhino-cli/LICENSE` to one canonical form **100% byte-identical across repos, zero carve-outs**,
  carrying the **union of all three command surfaces**. Synthesize canonical in ose-public; relicense
  infra's CLI to MIT with an `apps/rhino-cli/`-scoped `LICENSE` (Decision 3 + 12).
- **cucumber-rs harness**: adopt primer's harness structure (`tests/*.rs`, fixtures, golden-master)
  as canonical **migrated to `0.23.0`**, with the `.feature` surface as the **reconciled union** of all
  three trees; present + passing in all 3; golden-master regenerated from the canonical result.
- **Round-trip guard**: a Phase-0 primer behavior baseline (cucumber + golden + CLI output) that the
  Phase 3 gate must pass, plus a Phase-1 file-accounting ledger for every current-primer file.
- **Full `namedInputs.specs` rollout** on every Nx-registered project in all 3 repos, enumerated via
  `nx show projects` (including the `*-contracts` projects rooted under
  `specs/apps/*/containers/contracts/`).
- **Config schema-parity gate**: a new `rhino-cli repo-config validate` command that strict-deserializes
  `repo-config.yml` against the canonical schema (deny-unknown-fields + required/enum checks), giving
  all three repos an identical key set as an emergent property (values may differ); run on
  **pre-commit** (fast path, fires only when `repo-config.yml` is staged) and **pre-push/PR**
  (defense-in-depth).
- **Missing mandatory targets** added to the 6 infra projects; `coverage.projects` registry completed.
- **Repo-specific behaviour driven from `repo-config.yml`** (env-validation scan paths, domain/ddd
  areas) so `src/` AND every `project.json` command string are byte-identical (Decision 5).
- **Latent-bug fixes**: `.opencode/agent/`→`.opencode/agents/` trigger path; public PR-gate
  `gherkin-cardinality` step; stale `specs/libs/golang-commons` orphan removed in public;
  `repo-config.yml` header-comment canonicalized. Both validator fixes are dry-run in Phase 0 before
  arming.
- **SDLC mechanism convergence (zero `⚠️`)**: infra hooks/CI converge to direct `cargo run` +
  lint-staged + lower-kebab workflow names + missing jobs added; `*.cs/.clj/.dart` format mechanism
  unified across repos.
- **Governance/docs convergence**: reference docs + governance conventions + `AGENTS.md` sections kept
  identical across repos.

### Out of Scope

- App-set / language-set unification; validator logic changes.
- New automated parity-enforcement tooling for rhino-cli byte-identity (possible future follow-up).
- **No descope path** — Phase 4 (infra) is required; byte-identity is non-negotiable (Decision 7).

## Acceptance Criteria (Gherkin)

```gherkin
Feature: rhino-cli source is byte-identical across the three repos

  Scenario: The Rust source directories are identical
    Given apps/rhino-cli/src in ose-public, ose-primer, and ose-infra
    When `diff -rq` is run between any two of them
    Then it reports no differing files and no only-in-one files
    And each binary carries the union of all three command surfaces (ddd, specs, workflows, contracts, java, test-coverage)

  Scenario: Cargo manifests, lockfile, and license are byte-identical
    Given apps/rhino-cli/Cargo.toml, Cargo.lock, and LICENSE in all three repos
    When they are diffed pairwise
    Then there are no differences (infra's CLI is relicensed to MIT with an apps/rhino-cli/-scoped LICENSE)
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
    And the specs/apps/rhino/behavior/rhino-cli/gherkin tree — the reconciled union of all three original trees — is identical across repos

  Scenario: A union scenario for a repo-inapplicable toolchain no-ops safely
    Given a .feature scenario requiring a toolchain absent in this repo (e.g. a java scenario in ose-public)
    When `cargo test` runs in that repo
    Then the scenario is skipped by data (tag), not failed
    And `cargo test` stays green

  Scenario: A new rhino-cli behaviour lands with a scenario
    Given a change to rhino-cli behaviour
    When the change is committed
    Then a companion .feature scenario exists and is covered by a step definition
```

```gherkin
Feature: The primer round trip does not regress primer

  Scenario: Canonical-primer passes the frozen behavior baseline
    Given the Phase-0 behavior baseline of ose-primer's rhino-cli (cucumber, golden-master, CLI --help/output)
    When ose-primer's rhino-cli is replaced by the canonical (Phase 3)
    Then canonical-primer passes every check in the frozen baseline

  Scenario: Every current-primer file is accounted for
    Given the Phase-1 file-accounting ledger of current-primer's rhino-cli files
    When the canonical is synthesized
    Then each file is recorded as ported, merged, or explicitly-dropped-with-reason
    And no current-primer capability is dropped without a logged reason
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
    When the workflow files are inspected
    Then pr-quality-gate.yml, validate-env.yml, main-ci.yml, and deps-audit.yml exist with lower-kebab names
    And no validate-markdown.yml / markdown-validate.yml exists
    And pr-quality-gate.yml runs gherkin-cardinality in its specs-gate in all three
    And main-ci.yml has standalone compat-min-version and env-validate jobs in all three
    And ose-infra's workflow `name:` values are lower-kebab consistent with public and primer
```

```gherkin
Feature: namedInputs.specs is wired on every project

  Scenario: Every Nx project wires the specs input
    Given every Nx-registered project in each repo, enumerated via nx show projects (including the *-contracts projects rooted under specs/apps/*/containers/contracts/)
    When namedInputs.specs presence is counted
    Then the count equals the total nx show projects count in that repo
    And a specs-only change marks the owning project affected at pre-push and the PR gate
```

```gherkin
Feature: Every project declares the mandatory targets

  Scenario: No project is missing a mandatory target
    Given every Nx-registered project in each repo, enumerated via nx show projects (not just direct children of apps/ or libs/ — also the *-contracts projects rooted under specs/apps/*/containers/contracts/)
    When its project.json targets are inspected
    Then test:unit, test:integration, test:e2e, test:quick, lint, typecheck, test:coverage, the specs:* targets, deps:audit, and compat:min-version are all present (echo where N/A)
    And the 6 previously-missing ose-infra projects declare deps:audit and compat:min-version
```

```gherkin
Feature: repo-config.yml is byte-identical modulo per-repo data

  Scenario: The schema and header comment are identical
    Given repo-config.yml in all three repos
    When the top-level keys and the header comment block are diffed
    Then the schema keys and the header comment block are byte-identical
    And only the per-repo data values (harness list is identical; domain-areas / env globs differ per repo) vary

  Scenario: A schema-parity gate enforces the identical key set
    Given "rhino-cli repo-config validate" in each repo's pre-commit and pre-push/PR
    When repo-config.yml is validated
    Then the command strict-deserializes it against the canonical RepoConfig schema
    And it passes when only values differ
    And it fails when a required key is missing or an unknown key is present
    And running it independently against the byte-identical schema in all three repos is equivalent to
      an identical key set across all three repo-config.yml files

  Scenario: Repo-specific behaviour is data-driven, not hard-coded
    Given rhino-cli's repo-specific behaviour (env globs, domain/ddd areas)
    When rhino-cli runs
    Then it reads that behaviour from repo-config.yml, not from source hard-coded per repo

  Scenario: IaC env-validation is preserved in the canonical
    Given ose-infra declares terraform and ansible surfaces in repo-config.yml
    When env validate runs
    Then validate_terraform and validate_ansible execute and report drift
    And ose-public and ose-primer, which declare no such surfaces, skip validation by data, not by stub
```

```gherkin
Feature: Latent validator bugs are fixed

  Scenario: Dormant gates are dry-run before they are armed
    Given the two fixed validators (agent-naming trigger path, gherkin-cardinality)
    When Phase 0 dry-runs each against the current tree in all three repos
    Then any existing violation is recorded as an explicit remediation item before the gate is armed

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

  Scenario: No phase leaves a repo half-converged
    Given a phase boundary where the plan may pause
    When the phase gate runs
    Then the just-touched repo passes its own full pre-push + PR gate before a pause is allowed
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
  canonical carries the cucumber + unit + golden-master suites (golden-master regenerated from the
  canonical result); infra must pass all three post-port. Phase 4 is required — there is no descope.
- **Risk: the canonical synthesis silently drops infra's real Terraform/Ansible env-drift
  validators.** public/primer today carry only a doc-comment stub for non-`"app"` surface kinds; a
  best-of-two synthesis (cucumber + testcoverage only) would regenerate infra to that stub, deleting
  its only real IaC drift-detection logic without any acceptance criterion catching the loss.
  Mitigation: the canonical synthesis is explicitly best-of-**three** — `validate_terraform`/
  `validate_ansible` (+ their tests) are ported from infra into the canonical `application/env/
validate.rs` in Phase 1, asserted by the "IaC env-validation is preserved in the canonical" scenario
  above, and infra's env-validation scan paths move to `repo-config.yml` as data, so the same
  validators activate for infra (which declares `terraform`/`ansible` surfaces) and no-op for
  public/primer (which do not) — the Phase 5 diff matrix and Phase 4 Gate's
  `terraform_validator::`/`ansible_validator::` test runs both confirm zero `apps/rhino-cli`
  differences and functional presence.
- **Risk: the union `.feature` surface pulls a toolchain into a repo that lacks it.** Mitigation:
  union scenarios requiring a repo-inapplicable toolchain are tagged and skipped by data (the same
  no-op-by-data pattern as SurfaceKind); the Phase 1 RED/GREEN cycle asserts `cargo test` stays green
  in a repo that lacks the toolchain.
- **Risk: config schema drift makes byte-identical source runtime-broken in one repo.** Mitigation:
  `rhino-cli repo-config validate` strict-deserializes each repo's `repo-config.yml` against the
  byte-identical schema on pre-commit (fast, local, catches the mistake before it's even committed)
  and again on pre-push/PR as defense-in-depth.
- **Risk: pulling primer's testcoverage/cucumber into public expands public's rhino-cli surface, and
  the round trip could regress primer.** Mitigation: synthesis is gated by public's own suites; the
  golden-master is regenerated deliberately; the primer round-trip is guarded by the Phase-0 behavior
  baseline + the file-accounting ledger.
