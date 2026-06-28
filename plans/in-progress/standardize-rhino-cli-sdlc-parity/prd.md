# PRD — Standardize rhino-cli Checks & SDLC Commands

## Product Overview

A documentation-plus-configuration product: one committed **command triage** doc, one committed
**SDLC standard** doc, and the convergence edits to three repos' hooks (`.husky/*`), workflows
(`.github/workflows/*`), and rhino-cli Nx targets (`apps/rhino-cli/project.json`) so the gate
mechanics match the standard.

This plan primarily standardizes wiring (shell hooks, YAML workflows, Nx target definitions,
reference docs, config files), with targeted Rust source changes in `apps/rhino-cli/src/` to
rename, extend, and remove commands as part of the command-set convergence. Where it edits an
`apps/rhino-cli` Nx target's wiring (not its Rust source), the change is a config edit verified
by running the target, not a TDD code cycle.

## Personas

- **Maya, the maintainer** — operates all three repos; wants one mental model and frictionless propagation.
- **Theo, the agent** (`ci-checker`) — needs a single canonical standard to validate each repo against.
- **Sam, the downstream consumer** — clones `ose-primer` as a template; inherits the coherent gate model.

## User Stories

- As Maya, I want every rhino-cli command labelled wired/not-wired so I know which gates are actually enforced.
- As Maya, I want the PR quality-gate workflow to have the same filename and job structure in all three repos, so that a cross-repo change always targets the same gate file without consulting a per-repo lookup table.
- As Maya, I want the markdown / env validation workflows to run the same validator set everywhere, so that markdown and env issues surface uniformly regardless of which repo a change lands in.
- As Theo, I want a committed standard doc I can diff each repo against, so that drift from the standard is detectable mechanically without manual inspection.
- As Sam, I want infra-only gates (terraform/ansible) to stay in infra and not leak into the template, so that cloned templates remain lean and do not carry IaC tooling requirements irrelevant to their context.

## Scope

### In Scope

- Triage table covering every leaf subcommand under the 11 rhino-cli families (TestCoverage, RepoGovernance, Md, Convention, Harness, Workflows, Specs, Lang, Git, Env, Doctor), each with a one-line description, wired/not-wired status, and invocation site. [Repo-grounded]
- Nx target-name standardization: canonical lifecycle + `{domain}:{work}` names for every hook/CI-invoked target, identical rhino-cli target sets across the three repos (remove `fmt`/`format:check` → formatting via file-type lint-staged, shell/Dockerfile/workflow linting via lint-staged file-type entries (no `{tool}:lint` Nx targets), `harness:bindings-validation` as a cacheable Nx target while `harness:bindings-generate` + the env-guard run as direct `cargo run` calls, remove `test-coverage` → native `test:coverage`, `specs:coverage`→`specs:behavior:coverage` + new `specs:domain:coverage` on `*-be`). [Repo-grounded]
- Single merged `repo-config.yml` (instruction-size + env-contract + env-injection sections); Codecov fully removed (native coverage only); every project covered by a standardized GitHub CI named per ose-public convention. [Repo-grounded]
- Testing-architecture standard: mandatory-six targets (no `format`) on every project (echo where N/A), `test:quick` = typecheck→lint→test:unit→test:coverage→test:specs (test:specs aggregates the specs:_ validators; all composed targets present on every project, echo where N/A), three levels consuming the same Gherkin, BE service-level integration / FE-DB-only integration / `_-e2e`-only e2e, pre-push/PR/main-ci running `test:quick`while pre-commit stays fast (format + tool-lint + guards, no`test:quick`), no gate running integration/e2e (CRON-only), and rhino-cli feature-consumption enforcement. [Repo-grounded]
- Standardized gate mechanics for: commit-msg, pre-commit, pre-push, PR quality-gate, main-ci, env-validate, and the CRON "test local + deploy stag" / "test stag + deploy prod" pipeline _shape_ (markdown validation folds into the gates — no standalone markdown workflow).
- Convergence edits in all three repos.

### Out of Scope

- App-set / language-set unification.
- Validator logic changes.
- Removing or newly-wiring not-wired commands (triage only).
- New automated parity-enforcement tooling (noted as a follow-up).

