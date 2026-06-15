# In-Progress Plans

Active project plans currently being worked on.

## Active Plans

| Plan                                                                   | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| ---------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [wire-vercel-www-app-cutover](./wire-vercel-www-app-cutover/README.md) | Downstream Vercel prod cutover deferred by restructure-fsharp-be-and-web-app-tiers. **Assumes standardize-github-actions-pipeline-naming is done** (edits no workflow). Rewires four renamed public-website projects to new prod-\*-www branches (ose-www, ayokoding-www, organiclever-www, wahidyankf-www), creates two new Vercel projects for the app-web tier (organiclever-app-web at app.organiclever.com, ose-app-web at app.oseplatform.com), creates the prod/stag-\*-app-web + stag-\*-be branches, retires the old prod-\*-web branches, and **populates env/secret values per the env-injection.yaml manifest** — including enabling Vercel Protection Bypass for Automation and setting `VERCEL_AUTOMATION_BYPASS_SECRET` so staging E2E doesn't 401. Updates every in-repo wiring artifact. |
| [rename-be-e2e-api-base-url](./rename-be-e2e-api-base-url/README.md)   | Renames the backend (F#/Giraffe) E2E Playwright base-URL env var `BASE_URL` → `API_BASE_URL` in `ose-be-e2e` + `organiclever-be-e2e` and their single CI setter, mirroring the app-web FE suites' `WEB_BASE_URL`. Keeps localhost fallbacks (behavior-neutral), updates both READMEs, and records `API_BASE_URL` in `env-injection.yaml`. Leaves the three `www-be-e2e` suites on `BASE_URL`. Phase 2 (running BE E2E against a deployed staging backend URL that consumes the `*-app-staging` `API_BASE_URL` variable) is **DEFERRED** — blocked on ose-infra exposing a reachable staging backend URL.                                                                                                                                                                                                  |

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
