# PRD — Standardize Repo Toolchain Parity (ose-public)

This Product Requirements Document specifies **what** gets built. The **why** lives in
[brd.md](./brd.md); the **how** lives in [tech-docs.md](./tech-docs.md). The Gherkin scenarios below
are the source of the first failing verification assertions in [delivery.md](./delivery.md).

## Product Overview

A set of verifiable changes across **seven workstreams (A–G)** that close ose-public's gaps against the fixed
**Converged Toolchain Target** shared with `ose-infra` and `ose-primer`, except the recorded per-repo
deviations. A/B/E/F are parallel-safe (no single anchor); C/D are reference-first (ose-public leads):

1. **A — CI**: per-language PR-gate jobs `nx run-many` → `nx affected`; canonical concurrency on
   every workflow; lint jobs renamed to `shellcheck`/`hadolint`/`actionlint`;
   `specs:gherkin-cardinality-validation` target wired into the `specs-gate` job; the full
   quality gate also running on `push` to `main`; scheduler cadence aligned 2× WIB.
2. **B — Hooks**: `commit-msg`/`pre-commit`/`pre-push` converge to the canonical BLOCK 1-B lifecycle.
3. **C — rhino-cli architecture (REFERENCE)**: flat layout → hexagonal, behavior-frozen by a
   golden-master CLI suite.
4. **D — rhino-cli commands (REFERENCE for additions)**: rationalize + scope-based regroup
   (`docs`→`md`, `agents`→`harness`, `java`→`lang`; fold `spec-coverage`/`ddd`/`contracts`/`gherkin`→
   `specs`; new `convention`; `docs` reserved) + uniform grammar; port JVM/contract cmds → `lang` +
   `specs`.
5. **E — Target naming**: `{domain}:{work}` rename + `spec-coverage`→`specs:coverage` repo-wide.
6. **F — Governance**: update all related docs, run `repo-rules-maker`, then `repo-rules-quality-gate`
   until clean.
7. **G — Mermaid state-diagram validation (REFERENCE)**: add the `state.rs` front-end + width/label
   rules for state diagrams to the migrated Mermaid slice, land the shared golden corpus, and clean
   up every violating state diagram repo-wide. Depends on workstream C.
8. **H — Test Lifecycle Architecture**: three-level testing (unit/integration/e2e) sharing the same
   `.feature` files; `test:unit` (mocked) at pre-commit via `test:quick`; `specs:coverage` +
   `test-coverage` at pre-push; **`test:integration`+`test:e2e` CRON-only** per app-group (2× WIB
   public+infra, 1×/day primer); `specs:coverage` enforces all scenarios across all three levels;
   heavy-test workflows (`test-and-deploy-{app-group}-development.yml` + `test-{app-group}-staging.yml`,
   primer no staging); prod deploy manual.

## Personas

Solo-maintainer repo — hats the maintainer wears plus consuming agents:

- **CI maintainer** — wants all three repos' pipelines to behave identically.
- **Toolchain/CLI maintainer** — wants a testable, identical rhino-cli across repos.
- **Contributor / AI agent** — pushes changes and expects fast, affected-only feedback with
  superseded runs cancelled and direct main pushes fully gated.
- **`ci-checker` / `repo-rules-*` agents** — validate the toolchain and propagate/gate the docs.
- **Release/deploy hat** — needs a converged baseline before the downstream twin-k3s deployment.

## User Stories

- **US-1 (A)** — As a CI maintainer, I want the non-TS PR-gate jobs to use `nx affected` so that
  ose-public matches the siblings' affected-only semantics.
- **US-2 (A)** — As a contributor, I want superseded CI runs cancelled so that I do not wait on stale
  runs and CI minutes are not wasted.
- **US-3 (A)** — As a CI maintainer, I want the lint-gate jobs renamed to the tool-named scheme so
  that the lint job graph reads identically to the siblings'.
- **US-3b (A)** — As a CI maintainer, I want Go stripped from ose-public's CI matrix, doctor scope, and
  AGENTS.md so that the toolchain matches the repo's real (Go-free) portfolio.
- **US-3c (A)** — As a CI maintainer, I want every workflow file name, `name:` field, and job id on the
  canonical BLOCK 1-A scheme so that the workflow graph reads identically across repos — with the
  `Quality gate` required-check name kept (any required-check rename paired with a branch-protection
  update).
