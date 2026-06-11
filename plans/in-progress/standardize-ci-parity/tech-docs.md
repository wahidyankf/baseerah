# Tech Docs — Standardize CI Parity (ose-public anchor)

This document explains **how** the convergence is built. The **why** lives in [brd.md](./brd.md);
the **what** lives in [prd.md](./prd.md). All claims are labeled with confidence; `[Repo-grounded]`
claims were verified against the current worktree, `[Web-cited]` against external docs accessed
2026-06-11.

## Reference Documents

- [CI/CD Conventions](../../../repo-governance/development/infra/ci-conventions.md) — the standard
  this plan aligns and extends.
- [GitHub Actions Workflow Naming Convention](../../../repo-governance/development/infra/github-actions-workflow-naming.md)
- [Nx Target Standards](../../../repo-governance/development/infra/nx-targets.md)
- [CI Post-Push Verification](../../../repo-governance/development/workflow/ci-post-push-verification.md)
- [CI Monitoring](../../../repo-governance/development/workflow/ci-monitoring.md)
- [Test-Driven Development Convention](../../../repo-governance/development/workflow/test-driven-development.md)
- `ci-checker` agent: [.claude/agents/ci-checker.md](../../../.claude/agents/ci-checker.md)
- `ci-fixer` agent: [.claude/agents/ci-fixer.md](../../../.claude/agents/ci-fixer.md)

## Current-State Divergences

The complete divergence table across both repos, with the converged target end-state. The
**Owner** column states which plan delivers the change (anchor = this plan; sibling = ose-infra's
plan; neither = recorded deviation). All `now` values for ose-public are `[Repo-grounded]`; the
ose-infra `now` values are stated from the user-provided current-state inventory and tagged
`[Provided — verify in sibling]`.

| Dimension                  | ose-public (now)                       | ose-infra (now)                              | Target end-state                                          | Owner                |
| -------------------------- | -------------------------------------- | -------------------------------------------- | --------------------------------------------------------- | -------------------- |
| Runners                    | all `ubuntu-latest`                    | all `[self-hosted, linux, ose-infra-runner]` | KEEP per-repo (accepted deviation)                        | Neither              |
| Reusable workflows         | 7 `_reusable-*.yml`                    | none (monolithic test workflow)              | ose-infra adopts reusable-workflow pattern                | Sibling              |
| Non-TS test invocation     | `nx run-many --projects=tag:lang:*`    | `nx affected --projects=tag:lang:*`          | BOTH use `nx affected`                                    | Anchor               |
| PR-gate extra jobs         | has `naming` + `specs-gate`            | absent                                       | ose-infra adds equivalents where applicable               | Sibling              |
| Validator set              | no `gherkin-keyword-cardinality` in CI | has it                                       | both run the same validator set                           | Anchor               |
| GH Actions action versions | current majors (`@v6`/`@v5`)           | `@v4` (2 majors behind)                      | ose-infra bumps to current majors                         | Sibling              |
| Concurrency groups         | none                                   | present (cancel-in-progress)                 | both use the canonical pattern                            | Anchor (public side) |
| .NET language detection    | present (TS/Go/.NET/Rust)              | absent (TS/Go/Rust — no .NET)                | RECORDED deviation: detection matrix differs by portfolio | Neither              |
| npm install flag           | `npm ci`                               | `npm ci --ignore-scripts`                    | accepted self-hosted deviation (see below)                | Neither              |
| setup-docker composite     | absent (ubuntu has docker)             | present (self-hosted needs it)               | accepted deviation tied to runner choice                  | Neither              |
| Scheduled cadence          | 2x daily (06:00/18:00 WIB)             | 1x daily                                     | align ose-infra to 2x WIB                                 | Sibling              |

`[Repo-grounded]` evidence for ose-public `now` values: `pr-quality-gate.yml:93,109,125`
(run-many), absence of any `concurrency:` line across `.github/workflows/`, absence of a
`validate:gherkin-keyword-cardinality` target in `apps/rhino-cli/project.json`, presence of
`actions/checkout@v6` / `setup-dotnet@v5` throughout, and `setup-node` composite action running
plain `npm ci`.

