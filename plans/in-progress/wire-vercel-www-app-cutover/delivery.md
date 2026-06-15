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

- [x] [AI] Provision worktree — **ADAPTED**: per user directive, executing directly on `main` (no worktree). `git status` clean at start.

  ```bash
  claude --worktree wire-vercel-www-app-cutover
  ```

  Acceptance criterion: `worktrees/wire-vercel-www-app-cutover/` exists and `git status` is clean.

- [x] [AI] Run `ls apps/ose-www apps/ayokoding-www apps/organiclever-www apps/wahidyankf-www apps/organiclever-app-web apps/ose-app-web` — acceptance: all six directories exist (restructure merged). ✓ all six present.
- [x] [AI] Run `rg 'prod-(ose|ayokoding|organiclever|wahidyankf)-web' apps/ .claude/ .github/ AGENTS.md docs/ --count` to record starting stale-reference count — acceptance: output logged for comparison in Phase 4. **Baseline = 105 matches** (note: `--count-matches` total; `.github/` already clean from the standardize plan).
- [x] [AI] Run `git ls-remote --heads origin` and verify `prod-ose-web`, `prod-ayokoding-web`, `prod-wahidyankf-web`, `stag-organiclever-web`, `prod-organiclever-web` still exist (rollback anchors intact) — acceptance: all five listed. ✓ all five present on origin.
- [x] [AI] Run `npm install && npm run doctor -- --scope minimal` — acceptance: exits 0. ✓ `6/6 tools OK` after the doctor purge committed (`e52dc712f`). Polyglot phantom-tool noise gone.

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [x] [AI] `ls apps/ose-www apps/ayokoding-www apps/organiclever-www apps/wahidyankf-www apps/organiclever-app-web apps/ose-app-web` — acceptance: exits 0 (all six dirs exist). ✓
- [x] [AI] `git ls-remote --heads origin | grep -E 'prod-ose-web|prod-ayokoding-web|prod-wahidyankf-web'` — acceptance: at least these three listed. ✓ all three present.

> **Pause Safety**: Phase 0 is read-only verification only. Repository and Vercel state unchanged.
> Safe to stop. To resume: re-run the Phase 0 checklist.

---

## Phase 1: In-Repo Wiring Edits (vercel.json + deployer agents + workflow verification + docs)

> All edits stay on the local worktree branch. Nothing deploys in this phase.

### 1a — Update vercel.json ignoreCommand for each www site

- [x] [AI] Edit `apps/ose-www/vercel.json`: `ignoreCommand` → `prod-ose-www`. ✓
- [x] [AI] Edit `apps/ayokoding-www/vercel.json`: `ignoreCommand` → `prod-ayokoding-www`. ✓
- [x] [AI] Edit `apps/wahidyankf-www/vercel.json`: `ignoreCommand` → `prod-wahidyankf-www`. ✓
- [x] [AI] Create `apps/organiclever-www/vercel.json`: `ignoreCommand` → `prod-organiclever-www`. ✓
- [x] [AI] Create `apps/organiclever-app-web/vercel.json`: `ignoreCommand` → `prod-organiclever-app-web`. ✓
- [x] [AI] Create `apps/ose-app-web/vercel.json`: `ignoreCommand` → `prod-ose-app-web`. ✓

> These paths exist only after `restructure-fsharp-be-and-web-app-tiers` has merged — Phase 0 gate
> verifies their presence.

### 1b — Rename and update deployer agents (EXPANDED to a full `-web` → `-www`/`-app-web` sweep per user)

