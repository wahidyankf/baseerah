# Delivery — Standardize Repo Toolchain Parity (ose-public)

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.
>
> **Phase Gate** — every phase ends with a `### Phase N Gate` (must-pass verification) plus a
> `> **Pause Safety**:` note (the safe-to-stop state and the single command to resume). A phase
> is not complete until its gate is green; do not start phase N+1 while any gate check fails.

This checklist delivers **only ose-public's** convergence. Workstreams **A (CI), B (hooks),
E (target rename), F (governance docs)** are **parallel-safe** with the sibling plans (`ose-infra`,
`ose-primer`): the [Converged Toolchain Target](./tech-docs.md#converged-toolchain-target-shared-across-the-three-repo-sibling-set)
is a fixed static spec, so no sibling plan must finish first. Workstreams **C (rhino-cli hexagonal
arch, Phase 7), D (union commands, Phase 9), and G (Mermaid state-diagram validation, Phase 8) are
the REFERENCE**: ose-public authors them first; `ose-infra` and `ose-primer` port from ose-public —
nothing blocks ose-public's C/D/G. **G depends on C** — the Mermaid feature is migrated into its
hexagonal slice in Phase 7, then state-diagram support is added to that slice in Phase 8. Each step is
`[AI]` unless genuinely human-only. See
[tech-docs.md § Deviation Matrix](./tech-docs.md#deviation-matrix).

## Worktree

Worktree path: `worktrees/standardize-repo-toolchain-parity/`

Optional manual pre-provisioning (run from repo root):

```bash
claude --worktree standardize-repo-toolchain-parity
```

The plan-execution Step 0 gate enters this worktree by default: it auto-provisions from the latest
`origin/main` when missing, syncs with `origin/main` before implementing, and prompts before
deleting the worktree after the plan is archived and pushed.

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
[Plans Organization Convention § Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

## Phase 0: Environment Setup, Baseline, Prerequisite Verify, and Golden-Master Capture

> _Executor: repo-setup-manager_

This phase converges the toolchain, records the baseline, **hard-verifies the upstream prerequisite**
(`bootstrap-be-messaging-and-crane-media`), and **captures the golden-master CLI corpus** that
behavior-freezes the rhino-cli migration (Phases 7–8).

- [ ] [AI] Install dependencies in the root worktree: `npm install`
      — acceptance: exits 0, `node_modules/` synchronized.
- [ ] [AI] Converge the full polyglot toolchain: `npm run doctor -- --fix`
      — acceptance: exits 0 with no unresolved drift.
- [ ] [AI] Record the affected baseline: `npx nx affected -t typecheck lint test:quick spec-coverage`
      — acceptance: pass/fail count recorded; every preexisting failure documented.
      (Note: target is still `spec-coverage` until Phase 10 renames it to `spec:coverage`.)
- [ ] [AI] Resolve all preexisting failures before proceeding (root-cause orientation)
      — acceptance: no preexisting failures remain unresolved.
- [ ] [AI] **Prerequisite — `crane-be` exists**: `test -d apps/crane-be && echo OK`
      — acceptance: prints `OK`.
- [ ] [AI] **Prerequisite — GHCR publish workflow exists**:
      `ls .github/workflows/ | grep -Ei 'ghcr|publish|image' && echo OK`
      — acceptance: at least one matching workflow file is listed. If naming differs, confirm by
      reading the workflow for `ghcr.io/wahidyankf/crane-be`.
- [ ] [AI] **Prerequisite — .NET detection present**:
      `grep -E 'lang:fsharp|lang:csharp|has-dotnet' .github/workflows/pr-quality-gate.yml && echo OK`
      — acceptance: `.NET` detection lines are present in the PR gate.
- [ ] [AI] **Golden-master capture**: enumerate every `rhino-cli` subcommand
      (`cargo run --release --manifest-path apps/rhino-cli/Cargo.toml -- --help` then each
      subcommand's `--help`) and record, against a fixed input fixture set, the stdout/stderr/exit
      code of each invocation into a versioned corpus under
      `apps/rhino-cli/tests/golden-master/` (or the repo's existing test-fixtures location)
      — acceptance: a re-run of the capture produces a byte-identical corpus (deterministic);
      the corpus covers every subcommand listed by `--help`.
- [ ] [AI] Add a golden-master harness test that replays the corpus and diffs byte-for-byte
      — acceptance: `npx nx run rhino-cli:test:unit` (or the golden-master test target) is GREEN on
      the unmodified tree.

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [ ] [AI] `npm install` exited 0 and `npm run doctor -- --fix` reports no unresolved drift.
- [ ] [AI] Baseline recorded and every preexisting failure resolved (zero unresolved).
- [ ] [AI] All three prerequisite verifications printed `OK`. If any failed, STOP — the upstream
      prerequisite is not done and this plan must not proceed.
- [ ] [AI] Golden-master corpus captured, deterministic on re-capture, and the replay harness is
      GREEN.

> **Pause Safety**: only the local toolchain was verified, the baseline recorded, the prerequisite
> confirmed, and the golden-master corpus captured — no toolchain changes exist yet. Safe to stop
> indefinitely. To resume: re-run the baseline command, the three prerequisite greps, and the
> golden-master replay harness; confirm all still clean.

## Phase 1: CI — PR-gate `nx affected` Convergence + Go-Strip + Workflow Naming

Three concerns land in this phase: (1) replace `nx run-many` with `nx affected` for the per-language
jobs; (2) **strip Go from ose-public** (it has no Go code — see
[tech-docs.md Go-removal note](./tech-docs.md#ose-public-specific-reading-of-the-convergence-table)); and
(3) bring workflow **file names**, `name:` fields, and **job ids** onto the canonical
[BLOCK 1-A naming scheme](./tech-docs.md#a--ci-workflows) (see also
[§ D14](./tech-docs.md#d14--canonical-workflow--actions-name-scheme)).

Replace `nx run-many` with `nx affected` for the **.NET and Rust** per-language jobs in
`pr-quality-gate.yml`, keeping the identical target list and project-tag scoping. The TypeScript job
(already `nx affected`) and the single-project `specs-gate` `run-many` are left intact (see
[tech-docs.md § D1](./tech-docs.md#d1--converge-to-nx-affected-for-all-per-language-pr-gate-jobs)).
The **Go job is removed, not converted** — ose-public has no Go
([Repo-grounded: `git ls-files '*.go' ':!:archived/**'` → 0; `ayokoding-cli`/`ose-cli` are Rust]).

This phase applies the **affected-first PR-gate principle**: the PR gate runs `nx affected` for
**everything that is affected-computable** (per-language typecheck/lint/test/coverage and project-scoped
validators); a check runs whole-repository **only** where correctness requires repo-wide scope, and each
such exception is justified in the CI/toolchain Parity Checklist (Phase 11). See
[tech-docs.md § D13](./tech-docs.md#d13--affected-first-pr-gate-whole-repo-only-by-exception) for the
scope table. Any safely-affected check still run whole-repo is moved onto `nx affected` here.

_Suggested executor: `ci-fixer`_

- [ ] [AI] **RED**: assert `run-many` still present in the per-language jobs:
      `grep -n "nx run-many -t typecheck lint test:quick spec-coverage" .github/workflows/pr-quality-gate.yml`
      — acceptance: matches the Go, .NET, and Rust job lines (3 hits ~133/149/165). The
      single-project `specs-gate` `run-many` (~197) is separate and intentionally kept.
- [ ] [AI] **GREEN — strip the Go job entirely**: remove the `golang:` job, its
      `if: needs.detect.outputs.has-golang == 'true'` guard, the `./.github/actions/setup-golang` step,
      the `has-golang` output + the `lang:golang) ... has-golang=true` detection arm in the `detect`
      job, and the `golang` entry from `quality-gate.needs` in
      `.github/workflows/pr-quality-gate.yml`
      — acceptance: `grep -nE 'golang|has-golang|setup-golang|lang:golang' .github/workflows/pr-quality-gate.yml`
      returns nothing.
  - _Suggested executor: `ci-fixer`_
- [ ] [AI] **GREEN — drop Go from `rhino-cli doctor` (ose-public scope)**: remove Go from ose-public's
      required-tool scope in the doctor toolchain manifest / env-contract (the file the doctor reads for
      this repo's required tools — confirm exact path via
      `rtk grep -rln 'golang\|go.*toolchain\|"go"' apps/rhino-cli/ .tool-versions`), leaving Go in the
      shared doctor **binary** for infra/primer. Do NOT remove the Go capability from the doctor code
      itself — only ose-public's required-tool list
      — acceptance: `npm run doctor` no longer reports Go as required/missing for ose-public; infra/primer
      doctor scope is untouched.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: change the .NET job (`--projects='tag:lang:fsharp,tag:lang:csharp'`) to
      `nx affected` — acceptance: the .NET job uses `affected`.
- [ ] [AI] **GREEN**: change the Rust job (`--projects='tag:lang:rust'`) to `nx affected`; leave the
      subsequent `rhino-cli:fmt:check` / `deny:check` / `check:msrv` steps unchanged
      — acceptance: the Rust job uses `affected`; the three rhino-cli single-target steps remain.
- [ ] [AI] **GREEN — verify no per-language run-many remains**:
      `grep -n "nx run-many" .github/workflows/pr-quality-gate.yml`
      — acceptance: the only remaining match is the `specs-gate` `--projects=rhino-cli` line.
- [ ] [AI] **REFACTOR**: confirm each affected job retains its inline
      `NX_BASE`/`NX_HEAD` env block (`grep -n "NX_BASE\|NX_HEAD" .github/workflows/pr-quality-gate.yml`)
      — acceptance: every per-language affected job retains its SHA env block.
- [ ] [AI] **GREEN — affected-first sweep**: audit `pr-quality-gate.yml` (and the per-file lint jobs)
      for any check run whole-repo that is **safely affected/changed-file computable** — the per-file
      linters/validators (`shell`/`dockerfile`/`actions` lint, `mermaid`, `heading-hierarchy`) should
      be scoped to changed/affected files where computable; move any such check onto `nx affected` (or
      changed-file scoping). Leave the documented whole-repo exceptions (`links`, `specs:*` structural,
      `naming:*`, governance/parity, `gherkin`, `env`) whole-repo, per
      [tech-docs.md § D13](./tech-docs.md#d13--affected-first-pr-gate-whole-repo-only-by-exception)
      — acceptance: each remaining whole-repo check matches a justified row in the D13 scope table; no
      safely-affected check is left running whole-repo.
- [ ] [AI] **GREEN — workflow file / `name:` / job-id naming (BLOCK 1-A scheme)**: audit every
      `.github/workflows/*.yml` against the canonical scheme — **file** = kebab-case
      `<verb>-<noun>[-<qualifier>].yml`, **`name:`** = Title Case matching the file, **job ids** =
      kebab-case (`rtk grep -nE '^name:|^  [a-zA-Z0-9_-]+:' .github/workflows/*.yml`); `git mv` any
      non-conforming file name and update its `name:` field + any kebab-case-violating job id. The
      PR-gate aggregate job **keeps the branch-protection-required name `Quality gate`** (do NOT rename
      it — see the `[HUMAN]` step below)
      — acceptance: every workflow file is kebab-case `<verb>-<noun>`, every `name:` is Title Case
      matching the file, every job id is kebab-case, and `Quality gate` is unchanged.
  - _Suggested executor: `ci-fixer`_
- [ ] [AI] **GREEN — update workflow cross-references after any `git mv`**: if a workflow file was
      renamed, update every reference to its old filename (reusable-workflow `uses:` paths, badge URLs
      in READMEs, branch-protection notes in docs) —
      `rtk grep -rn '<old-workflow-filename>' .github docs repo-governance AGENTS.md`
      — acceptance: no reference to a renamed workflow's old filename remains.
- [ ] [HUMAN] **Branch-protection sync (only if a required-check job was renamed)**: if — and only if
      — any branch-protection **required-check** job (e.g. the `Quality gate` aggregate) was renamed in
      the step above, a human MUST update the required-check list in GitHub repo settings (Settings →
      Branches → `main` → required status checks) to the new job name; GitHub keys required checks by
      job name, so a renamed-but-green job silently stops satisfying the gate. The standing decision is
      to **keep `Quality gate` unchanged**, so this step is normally a no-op
      — handoff: the agent reports whether any required-check job name changed; the human confirms
      "branch-protection required checks updated to <new name>" (or "no required-check rename — no
      action") — observable resume signal: the human's confirmation message; the agent then re-checks
      that a test PR's `Quality gate` check still reports.

> **Note**: `[HUMAN]` because editing GitHub branch-protection settings is an out-of-band,
> privileged-authority action an agent cannot perform. It is normally a no-op (the required-check job
> is intentionally not renamed).

- [ ] [AI] Lint: `actionlint .github/workflows/pr-quality-gate.yml` if available, else
      `npx prettier --check .github/workflows/pr-quality-gate.yml` — acceptance: exits 0.

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [ ] [AI] `grep -c "nx affected -t typecheck lint test:quick spec-coverage" .github/workflows/pr-quality-gate.yml`
      — expected: at least 3 (TypeScript + .NET + Rust; **no Go job**).
- [ ] [AI] `grep -nE 'golang|has-golang|setup-golang|lang:golang' .github/workflows/pr-quality-gate.yml`
      — expected: empty (Go fully stripped).
- [ ] [AI] `grep "nx run-many" .github/workflows/pr-quality-gate.yml` — expected: only the
      `specs-gate` `--projects=rhino-cli` line remains.
- [ ] [AI] Every workflow file name is kebab-case `<verb>-<noun>`, every `name:` Title Case, every job
      id kebab-case; `Quality gate` aggregate name unchanged — expected: BLOCK 1-A scheme satisfied.
- [ ] [AI] `npm run doctor` no longer flags Go as required/missing for ose-public — expected: Go absent
      from ose-public's required-tool scope.
- [ ] [AI] Workflow lints clean — expected: exits 0.
- [ ] [AI] Commit thematically (split the affected convergence, the Go-strip, and the workflow rename
      into separate commits): e.g. `rtk git commit -m "ci(pr-gate): converge non-TS jobs to nx affected"`,
      `rtk git commit -m "ci(pr-gate): strip Go from ose-public (no Go code)"`,
      `rtk git commit -m "ci(workflows): normalize workflow file/name/job-id naming"`.

> **Pause Safety**: `pr-quality-gate.yml` is self-consistent (non-TS jobs on `nx affected`, Go fully
> stripped, workflow names canonical), all workflows lint clean, and the changes are committed. Safe to
> stop. To resume: re-run the affected-count, Go-strip, and naming grep checks and confirm the commits.

## Phase 2: CI — Canonical Concurrency Across All Workflows

Add the canonical concurrency block (see
[tech-docs.md § D3](./tech-docs.md#d3--canonical-concurrency-pattern)) to **every** workflow — the PR
gate, validator workflows, and scheduled `test-and-deploy-*` quartet. No ose-public workflow declares
a concurrency group today [Repo-grounded].

_Suggested executor: `ci-fixer`_

The canonical block (insert at top level, after `on:` / `permissions:`):

```yaml
concurrency:
  group: ${{ github.workflow }}-${{ github.event_name == 'pull_request' && github.event.pull_request.number || github.ref }}
  cancel-in-progress: ${{ github.event_name == 'pull_request' }}
```

- [ ] [AI] **RED**: assert no concurrency block exists across the targeted workflows:
      `grep -rL "concurrency:" .github/workflows/*.yml`
      — acceptance: every workflow file is listed (none has a concurrency block).
- [ ] [AI] **GREEN**: add the canonical block to `.github/workflows/pr-quality-gate.yml`
      — acceptance: `grep -A2 "concurrency:" pr-quality-gate.yml` shows the group + cancel lines.
- [ ] [AI] **GREEN**: add the block to `validate-markdown.yml` and `validate-env.yml`
      — acceptance: block present in both.
- [ ] [AI] **GREEN**: add the block to each scheduled workflow
      (`test-and-deploy-ayokoding-web.yml`, `test-and-deploy-ose-web.yml`,
      `test-and-deploy-organiclever-web-development.yml`,
      `test-and-deploy-ose-app-web-development.yml`, `test-and-deploy-wahidyankf-web.yml`)
      — acceptance: each declares the block; for these `schedule`+`push` workflows the group is keyed
      by `github.ref` and cancel-in-progress stays effectively off (PR-only).
- [ ] [AI] **REFACTOR**: confirm consistent placement (after `permissions:`, before `jobs:`)
      — acceptance: visual/grep consistency across all edited files.
- [ ] [AI] Lint all edited workflows — acceptance: exits 0.

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [ ] [AI] `grep -l "concurrency:" .github/workflows/*.yml | wc -l` — expected: at least 8.
- [ ] [AI] `grep -A2 "concurrency:" .github/workflows/pr-quality-gate.yml` shows
      `cancel-in-progress: ${{ github.event_name == 'pull_request' }}` — expected: exact canonical line.
- [ ] [AI] Workflows lint clean — expected: exits 0.
- [ ] [AI] Commit thematically: `rtk git commit -m "ci(workflows): add canonical concurrency groups"`.

> **Pause Safety**: every targeted workflow declares the canonical concurrency block, lints clean,
> and the change is committed. Safe to stop. To resume: re-run the count and confirm the commit.

## Phase 3: CI — Lint-Gate Job Rename to the Tool-Named Scheme

Rename the three category-named lint-gate jobs — `shell`, `dockerfile`, `actions` — to the converged
**tool-named** scheme `shellcheck`, `hadolint`, `actionlint`. **Pure rename** — same linters, same
thresholds, same file sets; only job identifiers change. Every reference moves with the rename
(`quality-gate.needs`; the "CI job" column of `cross-language-lint-strictness.md`) (see
[tech-docs.md § D6](./tech-docs.md#d6--lint-gate-job-rename-to-the-tool-named-scheme)).

_Suggested executor: `ci-fixer`_

- [ ] [AI] **RED**: `grep -nE '^  (shell|dockerfile|actions):' .github/workflows/pr-quality-gate.yml`
      — acceptance: matches the three job keys (~L66/78/92).
- [ ] [AI] **GREEN**: rename `shell:`→`shellcheck:`, `dockerfile:`→`hadolint:`, `actions:`→`actionlint:`
      — acceptance: the three new keys present; the three old keys gone.
- [ ] [AI] **GREEN — `quality-gate.needs`**: change the `needs:` list from
      `[..., shell, dockerfile, actions, ...]` to `[..., shellcheck, hadolint, actionlint, ...]`
      — acceptance: `grep -n "shell\|dockerfile\|actions" pr-quality-gate.yml` no longer matches the
      old job names as job keys or `needs` entries.
- [ ] [AI] **GREEN — governance doc "CI job" column**: in
      `repo-governance/development/quality/cross-language-lint-strictness.md` change the
      `shell`/`dockerfile`/`actions` job-name references to `shellcheck`/`hadolint`/`actionlint`
      — acceptance: the updated column uses the tool names; old category names no longer appear as
      CI-job references.
- [ ] [AI] **REFACTOR**: `grep -rnE '\b(shell|dockerfile|actions):' .github/workflows/` returns no
      lint-gate-job match and actionlint reports the `needs` graph consistent
      — acceptance: actionlint clean (or `prettier --check` fallback) and no stale references.
- [ ] [AI] Lint the workflow — acceptance: exits 0.

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [ ] [AI] `grep -nE '^  (shellcheck|hadolint|actionlint):' .github/workflows/pr-quality-gate.yml`
      — expected: all three new job keys present.
- [ ] [AI] `quality-gate` `needs:` lists `shellcheck, hadolint, actionlint` and not the old names.
- [ ] [AI] `grep -nE 'shellcheck|hadolint|actionlint' repo-governance/development/quality/cross-language-lint-strictness.md`
      — expected: the "CI job" column uses the tool-named jobs.
- [ ] [AI] Workflow lints clean — expected: exits 0.
- [ ] [AI] Commit: `rtk git commit -m "ci(pr-gate): rename lint jobs to tool-named scheme"`.

> **Pause Safety**: the three lint-gate jobs are renamed, `needs` and the governance doc reference
> them by tool name, the workflow lints clean, and the change is committed. Safe to stop. To resume:
> re-run the three grep checks and confirm the commit.

## Phase 4: CI — `gherkin:keyword-cardinality-validation` Target + Wiring

Create the Nx target **directly under the canonical `{domain}:{work}` name**
`gherkin:keyword-cardinality-validation`, wrapping the already-shipped
`rhino-cli repo-governance gherkin-keyword-cardinality` command, then wire it into
`validate-markdown.yml` (see [tech-docs.md § D4](./tech-docs.md#d4--gherkinkeyword-cardinality-validation-nx-target)).
Authoring it under the canonical name now means **no later rename in Phase 10**.

_Suggested executor: `swe-rust-dev`_

- [ ] [AI] **RED — target absent**: `npx nx run rhino-cli:gherkin:keyword-cardinality-validation`
      — acceptance: fails with "target not found" / "cannot find configuration".
- [ ] [AI] Pre-implementation research — confirm subcommand path + args:
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- repo-governance gherkin-keyword-cardinality --help`
      — acceptance: help prints; record the exact subcommand path + required args for the target
      command string.
- [ ] [AI] **GREEN**: add `gherkin:keyword-cardinality-validation` to `apps/rhino-cli/project.json`,
      mirroring the existing `validate:specs-links` target shape (executor, `options.command`,
      `cache`, `inputs` keyed to the relevant `.feature`/`.md` globs)
      — acceptance: `npx nx run rhino-cli:gherkin:keyword-cardinality-validation` now runs the audit.
- [ ] [AI] **GREEN — passes on current tree**: re-run the target
      — acceptance: exits 0. If it surfaces preexisting cardinality violations, fix them at the source
      (root-cause orientation); do NOT disable the validator.
- [ ] [AI] **GREEN — wire into CI**: add a `Validate gherkin keyword cardinality` step to
      `.github/workflows/validate-markdown.yml` running
      `npx nx run rhino-cli:gherkin:keyword-cardinality-validation`, alongside the existing
      mermaid/links/heading-hierarchy steps — acceptance: the step is present after Setup Rust.
- [ ] [AI] **REFACTOR**: confirm `inputs` scoping (correct caching) and step ordering
      — acceptance: a no-op re-run is a cache hit.
- [ ] [AI] Lint the workflow — acceptance: exits 0.

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [ ] [AI] `npx nx run rhino-cli:gherkin:keyword-cardinality-validation` — expected: exits 0.
- [ ] [AI] `grep "gherkin:keyword-cardinality-validation" .github/workflows/validate-markdown.yml`
      — expected: the CI step is present.
- [ ] [AI] Workflow lints clean — expected: exits 0.
- [ ] [AI] Commit: `rtk git commit -m "ci(validators): add gherkin keyword-cardinality target to markdown gate"`.

> **Pause Safety**: the canonical-named target exists, passes on the current tree, and runs in
> `validate-markdown.yml`; the change is committed. Safe to stop. To resume: re-run the target and
> confirm green, then confirm the commit.

## Phase 5: CI — Full Quality Gate on Push-to-Main + Scheduler Cadence

Add the **full quality gate on `push` to `main`** (today `pr-quality-gate.yml` is `pull_request`-only)
and confirm/align the governance scheduler cadence to 2× WIB (see
[tech-docs.md § D10](./tech-docs.md#d10--full-quality-gate-on-push-to-main)).

The push-to-main gate carries the **same affected-first discipline** as the PR gate
([tech-docs.md § D13](./tech-docs.md#d13--affected-first-pr-gate-whole-repo-only-by-exception)):
affected-computable checks run via `nx affected` (base resolved from the prior successful `main` SHA per
D2), and only the justified repo-wide checks run whole-repo.

> **Image-publishing (recorded deviation, not a convergence gap).** ose-public **keeps** its
> `publish-images.yml` → GHCR workflow — confirm it carries the canonical concurrency block (Phase 2)
> and the BLOCK 1-A naming (Phase 1). **ose-primer carries NO image-publishing workflow** — it is a
> demo/showcase template that ships no deployable images, so the absence is a recorded
> [Deviation Matrix](./tech-docs.md#deviation-matrix) entry, not a gap this plan or the primer sibling
> plan must close. Do not add an image-publishing workflow to ose-primer.

_Suggested executor: `ci-fixer`_

- [ ] [AI] **RED — push trigger absent**:
      `grep -nA4 "^on:" .github/workflows/pr-quality-gate.yml`
      — acceptance: the `on:` block triggers `pull_request` only (no `push: branches: [main]`).
- [ ] [AI] Decision step — choose the mechanism (per D2/D10): extend `pr-quality-gate.yml`'s `on:`
      to add `push: branches: [main]` (with the affected base computed for push events), OR add a
      thin caller workflow that runs the same gate on push. Record the choice inline in the workflow
      comment — acceptance: the chosen mechanism is documented in the workflow.
- [ ] [AI] **GREEN**: implement the chosen mechanism so the full gate runs on push to `main`
      — acceptance: the gate's `on:` (or the caller) includes `push: branches: [main]`; for push
      events the affected base resolves correctly (e.g. prior `main` SHA or full non-affected run).
- [ ] [AI] **GREEN — scheduler cadence**: confirm the governance/scheduled validators run twice-daily
      WIB (`0 23 * * *`, `0 11 * * *`); align any single-schedule workflow to the 2× cadence
      — acceptance: `grep -n "cron:" .github/workflows/*.yml` shows the 2× WIB cadence for governance
      schedulers (app-deploy schedules stay per-portfolio, documented in the deviation matrix).
- [ ] [AI] **REFACTOR**: ensure the push-gate path does not double-run on PR merge in a wasteful way
      (concurrency group from Phase 2 keys push runs by ref) — acceptance: no redundant concurrent
      push run.
- [ ] [AI] Lint all edited workflows — acceptance: exits 0.

### Phase 5 Gate

> All checks below must pass before starting Phase 6.

- [ ] [AI] `grep -nA4 "^on:" .github/workflows/pr-quality-gate.yml` (or the caller) shows
      `push: branches: [main]` — expected: present.
- [ ] [AI] Governance scheduler cadence is 2× WIB — expected: the two cron lines present.
- [ ] [AI] Workflows lint clean — expected: exits 0.
- [ ] [AI] Commit: `rtk git commit -m "ci(pr-gate): run full quality gate on push to main"`.

> **Pause Safety**: the full quality gate now runs on push to `main` and the scheduler cadence is
> aligned; workflows lint clean and the change is committed. Safe to stop. To resume: re-run the
> `on:` grep and the cron check, confirm the commit.

## Phase 6: Git Hooks — Converge to BLOCK 1-B Canonical

Converge `commit-msg`/`pre-commit`/`pre-push` to the canonical BLOCK 1-B lifecycle (see
[tech-docs.md § B](./tech-docs.md#b--git-hooks-canonical-identical-behavior) and
[§ D11](./tech-docs.md#d11--git-hook-convergence)). This phase introduces the **canonical hook
shape**; the pre-push target list is written to reference the renamed `{domain}:{work}` + `spec:coverage`
targets, which become real in Phase 10. To avoid the hook ever pointing at a non-existent target,
**keep the current target names in the hook here and re-point them in Phase 10** — see the gate note.

_Suggested executor: `ci-fixer`_

- [ ] [AI] **RED**: diff the current hooks against BLOCK 1-B:
      `cat .husky/commit-msg .husky/pre-commit .husky/pre-push`
      — acceptance: record which BLOCK 1-B elements are missing/divergent (build flag, lint-staged
      wiring, conditional validators, ordering).
- [ ] [AI] **GREEN — commit-msg**: ensure `commit-msg` is exactly
      `npx --no -- commitlint --edit "$1"` — acceptance: matches BLOCK 1-B.
- [ ] [AI] **GREEN — pre-commit**: ensure the order is
      `git-identity-check.sh` → `check-no-env-staged.sh` → canonical staged-file lint
      (`shellcheck`/`hadolint`/`actionlint` on staged files, graceful skip if absent) →
      `rhino-cli git pre-commit` built with `--release`
      — acceptance: pre-commit matches BLOCK 1-B order and uses the `--release` build.
- [ ] [AI] **GREEN — pre-push**: ensure pre-push runs `nx affected -t` with the BLOCK 1-B target set
      followed by `markdown:lint` → `env:validation` → the changed-path-gated conditionals
      (`naming:*-validation`, `governance:vendor-audit-validation`, `cross-vendor:parity-validation`,
      `harness:bindings-validation`, `shell`/`dockerfile`/`actions` lint). **Keep the
      currently-existing target names** (e.g. `spec-coverage`, `validate:specs-*`, `validate:env`)
      so the hook stays runnable; Phase 10 re-points them to the canonical names
      — acceptance: pre-push matches the BLOCK 1-B lifecycle shape; every target it references
      currently exists.
- [ ] [AI] **REFACTOR**: run a no-op commit + dry-run push in the worktree to confirm the hooks
      execute end-to-end without referencing a missing target
      — acceptance: hooks run clean on a trivial change.
- [ ] [AI] Lint the hook shell scripts: `shellcheck .husky/*` if available
      — acceptance: exits 0 (warning threshold).

### Phase 6 Gate

> All checks below must pass before starting Phase 7.

- [ ] [AI] `commit-msg`/`pre-commit`/`pre-push` match the BLOCK 1-B lifecycle shape.
- [ ] [AI] Every target the hooks reference **currently exists** (no forward reference to a
      not-yet-renamed target) — expected: a dry-run push runs clean. **NOTE for Phase 10**: the
      target-name re-point in the hooks happens in Phase 10, atomically with the project.json renames.
- [ ] [AI] `shellcheck .husky/*` clean — expected: exits 0.
- [ ] [AI] Commit: `rtk git commit -m "chore(hooks): converge git hooks to canonical lifecycle"`.

> **Pause Safety**: the hooks match the canonical lifecycle and reference only existing targets;
> hooks run clean. Safe to stop. To resume: re-run a dry-run push and confirm the commit.

## Phase 7: rhino-cli Hexagonal Migration (REFERENCE — sub-phased, golden-frozen)

> **REFERENCE WORKSTREAM C.** ose-public authors the hexagonal migration in full; `ose-infra` and
> `ose-primer` port the identical crate structure from here. Behavior is **frozen** — the Phase 0
> golden-master corpus must stay byte-identical through every sub-phase (see
> [tech-docs.md § Hexagonal Architecture Design](./tech-docs.md#hexagonal-architecture-design-rhino-cli--reference-migration)
> and [§ Golden-master CLI suite](./tech-docs.md#golden-master-cli-suite-rhino-cli-migration)).

_Suggested executor: `swe-rust-dev`_

Each feature moves through the state lifecycle below; a feature is only `Done` once its golden-master
replay is byte-identical and coverage is met (any drift returns it to `Verifying`):

```mermaid
%% Feature migration state lifecycle through hexagonal migration
stateDiagram-v2
  [*] --> Flat
  Flat --> CoreExtracted: extract pure core
  CoreExtracted --> PortsDefined: define ports
  PortsDefined --> AdaptersWired: implement + wire
  AdaptersWired --> Verifying: replay + coverage
  Verifying --> AdaptersWired: drift or fail
  Verifying --> Done: byte-identical + green
  Done --> [*]
```

### Phase 7a — Shared kernel (`mermaid`, `cliout`)

> The **Mermaid feature migrates here**, as the shared-kernel slice (workstream G prerequisite). It
> moves **once**, straight into hexagonal layers — there is NO intermediate 8-file flat split (see
> [tech-docs.md § BLOCK 4 Mermaid slice](./tech-docs.md#hexagonal-architecture-design-rhino-cli--reference-migration)).
> Behavior is byte-for-byte preserved: every existing flowchart test stays green and `state.rs` is a
> stub at this stage (state behavior lands in Phase 8).

- [ ] [AI] **RED**: golden-master replay harness GREEN on the unmodified tree
      — acceptance: corpus diff empty (precondition for any move).
- [ ] [AI] **GREEN**: move the shared-kernel modules (`mermaid`, `cliout`, and any 2+-consumer helper
      currently in `src/internal/`) into `src/domain/<kernel>/` (pure) with the outbound ports they
      need defined in `src/application/` — acceptance: `cargo build` clean; modules compile in the
      new location.
- [ ] [AI] **GREEN — Mermaid slice**: migrate `apps/rhino-cli/src/internal/mermaid.rs` straight into
      the hexagonal layers — `domain/mermaid/` holds the kind-agnostic core (`ParsedDiagram`/`Node`/
      `Edge`/`Subgraph` types, the rank/width/depth `graph` computation, the width/label `validator`
      rules) plus the pure front-end parsers (the existing `flowchart` parser; a `state.rs` **stub**
      that returns an empty `ParsedDiagram` for now); `application/mermaid/` holds the validate use
      case + an extractor **port**; `infrastructure/mermaid/` holds the markdown-extractor adapter +
      the text/JSON `reporter` adapter; `commands/` keeps the `docs validate-mermaid` inbound adapter.
      Run `npx nx run rhino-cli:test:unit`
      — acceptance: `cargo build` clean; every existing flowchart test stays green; the `state.rs`
      stub compiles but adds no behavior.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: re-run golden-master replay + `npx nx run rhino-cli:test:unit`
      — acceptance: corpus byte-identical; unit tests GREEN; coverage threshold met (update the
      coverage-ignore allowlist if a file moved).
- [ ] [AI] Commit: `rtk git commit -m "refactor(rhino-cli): extract shared kernel + migrate mermaid slice to hexagonal domain"`.

### Phase 7b — Pilot feature (`git`)

- [ ] [AI] **RED**: golden-master GREEN; identify `git`'s IO boundaries (already injects via `Deps`)
      — acceptance: precondition confirmed.
- [ ] [AI] **GREEN**: extract `git`'s pure core to `domain/git/`, define inbound + outbound ports in
      `application/git/`, implement adapters in `infrastructure/git/`, wire `commands/git_*` to the
      use case — acceptance: `cargo build` clean; the `git` command runs.
- [ ] [AI] **REFACTOR**: golden-master replay + unit/integration/coverage
      — acceptance: corpus byte-identical; tests GREEN; coverage met.
- [ ] [AI] Commit: `rtk git commit -m "refactor(rhino-cli): migrate git feature to hexagonal layout"`.

### Phase 7c — IO-heavy features (envbackup, doctor, testcoverage)

- [ ] [AI] For each of `env_*`, `doctor`, `test_coverage_*`: apply the BLOCK 4 six-step recipe
      (golden-master GREEN → extract pure core → define ports → implement adapters → wire commands →
      re-run golden-master + tests/coverage) — acceptance: after each feature the corpus is
      byte-identical and tests/coverage are GREEN.
- [ ] [AI] Commit each feature (or coherent group) thematically:
      `rtk git commit -m "refactor(rhino-cli): migrate <feature> to hexagonal layout"`.

### Phase 7d — Lighter validators (docs/specs/naming/governance groups)

- [ ] [AI] Group-migrate the remaining lighter validator features (`docs_*`, `specs_*`,
      `*_validate_naming`, `governance_*`) applying the six-step recipe per group
      — acceptance: corpus byte-identical and tests/coverage GREEN after each group.
- [ ] [AI] Commit each group thematically.

### Phase 7 Gate

> All checks below must pass before starting Phase 8.

- [ ] [AI] `ls apps/rhino-cli/src/` shows `domain/`, `application/`, `infrastructure/`, `commands/`
      — expected: the four hexagonal layers present; `src/internal/` emptied/removed (or only
      truly-internal non-domain glue remains, documented).
- [ ] [AI] `ls apps/rhino-cli/src/domain/mermaid/` shows the migrated Mermaid slice including the
      `state.rs` **stub** — expected: the kind-agnostic core + flowchart parser + `state.rs` stub
      present; `src/internal/mermaid.rs` removed.
- [ ] [AI] Golden-master replay harness — expected: corpus byte-identical to the Phase 0 baseline.
- [ ] [AI] `npx nx run rhino-cli:test:unit` and `:lint` (clippy `-D warnings`) — expected: GREEN
      (every existing flowchart test stays green).
- [ ] [AI] Coverage threshold met; coverage-ignore allowlist updated for every moved file.
- [ ] [AI] All sub-phase commits present.

> **Pause Safety**: every committed sub-phase leaves the golden-master corpus byte-identical, so the
> CLI's observable behavior is unchanged at each checkpoint — safe to stop between sub-phases. The
> Mermaid feature is now a hexagonal slice with a `state.rs` stub; no state behavior yet. To resume:
> re-run the golden-master replay and `:test:unit`, confirm the last sub-phase commit.

## Phase 8: Mermaid State-Diagram Validation (REFERENCE — `state.rs` + golden corpus + D-CLEAN)

> **REFERENCE WORKSTREAM G.** ose-public authors the `state.rs` front-end + the shared golden corpus;
> `ose-infra` and `ose-primer` mirror the identical parser semantics + byte-identical fixtures.
> **Depends on Phase 7's Mermaid slice** — state support is a second pure front-end (`state.rs` in
> `domain/mermaid/`) feeding the same kind-agnostic `ParsedDiagram` the flowchart parser emits, so the
> width/label core is unchanged beyond wiring state edges through the width axis (see
> [tech-docs.md § Mermaid State-Diagram Validation Design](./tech-docs.md#mermaid-state-diagram-validation-design-workstream-g)
> and the ported Gherkin scenarios in [prd.md § Workstream G](./prd.md#workstream-g--mermaid-state-diagram-validation-acceptance-criteria)).
> **Target name note**: this phase precedes the Phase 10 rename, so it uses the **current** target
> name `validate:mermaid` (renamed to `mermaid:validation` in Phase 10). No gate wiring changes —
> state diagrams stop being skipped because the kind-detector recognizes their header.

_Suggested executor: `swe-rust-dev`_

### Phase 8a — State header detection + parser

- [ ] [AI] **RED**: add a unit test in `apps/rhino-cli/src/domain/mermaid/diagram.rs` asserting the
      kind detector returns `State` for both `stateDiagram-v2` and `stateDiagram` (v1) headers. Run
      `npx nx run rhino-cli:test:unit`
      — acceptance: test FAILS (the Phase 7 stub still maps state headers to an empty parse / wrong
      kind).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: implement state-header detection in `domain/mermaid/diagram.rs` for
      `stateDiagram-v2` and `stateDiagram`. Run `npx nx run rhino-cli:test:unit`
      — acceptance: the detection test passes; flowchart detection unchanged.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **RED**: add a unit test in `apps/rhino-cli/src/domain/mermaid/state.rs` parsing an
      11-state `direction LR` chain and asserting 11 `Node`s with the chain shape. Run
      `npx nx run rhino-cli:test:unit`
      — acceptance: test FAILS (the `state.rs` stub returns an empty `ParsedDiagram`).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: implement the `state.rs` parser per the
      [tech-docs.md pinned grammar facts](./tech-docs.md#mermaid-state-diagram-validation-design-workstream-g)
      — bare ids, `id : desc`, `state "desc" as id`, `[*]`, stereotype states (`<<choice>>`/
      `<<fork>>`/`<<join>>` and `[[...]]`) as `Node`s; `A --> B : lbl` as `Edge`; composite
      `state X { }` as `Subgraph` (recursed); skip notes/comments/`--`; match `-->` before `--`;
      `direction` accepts `TB|BT|LR|RL` only (reject `TD`). Run `npx nx run rhino-cli:test:unit`
      — acceptance: the 11-node parse test passes.
  - _Suggested executor: `swe-rust-dev`_

### Phase 8b — Width + label rules over the shared core

- [ ] [AI] **RED**: add a unit test asserting the 11-state `direction LR` chain yields a
      `width_exceeded` violation with width 11 through the validate use case. Run
      `npx nx run rhino-cli:test:unit`
      — acceptance: test FAILS (state edges not yet fed to the shared `graph` width core).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: wire the state `ParsedDiagram` through the shared `domain/mermaid/` width core
      so `LR`/`RL` map to the depth-as-horizontal axis like flowcharts. Run
      `npx nx run rhino-cli:test:unit`
      — acceptance: the `width_exceeded` width-11 test passes.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **RED**: add unit tests in `domain/mermaid/validator.rs` for label rules — a `>30`-char
      state display label and a `>30`-char transition label (`A --> B : <long>`) each yield
      `label_too_long`; a short colon label yields none. Run `npx nx run rhino-cli:test:unit`
      — acceptance: tests FAIL (transition-label check absent).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: extend `domain/mermaid/validator.rs` to check both state display labels and
      transition-edge labels against `max_label_len` using the existing `effective_label_len`
      per-segment measure [Repo-grounded: `effective_label_len` at
      `apps/rhino-cli/src/internal/mermaid.rs:670` pre-migration; now in the migrated slice]. Run
      `npx nx run rhino-cli:test:unit`
      — acceptance: all three label tests pass.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **RED**: add unit tests for structure-to-width — a rank holding `[*]`, `<<choice>>`,
      `<<fork>>`, `<<join>>` plus one more yields `width_exceeded` (5 nodes); a composite
      `state Outer { Inner1 --> Inner2 }` is recorded as a `Subgraph`. Run
      `npx nx run rhino-cli:test:unit`
      — acceptance: tests FAIL.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: implement pseudostate/stereotype node-counting and composite-as-subgraph
      recursion in `state.rs`. Run `npx nx run rhino-cli:test:unit`
      — acceptance: both tests pass.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **RED**: add a unit test asserting a block with a multiline `note right of X ... end note`,
      a `%%` comment, and a `--` separator produces zero violations and zero spurious nodes. Run
      `npx nx run rhino-cli:test:unit`
      — acceptance: test FAILS.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: implement note/comment/`--` skipping in `state.rs`. Run
      `npx nx run rhino-cli:test:unit`
      — acceptance: the free-text test passes (note text exempt from the label rule).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: deduplicate any shared parsing helpers between the flowchart parser and
      `state.rs` into a small shared util in `domain/mermaid/diagram.rs`; run `cargo fmt`. Run
      `npx nx run rhino-cli:lint && npx nx run rhino-cli:test:unit`
      — acceptance: lint exits 0 (clippy `-D warnings`); all tests pass.
  - _Suggested executor: `swe-rust-dev`_

### Phase 8c — Shared golden corpus (the parity lock)

- [ ] [AI] **RED**: add the corpus test harness under `apps/rhino-cli/tests/` (confirm the exact
      subdir against the existing `tests/**/*.rs` layout — e.g.
      `apps/rhino-cli/tests/mermaid_golden_corpus.rs`) that iterates over fixture `.md` files in a
      `fixtures/state/` subdirectory and asserts actual violation JSON equals expected JSON companion
      files. Run `npx nx run rhino-cli:test:unit`
      — acceptance: test FAILS because the fixture directory is empty or absent.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: land the shared golden corpus — create fixture `.md` files + expected violation
      JSON under `apps/rhino-cli/tests/` covering over-wide LR chain, compliant narrow chain, long
      state label, long transition label, `[*]`/stereotype counting, composite-as-subgraph, and
      note/comment/`--` exemption; the corpus test asserts each fixture's actual violations equal its
      expected JSON. Run `npx nx run rhino-cli:test:unit`
      — acceptance: the corpus test passes; **this exact fixture set is the one mirrored byte-identical
      to `ose-primer` and `ose-infra`**.
  - _Suggested executor: `swe-rust-dev`_

### Phase 8d — Aggressive repo-wide state-diagram cleanup (D-CLEAN)

> Per D-CLEAN, fix every violating state diagram repo-wide INCLUDING `plans/done/` and otherwise
> gate-excluded paths (maximum hygiene; diagram-only edits).

- [ ] [AI] Enumerate every violating state diagram: run the validator without exclusions —
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-mermaid`
      and additionally scan `plans/done` and excluded paths explicitly (no `--exclude` flags)
      — acceptance: a complete list of `width_exceeded`/`label_too_long` state-diagram findings is
      produced.
- [ ] [AI] Fix each `width_exceeded` state diagram using the width-fix strategies in
      `repo-governance/conventions/formatting/diagrams.md §Width Violation Fix Strategy Guide`
      (direction flip, sequential chaining, splitting) — edit each offending `.md` file
      — acceptance: re-running the validator on each fixed file reports no `width_exceeded`.
- [ ] [AI] Fix each `label_too_long` state diagram by shortening state/transition labels per
      `§Strategy 4 — Label Shortening` — edit each offending `.md` file
      — acceptance: re-running the validator on each fixed file reports no `label_too_long`.
- [ ] [AI] Verify the gate-scoped scan is clean: `npx nx run rhino-cli:validate:mermaid`
      — acceptance: zero state-diagram violations in gate scope.
- [ ] [AI] Verify the full repo-wide scan (including `plans/done`) is clean:
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-mermaid`
      — acceptance: zero state-diagram violations anywhere.

### Local Quality Gates (Before Push) — Phase 8

- [ ] [AI] Run affected typecheck: `npx nx affected -t typecheck`.
- [ ] [AI] Run affected linting: `npx nx affected -t lint`.
- [ ] [AI] Run affected quick tests: `npx nx affected -t test:quick`
      — acceptance: rhino-cli library coverage stays `≥90`.
- [ ] [AI] Run affected spec coverage: `npx nx affected -t spec-coverage`
      (target still `spec-coverage` until Phase 10 renames it to `spec:coverage`).
- [ ] [AI] Run `npm run lint:md` — acceptance: exits 0, no markdownlint violations in edited files.
- [ ] [AI] Run `npx nx run rhino-cli:validate:links` — acceptance: exits 0, no broken links introduced
      by the cleanup edits.
- [ ] [AI] Fix ALL failures — including preexisting issues not caused by your changes; re-run to
      confirm zero failures before pushing.

> **Important**: Fix ALL failures found during quality gates, not just those caused by your changes
> (root-cause orientation). Commit preexisting fixes separately with appropriate conventional commit
> messages.

### Commit Guidelines — Phase 8

- [ ] [AI] Commit the state front-end thematically:
      `rtk git commit -m "feat(rhino-cli): validate mermaid state diagrams"`.
- [ ] [AI] Keep the golden corpus in its own commit:
      `rtk git commit -m "test(rhino-cli): add shared state-diagram golden corpus"`.
- [ ] [AI] Keep the D-CLEAN repo-wide cleanup in its own commit (split by domain if it spans many):
      `rtk git commit -m "docs: fix over-wide and over-long mermaid state diagrams"`.

### Phase 8 Gate

> All checks below must pass before starting Phase 9.

- [ ] [AI] `npx nx run rhino-cli:test:unit` — expected: all new state tests + every preexisting
      flowchart test pass.
- [ ] [AI] `npx nx run rhino-cli:test:quick` — expected: coverage `≥90`, exits 0.
- [ ] [AI] `npx nx run rhino-cli:lint` — expected: exits 0 (clippy `-D warnings`).
- [ ] [AI] `npx nx run rhino-cli:validate:mermaid` — expected: exits 0, zero state-diagram violations
      in gate scope.
- [ ] [AI] `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-mermaid`
      — expected: zero state-diagram violations repo-wide including `plans/done`.
- [ ] [AI] Golden-master replay — expected: flowchart behavior byte-identical (state support is
      additive; the corpus extends but existing flowchart entries are unchanged).
- [ ] [AI] All Phase 8 commits present.

> **Pause Safety**: the migrated Mermaid slice now parses and validates state diagrams, every state
> diagram repo-wide is compliant, and flowchart behavior is unchanged; the gate wiring is untouched
> (still `validate:mermaid` until Phase 10). Safe to stop. To resume: `npx nx run rhino-cli:test:unit`
> and `npx nx run rhino-cli:validate:mermaid`, confirm the Phase 8 commits.

## Phase 9: rhino-cli Union Commands — Rationalize Surface, Verb-First Rename, then Add `Java` + `Contracts`

> **REFERENCE WORKSTREAM D.** Three parts: **9a** rationalizes the existing surface (merge overlaps,
> delete unused subcommands per the catalogued dispositions); **9b** renames every subcommand to the
> **verb-first git-style** scheme (BLOCK 11) and updates all callers + the golden-master corpus; then
> **9c** ports the `Java` and `Contracts` subcommands from the infra/primer reference implementations
> **into the hexagonal layout** (after Phase 7), so the CLI surface is the **rationalized + verb-first**
> union superset (see [tech-docs.md § D8](./tech-docs.md#d8--union-command-surface-add-java--contracts)
> and [§ (a-ter) verb-first rename](./tech-docs.md#a-ter-rhino-cli-verb-first-subcommand-rename-beforeafter)).

_Suggested executor: `swe-rust-dev`_

### Phase 9a — Command rationalization pass (keep / merge / delete, before the port)

> Resolve the overlap/deletion shortlist in
> [tech-docs.md § (a-bis)](./tech-docs.md#a-bis-command-surface-rationalization--overlap--deletion-candidates)
> and [§ D8](./tech-docs.md#d8--union-command-surface-add-java--contracts) BEFORE porting Java/Contracts,
> so the union lands against the rationalized surface. Reference-first: ose-public decides; infra/primer
> mirror. Any surface change (merge that renames a subcommand, or a deletion) is a **deliberate
> golden-master update** — update the frozen corpus entry in the same step and note it in the commit.

- [ ] [AI] **`env init`/`backup`/`restore` — KEEP verdict (no longer delete-candidates)**: these
      manage `.env` secret files (create from `.env.example`, back up, restore) and are **KEPT** per
      [tech-docs.md § (a-bis)](./tech-docs.md#a-bis-command-surface-rationalization--overlap--deletion-candidates)
      and [§ D8](./tech-docs.md#d8--union-command-surface-add-java--contracts). Do **not** remove them
      — record the KEEP rationale ("manage `.env` secret files") in the rationalization notes
      — acceptance: `rhino-cli env --help` still lists `init`/`backup`/`restore`/`validate`; no env
      subcommand removed; golden-master `env` entries unchanged.
- [ ] [AI] **Usage check (residual delete-candidate)**: confirm whether `test-coverage diff` /
      `test-coverage merge` have a live caller (Nx may handle coverage merge natively) —
      `rtk grep -rn 'test-coverage (diff|merge)' .github .husky package.json apps/*/project.json repo-governance docs`
      — acceptance: a written keep/delete verdict for `diff`/`merge` with the grep evidence (this is
      the only remaining evaluate; if no caller, delete the CLI variants + dispatch arms + modules +
      tests and drop their golden-master entries; if a caller exists, record "kept — caller at <path>").
- [ ] [AI] **Merge — link engine**: make `specs validate-links` and the `links:validation` target
      reuse the `docs validate-links` resolver (one link-resolution core; no duplicated logic)
      — acceptance: behavior unchanged (golden-master + corpus identical); the duplicate logic is gone.
- [ ] [AI] **Merge — filename-convention core**: extract the shared kebab-case filename pass used by
      `docs`/`agents`/`workflows` `validate-naming` into one core in `domain/`; each keeps its
      domain-specific rule (agent mirror parity, workflow frontmatter-name) layered on top
      — acceptance: all three `validate-naming` outputs byte-identical to baseline.
- [ ] [AI] **Merge — binding generation**: collapse `agents sync` (+OpenCode) and `agents emit-bindings`
      (+Amazon Q) into one `agents generate-bindings` with per-harness flags (keep thin aliases only if
      a caller needs them); `npm run generate:bindings` calls the merged command
      — acceptance: `.opencode/` + `.amazonq/` regenerate byte-identically; golden-master updated for
      the surface change.
- [ ] [AI] **Merge — binding parity**: consolidate `agents validate-sync` + `validate-bindings` +
      `validate-claude` (and the `cross-vendor:parity-validation` / `harness:bindings-validation`
      target logic) into one binding-parity validator family with per-harness arms
      — acceptance: each parity check still runs; one shared implementation; outputs unchanged.
- [ ] [AI] **Merge — governance audit sharing**: ensure `repo-governance audit` and the nine granular
      audit subcommands share one rule implementation each (no duplicated rule bodies)
      — acceptance: `audit` envelope == union of the granular outputs; no rule logic duplicated.
- [ ] [AI] **Merge — frontmatter parse**: `docs validate-frontmatter` and
      `repo-governance frontmatter-audit` share one frontmatter parse; the two distinct rules stay
      — acceptance: both validators' outputs unchanged; one parse path.
- [ ] [AI] Commit the rationalization separately:
      `rtk git commit -m "refactor(rhino-cli): rationalize command surface (merge overlaps, drop unused env utils)"`.

### Phase 9b — Verb-first git-style subcommand rename (BLOCK 11)

> Rename every subcommand to the **verb-first git-style** `<group> <verb> [<object>]` scheme per
> [tech-docs.md § (a-ter) BLOCK 11](./tech-docs.md#a-ter-rhino-cli-verb-first-subcommand-rename-beforeafter)
> (e.g. `docs validate-mermaid` → `docs validate mermaid`, `repo-governance vendor-audit` →
> `repo-governance audit vendor`, `agents sync` → `agents sync opencode`, `agents emit-bindings` →
> `agents emit amazonq`, `specs validate-tree` → `specs validate tree`). Top-level groups are
> **unchanged**. `env init`/`backup`/`restore`/`validate` and `git pre-commit` are already verb-first
> (unchanged). This is a **deliberate divergence** from the object-verb `{domain}:{work}` Nx target
> scheme — the CLI optimizes for natural typing, the targets for namespaced grouping. The subcommand
> surface change is a **deliberate golden-master corpus update** (re-capture the renamed invocations).
> Reference-first: ose-public renames; infra/primer mirror the identical surface.

- [ ] [AI] **RED**: add/extend a CLI-surface test asserting the **new** verb-first invocations resolve
      (e.g. parse `docs validate mermaid`, `repo-governance audit vendor`, `agents sync opencode`) and
      the old hyphenated forms (`docs validate-mermaid`, `repo-governance vendor-audit`, `agents sync`)
      no longer parse. Run `npx nx run rhino-cli:test:unit`
      — acceptance: test FAILS (the clap command tree still uses the old hyphenated subcommands).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN — rename the clap command tree**: in `apps/rhino-cli/src/commands/` (post-Phase-7
      hexagonal layout) rename every `*Commands` enum variant + its clap attributes to the verb-first
      scheme per the BLOCK 11 table — `docs`, `agents`, `workflows`, `specs`, `ddd`, `repo-governance`,
      `java`, `contracts` groups; `git`/`env`/`doctor` unchanged. Run `npx nx run rhino-cli:test:unit`
      — acceptance: the new-invocation parse test passes; old forms rejected.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN — update ALL callers**: re-point every invocation of a renamed subcommand in Nx
      `project.json` target `options.command` strings (`apps/*/project.json`, `libs/*/project.json`),
      `.husky/*` hooks (note: `rhino-cli git pre-commit` is unchanged, but any renamed invocation in a
      hook changes), `package.json` scripts, and docs that show the old command form —
      `rtk grep -rn 'docs validate-|agents sync|agents emit-bindings|vendor-audit|validate-tree|validate-naming|validate-counts|validate-adoption|validate-annotations|java-clean-imports|dart-scaffold' .husky .github package.json apps/*/project.json libs/*/project.json repo-governance docs AGENTS.md`
      then rewrite each hit to the verb-first form
      — acceptance: the grep returns no old-form invocation in any caller (docs prose examples updated too).
- [ ] [AI] **GREEN — update the golden-master corpus**: re-capture the renamed subcommand invocations
      into the golden-master corpus (the surface change is a **deliberate** corpus update, not drift) —
      record the old→new mapping in the commit body
      — acceptance: the corpus replay is GREEN against the renamed surface; every renamed invocation has
      a corpus entry; no **unrenamed** entry silently changed.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: confirm the controlled verb vocabulary (`validate`, `audit`, `detect`, `sync`,
      `emit`, `clean`, `scaffold`, `diff`, `merge`, `init`, `backup`, `restore`, `pre-commit`, `doctor`)
      is the complete set after rename; `cargo fmt`; run `npx nx run rhino-cli:lint && npx nx run rhino-cli:test:unit`
      — acceptance: lint exits 0 (clippy `-D warnings`); all tests pass; no stray verb outside the vocabulary.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] Commit the rename separately:
      `rtk git commit -m "refactor(rhino-cli)!: rename subcommands to verb-first git-style surface"`.

### Phase 9c — Port the union additions (`Java` + `Contracts`)

> The two new groups land in the **already-renamed verb-first surface** (Phase 9b ran first), so
> `Java` is added as `java validate annotations` and `Contracts` as `contracts clean java-imports`
> and `contracts scaffold dart` (per the BLOCK 11 after-column), not the old hyphenated forms.

- [ ] [AI] **RED**: assert the subcommands are absent:
      `cargo run --release --manifest-path apps/rhino-cli/Cargo.toml -- --help | grep -Ei 'java|contracts'`
      — acceptance: no match (neither subcommand exists).
- [ ] [AI] Port-research — read the infra/primer `Java` and `Contracts` reference implementations
      (cited by path; reader not assumed to have private-repo access) and the union-surface spec in
      BLOCK 1-D — acceptance: the expected subcommand surface (args, output) is recorded.
- [ ] [AI] **GREEN — `Java`**: add the `Java` subcommand in the hexagonal layout
      (`domain/java/` + `application/java/` ports + `infrastructure/java/` adapters +
      `commands/java_*`) with the verb-first surface `java validate annotations`, behavior matching the
      reference — acceptance: `rhino-cli java validate annotations --help` works; on ose-public (no JVM
      project) detection is a documented no-op.
- [ ] [AI] **GREEN — `Contracts`**: add the `Contracts` subcommand similarly with the verb-first
      surface `contracts clean java-imports` + `contracts scaffold dart`
      — acceptance: `rhino-cli contracts --help` lists `clean` and `scaffold`.
- [ ] [AI] **GREEN — extend golden-master**: capture the new subcommands into the golden-master corpus
      (this is an additive corpus extension, not a change to existing entries)
      — acceptance: existing corpus entries unchanged; new `java`/`contracts` entries recorded.
- [ ] [AI] **REFACTOR**: unit tests for the two new commands + clippy `-D warnings`
      — acceptance: `:test:unit` and `:lint` GREEN; coverage met.
- [ ] [AI] Commit: `rtk git commit -m "feat(rhino-cli): add Java and Contracts subcommands (union surface)"`.

### Phase 9 Gate

> All checks below must pass before starting Phase 10.

- [ ] [AI] **Rationalization (9a) resolved**: a written keep/merge/delete verdict exists for every
      shortlist item; `env init`/`backup`/`restore` recorded **KEPT** (`.env` secret management);
      `test-coverage diff`/`merge` carry a usage-check verdict; merges leave one shared engine with
      unchanged outputs — expected: the rationalization commit is present.
- [ ] [AI] **Verb-first rename (9b) applied**: every subcommand uses the verb-first git-style scheme
      (BLOCK 11); no old hyphenated invocation remains in any caller —
      `rtk grep -rn 'docs validate-|agents sync$|agents emit-bindings|vendor-audit|validate-tree|validate-naming|validate-annotations|java-clean-imports|dart-scaffold' .husky .github package.json apps/*/project.json libs/*/project.json repo-governance docs AGENTS.md`
      returns nothing — expected: the verb-first rename commit is present and the golden-master corpus
      was deliberately re-captured for the renamed surface.
- [ ] [AI] `rhino-cli --help` lists `java` and `contracts` (verb-first surface) and the kept
      (rationalized) subcommand set — expected: union groups present (TestCoverage, SpecCoverage,
      RepoGovernance, Docs, Agents, Workflows, Specs, Ddd, Git, Env, Java, Contracts); `env`
      init/backup/restore/validate all present; any deleted subcommand absent in all three repos.
- [ ] [AI] Golden-master replay — expected: **unrenamed** entries byte-identical; deliberately
      renamed/merged/deleted/added entries match the updated corpus (no accidental drift).
- [ ] [AI] `:test:unit` and `:lint` GREEN; coverage met.
- [ ] [AI] All three sub-phase commits (9a rationalization, 9b verb-first rename, 9c union port) present.

> **Pause Safety**: the command surface is rationalized, renamed verb-first, and the union additions
> are complete in the hexagonal layout; the golden-master corpus matches the deliberately changed
> surface and tests/coverage are GREEN. Safe to stop. To resume: re-run `--help`, the golden-master
> replay, and `:test:unit`; confirm the three sub-phase commits.

## Phase 10: Target Rename `{domain}:{work}` + `spec-coverage`→`spec:coverage` + Callers

Rename every governance/validation/lint/check target per
[tech-docs.md § Nx Target Rename Map](./tech-docs.md#domainwork-nx-target-rename-map) and rename
`spec-coverage`→`spec:coverage` **repo-wide** (every app/lib `project.json`), then update **every
caller** atomically — the pre-push hook (re-pointing the Phase 6 lifecycle to the canonical names),
`pr-quality-gate.yml`, any `package.json` script, and docs. This is the highest-blast-radius phase
(see [tech-docs.md § D9](./tech-docs.md#d9--domainwork-target-naming--spec-coveragespeccoverage)).

_Suggested executor: `ci-fixer`_

- [ ] [AI] **RED — inventory the old names**:
      `grep -rEl '"(validate:[a-z-]+|fmt:check|check:msrv|lint:[a-z]+|spec-coverage)"' apps/*/project.json libs/*/project.json`
      and `grep -rn 'spec-coverage' .husky/ .github/workflows/ package.json`
      — acceptance: the full set of files carrying old target names + callers is listed.
- [ ] [AI] **GREEN — rename in `apps/rhino-cli/project.json`**: apply the rename map
      (`validate:env`→`env:validation`, `validate:specs-tree`→`specs:tree-validation`, …,
      `fmt:check`→`format:check`, `check:msrv`→`msrv:check`; `deny:check` unchanged;
      `gherkin:keyword-cardinality-validation` already canonical from Phase 4)
      — acceptance: `grep -oE '"[a-z-]+:[a-z-]+"' apps/rhino-cli/project.json` shows only canonical
      `{domain}:{work}` names; no `validate:*`/`fmt:check`/`check:msrv` remain.
- [ ] [AI] **GREEN — `spec-coverage`→`spec:coverage` repo-wide**: rename the target key in **every**
      app/lib `project.json`
      — acceptance: `grep -rn '"spec-coverage"' apps/ libs/` returns nothing; `grep -rn '"spec:coverage"' apps/ libs/`
      lists every project that previously had it.
- [ ] [AI] **GREEN — update callers (atomic with the renames)**:
  - pre-push hook: re-point the Phase 6 lifecycle target list to the canonical names
    (`spec:coverage`, `specs:*-validation`, `env:validation`, `naming:*-validation`,
    `governance:vendor-audit-validation`, `cross-vendor:parity-validation`,
    `harness:bindings-validation`, `markdown:lint`).
  - `pr-quality-gate.yml` (and any other workflow): replace `spec-coverage` in the affected target
    lists with `spec:coverage`; replace `rhino-cli:fmt:check`/`check:msrv` with
    `rhino-cli:format:check`/`msrv:check`.
  - `package.json`: replace any script referencing an old target name.
    — acceptance: `grep -rn 'spec-coverage\|fmt:check\|check:msrv\|validate:env\|validate:specs' .husky/ .github/workflows/ package.json`
    returns nothing.
- [ ] [AI] **REFACTOR — live-run the renamed targets**:
      `npx nx run rhino-cli:env:validation` and a representative `:specs:tree-validation`,
      `:format:check`, and `npx nx affected -t spec:coverage`
      — acceptance: each resolves and runs (no "target not found"); the pre-push dry-run is clean.
- [ ] [AI] Lint all edited workflows + `shellcheck .husky/*` — acceptance: exits 0.

### Phase 10 Gate

> All checks below must pass before starting Phase 11.

- [ ] [AI] `grep -rn '"spec-coverage"' apps/ libs/` — expected: empty.
- [ ] [AI] No old target name remains in any caller
      (`grep -rn 'spec-coverage\|fmt:check\|check:msrv\|validate:env\|validate:specs\|validate:links\|validate:mermaid\|validate:heading-hierarchy\|validate:naming\|validate:cross-vendor\|validate:repo-governance' .husky/ .github/workflows/ package.json apps/*/project.json libs/*/project.json`)
      — expected: empty.
- [ ] [AI] `npx nx run rhino-cli:env:validation` resolves and runs; pre-push dry-run clean.
- [ ] [AI] Workflows + hooks lint clean — expected: exits 0.
- [ ] [AI] Commit thematically (split project.json renames from caller updates if cleaner):
      `rtk git commit -m "refactor(nx): rename governance targets to {domain}:{work} and spec:coverage"`.

> **Pause Safety**: every target uses the canonical name and every caller is re-pointed; the renamed
> targets run and the pre-push dry-run is clean. Safe to stop. To resume: re-run the two grep sweeps
> and a renamed target, confirm the commit.

## Phase 11: Governance Docs → `repo-rules-maker` → Repo-Rules Quality Gate (HARD GATE)

Update **all** related docs (see [tech-docs.md § File Impact](./tech-docs.md#file-impact) and
BLOCK 6), run `repo-rules-maker` to propagate, then run the
[`repo-rules-quality-gate`](../../../repo-governance/workflows/repo/repo-rules-quality-gate.md)
workflow (repo-rules-checker → repo-rules-fixer loop) until it reports **clean**. This is a **hard
gate** — Phase 12 cannot start with the repo-rules gate unsatisfied (see
[tech-docs.md § D5](./tech-docs.md#d5--governance-alignment--citoolchain-parity-checklist) and
[§ D12](./tech-docs.md#d12--final-governance-gate-repo-rules-quality-gate)).

_Suggested executor: `repo-rules-maker`_

> _These are governance-doc + agent-definition edits (non-code) — direct-action + acceptance criteria,
> not RED/GREEN/REFACTOR (per the TDD convention's non-code carve-out)._

- [ ] [AI] Update `repo-governance/development/infra/ci-conventions.md`: converged standard
      (`nx affected` per-language; canonical concurrency; tool-named lint jobs; full-gate-on-push-to-main) + a new `## CI/toolchain Parity Checklist` enumerating the A–G invariants and recording the
      deviations. The checklist MUST embed the **affected-first PR-gate principle + scope table**
      (BLOCK 9 / [tech-docs.md § D13](./tech-docs.md#d13--affected-first-pr-gate-whole-repo-only-by-exception)):
      default = `nx affected`; whole-repo only by justified exception, with each whole-repo check named
      and justified — acceptance: the section lists the A–G invariants (including the state-diagram
      validation invariant), the deviations, and the affected-first principle with its scope table
      (every whole-repo check justified).
- [ ] [AI] Update `repo-governance/development/infra/nx-targets.md`: `{domain}:{work}` naming +
      `spec:coverage` — acceptance: the doc describes the canonical scheme.
- [ ] [AI] Confirm/extend `repo-governance/development/pattern/hexagonal-architecture-cli.md` (this
      convention **already exists**): add the rhino-cli reference layout, the shared-kernel (2+
      consumers) rule, the maximal-port-depth trade-off, and the golden-master enforcement note from
      BLOCK 4 — acceptance: the convention covers the BLOCK 4 design and stays linked from the pattern
      index.
- [ ] [AI] Create the `{domain}:{work}` target-naming convention
      (`repo-governance/development/infra/nx-target-naming.md` or equivalent) — acceptance: exists +
      linked from the infra index.
- [ ] [AI] Create the git-hook-lifecycle convention under
      `repo-governance/development/workflow/` (canonical commit-msg/pre-commit/pre-push) — acceptance:
      exists + linked from the workflow index.
- [ ] [AI] Confirm/extend `repo-governance/development/quality/cross-language-lint-strictness.md`
      (already exists in public; tool-named CI jobs already updated in Phase 3) — acceptance: consistent
      with the converged standard.
- [ ] [AI] **Workstream G** — update `repo-governance/conventions/formatting/diagrams.md` so the
      width/label rules and the `mermaid:validation` enforcement sections enumerate **state diagrams**
      (`stateDiagram-v2` + `stateDiagram` v1): `[*]`/stereotype nodes count toward width; composite
      states are subgraphs; both state display labels and transition-edge labels are checked;
      `direction` is `TB|BT|LR|RL` only — acceptance: the diagram convention lists state diagrams
      alongside flowcharts in both the width/label rule and the enforcement sections.
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] **Workstream G** — note state diagrams are now in `mermaid:validation` scope in
      `repo-governance/development/quality/markdown.md` and
      `repo-governance/development/quality/repository-validation.md` [Repo-grounded: both reference
      `validate:mermaid`/`mermaid:validation`] — acceptance: each register/checker that lists the
      Mermaid gate notes state diagrams are now in scope.
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] Update `AGENTS.md`: Cross-Language Lint Gates, rhino-cli command surface (union superset),
      target naming — acceptance: the three areas reflect the converged toolchain.
- [ ] [AI] Update `apps/rhino-cli/README.md`: command surface + hexagonal architecture — acceptance:
      both documented.
- [ ] [AI] Update the index READMEs that list the above (governance dev/quality/infra/pattern/workflow
      indexes) — acceptance: each new/changed doc is linked from its index (no orphan).
- [ ] [AI] Evaluate `.claude/agents/ci-checker.md` for parity checks (concurrency present; no
      per-language `run-many`; push-to-main gate; canonical target names). Add if they fit the
      deterministic-check shape; otherwise record the skip decision — acceptance: an explicit
      add-or-skip decision is made.
- [ ] [AI] Run the doc validators on the edited files:
      `npx nx run rhino-cli:links:validation && npx nx run rhino-cli:headings:hierarchy-validation && npx nx run rhino-cli:mermaid:validation`
      (canonical names post-Phase-10) — acceptance: all three exit 0.
- [ ] [AI] If any agent definition changed, re-sync bindings: `npm run generate:bindings`
      — acceptance: exits 0; `.opencode/`/`.amazonq/` mirrors regenerated with no parity-guard failure.
- [ ] [AI] **Run `repo-rules-maker`** to propagate the doc changes across all surfaces (registers,
      indexes, checker references) — acceptance: the maker completes and reports the surfaces it
      updated.
- [ ] [AI] **Run the `repo-rules-quality-gate` workflow** (repo-rules-checker → repo-rules-fixer loop)
      and iterate until it reports clean — acceptance: the workflow's terminal report is clean (no
      open CRITICAL/HIGH findings).
- [ ] [AI] Commit thematically (split docs vs agent-definition vs binding-sync commits).

### Phase 11 Gate

> All checks below must pass before starting Phase 12. **This is the hard governance gate.**

- [ ] [AI] All BLOCK 6 docs updated/created and linked from their indexes (no orphan) — expected:
      index link checks pass.
- [ ] [AI] `grep -n "CI/toolchain Parity Checklist" repo-governance/development/infra/ci-conventions.md`
      — expected: the section is present.
- [ ] [AI] `grep -n "stateDiagram" repo-governance/conventions/formatting/diagrams.md` — expected: the
      width/label + enforcement sections enumerate state diagrams (workstream G).
- [ ] [AI] Doc validators (`links:validation`, `headings:hierarchy-validation`, `mermaid:validation`)
      exit 0; bindings in sync if changed.
- [ ] [AI] **`repo-rules-quality-gate` workflow reports clean** — expected: no open CRITICAL/HIGH
      findings. If not clean, STOP — do not proceed to Phase 12.
- [ ] [AI] All governance commits present.

> **Pause Safety**: all related docs are updated, propagated, and the repo-rules quality gate is
> clean; the changes are committed. Safe to stop. To resume: re-run the doc validators and the
> repo-rules quality gate, confirm the commits.

## Phase 12: Final Quality Gate + Push + CI Verify + Archival

### Local Quality Gates (Before Push)

- [ ] [AI] `npx nx affected -t typecheck` — exits 0.
- [ ] [AI] `npx nx affected -t lint` — exits 0.
- [ ] [AI] `npx nx affected -t test:quick` — exits 0.
- [ ] [AI] `npx nx affected -t spec:coverage` — exits 0 (canonical name post-Phase-10).
- [ ] [AI] Full validator set locally (canonical names):
      `npx nx run rhino-cli:gherkin:keyword-cardinality-validation`,
      `:links:validation`, `:mermaid:validation` (now covers state diagrams),
      `:headings:hierarchy-validation`, `:env:validation` — all exit 0.
- [ ] [AI] Golden-master replay harness — corpus byte-identical (existing flowchart entries) + new
      union entries present; the state golden corpus passes — exits 0.
- [ ] [AI] Lint all workflows: `actionlint .github/workflows/*.yml` (or `prettier --check` fallback)
      and `shellcheck .husky/*` — exits 0.

> **Important**: Fix ALL failures found during quality gates, not just those caused by your changes
> (root-cause orientation). Commit preexisting fixes separately with appropriate conventional commit
> messages.

### Commit Guidelines

- [ ] [AI] Commit changes thematically — group related changes into logically cohesive commits.
- [ ] [AI] Follow Conventional Commits format: `<type>(<scope>): <description>`.
- [ ] [AI] Split different domains/concerns (workflows vs hooks vs rhino-cli code vs governance docs
      vs agent definitions) into separate commits.
- [ ] [AI] Preexisting fixes get their own commits, separate from plan work.

### Post-Push CI Verification

- [ ] [AI] Push to `main`: `rtk git push origin HEAD:main` (worktree-to-main, no PR).
- [ ] [AI] Monitor ALL GitHub Actions workflows triggered by the push, polling every 3 minutes via
      `gh run view --json status,conclusion` (do NOT use `gh run watch`).
- [ ] [AI] Verify ALL CI checks pass — confirm the renamed lint jobs
      (`shellcheck`/`hadolint`/`actionlint`), the `gherkin:keyword-cardinality-validation` step, and
      the **push-to-main full gate** all ran and are green.
- [ ] [AI] If any CI check fails, fix immediately and push a follow-up commit; repeat until ALL pass.
- [ ] [AI] Do NOT proceed to archival until CI is fully green.

### Plan Archival

- [ ] [AI] Verify ALL delivery checklist items are ticked and ALL quality gates pass (local + CI).
- [ ] [AI] Rename and move:
      `git mv plans/in-progress/standardize-repo-toolchain-parity/ plans/done/2026-06-12__standardize-repo-toolchain-parity/`
      using the **completion date** (adjust to the actual completion date at archival time).
- [ ] [AI] Update `plans/in-progress/README.md` — remove the plan entry.
- [ ] [AI] Update `plans/done/README.md` — add the entry with the completion date.
- [ ] [AI] Update any other READMEs that reference this plan.
- [ ] [AI] Commit the archival:
      `rtk git commit -m "chore(plans): move standardize-repo-toolchain-parity to done"` and push to
      `origin main`.

### Phase 12 Gate

> All checks below must pass to consider the plan complete.

- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec:coverage` — expected: exits 0.
- [ ] [AI] Full validator set + golden-master replay run green locally — expected: all exit 0.
- [ ] [AI] `gh run view --json status,conclusion` on the latest `main` runs — expected: all
      `conclusion: success`; renamed lint jobs, the gherkin step, and the push-to-main gate all green.
- [ ] [AI] Plan folder moved under `plans/done/<completion-date>__standardize-repo-toolchain-parity/`
      (`ls plans/done/ | grep standardize-repo-toolchain-parity` returns exactly one dated entry) and
      the index READMEs updated — expected: `git status` clean after the archival commit is pushed.

> **Pause Safety**: the standardized toolchain is live on `origin main`, all CI is green, the
> repo-rules gate is clean, and the plan is archived to `done/`. This is the terminal state. To
> resume verification: re-run the affected gate, the golden-master replay, and `gh run view` on the
> latest `main` runs.
