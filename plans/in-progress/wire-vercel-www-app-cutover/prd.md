---
title: "PRD — Wire the www + app-web Tiers to the Vercel Pipeline"
description: Product requirements, personas, user stories, and Gherkin acceptance criteria for the Vercel prod cutover
---

# Product Requirements — Vercel www + app-web Cutover

## Product overview

Rewire the Vercel deployment pipeline so the renamed public-website tier deploys from `prod-*-www`
branches and the new app-web tier (`organiclever-app-web`, `ose-app-web`) gets its own Vercel projects,
domains, and staging-gated promotion. Update all in-repo wiring artifacts so the documented pipeline
matches the live one, and retire the obsolete `prod-*-web` branches.

## Personas

- **Deployer (maintainer hat):** performs the cutover; holds Vercel + DNS credentials; runs dashboard
  and DNS steps.
- **Operator (maintainer hat):** ships day-to-day after cutover via `git push` / deployer agents.
- **AI execution agent:** performs in-repo wiring edits, branch creation, and verification commands.
- **AI deployer agents** (`apps-*-deployer`): consume the new branch names post-cutover.

## User stories

1. **As the deployer**, I want each public site repointed to its `prod-*-www` branch **so that** the
   deploy branch name matches the renamed app and the docs stay trustworthy.
2. **As the deployer**, I want new Vercel projects + DNS for `organiclever-app-web` and `ose-app-web`
   **so that** the app-web tier is reachable at `app.organiclever.com` and `app.oseplatform.com`.
3. **As an operator**, I want one enumerated list of "branch → domain" wiring in the repo **so that** I
   never guess which branch deploys which site.
4. **As an operator**, I want the obsolete `prod-*-web` branches deleted after cutover **so that** there
   is no ambiguous or accidental deploy source.
5. **As an AI deployer agent**, I want my definition to reference the correct new branch **so that** a
   delegated deploy pushes to the right place.

## Acceptance criteria (Gherkin)

> Step-keyword cardinality: each scenario uses exactly one `Given`, one `When`, one `Then`; extra steps
> chain with `And`.

```gherkin
Feature: Public-website tier deploys from prod-*-www branches

  Background:
    Given the restructure-fsharp-be-and-web-app-tiers rename has landed on main
    And apps/ose-www, apps/ayokoding-www, apps/organiclever-www, apps/wahidyankf-www exist

  Scenario: ose-www serves from its new production branch
    Given the ose-www Vercel project's production branch is set to prod-ose-www
    When main is force-pushed to prod-ose-www
    Then www.oseplatform.com responds 200 from the new build
    And the build originates from the prod-ose-www branch

  Scenario: wahidyankf-www vercel.json gates on the new branch
    Given apps/wahidyankf-www/vercel.json exists
    When the ignoreCommand is read
    Then it compares VERCEL_GIT_COMMIT_REF against prod-wahidyankf-www
    And it no longer references prod-wahidyankf-web
```

```gherkin
Feature: App-web tier gets new Vercel projects and DNS

  Scenario: organiclever-app-web is reachable at its app subdomain
    Given a new Vercel project organiclever-app-web is created with root apps/organiclever-app-web
    When prod-organiclever-app-web is deployed and DNS for app.organiclever.com is pointed at Vercel
    Then app.organiclever.com responds 200 from the organiclever-app-web build
    And the project's production branch is prod-organiclever-app-web

  Scenario: ose-app-web is reachable at its app subdomain
    Given a new Vercel project ose-app-web is created with root apps/ose-app-web
    When prod-ose-app-web is deployed and DNS for app.oseplatform.com is pointed at Vercel
    Then app.oseplatform.com responds 200 from the ose-app-web build
    And the project's production branch is prod-ose-app-web
```

```gherkin
Feature: Obsolete branches and references are retired

  Scenario: No stale prod-*-web references remain in the repo
    Given the cutover wiring edits are committed
    When rg "prod-(ose|ayokoding|organiclever|wahidyankf)-web\b" runs over apps/ .claude/ .github/ AGENTS.md docs/
    Then it returns zero matches outside plans/done/
    And the new prod-*-www / prod-*-app-web branch names appear in their place

  Scenario: Old production branches are deleted after verification
    Given all six new production domains have been verified 200
    When the operator deletes prod-ose-web, prod-ayokoding-web, prod-organiclever-web, prod-wahidyankf-web, stag-organiclever-web
    Then git ls-remote --heads origin lists none of the retired branches
    And only the new prod-*-www, prod-*-app-web, and stag-*-app-web branches remain as deploy branches
```

```gherkin
Feature: Deployer agents reference the new branches

  # Target state — agent created in Phase 1b of this plan
  Scenario: ose-www deployer pushes to the renamed branch
    Given the apps-ose-www-deployer agent definition exists
    When its deployment workflow section is read
    Then it pushes main to prod-ose-www
    And npm run generate:bindings has resynced the .opencode mirror
```

## Product scope

### In scope (product-visible outcomes)

- Six production domains served from correctly named branches.
- Two new `app.*` domains live.
- One enumerated branch→domain mapping in `README.md` + architecture docs.
- Deployer agents and workflows that push to the right branches.

### Out of scope

- Backend reachability/domains (`api.*`) — ose-infra owns these.
- Any change to page content, routing, or features of the apps.
- Preview-deployment policy changes beyond the staging-gate branches named here.

## Product-level risks

- **Mid-cutover inconsistency:** a domain briefly served by both old and new branches. Mitigated by
  verify-new-before-deleting-old ordering (see tech-docs rollback).
- **Operator muscle memory:** force-pushing to a now-deleted `prod-*-web` branch. Mitigated by deleting
  old branches only after the deployer agents/docs are updated, so the documented command is correct.
