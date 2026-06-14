---
title: "Delivery — Wire the www + app-web Tiers to the Vercel Pipeline"
description: Phased, executor-tagged delivery checklist with gates for the Vercel prod cutover
---

# Delivery Checklist — Vercel www + app-web Cutover

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.

## Worktree

Worktree path: `worktrees/wire-vercel-www-app-cutover/`

Optional manual pre-provisioning (run from repo root):

```bash
claude --worktree wire-vercel-www-app-cutover
```

The plan-execution Step 0 gate enters this worktree by default: it auto-provisions from the latest
`origin/main` when missing, syncs with `origin/main` before implementing, and prompts before deleting
the worktree after the plan is archived and pushed.

---

## Commit Guidelines

- Use Conventional Commits format: `<type>(<scope>): <description>`
- Split commits by domain/concern: wiring edits, branch creation, docs updates are separate commits
- Imperative mood, no period at end
- Never bundle unrelated fixes into a commit
- Fix ALL pre-commit hook failures before retrying — do not use `--no-verify`

**See**: [Commit Messages Convention](../../../repo-governance/development/workflow/commit-messages.md)

---

## Phase 0: Prerequisite Verification

> All Phase 0 checks confirm that the upstream restructure landed before any wiring edit begins.

- [ ] [AI] Provision worktree:

  ```bash
  claude --worktree wire-vercel-www-app-cutover
  ```

  Acceptance criterion: `worktrees/wire-vercel-www-app-cutover/` exists and `git status` is clean.

- [ ] [AI] Run `ls apps/ose-www apps/ayokoding-www apps/organiclever-www apps/wahidyankf-www apps/organiclever-app-web apps/ose-app-web` — acceptance: all six directories exist (restructure merged).
- [ ] [AI] Run `rg 'prod-(ose|ayokoding|organiclever|wahidyankf)-web' apps/ .claude/ .github/ AGENTS.md docs/ --count` to record starting stale-reference count — acceptance: output logged for comparison in Phase 4.
- [ ] [AI] Run `git ls-remote --heads origin` and verify `prod-ose-web`, `prod-ayokoding-web`, `prod-wahidyankf-web`, `stag-organiclever-web`, `prod-organiclever-web` still exist (rollback anchors intact) — acceptance: all five listed.
- [ ] [AI] Run `npm install && npm run doctor -- --scope minimal` — acceptance: exits 0.

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [ ] [AI] `ls apps/ose-www apps/ayokoding-www apps/organiclever-www apps/wahidyankf-www apps/organiclever-app-web apps/ose-app-web` — acceptance: exits 0 (all six dirs exist).
- [ ] [AI] `git ls-remote --heads origin | grep -E 'prod-ose-web|prod-ayokoding-web|prod-wahidyankf-web'` — acceptance: at least these three listed.

> **Pause Safety**: Phase 0 is read-only verification only. Repository and Vercel state unchanged.
> Safe to stop. To resume: re-run the Phase 0 checklist.

---

## Phase 1: In-Repo Wiring Edits (vercel.json + deployer agents + workflows + docs)

> All edits stay on the local worktree branch. Nothing deploys in this phase.

### 1a — Update vercel.json ignoreCommand for each www site

- [ ] [AI] Edit `apps/ose-www/vercel.json` (copy `apps/ose-www/vercel.json` if renamed app does not have one yet): set `ignoreCommand` to `[ "$VERCEL_GIT_COMMIT_REF" != "prod-ose-www" ]` — acceptance: `cat apps/ose-www/vercel.json | grep ignoreCommand` shows `prod-ose-www`.
- [ ] [AI] Edit `apps/ayokoding-www/vercel.json`: set `ignoreCommand` to `[ "$VERCEL_GIT_COMMIT_REF" != "prod-ayokoding-www" ]` — acceptance: grep confirms `prod-ayokoding-www`.
- [ ] [AI] Edit `apps/wahidyankf-www/vercel.json`: set `ignoreCommand` to `[ "$VERCEL_GIT_COMMIT_REF" != "prod-wahidyankf-www" ]` — acceptance: grep confirms `prod-wahidyankf-www`.
- [ ] [AI] Create `apps/organiclever-www/vercel.json` (model on `apps/wahidyankf-www/vercel.json`): set `ignoreCommand` to `[ "$VERCEL_GIT_COMMIT_REF" != "prod-organiclever-www" ]` — acceptance: file exists and grep confirms branch name.
- [ ] [AI] Create `apps/organiclever-app-web/vercel.json` (model on `apps/wahidyankf-www/vercel.json`): set `ignoreCommand` to `[ "$VERCEL_GIT_COMMIT_REF" != "prod-organiclever-app-web" ]` — acceptance: file exists and grep confirms branch name.
- [ ] [AI] Create `apps/ose-app-web/vercel.json` (model on `apps/wahidyankf-www/vercel.json`): set `ignoreCommand` to `[ "$VERCEL_GIT_COMMIT_REF" != "prod-ose-app-web" ]` — acceptance: file exists and grep confirms branch name.

