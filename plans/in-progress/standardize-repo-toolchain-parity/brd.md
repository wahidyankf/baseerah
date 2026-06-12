# BRD — Standardize Repo Toolchain Parity (ose-public)

This Business Requirements Document explains **why** the toolchain standardization exists. The
**what** (features, scope, acceptance criteria) lives in [prd.md](./prd.md); the **how** lives in
[tech-docs.md](./tech-docs.md).

## Business Goal

Make the **repository toolchain** of `ose-public` and its two sibling repos (`ose-infra`,
`ose-primer`) converge to a single **fixed Converged Toolchain Target** — CI workflows, git hooks,
the `rhino-cli` CLI (architecture + command surface + Nx target names), and the governing docs all
functionally identical except for recorded per-repo deviations — so that a contributor (or AI agent)
who understands one repo's toolchain understands all three, and so that the downstream
[`ose-infra/plans/in-progress/deploy-twin-k3s-clusters/`](../../../docs/reference/related-repositories.md)
deployment runs against a converged, version-current, known-good toolchain.

For CI/hooks/target-naming/docs (workstreams A/B/E/F) there is **no single anchor repo**: each repo
closes only its own gaps and the plans are **parallel-safe**. For the rhino-cli architecture and
command surface (workstreams C/D) the convergence is **reference-first**: `ose-public` is the
reference that authors first, and the siblings port from it.

## Business Rationale

Toolchain drift between sibling repos is a slow, compounding tax across six surfaces:

- **CI cognitive load (A)** — every divergence (`run-many` here, `affected` there; `@v6` here,
  `@v4` there; `shell`/`dockerfile`/`actions` here, `shellcheck`/`hadolint`/`actionlint` there; a
  gate that runs on PR here but also on push-to-main there) is a thing a maintainer must hold in
  their head per repo. This repo is solo-maintained, so the load lands on one person. [Judgment call]
