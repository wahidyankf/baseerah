# BRD — Standardize CI Parity (ose-public anchor)

This Business Requirements Document explains **why** the CI standardization exists. The **what**
(features, scope, acceptance criteria) lives in [prd.md](./prd.md); the **how** lives in
[tech-docs.md](./tech-docs.md).

## Business Goal

Make the GitHub Actions CI of `ose-public` and `ose-infra` **functionally identical except for the
runner target**, so that a contributor (or AI agent) who understands one repo's pipeline
understands both, and so that the downstream
[`ose-infra/plans/in-progress/deploy-twin-k3s-clusters/`](../../../docs/reference/related-repositories.md)
deployment runs against a converged, version-current, known-good pipeline.

## Business Rationale

CI drift between sibling repos is a slow, compounding tax:

- **Cognitive load** — every divergence (`run-many` here, `affected` there; `@v6` here, `@v4`
  there) is a thing a maintainer must hold in their head and re-learn per repo. This repo is
  solo-maintained, so that load lands on one person wearing every hat. [Judgment call]
- **Inconsistent gate strength** — ose-public's PR gate runs `nx run-many` for non-TS languages,
  testing the whole tagged project set regardless of what changed; ose-infra runs `nx affected`,
  testing only impacted projects. The two repos therefore enforce subtly different contracts on the
  same kind of change. Converging to `nx affected` everywhere gives both repos the same
  fast-feedback semantics. [Repo-grounded — `pr-quality-gate.yml:93,109,125`]
- **No concurrency cancellation in ose-public** — without a concurrency group, superseded pushes
  keep burning CI minutes; ose-infra already cancels in-progress runs. Adding the canonical
  concurrency block aligns behaviour and reduces wasted compute. [Repo-grounded — no
  `concurrency:` in any ose-public workflow]
- **Validator-set drift** — ose-public does not run the Gherkin keyword-cardinality validator in
  CI even though the `rhino-cli` command exists; ose-infra does. A rule enforced in one repo and
  not the other is a rule that quietly rots. [Repo-grounded — `rhino-cli repo-governance
  gherkin-keyword-cardinality` command exists; no `validate:gherkin-keyword-cardinality` Nx target]
- **Deployment risk** — the downstream twin-k3s deployment is built on the assumption that CI is
  trustworthy. Standardizing CI first de-risks that deployment. [Judgment call]

## Business Impact

### Pain Points Addressed

- A maintainer reading `ose-infra` CI cannot assume it matches `ose-public` CI, and vice versa.
- ose-public wastes CI minutes on superseded runs (no cancel-in-progress).
- ose-public's PR gate over-tests non-TS languages via `run-many`, slowing feedback on large
  pushes.
- A governance rule (Gherkin cardinality) is enforced unevenly across the family.

### Expected Benefits

- **One mental model** of CI across both repos (minus the runner line, which is intentionally
  different and documented).
- **Faster, cheaper PR feedback** in ose-public (affected-only non-TS jobs + cancel-in-progress).
- **Even governance enforcement** — the same validator set runs in both repos.
- **A converged baseline** the twin-k3s deployment can rely on.

## Affected Roles

This is a solo-maintainer repository — no sign-off ceremonies. The roles below are **hats the
maintainer wears** and **agents that consume the outputs**:

- **CI maintainer hat** — edits the workflows and the `ci-conventions.md` standard.
- **Release/deploy hat** — depends on the converged pipeline before running the downstream
  twin-k3s deployment.
- **`ci-checker` agent** — validates projects against `ci-conventions.md`; consumes the new CI
  Parity Checklist section and any added parity checks.
- **`ci-fixer` agent** — applies fixes flagged by `ci-checker`.
- **`plan-checker` / `plan-execution-checker` agents** — validate this plan and its execution.

## Business-Level Success Metrics

- **CI parity invariants documented and met** — every invariant in the new CI Parity Checklist
  section of `ci-conventions.md` is satisfied by ose-public's workflows. [Observable — grep/diff
  the workflows against the checklist]
- **Zero `nx run-many` in per-language PR-gate jobs** — `pr-quality-gate.yml` uses `nx affected`
  for Go, .NET, and Rust. [Observable — `grep -c "run-many" pr-quality-gate.yml` for the language
  jobs returns 0]
- **Concurrency present on all targeted workflows** — every workflow listed in scope carries a
  `concurrency:` block. [Observable — `grep -l concurrency:` count matches]
- **Gherkin cardinality validator runs in CI** — `validate-markdown.yml` invokes the new Nx
  target and it passes on the current tree. [Observable — CI run is green with the step present]
- **All CI checks green after push** — the standardized pipeline passes on `origin main`.
  [Observable — GitHub Actions status]

## Business-Scope Non-Goals

- This plan does **not** seek to converge the runner target — that divergence is an accepted,
  documented business decision (ephemeral no-infra-dependency runners for the public repo;
  warm self-hosted runners with Docker/IaC tooling for the infra repo).
- This plan does **not** introduce new CI capabilities, deploy targets, or Nx Cloud changes
  beyond what parity requires.
- This plan does **not** perform ose-infra's side of the work (version bumps, reusable-workflow
  adoption) — that is the sibling plan's responsibility, referenced not executed here.

## Business Risks and Mitigations

| Risk                                                                          | Likelihood | Impact | Mitigation                                                                                                   |
| ----------------------------------------------------------------------------- | ---------- | ------ | ------------------------------------------------------------------------------------------------------------ |
| Prerequisite (bootstrap-be-messaging) not actually done, .NET surface missing | Medium     | High   | Phase 0 gate hard-verifies `crane-be`, GHCR workflow, and `.NET` detection before any work begins            |
| `nx affected` misses a project the old `run-many` would have caught           | Low        | Medium | PR-only gate has `github.base_ref` always defined; inline `NX_BASE`/`NX_HEAD` already proven on the TS job   |
| New gherkin validator surfaces preexisting cardinality violations             | Medium     | Low    | Root-cause orientation — fix any flagged violations in-plan rather than disabling the validator              |
| Concurrency cancel-in-progress cancels a needed run                           | Low        | Low    | Canonical pattern only cancels on PR events, not on `push` to `main`, per GitHub-recommended group key       |
| Sibling plan diverges instead of converging                                   | Low        | Medium | Deviation matrix in tech-docs.md is the shared contract; both plans cite the same converged target end-state |