## Deviation Matrix

These divergences are **intentionally not converged**. Each respects a genuine per-repo
constraint. They are recorded so a reader knows they are decisions, not drift.

| Deviation               | ose-public                | ose-infra                                | Rationale                                                                                                                      |
| ----------------------- | ------------------------- | ---------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| Runner target           | `ubuntu-latest`           | `[self-hosted, linux, ose-infra-runner]` | public wants ephemeral, no-infra-dependency runners; infra needs warm Docker/Terraform/Ansible tooling + on-prem network reach |
| .NET language detection | present (TS/Go/.NET/Rust) | absent (TS/Go/Rust)                      | detection matrix follows each repo's actual language portfolio; infra has no .NET surface                                      |
| npm install flag        | `npm ci`                  | `npm ci --ignore-scripts`                | self-hosted runners harden against arbitrary lifecycle scripts; ephemeral ubuntu does not need it                              |
| setup-docker composite  | absent                    | present                                  | ubuntu-latest ships Docker; self-hosted runner must install/warm it                                                            |

**Self-hosted + fork-PR note** [Web-cited — GitHub security-hardening docs,
<https://docs.github.com/en/actions/security-for-github-actions/security-guides/security-hardening-for-github-actions>,
accessed 2026-06-11; excerpt: "Self-hosted runners should almost never be used for public
repositories ... any user can open pull requests against the repository and compromise the
environment"]:
GitHub warns that self-hosted runners should almost never run untrusted fork-PR code, recommending
ephemeral runners for public repos. `ose-infra` is **private**, so the fork-PR risk is materially
lower, but the sibling plan should record the ephemeral-runner guidance as a note. This anchor
(public) repo uses `ubuntu-latest`, so the concern does not apply here.

## Design Decisions

### D1 — Converge to `nx affected` for all per-language PR-gate jobs

ose-public's PR gate already uses `nx affected` for TypeScript [Repo-grounded —
`pr-quality-gate.yml:77`] but `nx run-many --projects=tag:lang:*` for Go, .NET, and Rust
[Repo-grounded — lines 93/109/125]. The convergence replaces `run-many` with `affected` on those
three jobs, keeping the identical target list (`typecheck lint test:quick spec-coverage`) and the
identical project-tag scoping via the affected graph.

The `specs-gate` job's `nx run-many ... --projects=rhino-cli` [Repo-grounded —
`pr-quality-gate.yml:157`] is a **single-project deterministic governance gate**, not a
per-language affected job, and is **left intact** — "all per-language PR-gate jobs" does not include
the single-project rhino-cli specs gate.

### D2 — SHA-computation mechanism: keep inline `NX_BASE`/`NX_HEAD`

`nx affected` needs a base and head SHA. ose-public already sets these inline on every affected
job [Repo-grounded — `pr-quality-gate.yml:21-22,70-71` etc.]:

```yaml
env:
  NX_BASE: origin/${{ github.base_ref }}
  NX_HEAD: ${{ github.sha }}
```

**Decision: keep the inline mechanism; do not adopt `nrwl/nx-set-shas@v5`.** [Web-cited —
Nx CI-setup docs, <https://nx.dev/docs/guides/nx-cloud/setup-ci>, accessed 2026-06-11; excerpt:
"uses: nrwl/nx-set-shas@v5" — `@v5` is the version referenced by the official Nx docs (the
releases page shows a v4.3.0 point release; pin `@v5` per the docs and confirm before adopting)].
For a **PR-only** gate,
`github.base_ref` is always defined, so the inline form is correct, dependency-free, and already
in use on the TS job. `nrwl/nx-set-shas` earns its keep on `push`-to-main workflows where the
"last successful run" SHA is non-trivial to compute — not applicable to this PR gate. Documenting
the choice here satisfies the resolved-decision requirement to record it.

### D3 — Canonical concurrency pattern