> These paths exist only after `restructure-fsharp-be-and-web-app-tiers` has merged — Phase 0 gate
> verifies their presence.

### 1b — Rename and update deployer agents

- [ ] [AI] In `.claude/agents/`: rename `apps-ose-web-deployer.md` → `apps-ose-www-deployer.md`; update `name: apps-ose-www-deployer`, `description`, and every occurrence of `prod-ose-web` → `prod-ose-www` inside the file — acceptance: `grep 'prod-ose-web' .claude/agents/apps-ose-www-deployer.md` returns nothing.
- [ ] [AI] Rename `apps-ayokoding-web-deployer.md` → `apps-ayokoding-www-deployer.md`; update name, description, `prod-ayokoding-web` → `prod-ayokoding-www` — acceptance: no stale branch name in new file.
- [ ] [AI] Rename `apps-organiclever-web-deployer.md` → `apps-organiclever-www-deployer.md`; update name, description, branch refs — acceptance: no stale branch name in new file.
- [ ] [AI] Rename `apps-wahidyankf-web-deployer.md` → `apps-wahidyankf-www-deployer.md`; update name, description, `prod-wahidyankf-web` → `prod-wahidyankf-www` — acceptance: no stale branch name in new file.
- [ ] [AI] Create `.claude/agents/apps-ose-app-web-deployer.md` (model on `apps-wahidyankf-www-deployer.md`): set `name: apps-ose-app-web-deployer`, production branch `prod-ose-app-web`, staging branch `stag-ose-app-web`, domain `app.oseplatform.com` — acceptance: file exists and `grep 'prod-ose-app-web'` returns a match.
- [ ] [AI] Create `.claude/agents/apps-organiclever-app-web-deployer.md` similarly: `name: apps-organiclever-app-web-deployer`, production branch `prod-organiclever-app-web`, staging branch `stag-organiclever-app-web`, domain `app.organiclever.com` — acceptance: file exists and `grep 'prod-organiclever-app-web'` returns a match.
- [ ] [AI] Run `npm run generate:bindings` to resync `.opencode/agents/` — acceptance: exits 0; `git diff --stat .opencode/agents/` shows agent mirror changes.

### 1c — Update GitHub Actions deploy workflows

- [ ] [AI] Edit `.github/workflows/test-and-deploy-ose-web.yml`: update `push.branches` filter `prod-ose-web` → `prod-ose-www`, update `paths` filter `apps/ose-www/**` → `apps/ose-www/**`, update the force-push command target — acceptance: `grep 'prod-ose-web' .github/workflows/test-and-deploy-ose-web.yml` returns nothing. Do NOT rename the workflow file — keep existing filename `test-and-deploy-ose-web.yml` to preserve CI history. Only update the branch references and affected-path filters inside the file.
- [ ] [AI] Edit `.github/workflows/test-and-deploy-ayokoding-web.yml`: same pattern → `prod-ayokoding-www`, `apps/ayokoding-www/**` — acceptance: no stale branch name.
- [ ] [AI] Edit `.github/workflows/test-and-deploy-wahidyankf-web.yml` (if exists): `prod-wahidyankf-web` → `prod-wahidyankf-www`, `apps/wahidyankf-www/**` → `apps/wahidyankf-www/**` — acceptance: no stale branch name.
- [ ] [AI] Edit `.github/workflows/deploy-organiclever-web-to-production.yml`: update staging branch `stag-organiclever-web` → `stag-organiclever-app-web`, production branch `prod-organiclever-web` → `prod-organiclever-app-web`, path filter `apps/organiclever-web/**` → `apps/organiclever-app-web/**` — acceptance: no stale names remain.
- [ ] [AI] Create `.github/workflows/deploy-ose-app-web-to-production.yml` (model on updated `deploy-organiclever-web-to-production.yml`): staging → `stag-ose-app-web`, production → `prod-ose-app-web`, path filter `apps/ose-app-web/**` — acceptance: file exists, lints cleanly with actionlint.
- [ ] [AI] Create `.github/workflows/deploy-organiclever-www-to-production.yml` for the marketing site direct deploy (model on `test-and-deploy-wahidyankf-web.yml`): branch `prod-organiclever-www`, path `apps/organiclever-www/**` — acceptance: file exists, lints cleanly.

