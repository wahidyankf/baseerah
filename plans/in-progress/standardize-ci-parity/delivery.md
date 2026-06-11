# Delivery — Standardize CI Parity (ose-public anchor)

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.
>
> **Phase Gate** — every phase ends with a `### Phase N Gate` (must-pass verification) plus a
> `> **Pause Safety**:` note (the safe-to-stop state and the single command to resume). A phase
> is not complete until its gate is green; do not start phase N+1 while any gate check fails.

This checklist delivers **only ose-public's** convergence changes. The action-version bumps,
reusable-workflow adoption, and `nx affected` migration on the **ose-infra** side belong to the
sibling plan and are referenced, not executed here. See
[tech-docs.md § Deviation Matrix](./tech-docs.md#deviation-matrix).

## Worktree

Worktree path: `worktrees/standardize-ci-parity/`

Optional manual pre-provisioning (run from repo root):

```bash
claude --worktree standardize-ci-parity
```

The plan-execution Step 0 gate enters this worktree by default: it auto-provisions from the latest
`origin/main` when missing, syncs with `origin/main` before implementing, and prompts before
deleting the worktree after the plan is archived and pushed.

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
[Plans Organization Convention § Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

## Phase 0: Environment Setup and Baseline

> _Executor: repo-setup-manager_

This phase converges the toolchain, records the baseline, and **hard-verifies the upstream
prerequisite** (`bootstrap-be-messaging-and-crane-media`) landed before any CI work begins.

- [ ] [AI] Install dependencies in the root worktree: `npm install`
      — acceptance: exits 0, `node_modules/` synchronized.
- [ ] [AI] Converge the full polyglot toolchain in the root worktree: `npm run doctor -- --fix`
      — acceptance: exits 0 with no unresolved drift.
- [ ] [AI] Record the affected baseline: `npx nx affected -t typecheck lint test:quick spec-coverage`
      — acceptance: pass/fail count recorded; every preexisting failure documented.
- [ ] [AI] Resolve all preexisting failures before proceeding (root-cause orientation)
      — acceptance: no preexisting failures remain unresolved.
- [ ] [AI] **Prerequisite verification — `crane-be` exists**: `test -d apps/crane-be && echo OK`
      — acceptance: prints `OK` (the bootstrap-be plan shipped `apps/crane-be/`).
- [ ] [AI] **Prerequisite verification — GHCR publish workflow exists**:
      `ls .github/workflows/ | grep -Ei 'ghcr|publish|image' && echo OK`
      — acceptance: at least one matching workflow file is listed (the affected-aware GHCR
      image-publish workflow from the bootstrap-be plan). If naming differs, confirm by reading the
      workflow contents for `ghcr.io/wahidyankf/crane-be`.
- [ ] [AI] **Prerequisite verification — .NET detection present**:
      `grep -E 'lang:fsharp|lang:csharp|has-dotnet' .github/workflows/pr-quality-gate.yml && echo OK`
      — acceptance: `.NET` detection lines are present in the PR gate.

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [ ] [AI] `npm install` exited 0 and `npm run doctor -- --fix` reports no unresolved drift.
- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` baseline recorded and every
      preexisting failure resolved (zero unresolved).
- [ ] [AI] All three prerequisite verifications printed `OK` (crane-be dir, GHCR workflow, .NET
      detection). If any failed, STOP — the upstream prerequisite is not done and this plan must not
      proceed.

> **Pause Safety**: only the local toolchain was verified, the baseline recorded, and the upstream
> prerequisite confirmed — no CI changes exist yet. Safe to stop indefinitely. To resume: re-run
> the baseline command and the three prerequisite greps and confirm they are still clean.

## Phase 1: PR-gate test semantics — `nx run-many` → `nx affected`

Replace `nx run-many` with `nx affected` for the Go, .NET, and Rust per-language jobs in
`pr-quality-gate.yml`, keeping the identical target list and project-tag scoping. The TypeScript
job (already `nx affected`) and the single-project `specs-gate` `run-many` are left intact (see
[tech-docs.md § D1](./tech-docs.md#d1--converge-to-nx-affected-for-all-per-language-pr-gate-jobs)).

_Suggested executor: `ci-fixer`_

- [ ] [AI] **RED**: assert `run-many` still present in the per-language jobs:
      `grep -n "nx run-many -t typecheck lint test:quick spec-coverage" .github/workflows/pr-quality-gate.yml`
      — acceptance: matches the Go, .NET, and Rust job lines (3 hits at lines ~93/109/125).
- [ ] [AI] **GREEN**: in `.github/workflows/pr-quality-gate.yml`, change the Go job command from
      `npx nx run-many -t typecheck lint test:quick spec-coverage --projects='tag:lang:golang'`
      to `npx nx affected -t typecheck lint test:quick spec-coverage --projects='tag:lang:golang'`
      — acceptance: the Go job line uses `nx affected`.
- [ ] [AI] **GREEN**: change the .NET job command (currently
      `--projects='tag:lang:fsharp,tag:lang:csharp'`) from `nx run-many` to `nx affected`, target
      list unchanged — acceptance: the .NET job line uses `nx affected`.
- [ ] [AI] **GREEN**: change the Rust job command (currently `--projects='tag:lang:rust'`) from
      `nx run-many` to `nx affected`, target list unchanged; leave the subsequent
      `rhino-cli:fmt:check` / `deny:check` / `check:msrv` steps unchanged
      — acceptance: the Rust job line uses `nx affected`; the three rhino-cli single-target steps
      remain.
- [ ] [AI] **GREEN — verify no per-language run-many remains**:
      `grep -n "nx run-many" .github/workflows/pr-quality-gate.yml`
      — acceptance: the only remaining match is the `specs-gate` job's
      `--projects=rhino-cli` line (single-project deterministic gate, intentionally kept); the three
      per-language matches are gone.
- [ ] [AI] **REFACTOR**: confirm each affected job still declares the inline
      `NX_BASE: origin/${{ github.base_ref }}` / `NX_HEAD: ${{ github.sha }}` env block
      (`grep -n "NX_BASE\|NX_HEAD" .github/workflows/pr-quality-gate.yml`)
      — acceptance: every per-language affected job retains its SHA env block.
- [ ] [AI] Lint the workflow: `actionlint .github/workflows/pr-quality-gate.yml` if available, else
      `npx prettier --check .github/workflows/pr-quality-gate.yml`
      — acceptance: exits 0 (no syntax errors introduced).

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [ ] [AI] `grep -c "nx affected -t typecheck lint test:quick spec-coverage" .github/workflows/pr-quality-gate.yml`
      — expected: at least 4 (TypeScript + Go + .NET + Rust jobs).
- [ ] [AI] `grep "nx run-many" .github/workflows/pr-quality-gate.yml` — expected: only the
      `specs-gate` `--projects=rhino-cli` line remains.
- [ ] [AI] Workflow lints clean (`actionlint` or `prettier --check`) — expected: exits 0.
- [ ] [AI] Commit thematically: `git add .github/workflows/pr-quality-gate.yml && rtk git commit -m "ci(pr-gate): converge non-TS jobs to nx affected"`.

> **Pause Safety**: `pr-quality-gate.yml` is self-consistent — all per-language jobs use
> `nx affected`, the workflow lints clean, and the change is committed. Safe to stop. To resume:
> re-run the two grep checks above and confirm the commit is present (`rtk git log --oneline -1`).

## Phase 2: Concurrency groups — canonical pattern across workflows

Add the canonical concurrency block (see
[tech-docs.md § D3](./tech-docs.md#d3--canonical-concurrency-pattern)) to the PR gate, the
validator workflows, and the scheduled `test-and-deploy-*` quartet. No ose-public workflow declares
a concurrency group today [Repo-grounded].

_Suggested executor: `ci-fixer`_

The canonical block (insert at top level, after `on:` / `permissions:`):

```yaml
concurrency:
  group: ${{ github.workflow }}-${{ github.event_name == 'pull_request' && github.event.pull_request.number || github.ref }}
  cancel-in-progress: ${{ github.event_name == 'pull_request' }}
```

- [ ] [AI] **RED**: assert no concurrency block exists:
      `grep -rL "concurrency:" .github/workflows/pr-quality-gate.yml .github/workflows/validate-markdown.yml .github/workflows/validate-env.yml`
      — acceptance: all three files are listed (none has a concurrency block).
- [ ] [AI] **GREEN**: add the canonical concurrency block to
      `.github/workflows/pr-quality-gate.yml` — acceptance: `grep -A2 "concurrency:" pr-quality-gate.yml`
      shows the group + cancel-in-progress lines.
- [ ] [AI] **GREEN**: add the canonical concurrency block to
      `.github/workflows/validate-markdown.yml` — acceptance: block present.
- [ ] [AI] **GREEN**: add the canonical concurrency block to
      `.github/workflows/validate-env.yml` — acceptance: block present.
- [ ] [AI] **GREEN**: add the canonical concurrency block to each scheduled workflow:
      `test-and-deploy-ayokoding-web.yml`, `test-and-deploy-ose-web.yml`,
      `test-and-deploy-organiclever-web-development.yml`, `test-and-deploy-ose-app-web-development.yml`,
      `test-and-deploy-wahidyankf-web.yml`
      — acceptance: each file declares the concurrency block. Note: for these `schedule`+`push`
      workflows the group is keyed by `github.ref` and cancel-in-progress stays effectively off
      (PR-only), so scheduled runs are never cancelled.
- [ ] [AI] **REFACTOR**: confirm consistent placement (block sits after `permissions:` and before
      `jobs:` in every edited file) — acceptance: visual/grep consistency across all edited files.
- [ ] [AI] Lint all edited workflows: `actionlint .github/workflows/*.yml` if available, else
      `npx prettier --check .github/workflows/` — acceptance: exits 0.

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [ ] [AI] `grep -l "concurrency:" .github/workflows/*.yml | wc -l` — expected: at least 8 (PR gate + 2 validators + 5 scheduled). Confirm each targeted file is in the list.
- [ ] [AI] `grep -A2 "concurrency:" .github/workflows/pr-quality-gate.yml` shows
      `cancel-in-progress: ${{ github.event_name == 'pull_request' }}` — expected: exact canonical line.
- [ ] [AI] Workflows lint clean — expected: exits 0.
- [ ] [AI] Commit thematically: `rtk git commit -m "ci(workflows): add canonical concurrency groups"`.

> **Pause Safety**: every targeted workflow declares the canonical concurrency block, lints clean,
> and the change is committed. Safe to stop. To resume: re-run the `grep -l "concurrency:"` count
> and confirm the commit is present.

## Phase 3: Validator-set parity — `validate:gherkin-keyword-cardinality`

Create the `validate:gherkin-keyword-cardinality` Nx target wrapping the already-shipped
`rhino-cli repo-governance gherkin-keyword-cardinality` command, then wire it into
`validate-markdown.yml` (see
[tech-docs.md § D4](./tech-docs.md#d4--validategherkin-keyword-cardinality-nx-target)).

_Suggested executor: `swe-rust-dev`_

- [ ] [AI] **RED — target absent**:
      `npx nx run rhino-cli:validate:gherkin-keyword-cardinality`
      — acceptance: fails with an "target not found" / "cannot find configuration" error (the target
      does not exist yet).
- [ ] [AI] Pre-implementation research — confirm the subcommand path and args:
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- repo-governance gherkin-keyword-cardinality --help`
      — acceptance: help text prints; record the exact subcommand path and any required
      positional/flag arguments to bake into the GREEN step's target command string.
- [ ] [AI] **GREEN**: add the `validate:gherkin-keyword-cardinality` target to
      `apps/rhino-cli/project.json`, mirroring the existing `validate:specs-links` target shape
      (executor, `options.command`, `cache`, and `inputs` keyed to the relevant `.feature` / `.md`
      globs). Use the command confirmed above. Acceptance:
      `npx nx run rhino-cli:validate:gherkin-keyword-cardinality` now runs the audit.
- [ ] [AI] **GREEN — target passes on current tree**: re-run
      `npx nx run rhino-cli:validate:gherkin-keyword-cardinality`
      — acceptance: exits 0. If it surfaces preexisting cardinality violations, fix them at the
      source (root-cause orientation) until the target is green; do NOT disable the validator.
- [ ] [AI] **GREEN — wire into CI**: add a `Validate gherkin keyword cardinality` step to
      `.github/workflows/validate-markdown.yml` running
      `npx nx run rhino-cli:validate:gherkin-keyword-cardinality`, placed alongside the existing
      mermaid / links / heading-hierarchy steps — acceptance: the step is present after the Setup
      Rust step.
- [ ] [AI] **REFACTOR**: confirm the new target's `inputs` are scoped (so Nx caches correctly) and
      the workflow step ordering reads cleanly — acceptance:
      `npx nx run rhino-cli:validate:gherkin-keyword-cardinality` is cache-hit on a no-op re-run.
- [ ] [AI] Lint the workflow: `actionlint .github/workflows/validate-markdown.yml` if available,
      else `npx prettier --check .github/workflows/validate-markdown.yml` — acceptance: exits 0.

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [ ] [AI] `npx nx run rhino-cli:validate:gherkin-keyword-cardinality` — expected: exits 0 (green
      on the current tree).
- [ ] [AI] `grep "gherkin-keyword-cardinality" .github/workflows/validate-markdown.yml` — expected:
      the CI step is present.
- [ ] [AI] Workflow lints clean — expected: exits 0.
- [ ] [AI] Commit thematically: `rtk git commit -m "ci(validators): add gherkin-keyword-cardinality to markdown gate"`.

> **Pause Safety**: the new Nx target exists, passes on the current tree, and runs in
> `validate-markdown.yml`; the change is committed. Safe to stop. To resume: re-run the target and
> confirm green, then confirm the commit is present.

## Phase 4: Governance — `ci-conventions.md` converged + CI Parity Checklist

Bring `ci-conventions.md` into sync with the converged standard and add a **CI Parity Checklist**
section (see [tech-docs.md § D5](./tech-docs.md#d5--governance-alignment--ci-parity-checklist)).
Evaluate `ci-checker` for parity-check additions.

_Suggested executor: `repo-rules-maker`_

> _These are governance doc + agent-definition edits (non-code), so they use direct-action +
> acceptance criteria — not RED/GREEN/REFACTOR (per the TDD convention's non-code carve-out)._

- [ ] [AI] Confirm the gaps this phase fills: read
      `repo-governance/development/infra/ci-conventions.md` and verify it does not yet document
      `nx affected` as the per-language PR-gate standard, the canonical concurrency pattern, or a CI
      Parity Checklist — acceptance: none of those three are present.
- [ ] [AI] Update the GitHub Actions Conventions area of
      `ci-conventions.md` so the per-language PR-gate standard reads `nx affected` (not `run-many`),
      noting the single-project `specs-gate` exception — acceptance: the doc states `nx affected`
      for per-language jobs.
- [ ] [AI] Document the canonical concurrency block (group key +
      PR-only cancel-in-progress) in `ci-conventions.md` under the GitHub Actions Conventions area
      — acceptance: the canonical YAML block is present in the doc.
- [ ] [AI] Add a new `## CI Parity Checklist` section to
      `ci-conventions.md` enumerating the parity invariants (all per-language PR-gate jobs use
      `nx affected`; every PR/validator/scheduled workflow declares a concurrency group; both repos
      run the same validator set including `gherkin-keyword-cardinality`; action majors are current)
      AND recording the accepted deviations (runner target, .NET detection, npm flag, setup-docker)
      with their rationale — acceptance: the section lists invariants and deviations.
- [ ] [AI] Evaluate `.claude/agents/ci-checker.md` for parity checks (e.g., "concurrency group
      present", "no per-language `run-many`"). If they
      fit the agent's deterministic-check shape, add them to the Validation Checks list; otherwise
      record the decision not to in the plan notes — acceptance: an explicit add-or-skip decision is
      made and, if added, `ci-checker.md` lists the new checks.
- [ ] [AI] Run the doc validators on the edited governance file:
      `npx nx run rhino-cli:validate:links && npx nx run rhino-cli:validate:heading-hierarchy && npx nx run rhino-cli:validate:mermaid`
      — acceptance: all three exit 0 (heading nesting correct, links resolve, no mermaid issues).
- [ ] [AI] If `ci-checker.md` was edited, re-sync platform bindings: `npm run generate:bindings`
      — acceptance: exits 0; `.opencode/` / `.amazonq/` mirrors regenerated with no parity-guard
      failure.

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [ ] [AI] `grep -n "nx affected" repo-governance/development/infra/ci-conventions.md` — expected:
      the per-language PR-gate standard now reads `nx affected`.
- [ ] [AI] `grep -n "CI Parity Checklist" repo-governance/development/infra/ci-conventions.md` —
      expected: the new section heading is present.
- [ ] [AI] `npx nx run rhino-cli:validate:links` and `:validate:heading-hierarchy` and
      `:validate:mermaid` — expected: all exit 0.
- [ ] [AI] If bindings changed: `npm run generate:bindings` exits 0 with no parity-guard failure.
- [ ] [AI] Commit thematically:
      `rtk git commit -m "docs(ci-conventions): converge standard and add CI Parity Checklist"`
      (and a separate `chore(agents): add CI parity checks to ci-checker` commit if `ci-checker.md`
      changed).

> **Pause Safety**: `ci-conventions.md` describes the converged standard with a CI Parity
> Checklist, doc validators pass, bindings are in sync, and the changes are committed. Safe to stop.
> To resume: re-run the doc validators and confirm the commits are present.

## Phase 5: Final Quality Gate + Commit + Push + CI Verify + Archival

### Local Quality Gates (Before Push)

- [ ] [AI] Run affected typecheck: `npx nx affected -t typecheck` — exits 0.
- [ ] [AI] Run affected linting: `npx nx affected -t lint` — exits 0.
- [ ] [AI] Run affected quick tests: `npx nx affected -t test:quick` — exits 0.
- [ ] [AI] Run affected spec coverage: `npx nx affected -t spec-coverage` — exits 0.
- [ ] [AI] Run the full validator set locally:
      `npx nx run rhino-cli:validate:gherkin-keyword-cardinality` and
      `npx nx run rhino-cli:validate:links` and `:validate:mermaid` and `:validate:heading-hierarchy`
      and `:validate:env` — all exit 0.
- [ ] [AI] Lint all workflows: `actionlint .github/workflows/*.yml` if available, else
      `npx prettier --check .github/workflows/` — exits 0.

> **Important**: Fix ALL failures found during quality gates, not just those caused by your changes.
> This follows the root-cause-orientation principle — proactively fix preexisting errors
> encountered during work. Commit preexisting fixes separately with appropriate conventional commit
> messages.

### Commit Guidelines

- [ ] [AI] Commit changes thematically — group related changes into logically cohesive commits.
- [ ] [AI] Follow Conventional Commits format: `<type>(<scope>): <description>`.
- [ ] [AI] Split different domains/concerns into separate commits (workflows vs governance docs vs
      agent definitions).
- [ ] [AI] Preexisting fixes get their own commits, separate from plan work.

### Post-Push CI Verification

- [ ] [AI] Push changes to `main`: `rtk git push origin HEAD:main` (worktree-to-main, no PR).
- [ ] [AI] Monitor ALL GitHub Actions workflows triggered by the push, polling every 3 minutes via
      `gh run view --json status,conclusion` (do NOT use `gh run watch`).
- [ ] [AI] Verify ALL CI checks pass — no exceptions; confirm the new
      `gherkin-keyword-cardinality` step ran and is green.
- [ ] [AI] If any CI check fails, fix immediately and push a follow-up commit; repeat until ALL
      GitHub Actions pass with zero failures.
- [ ] [AI] Do NOT proceed to archival until CI is fully green.

### Plan Archival

- [ ] [AI] Verify ALL delivery checklist items are ticked.
- [ ] [AI] Verify ALL quality gates pass (local + CI).
- [ ] [AI] Rename and move:
      `git mv plans/in-progress/standardize-ci-parity/ plans/done/2026-06-11__standardize-ci-parity/`
      using the completion date (NOT the creation date) — adjust to the actual completion date at
      archival time.
- [ ] [AI] Update `plans/in-progress/README.md` — remove the plan entry.
- [ ] [AI] Update `plans/done/README.md` — add the plan entry with the completion date.
- [ ] [AI] Update any other READMEs that reference this plan (e.g., `plans/README.md`).
- [ ] [AI] Commit the archival: `rtk git commit -m "chore(plans): move standardize-ci-parity to done"`
      and push to `origin main`.

### Phase 5 Gate

> All checks below must pass to consider the plan complete.

- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — expected: exits 0.
- [ ] [AI] Full validator set runs green locally (gherkin-keyword-cardinality, links, mermaid,
      heading-hierarchy, env) — expected: all exit 0.
- [ ] [AI] `gh run view --json status,conclusion` on the latest `main` runs — expected: all
      `conclusion: success`.
- [ ] [AI] Plan folder moved under `plans/done/<completion-date>__standardize-ci-parity/`
      (`ls plans/done/ | grep standardize-ci-parity` returns exactly one dated entry) and the index
      READMEs are updated — expected: `git status` clean after the archival commit is pushed.

> **Pause Safety**: the standardized pipeline is live on `origin main`, all CI is green, and the
> plan is archived to `done/`. This is the terminal state. To resume verification: re-run the
> affected gate and `gh run view` on the latest `main` runs.