> **Scope expansion (user-approved 2026-06-15).** The user directed a full rename of every
> outdated `-web` agent and skill, not just the deployers. Executed sweep:
>
> - **Deployers** (`git mv` + content update): `apps-ose-web-deployer` → `apps-ose-www-deployer`,
>   `apps-ayokoding-web-deployer` → `apps-ayokoding-www-deployer`,
>   `apps-wahidyankf-web-deployer` → `apps-wahidyankf-www-deployer`.
> - **Reconciliation**: `apps-organiclever-web-deployer` was already the OrganicLever **app-group
>   staging** deployer (force-pushes `stag-organiclever-app-web` + `stag-organiclever-be`, prod CD
>   deferred), NOT a www-marketing deployer. Renamed it to its true identity
>   `apps-organiclever-app-web-deployer`, and **created a separate** `apps-organiclever-www-deployer`
>   for the marketing site (`prod-organiclever-www`).
> - **New**: `apps-ose-app-web-deployer` (ose app group: `stag-ose-app-web`, `app.oseplatform.com`,
>   prod CD deferred — modeled on the organiclever app-group deployer).
> - **Content-agent families** (`git mv` + cross-ref sweep): `apps-ose-web-content-{maker,checker,fixer}`
>   → `apps-ose-www-content-*`; all 13 `apps-ayokoding-web-*` → `apps-ayokoding-www-*`.
> - **Skills** (`git mv` dir + `name:` + all references): `apps-ose-web-developing-content`,
>   `apps-ayokoding-web-developing-content`, `apps-organiclever-web-developing-content` → `…-www-…`.
> - **Cross-refs swept** across `AGENTS.md`, `CLAUDE.md`, `.claude/agents/README.md`,
>   `.claude/skills/**`, `repo-governance/**`, app READMEs, active plans. **Excluded** (history not
>   falsified): `apps/*/content/**` changelog posts, `generated-reports/**`, `generated-socials/**`,
>   `plans/done/**`, and the regenerated `.opencode/**` / `.amazonq/**` bindings.

- [x] [AI] Rename `apps-ose-web-deployer.md` → `apps-ose-www-deployer.md`; update `name`, `description`, and `prod-ose-web` → `prod-ose-www` — acceptance: `grep 'prod-ose-web' .claude/agents/apps-ose-www-deployer.md` returns nothing. ✓
- [x] [AI] Rename `apps-ayokoding-web-deployer.md` → `apps-ayokoding-www-deployer.md`; `prod-ayokoding-web` → `prod-ayokoding-www` — acceptance: no stale branch name. ✓
- [x] [AI] Rename `apps-organiclever-web-deployer.md` → `apps-organiclever-app-web-deployer.md` (app-group deployer) **and** create `apps-organiclever-www-deployer.md` (marketing, `prod-organiclever-www`) — acceptance: both exist; no stale branch name. ✓
- [x] [AI] Rename `apps-wahidyankf-web-deployer.md` → `apps-wahidyankf-www-deployer.md`; `prod-wahidyankf-web` → `prod-wahidyankf-www` — acceptance: no stale branch name. ✓
- [x] [AI] Create `.claude/agents/apps-ose-app-web-deployer.md`: `name: apps-ose-app-web-deployer`, staging `stag-ose-app-web`, domain `app.oseplatform.com`, prod CD deferred — acceptance: file exists and `grep 'stag-ose-app-web'` matches. ✓
- [x] [AI] Rename content-agent families + the 3 `*-developing-content` skills, sweep all cross-refs (see scope box) — acceptance: `git grep 'apps-(ose|ayokoding|wahidyankf|organiclever)-web-' -- ':!plans/done' ':!*/content/**'` returns only `apps-organiclever-app-web-deployer`. ✓
- [x] [AI] Run `npm run generate:bindings` to resync `.opencode/agents/` + `.amazonq/` — acceptance: exits 0; mirrors regenerated. ✓ (run at end of Phase 1)

### 1c — Verify the standardized GitHub Actions workflows (no edits here)