### 1d — Update AGENTS.md and app READMEs

- [ ] [AI] Edit `AGENTS.md` lines ~231–234 (prod-branch list): replace `prod-ose-web`, `prod-ayokoding-web`, `prod-organiclever-web`, `prod-wahidyankf-web` with `prod-ose-www`, `prod-ayokoding-www`, `prod-organiclever-www`, `prod-wahidyankf-www`; add `prod-organiclever-app-web`, `prod-ose-app-web` — acceptance: `grep 'prod-.*-web\b' AGENTS.md` returns zero matches outside historical context.
- [ ] [AI] Edit `AGENTS.md` per-site "Production branch" rows (~459, 471, 483, 495, 507): update each branch name and URL — acceptance: all six sites have correct branch in AGENTS.md.
- [ ] [AI] Edit `apps/ose-www/README.md`: update deploy-branch reference `prod-ose-web` → `prod-ose-www` — acceptance: `grep 'prod-ose-web' apps/ose-www/README.md` returns nothing.
- [ ] [AI] Edit `apps/ayokoding-www/README.md`: `prod-ayokoding-web` → `prod-ayokoding-www` — acceptance: no stale branch name.
- [ ] [AI] Edit `apps/wahidyankf-www/README.md`: `prod-wahidyankf-web` → `prod-wahidyankf-www` — acceptance: no stale branch name.
- [ ] [AI] Edit `apps/organiclever-www/README.md` (create if absent): add deploy-branch `prod-organiclever-www`, URL `www.organiclever.com` — acceptance: file exists with correct branch.
- [ ] [AI] Edit `apps/organiclever-app-web/README.md` (create if absent): add deploy-branch `prod-organiclever-app-web`, staging `stag-organiclever-app-web`, URL `app.organiclever.com` — acceptance: file exists with correct branches.
- [ ] [AI] Edit `apps/ose-app-web/README.md` (create if absent): add deploy-branch `prod-ose-app-web`, staging `stag-ose-app-web`, URL `app.oseplatform.com` — acceptance: file exists with correct branches.

### 1e — Update architecture docs

- [ ] [AI] Edit `docs/reference/system-architecture/applications.md`: update prod-branch references for all four www sites and add the two app-web entries — acceptance: no `prod-*-web` references outside historical notes.
- [ ] [AI] Edit `docs/reference/system-architecture/ci-cd.md`: update all `prod-*-web`, `stag-organiclever-web` references and the workflow table — acceptance: `grep 'prod-.*-web\b' docs/reference/system-architecture/ci-cd.md` returns nothing.
- [ ] [AI] Edit `docs/reference/system-architecture/deployment.md`: update branch names in the Mermaid diagram nodes and the branch list — acceptance: diagram compiles via `npx nx run rhino-cli:mermaid:validation`.
- [ ] [AI] Run `npx nx run rhino-cli:links:validation` — acceptance: exits 0 (no broken links introduced by edits).

### 1f — Commit wiring edits

- [ ] [AI] Stage explicit file paths (`git add apps/*/vercel.json .claude/agents/ .github/workflows/ AGENTS.md apps/*/README.md docs/reference/system-architecture/`) and commit with message `chore(vercel): rewire www + app-web tier prod branches and deployer agents` — acceptance: `git log --oneline -1` shows the commit; `git status` clean.

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [ ] [AI] `rg 'prod-(ose|ayokoding|organiclever|wahidyankf)-web\b' apps/ .claude/agents/ .github/workflows/ AGENTS.md docs/` — acceptance: zero matches.
- [ ] [AI] `npx nx run rhino-cli:links:validation` — acceptance: exits 0.
- [ ] [AI] `npx nx run rhino-cli:mermaid:validation` — acceptance: exits 0.
- [ ] [AI] `git log --oneline -1` — acceptance: wiring commit exists.

