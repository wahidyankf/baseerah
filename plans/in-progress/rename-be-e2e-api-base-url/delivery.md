---
title: "Delivery — Standardize Backend E2E Base-URL Env Var to API_BASE_URL"
description: Phased, executor-tagged, TDD-shaped delivery checklist with gates
---

# Delivery Checklist — Backend E2E `API_BASE_URL` Standardization

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it. `[DEFERRED]`: documented, not executed by this plan.

## Worktree

Worktree path: `worktrees/rename-be-e2e-api-base-url/`

```bash
claude --worktree rename-be-e2e-api-base-url
```

> **ADAPTED (per user directive):** execution runs directly on `main`, no worktree — mirroring how
> `wire-vercel-www-app-cutover` was executed. The `## Worktree` section is retained to satisfy the
> plan-execution Step 0 gate; substitute "main checkout" wherever the workflow says "worktree."

---

## Commit Guidelines

- Conventional Commits: `<type>(<scope>): <description>`, imperative, no trailing period.
- The rename (configs + setter + READMEs + manifest) is one thematic commit.
- Fix ALL pre-commit/pre-push hook failures before retrying — never `--no-verify`.

---

## Phase 0: Baseline

- [ ] [AI] Run `npm install && npm run doctor -- --scope minimal` — acceptance: exits 0.
- [ ] [AI] Establish a green baseline for both in-scope suites against docker-compose (current `BASE_URL`
      wiring), via the local-stack harness or manual compose — acceptance: `ose-be-e2e` and
      `organiclever-be-e2e` pass before any edit. If the local backend stack cannot be started in this
      environment, record that explicitly and rely on the Phase 1 gate re-run instead.

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [ ] [AI] `git grep -nE '\bBASE_URL\b' -- apps/ose-be-e2e apps/organiclever-be-e2e` — acceptance: 2 hits
      (the current readers), confirming the starting state.

> **Pause Safety**: read-only baseline. Safe to stop.

---

## Phase 1: Rename `BASE_URL` → `API_BASE_URL`

> _Suggested executor: swe-e2e-dev_ for the config/workflow edits; READMEs via `readme-fixer` or direct.

### 1a — RED (prove the new name is not yet wired)

- [ ] [AI] Confirm the failing-state assertion: `git grep -nE 'process\.env\.API_BASE_URL' -- apps/ose-be-e2e apps/organiclever-be-e2e`
      returns **no hits** (the new variable is not read yet) — acceptance: zero matches (RED).

### 1b — GREEN (apply the rename in lockstep)

- [ ] [AI] Edit `apps/ose-be-e2e/playwright.config.ts`: `process.env.BASE_URL` → `process.env.API_BASE_URL`
      (keep `|| "http://localhost:8302"`) — acceptance: `git grep -n 'API_BASE_URL' apps/ose-be-e2e/playwright.config.ts`
      matches and `git grep -n '\bBASE_URL\b' apps/ose-be-e2e/playwright.config.ts` returns nothing.
- [ ] [AI] Edit `apps/organiclever-be-e2e/playwright.config.ts`: same rename (keep `|| "http://localhost:8202"`)
      — acceptance: new name present, old name absent in that file.
- [ ] [AI] Edit `.github/workflows/_reusable-app-test-local-deploy-stag.yml` "Run BE E2E tests" step:
      `env:` key `BASE_URL` → `API_BASE_URL` — acceptance: `grep -n 'API_BASE_URL' .github/workflows/_reusable-app-test-local-deploy-stag.yml`
      matches; no `BASE_URL:` key remains in that step.
- [ ] [AI] Edit `apps/ose-be-e2e/README.md` and `apps/organiclever-be-e2e/README.md`: env-var table +
      prose `BASE_URL` → `API_BASE_URL` (keep documented localhost default) — acceptance:
      `git grep -nE '\bBASE_URL\b' -- apps/ose-be-e2e/README.md apps/organiclever-be-e2e/README.md` returns nothing.
- [ ] [AI] Edit `env-injection.yaml`: add the `API_BASE_URL` `ci-harness` entry (class `var`, environments
      `[organiclever-app-staging, ose-app-staging]`, with the deferred-consumption comment) — acceptance:
      `API_BASE_URL` present in the `ci-harness` block.

