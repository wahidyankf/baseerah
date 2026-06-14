---
title: "Tech Docs — Wire the www + app-web Tiers to the Vercel Pipeline"
description: Wiring architecture, per-project mechanics, gated-promotion design, file-impact analysis, and rollback for the Vercel prod cutover
---

# Technical Documentation — Vercel www + app-web Cutover

## Current state (pre-cutover, grounded)

`[Repo-grounded: apps/*/vercel.json, .claude/agents/apps-*-deployer.md, AGENTS.md]`

- Four public sites deploy via **force-push `main` → `prod-*-web`**, Vercel auto-builds:
  `prod-ose-web`, `prod-ayokoding-web`, `prod-wahidyankf-web`. Each `apps/<app>/vercel.json` carries an
  `ignoreCommand` of the form `[ "$VERCEL_GIT_COMMIT_REF" != "prod-<app>-web" ]` so Vercel only builds
  the production branch.
- OrganicLever uses a **gated promotion**: FE E2E runs against `stag-organiclever-web`, then a
  dispatch-only workflow force-pushes `stag-organiclever-web → prod-organiclever-web`
  `[Repo-grounded: docs/reference/system-architecture/ci-cd.md]`.
- `ose-app-web` exists post-restructure but has **no Vercel project, no branch, no domain**
  `[Repo-grounded: AGENTS.md "prod-ose-app-web (TBD)"]`.
- Backends (`organiclever-be`, `ose-be`) are **not** on Vercel.

## Target state (post-cutover)

```mermaid
flowchart LR
    subgraph www [www tier - direct deploy]
      M1[main] -->|force-push| POW[prod-ose-www] --> D1[www.oseplatform.com]
      M1 -->|force-push| PAW[prod-ayokoding-www] --> D2[www.ayokoding.com]
      M1 -->|force-push| POLW[prod-organiclever-www] --> D3[www.organiclever.com]
      M1 -->|force-push| PWW[prod-wahidyankf-www] --> D4[www.wahidyankf.com]
    end
    subgraph app [app-web tier - gated promotion]
      M1 -->|push| SOL[stag-organiclever-app-web] -->|E2E gate, dispatch| POLA[prod-organiclever-app-web] --> D5[app.organiclever.com]
      M1 -->|push| SOSE[stag-ose-app-web] -->|E2E gate, dispatch| POSA[prod-ose-app-web] --> D6[app.oseplatform.com]
    end

    style M1 fill:#0173B2,stroke:#000,color:#FFF
    style POW fill:#DE8F05,stroke:#000,color:#000
    style PAW fill:#DE8F05,stroke:#000,color:#000
    style POLW fill:#DE8F05,stroke:#000,color:#000
    style PWW fill:#DE8F05,stroke:#000,color:#000
    style SOL fill:#CC78BC,stroke:#000,color:#000
    style SOSE fill:#CC78BC,stroke:#000,color:#000
    style POLA fill:#DE8F05,stroke:#000,color:#000
    style POSA fill:#DE8F05,stroke:#000,color:#000
    style D1 fill:#029E73,stroke:#000,color:#FFF
    style D2 fill:#029E73,stroke:#000,color:#FFF
    style D3 fill:#029E73,stroke:#000,color:#FFF
    style D4 fill:#029E73,stroke:#000,color:#FFF
    style D5 fill:#029E73,stroke:#000,color:#FFF
    style D6 fill:#029E73,stroke:#000,color:#FFF
```

## Design decisions

### D1 — www tier keeps direct deploy; app-web tier uses a staging gate

The `-www` sites are content/marketing and already deploy directly (`main → prod-*-www`). The
`-app-web` apps are real CSR clients that call a backend; they get a staging gate
(`stag-*-app-web → prod-*-app-web`) mirroring today's OrganicLever promotion. This preserves the
existing OrganicLever safety pattern and extends it to `ose-app-web`, while keeping the simple sites
simple. _Judgment call:_ a staging gate for app clients reduces blast radius of a bad app build; no
incident baseline measured.

**Staging URL (private).** Each `-app-web` Vercel project also listens on its `stag-*-app-web` branch and
serves it as a persistent staging deployment at a dedicated staging URL, so the app can be exercised before
the gated promotion to prod. That staging URL is environment-private and **must never be committed** to the
repo (per [Secrets and Env Standards](../../../repo-governance/conventions/security/secrets-and-env-standards.md)):
every committed artifact — deployer agents, app READMEs, architecture docs, deploy workflows — refers to it
only via a placeholder (e.g. `<staging-url:ose-app-web>`) or a GitHub Actions secret (e.g.
`STAGING_BASE_URL_OSE_APP_WEB`). The FE E2E gate that promotes `stag-*-app-web → prod-*-app-web` reads the
staging base URL from that secret, not from a literal in the workflow file.

### D2 — Repoint by add-then-verify-then-delete (zero-downtime ordering)