Add the GitHub-recommended concurrency block [Web-cited — GitHub Actions concurrency docs,
<https://docs.github.com/en/actions/how-tos/write-workflows/choose-when-workflows-run/control-workflow-concurrency>,
accessed 2026-06-11; excerpt: "use the `concurrency` key ... `cancel-in-progress: true` ... to
cancel any currently running job or workflow in the same concurrency group"] to the PR gate, the
validator workflows, and the scheduled workflows:

```yaml
concurrency:
  group: ${{ github.workflow }}-${{ github.event_name == 'pull_request' && github.event.pull_request.number || github.ref }}
  cancel-in-progress: ${{ github.event_name == 'pull_request' }}
```

The group key uses the PR number for `pull_request` events and the ref otherwise, so PR re-pushes
cancel the prior run while `push`-to-main and scheduled runs are keyed by ref and **not** cancelled
(cancel-in-progress is `true` only for PR events). This matches the canonical pattern the sibling
already uses.

### D4 — `validate:gherkin-keyword-cardinality` Nx target

The audit logic already ships as a rhino-cli command: `rhino-cli repo-governance
gherkin-keyword-cardinality` [Repo-grounded —
`apps/rhino-cli/src/commands/governance_gherkin_keyword_cardinality_audit.rs`]. There is **no Nx
target** wrapping it [Repo-grounded — no `validate:gherkin-keyword-cardinality` key in
`apps/rhino-cli/project.json`]. This plan creates the target following the exact shape of the
existing `validate:*` targets (e.g., `validate:specs-links`):

```jsonc
"validate:gherkin-keyword-cardinality": {
  "executor": "nx:run-commands",
  "options": {
    "command": "cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- repo-governance gherkin-keyword-cardinality"
  },
  "cache": true
}
```

The exact `executor`/`options`/`inputs`/`cache` keys and the precise positional/flag arguments the
subcommand expects MUST be confirmed against the existing sibling targets and the command's
`--help` during Phase 3 (RED step). The wrapper is then added to `validate-markdown.yml` as a new
step alongside the existing mermaid/links/heading-hierarchy steps.

### D5 — Governance alignment + CI Parity Checklist

`ci-conventions.md` is the standard both repos align to. This plan updates it so the PR-gate
per-language semantics read `nx affected` (not `run-many`), documents the canonical concurrency
pattern, and adds a new **CI Parity Checklist** section enumerating the parity invariants. The
checklist explicitly records the runner / .NET-detection / npm-flag / setup-docker deviations so
they read as decisions. The `ci-checker` agent is evaluated for new parity checks (e.g., "every PR
workflow declares a concurrency group", "no per-language PR-gate job uses `run-many`"); additions
are made only if they fit the agent's existing deterministic-check shape.

## File Impact

| File                                                          | Change                                                         | Phase |
| ------------------------------------------------------------- | -------------------------------------------------------------- | ----- |
| `.github/workflows/pr-quality-gate.yml`                       | run-many → affected (Go/.NET/Rust jobs); add concurrency block | 1, 2  |
| `.github/workflows/validate-markdown.yml`                     | add gherkin-keyword-cardinality step; add concurrency block    | 2, 3  |
| `.github/workflows/validate-env.yml`                          | add concurrency block                                          | 2     |
| `.github/workflows/test-and-deploy-*.yml` (scheduled quartet) | add concurrency block                                          | 2     |
| `apps/rhino-cli/project.json`                                 | new `validate:gherkin-keyword-cardinality` target              | 3     |
| `repo-governance/development/infra/ci-conventions.md`         | converged standard text + new CI Parity Checklist section      | 4     |
| `.claude/agents/ci-checker.md`                                | parity-check additions (if warranted)                          | 4     |

## Action-Version Reference (sibling-facing — recorded for the converged target)

ose-public is **already** on these majors [Repo-grounded for the ones present in ose-public;
`[Web-cited — accessed 2026-06-11]` for "current major" status — sources in the note below the
table]. The bump work is the sibling's; this table is the shared target both plans converge to.

| Action                                   | Current major | Notes                                                             |
| ---------------------------------------- | ------------- | ----------------------------------------------------------------- |
| `actions/checkout`                       | `@v6`         | v6 stores creds under `$RUNNER_TEMP`; needs runner ≥ 2.329.0      |
| `actions/setup-node`                     | `@v6`         | —                                                                 |
| `actions/setup-go`                       | `@v6`         | —                                                                 |
| `actions/setup-dotnet`                   | `@v5`         | only where a .NET surface exists                                  |
| `actions/cache`                          | `@v5`         | —                                                                 |
| `actions/upload-artifact`                | `@v7`         | must pair with `download-artifact@v8`; only if artifacts are used |
| `actions/download-artifact`              | `@v8`         | pairs with `upload-artifact@v7`                                   |
| `actions-rust-lang/setup-rust-toolchain` | `@v1`         | ose-public already uses this                                      |
| `Swatinem/rust-cache`                    | `@v2`         | ose-public already uses this                                      |
| `nrwl/nx-set-shas`                       | `@v5`         | NOT adopted here — see D2 (inline mechanism for PR-only gates)    |

All "current major" statements above are [Web-cited — official GitHub Actions release pages,
accessed 2026-06-11]: `actions/checkout` <https://github.com/actions/checkout/releases> (v6);
`actions/setup-node` <https://github.com/actions/setup-node/releases> (v6); `actions/setup-go`
<https://github.com/actions/setup-go/releases> (v6); `actions/setup-dotnet`
<https://github.com/actions/setup-dotnet/releases> (v5); `actions/cache`
<https://github.com/actions/cache/releases> (v5); `actions/upload-artifact`
<https://github.com/actions/upload-artifact/releases> (v7) pairing `actions/download-artifact`
<https://github.com/actions/download-artifact/releases> (v8);
`actions-rust-lang/setup-rust-toolchain` <https://github.com/actions-rust-lang/setup-rust-toolchain/releases>
(v1); `Swatinem/rust-cache` <https://github.com/Swatinem/rust-cache/releases> (v2). Excerpt
(checkout): "v6.0.2 ... Latest". The versions actually present in ose-public are `[Repo-grounded]`.

## Testing Strategy

CI-YAML and Nx-config changes are not classic unit-testable code, so each delivery step uses a
**verify-command-driven** TDD shape per the
[Test-Driven Development Convention](../../../repo-governance/development/workflow/test-driven-development.md):

- **RED** — an assertion proving the undesired/absent state (e.g., `grep` proving `run-many` is
  still present; the new Nx target absent; no `concurrency:` block).
- **GREEN** — the YAML / JSON edit that makes the assertion flip.
- **REFACTOR** — dedup / cleanup (e.g., shared anchors, consistent ordering) with the assertion
  still green.

Where a YAML linter helps, `actionlint` is used as an additional gate; if `actionlint` is not
installed, the step falls back to a `grep`/`yq`-based structural assertion plus the live
`npx nx run rhino-cli:validate:*` runs. Each Gherkin scenario in [prd.md](./prd.md) maps to a
phase gate check.

| Acceptance criterion (Gherkin)            | Verification level | Where        |
| ----------------------------------------- | ------------------ | ------------ |
| Prerequisite verified                     | structural (Bash)  | Phase 0 gate |
| Non-TS jobs use `nx affected`             | grep / actionlint  | Phase 1 gate |
| Concurrency block added                   | grep / actionlint  | Phase 2 gate |
| Gherkin validator runs + passes           | Nx target run      | Phase 3 gate |
| `ci-conventions.md` converged + checklist | grep / link check  | Phase 4 gate |
| Full pipeline green                       | CI status          | Phase 5 gate |

## Rollback

Each phase is an independent thematic commit pushed at its gate. Rollback = `git revert` of the
offending phase commit on `origin main`; no schema, data, or deploy state is touched, so revert is
clean and immediate.

## Dependencies

No new runtime or build dependencies are introduced. The `validate:gherkin-keyword-cardinality`
target wraps an **already-shipped** rhino-cli command — no new crates, packages, or actions. The
[Dependency Bump Policy](../../../repo-governance/development/workflow/dependency-bump-policy.md)
is therefore not triggered by this plan.
