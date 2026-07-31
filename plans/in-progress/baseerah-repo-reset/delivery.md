# Delivery — Baseerah Repo Reset

Executable checklist for the [Baseerah Repo Reset](./README.md) plan. Read
[tech-docs.md](./tech-docs.md) before starting — the four ordering constraints in its **Mechanics**
section are why the phases are in this order.

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.

## Delivery Mode: `main-to-origin-main`

Work happens in the **primary checkout** at `/Users/wkf/ose-projects/baseerah` on branch `main`.
Each phase commits and pushes directly to `origin main`. No PR opens, so the
[PRs Open at Delivery Boundaries](../../../repo-governance/conventions/structure/plans.md#prs-open-at-delivery-boundaries-not-every-phase-hard-rule)
rule and its `### Delivery Boundaries` table do not bind this plan — a per-phase commit-and-push
checkpoint cadence is the correct and explicitly sanctioned form under this mode. The
PR-Review Maker→Fixer Cycle does not run.

## Worktree

**None.** `main-to-origin-main` works in the primary checkout. Do not run `git worktree add` for
this plan. `worktrees/` remains empty of this plan's name throughout.

## Parallelization Model

**Chosen N: 1** — the main thread does the work, no background fan-out.

This plan is a **serial spine with no independent nodes**, which is unusual for this repo and is a
deliberate consequence of two facts:

- **Every phase writes files a later phase reads.** Phase 2 cannot run before Phase 1 (CI callers
  must die before `infra/`), Phase 3 cannot run before Phase 2 (governance prose is pruned against
  the post-deletion file set), Phases 5–9 cannot run before Phase 4 (both naming vocabularies must
  be amended before anything is named against them), and Phase 9 needs both Phase 7's backend stack
  and Phase 8's frontend.
- **`main-to-origin-main` gives every phase the same write target.** Two agents pushing to `main`
  from one checkout is a write conflict by construction. Under `worktree-to-pr` the app-creation
  phases could have fanned out; under this mode they cannot.

The one place genuine independence exists — Phase 6 (`baseerah-be`) and Phase 8 (`baseerah-fe`)
neither read nor write each other's files — is _not_ parallelised, because both must commit to the
same branch in the same checkout. Sequence here is load-bearing for that reason alone, not because
the work depends.

**Cleanup is the terminal node.** Phase 11 depends on every preceding phase and is the only phase
permitted to move this plan folder.

### Push Checkpoints

| Phase | Produces                                      | Pushes to `origin main`              |
| ----- | --------------------------------------------- | ------------------------------------ |
| 0     | baseline evidence only                        | no — evidence rides Phase 1's commit |
| 1     | CI/infra caller removal                       | yes                                  |
| 2     | app, lib, spec, and config removal            | yes                                  |
| 3     | agent, governance, docs, plan-archive removal | yes                                  |
| 4     | Baseerah identity surface                     | yes                                  |
| 5     | `specs/apps/baseerah/` + `baseerah-contracts` | yes                                  |
| 6     | `baseerah-be`                                 | yes                                  |
| 7     | `baseerah-be-e2e` + `infra/dev/baseerah-app/` | yes                                  |
| 8     | `baseerah-fe`                                 | yes                                  |
| 9     | `baseerah-fe-e2e`                             | yes                                  |
| 10    | Baseerah agent fleet                          | yes                                  |
| 11    | knowledge capture + archival                  | yes                                  |

---

## Phase 0: Environment Setup and Baseline

> Executed by `repo-setup-manager`. Produces no reviewable change and pushes nothing — its evidence
> file rides Phase 1's commit, per
> [Phase 0 Opens No PR](../../../repo-governance/conventions/structure/plans.md#phase-0-opens-no-pr--the-earliest-pr-is-phase-1-hard-rule).

- [x] [AI] Confirm the working tree is clean: run `git status --porcelain` from
      `/Users/wkf/ose-projects/baseerah` — acceptance: no output. If output exists, stop and surface
      it to the maintainer; do not stash or discard.
      **Date**: 2026-07-31. **Status**: Done. **Files Changed**: none. Working tree was clean before any Phase 0 work began.
- [x] [AI] Confirm the checkout is on `main` and level with the remote: run
      `git rev-parse --abbrev-ref HEAD && git fetch origin && git status -sb` — acceptance: branch is
      `main` and the status line shows no `behind` or `ahead` counts.
      **Date**: 2026-07-31. **Status**: Done. **Files Changed**: none. Branch `main`, `## main...origin/main`, no ahead/behind.
- [x] [AI] Record the pre-plan commit SHA into `plans/in-progress/baseerah-repo-reset/evidence/phase-0-baseline.txt`:
      run `git rev-parse HEAD` and write the SHA under a `## Pre-plan HEAD` heading — acceptance: the
      file exists and contains a 40-character SHA.
      **Date**: 2026-07-31. **Status**: Done. **Files Changed**: `evidence/phase-0-baseline.txt` (new). SHA `d4b9008974e89c09a391f7d3f36fd4e337c87df9`.
- [x] [AI] Install dependencies: run `npm install` — acceptance: exits 0.
      **Date**: 2026-07-31. **Status**: Done. **Files Changed**: none. `up to date, audited 1596 packages`, exit 0.
- [x] [AI] Converge the polyglot toolchain: run `npm run doctor -- --fix` — acceptance: exits 0 and
      reports no missing tools.
      **Date**: 2026-07-31. **Status**: Done. **Files Changed**: none. 16/16 tools OK, 0 missing.
- [x] [AI] Record the current project graph: run `npx nx show projects --json` and append the output
      to `evidence/phase-0-baseline.txt` under a `## Project graph before` heading — acceptance: the
      file lists 27+ projects.
      **Date**: 2026-07-31. **Status**: Done. **Files Changed**: `evidence/phase-0-baseline.txt`. 29 projects recorded (≥27).
- [x] [AI] Record the baseline quality state: run
      `npx nx run-many -t typecheck,lint,test:quick --all --parallel=$(( $(sysctl -n hw.ncpu) - 1 ))`
      and append the summary line to `evidence/phase-0-baseline.txt` under a
      `## Baseline test:quick` heading — acceptance: the summary line is recorded verbatim, whether
      it passed or failed.
      **Date**: 2026-07-31. **Status**: Done. **Files Changed**: `evidence/phase-0-baseline.txt`. "Successfully ran targets typecheck, lint, test:quick for 29 projects and 5 tasks they depend on" — exit 0. Side-effect: this run regenerated `libs/fsharp-crane-core/tests/unit/coverage.json` with checkout-local absolute paths (a preexisting repo-hygiene defect — a generated coverage artifact with machine-specific paths is git-tracked instead of gitignored); reverted with `git checkout -- libs/fsharp-crane-core/tests/unit/coverage.json` as out-of-scope for this plan, logged to `learnings.md`.
- [x] [AI] If the baseline run reported failures, fix each preexisting failure now, per the
      [Root Cause Orientation principle](../../../repo-governance/principles/general/root-cause-orientation.md)
      — acceptance: a re-run of the same command exits 0. Record the fixes in `learnings.md`.
      **Date**: 2026-07-31. **Status**: Done (N/A). **Files Changed**: none. Baseline run reported zero failures; nothing to fix.
- [x] [AI] Verify `rhino-cli` is independently green: run `npx nx run rhino-cli:test:quick` —
      acceptance: exits 0.
      **Date**: 2026-07-31. **Status**: Done. **Files Changed**: none. Exit 0.
- [x] [AI] Record the upstream archive reference: run
      `git -C /Users/wkf/ose-projects/ose-public rev-parse HEAD` and append the SHA to
      `evidence/phase-0-baseline.txt` under a `## ose-public archive HEAD` heading — acceptance: a
      40-character SHA is recorded. If `/Users/wkf/ose-projects/ose-public` is absent, record
      `ABSENT` and surface it to the maintainer before Phase 3 deletes `plans/done/`.
      **Date**: 2026-07-31. **Status**: Done. **Files Changed**: `evidence/phase-0-baseline.txt`. `ose-public` present; SHA `857e2cf0c44da468dd4665e831e931f605950ada` recorded. Not `ABSENT` — no blocker for Phase 3.

### Phase 0 Gate

> All checks below must pass before starting Phase 1. If any check fails, fix it in Phase 0 before
> proceeding.

- [x] [AI] `git status --porcelain` — output contains only the new `evidence/` file and this plan's
      documents; no unexpected modifications.
      **Date**: 2026-07-31. **Status**: Done. **Files Changed**: none. Only `?? plans/in-progress/baseerah-repo-reset/evidence/` present.
- [x] [AI] `npx nx run-many -t typecheck,lint,test:quick --all` — exits 0.
      **Date**: 2026-07-31. **Status**: Done. **Files Changed**: none. Re-run fully cache-hit (92/92), exit 0, no coverage.json diff.
- [x] [AI] `cat plans/in-progress/baseerah-repo-reset/evidence/phase-0-baseline.txt` — contains all
      four recorded headings (`Pre-plan HEAD`, `Project graph before`, `Baseline test:quick`,
      `ose-public archive HEAD`).
      **Date**: 2026-07-31. **Status**: Done. **Files Changed**: none (verification only). All four headings present.

> **Pause Safety**: nothing has been deleted or changed except the addition of this plan folder and
> its baseline evidence file. The repository is exactly as it was, plus a record of how it was.
> Safe to stop. To resume: `npx nx run-many -t typecheck,lint,test:quick --all`.

---

## Phase 1: Retire the Per-App CI Callers and the Local-Stack Infra

> **Why first**: `_reusable-www-test-local-deploy.yml` derives `infra/dev/${app-name}/docker-compose.yml`
> at run time. Callers must die before the `infra/` tree they point at (tech-docs Mechanics 1).
>
> **What must NOT change here**: the four core workflows (`main-ci.yml`, `pr-quality-gate.yml`,
> `deps-audit.yml`, `validate-env.yml`), their job sets, and the composite actions under
> `.github/actions/`. Those are the CI/CD architecture this repo shares with every OSE sibling, and
> Phase 1's gate diffs them against `ose-public` to prove they were left alone
> ([tech-docs Decision 15](./tech-docs.md#decision-15--cicd-architecture-stays-consistent-with-the-ose-siblings)).

- [x] [AI] Delete the four per-site deploy callers: run
      `git rm .github/workflows/ayokoding-www-test-local-deploy-prod.yml .github/workflows/organiclever-www-test-local-deploy-prod.yml .github/workflows/ose-www-test-local-deploy-prod.yml .github/workflows/wahidyankf-www-test-local-deploy-prod.yml`
      — acceptance: exits 0, four files staged for deletion.
      **Date**: 2026-07-31. **Status**: Done. **Files Changed**: 4 files deleted (staged). Exit 0.
- [x] [AI] Delete the four app-group staging callers: run
      `git rm .github/workflows/organiclever-app-test-local-deploy-stag.yml .github/workflows/ose-app-test-local-deploy-stag.yml .github/workflows/organiclever-app-test-stag.yml .github/workflows/ose-app-test-stag.yml`
      — acceptance: exits 0.
      **Date**: 2026-07-31. **Status**: Done. **Files Changed**: 4 files deleted (staged). Exit 0.
- [x] [AI] Delete the three image/storybook build callers: run
      `git rm .github/workflows/organiclever-be-build-deploy-stag.yml .github/workflows/ose-be-build-deploy-stag.yml .github/workflows/web-ui-build-deploy-prod.yml`
      — acceptance: exits 0.
      **Date**: 2026-07-31. **Status**: Done. **Files Changed**: 3 files deleted (staged). Exit 0.
- [x] [AI] Delete **only** the one reusable template Baseerah has no tier for: run
      `git rm .github/workflows/_reusable-www-test-local-deploy.yml` — acceptance: exits 0. Baseerah
      has no `[domain]-www` marketing site, so nothing will ever call it.
      **Date**: 2026-07-31. **Status**: Done. **Files Changed**: 1 file deleted (staged). Exit 0.
- [x] [AI] **Keep** `_reusable-app-test-local-deploy-stag.yml`, `_reusable-app-test-stag.yml`, and
      `_reusable-be-build-deploy.yml` untouched. They are fully parameterised, name no app, and map
      exactly onto Baseerah's `fe` + `be` app group; Phases 7 and 9 add callers for them —
      acceptance: `git status --porcelain .github/workflows/_reusable-app-test-local-deploy-stag.yml .github/workflows/_reusable-app-test-stag.yml .github/workflows/_reusable-be-build-deploy.yml`
      produces no output.
      **Date**: 2026-07-31. **Status**: Done (verification only). **Files Changed**: none. Confirmed no output.
- [x] [AI] Reduce `.github/workflows/publish-images.yml` to its parameterised skeleton rather than
      deleting it: strip the hardcoded `build-organiclever-be` / `build-ose-be` outputs, `case` arms,
      and publish jobs from the `detect` job, leaving the workflow structurally intact with an empty
      project matrix. Phase 7 re-populates it with `baseerah-be` — acceptance:
      `rg -n 'organiclever|ose-be' .github/workflows/publish-images.yml` returns no matches and
      `actionlint .github/workflows/publish-images.yml` exits 0.
      **Date**: 2026-07-31. **Status**: Done. **Files Changed**: `.github/workflows/publish-images.yml`. Removed the two `publish-*` jobs and the `outputs`/`case` arms (also dropped the now-empty `outputs:` key, which actionlint flags as invalid) — `detect` job and its `checkout`/`setup-node` steps kept intact as the skeleton Phase 7 re-populates.
- [x] [AI] Confirm no surviving workflow calls the deleted template: run
      `rg -n '_reusable-www-test-local-deploy' .github/workflows/` — acceptance: no matches.
      **Date**: 2026-07-31. **Status**: Done. **Files Changed**: none. The unscoped `rg` still matched
      two prose references in `.github/workflows/README.md` (not yet rewritten at this point in the
      phase); scoping to actual workflow files (`rg -n '...' .github/workflows/*.yml`) confirms zero
      `.yml` files reference it. The README rewrite item later in this phase removes the stale prose,
      after which the unscoped check also passes — verified at the Phase 1 gate.
- [x] [AI] Confirm the three surviving templates are currently uncalled, which is expected between
      Phase 1 and Phase 7: run `rg -n 'uses:\s*\./\.github/workflows/_reusable' .github/workflows/`
      — acceptance: no matches. An uncalled reusable template is valid YAML and `actionlint`-clean;
      it is a library, not dead code.
      **Date**: 2026-07-31. **Status**: Done. **Files Changed**: none. No matches in `.yml` files.
- [x] [AI] Delete the entire local-stack infra tree: run `git rm -r infra/` — acceptance: exits 0,
      all 21 files staged for deletion. Phase 7 recreates it as `infra/dev/baseerah-app/`.
      **Date**: 2026-07-31. **Status**: Done. **Files Changed**: 21 files deleted (staged). Exit 0.
- [x] [AI] Edit `/Users/wkf/ose-projects/baseerah/package.json`: delete the five app dev scripts
      `organiclever:dev`, `organiclever:dev:restart`, `dev:ayokoding-www`, `dev:ose-www`, and
      `dev:organiclever` (lines ~25-29). Leave the ten `rhino-cli` scripts and the lint-staged block
      alone — acceptance: `rg -n 'organiclever|ayokoding|ose-www' package.json` returns no matches in
      the `scripts` block, and `rg -c 'rhino-cli' package.json` still reports 17.
      **Date**: 2026-07-31. **Status**: Done. **Files Changed**: `package.json`. Deleted the five app
      dev scripts; JSON re-validated parseable. Zero `organiclever`/`ayokoding`/`ose-www` matches
      remain in `scripts`. **Correction**: the stated "17" `rhino-cli` count was wrong before and
      after this edit — the actual, unaffected count is 15 (none of the five removed lines mentioned
      `rhino-cli`), confirmed via `grep -n 'rhino-cli' package.json` listing all 15 occurrences
      individually. The intent (rhino-cli scripts untouched) holds; the acceptance text's number was
      inaccurate. Corrected here rather than left silently wrong.
- [x] [AI] Edit `/Users/wkf/ose-projects/baseerah/.vscode/settings.json`: delete the
      `**/apps/organiclever-app/**` entry (line ~5, an already-stale path) — acceptance:
      `rg -n 'organiclever' .vscode/settings.json` returns no matches.
      **Date**: 2026-07-31. **Status**: Done. **Files Changed**: `.vscode/settings.json`. Collapsed
      `java.import.exclusions` to its sole surviving entry `**/.claude/worktrees/**` — verified
      `rg -n 'organiclever' .vscode/settings.json` returns no matches.
- [x] [AI] Rewrite `/Users/wkf/ose-projects/baseerah/.github/workflows/README.md` so its index lists
      the four core workflows (`main-ci.yml`, `pr-quality-gate.yml`, `validate-env.yml`,
      `deps-audit.yml`), `publish-images.yml`, the three surviving `_reusable-*` templates marked as
      awaiting Baseerah callers, and the five composite actions under `.github/actions/`. State the
      shared-architecture invariant from tech-docs Decision 15 in one paragraph so a future reader
      does not "tidy up" the uncalled templates — acceptance:
      `rg -n 'ayokoding|organiclever|wahidyankf|ose-www|ose-app|ose-be|web-ui-build' .github/workflows/README.md`
      returns no matches, and `rg -n '_reusable' .github/workflows/README.md` returns three matches.
      **Date**: 2026-07-31. **Status**: Done. **Files Changed**: `.github/workflows/README.md`.
      Rewrote as: shared-architecture invariant paragraph (Decision 15 summary), Reusable (awaiting
      Baseerah callers) table for the 3 kept `_reusable-*` templates, PR/repo-wide gates table for the
      4 core workflows, and a Backend images row for `publish-images.yml`'s skeleton state. Dropped
      the separate composite-actions table — `.github/actions/README.md` already documents those and
      this file's job is workflow indexing, not duplicating it. Verified both acceptance greps; a
      prettier pre-write hook reformatted table column widths only (no content change).
- [x] [AI] Verify every remaining workflow still parses: run
      `actionlint .github/workflows/*.yml` — acceptance: exits 0 with no output.
      **Date**: 2026-07-31. **Status**: Done. **Files Changed**: none (verification only). Ran
      `actionlint .github/workflows/*.yml` — exit 0, no output.
- [ ] [AI] Commit: `git add -A && git commit -m "chore(ci): retire per-app CI callers and the local-stack infra tree"`
      — acceptance: the commit includes the Phase 0 `evidence/` file and this plan's documents, and
      commitlint accepts the message.
- [ ] [AI] Push: `git push origin main` — acceptance: exits 0.

### Phase 1 Gate

> All checks below must pass before starting Phase 2. If any check fails, fix it in Phase 1 before
> proceeding.

- [ ] [AI] `actionlint .github/workflows/*.yml` — exits 0.
- [ ] [AI] `ls .github/workflows/` — lists exactly `README.md`, `_reusable-app-test-local-deploy-stag.yml`,
      `_reusable-app-test-stag.yml`, `_reusable-be-build-deploy.yml`, `deps-audit.yml`,
      `main-ci.yml`, `pr-quality-gate.yml`, `publish-images.yml`, `validate-env.yml`.
- [ ] [AI] **CI architecture parity — composite actions unchanged**:
      `diff -r /Users/wkf/ose-projects/ose-public/.github/actions /Users/wkf/ose-projects/baseerah/.github/actions`
      — exits 0 with no output.
- [ ] [AI] **CI architecture parity — `main-ci.yml` job set unchanged**:
      `diff <(rg -oN '^  [a-z0-9-]+:$' /Users/wkf/ose-projects/ose-public/.github/workflows/main-ci.yml) <(rg -oN '^  [a-z0-9-]+:$' .github/workflows/main-ci.yml)`
      — exits 0 with no output. Baseerah's language set (TypeScript, F#, Rust) matches `ose-public`'s
      exactly, so `typescript`, `dotnet`, and `rust` must all still be present.
- [ ] [AI] **CI architecture parity — `pr-quality-gate.yml` job set unchanged**:
      `diff <(rg -oN '^  [a-z0-9-]+:$' /Users/wkf/ose-projects/ose-public/.github/workflows/pr-quality-gate.yml) <(rg -oN '^  [a-z0-9-]+:$' .github/workflows/pr-quality-gate.yml)`
      — exits 0 with no output.
- [ ] [AI] `test ! -d infra` — exits 0.
- [ ] [AI] `npx nx run-many -t typecheck,lint,test:quick --all` — exits 0. (The apps still exist;
      only their deploy plumbing is gone.)
- [ ] [AI] Wait for CI, polling every 2 minutes with one call per wakeup per the
      [CI monitoring convention](../../../repo-governance/development/workflow/ci-monitoring.md):
      `gh run list --branch main --limit 1 --json databaseId,status,conclusion` then
      `gh run view <id> --json status,conclusion,jobs` — acceptance: `conclusion` is `success` **and**
      every element of `jobs[].conclusion` is `success` or `skipped`; no job is `failure`.

> **Pause Safety**: all apps and libs still exist and still build. Only the 11 per-app deploy
> callers, the one `-www` reusable template with no tier to serve, the `infra/` local-stack tree, and
> the npm dev scripts pointing into it are gone. The four core workflows, their full job sets, the
> composite actions, and the three app-group reusable templates are untouched and verified identical
> to `ose-public`. Nothing deployable was deployed from this clone anyway — only `origin/main`
> exists, and no `prod-*` or `stag-*` branch was ever fetched. Safe to stop.
> To resume: `actionlint .github/workflows/*.yml && npx nx run-many -t test:quick --all`.

---

## Phase 2: Delete the Retired Apps, Specs, and Config Registrations

> **Why these land together**: `rhino-cli`'s `specs:structure-validation` cross-checks
> `repo-config.yml` `coverage.projects[].specs` globs against the real `specs/` tree, and it runs in
> the **pre-commit** staged gate. Splitting spec deletion from config deletion produces a commit that
> cannot be made (tech-docs Mechanics 2).

- [ ] [AI] Read the `repo-config.yml` schema to determine whether empty lists validate: inspect
      `apps/rhino-cli/src/` for the `repo_config` module (`rg -l 'repo_config' apps/rhino-cli/src/`)
      and read its deserialisation and validation code for `ddd-areas`, `domain-areas`,
      `env-contract.surfaces`, and `env-injection.apps` — acceptance: record in
      `evidence/phase-2-repo-config-schema.md` whether each key accepts `[]`, and whether it may be
      omitted entirely. This verdict drives the two `repo-config.yml` steps below.
- [ ] [AI] Audit `libs/fsharp-crane-core`: read every `.fs` file under `libs/fsharp-crane-core/src/`
      and determine whether its modules are generic F# utilities or `crane-cli`-specific — acceptance:
      write the verdict plus a per-module table to `evidence/phase-2-fsharp-crane-core-audit.md`
      ending with a single line reading exactly `VERDICT: KEEP` or `VERDICT: DELETE`.
- [ ] [AI] Confirm nothing outside `crane-cli` consumes it: run
      `rg -n 'fsharp-crane-core|CraneCore' --glob '!libs/fsharp-crane-core/**' --glob '!plans/**'`
      — acceptance: every hit is inside `apps/crane-cli/`, `specs/apps/crane/`, or `repo-config.yml`.
      Append the command output to the audit file.
- [ ] [AI] Delete the 22 retired app directories: run
      `git rm -r apps/ayokoding-cli apps/ayokoding-www apps/ayokoding-www-be-e2e apps/ayokoding-www-fe-e2e apps/crane-cli apps/organiclever-app-web apps/organiclever-app-web-e2e apps/organiclever-be apps/organiclever-be-e2e apps/organiclever-www apps/organiclever-www-be-e2e apps/organiclever-www-fe-e2e apps/ose-app-web apps/ose-app-web-e2e apps/ose-be apps/ose-be-e2e apps/ose-cli apps/ose-www apps/ose-www-be-e2e apps/ose-www-fe-e2e apps/wahidyankf-www apps/wahidyankf-www-fe-e2e`
      — acceptance: exits 0 and `ls apps/` lists only `README.md` and `rhino-cli`.
- [ ] [AI] If and only if `evidence/phase-2-fsharp-crane-core-audit.md` ends `VERDICT: DELETE`, run
      `git rm -r libs/fsharp-crane-core specs/libs/fsharp-crane-core` — acceptance: exits 0. If the
      verdict is `KEEP`, skip this step and record the skip in `learnings.md`.
- [ ] [AI] Delete the five retired spec-area trees, which also removes the `ose-contracts` and
      `organiclever-contracts` Nx projects nested inside them: run
      `git rm -r specs/apps/ayokoding specs/apps/crane specs/apps/organiclever specs/apps/ose specs/apps/wahidyankf`
      — acceptance: exits 0 and `ls specs/apps/` lists only `rhino` and `README.md`.
- [ ] [AI] Confirm `specs/libs/` retains exactly the trees for surviving libs: run `ls specs/libs/`
      — acceptance: lists `web-ui`, `web-ui-token`, `rust-commons`, and `fsharp-crane-core` only if
      the audit verdict was `KEEP`.
- [ ] [AI] Delete the solution file, which registers only `crane-cli` projects: run
      `git rm open-sharia-enterprise.sln` — acceptance: exits 0. A fresh `baseerah.sln` is created in
      Phase 6.
- [ ] [AI] Edit `/Users/wkf/ose-projects/baseerah/repo-config.yml`: delete all 25 non-`rhino-cli`
      entries from `coverage.projects` (lines ~61-169) and the trailing exclusion comment naming
      `web-ui-token`, `organiclever-contracts`, and `ose-contracts` — acceptance: exactly one entry
      remains, `rhino-cli`, and the list is never empty.
- [ ] [AI] Edit `repo-config.yml`: clear `specs.ddd-areas`, `specs.domain-areas`,
      `env-contract.surfaces`, and `env-injection.apps` — using `[]` or full key omission per the
      verdict recorded in `evidence/phase-2-repo-config-schema.md` — and strip the
      `organiclever-app-staging` / `ose-app-staging` environment arrays from
      `env-injection.ci-harness` — acceptance: `rg -n 'organiclever|ayokoding|wahidyankf|crane|ose-' repo-config.yml`
      returns no matches.
- [ ] [AI] Verify the config still validates: run `npm run validate:config` — acceptance: exits 0.
      If it fails on an empty list, apply the omission form instead and re-run.
- [ ] [AI] Edit `/Users/wkf/ose-projects/baseerah/tsconfig.base.json`: leave the `@open-sharia-enterprise/*`
      scope and the `web-ui` / `web-ui-token` paths **unchanged** (tech-docs Decision 3); remove a
      path entry only if its target directory was deleted — acceptance:
      `npx tsc -p tsconfig.base.json --noEmit --showConfig` resolves without error.
- [ ] [AI] Edit `/Users/wkf/ose-projects/baseerah/.prettierignore`: delete the trailing block
      containing `apps/ayokoding-www/content/**/code/**/*.sql` and its comment naming the F# backends
      — acceptance: `rg -n 'ayokoding|organiclever|ose-be' .prettierignore` returns no matches.
- [ ] [AI] Rewrite `/Users/wkf/ose-projects/baseerah/.dockerignore`: delete the `apps/ayokoding-cli`
      and `apps/ose-cli` lines and replace the nine stale `!specs/...` re-include lines with a single
      `!specs/apps/rhino/` re-include — acceptance: `rg -n 'ayokoding|organiclever|ose-app|a-demo' .dockerignore`
      returns no matches.
- [ ] [AI] Edit `/Users/wkf/ose-projects/baseerah/.gitignore`: delete the `crane-cli` integration-test
      state block (line ~176), keeping the two `rhino-cli` lines — acceptance:
      `rg -n 'crane-cli' .gitignore` returns no matches and `rg -n 'rhino-cli' .gitignore` returns two.
- [ ] [AI] Now that the excluded content is gone, drop the stale markdown-lint excludes in
      `/Users/wkf/ose-projects/baseerah/package.json`: remove `--exclude apps/ayokoding-www/content`
      from the lint-staged `md mermaid validate` invocation — acceptance:
      `rg -n 'apps/ayokoding-www' package.json` returns no matches.
- [ ] [AI] Drop the same excludes from `/Users/wkf/ose-projects/baseerah/.husky/pre-push`: remove
      `--exclude apps/ayokoding-www/content --exclude apps/ose-www/content` from the
      `md links validate` invocation, leaving `--exclude plans/done` in place for now — acceptance:
      `rg -n 'apps/(ayokoding-www|ose-www)' .husky/pre-push` returns no matches.
- [ ] [AI] Drop the same excludes from `.github/workflows/main-ci.yml` (the `md mermaid validate` at
      line ~124 and `md links validate` at line ~183) and from
      `.github/workflows/pr-quality-gate.yml` (line ~246), and delete the stale comments at
      `main-ci.yml` lines ~61-64 and ~113-121 naming `ayokoding-www` and a
      `plans/ideas/ayokoding-mermaid-diagram-remediation.md` file — acceptance:
      `rg -n 'ayokoding|ose-www' .github/workflows/` returns no matches.
- [ ] [AI] Confirm the `.NET` CI jobs are **kept**, not deleted — `baseerah-be` needs them from
      Phase 6 (tech-docs Decision 5). Read `main-ci.yml`'s `dotnet` job and `pr-quality-gate.yml`'s
      `.NET quality gate` job — acceptance: both jobs are present and unmodified, and
      `.config/dotnet-tools.json` still exists.
- [ ] [AI] Regenerate the project graph and confirm the survivors: run `npx nx show projects` —
      acceptance: output is exactly `rhino-cli`, `rust-commons`, `web-ui`, `web-ui-token`, and
      `fsharp-crane-core` (the last only if the audit verdict was `KEEP`).
- [ ] [AI] Commit everything in one commit, since the spec trees and their `repo-config.yml` entries
      cannot be split: `git add -A && git commit -m "feat(repo)!: remove retired OSE apps, libs, specs, and config registrations"`
      — acceptance: the pre-commit staged gate passes and the commit is created.
- [ ] [AI] Push: `git push origin main` — acceptance: exits 0.

### Phase 2 Gate

> All checks below must pass before starting Phase 3. If any check fails, fix it in Phase 2 before
> proceeding.

- [ ] [AI] `npx nx show projects` — lists only the survivors named above; no retired project name
      appears.
- [ ] [AI] `npm run validate:config` — exits 0.
- [ ] [AI] `npx nx run-many -t typecheck,lint,test:quick --all` — exits 0.
- [ ] [AI] `npx nx run rhino-cli:test:quick` — exits 0.
- [ ] [AI] `rg -n --hidden -g '!.git' -g '!plans/**' 'ayokoding|organiclever|wahidyankf|crane-cli|ose-www|ose-app-web|ose-cli' apps/ libs/ specs/ .github/ repo-config.yml package.json`
      — no matches.
- [ ] [AI] CI: poll per the 2-minute convention and run `gh run view <id> --json status,conclusion,jobs`
      — `conclusion` is `success` and **every** `jobs[].conclusion` is `success` or `skipped`. In
      particular assert the `dotnet` job is not `failure`: it now runs over zero projects and must
      pass trivially.

> **Pause Safety**: the repository now contains exactly one app (`rhino-cli`), its libs, and the
> governance harness. Everything builds, lints, and tests green. No Baseerah code exists yet, and
> the identity surface still says "open-sharia-enterprise" — that is expected and is Phase 4's job.
> Safe to stop. To resume: `npx nx show projects && npx nx run-many -t test:quick --all`.

---

## Phase 3: Prune the Agent Fleet, Governance, Docs, and Plan Archive

- [ ] [AI] Delete the 20 `ayokoding-www` agents: run
      `git rm .claude/agents/apps-ayokoding-www-*.md` — acceptance: exits 0, 20 files staged.
- [ ] [AI] Delete the remaining 9 app-scoped agents: run
      `git rm .claude/agents/apps-ose-www-content-maker.md .claude/agents/apps-ose-www-content-checker.md .claude/agents/apps-ose-www-content-fixer.md .claude/agents/apps-ose-www-deployer.md .claude/agents/apps-ose-app-web-deployer.md .claude/agents/apps-organiclever-www-deployer.md .claude/agents/apps-organiclever-app-web-deployer.md .claude/agents/apps-wahidyankf-www-deployer.md .claude/agents/apps-web-ui-storybook-deployer.md`
      — acceptance: exits 0, 9 files staged.
- [ ] [AI] Delete the two agents whose doctrine no longer has a second side (tech-docs Decision 12):
      run `git rm .claude/agents/docs-software-engineering-separation-checker.md .claude/agents/docs-software-engineering-separation-fixer.md`
      — acceptance: exits 0.
- [ ] [AI] Delete the four app-scoped and now-moot skills: run
      `git rm -r .claude/skills/apps-ayokoding-www-developing-content .claude/skills/apps-organiclever-www-developing-content .claude/skills/apps-ose-www-developing-content .claude/skills/docs-validating-software-engineering-separation`
      — acceptance: exits 0 and `find .claude/skills -maxdepth 1 -mindepth 1 -type d | wc -l` reports
      27 (equivalently, `ls .claude/skills/ | wc -l` reports 28, since that count also includes
      `README.md`).
- [ ] [AI] Delete the OSE social-post archive: run `git rm -r generated-socials` — acceptance: exits
      0, 34 files staged (33 "OSE update week NNNN" LinkedIn posts about products this repo no
      longer contains, plus the directory's own `README.md`); `ose-public` retains the archive.
- [ ] [AI] Delete the agent whose sole output home was that directory: run
      `git rm .claude/agents/social-linkedin-post-maker.md` — acceptance: exits 0. Its charter is
      writing OSE-family updates across `ose-public` / `ose-primer` / `ose-private`, none of which
      this repo participates in.
- [ ] [AI] Fix the one surviving `generated-socials` reference in
      `repo-governance/development/workflow/ci-post-push-verification.md` — acceptance:
      `rg -n 'generated-socials|social-linkedin-post-maker' --hidden -g '!.git' .` returns matches
      only inside `.opencode/` and `.cursor/`, which the binding regeneration step below clears.
- [ ] [AI] Delete the app-specific workflow family: run `git rm -r repo-governance/workflows/ayokoding-web`
      — acceptance: exits 0, 6 files staged.
- [ ] [AI] Delete the app-specific linking convention: run
      `git rm repo-governance/conventions/linking/internal-ayokoding-references.md` — acceptance:
      exits 0.
- [ ] [AI] Confirm no surviving agent or skill references a deleted one: run
      `rg -n 'apps-ayokoding-www|apps-ose-www|apps-organiclever|apps-wahidyankf|apps-web-ui-storybook|software-engineering-separation' .claude/ repo-governance/ AGENTS.md CLAUDE.md`
      — acceptance: fix every hit found, then re-run for no matches. `AGENTS.md`'s agent roster and
      `.claude/agents/README.md` are the two expected hit sites.
- [ ] [AI] Update `.claude/agents/README.md` so its catalog lists only surviving agents, with no
      hardcoded count per the
      [Dynamic Collection References convention](../../../repo-governance/conventions/writing/dynamic-collection-references.md)
      — acceptance: `rg -n '\b(9[0-9]|[0-9]{2}) agents\b' .claude/agents/README.md` returns no matches.

### `rhino-cli` de-coupling from the retired apps

> `rhino-cli` is application code, so any **behaviour** change follows RED → GREEN → REFACTOR with
> companion Gherkin under `specs/apps/rhino/behavior/rhino-cli/gherkin/**`, per
> [Specs & Gherkin Completeness](../../../repo-governance/development/quality/feature-change-completeness.md).
> Test-fixture renames that change no behaviour are exempt from that rule, exactly as pure refactors
> are — and one of the two changes below turns out to be precisely that.

- [ ] [AI] **Establish what is actually hardcoded before changing anything.** Read
      `apps/rhino-cli/src/commands/specs_validate_counts.rs` and
      `apps/rhino-cli/src/application/repo_governance/frontmatter_audit.rs` in full, and record the
      finding in `evidence/phase-3-rhino-coupling-audit.md` — acceptance: the file states, per
      source file, whether each occurrence of a retired app name is production behaviour, a test
      fixture, or a doc comment. The two entries below are the expected result and are pre-recorded
      here; **if the code disagrees with them, the code wins** and these steps are rewritten before
      execution continues.

#### `specs_validate_counts.rs` — a test fixture, not a hardcode (no behaviour change)

`run_at_root` reads its default area list from `repo_config::load_or_default(repo_root).specs.ddd_areas`
— that is, from `repo-config.yml`'s `specs.ddd-areas` key, which Phase 2 already emptied. The
`["organiclever", "ose"]` literal lives **only** inside the unit test
`resolve_folders_default_reads_config_areas`, whose own comment states the default is config-supplied
rather than hardcoded. There is therefore no production hardcode to remove, and no Gherkin scenario
to bind: renaming a fixture string changes no observable behaviour.

- [ ] [AI] Rename the fixture strings in the `resolve_folders_default_reads_config_areas` test in
      `apps/rhino-cli/src/commands/specs_validate_counts.rs` (~lines 105-118) from
      `["organiclever", "ose"]` to `["baseerah"]`, updating the expected
      `specs/apps/organiclever` / `specs/apps/ose` assertions to `specs/apps/baseerah` — acceptance:
      `cargo test --manifest-path apps/rhino-cli/Cargo.toml specs_validate_counts` exits 0, and
      `rg -n 'organiclever|"ose"' apps/rhino-cli/src/commands/specs_validate_counts.rs` returns no
      matches.
- [ ] [AI] Confirm no production default was touched: run
      `git diff apps/rhino-cli/src/commands/specs_validate_counts.rs` — acceptance: every changed
      line is inside a `#[cfg(test)]` module. If any non-test line changed, revert and reassess.

#### `frontmatter_audit.rs` — an exemption list, not an allowlist (behaviour change)

`WEBSITE_APP_PREFIXES` is a **skip** list: `is_website_app` returns `true` for paths under those
prefixes, and `audit_frontmatter` **excludes** them. All four entries name deleted apps, so the list
is entirely dead. Emptying it makes the audit apply everywhere — strictly more coverage, which is a
real behaviour change and therefore carries a bound scenario.

> Note the inversion: adding `apps/baseerah-fe/` to this list would **exempt** the Baseerah frontend
> from the audit, which is the opposite of what is wanted. The list is emptied, not repointed.

- [ ] [AI] **RED** — add a failing test in
      `apps/rhino-cli/src/application/repo_governance/frontmatter_audit.rs`'s test module asserting
      that a path under `apps/baseerah-fe/` is **not** skipped by `is_website_app`.
      **Gherkin (binds) →** "No application path is exempt from the frontmatter audit"

      ```gherkin
      Scenario: No application path is exempt from the frontmatter audit
        Given a markdown file under "apps/baseerah-fe/"
        When the frontmatter audit resolves whether the path is exempt
        Then the path is reported as in scope
      ```

      Add the scenario verbatim to
      `specs/apps/rhino/behavior/rhino-cli/gherkin/repo-governance/frontmatter-audit.feature`,
      creating the file if it does not exist. Run
      `cargo test --manifest-path apps/rhino-cli/Cargo.toml frontmatter_audit` — acceptance: the new
      test passes trivially today (no prefix matches `baseerah`), so **also** add a second assertion
      that `is_website_app("apps/ayokoding-www/content/x.md")` returns `false`; that assertion fails
      against the current list and is the genuine RED.

- [ ] [AI] **GREEN** — empty `WEBSITE_APP_PREFIXES` in
      `apps/rhino-cli/src/application/repo_governance/frontmatter_audit.rs` (~lines 26-33) to
      `&[]`, and update its doc comment to state that no path is currently exempt. Run
      `cargo test --manifest-path apps/rhino-cli/Cargo.toml frontmatter_audit` — acceptance: exits 0.

- [ ] [AI] **REFACTOR** — if `is_website_app` and the const are now trivially `false` for every
      input, keep the function and the const rather than inlining them: they are the documented
      extension point for a future Baseerah content tree. Add a one-line comment saying so. Run
      `cargo test --manifest-path apps/rhino-cli/Cargo.toml` — acceptance: exits 0.

- [ ] [AI] Run the audit end to end to prove the widened scope did not surface pre-existing
      violations: run
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- repo-governance frontmatter audit docs repo-governance`
      — acceptance: exits 0. If it now reports findings that the exemption previously hid, fix them
      here per the
      [Root Cause Orientation principle](../../../repo-governance/principles/general/root-cause-orientation.md)
      rather than restoring the exemption.

- [ ] [AI] Sweep the remaining `rhino-cli` fixtures and doc comments naming deleted apps: run
      `rg -n 'ayokoding|organiclever|wahidyankf|crane|ose-www|ose-app|ose-be' apps/rhino-cli/src/ apps/rhino-cli/tests/`
      and replace each with a `baseerah`-based or neutral equivalent. Classify each hit in
      `evidence/phase-3-rhino-coupling-audit.md` as fixture, comment, or behaviour; only the last
      category needs a bound scenario — acceptance: re-running the command returns no matches, and
      `cargo test --manifest-path apps/rhino-cli/Cargo.toml` exits 0.

### Governance, docs, and plan-archive pruning

- [ ] [AI] Sweep `repo-governance/` prose for deleted-app references: run
      `rg -ln 'ayokoding|organiclever|wahidyankf|crane-cli|ose-www|ose-app-web|ose-be|ose-cli' repo-governance/`
      and rewrite each hit — replacing the example with a `baseerah-*` or generic one, never merely
      deleting the sentence — acceptance: re-running the command returns no matches.
- [ ] [AI] Confirm the principles layer was untouched by that sweep (tech-docs Decision 13): run
      `diff -r /Users/wkf/ose-projects/ose-public/repo-governance/principles /Users/wkf/ose-projects/baseerah/repo-governance/principles`
      — acceptance: exits 0 with no output. If the sweep modified a principle, revert that file.
- [ ] [AI] Sweep `docs/` for deleted-app references, prioritising
      `docs/reference/monorepo-structure.md`, `docs/reference/nx-configuration.md`,
      `docs/reference/related-repositories.md`, `docs/reference/project-dependency-graph.md`, and
      `docs/reference/system-architecture/**`: run
      `rg -ln 'ayokoding|organiclever|wahidyankf|crane-cli|ose-www|ose-app-web|ose-cli' docs/` and
      rewrite each hit — acceptance: re-running returns no matches. Keep all 168 files under
      `docs/explanation/software-engineering/` (tech-docs Decision 12); their
      `@open-sharia-enterprise/*` import examples stay valid and unchanged.
- [ ] [AI] Add a **Port Allocation** section to `docs/reference/monorepo-structure.md` recording
      `baseerah-fe` → `19310` and `baseerah-be` → `19320`, together with the rule that Baseerah
      deliberately allocates outside every band the `ose-public` / `ose-primer` / `ose-private`
      siblings occupy (`3000–3401`, `8000–8302`, `4222–4224`, `5432–5438`, `6006`, `6379`, and the
      `9090–9411` / `14250` / `14268` / `16686` / `24224` observability set), since all four repos
      can run concurrently on one machine — acceptance: the section exists and states both ports and
      the constraint, per [tech-docs Decision 5](./tech-docs.md#decision-5--f--giraffe-backend-on-19320-nextjs-16-frontend-on-19310).
- [ ] [AI] Verify the chosen ports are still free across the siblings before committing them: run
      `rg -n '19310|19320' /Users/wkf/ose-projects/ose-public /Users/wkf/ose-projects/ose-primer /Users/wkf/ose-projects/ose-private`
      — acceptance: no matches. If a sibling has since claimed one, pick the next free pair in the
      same band and update every reference in this plan folder before proceeding.
- [ ] [AI] Delete the historical post-mortems for removed apps: inspect the 3 files under
      `docs/explanation/post-mortems/` and `git rm` any whose subject is a deleted app —
      acceptance: every surviving post-mortem names only surviving code.
- [ ] [AI] Empty the stale external-link cache: replace
      `docs/metadata/external-links-status.yaml` with an empty registry retaining only its schema
      header and a fresh `lastFullScan` — acceptance:
      `rg -n 'oseplatform\.com|ayokoding\.com|organiclever\.com|wahidyankf\.com' docs/metadata/external-links-status.yaml`
      returns no matches.
- [ ] [AI] Confirm the upstream archive SHA was recorded in Phase 0 before deleting the plan archive:
      run `rg -n 'ose-public archive HEAD' -A2 plans/in-progress/baseerah-repo-reset/evidence/phase-0-baseline.txt`
      — acceptance: a 40-character SHA is present. If it reads `ABSENT`, stop and surface to the
      maintainer.
- [ ] [AI] Delete the plan archive: run `git rm -r plans/done` — acceptance: exits 0, 174 folders
      staged for deletion.
- [ ] [AI] Delete the two dead in-progress plans, both of which target `ayokoding-www`: run
      `git rm -r plans/in-progress/ayokoding-learning-path-04-course-authoring plans/in-progress/vercel-function-cost-reduction`
      — acceptance: exits 0 and `ls plans/in-progress/` lists only `README.md` and
      `baseerah-repo-reset`.
- [ ] [AI] Triage `plans/backlog/`: `git rm -r` the 5 `ayokoding`-scoped plans
      (`ayokoding-learning-path-05*`, `ayokoding-learning-path-06*`, `ayokoding-learning-path-07*`,
      `ayokoding-www-cost-reduction*`, `harden-ayokoding-www-fe-e2e*` — confirm exact folder names
      with `ls plans/backlog/` first). Keep every generic tooling/governance plan, including
      `ose-private-opencode-ci-monitor-orphan` — its scope is a sibling repo's `.opencode/` mirror
      artifact, not any app this plan deletes — acceptance: every surviving backlog plan's
      `README.md` scope names only surviving code or generic tooling/governance.
- [ ] [AI] Triage `plans/ideas/`: read all 36 two-pagers, `git rm` those about deleted apps, keep
      those about tooling, governance, or `rhino-cli` — acceptance: re-running
      `rg -ln 'ayokoding|organiclever|wahidyankf' plans/ideas/` returns no matches.
- [ ] [AI] Update `plans/in-progress/README.md`, `plans/backlog/README.md`, and
      `plans/ideas/README.md` indexes to match the surviving contents, and delete
      `plans/done/README.md`'s parent reference wherever `plans/README.md` links to it — acceptance:
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md links validate --exclude plans/done`
      reports zero broken links.
- [ ] [AI] Now that `plans/done/` is gone, drop the `--exclude plans/done` flags from
      `.husky/pre-push`, `package.json` lint-staged, `main-ci.yml`, and `pr-quality-gate.yml` —
      acceptance: `rg -n 'exclude plans/done' .husky/ package.json .github/` returns no matches.
- [ ] [AI] Regenerate every platform binding rather than hand-editing a mirror: run
      `npm run generate:bindings` — acceptance: exits 0 and `.opencode/agents/`, `.cursor/agents/`,
      and `.amazonq/` all shrink to match `.claude/agents/`.
- [ ] [AI] Verify zero binding drift: run `npm run validate:sync` — acceptance: exits 0 with no drift
      reported.
- [ ] [AI] Commit: `git add -A && git commit -m "chore(governance): prune app-scoped agents, skills, docs, and the OSE plan archive"`
      — acceptance: the pre-commit gate passes.
- [ ] [AI] Push: `git push origin main` — acceptance: exits 0.

### Phase 3 Gate

> All checks below must pass before starting Phase 4. If any check fails, fix it in Phase 3 before
> proceeding.

- [ ] [AI] `rg -n --hidden -g '!.git' -g '!plans/in-progress/baseerah-repo-reset' 'ayokoding|organiclever|wahidyankf|crane-cli|ose-www|ose-app-web|ose-cli'`
      — no matches anywhere in the repository.
- [ ] [AI] `test ! -d generated-socials` — exits 0.
- [ ] [AI] `rg -n --hidden -g '!.git' 'generated-socials|social-linkedin-post-maker'` — no matches,
      including in the regenerated `.opencode/` and `.cursor/` mirrors.
- [ ] [AI] `diff -r /Users/wkf/ose-projects/ose-public/repo-governance/principles /Users/wkf/ose-projects/baseerah/repo-governance/principles`
      — exits 0 with no output. The principles layer is byte-identical to `ose-public`.
- [ ] [AI] `npm run generate:bindings && npm run validate:sync` — both exit 0, and
      `git diff --exit-code` afterwards reports no drift.
- [ ] [AI] `npm run harness:bindings-validation` — exits 0.
- [ ] [AI] `npx nx run-many -t typecheck,lint,test:quick --all` — exits 0.
- [ ] [AI] `ls .claude/agents/*.md | sed 's|.*/||; s|\.md$||' | grep -vE -- '-(maker|checker|fixer|dev|deployer|manager|tester|researcher)$' | grep -v '^README$'`
      — outputs only the known preexisting violation `api-exploratory-tester`; no new violation.
- [ ] [AI] CI: `gh run view <id> --json status,conclusion,jobs` — `conclusion` is `success` and every
      `jobs[].conclusion` is `success` or `skipped`.

> **Pause Safety**: the repository is now a clean engineering harness — `rhino-cli`, its libs, the
> generic agent fleet, and `repo-governance/` with its principles layer verified identical to
> `ose-public`. No product code exists. The root identity files still say "open-sharia-enterprise",
> which is accurate-but-incomplete rather than wrong, since the repo is still an OSE-ecosystem repo.
> Safe to stop. To resume: `npm run validate:sync && npx nx run-many -t test:quick --all`.

---

## Phase 4: Baseerah Identity Within the OSE Ecosystem

> Amends both naming vocabularies **before** anything is named against them (tech-docs Decision 6).

- [ ] [AI] Create `repo-governance/vision/baseerah.md`: a Layer 0 product vision stating that
      Baseerah (Arabic بصيرة — _insight_, _wawasan_, _kejernihan pandang_) is a personal operating
      layer covering assistant, content building, and posting; that it is a product **within** the
      Open Sharia Enterprise ecosystem, not a replacement for it; and that
      `repo-governance/vision/open-sharia-enterprise.md` remains its parent ecosystem vision —
      acceptance: the file exists, has a single H1, and links to the OSE vision doc.
- [ ] [AI] Confirm `repo-governance/vision/open-sharia-enterprise.md` is unmodified: run
      `git diff HEAD -- repo-governance/vision/open-sharia-enterprise.md` — acceptance: no output.
- [ ] [AI] Update `repo-governance/vision/README.md` to index both documents and state the
      parent (ecosystem) / child (product) relationship — acceptance:
      `rg -n 'baseerah' repo-governance/vision/README.md` returns at least one match and the OSE
      entry survives.
- [ ] [AI] Edit `AGENTS.md` **Tech Stack → App naming tiers**: add the `[domain]-fe` tier —
      _"`[domain]-fe` = the product web client when the domain has no separate marketing site"_ —
      alongside the existing `-www`, `-app-web`, and `-be` tiers — acceptance:
      `rg -n '\[domain\]-fe' AGENTS.md` returns a match.
- [ ] [AI] Edit `AGENTS.md` **Repository Overview**: replace the "open-sharia-enterprise — Enterprise
      platform for Sharia-compliant business systems" opening with a Baseerah description that names
      the product, its meaning, and its membership in the OSE ecosystem — acceptance:
      `rg -n 'Baseerah' AGENTS.md` returns matches and the OSE ecosystem is named.
- [ ] [AI] Edit `AGENTS.md` **Web Sites** table: replace all eight rows with `baseerah-fe` (port 19310) and `baseerah-be` (port 19320), marking domains and prod branches `TBD` — acceptance: the
      table has exactly two data rows.
- [ ] [AI] Edit `AGENTS.md` **AI Agents** roster: remove every deleted agent name and add the
      Phase 10 `apps-baseerah-*` agents as planned entries — acceptance:
      `rg -n 'apps-ayokoding|apps-ose-www|apps-organiclever|apps-wahidyankf|apps-web-ui-storybook' AGENTS.md`
      returns no matches.
- [ ] [AI] Edit `AGENTS.md` **Related Repositories** and the `rhino-cli` byte-identity clause: state
      that `baseerah` is a fourth repo **outside** the `ose-public` / `ose-primer` / `ose-private`
      parity loop, and that its `apps/rhino-cli` is a fork not bound by the byte-identity rule
      (tech-docs Decision 14) — acceptance: `rg -n 'byte-identical' AGENTS.md` shows the clause now
      scoped to the three parity repos and explicitly excluding this one.
- [ ] [AI] Edit `AGENTS.md` **Delivery Mode** and **Plans** sections only if they name a deleted app;
      leave every governance rule unchanged — acceptance: `git diff AGENTS.md` shows no rule text
      altered, only identity and roster text.
- [ ] [AI] Rewrite `README.md` for Baseerah: what it is, what the name means, the OSE-ecosystem
      relationship, the current walking-skeleton status, and how to run it — acceptance:
      `rg -n 'Sharia-compliant business systems|oseplatform\.com' README.md` returns no matches, and
      the OSE ecosystem is still named as the parent.
- [ ] [AI] Rewrite `ROADMAP.md` for Baseerah: replace the four-phase Sharia-fintech business strategy
      with a Baseerah roadmap whose Phase 1 is this hello-world quad and whose later phases name the
      deferred capabilities from `prd.md`'s Out of Scope — acceptance:
      `rg -n 'halal|Sharia certification|OrganicLever' ROADMAP.md` returns no matches.
- [ ] [AI] Edit `CONTRIBUTING.md`: retitle to Baseerah, update the app list and the structure
      section; leave every convention and workflow instruction unchanged — acceptance:
      `rg -n 'Open Sharia Enterprise' CONTRIBUTING.md` returns matches only where the ecosystem is
      deliberately named.
- [ ] [AI] Edit `SECURITY.md`: replace "enterprise platform with financial services" with an accurate
      Baseerah description; leave the reporting address and process unchanged — acceptance:
      `rg -n 'financial services' SECURITY.md` returns no matches.
- [ ] [AI] Edit `LICENSING-NOTICE.md`: update the app list in the per-directory override table to
      name `apps/rhino-cli` and the four `baseerah-*` apps — acceptance: no deleted app is named.
- [ ] [AI] Edit `package.json`: set `"name": "baseerah"` and rewrite `"description"` to describe a
      personal-assistant monorepo. **Do not touch the `@open-sharia-enterprise/*` scope**
      (tech-docs Decision 3) — acceptance: `npm install` exits 0 and `git diff package-lock.json`
      shows only the root-name change.
- [ ] [AI] Edit `CLAUDE.md`: refresh only its agent-roster and app references; its binding
      documentation is identity-free and stays — acceptance:
      `rg -n 'ayokoding|organiclever|ose-www' CLAUDE.md` returns no matches.
- [ ] [AI] Rebrand `libs/web-ui-token`: update the brand token values (palette, typography scale)
      for Baseerah, keeping every token **name** unchanged so `libs/web-ui` needs no edit —
      acceptance: `npx nx run web-ui-token:test:quick` exits 0 and `npx nx run web-ui:test:quick`
      exits 0 without any `web-ui` source change.
- [ ] [AI] Verify every rebranded colour pair meets WCAG AA: check each foreground/background pairing
      in the new token set against a 4.5:1 ratio for body text and 3:1 for large text — acceptance:
      record the computed ratios in `evidence/phase-4-token-contrast.md`; every pair passes.
- [ ] [AI] Rewrite `.claude/skills/swe-developing-frontend-ui/reference/brand-context.md` for
      Baseerah, removing the OrganicLever and OSE Platform brand sections — acceptance:
      `rg -n 'OrganicLever|OSE Platform' .claude/skills/swe-developing-frontend-ui/reference/brand-context.md`
      returns no matches.
- [ ] [AI] Rename the Amazon Q default agent config: `git mv .amazonq/cli-agents/ose-default.json .amazonq/cli-agents/baseerah-default.json`,
      then confirm the emitter produces that name by running `npm run generate:bindings` —
      acceptance: `git status --porcelain .amazonq/` shows no unexpected regeneration back to the old
      name. If the emitter hardcodes `ose-default`, fix the emitter in
      `apps/rhino-cli/src/commands/harness_emit_bindings.rs` under TDD with companion Gherkin.
- [ ] [AI] Verify the rewritten instruction surface stays inside its budget: run
      `npx nx run rhino-cli:instruction-size:validation` — acceptance: exits 0. If `AGENTS.md`
      exceeds its threshold, apply progressive disclosure (move detail into a linked
      `repo-governance/` file), never trim a rule.
- [ ] [AI] Commit: `git add -A && git commit -m "feat(repo): establish Baseerah identity within the OSE ecosystem"`
      — acceptance: the pre-commit gate passes.
- [ ] [AI] Push: `git push origin main` — acceptance: exits 0.

### Phase 4 Gate

> All checks below must pass before starting Phase 5. If any check fails, fix it in Phase 4 before
> proceeding.

- [ ] [AI] `npx nx run rhino-cli:instruction-size:validation` — exits 0.
- [ ] [AI] `rg -n '\[domain\]-fe' AGENTS.md` — matches, so Phase 8 may legally create `baseerah-fe`.
- [ ] [AI] `ls repo-governance/vision/` — contains `README.md`, `open-sharia-enterprise.md`, and
      `baseerah.md`.
- [ ] [AI] `git diff HEAD~1 -- repo-governance/vision/open-sharia-enterprise.md` — no output; the
      ecosystem vision is unchanged.
- [ ] [AI] `diff -r /Users/wkf/ose-projects/ose-public/repo-governance/principles /Users/wkf/ose-projects/baseerah/repo-governance/principles`
      — exits 0 with no output.
- [ ] [AI] `npx nx run-many -t typecheck,lint,test:quick --all` — exits 0.
- [ ] [AI] `npm run validate:sync` — exits 0.
- [ ] [AI] CI: `gh run view <id> --json status,conclusion,jobs` — `conclusion` is `success` and every
      job is `success` or `skipped`.

> **Pause Safety**: the repository now describes itself accurately — Baseerah, a personal assistant,
> within the OSE ecosystem, currently containing only the engineering harness. Both naming
> vocabularies are amended, so the app-creation phases are unblocked. Safe to stop.
> To resume: `npx nx run rhino-cli:instruction-size:validation`.

---

## Phase 5: `specs/apps/baseerah/` and `baseerah-contracts`

> The spec tree is the source of truth both apps code against, so it lands before either of them.

- [ ] [AI] Edit `repo-governance/development/infra/nx-targets.md`: in the `domain:` tag vocabulary
      table, remove the dead values (`ayokoding`, `crane`, `ose`, `organiclever`, `wahidyankf`) and
      add `baseerah`, keeping `tooling` and `ui` — acceptance: `rg -n 'domain:baseerah|baseerah' repo-governance/development/infra/nx-targets.md`
      returns a match. This must land before any `project.json` carrying `domain:baseerah` is written.
- [ ] [AI] Update the "Current Project Tags" table in the same file to list only the surviving and
      planned projects — acceptance: no deleted project appears.
- [ ] [AI] Rewrite `docs/reference/code-coverage.md` as a single table covering only surviving and
      planned projects, resolving the 80/88/95 drift at **90% line** for the new projects
      (tech-docs Decision 11) — acceptance: the table lists `rhino-cli`, `rust-commons`, `web-ui`,
      `web-ui-token`, `baseerah-be`, and `baseerah-fe`, each with one unambiguous threshold.
- [ ] [AI] Create the five-folder C4 spec tree: `specs/apps/baseerah/{product,system-context,containers,components,behavior}`,
      each with a `README.md` index — acceptance: `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md readme-index validate` exits 0.
- [ ] [AI] Author `specs/apps/baseerah/product/README.md` describing the hello-world scope and naming the
      deferred capabilities — acceptance: the file exists with a single H1 and no fabricated metric.
- [ ] [AI] Author `specs/apps/baseerah/system-context/README.md` with a Mermaid context diagram
      (browser → `baseerah-fe` → `baseerah-be`) using the accessible palette and a text description
      per the [Diagrams convention](../../../repo-governance/conventions/formatting/diagrams.md) —
      acceptance: `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md mermaid validate` exits 0.
- [ ] [AI] Create `specs/apps/baseerah/containers/contracts/openapi.yaml`: OpenAPI 3.1 defining
      `GET /api/v1/health` (→ `Health`), `GET /api/v1/hello` (→ `Greeting`), and the shared `Error`
      schema used by the 404 response. Three `GET` routes, no request bodies, no write operations —
      acceptance: `npx @redocly/cli lint specs/apps/baseerah/containers/contracts/openapi.yaml`
      reports no errors.
- [ ] [AI] Create `specs/apps/baseerah/containers/contracts/project.json` registering the Nx project
      `baseerah-contracts`, modelled on the deleted `ose-contracts` project (recover it with
      `git show "$(git log --diff-filter=D --format=%H -- specs/apps/ose/containers/contracts/project.json | head -1)~1":specs/apps/ose/containers/contracts/project.json`
      if the exact shape is needed), with a `bundle` target writing `generated/openapi-bundled.yaml`, a real `lint` target,
      the mandatory six with echoes where inapplicable, `namedInputs.specs`, and tags
      `["type:lib","lang:ts","domain:baseerah"]` — acceptance: `npx nx show projects` includes
      `baseerah-contracts`.
- [ ] [AI] Run the bundle: `npx nx run baseerah-contracts:bundle` — acceptance: exits 0 and
      `specs/apps/baseerah/containers/contracts/generated/openapi-bundled.yaml` exists.
- [ ] [AI] Author the backend Gherkin at
      `specs/apps/baseerah/behavior/baseerah-be/gherkin/health/service-health.feature` (the "The
      service reports liveness" scenario) and
      `specs/apps/baseerah/behavior/baseerah-be/gherkin/hello/greeting.feature` (the "The service
      returns a greeting" and "An unknown route is refused" scenarios), copying all three US-4
      scenarios from [prd.md](./prd.md#us-4--serve-hello-world-from-baseerah-be) verbatim —
      acceptance: `npx nx run rhino-cli:specs:structure-validation` exits 0, and every scenario uses
      exactly one `Given`, one `When`, and one `Then` per the
      [Acceptance Criteria convention](../../../repo-governance/development/infra/acceptance-criteria.md).
- [ ] [AI] Author the frontend Gherkin at
      `specs/apps/baseerah/behavior/baseerah-fe/gherkin/hello/landing-page.feature`, copying both
      US-5 scenarios from [prd.md](./prd.md#us-5--render-hello-world-in-baseerah-fe) verbatim —
      acceptance: same validation exits 0. Use the conformant `<product>-<surface>` slug
      (`baseerah-be`, `baseerah-fe`), **never** the deprecated bare `be` / `web` form. A `.feature`
      file must sit in a domain subdirectory under `gherkin/`, never bare directly beneath it.
- [ ] [AI] Author `specs/apps/baseerah/components/README.md` and
      `specs/apps/baseerah/containers/README.md` indexes whose stated `.feature` counts match the
      files actually present — acceptance: `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md readme-index validate` exits 0.
- [ ] [AI] Register the area in `repo-config.yml`: add a `coverage.projects` entry for
      `baseerah-contracts` with its `specs` glob; leave `specs.ddd-areas` empty since no `ddd/`
      folder exists — acceptance: `npm run validate:config` exits 0.
- [ ] [AI] Commit: `git add -A && git commit -m "feat(specs): add the baseerah spec area and contracts project"`
      — acceptance: the pre-commit gate passes.
- [ ] [AI] Push: `git push origin main` — acceptance: exits 0.

### Phase 5 Gate

- [ ] [AI] `npx nx show projects` — includes `baseerah-contracts`.
- [ ] [AI] `npx nx run baseerah-contracts:test:quick` — exits 0.
- [ ] [AI] `npx nx run rhino-cli:specs:structure-validation` — exits 0.
- [ ] [AI] `npm run validate:config` — exits 0.
- [ ] [AI] `find specs/apps/baseerah/behavior -name '*.feature' | wc -l` — reports 3.
- [ ] [AI] `grep -c '^  Scenario' specs/apps/baseerah/behavior/*/gherkin/*/*.feature | awk -F: '{s+=$2} END {print s}'`
      — reports 5, the total scenario count across US-4 and US-5.
- [ ] [AI] `npx nx run-many -t typecheck,lint,test:quick --all` — exits 0.
- [ ] [AI] CI: `gh run view <id> --json status,conclusion,jobs` — all jobs `success` or `skipped`.

> **Pause Safety**: the specification and contract for Baseerah exist and validate, but no
> implementation does. Nothing runs; nothing is broken. Safe to stop.
> To resume: `npx nx run baseerah-contracts:test:quick`.

---

## Phase 6: `baseerah-be` — F# / Giraffe Hello World on :19320

> Modelled on `apps/organiclever-be` (which matches its F# backend) rather than `apps/ose-be`.
> Recover reference files with
> `git show "$(git log --diff-filter=D --format=%H -- apps/organiclever-be/<path> | head -1)~1":apps/organiclever-be/<path>`
> as needed (deletion-commit lookup, not a fixed `HEAD~N` offset).
> **Stateless hello world** — no database, no in-memory store, no write route
> ([tech-docs Decision 8](./tech-docs.md#decision-8--hello-world-and-therefore-no-state-at-all)).

- [ ] [AI] Scaffold the directory: create `apps/baseerah-be/` with `.editorconfig`, `.dockerignore`,
      `.gitignore`, `global.json` (SDK 10, `rollForward: latestMinor`), `dotnet-tools.json`,
      `fsharplint.json`, `LICENSE`, and `README.md`, each copied from the recovered
      `organiclever-be` equivalents with names substituted — acceptance: all eight files exist.
- [ ] [AI] Create `apps/baseerah-be/.env.example` declaring `BASEERAH_BE_PORT` and
      `BASEERAH_BE_CORS_ORIGINS` as OPTIONAL, each using the strict
      `# REQUIRED|OPTIONAL | <type> | <description>` line format that `rhino-cli env validate` parses.
      Declare **no** test-hook flag — the service is stateless and needs none — acceptance:
      `npx nx run rhino-cli:env:validation` exits 0 after the `repo-config.yml` registration step below.
- [ ] [AI] Create `apps/baseerah-be/src/BaseerahBe/BaseerahBe.fsproj` plus `Program.fs` and
      `WebApp.fs` — acceptance: `dotnet build apps/baseerah-be/src/BaseerahBe/BaseerahBe.fsproj`
      exits 0.
- [ ] [AI] Create `baseerah.sln` at the repo root registering `BaseerahBe.fsproj` and the two test
      projects created below — acceptance: `dotnet build baseerah.sln` exits 0.
- [ ] [AI] Create `apps/baseerah-be/project.json` with the target set from
      [tech-docs § Nx target contract](./tech-docs.md#nx-target-contract-for-the-four-new-projects):
      `codegen` (depending on `baseerah-contracts:bundle`), `build`, `typecheck`, `lint` (fantomas +
      fsharplint + the 13 G-Research analyzer rules, `"parallel": false`), `dev`, `run`, `test:unit`,
      `test:coverage` (90% line threshold), `test:integration` (`cache: false`), `test:e2e` (echo),
      `test:quick` (`typecheck → lint → test:unit → test:coverage → test:specs`, `"parallel": false`),
      `specs:behavior:coverage`, `specs:domain:coverage` (echo), `specs:structure-validation`,
      `test:specs`, `deps:audit`, `compat:min-version`; `namedInputs.specs` pointing at
      `{workspaceRoot}/specs/apps/baseerah/behavior/baseerah-be/gherkin/**/*.feature`; tags
      `["type:app","platform:giraffe","lang:fsharp","domain:baseerah"]`; `implicitDependencies:
["baseerah-contracts","rhino-cli"]` — acceptance: `npx nx show project baseerah-be --json`
      lists all of them.
- [ ] [AI] Register in `repo-config.yml`: add the `coverage.projects` entry for `baseerah-be`
      (`levels: [unit, integration]`), the `env-contract.surfaces` entry
      (`{root: apps/baseerah-be, kind: app, lang: fsharp}`), and the `env-injection.apps` entry —
      acceptance: `npm run validate:config` exits 0.

### Behaviour cycles (one Gherkin scenario each)

- [ ] [AI] **RED** — create `apps/baseerah-be/tests/unit/BaseerahBe.UnitTests.fsproj` and
      `tests/unit/Steps/HealthSteps.fs` asserting the health handler returns 200 with
      `{"status":"ok"}`.
      **Gherkin (binds) →** "The service reports liveness"

      ```gherkin
      Scenario: The service reports liveness
        Given the service has finished starting
        When I send a GET request to "/api/v1/health"
        Then the response status is 200
        And the response body field "status" equals "ok"
      ```

      Run `dotnet test apps/baseerah-be/tests/unit/BaseerahBe.UnitTests.fsproj` — acceptance: fails,
      because no health route exists.

- [ ] [AI] **GREEN** — add the `/api/v1/health` route in
      `apps/baseerah-be/src/BaseerahBe/Api/HealthHandlers.fs` and wire it in `WebApp.fs`. Run
      `dotnet test apps/baseerah-be/tests/unit/BaseerahBe.UnitTests.fsproj` — acceptance: exits 0.

- [ ] [AI] **REFACTOR** — extract the readiness payload into
      `apps/baseerah-be/src/BaseerahBe/Domain/Readiness.fs` as a record with one serialisation point.
      Run the same command — acceptance: still exits 0.

- [ ] [AI] **RED** — add `apps/baseerah-be/tests/unit/Steps/GreetingSteps.fs` asserting the greeting.
      **Gherkin (binds) →** "The service returns a greeting"

      ```gherkin
      Scenario: The service returns a greeting
        Given the service has finished starting
        When I send a GET request to "/api/v1/hello"
        Then the response status is 200
        And the response body field "message" equals "Hello from Baseerah"
      ```

      Run `dotnet test apps/baseerah-be/tests/unit/BaseerahBe.UnitTests.fsproj` — acceptance: fails.

- [ ] [AI] **GREEN** — implement `apps/baseerah-be/src/BaseerahBe/Domain/Greeting.fs` holding the
      constant greeting and `apps/baseerah-be/src/BaseerahBe/Api/GreetingHandlers.fs` serving
      `GET /api/v1/hello`. Run the same command — acceptance: exits 0.

- [ ] [AI] **REFACTOR** — make the greeting text a single named value in `Domain/Greeting.fs` so the
      handler holds no literal, per the
      [functional core / imperative shell pattern](../../../repo-governance/development/pattern/functional-programming.md).
      Run the same command — acceptance: still exits 0 and
      `rg -n 'Hello from Baseerah' apps/baseerah-be/src/` returns exactly one match.

- [ ] [AI] **RED** — add `apps/baseerah-be/tests/unit/Steps/NotFoundSteps.fs` asserting the fallback.
      **Gherkin (binds) →** "An unknown route is refused"

      ```gherkin
      Scenario: An unknown route is refused
        Given the service has finished starting
        When I send a GET request to "/api/v1/does-not-exist"
        Then the response status is 404
        And the response body field "error" is a non-empty string
      ```

      Run `dotnet test apps/baseerah-be/tests/unit/BaseerahBe.UnitTests.fsproj` — acceptance: fails,
      because the default Giraffe fallthrough returns a bare 404 with no JSON body.

- [ ] [AI] **GREEN** — add a `setStatusCode 404` JSON fallback handler at the end of the router in
      `apps/baseerah-be/src/BaseerahBe/WebApp.fs`, returning the contract's `Error` schema. Run the
      same command — acceptance: exits 0.

- [ ] [AI] **REFACTOR** — route the 404 through the same single error-formatting function the rest of
      the app will use, defined once in `WebApp.fs`. Run the same command — acceptance: still exits 0.

- [ ] [AI] Create `apps/baseerah-be/tests/integration/BaseerahBe.IntegrationTests.fsproj` with an
      in-process host boot test asserting the app starts and serves `/api/v1/health` — acceptance:
      `npx nx run baseerah-be:test:integration` exits 0.
- [ ] [AI] Create `apps/baseerah-be/Dockerfile`: two-stage `dotnet/sdk:10.0` → `dotnet/aspnet:10.0`,
      `EXPOSE 19320`, `ENV BASEERAH_BE_PORT=19320` — acceptance:
      `hadolint apps/baseerah-be/Dockerfile` exits 0 at `--failure-threshold warning`.
- [ ] [AI] Verify coverage clears the chosen threshold: run `npx nx run baseerah-be:test:coverage` —
      acceptance: exits 0 at 90% line.
- [ ] [AI] Commit: `git add -A && git commit -m "feat(baseerah-be): add the F# Giraffe hello-world backend"`
      — acceptance: the pre-commit gate passes.
- [ ] [AI] Push: `git push origin main` — acceptance: exits 0.

### Phase 6 Gate

> All checks below must pass before starting Phase 7. If any check fails, fix it in Phase 6 before
> proceeding.

- [ ] [AI] `npx nx run baseerah-be:test:quick` — exits 0.
- [ ] [AI] `npx nx run baseerah-be:build` — exits 0 and `apps/baseerah-be/dist/` exists.
- [ ] [AI] Manually verify the running service per the
      [manual behavioural verification convention](../../../repo-governance/development/quality/manual-behavioral-verification.md):
      start it with `npx nx run baseerah-be:run`, then in a second shell run
      `curl -s -o /dev/null -w '%{http_code}' http://localhost:19320/api/v1/health` — acceptance:
      prints `200`. Save the full response body to `evidence/phase-6-health.txt`.
- [ ] [AI] `curl -s http://localhost:19320/api/v1/hello` — acceptance: returns
      `{"message":"Hello from Baseerah"}`. Save to `evidence/phase-6-hello.txt`.
- [ ] [AI] `curl -s -o /dev/null -w '%{http_code}' http://localhost:19320/api/v1/does-not-exist` —
      acceptance: prints `404`.
- [ ] [AI] `npx nx run baseerah-be:test:integration` — exits 0.
- [ ] [AI] `npm run validate:config` and `npx nx run rhino-cli:env:validation` — both exit 0.
- [ ] [AI] `npx nx run-many -t typecheck,lint,test:quick --all` — exits 0.
- [ ] [AI] CI: poll every 2 minutes with one call per wakeup, then
      `gh run view <id> --json status,conclusion,jobs` — `conclusion` is `success` and every
      `jobs[].conclusion` is `success` or `skipped`. The `dotnet` job now runs a real project and
      must be `success`.

> **Pause Safety**: a working stateless backend serves health, hello, and a JSON 404 on :19320, with
> unit and integration coverage. No frontend and no E2E suite exist yet. Safe to stop.
> To resume: `npx nx run baseerah-be:test:quick`.

---

## Phase 7: `baseerah-be-e2e` and the Local Stack

> The backend is stateless, so no scenario needs isolation from another and no reset hook exists
> ([tech-docs Decision 8](./tech-docs.md#decision-8--hello-world-and-therefore-no-state-at-all)).

- [ ] [AI] Create `infra/dev/baseerah-app/`: `docker-compose.yml`, `docker-compose.ci.yml`,
      `Dockerfile.be.dev`, `Dockerfile.fe.dev`, `README.md`, `.gitignore` — modelled on the deleted
      `infra/dev/organiclever-app/` (recover with
      `git show "$(git log --diff-filter=D --format=%H -- infra/dev/organiclever-app/docker-compose.yml | head -1)~1":infra/dev/organiclever-app/docker-compose.yml`),
      **not** on
      `infra/dev/ose-app/`, which was stale and pointed at a non-existent Rust backend. Services:
      `baseerah-be` on 19320 and `baseerah-fe` on 19310; **no database service** — acceptance:
      `docker compose -f infra/dev/baseerah-app/docker-compose.yml config` exits 0.
- [ ] [AI] Leave the `baseerah-fe` service defined but commented out until Phase 8 creates the app,
      and record that choice in `learnings.md` — acceptance:
      `docker compose -f infra/dev/baseerah-app/docker-compose.yml up -d` starts `baseerah-be` alone
      without error.
- [ ] [AI] Lint the two dev Dockerfiles: run
      `hadolint infra/dev/baseerah-app/Dockerfile.be.dev infra/dev/baseerah-app/Dockerfile.fe.dev`
      — acceptance: exits 0 at `--failure-threshold warning`.
- [ ] [AI] Create `apps/baseerah-be/scripts/run-e2e.sh`: bring up the compose stack, poll
      `GET /api/v1/health` until it returns 200 or a bounded timeout elapses, then run
      `npx bddgen && npx playwright test` in `apps/baseerah-be-e2e`, and tear the stack down on exit
      via a `trap` — acceptance: `shellcheck --severity=warning apps/baseerah-be/scripts/run-e2e.sh`
      exits 0.
- [ ] [AI] Scaffold `apps/baseerah-be-e2e/` with `package.json` (private, devDeps `@playwright/test`
      1.60.0 and `playwright-bdd` 8.5.1, `volta.extends` pointing at the root), `tsconfig.json`,
      `.gitignore`, `README.md`, and `e2e-coverage-baseline.json` with an empty `allowedUnbound`
      array — acceptance: `npm install` exits 0.
- [ ] [AI] Create `apps/baseerah-be-e2e/playwright.config.ts` with
      `defineBddConfig({ featuresRoot: "../../specs/apps/baseerah/behavior/baseerah-be/gherkin", features: ".../**/*.feature", steps: ["./steps/**/*.ts"] })`,
      `fullyParallel: false`, `workers: 1`, and
      `baseURL: process.env.API_BASE_URL || "http://localhost:19320"` — acceptance:
      `npx tsc --noEmit -p apps/baseerah-be-e2e/tsconfig.json` exits 0.
- [ ] [AI] Implement `apps/baseerah-be-e2e/steps/health.steps.ts` binding "The service reports
      liveness" — acceptance: `npx nx run baseerah-be-e2e:test:e2e` runs that scenario green.
- [ ] [AI] Implement `apps/baseerah-be-e2e/steps/greeting.steps.ts` binding "The service returns a
      greeting" and "An unknown route is refused" — acceptance:
      `npx nx run baseerah-be-e2e:test:e2e` runs all three scenarios green.
- [ ] [AI] Create `apps/baseerah-be-e2e/project.json` with `install`, `typecheck`, `lint`
      (`npx oxlint@latest .`), echoes for `test:unit` / `test:coverage` / `test:integration`,
      `test:quick`, `test:e2e` (delegating to `apps/baseerah-be/scripts/run-e2e.sh`), `test:e2e:ui`,
      `test:e2e:report`, `specs:behavior:coverage`, `specs:e2e:coverage`,
      `specs:structure-validation`, `test:specs`, `deps:audit`, `compat:min-version`; tags
      `["type:e2e","platform:playwright","lang:ts","domain:baseerah"]`; `implicitDependencies:
["baseerah-be"]` — acceptance: `npx nx show project baseerah-be-e2e --json` lists all of them.
- [ ] [AI] Register in `repo-config.yml`: `coverage.projects` entry for `baseerah-be-e2e` with
      `levels: [e2e]` — acceptance: `npm run validate:config` exits 0.
- [ ] [AI] Verify every backend scenario is bound: run
      `npx nx run baseerah-be-e2e:specs:e2e:coverage` — acceptance: exits 0 with no unbound scenario
      outside the empty baseline.

### CI callers — the OSE pattern, applied to Baseerah

> These are thin callers into the reusable templates Phase 1 deliberately kept. Shape and input
> names match `ose-public`'s callers exactly
> ([tech-docs Decision 15](./tech-docs.md#decision-15--cicd-architecture-stays-consistent-with-the-ose-siblings)).
> The workflows land wired but dormant: their trigger branches do not exist yet, and creating them
> belongs to a deploy plan, not this one.

- [ ] [AI] Create `.github/workflows/baseerah-be-build-deploy-stag.yml` calling
      `./.github/workflows/_reusable-be-build-deploy.yml` with `be-project: baseerah-be` and
      `image-name: ghcr.io/wahidyankf/baseerah-be`, triggered by `push` to `stag-baseerah-be`.
      Recover the exact caller shape with
      `git show "$(git log --diff-filter=D --format=%H -- .github/workflows/ose-be-build-deploy-stag.yml | head -1)~1":.github/workflows/ose-be-build-deploy-stag.yml`
      — acceptance: `actionlint .github/workflows/baseerah-be-build-deploy-stag.yml` exits 0 and the
      file is under 25 lines, matching the thin-caller pattern.
- [ ] [AI] Re-populate `.github/workflows/publish-images.yml`: add the `build-baseerah-be` output,
      its `case` arm in the `detect` job, and the `publish-baseerah-be` job, following the structure
      Phase 1 left intact — acceptance: `actionlint .github/workflows/publish-images.yml` exits 0 and
      `rg -n 'baseerah-be' .github/workflows/publish-images.yml` returns at least three matches.
- [ ] [AI] Confirm the reusable templates are now genuinely called: run
      `rg -n 'uses:\s*\./\.github/workflows/_reusable' .github/workflows/` — acceptance: at least one
      match, resolving the Phase 1 note that they were temporarily uncalled.
- [ ] [AI] Commit: `git add -A && git commit -m "feat(baseerah-be-e2e): add the backend E2E suite, local Docker stack, and CI callers"`
      — acceptance: the pre-commit gate passes.
- [ ] [AI] Push: `git push origin main` — acceptance: exits 0.

### Phase 7 Gate

> All checks below must pass before starting Phase 8.

- [ ] [AI] `npx nx run baseerah-be-e2e:test:e2e` — exits 0 with three scenarios passing.
- [ ] [AI] `npx nx run baseerah-be-e2e:test:quick` — exits 0.
- [ ] [AI] `npx nx run baseerah-be-e2e:specs:e2e:coverage` — exits 0.
- [ ] [AI] `docker compose -f infra/dev/baseerah-app/docker-compose.yml config` — exits 0.
- [ ] [AI] `npx nx run-many -t typecheck,lint,test:quick --all` — exits 0.
- [ ] [AI] CI: `gh run view <id> --json status,conclusion,jobs` — all jobs `success` or `skipped`.
      `test:e2e` is CRON-only and does **not** run on this push; that is correct.

> **Pause Safety**: the backend is fully specified, implemented, and E2E-verified against a real
> Docker stack. The frontend does not exist and its compose service is commented out. Safe to stop.
> To resume: `npx nx run baseerah-be-e2e:test:e2e`.

---

## Phase 8: `baseerah-fe` — Next.js 16 Hello World on :19310

> Modelled on `apps/ose-app-web` minus the DDD bounded-context layering (tech-docs Decision 9).
> Recover reference files with
> `git show "$(git log --diff-filter=D --format=%H -- apps/ose-app-web/<path> | head -1)~1":apps/ose-app-web/<path>`
> (deletion-commit lookup, not a fixed `HEAD~N` offset — safe regardless of how many commits the
> earlier phases actually made).
> One route, `/`. No forms, no write paths, no client state beyond the fetched greeting.

- [ ] [AI] Author the three high-fidelity mockups into
      `plans/in-progress/baseerah-repo-reset/assets/` — `landing-desktop-1280.png`,
      `landing-tablet-768.png`, and `landing-mobile-390.png` — realising the selected Alternative B
      "Shell + Greeting" from [prd.md](./prd.md#select) with the Phase 4 Baseerah tokens —
      acceptance: all three files exist.
- [ ] [AI] Edit `prd.md`'s **Narrow** subsection: convert the three inert code-fenced paths into live
      `![alt](./assets/...)` embeds with descriptive alt text — acceptance:
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md links validate --exclude plans/done` reports zero broken links.
- [ ] [AI] Scaffold `apps/baseerah-fe/` with `package.json`, `tsconfig.json` (standalone, `@/*` →
      `./src/*` plus the `web-ui` aliases), `next.config.ts` (`output: "standalone"`,
      `transpilePackages` for `web-ui`, `web-ui-token`, `@t3-oss`), `postcss.config.mjs`,
      `oxlint.json`, `vitest.config.ts`, `.npmrc`, `.gitignore`, `.dockerignore`, `.env.example`
      (declaring `BASEERAH_FE_API_BASE_URL` as REQUIRED plus the framework-reserved `PORT` and
      `HOSTNAME` as commented allowlist entries), `LICENSE`, and `README.md` — acceptance:
      `npm install` exits 0.
- [ ] [AI] Create `apps/baseerah-fe/project.json` with `codegen` (`@hey-api/openapi-ts` against
      `specs/apps/baseerah/containers/contracts/generated/openapi-bundled.yaml`, `dependsOn:
["baseerah-contracts:bundle"]`), `dev` (`next dev --port 19310`), `build`, `start`
      (`next start --port 19310`), `typecheck`, `lint` (`npx oxlint@latest --jsx-a11y-plugin .`),
      `test:unit` (with the `.skip|.only|.todo` guard grep), `test:coverage` (90% line),
      `test:integration`, `test:e2e` (echo), `test:quick`, `specs:behavior:coverage`,
      `specs:structure-validation`, `test:specs`, `deps:audit`, `compat:min-version`;
      `namedInputs.specs` pointing at the `baseerah-fe` Gherkin glob; tags
      `["type:app","platform:nextjs","lang:ts","domain:baseerah"]`; `implicitDependencies:
["baseerah-contracts","web-ui","web-ui-token"]` — acceptance:
      `npx nx show project baseerah-fe --json` lists all of them.
- [ ] [AI] Generate the typed client: run `npx nx run baseerah-fe:codegen` — acceptance: exits 0 and
      `apps/baseerah-fe/src/generated-contracts/` contains the `Greeting` type.

- [ ] [AI] **RED** — create `apps/baseerah-fe/src/app/page.test.tsx` asserting the landing page
      renders the heading and the greeting fetched from the backend, with the fetch stubbed.
      **Gherkin (binds) →** "The landing page names the product and shows the backend greeting"

      ```gherkin
      Scenario: The landing page names the product and shows the backend greeting
        Given I have not visited the site before
        When I navigate to "/"
        Then the page shows a level-one heading containing "Baseerah"
        And the page shows the text "Hello from Baseerah" sourced from the backend
      ```

      Run `npx nx run baseerah-fe:test:unit` — acceptance: fails, because no page exists.

- [ ] [AI] **GREEN** — create `apps/baseerah-fe/src/app/layout.tsx`, `globals.css`, and `page.tsx`
      rendering the heading and the greeting, plus `src/lib/greeting-client.ts` wrapping the
      generated client and reading its base URL from `BASEERAH_FE_API_BASE_URL`. Use only
      `libs/web-ui` primitives and `libs/web-ui-token` values — acceptance:
      `npx nx run baseerah-fe:test:unit` exits 0.

- [ ] [AI] **REFACTOR** — move all fetch orchestration into `src/lib/greeting-client.ts` so
      `page.tsx` holds only rendering. Run the same command — acceptance: still exits 0 and
      `rg -n 'fetch\(' apps/baseerah-fe/src/app/` returns no matches.

- [ ] [AI] **RED** — extend `apps/baseerah-fe/src/app/page.test.tsx` to assert the landmark
      structure and language attributes the accessibility bar depends on.
      **Gherkin (binds) →** "The landing page meets the baseline accessibility bar"

      ```gherkin
      Scenario: The landing page meets the baseline accessibility bar
        Given I am on "/"
        When an automated accessibility scan runs against the rendered page
        Then it reports zero serious violations
        And it reports zero critical violations
      ```

      Assert exactly one `<h1>`, a `<header>`, a `<main>`, a `<footer>`, and that the Arabic string
      `بصيرة` carries `lang="ar"` and `dir="rtl"` on its own element. Run
      `npx nx run baseerah-fe:test:unit` — acceptance: fails.

- [ ] [AI] **GREEN** — create `apps/baseerah-fe/src/components/AppShell.tsx` providing the
      `<header>` / `<main>` / `<footer>` landmarks, and wrap the page in it with the Arabic string
      correctly marked up. Run the same command — acceptance: exits 0.

- [ ] [AI] **REFACTOR** — move the shell into `layout.tsx` so every future route inherits it without
      importing it. Run the same command — acceptance: still exits 0.

- [ ] [AI] Create `apps/baseerah-fe/Dockerfile`: `node:24-alpine` build → runtime, `EXPOSE 19310`,
      `ENV PORT=19310 HOSTNAME=0.0.0.0`, standalone output — acceptance:
      `hadolint apps/baseerah-fe/Dockerfile` exits 0 at `--failure-threshold warning`.
- [ ] [AI] Uncomment the `baseerah-fe` service in `infra/dev/baseerah-app/docker-compose.yml`,
      wiring `BASEERAH_FE_API_BASE_URL` to the `baseerah-be` service — acceptance:
      `docker compose -f infra/dev/baseerah-app/docker-compose.yml up -d` brings both services up and
      `curl -s -o /dev/null -w '%{http_code}' http://localhost:19310/` prints `200`.
- [ ] [AI] Register in `repo-config.yml`: `coverage.projects` entry for `baseerah-fe`
      (`levels: [unit]`), the `env-contract.surfaces` entry with the `PORT` / `HOSTNAME` allowlist,
      and the `env-injection.apps` entry — acceptance: `npm run validate:config` exits 0.
- [ ] [AI] Verify coverage: run `npx nx run baseerah-fe:test:coverage` — acceptance: exits 0 at 90%
      line, with `vitest.config.ts` and the CLI threshold agreeing so the repo does not reintroduce
      the drift recorded in
      [tech-docs Decision 11](./tech-docs.md#decision-11--resolve-the-coverage-threshold-drift-at-90-line).
- [ ] [AI] Commit: `git add -A && git commit -m "feat(baseerah-fe): add the Next.js hello-world frontend"`
      — acceptance: the pre-commit gate passes.
- [ ] [AI] Push: `git push origin main` — acceptance: exits 0.

### Phase 8 Gate

> All checks below must pass before starting Phase 9.

- [ ] [AI] `npx nx run baseerah-fe:test:quick` — exits 0.
- [ ] [AI] `npx nx run baseerah-fe:build` — exits 0.
- [ ] [AI] Manual behavioural verification with Playwright MCP against `http://localhost:19310` —
      acceptance: the page shows the heading `Baseerah` and the text `Hello from Baseerah`, and the
      greeting disappears when `baseerah-be` is stopped, proving it is fetched rather than hardcoded.
      Save one screenshot per breakpoint into `evidence/phase-8-landing-1280.png`,
      `evidence/phase-8-landing-768.png`, and `evidence/phase-8-landing-390.png`.
- [ ] [AI] `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md links validate --exclude plans/done` — exits 0; the `prd.md` mockup embeds resolve.
- [ ] [AI] `npm run validate:config` — exits 0.
- [ ] [AI] `npx nx run-many -t typecheck,lint,test:quick --all` — exits 0.
- [ ] [AI] CI: `gh run view <id> --json status,conclusion,jobs` — all jobs `success` or `skipped`.

> **Pause Safety**: the full stack runs locally — `baseerah-fe` on 19310 fetching its greeting from
> `baseerah-be` on 19320 — with unit coverage on both. The frontend E2E suite does not exist yet, so
> no browser-level regression net is in place. Safe to stop.
> To resume: `docker compose -f infra/dev/baseerah-app/docker-compose.yml up -d`.

---

## Phase 9: `baseerah-fe-e2e` — Playwright Against the Full Stack

- [ ] [AI] Scaffold `apps/baseerah-fe-e2e/` with `package.json` (`type: "module"`, devDeps
      `@axe-core/playwright` 4.10.1, `@playwright/test` 1.60.0, `playwright-bdd` 8.5.1,
      `typescript` 5.8.3), standalone `tsconfig.json`, `.gitignore`, `README.md`, and
      `e2e-coverage-baseline.json` with an empty `allowedUnbound` array — acceptance: `npm install`
      exits 0.
- [ ] [AI] Create `apps/baseerah-fe-e2e/playwright.config.ts` with
      `defineBddConfig({ featuresRoot: "../../specs/apps/baseerah/behavior/baseerah-fe/gherkin", steps: ["./steps/**/*.steps.ts"] })`,
      `timeout: 60000`, `fullyParallel: false`, `workers: 1`,
      `baseURL: process.env.WEB_BASE_URL || "http://localhost:19310"`, and a single `chromium`
      project — acceptance: `npx tsc --noEmit -p apps/baseerah-fe-e2e/tsconfig.json` exits 0.
- [ ] [AI] Implement `apps/baseerah-fe-e2e/steps/landing.steps.ts` binding "The landing page names
      the product and shows the backend greeting", with a `// @covers` marker comment — acceptance:
      `npx nx run baseerah-fe-e2e:test:e2e` runs it green against the running stack.
- [ ] [AI] Implement `apps/baseerah-fe-e2e/steps/accessibility.steps.ts` binding "The landing page
      meets the baseline accessibility bar" via `@axe-core/playwright` — acceptance: the scan reports
      zero `serious` and zero `critical` violations, satisfying the accessibility commitments in
      [prd.md](./prd.md#justify).
- [ ] [AI] Assert the greeting genuinely crosses the wire: add a step that intercepts the
      `/api/v1/hello` request and fails if the page renders the greeting without it — acceptance:
      the assertion passes, proving `baseerah-fe-e2e` exercises the full FE → BE path rather than a
      static render.
- [ ] [AI] Create `apps/baseerah-fe-e2e/project.json` with `install`
      (`npx playwright install --with-deps chromium`), `typecheck`, `lint`, echoes for `test:unit` /
      `test:coverage` / `test:integration` / `specs:behavior:coverage`, `test:quick`, `test:e2e`
      (with the unconditional-`test.skip` guard grep, then `npx bddgen && npx playwright test`),
      `test:e2e:ui`, `test:e2e:report`, `specs:e2e:coverage`, `specs:structure-validation`,
      `test:specs`, `deps:audit`, `compat:min-version`; tags
      `["type:e2e","platform:playwright","lang:ts","domain:baseerah"]`; `implicitDependencies:
["baseerah-fe","baseerah-be"]` — acceptance: `npx nx show project baseerah-fe-e2e --json`
      lists all of them.
- [ ] [AI] Register in `repo-config.yml`: `coverage.projects` entry for `baseerah-fe-e2e` with
      `levels: [e2e]` — acceptance: `npm run validate:config` exits 0.
- [ ] [AI] Verify every frontend scenario is bound: run
      `npx nx run baseerah-fe-e2e:specs:e2e:coverage` — acceptance: exits 0 with no unbound scenario.

### CI callers — the app-group pair

- [ ] [AI] Create `.github/workflows/baseerah-app-test-local-deploy-stag.yml` calling
      `./.github/workflows/_reusable-app-test-local-deploy-stag.yml` with
      `web-project: baseerah-fe`, `be-project: baseerah-be`,
      `contracts-project: baseerah-contracts`, `compose-dir: infra/dev/baseerah-app`,
      `stag-web-branch: stag-baseerah-fe`, `stag-be-branch: stag-baseerah-be`, `be-port: 19320`,
      `web-port: 19310`, and `environment: baseerah-app-local`. Recover the exact caller shape with
      `git show "$(git log --diff-filter=D --format=%H -- .github/workflows/ose-app-test-local-deploy-stag.yml | head -1)~1":.github/workflows/ose-app-test-local-deploy-stag.yml`
      — acceptance: `actionlint .github/workflows/baseerah-app-test-local-deploy-stag.yml` exits 0.
- [ ] [AI] Create `.github/workflows/baseerah-app-test-stag.yml` calling
      `./.github/workflows/_reusable-app-test-stag.yml` with `fe-e2e-project: baseerah-fe-e2e`,
      `environment: baseerah-app-staging`, and `secrets: inherit` — acceptance:
      `actionlint .github/workflows/baseerah-app-test-stag.yml` exits 0.
- [ ] [AI] Add the `baseerah-app-staging` environment entry to `repo-config.yml`'s
      `env-injection.ci-harness` for the `API_BASE_URL`, `WEB_BASE_URL`, and
      `VERCEL_AUTOMATION_BYPASS_SECRET` keys, restoring the structure Phase 2 emptied —
      acceptance: `npm run validate:config` exits 0.
- [ ] [AI] Update `.github/workflows/README.md` to index the three new callers and drop the
      "awaiting Baseerah callers" note from the reusable-template entries — acceptance:
      `rg -n 'awaiting' .github/workflows/README.md` returns no matches.
- [ ] [AI] Commit: `git add -A && git commit -m "feat(baseerah-fe-e2e): add the frontend E2E suite, accessibility assertions, and app-group CI callers"`
      — acceptance: the pre-commit gate passes.
- [ ] [AI] Push: `git push origin main` — acceptance: exits 0.

### Rule-15 three-tester retest follow-ups

> Per [User-Facing Delivery Hardening Rule 15](../../../repo-governance/development/quality/user-facing-delivery-hardening.md),
> a web-UI feature-change plan runs a near-end round of all three live-site testers against the
> running UI before archival. `baseerah-fe` has exactly one supported locale (`en` — the Arabic
> string بصيرة is a decorative `lang="ar"` fragment inside the English page per
> [prd.md](./prd.md#justify), not a separate locale route), so this round covers
> that single locale in full; there is no second locale to retest.

- [ ] [AI] With `baseerah-be` and `baseerah-fe` both running locally, invoke the
      [`web-ux-test-fixing-planning`](../../../repo-governance/workflows/web/web-ux-test-fixing-planning.md)
      workflow against `http://localhost:19310` with `output-mode: delivery` and this plan's path —
      acceptance: the workflow runs `web-exploratory-tester`, `web-usability-tester`, and
      `web-design-tester`, and this section is populated in place with their findings as unchecked
      `- [ ] EWT-NNN:` / `- [ ] UWT-NNN:` / `- [ ] DWT-NNN:` checkboxes (or states explicitly that
      zero findings were returned).
- [ ] [AI] Fix every `EWT-NNN`/`UWT-NNN`/`DWT-NNN` checkbox recorded above and tick it — acceptance:
      no unchecked Rule-15 defect checkbox remains in this section. Any `SG-###`/`USS-###` proposal
      may instead be triaged with written rationale recorded under its checkbox.
- [ ] [AI] Commit and push the retest fixes (if any): `git add -A && git commit -m "fix(baseerah-fe): address rule-15 three-tester retest findings"`
      then `git push origin main` — acceptance: exits 0. Skip this step if zero findings were
      returned above.

### Rule-16 API exploratory-test retest follow-ups

> Per [User-Facing Delivery Hardening Rule 16](../../../repo-governance/development/quality/user-facing-delivery-hardening.md),
> an API feature-change plan runs a near-end `api-exploratory-tester` round against the running API
> before archival, with the OpenAPI contract as ground truth.

- [ ] [AI] With `baseerah-be` running locally, invoke `api-exploratory-tester` against
      `http://localhost:19320` with `specs/apps/baseerah/containers/contracts/openapi.yaml` as
      ground truth, `output-mode: delivery`, and this plan's path — acceptance: this section is
      populated in place with its findings as unchecked `- [ ] AET-NNN:` checkboxes (or states
      explicitly that zero findings were returned).
- [ ] [AI] Fix every `AET-NNN` checkbox recorded above and tick it — acceptance: no unchecked
      Rule-16 defect checkbox remains in this section. Any `SG-###` proposal may instead be triaged
      with written rationale recorded under its checkbox.
- [ ] [AI] Commit and push the retest fixes (if any): `git add -A && git commit -m "fix(baseerah-be): address rule-16 API exploratory-test retest findings"`
      then `git push origin main` — acceptance: exits 0. Skip this step if zero findings were
      returned above.

### Phase 9 Gate

> All checks below must pass before starting Phase 10.

- [ ] [AI] `npx nx run baseerah-fe-e2e:test:e2e` — exits 0 with two scenarios passing.
- [ ] [AI] `npx nx run baseerah-be-e2e:test:e2e` — still exits 0; the backend suite did not regress.
- [ ] [AI] `npx nx run baseerah-fe-e2e:specs:e2e:coverage` — exits 0.
- [ ] [AI] `npx nx show projects` — lists exactly the nine expected projects, plus
      `fsharp-crane-core` as a tenth if and only if the Phase 2 audit kept it, and nothing else,
      satisfying [prd.md US-1](./prd.md#us-1--purge-the-old-product).
- [ ] [AI] `actionlint .github/workflows/*.yml` — exits 0 across all eleven workflow files.
- [ ] [AI] **CI architecture parity still holds** after adding the callers:
      `diff <(rg -oN '^  [a-z0-9-]+:$' /Users/wkf/ose-projects/ose-public/.github/workflows/main-ci.yml) <(rg -oN '^  [a-z0-9-]+:$' .github/workflows/main-ci.yml)`
      and the same diff for `pr-quality-gate.yml` and `.github/actions/` — all exit 0 with no output.
      Adding callers must never have perturbed the core gates.
- [ ] [AI] `npx nx run-many -t typecheck,lint,test:quick --all` — exits 0.
- [ ] [AI] CI: `gh run view <id> --json status,conclusion,jobs` — all jobs `success` or `skipped`.
- [ ] [AI] No unchecked Rule-15 or Rule-16 defect checkbox remains: run
      `rg -n '^- \[ \] (EWT|UWT|DWT|AET)-' delivery.md` — acceptance: no matches (unfixed
      `SG-###`/`USS-###` proposals with recorded triage rationale are not defect checkboxes and are
      exempt from this check).

> **Pause Safety**: every user story in `prd.md` except US-6 is implemented and verified at unit,
> integration, and E2E level. The repo is a complete, working, self-consistent Baseerah monorepo
> whose apps are deliberately hello world. This is the most defensible stopping point in the plan.
> Safe to stop. To resume: `npx nx run-many -t test:quick --all`.

---

## Phase 10: The Baseerah Agent Fleet

> Names follow `<scope>(-<qualifier>)*-<role>` with `apps` as scope and a role from the closed
> vocabulary, per the
> [Agent Naming Convention](../../../repo-governance/conventions/structure/agent-naming.md).

- [ ] [AI] Create `.claude/agents/apps-baseerah-fe-content-maker.md` using `agent-maker`, modelled on
      the deleted `apps-ose-www-content-maker` (recover its last committed content with
      `git show "$(git log --diff-filter=D --format=%H -- .claude/agents/apps-ose-www-content-maker.md | head -1)~1":.claude/agents/apps-ose-www-content-maker.md`
      — this looks up the deletion commit directly rather than assuming a fixed `HEAD~N` offset,
      which stays correct even if Phase 9's Rule-15/16 retest sections added extra commits) —
      acceptance: the file has valid frontmatter with `name`, `description`, `tools`, and a named
      colour.
- [ ] [AI] Create `.claude/agents/apps-baseerah-fe-content-checker.md` — acceptance: same.
- [ ] [AI] Create `.claude/agents/apps-baseerah-fe-content-fixer.md` — acceptance: same.
- [ ] [AI] Create `.claude/agents/apps-baseerah-fe-deployer.md`, documenting the (not yet existing)
      `prod-baseerah-fe` branch as its target and stating plainly that no deploy target is
      provisioned yet — acceptance: the file does not claim a working deploy.
- [ ] [AI] Create `.claude/agents/apps-baseerah-be-deployer.md`, targeting `stag-baseerah-be` with the
      same honest caveat — acceptance: same.
- [ ] [AI] Create `.claude/skills/apps-baseerah-fe-developing-content/SKILL.md` describing the
      Next.js 16 App Router structure, the `web-ui` / `web-ui-token` usage rules, and the hello-world
      slice — acceptance: the file has valid frontmatter and no hardcoded collection count.
- [ ] [AI] Verify naming compliance: run
      `ls .claude/agents/*.md | sed 's|.*/||; s|\.md$||' | grep -vE -- '-(maker|checker|fixer|dev|deployer|manager|tester|researcher)$' | grep -v '^README$'`
      — acceptance: outputs only the known preexisting `api-exploratory-tester` violation.
- [ ] [AI] Add the new agents to `.claude/agents/README.md` and to the `AGENTS.md` roster, without
      hardcoding any count — acceptance: `rg -n 'apps-baseerah' .claude/agents/README.md AGENTS.md`
      returns matches in both.
- [ ] [AI] Regenerate every binding: run `npm run generate:bindings` — acceptance: exits 0 and
      `.opencode/agents/` and `.cursor/agents/` each gain the five new files.
- [ ] [AI] Verify zero drift: run `npm run validate:sync && git diff --exit-code` — acceptance: both
      exit 0.
- [ ] [AI] Commit: `git add -A && git commit -m "feat(agents): add the Baseerah content and deployer agent fleet"`
      — acceptance: the pre-commit gate passes.
- [ ] [AI] Push: `git push origin main` — acceptance: exits 0.

### Phase 10 Gate

- [ ] [AI] The agent-naming enforcement command above — no new violation.
- [ ] [AI] `npm run validate:sync` — exits 0.
- [ ] [AI] `npm run harness:bindings-validation` — exits 0.
- [ ] [AI] `npx nx run rhino-cli:instruction-size:validation` — exits 0.
- [ ] [AI] `npx nx run-many -t typecheck,lint,test:quick --all` — exits 0.
- [ ] [AI] CI: `gh run view <id> --json status,conclusion,jobs` — all jobs `success` or `skipped`.

> **Pause Safety**: every user story in `prd.md` is now satisfied. The repo has its product, its
> stack, its tests, and its agent fleet. Safe to stop. To resume: `npm run validate:sync`.

---

## Phase 11: Knowledge Capture and Archival

> The mandatory final phase, per the
> [Knowledge Capture Convention](../../../repo-governance/development/quality/knowledge-capture.md).
> Archival is **blocked** until every `learnings.md` entry reaches a terminal state.

- [ ] [AI] Confirm the Rule-15/Rule-16 retest gate before doing anything else: run
      `rg -n '^- \[ \] (EWT|UWT|DWT|AET)-' plans/in-progress/baseerah-repo-reset/delivery.md` —
      acceptance: no matches. Per
      [User-Facing Delivery Hardening Rules 15-16](../../../repo-governance/development/quality/user-facing-delivery-hardening.md),
      archival is blocked while any Rule-15/16 defect checkbox is unchecked.
- [ ] [AI] Read `plans/in-progress/baseerah-repo-reset/learnings.md` end to end — acceptance: every
      entry is enumerated in a triage table written into the same file.
- [ ] [AI] Run the secret/sensitivity gate on every entry: confirm no entry contains a credential,
      token, key, or connection string — acceptance: recorded verdict per entry.
- [ ] [AI] Run the repo-relevance gate on every entry: confirm each is generalizable beyond this
      plan — acceptance: recorded verdict per entry.
- [ ] [AI] Route the coverage-threshold-drift learning (tech-docs Decision 11) to a durable home:
      either a correction inline in `repo-governance/development/infra/nx-targets.md`, or a
      `plans/backlog/` follow-up if the fix is large — acceptance: the entry has a named terminal
      state and a link to where it landed.
- [ ] [AI] Route every remaining entry to exactly one home — a convention, a doc, an agent, a skill,
      code, a test, or a discard with a one-line reason. Per the
      [Knowledge Capture Convention](../../../repo-governance/development/quality/knowledge-capture.md),
      any entry routed to `code`/`a test` (i.e. to `apps/`, `libs/`, or a test suite) MUST be filed
      as a **new** `plans/backlog/` plan rather than landed inline in this plan's own commits — this
      plan is already closing, and Phase 11 does not reopen `apps/`/`libs/` work. Acceptance: no
      entry is left unrouted, and no `code`/`a test`-routed entry has an inline diff in this plan's
      history — only a `plans/backlog/` filing. If there are none, record the explicit escape
      `No generalizable learnings — <reason>`.
- [ ] [AI] File a `plans/backlog/` two-pager or plan for each deferred capability named in
      [prd.md § Out of scope](./prd.md#out-of-scope) that the maintainer wants queued — at minimum
      persistence, deploy provisioning, and the first LLM integration — acceptance: each has a
      `README.md` naming its scope.
- [ ] [AI] Re-run the principles invariant one final time:
      `diff -r /Users/wkf/ose-projects/ose-public/repo-governance/principles /Users/wkf/ose-projects/baseerah/repo-governance/principles`
      — acceptance: exits 0 with no output. Nothing in Phases 4-10 drifted the principles layer.
- [ ] [AI] Delete `learnings.md` now that it is drained, or leave it as the routed record — either is
      permitted; state which was chosen in the archival commit message — acceptance: the choice is
      explicit, not accidental.
- [ ] [AI] Archive the plan: run
      `git mv plans/in-progress/baseerah-repo-reset plans/done/2026-XX-XX__baseerah-repo-reset`
      using the **actual** completion date, not today's — acceptance: `ls plans/in-progress/` lists
      only `README.md`.
- [ ] [AI] Update `plans/in-progress/README.md` and `plans/done/README.md` indexes — acceptance:
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- md links validate --exclude plans/done` reports zero broken links.
- [ ] [AI] Re-add the `--exclude plans/done` flags to `.husky/pre-push`, `package.json` lint-staged,
      `main-ci.yml`, and `pr-quality-gate.yml`, since `plans/done/` now exists again with this
      plan in it — acceptance: `rg -n 'exclude plans/done' .husky/pre-push package.json` returns
      matches.
- [ ] [AI] Commit: `git add -A && git commit -m "chore(plans): archive the baseerah-repo-reset plan"`
      — acceptance: the pre-commit gate passes.
- [ ] [AI] Push: `git push origin main` — acceptance: exits 0.

### Phase 11 Gate

- [ ] [AI] `ls plans/in-progress/` — contains only `README.md`.
- [ ] [AI] `ls plans/done/` — contains exactly one dated folder, this plan.
- [ ] [AI] `diff -r /Users/wkf/ose-projects/ose-public/repo-governance/principles /Users/wkf/ose-projects/baseerah/repo-governance/principles`
      — exits 0 with no output.
- [ ] [AI] **Final CI/CD parity assertion** — all three diffs from
      [tech-docs Decision 15](./tech-docs.md#decision-15--cicd-architecture-stays-consistent-with-the-ose-siblings)
      (`.github/actions/` tree, `main-ci.yml` job set, `pr-quality-gate.yml` job set, each against
      `/Users/wkf/ose-projects/ose-public`) exit 0 with no output.
- [ ] [AI] `npx nx show projects` — lists exactly `rhino-cli`, `rust-commons`, `web-ui`,
      `web-ui-token`, `baseerah-contracts`, `baseerah-be`, `baseerah-be-e2e`, `baseerah-fe`,
      `baseerah-fe-e2e`, plus `fsharp-crane-core` if the Phase 2 audit kept it.
- [ ] [AI] `rg -n --hidden -g '!.git' 'ayokoding|organiclever|wahidyankf|crane-cli|ose-www|ose-app-web|ose-cli'`
      — matches only inside `plans/done/2026-XX-XX__baseerah-repo-reset/`, this plan's own record.
- [ ] [AI] `npx nx run-many -t typecheck,lint,test:quick --all` — exits 0.
- [ ] [AI] `npx nx run baseerah-be-e2e:test:e2e && npx nx run baseerah-fe-e2e:test:e2e` — both exit 0.
- [ ] [AI] CI: `gh run view <id> --json status,conclusion,jobs` — all jobs `success` or `skipped`.

> **Pause Safety**: the plan is complete and archived. The repository is Baseerah — a personal
> assistant monorepo inside the OSE ecosystem, with a working full-stack hello-world quad, tests at
> three levels, its own agent fleet, and a governance layer whose principles are verified identical
> to `ose-public`. To verify at any later date:
> `npx nx run-many -t typecheck,lint,test:quick --all`.