For each renamed site: create the new `prod-*-www` branch, set the Vercel project's production branch to
it, deploy, verify the domain is 200 from the new build, and only **then** delete the old `prod-*-web`
branch. The old branch is the rollback handle until the new one is proven.

### D3 — OrganicLever project reuse vs new project

`organiclever-www` **reuses** today's OrganicLever marketing Vercel project (repoint its production
branch + root directory to `apps/organiclever-www`). `organiclever-app-web` is a **brand-new** project.
This matches the restructure plan's "Reuse www project for marketing; new app project + DNS" decision
`[Repo-grounded: restructure README row 10]`.

### D4 — Backends excluded

`organiclever-be` and `ose-be` are deployed by the ose-infra k3s plans as GHCR images. They never get a
Vercel project. This plan's verification explicitly asserts their **absence** from Vercel scope.

## Per-project mechanics

| Project              | Vercel action                                       | Production branch           | Root directory              | Domain               |
| -------------------- | --------------------------------------------------- | --------------------------- | --------------------------- | -------------------- |
| ose-www              | Repoint prod branch + rename + root dir             | `prod-ose-www`              | `apps/ose-www`              | www.oseplatform.com  |
| ayokoding-www        | Repoint prod branch + rename + root dir             | `prod-ayokoding-www`        | `apps/ayokoding-www`        | www.ayokoding.com    |
| organiclever-www     | Reuse OL marketing project; repoint + rename + root | `prod-organiclever-www`     | `apps/organiclever-www`     | www.organiclever.com |
| wahidyankf-www       | Repoint prod branch + rename + root dir             | `prod-wahidyankf-www`       | `apps/wahidyankf-www`       | www.wahidyankf.com   |
| organiclever-app-web | **Create new project** + DNS                        | `prod-organiclever-app-web` | `apps/organiclever-app-web` | app.organiclever.com |
| ose-app-web          | **Create new project** + DNS                        | `prod-ose-app-web`          | `apps/ose-app-web`          | app.oseplatform.com  |

## Related GitHub Actions workflows (complete inventory)

`[Repo-grounded: .github/workflows/*.yml as of the .github CI cleanup]`

Every deploy/staging/promotion workflow that names a cutover branch, environment, or app is listed
below with its **current** references and the **cutover action**. Workflows that touch no cutover
branch/app are listed under "Out of scope" so the inventory is provably complete.

### www tier — direct deploy (thin callers of `_reusable-test-and-deploy.yml`)

| Workflow file                          | Current refs                                        | Cutover action                                                                            |
| -------------------------------------- | --------------------------------------------------- | ----------------------------------------------------------------------------------------- |
| `_reusable-test-and-deploy.yml`        | force-pushes to `inputs.prod-branch` (no hardcoded) | **No change** — generic; callers pass the new inputs                                      |
| `test-and-deploy-ose-web.yml`          | `app-name: ose-web`, `prod-branch: prod-ose-web`    | **Update inputs** → `app-name: ose-www`, `prod-branch: prod-ose-www`                      |
| `test-and-deploy-ayokoding-web.yml`    | `ayokoding-web` / `prod-ayokoding-web`              | **Update inputs** → `ayokoding-www` / `prod-ayokoding-www`                                |
| `test-and-deploy-wahidyankf-web.yml`   | `wahidyankf-web` / `prod-wahidyankf-web`            | **Update inputs** → `wahidyankf-www` / `prod-wahidyankf-www`                              |
| `test-and-deploy-organiclever-www.yml` | **does not exist**                                  | **Create** — model on the wahidyankf caller; `organiclever-www` / `prod-organiclever-www` |

> The three existing callers pass the **pre-restructure** `app-name`, so their nightly CRON currently
> runs `nx run ose-web:…` against a project that is now `ose-www` — i.e. they are already failing. The
> input update both fixes the build and repoints the deploy branch. `organiclever-www` (marketing) has
> **no** deploy workflow today because the old OrganicLever workflow was migrated to the app-web tier
> (see below), so a new caller is required.

### app-web tier — gated promotion (dev → staging → dispatch promotion)

`organiclever-app-web` — trio exists but still on **old** branch/env names → **update**:

| Workflow file                                      | Current refs                                                                                     | Cutover action                                                                                                           |
| -------------------------------------------------- | ------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------ |
| `test-and-deploy-organiclever-web-development.yml` | tests `organiclever-app-web`; pushes `stag-organiclever-web`; env `organiclever-web-development` | **Update** staging branch → `stag-organiclever-app-web`; env → `organiclever-app-web-development`                        |
| `test-organiclever-web-staging.yml`                | env `organiclever-web-staging`; `stag-organiclever-web` (comments)                               | **Update** env → `organiclever-app-web-staging`; branch refs → `stag-organiclever-app-web`                               |
| `deploy-organiclever-web-to-production.yml`        | `stag-organiclever-web → prod-organiclever-web`; envs `organiclever-web-{staging,production}`    | **Update** → `stag-organiclever-app-web → prod-organiclever-app-web`; envs → `organiclever-app-web-{staging,production}` |