## Acceptance Criteria (Gherkin)

```gherkin
Feature: rhino-cli command triage is complete and published

  Scenario: Every command is triaged
    Given the rhino-cli CLI definition in apps/rhino-cli/src/cli.rs
    When the command triage reference doc is generated
    Then every leaf subcommand appears exactly once in the triage table
    And each row is labelled "wired" or "not wired"
    And every "wired" row names its exact invocation site (hook step, workflow job, or Nx target)
```

```gherkin
Feature: PR quality-gate mechanics are identical across repos

  Scenario: The PR gate workflow has the same filename and job skeleton everywhere
    Given the three repos ose-public, ose-primer, and ose-infra
    When the PR quality-gate workflow file is inspected in each
    Then the workflow filename matches the standardized name in all three
    And the gate's job skeleton (detect, language-gate, markdown, naming, env, specs-gate, quality-gate sentinel) is present in all three
    And only the language-specific gate jobs and infra-only IaC jobs differ between repos
```

```gherkin
Feature: Markdown and env validation run identical validator sets

  Scenario: Markdown validation is folded into the gates, no standalone workflow
    Given the standardized gates in any of the three repos
    When markdown is validated
    Then the per-file validators (markdownlint-cli2, md mermaid validate, md heading-hierarchy validate) and gherkin-cardinality (`.feature` only) run via lint-staged
    And md links validate runs repo-wide as the md-links gate job (pre-push/PR/main)
    And no standalone validate-markdown.yml workflow exists
    And the set is identical across all three repos
```

```gherkin
Feature: Hook ordering and invocation mechanism are identical

  Scenario: pre-commit and pre-push run the same steps in the same order
    Given the standardized .husky/pre-commit and .husky/pre-push hooks
    When a contributor inspects the hooks in any repo
    Then the ordered list of gate steps matches the standard
    And each shared gate is invoked through the same mechanism (rhino-cli commands via direct `cargo run`, project targets via `nx affected`, not inline shell)
    And only infra-only IaC steps appear as documented additions in ose-infra
```

```gherkin
Feature: Nx target names are canonical and identical across repos

  Scenario: Hook/CI-invoked targets follow the canonical scheme
    Given the converged repos
    When the Nx targets invoked by any hook or CI workflow are inspected
    Then each target name comes from the canonical lifecycle or {domain}:{work} scheme in nx-targets.md
    And no project declares a `format` or `format:check` target (formatting is file-type lint-staged)
    And shell/Dockerfile/Actions linting runs as lint-staged file-type entries (`shellcheck`/`hadolint`/`actionlint`), not Nx targets, in all three
    And a check is wired into lint-staged only when it is file-type-based and per-file isolated, so cacheable read-only checks (test:quick, harness:bindings-validation) stay Nx targets while always-run commands (harness:bindings-generate, env staged-guard) are invoked directly, the env guard kept as the dedicated first-line secrets gate
    And `harness:bindings-validation` is invoked as an Nx target, not an npm script, in all three
    And the rhino-cli `test-coverage` command and Nx target are absent in all three

  Scenario: The rhino-cli target set is identical across repos
    Given the converged repos
    When `jq -r '.targets | keys[]' apps/rhino-cli/project.json` is run in each
    Then the sorted key set is identical across ose-public, ose-primer, and ose-infra
```

```gherkin
Feature: Every project declares the mandatory six targets

  Scenario: All apps and libs expose the six targets
    Given any direct child folder of apps/ or libs/ registered with Nx
    When its project.json targets are inspected
    Then test:unit, test:integration, test:e2e, test:quick, lint, and typecheck are all present
    And no `format` target is present (formatting is file-type lint-staged)
    And a native test:coverage target is present on every project (real ≥90% where test:unit is real, echo elsewhere) since test:quick composes it
    And targets that do not apply to the project are declared as no-op echo placeholders
    And test:e2e has a real (non-echo) command only on *-e2e projects

  Scenario: Backend projects additionally declare specs:domain:coverage
    Given a *-be backend project
    When its project.json targets are inspected
    Then specs:domain:coverage is present
    And non-*-be projects do not declare specs:domain:coverage
```

