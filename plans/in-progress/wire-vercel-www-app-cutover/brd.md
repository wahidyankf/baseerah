---
title: "BRD — Wire the www + app-web Tiers to the Vercel Pipeline"
description: Business rationale for the deferred Vercel production cutover of the renamed www tier and the new app-web tier
---

# Business Requirements — Vercel www + app-web Cutover

## Business goal

Make the post-restructure production topology **real**. After
[`restructure-fsharp-be-and-web-app-tiers`](../../done/2026-06-14__restructure-fsharp-be-and-web-app-tiers/README.md)
merges, the repository describes a clean three-tier model (`-www` public sites, `-app-web` app clients,
`-be` backends) but production still runs on the old `prod-*-web` branches and old Vercel projects, and
the app-web tier has no production presence at all. This plan performs the deferred cutover so the live
deployments match the documented architecture and the new app-web apps can ship.

## Business impact

**Pain points addressed:**

- **Documentation/production drift.** The moment the restructure lands, every prod-branch reference in
  the repo (deployer agents, workflows, READMEs, architecture docs) points at a branch whose name no
  longer matches the app. Operators and agents can no longer trust the docs to deploy correctly.
- **App-web tier is undeployable.** `organiclever-app-web` and `ose-app-web` have no Vercel project and
  no domain. The product split delivers no user value until they are wired and reachable.
- **Stale branches accumulate risk.** Leaving `prod-*-web` branches live alongside new `prod-*-www`
  branches invites accidental double-deploys and confusion about the source of truth.

**Expected benefits:**

- Each public site deploys from a correctly named `prod-*-www` branch; each app client deploys from a
  `prod-*-app-web` branch behind a staging gate.
- A single, enumerated, in-repo source of truth for "which branch feeds which domain."
- The marketing/app split is observable to users at distinct domains (root domain vs `app.*`).
- Every env/secret value lives in its standardized injection home (GitHub Environment, Vercel target)
  per the `env-injection.yaml` manifest — including the **Vercel Protection Bypass** token the staging
  E2E gate needs, so staging tests authenticate instead of 401-ing.

## Affected roles

- **Maintainer (deployer hat):** runs the cutover; performs the Vercel dashboard + DNS steps that
  require real credentials.
- **Maintainer (operator hat):** uses the updated deployer agents and workflows day-to-day after cutover.
- **AI deployer agents** (`apps-ose-web-deployer` and siblings): consume the updated branch names; will
  be renamed/repointed by this plan.
- **AI execution agent:** performs all in-repo `[AI]` wiring edits and branch creation.

## Business-level success metrics

1. **Observable fact — every documented domain serves from its new branch.** After cutover, each of the
   six production domains responds 200 and is built from its `prod-*-www` / `prod-*-app-web` branch
   (verified by a deploy + `curl` per domain, recorded in `delivery.md` Phase gates).
2. **Observable fact — zero stale branch references in repo.** `rg 'prod-(ose|ayokoding|organiclever|wahidyankf)-web\b'`
   over `apps/`, `.claude/`, `.github/`, `AGENTS.md`, and `docs/` returns no matches outside
   `plans/done/` after the plan completes (the retired-branch names survive only in history).
3. **Observable fact — the two new app-web projects exist and resolve.** `app.organiclever.com` and
   `app.oseplatform.com` resolve and serve their respective app builds.
4. _Judgment call:_ operator confidence that "push to deploy" matches the docs improves; no baseline
   measured.

## Business-scope non-goals

- Not changing the **deploy mechanism** (force-push-to-prod-branch + Vercel auto-build, and the gated
  staging→prod promotion for the app tier) — only the **branch names, project wiring, and domains**.
- Not migrating backends to or from Vercel — backends stay on self-hosted k8s (ose-infra).
- Not introducing a new CI provider, CDN, or hosting platform.

## Business risks and mitigations

| Risk                                                                                              | Likelihood | Impact | Mitigation                                                                                                                                                                      |
| ------------------------------------------------------------------------------------------------- | ---------- | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Cutover executed before the restructure rename lands → wiring points at apps that don't exist yet | Medium     | High   | Phase 0 gate hard-stops on a check that the renamed app directories exist in `main` before any wiring edit. This plan is explicitly **downstream** of the restructure.          |
| Production downtime during project repoint (Vercel serves old build or 404 mid-switch)            | Low        | High   | Repoint by **adding** the new prod branch and verifying its build green **before** deleting the old branch; keep old branch until the new domain is confirmed 200.              |
| DNS misconfiguration for the new `app.*` domains                                                  | Medium     | Medium | Treat DNS as an isolated `[HUMAN]` step with its own gate (`dig` / browser check) before declaring the app-web tier live.                                                       |
| Secrets/credentials needed for Vercel + DNS not available to an agent                             | High       | —      | All dashboard + DNS steps are tagged `[HUMAN]`; the agent prepares everything in-repo and hands off with explicit instructions. No real secret ever enters a committed file.    |
| `organiclever` gated-promotion pipeline breaks during the split (two pipelines from one)          | Medium     | Medium | tech-docs defines the new `stag-*-app-web → prod-*-app-web` promotion explicitly; the marketing site moves to the simple direct-deploy pattern. Verified per-pipeline in gates. |
| Staging E2E gate 401s because the Vercel Protection Bypass token is missing/wrong                 | Medium     | High   | Enabling Protection Bypass for Automation + setting `VERCEL_AUTOMATION_BYPASS_SECRET` is an explicit `[HUMAN]` step with its own Phase 3 gate (a 200-not-401 bypass check).     |
| Workflow/value scope collides with the standardize plan                                           | Low        | Medium | This plan assumes `standardize-github-actions-pipeline-naming` is **done**; it edits no workflow and only populates the values the manifest declares.                           |

## No secrets

This plan references Vercel and DNS operations but contains **no secret values**. Tokens, API keys, and
DNS credentials are referenced by name only and live in uncommitted files or the operator's dashboards,
per the [No Secrets in Git convention](../../../repo-governance/conventions/security/no-secrets-in-committed-files.md).
