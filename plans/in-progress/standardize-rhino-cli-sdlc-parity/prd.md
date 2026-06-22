# PRD — Standardize rhino-cli Checks & SDLC Commands

## Product Overview

A documentation-plus-configuration product: one committed **command triage** doc, one committed
**SDLC standard** doc, and the convergence edits to three repos' hooks (`.husky/*`), workflows
(`.github/workflows/*`), and rhino-cli Nx targets (`apps/rhino-cli/project.json`) so the gate
mechanics match the standard.

This plan touches **no application or library runtime code** and **no validator logic**. It touches
shell hooks, YAML workflows, Nx target definitions, and reference docs. Where it edits an
`apps/rhino-cli` Nx target's wiring (not its Rust source), the change is a config edit verified by
running the target, not a TDD code cycle.

## Personas

- **Maya, the maintainer** — operates all three repos; wants one mental model and frictionless propagation.
- **Theo, the agent** (`ci-checker`) — needs a single canonical standard to validate each repo against.
- **Sam, the downstream consumer** — clones `ose-primer` as a template; inherits the coherent gate model.

## User Stories

- As Maya, I want every rhino-cli command labelled wired/not-wired so I know which gates are actually enforced.
- As Maya, I want the PR quality-gate workflow to have the same filename and job structure in all three repos.
- As Maya, I want the markdown / env validation workflows to run the same validator set everywhere.
- As Theo, I want a committed standard doc I can diff each repo against.
- As Sam, I want infra-only gates (terraform/ansible) to stay in infra and not leak into the template.

## Scope

### In Scope

- Triage table covering every leaf subcommand under the 11 rhino-cli families (TestCoverage, RepoGovernance, Md, Convention, Harness, Workflows, Specs, Lang, Git, Env, Doctor), each with a one-line description, wired/not-wired status, and invocation site. [Repo-grounded]
- Nx target-name standardization: canonical lifecycle + `{domain}:{work}` names for every hook/CI-invoked target, identical rhino-cli target sets across the three repos (`fmt`→`format`, `{tool}:check` wrappers, `harness:bindings-validation` as an Nx target). [Repo-grounded]
- Testing-architecture standard: mandatory-seven targets on every project (echo where N/A), `test:quick` = typecheck→lint→test:unit, three levels consuming the same Gherkin, BE service-level integration / FE-DB-only integration / `*-e2e`-only e2e, pre-push ≡ PR running only `test:quick`, and rhino-cli feature-consumption enforcement. [Repo-grounded]
- Standardized gate mechanics for: commit-msg, pre-commit, pre-push, PR quality-gate, markdown-validate, env-validate, and the CRON "test local + deploy stag" / "test stag + deploy prod" pipeline _shape_.
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
    And the gate's job skeleton (detect, format, language-gate, markdown, naming, env, specs-gate, quality-gate sentinel) is present in all three
    And only the language-specific gate jobs and infra-only IaC jobs differ between repos
```

```gherkin
Feature: Markdown and env validation run identical validator sets

  Scenario: The markdown validation workflow runs the same validators everywhere
    Given the standardized markdown-validation workflow
    When it runs in any of the three repos
    Then it invokes mermaid, links, heading-hierarchy, and gherkin-cardinality validation
    And the set is identical across all three repos
```

```gherkin
Feature: Hook ordering and invocation mechanism are identical

  Scenario: pre-commit and pre-push run the same steps in the same order
    Given the standardized .husky/pre-commit and .husky/pre-push hooks
    When a contributor inspects the hooks in any repo
    Then the ordered list of gate steps matches the standard
    And each shared gate is invoked through the same mechanism (Nx-wrapped rhino-cli target, not inline shell)
    And only infra-only IaC steps appear as documented additions in ose-infra
```

```gherkin
Feature: Nx target names are canonical and identical across repos

  Scenario: Hook/CI-invoked targets follow the canonical scheme
    Given the converged repos
    When the Nx targets invoked by any hook or CI workflow are inspected
    Then each target name comes from the canonical lifecycle or {domain}:{work} scheme in nx-targets.md
    And the format/write target is named `format` (not `fmt`) in all three
    And the shell/Dockerfile/Actions lint wrappers are named `shell:check`, `dockerfiles:check`, `actions:check` in all three
    And `harness:bindings-validation` is invoked as an Nx target, not an npm script, in all three

  Scenario: The rhino-cli target set is identical across repos
    Given the converged repos
    When `jq -r '.targets | keys[]' apps/rhino-cli/project.json` is run in each
    Then the sorted key set is identical across ose-public, ose-primer, and ose-infra
```

```gherkin
Feature: Every project declares the mandatory seven targets

  Scenario: All apps and libs expose the seven targets
    Given any direct child folder of apps/ or libs/ registered with Nx
    When its project.json targets are inspected
    Then test:unit, test:integration, test:e2e, test:quick, lint, format, and typecheck are all present
    And targets that do not apply to the project are declared as no-op echo placeholders
    And test:e2e has a real (non-echo) command only on *-e2e projects
```

```gherkin
Feature: test:quick runs typecheck then lint then test:unit

  Scenario: test:quick composes the three in order
    Given a project's test:quick target
    When test:quick runs
    Then it runs typecheck, then lint, then test:unit, in that exact order
    And it stops at the first failing step
```

```gherkin
Feature: The three test levels consume the same Gherkin

  Scenario: unit, integration, and e2e share feature files
    Given a project with feature files under its specs gherkin directory
    When test:unit, test:integration, and test:e2e run
    Then all three consume the same feature files driven by the same tags
    And test:unit covers BDD step tests plus non-BDD unit tests with coverage measured
    And BE test:integration exercises behaviour at the service level, never through the HTTP API
    And the HTTP API surface is exercised only by test:e2e in the *-e2e project
```

```gherkin
Feature: Every feature file is consumed by a test

  Scenario: An orphan feature file fails the gate
    Given a feature file under specs that no test references
    When rhino-cli specs:coverage runs with --require-consumption
    Then it fails and names the orphan feature file
```

```gherkin
Feature: Pre-push and PR gate run identical fast commands

  Scenario: Both gates run only test:quick for affected projects
    Given a push and an opened pull request for the same change
    When the pre-push hook and the PR quality gate run
    Then both run `nx affected -t test:quick` for the affected projects
    And neither runs test:integration or test:e2e
    And test:integration and test:e2e run only on the scheduled CRON pipelines
```

```gherkin
Feature: Post-merge CI runs full tests and deploys per project

  Scenario: A merged PR triggers per-project test then staging deploy
    Given a PR is merged to main touching one or more projects
    When the post-merge CI runs
    Then each affected project runs test:quick, then test:integration, then test:e2e in isolation
    And a project whose tests all pass and that is deployable is deployed to staging independently
    And a failing project never blocks another project's tests or deploy
    And the scheduled stag-deploy CRON remains only as a nightly fallback
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