### 1c — REFACTOR / verify green

- [ ] [AI] Run `actionlint .github/workflows/_reusable-app-test-local-deploy-stag.yml` — acceptance: zero errors.
- [ ] [AI] Run `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- env validate` —
      acceptance: exits 0 (env-injection drift guard passes with the new key).
- [ ] [AI] Re-run both suites against docker-compose with `API_BASE_URL` set (via the local-stack harness or
      manual compose) — acceptance: `ose-be-e2e` and `organiclever-be-e2e` pass with the new variable name.
      If the backend stack cannot run in this environment, state so and rely on CI to exercise the gate.
- [ ] [AI] Run `npx nx run rhino-cli:links:validation` and `npm run lint:md` — acceptance: both exit 0.
- [ ] [AI] Run `npx nx affected -t typecheck lint test:quick specs:coverage` — acceptance: exits 0, no
      errors reported. Fix ALL failures found, including any preexisting issues not caused by this rename.

### 1d — Commit + push

- [ ] [AI] Stage explicit paths (`apps/ose-be-e2e/playwright.config.ts apps/organiclever-be-e2e/playwright.config.ts .github/workflows/_reusable-app-test-local-deploy-stag.yml apps/ose-be-e2e/README.md apps/organiclever-be-e2e/README.md env-injection.yaml`)
      and commit `refactor(e2e): rename backend E2E base URL var BASE_URL → API_BASE_URL` — acceptance:
      `git log --oneline -1` shows the commit; `git status` clean.
- [ ] [AI] Push to `origin main` and verify CI green (commons-quality-gate + actions + any app-affected
      jobs) — acceptance: conclusion=success; poll every 3 min, never `gh run watch` for long jobs.

### Phase 1 Gate

> All checks below must pass before archiving or starting Phase 2.

- [ ] [AI] `git grep -nE '\bBASE_URL\b' -- apps/ose-be-e2e apps/organiclever-be-e2e` — acceptance: zero matches.
- [ ] [AI] `git grep -nE '\bBASE_URL\b' -- apps/ose-www-be-e2e apps/ayokoding-www-be-e2e apps/organiclever-www-be-e2e`
      — acceptance: exactly 3 matches (www suites untouched).
- [ ] [AI] `grep -n 'API_BASE_URL' .github/workflows/_reusable-app-test-local-deploy-stag.yml` — acceptance: matches.
- [ ] [AI] `git grep -nE 'https?://' -- apps/ose-be-e2e/README.md apps/organiclever-be-e2e/README.md | grep -i stag`
      — acceptance: no private staging URL committed.

> **Pause Safety**: rename committed + CI green; behavior-neutral. Safe to stop.

---

## Phase 2: Staging backend E2E consumption — DEFERRED

- [ ] [DEFERRED] Wire a staging backend E2E gate that runs `*-be-e2e` against a deployed staging backend
      URL sourced from the `*-app-staging` GitHub Environment `API_BASE_URL`. **Blocked on** ose-infra
      exposing a reachable staging backend URL (backends deploy to k3s via `coralpolyp`, not Vercel). Not
      executed by this plan — tracked here so the operator-created `API_BASE_URL` variable's eventual
      reader is documented. Re-scope into its own plan when the staging backend URL exists.

---

## Plan Archival

- [ ] [AI] After Phase 1 lands and CI is green, move plan to `done/`:

  ```bash
  git mv plans/in-progress/rename-be-e2e-api-base-url/ plans/done/YYYY-MM-DD__rename-be-e2e-api-base-url/
  ```

  Acceptance: folder exists under `plans/done/`. (Phase 2 is DEFERRED, not a blocker for archival — note
  it in `plans/done/README.md` as carried-forward follow-up.)

- [ ] [AI] Remove this plan's row from `plans/in-progress/README.md`.
- [ ] [AI] Add an entry to `plans/done/README.md` (note the deferred Phase 2).
- [ ] [AI] Commit `chore(plans): move rename-be-e2e-api-base-url to done`.
