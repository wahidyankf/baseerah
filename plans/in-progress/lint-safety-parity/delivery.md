# Delivery Checklist — lint-safety-parity (ose-public)

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.
>
> **Phase Gate** — every phase ends with a `### Phase N Gate` (must-pass verification) plus a
> `> **Pause Safety**:` note (the safe-to-stop state and the single command to resume). A phase
> is not complete until its gate is green; do not start phase N+1 while any gate check fails.

## Delivery Mode

**main-to-main** — all work in this plan is committed and pushed directly to `ose-public`'s
`origin main` (no PR, no feature branch). This is the Trunk Based Development default for
`ose-public`. `ose-public` is the upstream source of truth and is NOT bound by the ose-primer
Sync Convention draft-PR invariant (that invariant, and its approved deviation M1, applies only to
the **primer** plan, recorded in the primer plan's tech-docs). Do NOT create a PR for this plan.

> **Planning-only reminder**: This plan's terminal deliverable is the validated five-document
> plan itself. The phases below describe the EXECUTION work that a downstream plan-execution run
> will perform. Authoring this plan does not execute any config change.

## Worktree

Worktree path: `worktrees/lint-safety-parity/`

Optional manual pre-provisioning (run from repo root):

```bash
claude --worktree lint-safety-parity
```

The plan-execution Step 0 gate enters this worktree by default: it auto-provisions from the latest
`origin/main` when missing, syncs with `origin/main` before implementing, and prompts before
deleting the worktree after the plan is archived and pushed.

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

---

## Phase 0: Environment Setup and Baseline

> _Executor: repo-setup-manager_

- [ ] [AI] Install dependencies in the root worktree: `npm install`
      — acceptance: exits 0, `node_modules/` synchronized
- [ ] [AI] Converge the full polyglot toolchain in the root worktree: `npm run doctor -- --fix`
      — acceptance: exits 0 with no unresolved drift (verifies .NET SDK ≥ 8 for F# TWAE)
- [ ] [AI] Confirm the F# surface: `find apps libs -name '*.fsproj' | grep -v node_modules`
      — acceptance: lists exactly 8 `.fsproj` files (crane-be ×3, crane-cli ×3, fsharp-crane-core ×2)
- [ ] [AI] Confirm no active Go: `find . -name go.mod -not -path '*/node_modules/*' -not -path '*/archived/*'`
      — acceptance: prints nothing (Go only in `archived/`); confirms D10 removal is safe
- [ ] [AI] Confirm root `.golangci.yml` exists and is unreferenced by workflows/scripts:
      `test -f .golangci.yml && grep -rn 'golangci' .github scripts apps/*/project.json nx.json || true`
      — acceptance: file exists; record every reference found (expected: none active)
- [ ] [AI] Record the F# lint baseline: `npx nx run-many -t lint --projects='tag:lang:dotnet'`
      — acceptance: baseline pass/fail recorded for crane-be, crane-cli, fsharp-crane-core
- [ ] [AI] Run the affected baseline gate and record it:
      `npx nx affected -t typecheck lint test:quick spec-coverage`
      — acceptance: baseline pass/fail count recorded; all preexisting failures documented
- [ ] [AI] Resolve all preexisting failures before proceeding
      — acceptance: no preexisting failures remain unresolved

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [ ] [AI] `npm install` exited 0 and `npm run doctor -- --fix` reports no unresolved drift
- [ ] [AI] The 8 `.fsproj` files and the dead-but-unreferenced `.golangci.yml` are confirmed
- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` baseline recorded and every
      preexisting failure resolved (zero unresolved)

> **Pause Safety**: only the local toolchain was verified and the baseline recorded — no feature
> work exists yet. Safe to stop indefinitely. To resume: re-run the baseline command and confirm
> it is still clean.

---

## Phase 1: D10 — Remove dead `.golangci.yml`

> Smallest, lowest-risk change first (a pure deletion).

- [ ] [AI] Re-confirm no active Go module references the config:
      `grep -rn 'golangci' .github .husky scripts apps libs nx.json package.json || true`
      — acceptance: no active workflow/script/Nx target references `.golangci.yml`
- [ ] [AI] Delete the dead config: `git rm .golangci.yml`
      — acceptance: `test -f .golangci.yml` returns non-zero (file gone)
  - _Suggested executor: `ci-fixer`_
- [ ] [AI] Run the affected gate to confirm nothing depended on it:
      `npx nx affected -t typecheck lint test:quick spec-coverage`
      — acceptance: exits 0; no job referenced the removed file
- [ ] [AI] Commit thematically: `git commit -m "chore(lint): remove dead .golangci.yml (no active Go)"`
      — acceptance: commit created with Conventional Commits format

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [ ] [AI] `test -f .golangci.yml` — expected: non-zero (file removed)
- [ ] [AI] `grep -rn 'golangci' .github .husky scripts nx.json` — expected: no active references
- [ ] [AI] `npx nx affected -t lint` — expected: exits 0

> **Pause Safety**: the dead config is gone and nothing referenced it; the repo lints clean. Safe
> to stop. To resume: `npx nx affected -t lint`.

---

## Phase 2: D7 — Shell lint (shellcheck)

> 14 `.sh` files (excluding `.husky/_/husky.sh` vendored + `archived/**`). Clean-then-gate.

- [ ] [AI] **RED**: run shellcheck across tracked shell scripts to surface the existing backlog:
      `shellcheck --severity=warning scripts/*.sh .claude/hooks/*.sh apps/rhino-cli/scripts/*.sh`
      — acceptance: command exits non-zero OR exits 0; record every finding as the cleanup backlog
      (this is the failing-gate state — the gate is not yet wired on)
  - _Suggested executor: `ci-checker`_
- [ ] [AI] **GREEN**: fix every shellcheck `severity=warning` finding in the affected `.sh` files
      (quote variables, fix `SC2086`/`SC2046`-class issues, add justified `# shellcheck disable=`
      with inline rationale only where genuinely needed)
      — command: `shellcheck --severity=warning scripts/*.sh .claude/hooks/*.sh apps/rhino-cli/scripts/*.sh`
      — acceptance: exits 0 (no warning-or-above findings remain)
  - _Suggested executor: `ci-fixer`_
- [ ] [AI] Create `.shellcheckrc` at repo root with `shell=bash`, `external-sources=true`, and any
      justified repo-wide disables (each with an inline `# rationale:` comment)
      — acceptance: `test -f .shellcheckrc` returns 0; file documents every disable
- [ ] [AI] **REFACTOR (flip-on)**: wire the shellcheck gate into CI — add a `shell` job to
      `.github/workflows/pr-quality-gate.yml` running
      `shellcheck --severity=warning` over the tracked script set, and register `shell` in the
      `quality-gate` job's `needs:` list and failure-check loop
      — acceptance: workflow YAML parses; the new job is listed in `quality-gate.needs`
  - _Suggested executor: `ci-fixer`_
- [ ] [AI] **REFACTOR (flip-on, local)**: add the shellcheck invocation to `.husky/pre-commit` (or
      `pre-push`) scoped to staged/changed `.sh` files
      — acceptance: hook file runs shellcheck; `git commit` on a clean tree succeeds
- [ ] [AI] Add `shellcheck` to the toolchain converger so `npm run doctor -- --fix` installs it
      (follow the existing doctor pattern; confirm the doctor config path before editing)
      — acceptance: `npm run doctor` reports shellcheck present
- [ ] [AI] Commit thematically: `git commit -m "ci(lint): add shellcheck gate (warning threshold)"`

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [ ] [AI] `shellcheck --severity=warning scripts/*.sh .claude/hooks/*.sh apps/rhino-cli/scripts/*.sh`
      — expected: exits 0
- [ ] [AI] `test -f .shellcheckrc` — expected: exits 0
- [ ] [AI] `grep -q 'shell' .github/workflows/pr-quality-gate.yml` and the `shell` job is in
      `quality-gate.needs` — expected: present

> **Pause Safety**: shell scripts are clean and the shellcheck gate is live in CI + hooks. Safe to
> stop. To resume: re-run the shellcheck command above.

---

## Phase 3: D6 — Dockerfile lint (hadolint)

> 10 app Dockerfiles under `apps/*/` (incl. 2 `Dockerfile.integration`); exclude `archived/**`.
> Executor confirms whether to also gate `infra/dev/**`. Clean-then-gate.

- [ ] [AI] **RED**: run hadolint across app Dockerfiles to surface the backlog:
      `hadolint --failure-threshold warning apps/*/Dockerfile apps/*/Dockerfile.integration`
      — acceptance: record every finding as the cleanup backlog (failing-gate state, gate not wired)
  - _Suggested executor: `ci-checker`_
- [ ] [AI] **GREEN**: fix every warning-or-above hadolint finding across the Dockerfiles (pin apt
      versions where feasible, fix `DL`-class issues; defer only truly-justified rules to the
      ignore list in the next step)
      — command: `hadolint --failure-threshold warning apps/*/Dockerfile apps/*/Dockerfile.integration`
      — acceptance: exits 0
  - _Suggested executor: `ci-fixer`_
- [ ] [AI] Create `.hadolint.yaml` at repo root with `failure-threshold: warning`,
      `trustedRegistries: [docker.io, ghcr.io]`, and justified per-rule `ignore` entries (e.g.
      `DL3008` for dev images), each with an inline rationale comment
      — acceptance: `test -f .hadolint.yaml` returns 0
- [ ] [AI] **REFACTOR (flip-on, CI)**: add a `dockerfile` job to
      `.github/workflows/pr-quality-gate.yml` running hadolint over the app Dockerfile set, and
      register `dockerfile` in `quality-gate.needs` + the failure-check loop
      — acceptance: workflow parses; job listed in `quality-gate.needs`
  - _Suggested executor: `ci-fixer`_
- [ ] [AI] **REFACTOR (flip-on, local)**: add the hadolint invocation to `.husky/pre-commit` scoped
      to changed Dockerfiles
      — acceptance: hook runs hadolint; clean commit succeeds
- [ ] [AI] Add `hadolint` to the toolchain converger (doctor `--fix` installs it)
      — acceptance: `npm run doctor` reports hadolint present
- [ ] [AI] Commit thematically: `git commit -m "ci(lint): add hadolint gate (warning threshold)"`

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [ ] [AI] `hadolint --failure-threshold warning apps/*/Dockerfile apps/*/Dockerfile.integration`
      — expected: exits 0
- [ ] [AI] `test -f .hadolint.yaml` — expected: exits 0
- [ ] [AI] `dockerfile` job present in `.github/workflows/pr-quality-gate.yml` `quality-gate.needs`

> **Pause Safety**: Dockerfiles are clean and the hadolint gate is live in CI + hooks. Safe to
> stop. To resume: re-run the hadolint command above.

---

## Phase 4: D8 — GitHub Actions lint (actionlint)

> 22 files under `.github/workflows/*.yml`. GitHub-hosted runners → runner-label config optional.
> Clean-then-gate.

- [ ] [AI] **RED**: run actionlint across all workflows to surface the backlog:
      `actionlint`
      — acceptance: run from repo root; record every finding as the cleanup backlog (failing-gate
      state, gate not yet wired)
  - _Suggested executor: `ci-checker`_
- [ ] [AI] **GREEN**: fix every actionlint finding in `.github/workflows/*.yml` (invalid
      expressions, shell quoting in `run:` steps, deprecated syntax)
      — command: `actionlint`
      — acceptance: exits 0
  - _Suggested executor: `ci-fixer`_
- [ ] [AI] (Optional) Create `.github/actionlint.yaml` only if self-hosted runner labels or
      config-variables need declaring; for `ose-public` (GitHub-hosted) this is likely unnecessary
      — acceptance: either the file is created with documented labels, OR the step is recorded as
      "not needed for ose-public (GitHub-hosted runners)"
- [ ] [AI] **REFACTOR (flip-on, CI)**: add an `actions` job to
      `.github/workflows/pr-quality-gate.yml` running `actionlint`, and register `actions` in
      `quality-gate.needs` + the failure-check loop
      — acceptance: workflow parses; job listed in `quality-gate.needs`
  - _Suggested executor: `ci-fixer`_
- [ ] [AI] **REFACTOR (flip-on, local)**: add the actionlint invocation to `.husky/pre-commit`
      scoped to changed workflow files
      — acceptance: hook runs actionlint; clean commit succeeds
- [ ] [AI] Add `actionlint` to the toolchain converger (doctor `--fix` installs it)
      — acceptance: `npm run doctor` reports actionlint present
- [ ] [AI] Commit thematically: `git commit -m "ci(lint): add actionlint gate"`

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [ ] [AI] `actionlint` from repo root — expected: exits 0
- [ ] [AI] `actions` job present in `.github/workflows/pr-quality-gate.yml` `quality-gate.needs`

> **Pause Safety**: workflows are clean and the actionlint gate is live in CI + hooks. Safe to
> stop. To resume: re-run `actionlint`.

---

## Phase 5: D2 — F# strict stack (LARGEST item)

> All 8 `.fsproj` files. Add TWAE + pinned G-Research analyzers; keep fantomas-check.
> Clean-then-gate: clean latent warnings per project BEFORE flipping TWAE on.
> _All F# code/cleanup steps — suggested executor: `swe-fsharp-dev`._

### Phase 5a — Latent-warning cleanup (GREEN-first, gate still off)

- [ ] [AI] **RED**: surface latent F# warnings per project by building with TWAE temporarily forced
      WITHOUT committing the flag — for each project run
      `dotnet build apps/crane-be/crane-be.fsproj /warnaserror` (repeat for crane-cli and
      fsharp-crane-core source projects)
      — acceptance: record the full latent-warning backlog per project (failing-gate state)
  - _Suggested executor: `swe-fsharp-dev`_
- [ ] [AI] **GREEN**: clean all latent warnings in `apps/crane-be/src/**` until
      `dotnet build apps/crane-be/crane-be.fsproj /warnaserror` exits 0.
      File targets come from the RED step's recorded backlog above.
      — acceptance: exits 0
  - _Suggested executor: `swe-fsharp-dev`_
- [ ] [AI] **GREEN**: clean all latent warnings in `apps/crane-cli/src/**` until
      `dotnet build apps/crane-cli/crane-cli.fsproj /warnaserror` exits 0.
      File targets come from the RED step's recorded backlog above.
      — acceptance: exits 0
  - _Suggested executor: `swe-fsharp-dev`_
- [ ] [AI] **GREEN**: clean all latent warnings in `libs/fsharp-crane-core/src/**` until
      `dotnet build libs/fsharp-crane-core/fsharp-crane-core.fsproj /warnaserror` exits 0.
      File targets come from the RED step's recorded backlog above.
      — acceptance: exits 0
  - _Suggested executor: `swe-fsharp-dev`_
- [ ] [AI] **GREEN**: clean latent warnings in the 5 test projects (`crane-be` unit+integration,
      `crane-cli` unit+integration, `fsharp-crane-core` unit) until each builds clean with
      `/warnaserror`. File targets come from the RED step's recorded backlog above.
      — acceptance: all 5 test `.fsproj` build with `/warnaserror` exit 0
  - _Suggested executor: `swe-fsharp-dev`_

### Phase 5b — G-Research analyzers + TWAE flip-on (REFACTOR)

- [ ] [AI] **REFACTOR (flip-on)**: add `<TreatWarningsAsErrors>true</TreatWarningsAsErrors>` to the
      `<PropertyGroup>` of all 8 `.fsproj` files (or introduce a shared root `Directory.Build.props`
      — the executor records which approach; default is per-`.fsproj` edits since no
      `Directory.Build.props` exists today)
      — command: `dotnet build apps/crane-be/crane-be.fsproj --no-restore`
      — acceptance: build exits 0 with TWAE active (warnings now break the build)
  - _Suggested executor: `swe-fsharp-dev`_
- [ ] [AI] **REFACTOR (flip-on)**: add a **version-pinned** G-Research.FSharp.Analyzers
      `PackageReference` (e.g. `Version="0.17.0"` — confirm the latest stable pin via the analyzer
      release page before committing) to the source `.fsproj` files, and add a
      `dotnet fsharp-analyzers` invocation to each F# project's `lint` target in `project.json`
      (siblings: existing `fantomas --check` + `dotnet fsharplint` commands)
      — command: `npx nx run-many -t lint --projects='tag:lang:dotnet'`
      — acceptance: lint runs the analyzers and exits 0
  - _Suggested executor: `swe-fsharp-dev`_
- [ ] [AI] Confirm `fantomas --check` remains in each F# `lint` target (already present — keep)
      — command: `npx nx run crane-be:lint`
      — acceptance: fantomas check runs and exits 0
- [ ] [AI] **REFACTOR (CI)**: confirm the existing `dotnet` job in
      `.github/workflows/pr-quality-gate.yml` exercises the stricter F# build+lint (it runs
      `nx run-many -t typecheck lint ... --projects='tag:lang:fsharp,tag:lang:csharp'`); add the
      `dotnet fsharp-analyzers` CI invocation if not covered by the `lint` target
      — acceptance: the `dotnet` job fails on an F# warning (verified by a scratch warning, then
      reverted)
  - _Suggested executor: `ci-fixer`_
- [ ] [AI] Run the F# test suites to confirm strictness did not break behavior:
      `npx nx run-many -t test:quick --projects='tag:lang:dotnet'`
      — acceptance: all F# unit tests pass
  - _Suggested executor: `swe-fsharp-dev`_
- [ ] [AI] Commit thematically (split cleanup vs flip-on):
      `git commit` for cleanup, then `git commit -m "build(fsharp): enable TreatWarningsAsErrors + pin G-Research analyzers"`

### Phase 5 Gate

> All checks below must pass before starting Phase 6.

- [ ] [AI] `npx nx run-many -t typecheck --projects='tag:lang:dotnet'` — expected: all 3 build with
      TWAE active, exit 0
- [ ] [AI] `npx nx run-many -t lint --projects='tag:lang:dotnet'` — expected: analyzers + fantomas +
      fsharplint all pass, exit 0
- [ ] [AI] `npx nx run-many -t test:quick --projects='tag:lang:dotnet'` — expected: exit 0
- [ ] [AI] Every `.fsproj` (8 total) contains TWAE OR inherits it from a committed
      `Directory.Build.props` — expected: confirmed

> **Pause Safety**: F# is clean, strict, and green under TWAE + pinned analyzers. Safe to stop. To
> resume: `npx nx run-many -t typecheck lint test:quick --projects='tag:lang:dotnet'`.

---

## Phase 6: Documentation and Governance

> _Suggested executor: `repo-rules-maker` (governance) / `docs-maker` (rationale doc)._

- [ ] [AI] Write `docs/explanation/lint-safety-parity-decisions.md` (plain-language rationale)
      following the sibling precedent
      `docs/explanation/gherkin-step-keyword-cardinality-parity-decisions.md`. It MUST cover:
      every ose-public dimension (D2/D6/D7/D8/D10) with its rationale; the documented Rust
      reference status (D1/D1b not executed here); the **D5 deferral**; and the **exemption
      philosophy** (DDD enforcement targets business-domain backends only — demo/content/frontend
      apps are exempt); plus cross-links to the two sibling plans
      — acceptance: `test -f docs/explanation/lint-safety-parity-decisions.md` returns 0; doc names
      all five executed dimensions + the D5 deferral + the exemption philosophy
  - _Suggested executor: `docs-maker`_
- [ ] [AI] Add the rationale doc to `docs/explanation/README.md` index (if it enumerates entries)
      — acceptance: index links the new doc; `npm run lint:md` passes
- [ ] [AI] Create or update a governance convention documenting the **shared cross-language
      strictness standard** (the warning-and-above error threshold across F#/Docker/shell/CI, plus
      the new Nx lint-target additions). Place under `repo-governance/development/quality/` following
      the sibling pattern of `markdown.md` / `repository-validation.md`
      — acceptance: new/updated convention names hadolint, shellcheck, actionlint, and F# TWAE as
      gated standards; `npx nx run rhino-cli:validate:repo-governance-vendor-audit` passes
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] Update `AGENTS.md` "Markdown Quality" / Quality-Gates style lists and the
      Build/Test/Lint commands section to mention the new gates (hadolint/shellcheck/actionlint)
      and any new Nx lint targets
      — acceptance: AGENTS.md lists the three new gates; `npm run lint:md` passes
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] Re-sync platform bindings if any agent/governance surface changed:
      `npm run generate:bindings`
      — acceptance: `npm run validate:harness-bindings` passes (no binding drift)
- [ ] [AI] Commit thematically: `git commit -m "docs(lint): add lint-safety-parity rationale + cross-language strictness convention"`

### Phase 6 Gate

> All checks below must pass before starting Phase 7.

- [ ] [AI] `test -f docs/explanation/lint-safety-parity-decisions.md` — expected: exits 0
- [ ] [AI] `npm run lint:md` — expected: exits 0
- [ ] [AI] `npx nx run rhino-cli:validate:repo-governance-vendor-audit` — expected: exits 0
- [ ] [AI] `npm run validate:harness-bindings` — expected: exits 0

> **Pause Safety**: all docs and governance reflect the new standard and links/bindings validate.
> Safe to stop. To resume: `npm run lint:md`.

---

## Phase 7: Final Verification, Push, and Archival

### Local Quality Gates (Before Push)

- [ ] [AI] Run affected typecheck: `npx nx affected -t typecheck`
      — acceptance: exits 0
- [ ] [AI] Run affected linting: `npx nx affected -t lint`
      — acceptance: exits 0
- [ ] [AI] Run affected quick tests: `npx nx affected -t test:quick`
      — acceptance: exits 0
- [ ] [AI] Run affected spec coverage: `npx nx affected -t spec-coverage`
      — acceptance: exits 0
- [ ] [AI] Run shellcheck gate: `shellcheck --severity=warning scripts/*.sh .claude/hooks/*.sh apps/rhino-cli/scripts/*.sh`
      — acceptance: exits 0
- [ ] [AI] Run hadolint gate: `hadolint --failure-threshold warning $(find apps -name 'Dockerfile*' -not -path '*/archived/*')`
      — acceptance: exits 0
- [ ] [AI] Run actionlint gate: `actionlint`
      — acceptance: exits 0
- [ ] [AI] Run markdown lint: `npm run lint:md`
      — acceptance: exits 0
- [ ] [AI] Fix ALL failures — including preexisting issues not caused by these changes
      — acceptance: all gates green
- [ ] [AI] Re-run failing checks to confirm resolution
      — acceptance: re-run exits 0
- [ ] [AI] Verify zero failures before pushing
      — acceptance: zero failures across all gates

> **Important**: Fix ALL failures found during quality gates, not just those caused by these
> changes. This follows the root cause orientation principle — proactively fix preexisting errors
> encountered during work. Do not defer or skip existing issues. Commit preexisting fixes
> separately with appropriate conventional commit messages.

### Commit Guidelines

- [ ] [AI] Commit changes thematically — group related changes into logically cohesive commits
- [ ] [AI] Follow Conventional Commits: `<type>(<scope>): <description>`
- [ ] [AI] Split different domains/concerns into separate commits (D10 / D7 / D6 / D8 / D2 / docs)
- [ ] [AI] Preexisting fixes get their own commits, separate from plan work

### Post-Push CI Verification

- [ ] [AI] Push changes to `main`: `git push origin main`
- [ ] [AI] Monitor ALL GitHub Actions workflows triggered by the push (poll every 3 min; do NOT use
      `gh run watch`): `gh run view --json status,conclusion`
- [ ] [AI] Verify ALL CI checks pass — including the newly-added `shell`, `dockerfile`, `actions`
      jobs and the stricter `dotnet` job — no exceptions
- [ ] [AI] If any CI check fails, fix immediately and push a follow-up commit
- [ ] [AI] Repeat until ALL GitHub Actions pass with zero failures
- [ ] [AI] Do NOT proceed to archival until CI is fully green

### Plan Archival

- [ ] [AI] Verify ALL delivery checklist items are ticked
- [ ] [AI] Verify ALL quality gates pass (local + CI)
- [ ] [AI] Rename and move:
      `git mv plans/in-progress/lint-safety-parity/ plans/done/YYYY-MM-DD__lint-safety-parity/`
      using today's date as the completion date (NOT the creation date)
- [ ] [AI] Update `plans/in-progress/README.md` — remove the plan entry
- [ ] [AI] Update `plans/done/README.md` — add the plan entry with completion date
- [ ] [AI] Update any other READMEs that reference this plan (e.g., `plans/README.md`)
- [ ] [AI] Commit the archival: `git commit -m "chore(plans): move lint-safety-parity to done"`

### Phase 7 Gate

> Terminal gate — the plan is complete only when every check below is green.

- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — expected: exits 0
- [ ] [AI] `shellcheck --severity=warning scripts/*.sh .claude/hooks/*.sh apps/rhino-cli/scripts/*.sh` — expected: exits 0
- [ ] [AI] `hadolint --failure-threshold warning $(find apps -name 'Dockerfile*' -not -path '*/archived/*')` — expected: exits 0
- [ ] [AI] `actionlint` — expected: exits 0
- [ ] [AI] All GitHub Actions workflows for the push — expected: all green
- [ ] [AI] Plan folder lives under `plans/done/YYYY-MM-DD__lint-safety-parity/` — expected: confirmed

> **Pause Safety**: the plan is fully executed, pushed, CI-green, and archived. Terminal state —
> nothing remains. To re-verify: `npx nx affected -t lint` on `main`.

---

## Validation Checklist

- [ ] [AI] All TDD cycles complete (RED→GREEN→REFACTOR for each lint gate: D7, D6, D8, D2)
- [ ] [AI] All acceptance criteria from `prd.md` verified
- [ ] [AI] Rationale doc + governance/convention/AGENTS.md updates complete
- [ ] [AI] D10 dead config removed; D1/D1b documented as reference (not executed)
- [ ] [AI] CI green on `main` after push