- **US-4 (A)** — As a CI maintainer, I want the Gherkin keyword-cardinality validator running in CI
  so that the rule is enforced evenly across the family.
- **US-5 (A)** — As a CI maintainer, I want the full quality gate to also run on `push` to `main` so
  that direct worktree-to-main pushes are gated identically to PRs.
- **US-6 (B)** — As a toolchain maintainer, I want the git hooks to match the canonical lifecycle so
  that the local pre-flight contract is identical across repos.
- **US-7 (C)** — As a toolchain maintainer, I want rhino-cli migrated to the hexagonal layout with
  its behavior frozen so that the CLI is testable and identical, and the siblings can port from it.
- **US-8 (D)** — As a toolchain maintainer, I want the rhino-cli commands **regrouped by scope** (group
  = its operation target) under a **uniform grammar**, and the JVM/contract commands ported (→ `lang`
  - `specs`), so that the CLI is the regrouped union superset and drop-in across repos.
- **US-9 (E)** — As a toolchain maintainer, I want every governance target renamed to `{domain}:{work}`
  and `spec-coverage`→`specs:coverage` so that target names are canonical everywhere.
- **US-10 (F)** — As a governance maintainer, I want all related docs updated, propagated by
  `repo-rules-maker`, and passed through the `repo-rules-quality-gate` so that the docs never drift
  from the toolchain.
- **US-11** — As a release/deploy hat, I want Phase 0 to verify the bootstrap-be prerequisite landed
  so that I never standardize a toolchain whose .NET surface does not yet exist.
- **US-12 (G)** — As a documentation author, I want over-wide state diagrams and long state /
  transition labels flagged by `mermaid:validation`, so that my state diagrams stay readable on
  mobile just like my flowcharts, and the golden corpus locks identical behavior across the three
  repos.
- **US-13 (H)** — As a toolchain maintainer, I want all three test levels (`test:unit`,
  `test:integration`, `test:e2e`) to share the same `.feature` files and run only through Nx project
  commands, with `test:integration`/`test:e2e` confined to the per-app-group CRON heavy-test
  workflows, so that the same behavior is verified at three fidelities without slowing the pre-merge
  loop, and `specs:coverage` fails if any scenario is unimplemented at any level.
