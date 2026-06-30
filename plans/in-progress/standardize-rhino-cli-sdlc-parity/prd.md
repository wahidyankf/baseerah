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
- Nx target-name standardization: canonical lifecycle + `{domain}:{work}` names for every hook/CI-invoked target, identical rhino-cli target sets across the three repos (remove `fmt`/`format:check` → formatting via file-type lint-staged, shell/Dockerfile/workflow linting via lint-staged file-type entries (no `{tool}:lint` Nx targets), the binding validators (`harness bindings validate`/`generate`) + the env-guard run as direct `cargo run` calls (no `harness:bindings-validation`/`harness:bindings-generate` Nx targets), remove `test-coverage` → native `test:coverage`, `specs:coverage`→`specs:behavior:coverage` + new `specs:domain:coverage` gated by the explicit `specs.domain-areas` allowlist in `repo-config.yml` (not folder-presence, not the `*-be` suffix — explicit-config principle), rename `deny:check`→`deps:audit` and `msrv:check`→`compat:min-version` and make both cross-language present-on-every-project columns — `deps:audit` (CVE + license via each language's native tool) CRON-only because uncacheable, `compat:min-version` (min-version-floor build, real only on Rust + Python) in the gates because cacheable). [Repo-grounded]
- Single merged `repo-config.yml` (instruction-size + env-contract + env-injection sections); Codecov fully removed (native coverage only); every project covered by a standardized GitHub CI named per ose-public convention. [Repo-grounded]
- Testing-architecture standard: mandatory-six targets (no `format`) on every project (echo where N/A), `test:quick` = typecheck→lint→test:unit→test:coverage→test:specs (test:specs aggregates the `specs:*` validators; all composed targets present on every project, echo where N/A), three levels consuming the same Gherkin, unit and integration tests in separate folders (`tests/unit` vs `tests/integration`; Rust co-located `#[cfg(test)]` + external `tests/`), BE service-level integration / FE-DB-only integration / `*-e2e`-only e2e, pre-push/PR/main-ci running `test:quick` while pre-commit stays fast (format + tool-lint + guards, no `test:quick`), no gate running integration/e2e (CRON-only), and rhino-cli per-level `@covers` coverage enforcement (the §4.1 registry + self-tag + exact-level model). [Repo-grounded]
- rhino-cli surface rationalization: the command triage lists the **target** (end-state) command column before the current-form column, and carries a per-command keep/merge/drop/wire recommendation; **every** `harness` command (bindings, naming, instruction-size, duplication, audit) covers all 11 supported harnesses (source/generated/native tiers) via the `repo-config.yml` `harness:` registry — not just OpenCode + Amazon Q, and none hard-coded to a `.claude`/`.opencode` pair. [Judgment call]
- Standardized gate mechanics for: commit-msg, pre-commit, pre-push, PR quality-gate, main-ci, env-validate, and the CRON "test local + deploy stag" / "test stag + deploy prod" pipeline _shape_ (markdown validation folds into the gates — no standalone markdown workflow).
- Worktree-agnostic guardrail execution: every guardrail runs identically from the primary checkout and a linked worktree (`git rev-parse --show-toplevel` for the current tree root, `--git-common-dir` for shared metadata; never assume `.git/` is a directory or read config from the main checkout), locked by a rhino-cli regression test and verified per repo. [Repo-grounded + Judgment call]
- Convergence edits in all three repos.

### Out of Scope

- App-set / language-set unification.
- Validator logic changes.
- Net-new validator behavior — the §3.3 ratified command removals/merges/wirings are in scope and executed here, but no surviving validator's checks change (no new lint rule or threshold).
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
    And a check is wired into lint-staged only when it is file-type-based and per-file isolated; `test:quick` stays an Nx target (pre-push onward); harness binding validation runs via direct `cargo run --release -- harness bindings validate` at pre-push (no `harness:bindings-validation` Nx target); always-run writes (`harness:bindings-generate`, `env staged-guard validate`) also run as direct `cargo run` calls, the env guard kept as the dedicated first-line secrets gate
    And no `harness:bindings-validation` or `harness:bindings-generate` Nx target exists in any of the three repos — both binding commands invoke `cargo run` directly
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

  Scenario: Projects listed in specs.domain-areas declare specs:domain:coverage
    Given a project listed in the repo-config.yml specs.domain-areas allowlist
    When its project.json targets are inspected
    Then specs:domain:coverage is present
    And projects not listed in specs.domain-areas do not declare specs:domain:coverage
    And eligibility comes from the explicit allowlist, not folder-presence or the *-be name suffix
```

```gherkin
Feature: test:quick runs typecheck, lint, test:unit, test:coverage, then test:specs

  Scenario: test:quick composes the five in order
    Given a project's test:quick target
    When test:quick runs
    Then it runs typecheck, then lint, then test:unit, then test:coverage (≥90% line), then test:specs, in that exact order
    And it stops at the first failing step
    And test:specs aggregates the specs:* validators (structure-validation [merged adoption + tree + counts], behavior:coverage, and domain:coverage where the project is listed in specs.domain-areas; spec links are covered repo-wide by md links validate, not in test:specs)
    And the specs gate runs inside test:quick, so there is no separate specs-structural gate step
```

```gherkin
Feature: The three test levels consume the same Gherkin

  Scenario: unit, integration, and e2e share feature files
    Given an app project keyed to its surface folder specs/apps/<domain>/behavior/<surface>/gherkin (one domain tree per apps/<domain>-* family)
    Or a lib project keyed to its own specs/libs/<lib>/behavior/gherkin (per-project, identical structure)
    When test:unit, test:integration, and test:e2e run
    Then all three consume the same feature files driven by the same tags
    And every scenario is covered at exactly its required levels via explicit @covers markers (the §4.1 per-level model)
    And test:unit may additionally carry non-Gherkin unit tests for behaviour not expressed as scenarios
    And BE test:integration exercises behaviour at the service level, never through the HTTP API
    And the HTTP API surface is exercised only by test:e2e in the *-e2e project
```

```gherkin
Feature: The specs tree uses one identical C4 structure across all projects and repos

  Scenario: Every spec area carries the mandatory identical C4 tree with gherkin
    Given any spec area (an app domain under specs/apps/<domain> or a lib under specs/libs/<lib>)
    When specs:structure-validation runs
    Then the area must contain product, system-context, containers, components, and behavior/.../gherkin folders
    And every area carries gherkin (behavior is never empty)
    And the same structure is required identically in all three repos
    And a missing mandatory folder is a structure error

  Scenario: ddd/ is required only for the explicitly listed ddd-areas
    Given the repo-config.yml specs.ddd-areas allowlist
    When specs:structure-validation runs
    Then an area listed in ddd-areas must contain a ddd/ folder
    And an area NOT listed must not contain a ddd/ folder
    And the allowlist replaces the hardcoded apps_with_ddd() (explicit config, no name-suffix inference)
```

```gherkin
Feature: Dependency-audit and min-version targets are cross-language and correctly placed

  Scenario: deps:audit runs CRON-only because it is uncacheable
    Given deps:audit queries a live advisory database (cache: false) and is slow on JVM/Clojure
    When the gates run (pre-commit, pre-push, PR, main)
    Then none of them runs deps:audit
    And deps:audit runs only in the nightly deps-audit.yml CRON across all projects
    And every project, including *-e2e runners, has a real deps:audit via its language's native CVE+license tool

  Scenario: compat:min-version runs in the gates because it is cacheable
    Given compat:min-version is a deterministic min-version-floor build
    When pre-push, the PR gate, and main-ci run
    Then each runs compat:min-version for its scoped projects
    And it is real only on Rust (cargo hack) and Python (vermin) projects, echo elsewhere
```

```gherkin
Feature: Unit and integration tests live in separate folders

  Scenario: A project with both real suites keeps them in disjoint folders
    Given a project that declares both a real test:unit and a real test:integration
    When its test source layout is inspected
    Then the unit tests and integration tests live in physically separate directories
    And the test:unit and test:integration source globs share no path
    And for TS/F# projects the folders are tests/unit and tests/integration (or test/unit and test/integration)
    And for Rust projects unit tests are co-located in #[cfg(test)] while integration tests live in the external tests/ directory
```

```gherkin
Feature: rhino-cli triage reads target-first and carries merge/drop verdicts

  Scenario: The triage table puts the target command column before the current one
    Given the rhino-cli command triage table
    When its columns are inspected
    Then the "Command (leaf) — target" column appears before the "Command (leaf) — current" column
    And each row (or row group) carries a keep/merge/drop/wire recommendation in the merge/drop section
    And the recommendation collapses redundancy (e.g. harness claude/sync validate fold into harness bindings validate; specs bc/ul fold into specs structure validate; git pre-commit and test-coverage validate are dropped)
    And every verdict is ratified (Decided ✅ in the triage table) after one-by-one maintainer review
```

```gherkin
Feature: harness binding commands cover every supported harness

  Scenario: All 11 harnesses are accounted for at their tier
    Given the harness binding commands and the repo-config.yml harness section
    When the harness coverage is inspected
    Then all 11 supported harnesses are listed (Claude Code, OpenCode, Amazon Q, Codex, Copilot, Cursor, Windsurf, Junie, Antigravity, Pi, Aider)
    And the generated tier (OpenCode, Amazon Q) is regenerated and byte-parity-validated
    And the native tier (Copilot, Cursor, Windsurf, Junie, Antigravity, Pi, Aider) is validated by the no-shadowing rule plus the AGENTS.md instruction-size budget
    And the harness set is data in repo-config.yml, identical across all three repos, not a hard-coded directory list
```

```gherkin
Feature: Every scenario is covered at exactly its required levels (explicit @covers)

  Scenario: An untagged scenario fails the gate
    Given a scenario with no @unit, @integration, or @e2e level tag
    When rhino-cli specs behavior-coverage validate runs
    Then it fails and names the untagged scenario

  Scenario: A scenario requiring a level outside the project envelope fails
    Given a project whose coverage registry declares only the unit level
    And a scenario in that project tagged @integration
    When rhino-cli specs behavior-coverage validate runs
    Then it fails because the scenario requires a level not in the project envelope

  Scenario: A scenario not covered at a required level fails
    Given a scenario tagged @unit and @e2e
    And a test marks it @covers at the unit level only
    When rhino-cli specs behavior-coverage validate runs
    Then it fails and names the missing e2e coverage

  Scenario: An @covers at an undeclared level fails
    Given a scenario tagged @unit only
    And a test marks it @covers at the e2e level
    When rhino-cli specs behavior-coverage validate runs
    Then it fails because the e2e level is not declared for that scenario

  Scenario: An orphan @covers marker fails the gate
    Given a test with an @covers marker referencing a scenario title that no feature file contains
    When rhino-cli specs behavior-coverage validate runs
    Then it fails and names the orphan marker

  Scenario: A @wip scenario is exempt from coverage
    Given a scenario tagged @wip with no @covers markers
    When rhino-cli specs behavior-coverage validate runs
    Then it does not fail and reports the scenario in the exempt count
```

```gherkin
Feature: Domain scenarios are covered (specs:domain:coverage)

  Scenario: An uncovered domain scenario fails the gate
    Given a project listed in the repo-config.yml specs.domain-areas allowlist
    And a domain scenario under domain/** not covered at its required level by any @covers marker
    When rhino-cli specs domain-coverage validate runs
    Then it fails and names the uncovered domain scenario
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
    And the standalone yaml config files (instruction-size, env-contract, env-injection) are absent from repo root

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
Feature: No quality gate runs heavy or uncacheable checks

  Scenario: None of the four gates runs test:integration, test:e2e, or deps:audit
    Given the pre-commit, pre-push, PR quality, and post-merge main gates
    And the gates are cacheable-by-construction so the cache can be warmed before commit/push
    When any of them runs
    Then from pre-push onward it runs test:quick plus compat:min-version (cacheable) plus the governance/spec validators
    And pre-commit runs the fast file-type set only, without test:quick
    And none of the four gates runs test:integration, test:e2e, or deps:audit (all heavy or uncacheable)

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
Feature: Heavy and uncacheable checks plus deploy run only on the scheduled CRON pipelines

  Scenario: The scheduled pipeline runs the full suite then deploys
    Given the scheduled *-test-local-deploy-stag pipeline runs
    When it executes
    Then it runs test:quick, then test:integration, then test:e2e per app in isolation
    And on success it deploys that app to staging independently
    And a failing app never blocks another app's tests or deploy
    And the *-test-stag pipeline promotes a green staging deployment to production

  Scenario: The nightly deps-audit pipeline scans every project's dependencies
    Given the scheduled deps-audit.yml pipeline runs nightly
    When it executes
    Then it runs deps:audit across all projects (CVE + license) against the live advisory DBs
    And it fails on any CVE or license violation
    And it is the only place deps:audit ever runs (never in a gate, because it is uncacheable)
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

```gherkin
Feature: Guardrails run identically inside and outside a linked worktree

  Scenario: The full gate passes from a linked worktree
    Given a linked git worktree created under worktrees/
    When the pre-push gate command set and the rhino-cli guardrail commands run from that worktree
    Then every guardrail resolves the worktree's own toplevel via git rev-parse --show-toplevel
    And no guardrail assumes .git is a directory or reads config from the main checkout
    And every guardrail exits with the same result as when run from the primary checkout

  Scenario: A regression test locks worktree-safe execution
    Given a synthetic linked worktree in the rhino-cli test suite
    When a guardrail command runs inside it
    Then it succeeds, proving repo-root and metadata resolution are worktree-aware

  Scenario: A bare worktree-only repo runs the gate
    Given ose-infra, a bare repository worked only through linked worktrees
    When the guardrails run from a linked worktree (its sole execution context)
    Then they pass without a primary checkout present
```

## Product Risks

- **Risk: triage misclassifies a command** (e.g. a command wired only via an npm script, not a hook). Mitigation: each "wired" row must cite a concrete invocation site verified by grep; ambiguous cases (pre-commit auto-sync) are flagged `[Unverified]` for confirmation during Phase 1.
- **Risk: the "identical" standard is impossible for a gate that legitimately differs.** Mitigation: the divergence policy is authoritative; a gate that cannot be identical is moved to the allowed-divergence list rather than forced.
