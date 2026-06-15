---
title: "PRD — Standardize Backend E2E Base-URL Env Var to API_BASE_URL"
description: Product scope, user stories, and Gherkin acceptance criteria for the BASE_URL→API_BASE_URL rename
---

# PRD — Backend E2E `API_BASE_URL` Standardization

## Personas

- **Platform engineer** — runs and maintains the CI E2E pipelines; wants symmetric, self-describing
  env-var names so the staging gate is readable.
- **Backend developer** — runs `nx run ose-be-e2e:test:e2e` locally; must not have their workflow broken
  by the rename.
- **Release operator** — sets GitHub Environment variables; wants every variable they set to have a
  defined reader and purpose.

## User stories

1. As a platform engineer, I want the backend E2E suites to read `API_BASE_URL` so it mirrors the
   frontend `WEB_BASE_URL` and the two halves of a staging promotion read consistently.
2. As a backend developer, I want the localhost fallback preserved so my local E2E runs need no env setup.
3. As a release operator, I want `API_BASE_URL` recorded in the env-injection manifest so the variable I
   created has a documented home and meaning.

## Functional requirements

- **FR1** — `apps/ose-be-e2e/playwright.config.ts` reads `process.env.API_BASE_URL`, falling back to
  `http://localhost:8302`.
- **FR2** — `apps/organiclever-be-e2e/playwright.config.ts` reads `process.env.API_BASE_URL`, falling back
  to `http://localhost:8202`.
- **FR3** — `.github/workflows/_reusable-app-test-local-deploy-stag.yml` injects `API_BASE_URL` (not
  `BASE_URL`) into the "Run BE E2E tests" step.
- **FR4** — Both backend E2E READMEs document `API_BASE_URL`.
- **FR5** — `env-injection.yaml` records `API_BASE_URL` in the `ci-harness` section (names only).
- **FR6** — The three `www-be-e2e` suites remain on `BASE_URL` (unchanged).

## Non-functional requirements

- **NFR1** — Behavior-neutral for local and local-CI runs (fallbacks preserved; setter+readers renamed in
  lockstep).
- **NFR2** — No secret or URL value committed to any tracked file.
- **NFR3** — `rhino-cli env validate`, `actionlint`, markdown lint, and links validation all pass.

## Acceptance criteria (Gherkin)

```gherkin
Feature: Backend E2E base URL uses the API_BASE_URL convention

  Scenario: ose-be-e2e reads API_BASE_URL with a localhost fallback
    Given the file apps/ose-be-e2e/playwright.config.ts
    When I inspect the Playwright use.baseURL expression
    Then it reads process.env.API_BASE_URL
    And it falls back to "http://localhost:8302"
    And it does not reference process.env.BASE_URL

  Scenario: organiclever-be-e2e reads API_BASE_URL with a localhost fallback
    Given the file apps/organiclever-be-e2e/playwright.config.ts
    When I inspect the Playwright use.baseURL expression
    Then it reads process.env.API_BASE_URL
    And it falls back to "http://localhost:8202"
    And it does not reference process.env.BASE_URL

  Scenario: the CI setter injects API_BASE_URL for the backend E2E step
    Given the workflow .github/workflows/_reusable-app-test-local-deploy-stag.yml
    When I inspect the "Run BE E2E tests" step env block
    Then it sets API_BASE_URL to the local backend URL
    And it does not set BASE_URL

  Scenario: backend E2E suites pass against docker-compose after the rename
    Given the local docker-compose backend is healthy
    When I run nx run ose-be-e2e:test:e2e with API_BASE_URL set to the compose backend URL
    Then the suite passes

  Scenario: www-be-e2e suites are not modified
    Given the files apps/ose-www-be-e2e/playwright.config.ts and apps/ayokoding-www-be-e2e/playwright.config.ts and apps/organiclever-www-be-e2e/playwright.config.ts
    When I inspect their use.baseURL expressions
    Then each still reads process.env.BASE_URL

  Scenario: the env-injection manifest records API_BASE_URL
    Given env-injection.yaml
    When I inspect the ci-harness section
    Then API_BASE_URL is listed as a var
    And rhino-cli env validate passes
```