> **Workflows are owned by `standardize-github-actions-pipeline-naming`, which has already landed.**
> This plan does **not** edit, rename, or create any workflow file — it only confirms the standardized
> set references the branches and Environments this plan creates. See
> [tech-docs → Workflows owned by the standardize plan](./tech-docs.md#github-actions-workflows--owned-by-the-standardize-plan-verify-only).

- [x] [AI] Confirm the standardized www callers force-push the right branches: `grep -l 'prod-{site}-www'`-style check across `{ose,ayokoding,organiclever,wahidyankf}-www-test-local-deploy-prod.yml` — acceptance: each names its `prod-*-www` branch; no `prod-*-web` remains.
- [x] [AI] Confirm the standardized app pipelines reference the staging branches + `{group}-app-staging` Environment: `grep -rnE 'stag-(organiclever|ose)-(app-web|be)|(organiclever|ose)-app-staging' .github/workflows/*-app-test-*.yml` — acceptance: the `stag-*-app-web`, `stag-*-be`, and `*-app-staging` references are present; no `*-app-web-development` / `*-app-web-production` env names exist (the standardize model has only `local` + `staging`).
- [x] [AI] Confirm the staging gate reads `vars.WEB_BASE_URL` + `secrets.VERCEL_AUTOMATION_BYPASS_SECRET` — acceptance: `grep -rn 'WEB_BASE_URL\|VERCEL_AUTOMATION_BYPASS_SECRET' .github/workflows/_reusable-app-test-stag.yml` shows both.
- [x] [AI] Run `actionlint .github/workflows/*.yml` — acceptance: zero errors (sanity; no files changed here).

### 1d — Update AGENTS.md and app READMEs

- [x] [AI] Edit `AGENTS.md` lines ~231–234 (prod-branch list): replace `prod-ose-web`, `prod-ayokoding-web`, `prod-organiclever-web`, `prod-wahidyankf-web` with `prod-ose-www`, `prod-ayokoding-www`, `prod-organiclever-www`, `prod-wahidyankf-www`; add `prod-organiclever-app-web`, `prod-ose-app-web` — acceptance: `grep 'prod-.*-web\b' AGENTS.md` returns zero matches outside historical context.
- [x] [AI] Edit `AGENTS.md` per-site "Production branch" rows (~459, 471, 483, 495, 507): update each branch name and URL — acceptance: all six sites have correct branch in AGENTS.md.
- [x] [AI] Edit `apps/ose-www/README.md`: update deploy-branch reference `prod-ose-web` → `prod-ose-www` — acceptance: `grep 'prod-ose-web' apps/ose-www/README.md` returns nothing.
- [x] [AI] Edit `apps/ayokoding-www/README.md`: `prod-ayokoding-web` → `prod-ayokoding-www` — acceptance: no stale branch name.
- [x] [AI] Edit `apps/wahidyankf-www/README.md`: `prod-wahidyankf-web` → `prod-wahidyankf-www` — acceptance: no stale branch name.
- [x] [AI] Edit `apps/organiclever-www/README.md` (create if absent): add deploy-branch `prod-organiclever-www`, URL `www.organiclever.com` — acceptance: file exists with correct branch.
- [x] [AI] Edit `apps/organiclever-app-web/README.md` (create if absent): add deploy-branch `prod-organiclever-app-web`, staging `stag-organiclever-app-web`, URL `app.organiclever.com` — acceptance: file exists with correct branches.
- [x] [AI] Edit `apps/ose-app-web/README.md` (create if absent): add deploy-branch `prod-ose-app-web`, staging `stag-ose-app-web`, URL `app.oseplatform.com` — acceptance: file exists with correct branches.

### 1e — Update architecture docs

- [x] [AI] Edit `docs/reference/system-architecture/applications.md`: update prod-branch references for all four www sites and add the two app-web entries — acceptance: no `prod-*-web` references outside historical notes.
- [x] [AI] Edit `docs/reference/system-architecture/ci-cd.md`: update all `prod-*-web`, `stag-organiclever-web` references and the workflow table — acceptance: `grep 'prod-.*-web\b' docs/reference/system-architecture/ci-cd.md` returns nothing.
- [x] [AI] Edit `docs/reference/system-architecture/deployment.md`: update branch names in the Mermaid diagram nodes and the branch list — acceptance: diagram compiles via `npx nx run rhino-cli:mermaid:validation`.
- [x] [AI] Run `npx nx run rhino-cli:links:validation` — acceptance: exits 0 (no broken links introduced by edits).

### 1g — Document the app-web staging deployments (placeholder/secret only)

> Vercel also listens on each `stag-*-app-web` branch and serves it at a private staging URL (see
> tech-docs D1). Propagate this topology to every related doc, referencing the URL ONLY via a placeholder
> (`<staging-url:ose-app-web>`) or a GitHub Actions secret — never a literal — per
> [Secrets and Env Standards](../../../repo-governance/conventions/security/secrets-and-env-standards.md).

- [x] [AI] In both app-web deployer agents (`.claude/agents/apps-ose-app-web-deployer.md`, `.claude/agents/apps-organiclever-app-web-deployer.md`), document the `stag-*-app-web` Vercel staging deployment, the `{group}-app-staging` Environment's `WEB_BASE_URL` var (private staging URL), and the required `VERCEL_AUTOMATION_BYPASS_SECRET` (Vercel Protection Bypass for Automation) — acceptance: each file names the staging branch, the `WEB_BASE_URL` var, and the bypass secret; `git grep -nE 'https?://[a-z0-9.-]*stag' .claude/agents/` returns nothing.
- [x] [AI] In `apps/organiclever-app-web/README.md` and `apps/ose-app-web/README.md`, add a "Staging" note: served from `stag-*-app-web`, staging URL kept private (placeholder/secret) — acceptance: both READMEs name the staging branch with no literal URL.
- [x] [AI] In `docs/reference/system-architecture/{applications,ci-cd,deployment}.md`, document the app-web staging deployments and the E2E promotion gate sourcing the staging base URL from a secret — acceptance: `rg -i 'stag-.*-app-web' docs/reference/system-architecture/` shows the staging deployments; `git grep -nE 'https?://[a-z0-9.-]*stag' docs/` returns nothing.
- [x] [AI] Re-run `npm run generate:bindings` to resync `.opencode/agents/` for the deployer-agent edits — acceptance: exits 0.

### 1f — Commit wiring edits

- [x] [AI] Stage explicit file paths (`git add apps/*/vercel.json .claude/agents/ .opencode/agents/ AGENTS.md apps/*/README.md docs/reference/system-architecture/`) and commit with message `chore(vercel): rewire www + app-web tier prod branches and deployer agents` — acceptance: `git log --oneline -1` shows the commit; `git status` clean. (No `.github/workflows/` — owned by the standardize plan.)

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [x] [AI] `rg 'prod-(ose|ayokoding|organiclever|wahidyankf)-web\b' apps/ .claude/agents/ AGENTS.md docs/` — acceptance: zero matches (workflows excluded — owned by the standardize plan).
- [x] [AI] `rg 'stag-organiclever-web\b|organiclever-web-(development|staging|production)' .github/workflows/` — acceptance: zero matches (the standardize plan already renamed these to the `{group}-app-staging` model).
- [x] [AI] `actionlint .github/workflows/*.yml` — acceptance: zero errors (sanity only; this plan edits no workflow).
- [x] [AI] `npx nx run rhino-cli:links:validation` — acceptance: exits 0.
- [x] [AI] `npx nx run rhino-cli:mermaid:validation` — acceptance: exits 0.
- [x] [AI] No literal staging URL committed anywhere: `git grep -nE 'https?://[a-z0-9.-]*stag'` — acceptance: zero matches (staging URLs live only in Vercel + GitHub Actions secrets).
- [x] [AI] `git log --oneline -1` — acceptance: wiring commit exists. ✓ `6f40031b2`.

**Phase 1 Gate — RESULT (2026-06-15): PASS** with two documented scope refinements:

1. **G1 (`prod-*-web` in `apps/`)** scoped to exclude `apps/*/content/**`. Five published changelog
   posts + hermes-agent tutorials reference the old branch names as point-in-time facts; rewriting them
   would falsify history (same principle as the rename sweep's content exclusion). All **active wiring**
   surfaces are zero.
2. **G6 (no literal staging URL)** — the only `https://…stag…` matches are generic tutorial example
   domains in `apps/ayokoding-www/content/**` (`staging.example.com`, `staging.myapp.com`, etc.), not
   our private Vercel staging URLs. No real staging URL is committed anywhere.

Other gate checks (G2 stale workflow refs, actionlint, links, mermaid) are clean (mermaid: 1 pre-existing
subgraph-density warning in `tech-docs.md`, non-blocking).

> **Pause Safety**: All in-repo references updated and committed. Vercel and DNS unchanged — production
> still serving from old `prod-*-web` branches. Safe to stop. To resume: skip to Phase 2 (branch
> creation).

---

## Phase 2: Create New Production Branches

- [x] [AI] Run `git fetch origin main && git push origin origin/main:refs/heads/prod-ose-www` — acceptance: `git ls-remote --heads origin prod-ose-www` shows the branch.
- [x] [AI] Run `git push origin origin/main:refs/heads/prod-ayokoding-www` — acceptance: branch listed in remote.
- [x] [AI] Run `git push origin origin/main:refs/heads/prod-organiclever-www` — acceptance: branch listed in remote.
- [x] [AI] Run `git push origin origin/main:refs/heads/prod-wahidyankf-www` — acceptance: branch listed in remote.
- [x] [AI] Run `git push origin origin/main:refs/heads/stag-organiclever-app-web` — acceptance: branch listed in remote.
- [x] [AI] Run `git push origin origin/main:refs/heads/prod-organiclever-app-web` — acceptance: branch listed in remote.
- [x] [AI] Run `git push origin origin/main:refs/heads/stag-ose-app-web` — acceptance: branch listed in remote.
- [x] [AI] Run `git push origin origin/main:refs/heads/prod-ose-app-web` — acceptance: branch listed in remote.
- [x] [AI] Run `git push origin origin/main:refs/heads/stag-organiclever-be` — acceptance: branch listed in remote (the standardized `organiclever-be-build-deploy-stag.yml` triggers on push here).
- [x] [AI] Run `git push origin origin/main:refs/heads/stag-ose-be` — acceptance: branch listed in remote (triggers `ose-be-build-deploy-stag.yml`).
- [x] [AI] Run quality gates: `npx nx affected -t typecheck lint && npm run lint:md && npx nx run rhino-cli:links:validation && npx nx run rhino-cli:mermaid:validation`
  - Acceptance criterion: zero errors
  - Fix ALL failures found, not just those caused by current changes
- [x] [AI] Push wiring commit to origin main: `git push origin HEAD:main` — acceptance: `git log origin/main --oneline -1` matches local HEAD.
- [x] [AI] Verify GitHub Actions CI passes: `gh run list --branch main --limit 5` then `gh run view <run-id> --json status,conclusion` — ✓ `commons-quality-gate` + markdown-validate + commons-env-validate + publish-images all **success** on `800c04db4`. The first push's `.NET quality gate` failure was a pre-existing codegen race (F# `lint`/`test` lacked `dependsOn: [codegen]`) — fixed in `800c04db4` (`fix(nx)`), re-verified green.
  - Poll every 3 minutes until status=completed
  - Acceptance criterion: conclusion=success for all affected workflows
  - If any fail: investigate root cause, fix, re-push — never skip or bypass

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [x] [AI] `git ls-remote --heads origin | grep -E 'prod-ose-www|prod-ayokoding-www|prod-organiclever-www|prod-wahidyankf-www|prod-organiclever-app-web|prod-ose-app-web'` — acceptance: all six listed.
- [x] [AI] `git ls-remote --heads origin | grep -E 'stag-organiclever-app-web|stag-ose-app-web'` — acceptance: both listed.
- [x] [AI] `git ls-remote --heads origin | grep -E 'stag-organiclever-be|stag-ose-be'` — acceptance: both backend staging branches listed (for the standardized `*-be-build-deploy-stag` workflows).
- [x] [AI] Exact-count guard (catches any partial/failed push): `git fetch origin --prune && git ls-remote --heads origin | grep -Ec 'refs/heads/(prod-(ose|ayokoding|organiclever|wahidyankf)-www|prod-(organiclever|ose)-app-web|stag-(organiclever|ose)-app-web)$'` — acceptance: returns exactly `8` (Vercel-referenced branches). Separately, `… | grep -Ec 'refs/heads/stag-(organiclever|ose)-be$'` returns exactly `2`. Every branch Vercel will reference MUST exist on origin before Phase 3.

> **Pause Safety**: New branches exist on origin; wiring edits pushed. Old `prod-*-web` branches still
> live — Vercel still deploying from them. Safe to stop. To resume: proceed to Phase 3 (Vercel + DNS).

---

## Phase 3: Vercel Dashboard and DNS (Human Steps)

> All steps in this phase require Vercel and DNS credentials. The AI agent cannot perform them.
>
> **Precondition — every target branch must already exist on origin before any Vercel wiring.**
> Vercel cannot point a Production/Preview Branch at a ref origin does not have; a project wired to a
> missing branch builds nothing and serves a silent 404. Confirm the full branch set is live on origin
> first, and do NOT begin the dashboard steps below until this check passes.

- [x] [AI] `git fetch origin --prune && git ls-remote --heads origin | grep -Ec 'refs/heads/(prod-(ose|ayokoding|organiclever|wahidyankf)-www|prod-(organiclever|ose)-app-web|stag-(organiclever|ose)-app-web)$'` — acceptance: returns exactly `8` (four `prod-*-www`, two `prod-*-app-web`, two `stag-*-app-web`). If fewer than 8, return to Phase 2 and push the missing branch(es) before continuing. **DONE 2026-06-15** — returned exactly `8`.
- [x] [AI] Per-branch confirmation that each ref resolves on origin: `for b in prod-ose-www prod-ayokoding-www prod-organiclever-www prod-wahidyankf-www prod-organiclever-app-web prod-ose-app-web stag-organiclever-app-web stag-ose-app-web; do git ls-remote --exit-code --heads origin "$b" >/dev/null && echo "ok $b" || echo "MISSING $b"; done` — acceptance: every line prints `ok`, none `MISSING`. **DONE 2026-06-15** — all 8 printed `ok`, none `MISSING`.

> Pure Vercel/DNS — no git pushes in this phase. Every branch already exists on origin (Phase 2), so
> wiring a project to it and clicking **Redeploy** is all that's needed; Vercel also auto-builds on connect.

- [x] [HUMAN] **ose-www**: in the Vercel dashboard set Production Branch to `prod-ose-www`, Root Directory to `apps/ose-www`, rename the project to `ose-www` if needed, then **Redeploy** from the dashboard (Deployments → Redeploy) — acceptance: Vercel shows a green build on `prod-ose-www` and `curl -sI https://www.oseplatform.com | head -1` → `HTTP/... 200`. **DONE 2026-06-15** (user-confirmed) — `curl https://www.oseplatform.com` → `HTTP/2 200`.
- [x] [HUMAN] **ayokoding-www**: set Production Branch `prod-ayokoding-www`, Root `apps/ayokoding-www`, rename if needed, Redeploy — acceptance: green build; `curl -sI https://www.ayokoding.com | head -1` → 200. **DONE 2026-06-15** (user-confirmed) — root `307 → /en → 200` (healthy i18n redirect).
- [x] [HUMAN] **wahidyankf-www**: set Production Branch `prod-wahidyankf-www`, Root `apps/wahidyankf-www`, rename if needed, Redeploy — acceptance: green build; `curl -sI https://www.wahidyankf.com | head -1` → 200. **DONE 2026-06-15** (user-confirmed) — `curl https://www.wahidyankf.com` → `HTTP/2 200`.
- [x] [HUMAN] **organiclever-www**: reuse the existing OrganicLever marketing project; set Production Branch `prod-organiclever-www`, Root `apps/organiclever-www`, Redeploy — acceptance: green build; `curl -sI https://www.organiclever.com | head -1` → 200. **DONE 2026-06-15** (user-confirmed) — `curl https://www.organiclever.com` → `HTTP/2 200`.
- [x] [HUMAN] **Create new Vercel project** `organiclever-app-web`: connect the repo, Root `apps/organiclever-app-web`, Production Branch `prod-organiclever-app-web`, staging/preview branch `stag-organiclever-app-web`; deploy from the dashboard — acceptance: Vercel shows the new project with a green build. Confirm Vercel also serves `stag-organiclever-app-web` at its staging URL. **DONE 2026-06-15** (user-confirmed project creation + branch wiring). ⚠️ Production domain `app.organiclever.com` currently serves `404` — see the DNS step and Phase 3 Gate item below (domain not yet attached to the production deployment, or production build pending).
- [x] [HUMAN] **Enable Vercel Protection Bypass for Automation** on the `organiclever-app-web` project (Settings → Deployment Protection) and copy the generated token into the `organiclever-app-staging` GitHub Environment secret `VERCEL_AUTOMATION_BYPASS_SECRET` — acceptance: the token is set as that secret (never committed). Without it the staging E2E gate 401s on the protected URL. **DONE 2026-06-15** (user-confirmed) — token copied from the `organiclever-app-web` project into `organiclever-app-staging`.
- [x] [HUMAN] Set the `organiclever-app-staging` GitHub Environment **var** `WEB_BASE_URL` to the `stag-organiclever-app-web` staging URL (private — Environment var, not committed); set the Vercel project env per target (Production for `prod-organiclever-app-web`, Preview for `stag-organiclever-app-web`) from `apps/organiclever-app-web/.env.example` per `env-injection.yaml` — acceptance: the staging E2E gate can resolve `vars.WEB_BASE_URL`; no value committed. **DONE 2026-06-15** (user-confirmed) — `WEB_BASE_URL` set in `organiclever-app-staging` (private; not committed).
- [ ] [HUMAN] Add DNS CNAME for `app.organiclever.com` → the Vercel-assigned `*.vercel.app` target — acceptance: `dig app.organiclever.com CNAME` shows the Vercel target; `curl -sI https://app.organiclever.com | head -1` → 200 (DNS may take up to 48 h to propagate). **⚠️ BLOCKED 2026-06-15** — DNS is set (`dig app.organiclever.com CNAME` → `cname.vercel-dns.com.`) but `curl https://app.organiclever.com` → `404`. Vercel resolves the domain but serves no production deployment. Fix in the `organiclever-app-web` Vercel project: confirm the domain is added to the project **and assigned to Production** (`prod-organiclever-app-web`), and that the production build succeeded (re-triggered by the latest force-push). Compare to `app.oseplatform.com`, which has a project-specific `*.vercel-dns-017.com` target and serves `200`.
- [x] [HUMAN] **Create new Vercel project** `ose-app-web`: connect repo, Root `apps/ose-app-web`, Production Branch `prod-ose-app-web`, staging/preview `stag-ose-app-web`; deploy from the dashboard — acceptance: Vercel shows a green build. Confirm Vercel also serves `stag-ose-app-web` at its staging URL. **DONE 2026-06-15** (user-confirmed) — `curl https://app.oseplatform.com` → `HTTP/2 200`.
- [x] [HUMAN] **Enable Vercel Protection Bypass for Automation** on the `ose-app-web` project and copy the token into the `ose-app-staging` GitHub Environment secret `VERCEL_AUTOMATION_BYPASS_SECRET` — acceptance: token set as that secret (never committed). **DONE 2026-06-15** (user-confirmed) — token copied from the `ose-app-web` project into `ose-app-staging`.
- [x] [HUMAN] Set the `ose-app-staging` GitHub Environment **var** `WEB_BASE_URL` to the `stag-ose-app-web` staging URL (private); set the Vercel project env per target from `apps/ose-app-web/.env.example` per `env-injection.yaml` — acceptance: gate resolves `vars.WEB_BASE_URL`; no value committed. **DONE 2026-06-15** (user-confirmed) — `WEB_BASE_URL` set in `ose-app-staging` (private; not committed).
- [x] [HUMAN] Add DNS CNAME for `app.oseplatform.com` → Vercel target — acceptance: `dig app.oseplatform.com CNAME` resolves; `curl -sI https://app.oseplatform.com | head -1` → 200. **DONE 2026-06-15** — `dig` → `57c89cbbedf9c23a.vercel-dns-017.com.`; `curl https://app.oseplatform.com` → `HTTP/2 200`.
- [x] [HUMAN] In repo **Settings → Environments**, ensure the standardize-model app Environments exist — `organiclever-app-staging`, `ose-app-staging` (and the compose-only `*-app-local`, which may be omitted if empty). **No `*-development` or `*-production` app Environments** (the standardize model is local + staging only; app-tier prod CD is a later plan). Each `*-app-staging` holds the `WEB_BASE_URL` var + `VERCEL_AUTOMATION_BYPASS_SECRET` secret set above — acceptance: every Environment named by a standardized workflow exists with its vars/secrets; no secret value committed. **DONE 2026-06-15** (user-confirmed) — the 4 valid app Environments exist (`organiclever-app-staging`, `ose-app-staging`, `organiclever-app-local`, `ose-app-local`); stale `-web`/`Production – *`/demo Environments deleted; no `*-development`/`*-production` app Environments remain.

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [ ] [AI] `curl -sI https://www.oseplatform.com | head -1` — acceptance: `HTTP/... 200`.
- [ ] [AI] `curl -sI https://www.ayokoding.com | head -1` — acceptance: `HTTP/... 200`.
- [ ] [AI] `curl -sI https://www.organiclever.com | head -1` — acceptance: `HTTP/... 200`.
- [ ] [AI] `curl -sI https://www.wahidyankf.com | head -1` — acceptance: `HTTP/... 200`.
- [ ] [HUMAN] Verify `app.organiclever.com` returns 200 in a browser (DNS propagation may require patience).
- [ ] [HUMAN] Verify `app.oseplatform.com` returns 200 in a browser.
- [ ] [HUMAN] Verify each app-web **staging** deployment serves the `stag-*-app-web` build (200) at its private staging URL — keep the URL private; do not paste it into this checklist or any committed file.
- [ ] [HUMAN] Verify the **Protection Bypass** works end-to-end: dispatch the standardized `{group}-app-test-stag-deploy-prod` workflow (or a manual `curl` with the bypass header) against the staging URL and confirm it returns 200, not 401 — acceptance: the staging E2E gate authenticates past Vercel Deployment Protection. A 401 here means `VERCEL_AUTOMATION_BYPASS_SECRET` is missing/wrong.

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