`ose-app-web` — trio already provisioned with the **target** names → **verify only**:

| Workflow file                                 | Current refs                                                                   | Cutover action                        |
| --------------------------------------------- | ------------------------------------------------------------------------------ | ------------------------------------- |
| `test-and-deploy-ose-app-web-development.yml` | pushes `stag-ose-app-web`; env `ose-app-web-development`                       | **Verify** — already correct, no edit |
| `test-ose-app-web-staging.yml`                | `stag-ose-app-web`; env `ose-app-web-staging`                                  | **Verify** — already correct, no edit |
| `deploy-ose-app-web-to-production.yml`        | `stag-ose-app-web → prod-ose-app-web`; envs `ose-app-web-{staging,production}` | **Verify** — already correct, no edit |

### GitHub Actions Environments (HUMAN, dashboard)

The staging/promotion jobs run under named **GitHub Actions Environments** that hold the deploy
secrets — `STAGING_BASE_URL_ORGANICLEVER_APP_WEB`, `STAGING_BASE_URL_OSE_APP_WEB`, and the staging
`WEB_BASE_URL` var. Renaming the OrganicLever environments to the `organiclever-app-web-*` form
requires creating those environments in **repo Settings → Environments** and setting their secrets
there (never in the repo). `ose-app-web-*` environments already match the workflow references.

### Out of scope (no cutover branch/app)

`publish-images.yml` (organiclever-be / ose-be GHCR images — owned by the ose-infra k3s plans),
`pr-quality-gate.yml`, `validate-markdown.yml`, `validate-env.yml`, and
`test-crane-cli-integration.yml` reference no cutover branch, environment, or web app.

## File-impact analysis (in-repo `[AI]` edits)

- `apps/ose-www/vercel.json`, `apps/ayokoding-www/vercel.json`, `apps/wahidyankf-www/vercel.json` —
  `ignoreCommand` branch string `prod-*-web` → `prod-*-www`. (`organiclever-www` and the two app-web
  apps need a `vercel.json` created if the restructure did not carry one; model on
  `apps/wahidyankf-www/vercel.json`.)
- `.claude/agents/apps-ose-web-deployer.md` → rename to `apps-ose-www-deployer.md`; update `name`,
  `description`, and the push target `prod-ose-web` → `prod-ose-www`. Same for `apps-ayokoding-web-deployer`,
  `apps-organiclever-web-deployer`, `apps-wahidyankf-web-deployer`. Add new `apps-ose-app-web-deployer`
  (model on an existing deployer). Run `npm run generate:bindings` to resync `.opencode/agents/`.
- `.github/workflows/` — every cutover-related workflow; see the
  [complete inventory](#related-github-actions-workflows-complete-inventory) above for each file's
  current refs and its update / create / verify action (4 www callers incl. a new `organiclever-www`
  one, the OrganicLever app-web trio to rename, and the already-correct ose-app-web trio to verify).
- `AGENTS.md` — the prod-branch list (lines ~231–234) and the per-site "Production branch" rows
  (~459, 471, 483, 495, 507).
- `apps/ose-www/README.md`, `apps/ayokoding-www/README.md`, `apps/wahidyankf-www/README.md`,
  `apps/organiclever-www/README.md`, and the two app-web READMEs — deploy-branch references.
- `docs/reference/system-architecture/applications.md`, `ci-cd.md`, `deployment.md` — branch names,
  the deployment mermaid nodes, and the workflow tables.

> **Boundary with the restructure plan:** the restructure plan renames the app **directories** and
> updates prose that names the **apps**; this plan owns every reference to the **deploy branch names**
> and the **Vercel/workflow wiring**. If the restructure already renamed a branch reference, this plan
> verifies it; the two must not double-edit the same line — Phase 0 diffs against `main` to confirm the
> starting state.

## Dependencies

- **Hard upstream dependency:** `restructure-fsharp-be-and-web-app-tiers` merged to `main`. Phase 0
  gate verifies the renamed app directories exist before any wiring edit.
- **Credentials (operator-held, never committed):** Vercel account access; DNS provider access for
  `organiclever.com` and `oseplatform.com`.

## Rollback

- **In-repo edits:** revert the wiring commit; branch names return to `prod-*-web`.
- **Vercel project repoint:** set the project's production branch back to `prod-*-web` (the old branch
  is retained until D2's verify step passes, so it is always available as the rollback target).
- **New app-web projects:** if a new project misbehaves, unpoint DNS and pause the project; no existing
  site is affected because the app-web tier is additive.
- **Branch deletion is the point of no easy return** — it is the **last** step and is gated on all six
  domains verified green.
