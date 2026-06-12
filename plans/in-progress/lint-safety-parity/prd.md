# Product Requirements — lint-safety-parity (ose-public)

## Product Overview

This plan delivers, for `ose-public`, the repo-specific slice of a three-repo lint/safety parity
effort. The "product" is a set of strict lint gates and a cleaned config surface:

1. **F# strict stack (D2)** across all 8 `.fsproj` files — `TreatWarningsAsErrors`,
   version-pinned G-Research.FSharp.Analyzers, and the `fantomas --check` format gate.
2. **Dockerfile lint (D6)** via hadolint.
3. **Shell lint (D7)** via shellcheck.
4. **GitHub Actions lint (D8)** via actionlint.
5. **Dead Go config removal (D10)** — delete root `.golangci.yml`.
6. **Documentation**: a plain-language rationale doc + governance/convention updates.

Each gate is wired into BOTH CI (`pr-quality-gate.yml`) and the local hooks
(`.husky/pre-commit` / `.husky/pre-push`), at the warning-and-above error threshold.

## Personas

Solo-maintainer repository — personas are hats the maintainer wears plus the agents that consume
the plan.

- **Platform Owner** (maintainer): wants a single honest strictness bar across all three siblings.
- **F# Developer** (maintainer / `swe-fsharp-dev`): cleans latent warnings, flips TWAE on.
- **CI Engineer** (maintainer / `ci-checker`): wires gates into CI + hooks consistently.
- **Governance Author** (maintainer / `repo-rules-maker`): writes the rationale + convention docs.
- **Plan Validators** (`plan-checker`, `plan-execution-checker`): validate authoring + execution.

## User Stories

- **US-1**: As the Platform Owner, I want `ose-public`'s F# stack to treat warnings as errors and
  run the G-Research analyzers, so that F# quality cannot erode silently and matches the primer bar.
- **US-2**: As the CI Engineer, I want a hadolint gate on every `Dockerfile`, so that broken or
  insecure container builds are caught before merge.
- **US-3**: As the CI Engineer, I want a shellcheck gate on every `.sh` script, so that shell
  defects (unquoted vars, etc.) are caught before merge.
- **US-4**: As the CI Engineer, I want an actionlint gate on every workflow YAML, so that invalid
  workflow expressions and syntax are caught before merge.
- **US-5**: As the Platform Owner, I want the dead root `.golangci.yml` removed, so the repo's
  config surface honestly reflects that no active Go exists.
- **US-6**: As any contributor, I want a plain-language rationale doc explaining every decision
  (including the D5 deferral and the DDD exemption philosophy), so the standard is understandable.
- **US-7**: As a Governance Author, I want the convention/AGENTS.md Quality-Gates surfaces updated
  to reflect the new gates, so governance docs stay truthful.

## Acceptance Criteria (Gherkin)

> Step-keyword cardinality: exactly one primary `Given`/`When`/`Then` per scenario; extras chained
> with `And`/`But`.

```gherkin
Scenario: F# strict stack flips on with a clean build
  Given all 8 .fsproj files have had their latent warnings cleaned
  When TreatWarningsAsErrors and the pinned G-Research analyzers are enabled and the F# build runs
  Then the build completes with zero warnings-as-errors failures
  And the change is safe to push to main without breaking the first gated build
```

```gherkin
Scenario: hadolint gate fails on an existing Dockerfile violation
  Given no Dockerfile lint gate exists yet and a Dockerfile contains a warning-level violation
  When hadolint runs with failure-threshold warning across all Dockerfiles
  Then hadolint exits non-zero reporting the violation
  And the violation is recorded as a cleanup item before the gate is wired on
```

```gherkin
Scenario: shellcheck gate passes after cleanup
  Given every tracked .sh script has been cleaned to satisfy shellcheck severity warning
  When shellcheck --severity=warning runs across all shell scripts
  Then shellcheck exits zero
  And the gate is wired into CI and the local hooks
```

```gherkin
Scenario: actionlint gate validates all workflows
  Given an actionlint gate is wired into CI and the pre-push hook
  When actionlint runs across .github/workflows/*.yml
  Then actionlint exits zero on the cleaned workflows
  And any pre-existing workflow finding was cleaned before the gate flipped on
```

```gherkin
Scenario: dead golangci config is removed
  Given the repo has no active Go module and root .golangci.yml is dead config
  When the .golangci.yml file is deleted and references are checked
  Then the file no longer exists in the repo
  And no workflow or script references it
```

```gherkin
Scenario: rationale doc explains every decision
  Given the lint-safety-parity decisions are finalized
  When docs/explanation/lint-safety-parity-decisions.md is written
  Then it explains each ose-public dimension plus the D5 deferral and DDD exemption philosophy
  And it cross-links the two sibling plans
```

```gherkin
Scenario: governance surfaces reflect the new gates
  Given the new D6/D7/D8 gates are defined
  When the cross-language strictness convention and AGENTS.md Quality-Gates lists are updated
  Then the governance docs name hadolint, shellcheck, and actionlint as active gates
  And the markdown link and naming validators pass on the edited docs
```

```gherkin
Scenario: plan passes the plan-quality-gate in strict mode
  Given the five-document plan is authored
  When plan-checker runs in strict double-zero mode
  Then it reports zero CRITICAL findings
  And it reports zero HIGH findings
```

## Product Scope

### In scope

- F# TWAE + pinned G-Research analyzers + fantomas-check gate across 8 `.fsproj` (D2).
- hadolint gate + `.hadolint.yaml` (D6).
- shellcheck gate + `.shellcheckrc` (D7).
- actionlint gate + optional `.github/actionlint.yaml` (D8).
- Remove root `.golangci.yml` (D10).
- Wire all new gates into `pr-quality-gate.yml` + `.husky/pre-commit` / `.husky/pre-push`.
- Rationale doc + governance/convention/AGENTS.md updates.

### Out of scope

- D1/D1b Rust work (ose-public is already the reference standard — documented, not changed).
- D3 C#, D4 Python (absent in ose-public).
- D5 TS DDD import-boundaries (dropped; deferral documented only).
- D9 Terraform/Ansible/YAML (no IaC in ose-public).
- Executing the changes — this is a planning-only deliverable.

## Product-Level Risks

| Risk                                                     | Mitigation                                                                    |
| -------------------------------------------------------- | ----------------------------------------------------------------------------- |
| F# analyzer version churn re-breaks TWAE builds silently | Pin the G-Research analyzer version explicitly (e.g. `0.17.0`) per the brief  |
| New gates double-run (CI + hook) slow down local pushes  | Make new lint targets Nx-cacheable where the underlying tool is deterministic |
| Rationale doc omits a decision and misleads contributors | Checklist item enumerates every D-row + the D5 deferral explicitly            |
