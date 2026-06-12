# Tech Docs — Standardize CI Parity (ose-public)

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

## Converged CI Target (shared across the three-repo sibling set)

This is the **fixed end-state** every sibling plan converges to. It is a **static
specification** — not a moving target produced by another plan — so **no plan depends on
another finishing first**. Each repo converges independently and **all three plans are safe
to execute in parallel**. The three plans embed this same target verbatim; per-repo
differences are recorded in the [Deviation Matrix](#deviation-matrix).

There is **no single anchor repo**. The target is the best-of-breed union across the three
pipelines as of 2026-06-12: `ose-primer` already ships the tool-named lint jobs and the
gherkin target; `ose-public` already ships current action majors; `ose-infra` already runs
`nx affected`. Each repo leads on some dimensions and trails on others.

Sibling plans (same slug in each repo):

- `ose-public` — `plans/in-progress/standardize-ci-parity/`
  (<https://github.com/wahidyankf/ose-public/tree/main/plans/in-progress/standardize-ci-parity>)
- `ose-infra` (private) — `plans/in-progress/standardize-ci-parity/`
- `ose-primer` — `plans/in-progress/standardize-ci-parity/`
  (<https://github.com/wahidyankf/ose-primer/tree/main/plans/in-progress/standardize-ci-parity>)

| Dimension                              | Converged target end-state                                                                                                                             |
| -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `actions/checkout` major               | `@v6`                                                                                                                                                  |
| Non-TS PR-gate test semantics          | `nx affected` (single-project governance gates such as `specs-gate` may keep `run-many`)                                                               |
| `validate:gherkin-keyword-cardinality` | Nx target present **and** wired into the markdown validator workflow                                                                                   |
| Reusable-workflow pattern              | adopted (`_reusable-*.yml` + thin callers)                                                                                                             |
| Concurrency                            | canonical block on every workflow: `group: ${{ github.workflow }}-${{ github.ref }}`, `cancel-in-progress: ${{ github.event_name == 'pull_request' }}` |
| Lint-gate jobs                         | three **tool-named** jobs: `shellcheck`, `hadolint`, `actionlint`                                                                                      |
| Governance jobs                        | `naming` (where `.claude/agents/` exists) + `specs-gate` (where `specs/` exists)                                                                       |
| Scheduled cadence                      | twice-daily WIB — `0 23 * * *` (06:00 WIB) and `0 11 * * *` (18:00 WIB) — for scheduled test/deploy workflows                                          |
| `ci-conventions.md`                    | carries a `## CI Parity Checklist` enumerating the invariants above and recording the deviations                                                       |

### Convergence status per repo (baseline 2026-06-12)

| Dimension                              | ose-public                                  | ose-infra                           | ose-primer                           |
| -------------------------------------- | ------------------------------------------- | ----------------------------------- | ------------------------------------ |
| `checkout@v6`                          | done                                        | gap — `@v4` → bump                  | done                                 |
| Non-TS `nx affected`                   | gap — `run-many` → affected                 | done                                | done                                 |
| `gherkin-keyword-cardinality` target   | gap — add + wire                            | done                                | done                                 |
| Reusable workflows                     | done                                        | gap — extract monolith              | done                                 |
| Concurrency (canonical, all workflows) | gap — add (0 today)                         | gap — add pr-gate + align 3 drifted | gap — add (0 today)                  |
| Lint jobs tool-named                   | gap — rename `shell`/`dockerfile`/`actions` | gap — split `infra-lint`            | done — reference scheme              |
| `naming` + `specs-gate`                | done — both                                 | gap — add both                      | gap — has `naming`; add `specs-gate` |
| Scheduled cadence 2× WIB               | done                                        | gap — align 1× → 2×                 | confirm/align per-language workflows |

Legend: _done_ = already at target (confirm only) · _gap_ = closed by this repo's plan.

## Deviation Matrix

Intentional per-repo differences — **recorded, not converged**. Each respects a genuine
per-repo constraint.

| Deviation                                             | ose-public                                  | ose-infra                                | ose-primer                                                             | Rationale                                                                                             |
| ----------------------------------------------------- | ------------------------------------------- | ---------------------------------------- | ---------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| Runner target                                         | `ubuntu-latest`                             | `[self-hosted, linux, ose-infra-runner]` | `ubuntu-latest`                                                        | infra needs warm Docker/Terraform/Ansible + on-prem reach; public/primer use ephemeral hosted runners |
| Language matrix                                       | TS + Go + F#/.NET + Rust                    | TS + Go + Rust                           | full polyglot (TS, Go, JVM, .NET, Python, Rust, Elixir, Clojure, Dart) | detection follows each repo's real portfolio; primer is the polyglot template                         |
| `npm` install flag                                    | `npm ci`                                    | `npm ci --ignore-scripts`                | `npm ci`                                                               | self-hosted hardening on the persistent infra runner                                                  |
| `setup-docker` composite                              | absent                                      | present                                  | absent                                                                 | hosted runners ship Docker; self-hosted must warm it                                                  |
| Rust toolchain action                                 | `actions-rust-lang/setup-rust-toolchain@v1` | `dtolnay/rust-toolchain@stable`          | `actions-rust-lang/setup-rust-toolchain@v1`                            | existing infra composite; kept to avoid churn                                                         |
| IaC lint job (`iac-lint`: terraform/ansible/yamllint) | absent                                      | present                                  | absent                                                                 | infra-only — terraform/ansible/yaml surface exists only in ose-infra                                  |

## Design Decisions

### D1 — Converge to `nx affected` for all per-language PR-gate jobs

ose-public's PR gate already uses `nx affected` for TypeScript but
`nx run-many --projects=tag:lang:*` for Go, F#/.NET, and Rust [Repo-grounded — the three
per-language `run-many` jobs at lines ~133 (golang), ~149 (fsharp,csharp), ~165 (rust)]. The
convergence replaces `run-many` with `affected` on those three jobs, keeping the identical target
list (`typecheck lint test:quick spec-coverage`) and the identical project-tag scoping via the
affected graph.

The `specs-gate` job's `nx run-many ... --projects=rhino-cli` (the kept `validate:specs-*`
single-project `run-many` at line ~197) [Repo-grounded] is a **single-project deterministic
governance gate**, not a
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

`ci-conventions.md` is the standard all three sibling repos align to. This plan updates it so the
PR-gate per-language semantics read `nx affected` (not `run-many`), documents the canonical
concurrency pattern, documents the tool-named lint-gate jobs (cross-referencing
`cross-language-lint-strictness.md`), and adds a new **CI Parity Checklist** section enumerating the
parity invariants. The checklist explicitly records the runner / language-matrix / npm-flag /
setup-docker / Rust-toolchain / infra-only `iac-lint` deviations so they read as decisions. The `ci-checker` agent is evaluated for new parity checks (e.g., "every PR
workflow declares a concurrency group", "no per-language PR-gate job uses `run-many`"); additions
are made only if they fit the agent's existing deterministic-check shape.

### D6 — Lint-gate job rename to the tool-named scheme

ose-public's three lint-gate jobs are **category-named**: `shell` (L66), `dockerfile` (L78), and
`actions` (L92) [Repo-grounded]. The converged target uses the **tool-named** scheme —
`shellcheck`, `hadolint`, `actionlint` — which is the scheme `ose-primer` already ships, adopted as
the canonical naming across the three-repo set. This plan renames the three jobs and updates every
reference to them:

- `quality-gate.needs` lists `shell, dockerfile, actions` today; these become
  `shellcheck, hadolint, actionlint`.
- The "CI job" column of
  [`cross-language-lint-strictness.md`](../../../repo-governance/development/quality/cross-language-lint-strictness.md)
  currently references the `shell`/`dockerfile`/`actions` job names; it is updated to the tool-named
  jobs.

This is a **pure rename — no behavior change**: the same three linters (`shellcheck`, `hadolint`,
`actionlint`) run at the same warning-and-above thresholds against the same file sets; only the job
identifiers change so the three sibling pipelines read identically.

## File Impact

| File                                                                    | Change                                                                                                                                                                               | Phase   |
| ----------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------- |
| `.github/workflows/pr-quality-gate.yml`                                 | run-many → affected (Go/.NET/Rust jobs); add concurrency block; rename lint jobs `shell`/`dockerfile`/`actions` → `shellcheck`/`hadolint`/`actionlint` + update `quality-gate.needs` | 1, 2, 3 |
| `.github/workflows/validate-markdown.yml`                               | add gherkin-keyword-cardinality step; add concurrency block                                                                                                                          | 2, 4    |
| `.github/workflows/validate-env.yml`                                    | add concurrency block                                                                                                                                                                | 2       |
| `.github/workflows/test-and-deploy-*.yml` (scheduled quartet)           | add concurrency block                                                                                                                                                                | 2       |
| `apps/rhino-cli/project.json`                                           | new `validate:gherkin-keyword-cardinality` target                                                                                                                                    | 4       |
| `repo-governance/development/quality/cross-language-lint-strictness.md` | update the "CI job" column from `shell`/`dockerfile`/`actions` to `shellcheck`/`hadolint`/`actionlint`                                                                               | 3       |
| `repo-governance/development/infra/ci-conventions.md`                   | converged standard text + new CI Parity Checklist section                                                                                                                            | 5       |
| `.claude/agents/ci-checker.md`                                          | parity-check additions (if warranted)                                                                                                                                                | 5       |

## Action-Version Reference (recorded for the converged target)

ose-public is **already** on these majors [Repo-grounded for the ones present in ose-public;
`[Web-cited — accessed 2026-06-11]` for "current major" status — sources in the note below the
table]. This dimension is a _done_ for ose-public (confirm only); the bump work falls to whichever
sibling plan still trails (`ose-infra`). This table is the shared target all three plans converge
to.

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
| Lint jobs renamed to tool-named scheme    | grep / actionlint  | Phase 3 gate |
| Gherkin validator runs + passes           | Nx target run      | Phase 4 gate |
| `ci-conventions.md` converged + checklist | grep / link check  | Phase 5 gate |
| Full pipeline green                       | CI status          | Phase 6 gate |

## Rollback

Each phase is an independent thematic commit pushed at its gate. Rollback = `git revert` of the
offending phase commit on `origin main`; no schema, data, or deploy state is touched, so revert is
clean and immediate.

## Dependencies

No new runtime or build dependencies are introduced. The `validate:gherkin-keyword-cardinality`
target wraps an **already-shipped** rhino-cli command — no new crates, packages, or actions. The
[Dependency Bump Policy](../../../repo-governance/development/workflow/dependency-bump-policy.md)
is therefore not triggered by this plan.
