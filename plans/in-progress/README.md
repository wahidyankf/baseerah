# In-Progress Plans

Active project plans currently being worked on.

## Active Plans

| Plan                                                                                                 | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| ---------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [standardize-github-actions-pipeline-naming](./standardize-github-actions-pipeline-naming/README.md) | Establishes a domain-first `{domain}-{action-chain}.yml` naming convention for everything under `.github/workflows/` + `.github/actions/`, restructures the deploy pipelines into a direct www tier and a gated app tier (test-local → deploy-stag, then test-stag gate with prod CD deferred), repoints the stale post-restructure www callers, renames cross-cutting workflows to `commons-*`/`markdown-*`, deletes the crane-cli integration workflow (service-only scope), keeps `test:integration`/`test:e2e` out of the PR gate + pre-commit + pre-push, and adds a tiered env/secret **injection** standard (one `.env.example` key set injected into GitHub Environments, Vercel targets, and k3s/coralpolyp across local/staging/prod) with a value-less `env-injection.yaml` manifest. Sweeps all related governance `.md`/rules. Prerequisite to wire-vercel — owns its entire `.github/workflows` editing scope. |
| [wire-vercel-www-app-cutover](./wire-vercel-www-app-cutover/README.md)                               | Downstream Vercel prod cutover deferred by restructure-fsharp-be-and-web-app-tiers. **Assumes standardize-github-actions-pipeline-naming is done** (edits no workflow). Rewires four renamed public-website projects to new prod-\*-www branches (ose-www, ayokoding-www, organiclever-www, wahidyankf-www), creates two new Vercel projects for the app-web tier (organiclever-app-web at app.organiclever.com, ose-app-web at app.oseplatform.com), creates the prod/stag-\*-app-web + stag-\*-be branches, retires the old prod-\*-web branches, and **populates env/secret values per the env-injection.yaml manifest** — including enabling Vercel Protection Bypass for Automation and setting `VERCEL_AUTOMATION_BYPASS_SECRET` so staging E2E doesn't 401. Updates every in-repo wiring artifact.                                                                                                                    |

## Instructions

**Quick Idea Capture**: For 1-3 liner ideas not ready for formal planning, use `../ideas.md`.

**Naming**: Plans in `in-progress/` use NO date prefix — just the slug (e.g., `organiclever-web-responsive-breakpoints/`). Strip the date prefix when moving from `backlog/`.

When starting work on a plan:

1. Move and rename the plan folder: `git mv backlog/YYYY-MM-DD__[identifier]/ in-progress/[identifier]/` (strip the date prefix)
2. Update the plan's README.md status to "In Progress"
3. Add the plan to this list

When completing a plan:

1. Rename and move: `git mv in-progress/[identifier]/ done/YYYY-MM-DD__[identifier]/` using today's completion date
2. Update this list
