# PRD — Standardize Repo Toolchain Parity (ose-public)

This Product Requirements Document specifies **what** gets built. The **why** lives in
[brd.md](./brd.md); the **how** lives in [tech-docs.md](./tech-docs.md). The Gherkin scenarios below
are the source of the first failing verification assertions in [delivery.md](./delivery.md).

## Product Overview

A set of verifiable changes across **six workstreams** that close ose-public's gaps against the fixed
**Converged Toolchain Target** shared with `ose-infra` and `ose-primer`, except the recorded per-repo
deviations. A/B/E/F are parallel-safe (no single anchor); C/D are reference-first (ose-public leads):

1. **A — CI**: per-language PR-gate jobs `nx run-many` → `nx affected`; canonical concurrency on
   every workflow; lint jobs renamed to `shellcheck`/`hadolint`/`actionlint`;
   `gherkin:keyword-cardinality-validation` target wired into `validate-markdown.yml`; the full
   quality gate also running on `push` to `main`; scheduler cadence aligned 2× WIB.
2. **B — Hooks**: `commit-msg`/`pre-commit`/`pre-push` converge to the canonical BLOCK 1-B lifecycle.
3. **C — rhino-cli architecture (REFERENCE)**: flat layout → hexagonal, behavior-frozen by a
   golden-master CLI suite.
4. **D — rhino-cli commands (REFERENCE for additions)**: add `Java` + `Contracts`.
5. **E — Target naming**: `{domain}:{work}` rename + `spec-coverage`→`spec:coverage` repo-wide.
6. **F — Governance**: update all related docs, run `repo-rules-maker`, then `repo-rules-quality-gate`
   until clean.
7. **G — Mermaid state-diagram validation (REFERENCE)**: add the `state.rs` front-end + width/label
   rules for state diagrams to the migrated Mermaid slice, land the shared golden corpus, and clean
   up every violating state diagram repo-wide. Depends on workstream C.

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
- **US-4 (A)** — As a CI maintainer, I want the Gherkin keyword-cardinality validator running in CI
  so that the rule is enforced evenly across the family.
- **US-5 (A)** — As a CI maintainer, I want the full quality gate to also run on `push` to `main` so
  that direct worktree-to-main pushes are gated identically to PRs.
- **US-6 (B)** — As a toolchain maintainer, I want the git hooks to match the canonical lifecycle so
  that the local pre-flight contract is identical across repos.
- **US-7 (C)** — As a toolchain maintainer, I want rhino-cli migrated to the hexagonal layout with
  its behavior frozen so that the CLI is testable and identical, and the siblings can port from it.
- **US-8 (D)** — As a toolchain maintainer, I want the `Java` and `Contracts` subcommands added so
  that the CLI is the union superset and drop-in across repos.
- **US-9 (E)** — As a toolchain maintainer, I want every governance target renamed to `{domain}:{work}`
  and `spec-coverage`→`spec:coverage` so that target names are canonical everywhere.
- **US-10 (F)** — As a governance maintainer, I want all related docs updated, propagated by
  `repo-rules-maker`, and passed through the `repo-rules-quality-gate` so that the docs never drift
  from the toolchain.
- **US-11** — As a release/deploy hat, I want Phase 0 to verify the bootstrap-be prerequisite landed
  so that I never standardize a toolchain whose .NET surface does not yet exist.
- **US-12 (G)** — As a documentation author, I want over-wide state diagrams and long state /
  transition labels flagged by `mermaid:validation`, so that my state diagrams stay readable on
  mobile just like my flowcharts, and the golden corpus locks identical behavior across the three
  repos.

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
  Given pr-quality-gate.yml currently runs nx run-many for the Go, .NET, and Rust jobs
  When the test-semantics convergence is applied
  Then each of the Go, .NET, and Rust jobs runs nx affected with the same target list
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
Scenario: Gherkin keyword-cardinality validator runs in CI under the canonical name
  Given the rhino-cli repo-governance gherkin-keyword-cardinality command already exists
  When a gherkin:keyword-cardinality-validation Nx target is created and wired into validate-markdown.yml
  Then validate-markdown.yml invokes nx run rhino-cli:gherkin:keyword-cardinality-validation
  And the target passes against the current repository tree
  And the target name already follows the canonical {domain}:{work} scheme
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
Scenario: rhino-cli exposes the union command superset
  Given rhino-cli is missing the Java and Contracts subcommands
  When the union-command port is applied
  Then rhino-cli --help lists the Java and Contracts subcommands
  And the command surface matches the union superset shared across the three repos
```

```gherkin
Scenario: Governance targets renamed to the canonical scheme
  Given the governance targets use ad-hoc validate/lint/fmt names and spec-coverage
  When the {domain}:{work} rename is applied repo-wide
  Then every governance, validation, lint, and check target uses the {domain}:{work} scheme
  And spec-coverage is renamed to spec:coverage in every project.json
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
  And the gherkin:keyword-cardinality-validation step is present and green
```

### Workstream G — Mermaid state-diagram validation acceptance criteria

> These scenarios (ported from the folded `mermaid-state-diagram-validation` plan) become the first
> failing tests in Phase 8. Each uses exactly one primary `Given`, one `When`, one `Then`; extras
> chain with `And`/`But`. The Phase 8 target name is still `validate:mermaid` (the rename to
> `mermaid:validation` is Phase 10); the underlying `docs validate-mermaid` CLI command is unchanged.

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

## Product Scope

### In Scope

- **A** — `pr-quality-gate.yml` (run-many→affected; concurrency; lint-job rename + `needs`;
  push-to-main full gate); `validate-markdown.yml` (gherkin target step; concurrency);
  `validate-env.yml` + `test-and-deploy-*.yml` (concurrency; scheduler cadence).
- **B** — `.husky/commit-msg`, `.husky/pre-commit`, `.husky/pre-push` converge to BLOCK 1-B.
- **C** — migrate `apps/rhino-cli/src/` to the hexagonal layout, golden-master-frozen.
- **D** — add `Java` + `Contracts` subcommands to rhino-cli.
- **E** — `{domain}:{work}` rename in `apps/rhino-cli/project.json`; `spec-coverage`→`spec:coverage`
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
