---
title: "BRD — Standardize Backend E2E Base-URL Env Var to API_BASE_URL"
description: Business rationale, impact, and risks for renaming the backend E2E base-URL variable
---

# BRD — Backend E2E `API_BASE_URL` Standardization

## Problem statement

The frontend (app-web) Playwright suites read their target from `WEB_BASE_URL`; the backend (F#/Giraffe)
Playwright suites read the generic `BASE_URL`. A full-stack staging promotion therefore references two
differently-named base-URL variables for what is conceptually one symmetric pair (web origin / API
origin). During `wire-vercel-www-app-cutover` the operator pre-created an `API_BASE_URL` GitHub
Environment variable expecting the backend E2E suites to consume it — but nothing reads that name today,
so the variable is inert and the intent is undocumented.

## Goal

Establish a single, symmetric naming convention for E2E base URLs:

- `WEB_BASE_URL` — frontend/web origin (already in place).
- `API_BASE_URL` — backend API origin (this plan).

Make the convention real for the two product-backend E2E suites, so the operator-created environment
variable has a defined meaning and a reader, and so future staging backend E2E wiring has an obvious name
to inject.

## Business-scope Non-Goals

- No behavior change for any E2E suite — localhost fallbacks are preserved; the rename is purely cosmetic
  at the variable-name level.
- No staging backend E2E gate wired in this plan — the deferred Phase 2 (consuming `API_BASE_URL` from a
  deployed staging backend) is blocked on ose-infra and is not a business deliverable of this plan.
- No renaming of the `www-be-e2e` suites (`ose-www-be-e2e`, `ayokoding-www-be-e2e`,
  `organiclever-www-be-e2e`) — they target the web server origin, not a backend API service.

## Impact

- **Developer experience**: consistent, self-describing env-var names across FE/BE E2E. Lower cognitive
  load when reading or writing the staging gate.
- **CI**: behavior-neutral. The single setter and the two readers are renamed in lockstep; localhost
  fallbacks are preserved, so local and local-CI runs are unchanged.
- **Operability**: the `API_BASE_URL` GitHub Environment variable gains a documented purpose (and a
  defined future reader), removing a dangling, unexplained config entry.

## Affected Roles

- **Platform engineer hat** — maintains the CI E2E pipelines; benefits from symmetric, self-describing
  env-var names (`WEB_BASE_URL` / `API_BASE_URL`) when reading or authoring the staging gate.
- **Backend developer hat** — runs `nx run ose-be-e2e:test:e2e` or `nx run organiclever-be-e2e:test:e2e`
  locally; must not have their workflow broken by the rename (localhost fallbacks preserved).
- **Release operator hat** — sets GitHub Environment variables; gains a documented reader for the
  `API_BASE_URL` variable they created during `wire-vercel-www-app-cutover`.
- **Consuming agent**: `swe-e2e-dev` — the suggested executor for the config and workflow edits in Phase 1.

## Risks and mitigations

| Risk                                                                                                           | Likelihood | Impact | Mitigation                                                                                                                       |
| -------------------------------------------------------------------------------------------------------------- | ---------- | ------ | -------------------------------------------------------------------------------------------------------------------------------- |
| Read site renamed but setter not — CI backend E2E loses its URL and falls back to localhost (wrong host in CI) | Low        | High   | Rename setter and both readers in the **same commit**; Phase 1 gate re-runs both suites and `actionlint`.                        |
| Accidentally renaming the `www-be-e2e` suites that legitimately use `BASE_URL`                                 | Low        | Medium | Scope explicitly excludes them; tech-docs lists the exact five `BASE_URL` sites and which two change.                            |
| Implying the staging `API_BASE_URL` variable is consumed when it is not                                        | Medium     | Low    | Phase 2 is marked DEFERRED; env-injection comment states local-CI inline injection today, staging consumption pending.           |
| Env-drift guard (`rhino-cli env validate`) flags the manifest change                                           | Low        | Medium | `API_BASE_URL` is a CI-harness key (not in any `apps/<app>/.env.example`), modeled exactly on the existing `WEB_BASE_URL` entry. |

## Success criteria

- Both backend E2E suites read `API_BASE_URL` (with localhost fallback) and pass against docker-compose.
- The single CI setter injects `API_BASE_URL`; `actionlint` is clean.
- No `BASE_URL` read remains in the two in-scope suites; the three `www-be-e2e` suites are untouched.
- `env-injection.yaml` records `API_BASE_URL`; `rhino-cli env validate` passes.
- No secret or URL value is committed.
