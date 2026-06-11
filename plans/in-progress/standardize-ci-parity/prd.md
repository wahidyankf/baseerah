# PRD — Standardize CI Parity (ose-public anchor)

This Product Requirements Document specifies **what** gets built. The **why** lives in
[brd.md](./brd.md); the **how** lives in [tech-docs.md](./tech-docs.md). The Gherkin scenarios
below are the source of the first failing verification assertions in [delivery.md](./delivery.md).

## Product Overview

A set of focused, verifiable changes to ose-public's GitHub Actions CI and its CI governance doc
that bring the pipeline into parity with the `ose-infra` sibling (except the runner target):

1. PR-gate per-language jobs switch from `nx run-many` to `nx affected`.
2. A canonical concurrency block is added to the PR gate, validator workflows, and scheduled
   workflows.
3. A `validate:gherkin-keyword-cardinality` Nx target is created and wired into
   `validate-markdown.yml`.
4. `ci-conventions.md` is brought into sync with the converged standard and gains a **CI Parity
   Checklist** section; `ci-checker` gains parity checks if warranted.

## Personas

Solo-maintainer repo — these are hats the maintainer wears plus consuming agents:

- **CI maintainer** — wants both repos' pipelines to behave identically so they can reason about
  CI once.
- **Contributor / AI agent** — pushes changes and expects fast, affected-only feedback with
  superseded runs cancelled.
- **`ci-checker` agent** — validates projects against `ci-conventions.md`; needs the parity
  invariants written down to check them.
- **Release/deploy hat** — needs a converged baseline before the downstream twin-k3s deployment.

## User Stories

- **US-1** — As a CI maintainer, I want the non-TS PR-gate jobs to use `nx affected` so that
  ose-public matches ose-infra's affected-only semantics and gives faster feedback.
- **US-2** — As a contributor, I want superseded CI runs cancelled so that I do not wait on stale
  runs and CI minutes are not wasted.
- **US-3** — As a CI maintainer, I want the Gherkin keyword-cardinality validator running in
  ose-public CI so that the rule is enforced evenly across the repo family.
- **US-4** — As a `ci-checker` agent, I want a CI Parity Checklist in `ci-conventions.md` so that
  I can validate ose-public's workflows against the converged standard.
- **US-5** — As a release/deploy hat, I want Phase 0 to verify the bootstrap-be prerequisite
  landed so that I never standardize a CI whose .NET surface does not yet exist.

## Acceptance Criteria (Gherkin)

Each scenario uses exactly one primary `Given`, one `When`, one `Then`; extras chain with
`And`/`But`.

```gherkin
Scenario: Prerequisite verified before work begins
  Given the bootstrap-be-messaging-and-crane-media plan is expected to be done
  When Phase 0 runs its prerequisite verification gate
  Then apps/crane-be/ exists in the worktree
  And a GHCR image-publish workflow exists under .github/workflows/
  And pr-quality-gate.yml contains .NET (lang:fsharp/lang:csharp) language detection
```

```gherkin
Scenario: Non-TS PR-gate jobs use nx affected
  Given pr-quality-gate.yml currently runs nx run-many for the Go, .NET, and Rust jobs
  When the test-semantics convergence is applied
  Then each of the Go, .NET, and Rust jobs runs nx affected with the same target list
  And no per-language PR-gate job invokes nx run-many
  And the inline NX_BASE/NX_HEAD env vars remain set on each affected job
```

```gherkin
Scenario: TypeScript job semantics are preserved
  Given the TypeScript PR-gate job already uses nx affected
  When the convergence is applied to the other language jobs
  Then the TypeScript job continues to use nx affected unchanged
  And the specs-gate rhino-cli single-project run-many is left intact
```

```gherkin
Scenario: Canonical concurrency block added to targeted workflows
  Given ose-public workflows currently declare no concurrency group
  When the concurrency block is added to the PR gate, validator, and scheduled workflows
  Then each targeted workflow declares a concurrency group keyed by workflow and PR number or ref
  And cancel-in-progress is true only for pull_request events
```

```gherkin
Scenario: Gherkin keyword-cardinality validator runs in CI
  Given the rhino-cli repo-governance gherkin-keyword-cardinality command already exists
  When a validate:gherkin-keyword-cardinality Nx target is created and wired into validate-markdown.yml
  Then validate-markdown.yml invokes nx run rhino-cli:validate:gherkin-keyword-cardinality
  And the target passes against the current repository tree
  And the validator set in ose-public matches the validator set in ose-infra
```

```gherkin
Scenario: ci-conventions.md reflects the converged standard
  Given ci-conventions.md has drifted from the converged CI standard
  When the governance alignment is applied
  Then ci-conventions.md describes nx affected as the PR-gate per-language standard
  And ci-conventions.md documents the canonical concurrency pattern
  And a CI Parity Checklist section enumerates the parity invariants with the runner deviation recorded
```

```gherkin
Scenario: Full pipeline green after push
  Given all phase changes are committed and pushed to origin main
  When GitHub Actions runs the standardized workflows
  Then all CI checks pass with zero failures
  And the new gherkin-keyword-cardinality step is present and green
```

## Product Scope

### In Scope

- Editing `pr-quality-gate.yml` (run-many → affected on Go/.NET/Rust; concurrency).
- Editing `validate-markdown.yml` (add gherkin validator step; concurrency).
- Editing `validate-env.yml` and `test-and-deploy-*.yml` (concurrency).
- Creating the `validate:gherkin-keyword-cardinality` Nx target in `apps/rhino-cli/project.json`.
- Editing `ci-conventions.md` (converged standard + CI Parity Checklist).
- Editing `.claude/agents/ci-checker.md` if parity checks are warranted.

### Out of Scope

- Converging the runner target (recorded deviation).
- ose-infra's version bumps / reusable-workflow adoption / its `nx affected` migration.
- Adding a .NET surface to ose-infra.
- New CI capabilities, deploy targets, or Nx Cloud changes beyond parity.

## Product-Level Risks

- **`nx affected` PR-only correctness** — relies on `github.base_ref` being defined on PR events;
  it always is, and the inline `NX_BASE`/`NX_HEAD` mechanism is already proven on the TS job.
- **Preexisting Gherkin cardinality violations** — adding the validator may surface existing
  violations; these are fixed in-plan (root-cause orientation), not waived.
- **Concurrency over-cancellation** — mitigated by the canonical group key that only cancels on
  PR events.