- **Inconsistent gate strength (A)** — ose-public's PR gate runs `nx run-many` for non-TS languages
  (whole tagged set regardless of change) while siblings run `nx affected`; ose-public gates only on
  `pull_request` while the converged target also gates `push` to `main`. Direct worktree-to-main
  pushes (the repo's TBD norm) currently skip the full gate. [Repo-grounded]
- **Wasted compute (A)** — ose-public has no concurrency cancellation, so superseded pushes keep
  burning CI minutes; the converged block cancels in-progress PR runs only. [Repo-grounded]
- **Hook lifecycle drift (B)** — the three repos' `commit-msg`/`pre-commit`/`pre-push` hooks differ
  in build flags, lint-staged wiring, and which conditional validators run, so the local pre-flight
  contract differs per repo. [Judgment call]
- **rhino-cli architecture drift (C)** — ose-public's CLI is a flat `src/commands/` + `src/internal/`
  layout, not hexagonal; testing IO-bound logic means reaching through to the filesystem/process
  layer. A hexagonal core (pure domain + injected ports) makes the CLI testable and identical across
  repos, and folds in the salvaged `migrate-rhino-cli-to-hexagonal` design. [Repo-grounded]
- **rhino-cli command-surface drift (D)** — ose-public is missing the `Java` and `Contracts`
  subcommands that the union superset (and the siblings) carry, so the CLI is not a drop-in across
  repos. [Repo-grounded — current set lacks both]
- **Target-naming drift (E)** — governance/validation/lint targets use ad-hoc `validate:*` / `lint:*`
  / `fmt:check` names rather than the canonical `{domain}:{work}` scheme, and `spec-coverage` is
  spelled inconsistently with the `:`-delimited lifecycle targets. [Repo-grounded]
- **Governance drift (F)** — a rule documented in one repo's conventions but not another's quietly
  rots; without a propagation + quality-gate step the docs fall out of sync with the toolchain they
  describe. [Judgment call]
- **State diagrams escape the render-width gate (G)** — the `mermaid:validation` discipline keeps
  diagrams readable on mobile viewports and in narrow PDF/doc columns, but it currently applies
  **only to flowcharts**: an 11-state `stateDiagram-v2 direction LR` chain renders far too wide for
  mobile yet sails through the gate, while an equivalent flowchart is blocked. State diagrams are an
  unguarded escape hatch from a discipline the repo already invests in. [Repo-grounded:
  `apps/rhino-cli/src/internal/mermaid.rs:342-356` returns count `0` for non-flowchart headers]
- **Deployment risk** — the downstream twin-k3s deployment assumes the toolchain is trustworthy;
  standardizing it first de-risks that deployment. [Judgment call]

## Business Impact

### Pain Points Addressed

- A maintainer reading any sibling repo's CI/hooks/CLI cannot assume it matches `ose-public`.
- ose-public wastes CI minutes (no cancel-in-progress) and under-gates direct main pushes (no
  push-to-main full gate).
- ose-public over-tests non-TS languages via `run-many`.
- ose-public's rhino-cli is hard to unit-test (flat IO-coupled layout) and is missing union commands.
- Target names and `spec-coverage` spelling diverge from the canonical scheme.
- Governance docs drift from the toolchain without a propagation + quality gate.

### Expected Benefits

- **One mental model** of the whole toolchain across all three repos (minus recorded deviations).
- **Faster, cheaper, fully-gated CI** in ose-public (affected-only non-TS jobs + cancel-in-progress
  - push-to-main full gate).
- **Identical, testable rhino-cli** — same hexagonal architecture and same union command surface
  everywhere.
- **Canonical target names** (`{domain}:{work}`, `spec:coverage`) across the family.
- **Self-healing governance** — docs propagated and quality-gated so they stay in sync.
- **State diagrams held to the same render-width discipline as flowcharts** — the readability
  benefit the flowchart rule already earns transfers directly to state diagrams. _Judgment call:
  the state width/label rule is identical to the trusted flowchart rule._
- **Parity locked by a machine-checked golden corpus** — one identical state-diagram fixture set
  (`.md` + expected violation JSON) committed to all three repos makes future Mermaid-rule parity
  automatic rather than a three-way manual reconciliation.
- **A converged baseline** the twin-k3s deployment can rely on.

## Affected Roles

Solo-maintainer repository — the roles below are **hats the maintainer wears** and **agents that
consume the outputs**:

- **CI maintainer hat** — edits the workflows and `ci-conventions.md`.
- **Toolchain/CLI maintainer hat** — performs the rhino-cli hexagonal migration and command ports.
- **Release/deploy hat** — depends on the converged toolchain before the downstream twin-k3s deploy.
- **`ci-checker` / `ci-fixer` agents** — validate/fix projects against `ci-conventions.md`.
- **`repo-rules-maker` / `repo-rules-checker` / `repo-rules-fixer` agents** — propagate the doc
  changes and run the final repo-rules quality gate.
- **`plan-checker` / `plan-execution-checker` agents** — validate this plan and its execution.

## Business-Level Success Metrics (per workstream)

- **A — CI parity met**: every per-language PR-gate job uses `nx affected`; every workflow declares a
  concurrency block; lint jobs are tool-named; the `gherkin:keyword-cardinality-validation` target
  runs in CI; the **full quality gate runs on `push` to `main`**; scheduler cadence is 2× WIB.
  [Observable — grep/diff the workflows against the CI/toolchain Parity Checklist]
- **B — Hook parity met**: `commit-msg`/`pre-commit`/`pre-push` match the BLOCK 1-B canonical
  lifecycle and reference the renamed targets. [Observable — diff the `.husky/*` hooks against
  BLOCK 1-B]
- **C — Hexagonal migration complete**: rhino-cli has the `domain`/`application`/`infrastructure`/
  `commands` layout and the golden-master CLI suite is byte-identical to the Phase 0 baseline.
  [Observable — directory layout + golden-master diff = empty]
- **D — Union command surface met**: `rhino-cli` exposes the full superset including `Java` and
  `Contracts`. [Observable — `rhino-cli --help` lists all union subcommands]
- **E — Target naming met**: every governance/validation/lint/check target uses `{domain}:{work}`
  and `spec:coverage` repo-wide; no caller references an old name. [Observable — grep the project.json
  files, hooks, workflows, package.json]
- **F — Governance gate clean**: all related docs updated, `repo-rules-maker` propagated, and the
  `repo-rules-quality-gate` workflow reports clean. [Observable — the workflow's terminal report]
- **G — State-diagram validation met**: `mermaid:validation` flags every state diagram exceeding 4
  nodes on a rank or 30 characters in a state/transition label; zero such violations remain repo-wide
  after the aggressive cleanup; the golden corpus produces byte-identical violation output across all
  three repos; flowchart behavior is unchanged. [Observable — run the gate over the over-wide
  fixtures and a full-repo scan; diff the committed expected-JSON across repos]
- **All CI green after push** — the standardized toolchain passes on `origin main`. [Observable —
  GitHub Actions status]

## Business-Scope Non-Goals

- This plan does **not** converge the runner target — an accepted, documented deviation (ephemeral
  hosted runners for public/primer; warm self-hosted runners for infra).
- This plan does **not** introduce new toolchain capabilities, deploy targets, or Nx Cloud changes
  beyond parity.
- This plan does **not** perform the siblings' side of A/B/E/F (their version bumps, reusable-workflow
  adoption, `infra-lint` split, `specs-gate` addition) — each is the respective sibling plan's
  responsibility. For C/D the siblings **port from** ose-public's reference, in their own repos.

## Business Risks and Mitigations

| Risk                                                                | Likelihood | Impact | Mitigation                                                                                                                                             |
| ------------------------------------------------------------------- | ---------- | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Prerequisite (bootstrap-be) not done, .NET surface missing          | Medium     | High   | Phase 0 gate hard-verifies `crane-be`, GHCR workflow, and `.NET` detection before any work begins                                                      |
| Hexagonal migration silently changes rhino-cli output               | Medium     | High   | Golden-master CLI suite captured in Phase 0 byte-verifies the output surface at every feature group and phase gate                                     |
| `nx affected` misses a project the old `run-many` would have caught | Low        | Medium | PR-only gate has `github.base_ref` defined; full gate on push-to-main covers the merge-time picture                                                    |
| Target rename leaves a caller pointing at a non-existent target     | Medium     | High   | Phase 10 caller-sweep checklist + Phase 6/10 sequencing so hooks never reference an unrenamed target between phases                                    |
| New gherkin validator surfaces preexisting cardinality violations   | Medium     | Low    | Root-cause orientation — fix flagged violations in-plan rather than disabling the validator                                                            |
| Sibling C/D port diverges from public's reference                   | Low        | Medium | ose-public is the single reference crate structure; siblings copy it; the deviation matrix records only true diffs                                     |
| Governance docs drift from the toolchain after edits                | Low        | Medium | Phase 11 runs `repo-rules-maker` + the `repo-rules-quality-gate` workflow as a hard gate before the plan can finish                                    |
| State front-end (G) silently changes flowchart behavior             | Low        | Medium | State support is a second front-end on the already-migrated, golden-frozen Mermaid slice; every flowchart test stays green before any state code lands |
| Aggressive D-CLEAN touches `plans/done/` history                    | Low        | Low    | Explicit recorded D-CLEAN choice for maximum hygiene; edits are diagram-only and reviewed in the cleanup phase                                         |