> **Pause Safety**: All in-repo references updated and committed. Vercel and DNS unchanged — production
> still serving from old `prod-*-web` branches. Safe to stop. To resume: skip to Phase 2 (branch
> creation).

---

## Phase 2: Create New Production Branches

- [ ] [AI] Run `git fetch origin main && git push origin origin/main:refs/heads/prod-ose-www` — acceptance: `git ls-remote --heads origin prod-ose-www` shows the branch.
- [ ] [AI] Run `git push origin origin/main:refs/heads/prod-ayokoding-www` — acceptance: branch listed in remote.
- [ ] [AI] Run `git push origin origin/main:refs/heads/prod-organiclever-www` — acceptance: branch listed in remote.
- [ ] [AI] Run `git push origin origin/main:refs/heads/prod-wahidyankf-www` — acceptance: branch listed in remote.
- [ ] [AI] Run `git push origin origin/main:refs/heads/stag-organiclever-app-web` — acceptance: branch listed in remote.
- [ ] [AI] Run `git push origin origin/main:refs/heads/prod-organiclever-app-web` — acceptance: branch listed in remote.
- [ ] [AI] Run `git push origin origin/main:refs/heads/stag-ose-app-web` — acceptance: branch listed in remote.
- [ ] [AI] Run `git push origin origin/main:refs/heads/prod-ose-app-web` — acceptance: branch listed in remote.
- [ ] [AI] Run quality gates: `npx nx affected -t typecheck lint && npm run lint:md && npx nx run rhino-cli:links:validation && npx nx run rhino-cli:mermaid:validation`
  - Acceptance criterion: zero errors
  - Fix ALL failures found, not just those caused by current changes
- [ ] [AI] Push wiring commit to origin main: `git push origin HEAD:main` — acceptance: `git log origin/main --oneline -1` matches local HEAD.
- [ ] [AI] Verify GitHub Actions CI passes: `gh run list --branch main --limit 5` then `gh run view <run-id> --json status,conclusion`
  - Poll every 3 minutes until status=completed
  - Acceptance criterion: conclusion=success for all affected workflows
  - If any fail: investigate root cause, fix, re-push — never skip or bypass

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [ ] [AI] `git ls-remote --heads origin | grep -E 'prod-ose-www|prod-ayokoding-www|prod-organiclever-www|prod-wahidyankf-www|prod-organiclever-app-web|prod-ose-app-web'` — acceptance: all six listed.
- [ ] [AI] `git ls-remote --heads origin | grep -E 'stag-organiclever-app-web|stag-ose-app-web'` — acceptance: both listed.

> **Pause Safety**: New branches exist on origin; wiring edits pushed. Old `prod-*-web` branches still
> live — Vercel still deploying from them. Safe to stop. To resume: proceed to Phase 3 (Vercel + DNS).

---

## Phase 3: Vercel Dashboard and DNS (Human Steps)

> All steps in this phase require Vercel and DNS credentials. The AI agent cannot perform them.

