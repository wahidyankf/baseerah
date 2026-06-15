---
title: Standardize Backend E2E Base-URL Env Var to API_BASE_URL
description: Rename the Playwright base-URL environment variable from BASE_URL to API_BASE_URL in the two F#/Giraffe backend E2E suites (ose-be-e2e, organiclever-be-e2e) and their single CI setter, mirroring the WEB_BASE_URL convention the app-web FE E2E suites already use. Keeps the localhost fallbacks so local runs are unaffected, updates both READMEs, and records API_BASE_URL in the env-injection manifest. Documents — but defers to a later phase — making the app-staging GitHub Environment API_BASE_URL variable actually consumed by a staging backend E2E gate, which depends on a reachable staging backend URL owned by ose-infra.
---

# Standardize Backend E2E Base-URL Env Var to `API_BASE_URL`

> **Status**: In progress — authored 2026-06-15. Execution **not started** (this plan is created and
> parked; it is not executed in the same session that created it).
> **Related**: [`wire-vercel-www-app-cutover`](../../done/2026-06-15__wire-vercel-www-app-cutover/README.md) — during that
> plan's Phase 3 the operator added an `API_BASE_URL` variable to the `organiclever-app-staging` and
> `ose-app-staging` GitHub Environments (alongside the staging gate's `WEB_BASE_URL`). That variable is
> currently **read by nothing**: the backend E2E suites read `BASE_URL`, not `API_BASE_URL`, and they run
> against local docker-compose, not against a deployed staging backend. This plan closes the naming half
> of that gap (rename `BASE_URL` → `API_BASE_URL`) and explicitly scopes the consumption half (running
> backend E2E against a staging backend URL) to a deferred Phase 2 that depends on ose-infra.

## Context

The repository's FE (app-web) Playwright suites read their base URL from `process.env.WEB_BASE_URL`
(`apps/ose-app-web-e2e/playwright.config.ts`, `apps/organiclever-app-web-e2e/playwright.config.ts`). The
two backend (F#/Giraffe) Playwright suites instead read the generic `process.env.BASE_URL`
(`apps/ose-be-e2e/playwright.config.ts:22`, `apps/organiclever-be-e2e/playwright.config.ts:22`). The
asymmetry is cosmetic today but it (a) makes the two halves of a full-stack staging promotion read
differently-named variables, and (b) leaves the operator-created `API_BASE_URL` GitHub Environment
variables with no reader.

This plan renames the backend-E2E base-URL variable to `API_BASE_URL` so the pair reads
`WEB_BASE_URL` (frontend) / `API_BASE_URL` (backend API), updating every read site, the single CI setter
that injects it, both suite READMEs, and the `env-injection.yaml` manifest. The localhost fallback
defaults are preserved so `nx run <project>-e2e:test:e2e` still works locally with no env set.

## Scope

### In scope (Phase 1 — the rename)

- `apps/ose-be-e2e/playwright.config.ts` — `process.env.BASE_URL` → `process.env.API_BASE_URL` (keep
  `|| "http://localhost:8302"`).
- `apps/organiclever-be-e2e/playwright.config.ts` — same, keep `|| "http://localhost:8202"`.
- `.github/workflows/_reusable-app-test-local-deploy-stag.yml:161` — the "Run BE E2E tests" step's
  `env:` key `BASE_URL` → `API_BASE_URL` (the only place the variable is _set_ for these two suites).
- `apps/ose-be-e2e/README.md`, `apps/organiclever-be-e2e/README.md` — env-var table + prose.
- `env-injection.yaml` — record `API_BASE_URL` in the `ci-harness` section (names only, never values).

### Out of scope

- **`www-be-e2e` suites** (`ose-www-be-e2e`, `ayokoding-www-be-e2e`, `organiclever-www-be-e2e`) — these
  test the Next.js **web** server origin (ports 3100/3101/3200), not a separate API service, and run via a
  **different** reusable workflow. They keep `BASE_URL`. Renaming them is a separate decision, deliberately
  excluded here. See [tech-docs.md](./tech-docs.md).
- **Staging consumption (deferred — Phase 2)** — actually running the backend E2E suites against a
  deployed **staging backend URL** sourced from the `*-app-staging` GitHub Environment `API_BASE_URL`. The
  backends deploy to self-hosted k3s via ose-infra `coralpolyp`, not to a public Vercel URL; wiring a
  staging backend E2E gate requires a reachable staging backend URL that ose-infra owns. Captured as a
  Phase 2 dependency, not delivered by this plan.

## Approach summary

1. **Phase 0** ([AI]) — baseline: install, doctor, run both backend E2E suites green locally against
   docker-compose with the current `BASE_URL` to confirm a known-good starting point.
2. **Phase 1** ([AI]) — the rename: edit both configs + the CI setter + both READMEs + the manifest in one
   thematic commit; re-run both suites green to prove the new variable name is wired end-to-end.
3. **Phase 2** ([DEFERRED]) — documented only: staging backend E2E gate consuming `API_BASE_URL` from the
   GitHub Environment, blocked on an ose-infra-provided reachable staging backend URL.

## Companion files

- [brd.md](./brd.md) — business rationale, impact, risks
- [prd.md](./prd.md) — user stories + Gherkin acceptance criteria
- [tech-docs.md](./tech-docs.md) — exact edits, blast radius, scope-exclusion rationale
- [delivery.md](./delivery.md) — phased, executor-tagged, TDD-shaped delivery checklist with gates