```gherkin
Feature: test:quick runs typecheck, lint, test:unit, test:coverage, then test:specs

  Scenario: test:quick composes the five in order
    Given a project's test:quick target
    When test:quick runs
    Then it runs typecheck, then lint, then test:unit, then test:coverage (≥90% line), then test:specs, in that exact order
    And it stops at the first failing step
    And test:specs aggregates the specs:* validators (adoption, tree, counts, links, behavior:coverage, and domain:coverage on *-be)
    And the specs gate runs inside test:quick, so there is no separate specs-structural gate step
```

```gherkin
Feature: The three test levels consume the same Gherkin

  Scenario: unit, integration, and e2e share feature files
    Given a project with feature files under its specs gherkin directory
    When test:unit, test:integration, and test:e2e run
    Then all three consume the same feature files driven by the same tags
    And every feature and every scenario is exercised by at least one eligible level
    And test:unit may additionally carry non-Gherkin unit tests for behaviour not expressed as scenarios
    And BE test:integration exercises behaviour at the service level, never through the HTTP API
    And the HTTP API surface is exercised only by test:e2e in the *-e2e project
```

```gherkin
Feature: Every feature and scenario is consumed by an eligible test

  Scenario: An orphan feature file fails the gate
    Given a feature file under specs that no test references
    When rhino-cli specs behavior-coverage validate runs with --require-consumption
    Then it fails and names the orphan feature file

  Scenario: An uncovered scenario fails the gate
    Given a scenario in a binding feature file that no eligible unit, integration, or e2e test exercises
    When rhino-cli specs behavior-coverage validate runs with --require-consumption
    Then it fails and names the uncovered feature and scenario
```

```gherkin
Feature: Backend domain entities are covered (specs:domain:coverage)

  Scenario: An uncovered domain entity fails the gate
    Given a *-be project with a domain entity that no domain unit test exercises
    When rhino-cli specs domain-coverage validate runs
    Then it fails and names the uncovered domain entity
```

```gherkin
Feature: Formatting is file-type based via lint-staged

  Scenario: Committing a source file formats it by file type
    Given a staged file of any supported type (md, json, yaml, ts, rs, fs)
    When the pre-commit hook runs
    Then lint-staged formats it by its file type
    And no per-project format or format:check Nx target is invoked
```

```gherkin
Feature: Git identity is not mechanically blocked; agents never set it

  Scenario: pre-commit no longer aborts on a per-repo identity override
    Given a contributor has set a per-repo user.email in .git/config
    When the pre-commit hook runs
    Then it does not abort because of the identity override
    And scripts/git-identity-check.sh does not exist in any repo

  Scenario: agents never set a per-repo git identity
    Given an AI agent operating in a repo or worktree
    When it prepares a commit
    Then it does not run git config to set user.name or user.email at any scope
    And commit identity comes from the developer's global git config
    And the Git Identity Guardrail is published in AGENTS.md and a governance convention
```

```gherkin
Feature: The staged-.env guard is a rhino-cli command, not inline shell

  Scenario: Committing a real .env file is rejected
    Given a real .env file is staged for commit
    When the pre-commit hook runs rhino-cli env staged-guard validate
    Then it exits non-zero and names the offending file
    And the commit is aborted

  Scenario: Committing .env.example is allowed
    Given only .env.example is staged
    When rhino-cli env staged-guard validate runs
    Then it exits zero and the commit proceeds

  Scenario: The guard is a direct rhino-cli call, not a shell script
    Given the converged repos
    When the pre-commit hook is inspected
    Then it invokes the rhino-cli env staged-guard validate command directly as the first step
    And no nx run wrapper or Nx target is used for it
    And scripts/check-no-env-staged.sh does not exist
```

```gherkin
Feature: Repo configuration is unified in repo-config.yml

  Scenario: rhino-cli reads merged config sections
    Given a repo with repo-config.yml at its root
    When rhino-cli runs instruction-size or env validation
    Then it reads the instruction-size / env-contract / env-injection section from repo-config.yml
    And the standalone instruction-size-budget.yaml / env-contract.yaml / env-injection.yaml files are absent

  Scenario: Codecov is fully removed
    Given any of the three repos
    When the working tree is scanned for codecov references
    Then only ExcludeFromCodeCoverage attribute hits remain
    And no codecov.yml config file exists
```