- [ ] [HUMAN] In the Vercel dashboard: for the **ose-www** project, set Production Branch to `prod-ose-www`, set Root Directory to `apps/ose-www`, rename the project to `ose-www` if needed — acceptance: Vercel dashboard shows production branch `prod-ose-www`.
- [ ] [HUMAN] Trigger a Vercel build for `ose-www` by pushing `prod-ose-www` (e.g., `git push origin origin/main:prod-ose-www --force`) — acceptance: Vercel build succeeds; `curl -sI https://oseplatform.com | head -1` returns `HTTP/... 200`.
- [ ] [HUMAN] Repeat for **ayokoding-www**: set production branch `prod-ayokoding-www`, root `apps/ayokoding-www`, force-push branch, verify `curl -sI https://ayokoding.com | head -1` → 200.
- [ ] [HUMAN] Repeat for **wahidyankf-www**: set production branch `prod-wahidyankf-www`, root `apps/wahidyankf-www`, force-push, verify `curl -sI https://www.wahidyankf.com | head -1` → 200.
- [ ] [HUMAN] For **organiclever-www**: reuse the existing OrganicLever marketing Vercel project; set production branch `prod-organiclever-www`, root `apps/organiclever-www`, force-push, verify `curl -sI https://www.organiclever.com | head -1` → 200.
- [ ] [HUMAN] **Create new Vercel project** for `organiclever-app-web`: connect the repo, set root `apps/organiclever-app-web`, production branch `prod-organiclever-app-web`, staging branch `stag-organiclever-app-web`; deploy — acceptance: Vercel dashboard shows new project with green build.
- [ ] [HUMAN] Add DNS CNAME for `app.organiclever.com` pointing to the Vercel-assigned `*.vercel.app` domain — acceptance: `dig app.organiclever.com CNAME` shows the Vercel target; `curl -sI https://app.organiclever.com | head -1` → 200 (may take up to 48 h for DNS propagation).
- [ ] [HUMAN] **Create new Vercel project** for `ose-app-web`: connect repo, root `apps/ose-app-web`, production branch `prod-ose-app-web`, staging `stag-ose-app-web`; deploy — acceptance: Vercel shows green build.
- [ ] [HUMAN] Add DNS CNAME for `app.oseplatform.com` → Vercel target — acceptance: `dig app.oseplatform.com CNAME` resolves; `curl -sI https://app.oseplatform.com | head -1` → 200.

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [ ] [AI] `curl -sI https://oseplatform.com | head -1` — acceptance: `HTTP/... 200`.
- [ ] [AI] `curl -sI https://ayokoding.com | head -1` — acceptance: `HTTP/... 200`.
- [ ] [AI] `curl -sI https://www.organiclever.com | head -1` — acceptance: `HTTP/... 200`.
- [ ] [AI] `curl -sI https://www.wahidyankf.com | head -1` — acceptance: `HTTP/... 200`.
- [ ] [HUMAN] Verify `app.organiclever.com` returns 200 in a browser (DNS propagation may require patience).
- [ ] [HUMAN] Verify `app.oseplatform.com` returns 200 in a browser.

> **Pause Safety**: All six production domains serve from new branches. Old branches still exist as
> rollback. Safe to stop. To resume: verify domains still 200, then proceed to Phase 4 (retire branches).

---

## Phase 4: Retire Obsolete Branches and Final Verification

- [ ] [AI] Run `rg 'prod-(ose|ayokoding|organiclever|wahidyankf)-web\b' apps/ .claude/ .github/ AGENTS.md docs/ --count` — acceptance: zero matches (compare to Phase 0 baseline).
- [ ] [HUMAN] Delete obsolete remote branches: `git push origin --delete prod-ose-web prod-ayokoding-web prod-organiclever-web prod-wahidyankf-web stag-organiclever-web` — acceptance: `git ls-remote --heads origin | grep -E 'prod-(ose|ayokoding|organiclever|wahidyankf)-web'` returns empty.
- [ ] [AI] Confirm the eight new branches remain: `git ls-remote --heads origin | grep -E 'prod-(ose|ayokoding|organiclever|wahidyankf)-www|prod-(organiclever|ose)-app-web|stag-(organiclever|ose)-app-web'` — acceptance: all eight listed.
- [ ] [AI] Run `npx nx run rhino-cli:links:validation` — acceptance: exits 0.
- [ ] [AI] Run `npm run lint:md` — acceptance: exits 0.

### Phase 4 Gate

> All checks below must pass before declaring this plan complete.

- [ ] [AI] `rg 'prod-(ose|ayokoding|organiclever|wahidyankf)-web\b' apps/ .claude/ .github/ AGENTS.md docs/` — acceptance: zero matches.
- [ ] [AI] `git ls-remote --heads origin | grep -E 'prod-.*-web\b'` — acceptance: zero matches (old branches gone).
- [ ] [AI] `git ls-remote --heads origin | grep -c 'prod-.*-www\|prod-.*-app-web\|stag-.*-app-web'` — acceptance: count ≥ 8.
- [ ] [AI] `npx nx run rhino-cli:links:validation` — acceptance: exits 0.

> **Pause Safety**: All production domains live on new branches; old branches deleted; zero stale
> references in repo. Plan complete. To resume verification: re-run the Phase 4 gate commands.

---

## Plan Archival

- [ ] [AI] Move plan to `done/`:

  ```bash
  git mv plans/in-progress/wire-vercel-www-app-cutover/ plans/done/YYYY-MM-DD__wire-vercel-www-app-cutover/
  ```

  Replace `YYYY-MM-DD` with actual completion date. Acceptance criterion: folder exists under `plans/done/`.

- [ ] [AI] Remove this plan's row from `plans/in-progress/README.md`
- [ ] [AI] Update `plans/done/README.md` to add an entry for this plan
- [ ] [AI] Commit: `chore(plans): move wire-vercel-www-app-cutover to done`