- **US-14 (H)** — As a toolchain maintainer, I want **every** project to declare **every** lifecycle
  target (with `echo` stubs where a level doesn't apply), so that `nx affected -t <target>` and
  `nx run-many -t <target>` sweep the whole graph without a missing-target failure.

## Acceptance Criteria (Gherkin)

Each scenario uses exactly one primary `Given`, one `When`, one `Then`; extras chain with
`And`/`But`.

```gherkin
Scenario: Prerequisite verified and golden-master captured before work begins
  Given the bootstrap-be-messaging-and-crane-media plan is expected to be done
  When Phase 0 runs its prerequisite and baseline gate
  Then apps/crane-be/ exists in the worktree
  And a GHCR image-publish workflow exists under .github/workflows/
  And pr-quality-gate.yml contains .NET (lang:fsharp/lang:csharp) language detection
  And a golden-master CLI corpus for every rhino-cli subcommand is recorded
```

```gherkin
Scenario: Non-TS PR-gate jobs use nx affected
  Given pr-quality-gate.yml currently runs nx run-many for the .NET and Rust jobs
  When the test-semantics convergence is applied
  Then each of the .NET and Rust jobs runs nx affected with the same target list
  And no per-language PR-gate job invokes nx run-many
  And the inline NX_BASE/NX_HEAD env vars remain set on each affected job
  But the single-project specs-gate run-many is left intact
```

```gherkin
Scenario: Canonical concurrency block added to every workflow
  Given ose-public workflows currently declare no concurrency group
  When the concurrency block is added to the PR gate, validator, and scheduled workflows
  Then each targeted workflow declares a concurrency group keyed by workflow and PR number or ref
  And cancel-in-progress is true only for pull_request events
```

```gherkin
Scenario: Lint-gate jobs renamed to the tool-named scheme
  Given pr-quality-gate.yml declares the lint jobs shell, dockerfile, and actions
  When the lint-gate job rename is applied
  Then the three jobs are named shellcheck, hadolint, and actionlint
  And quality-gate.needs references the new tool-named jobs and not the old category names
  And the CI job column of cross-language-lint-strictness.md uses the tool-named jobs
  But the linters, thresholds, and file sets are unchanged
```

```gherkin
Scenario: Go is stripped from ose-public CI and the workflow naming is canonical
  Given pr-quality-gate.yml carries a golang job and ad-hoc workflow file or job names
  When the Go-strip and BLOCK 1-A naming convergence is applied
  Then pr-quality-gate.yml contains no golang job, setup-golang step, or has-golang detection
  And every workflow file is kebab-case with a Title-Case name and kebab-case job ids
  And the Quality gate required-check job name is unchanged
  But infra and primer keep their Go language rows untouched
```

```gherkin
Scenario: Gherkin cardinality validator runs in CI under the canonical name
  Given the rhino-cli gherkin keyword-cardinality command already exists
  When a specs:gherkin-cardinality-validation Nx target is created and wired into the specs-gate job
  Then the specs-gate job invokes nx run rhino-cli:specs:gherkin-cardinality-validation
  And the target passes against the current repository tree
  And the target name already follows the canonical specs {domain}:{work} scheme
```

```gherkin
Scenario: Full quality gate runs on push to main
  Given pr-quality-gate.yml triggers on pull_request only today
  When the push-to-main full gate is added
  Then the full quality gate runs on push to the main branch
  And a direct worktree-to-main push is gated identically to a pull request
```

```gherkin
Scenario: Git hooks converge to the canonical lifecycle
  Given the ose-public hooks differ from the BLOCK 1-B canonical lifecycle
  When the hook convergence is applied
  Then commit-msg, pre-commit, and pre-push match the canonical lifecycle
  And every target the hooks invoke exists under its canonical name
```

```gherkin
Scenario: rhino-cli migrates to hexagonal architecture with behavior frozen
  Given rhino-cli uses a flat src/commands and src/internal layout
  When the hexagonal migration is applied feature by feature
  Then rhino-cli has src/domain, src/application, src/infrastructure, and src/commands layers
  And the golden-master CLI corpus is byte-identical to the Phase 0 baseline
  And ose-public stands as the reference layout the siblings port from
```

```gherkin
Scenario: rhino-cli commands are regrouped by scope and renamed to the uniform grammar
  Given rhino-cli subcommands use hyphenated forms in old groups like docs validate-mermaid and agents emit-bindings
  When the Phase 9a regroup and Phase 9b uniform rename are applied
  Then every subcommand reads uniform in its new group like md validate mermaid and harness emit amazonq
  And no caller (project.json, hooks, package.json, docs) invokes an old hyphenated or old-group subcommand
  And the golden-master corpus is re-captured for the regrouped surface
  But env init/backup/restore/validate and git pre-commit stay unchanged
```

```gherkin
Scenario: rhino-cli exposes the regrouped union command superset
  Given rhino-cli is missing the JVM and contract codegen commands
  When the union-command port into the lang and specs groups is applied
  Then rhino-cli lang java lists validate null-safety-annotations and specs lists clean and scaffold
  And the command surface matches the regrouped union superset shared across the three repos
```

```gherkin
Scenario: Governance targets renamed to the canonical scheme
  Given the governance targets use ad-hoc validate/lint/fmt names and spec-coverage
  When the {domain}:{work} rename is applied repo-wide
  Then every governance, validation, lint, and check target uses the {domain}:{work} scheme
  And spec-coverage is renamed to specs:coverage in every project.json
  And every caller (hooks, workflows, package.json) references only the new names
```

```gherkin
Scenario: Governance docs pass the repo-rules quality gate
  Given the related docs have been updated for the converged toolchain
  When repo-rules-maker propagates the changes and the repo-rules-quality-gate workflow runs
  Then all related docs reflect the converged toolchain
  And the repo-rules-quality-gate workflow reports clean before the plan is marked done
```

```gherkin
Scenario: Full toolchain green after push
  Given all phase changes are committed and pushed to origin main
  When GitHub Actions runs the standardized workflows
  Then all CI checks pass with zero failures
  And the renamed shellcheck, hadolint, and actionlint jobs ran and are green
  And the specs:gherkin-cardinality-validation step is present and green
```

### Workstream G — Mermaid state-diagram validation acceptance criteria

> These scenarios (ported from the folded `mermaid-state-diagram-validation` plan) become the first
> failing tests in Phase 8. Each uses exactly one primary `Given`, one `When`, one `Then`; extras
> chain with `And`/`But`. The Phase 8 target name is still `validate:mermaid` (the rename to
> `mermaid:validation` is Phase 10); the underlying `docs validate-mermaid` CLI command is unchanged at
> Phase 8 (Phase 9 later regroups it to `md validate mermaid`).

```gherkin
Feature: State diagram width validation

  Background:
    Given the validator default options use max_width 4 and max_label_len 30
    And state diagrams are in scope of validate-mermaid

  Scenario: Over-wide LR state chain is flagged width_exceeded
    Given a stateDiagram-v2 with "direction LR" and 11 sequential states
    When validate-mermaid parses the block
    Then a "width_exceeded" violation is reported for that block
    And the reported width is 11

  Scenario: Compliant narrow state chain passes
    Given a stateDiagram-v2 with "direction TB" and 3 sequential states
    When validate-mermaid parses the block
    Then no "width_exceeded" violation is reported for that block
```

```gherkin
Feature: State diagram label validation

  Background:
    Given the validator default options use max_label_len 30

  Scenario: A state display label over 30 characters is flagged
    Given a state declared as 'state "this label is far longer than thirty chars" as X'
    When validate-mermaid checks the state display label
    Then a "label_too_long" violation is reported for state X

  Scenario: A transition-edge label over 30 characters is flagged
    Given a transition "A --> B : this transition label exceeds thirty characters"
    When validate-mermaid checks the transition-edge label
    Then a "label_too_long" violation is reported for that edge

  Scenario: A short colon label passes
    Given a state declared as "Pending : awaiting input"
    When validate-mermaid checks the state display label
    Then no "label_too_long" violation is reported for that state
```

```gherkin
Feature: State diagram structure-to-node mapping

  Scenario: Pseudostates and stereotype states count as nodes
    Given a stateDiagram-v2 whose widest rank holds "[*]", a "<<choice>>" state, a "<<fork>>" state, and a "<<join>>" state plus one more
    When validate-mermaid computes rank width
    Then "[*]" and the stereotype states each count toward the rank width
    And a "width_exceeded" violation is reported because the rank holds 5 nodes

  Scenario: Composite state is treated as a subgraph
    Given a stateDiagram-v2 containing a composite "state Outer { Inner1 --> Inner2 }"
    When validate-mermaid parses the block
    Then the composite "Outer" is recorded as a subgraph
    And the subgraph-density warning applies to its inner contents
```

```gherkin
Feature: State diagram free text is not misparsed

  Scenario: Notes, comments and concurrency separators are skipped
    Given a stateDiagram-v2 containing a "note right of X ... end note", a "%% comment", and a "--" concurrency separator
    When validate-mermaid parses the block
    Then the note text is exempt from the label rule
    And the "%%" comment line produces no node
    But the "--" separator produces neither a node nor a transition
```

```gherkin
Feature: Flowchart behavior is preserved

  Scenario: Existing flowchart validation is unchanged
    Given the pre-existing flowchart unit test suite
    When the Mermaid slice gains the state front-end
    Then every pre-existing flowchart test still passes
    And no flowchart violation codes change
```

```gherkin
Feature: Legacy v1 state diagram header is recognized

  Scenario: stateDiagram v1 header is in scope
    Given a legacy "stateDiagram" (v1) block of 11 sequential states with "direction LR"
    When validate-mermaid parses the block
    Then a "width_exceeded" violation is reported
    But the "TD" direction value is rejected as invalid for state diagrams
```

### Workstream H — Test Lifecycle Architecture acceptance criteria

```gherkin
Scenario: The three test levels share one set of feature specs
  Given an app has Gherkin .feature files under specs
  When test:unit, test:integration, and test:e2e run for that app
  Then all three levels execute the same .feature scenarios
  And test:unit uses mocks while test:integration uses same-container deps and test:e2e may use any deps
  And specs:coverage fails if any scenario is unimplemented in any of the three levels
```

```gherkin
Scenario: Heavy tests run only from CRON
  Given test:integration and test:e2e are heavy
  When the pre-commit, pre-push, PR gate, and push-to-main stages run
  Then none of those stages invoke test:integration or test:e2e
  And the heavy tests run only from the scheduled per-app-group workflows
  But pre-commit still runs test:unit via test:quick
```

```gherkin
Scenario: Heavy-test workflows exist per app-group with the right cadence
  Given each app-group is a deployable family from the Nx project graph
  When the heavy-test workflows are created
  Then test-and-deploy-{app-group}-development.yml runs integration and e2e and builds the staging container
  And test-{app-group}-staging.yml runs the same tests against the staging URL
  And the cadence is 2x WIB for ose-public and ose-infra and 1x per day for ose-primer
  But ose-primer builds no staging container and runs no staging test
```

## Product Scope

### In Scope

- **A** — `pr-quality-gate.yml` (run-many→affected; **strip Go** — `golang` job + `setup-golang` +
  `has-golang` detection removed; concurrency; lint-job rename + `needs`; push-to-main full gate);
  workflow file/`name:`/job-id naming onto the BLOCK 1-A scheme across all workflows (`Quality gate`
  kept); `rhino-cli doctor` Go-scope drop (ose-public); `pr-quality-gate.yml` `specs-gate` job gains
  the `specs:gherkin-cardinality-validation` run; `validate-markdown.yml`,
  `validate-env.yml` + `test-and-deploy-*.yml` (concurrency; scheduler cadence).
  ose-public **keeps** `publish-images.yml` → GHCR (recorded deviation; ose-primer carries none).
- **B** — `.husky/commit-msg`, `.husky/pre-commit`, `.husky/pre-push` converge to BLOCK 1-B.
- **C** — migrate `apps/rhino-cli/src/` to the hexagonal layout, golden-master-frozen.
- **D** — scope-based regroup (`docs`→`md`, `agents`→`harness`, `java`→`lang`; fold `spec-coverage`/
  `ddd`/`contracts`/`gherkin`→`specs`; new `convention`; `docs` reserved) + uniform grammar; port
  JVM/contract commands → `lang java validate null-safety-annotations`, `specs clean java-imports`/
  `specs scaffold dart`.
- **E** — `{domain}:{work}` rename in `apps/rhino-cli/project.json`; `spec-coverage`→`specs:coverage`
  in every app/lib `project.json`; update all callers.
- **F** — update all BLOCK 6 governance docs; run `repo-rules-maker`; run `repo-rules-quality-gate`.
- **G** — add the `state.rs` front-end to the migrated Mermaid slice (`stateDiagram-v2` +
  `stateDiagram` v1); width rule + label rule (state display labels AND transition labels); `[*]` /
  stereotype counting; composite-as-subgraph; `direction` ∈ `TB|BT|LR|RL`; land the shared golden
  corpus; aggressive repo-wide state-diagram cleanup (incl. `plans/done/`); document the rule in
  `diagrams.md` + `markdown.md`/`repository-validation.md`.
- `.claude/agents/ci-checker.md` / `repo-rules-*` edits if warranted.

### Out of Scope

- Converging the runner target (recorded deviation).
- The siblings' own A/B/E/F gaps; the siblings' C/D **port** of ose-public's reference (their repos).
- Adding a JVM/.NET surface to ose-infra.
- New toolchain capabilities, deploy targets, or Nx Cloud changes beyond parity.

## Product-Level Risks

- **Hexagonal behavior drift** — mitigated by the golden-master CLI suite that byte-freezes the
  output surface at every feature group and phase gate.
- **Target-rename caller breakage** — mitigated by the Phase 10 caller-sweep and the Phase 6/10
  sequencing so hooks never reference an unrenamed target between phases.
- **`nx affected` PR-only correctness** — relies on `github.base_ref` (always defined on PR events);
  the push-to-main full gate covers the merge-time picture.
- **Preexisting Gherkin cardinality violations** — fixed in-plan (root-cause orientation), not waived.
- **Concurrency over-cancellation** — mitigated by the canonical group key that only cancels on PR
  events.
- **State arrow vs concurrency separator (G)** — `-->` could be matched after the `--` concurrency
  separator, mis-classifying transitions; mitigated by matching `-->` BEFORE `--` (pinned grammar
  fact) with a golden fixture covering a `--` separator inside a composite.
- **State note free-text misparse (G)** — a note's free text could be parsed as a state, producing a
  false `label_too_long`; mitigated by a fixture with a long multiline note that must produce zero
  violations.
- **Branch-protection required-check rename (A)** — renaming a required-check job (e.g. `Quality gate`)
  silently breaks the merge gate because GitHub keys required checks by job name; mitigated by keeping
  `Quality gate` unchanged and pairing any required-check rename with a `[HUMAN]` branch-protection
  settings update (Phase 1) before the gate is relied upon.