```gherkin
Feature: Every project is covered by a standardized GitHub CI

  Scenario: Canonical CI workflows exist with ose-public naming
    Given a converged repo
    When its .github/workflows directory is inspected
    Then pr-quality-gate.yml, validate-env.yml, and main-ci.yml are present with those exact names (no standalone validate-markdown.yml — markdown folds into the gates)
    And every project is covered by main-ci.yml, which runs `nx run-many --all` (total coverage by construction)
```

```gherkin
Feature: Pre-push and PR gate run identical fast commands

  Scenario: Both gates run only test:quick for affected projects
    Given a push and an opened pull request for the same change
    When the pre-push hook and the PR quality gate run
    Then both run `nx affected -t test:quick` for the affected projects
    And neither runs test:integration or test:e2e
    And test:integration and test:e2e run only in the scheduled CRON pipelines
```

```gherkin
Feature: No quality gate runs heavy tests

  Scenario: None of the four gates runs test:integration or test:e2e
    Given the pre-commit, pre-push, PR quality, and post-merge main gates
    When any of them runs
    Then from pre-push onward it runs test:quick plus the governance/spec validators
    And pre-commit runs the fast file-type set only, without test:quick
    And none of the four gates runs test:integration or test:e2e

  Scenario: Post-merge main CI re-verifies all projects on the fast gate
    Given a PR is merged to main touching one or more projects
    When main-ci.yml runs
    Then it runs `nx run-many --all -t test:quick` plus the governance/spec validators across every project
    And it does not run test:integration or test:e2e
    And it does not deploy
```

```gherkin
Feature: Gate scope follows the affected-then-all rule

  Scenario: PR gate is the affected-scope union of pre-commit and pre-push
    Given the pre-commit and pre-push hooks and the PR quality gate
    When all three run for the same change
    Then every check that runs at pre-commit or pre-push also runs in the PR gate
    And all three operate on the affected project graph via `nx affected`

  Scenario: main gate widens the PR check set to all projects
    Given the PR quality gate runs the fast check set for affected projects
    When main-ci.yml runs post-merge
    Then it runs the same check set across every project via `nx run-many --all`
    And it runs no check that the PR gate does not also run
    And neither gate runs test:integration or test:e2e

  Scenario: Gates parallelize their independent checks
    Given any CI quality gate
    When it runs
    Then its independent checks run as parallel jobs
    And Nx fans test:quick across projects with `--parallel`
```

```gherkin
Feature: Heavy tests and deploy run only on the scheduled CRON pipeline

  Scenario: The scheduled pipeline runs the full suite then deploys
    Given the scheduled *-test-local-deploy-stag pipeline runs
    When it executes
    Then it runs test:quick, then test:integration, then test:e2e per app in isolation
    And on success it deploys that app to staging independently
    And a failing app never blocks another app's tests or deploy
    And the *-test-stag pipeline promotes a green staging deployment to production
```

```gherkin
Feature: Convergence introduces no regressions

  Scenario: Each repo stays green after convergence
    Given a repo that has been converged to the standard
    When its affected pre-push gate and PR quality-gate run on a no-op change
    Then all gates pass
    And no previously-passing gate is removed without an entry in the divergence policy
```

```gherkin
Feature: Legitimate divergence is preserved

  Scenario: Infra IaC gates and per-app deploy CRONs are retained
    Given the converged repos
    When the divergence policy is applied
    Then ose-infra retains its terraform, ansible, and yamllint gates
    And each repo retains only the per-app deploy CRON workflows for apps it actually ships
    And these differences are recorded in the divergence policy, not flagged as drift
```

## Product Risks

- **Risk: triage misclassifies a command** (e.g. a command wired only via an npm script, not a hook). Mitigation: each "wired" row must cite a concrete invocation site verified by grep; ambiguous cases (pre-commit auto-sync) are flagged `[Unverified]` for confirmation during Phase 1.
- **Risk: the "identical" standard is impossible for a gate that legitimately differs.** Mitigation: the divergence policy is authoritative; a gate that cannot be identical is moved to the allowed-divergence list rather than forced.
