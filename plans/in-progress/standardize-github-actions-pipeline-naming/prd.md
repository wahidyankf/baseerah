# Product Requirements Document

## Personas

- **Maintainer (Wahidyan)** — owns the CI surface; wants filenames that read as pipelines and correct
  wiring so the Vercel cutover can proceed.
- **Contributor** — adds or debugs a pipeline; wants to find the right file by name and copy a thin
  caller for a new site.
- **Release operator** — runs the scheduled/dispatched pipelines; wants each tier to do exactly what its
  name says (www → prod directly; app → staging, then a staging gate that stops short of prod).

## User stories

1. As a maintainer, I want every workflow filename to read `{domain}-{action-chain}` so I can identify
   any pipeline at a glance.
2. As a contributor, I want cross-cutting workflows prefixed by domain (`commons-`, `markdown-`,
   `crane-cli-`) so non-app automation is grouped and discoverable.
3. As a maintainer, I want the four www sites to share one reusable direct-deploy pipeline so adding a
   site is a ~15-line caller.
4. As a release operator, I want the app tier split into `test-local-deploy-stag` and
   `test-stag-deploy-prod` so staging is verified against the real deployment before any future prod CD.
5. As a maintainer, I want the stale www callers repointed to their `-www` projects and `prod-*-www`
   branches so they stop being no-ops.
6. As the wire-vercel executor, I want this plan to own all workflow edits so my plan only creates
   Vercel projects, DNS, Environments, and branches.
7. As a maintainer, I want one tiered env/secret injection standard so a key declared once in
   `apps/<app>/.env.example` injects the same way into GitHub Actions, Vercel, and k3s at each stage,
   and a value-less manifest tells wire-vercel exactly which values to set where.

## Acceptance criteria (Gherkin)

```gherkin
Feature: Domain-first workflow naming

  Scenario: Every workflow filename follows the grammar
    Given the .github/workflows directory after this plan
    When I list every "*.yml" file
    Then each non-reusable filename matches "{domain}-{action-chain}.yml"
    And each reusable filename matches "_reusable-{domain}-{action-chain}.yml"
    And the name: field of each workflow derives to its filename by the kebab-case rule

  Scenario: Cross-cutting workflows carry a domain keyword prefix
    Given the renamed cross-cutting workflows
    When I inspect the .github/workflows directory
    Then "pr-quality-gate.yml" is "commons-quality-gate.yml"
    And "validate-env.yml" is "commons-env-validate.yml"
    And "validate-markdown.yml" is "markdown-validate.yml"
    And "test-crane-cli-integration.yml" is deleted (CLI CI out of scope this PR)
    And "publish-images.yml" is absorbed into "organiclever-be-build-deploy-stag.yml" and "ose-be-build-deploy-stag.yml"
```

```gherkin
Feature: www tier direct deploy

  Scenario: Four www sites share the reusable direct-deploy pipeline
    Given the www tier after this plan
    When I inspect each www workflow caller
    Then "ose-www-test-local-deploy-prod.yml" calls "_reusable-www-test-local-deploy.yml" with app-name "ose-www" and prod-branch "prod-ose-www"
    And "ayokoding-www-test-local-deploy-prod.yml" targets "prod-ayokoding-www"
    And "organiclever-www-test-local-deploy-prod.yml" exists and targets "prod-organiclever-www"
    And "wahidyankf-www-test-local-deploy-prod.yml" targets "prod-wahidyankf-www"
    And no workflow references the projects "ose-web", "ayokoding-web", or "wahidyankf-web"

  Scenario: organiclever-www gains a local test stack
    Given organiclever-www had no infra/dev stack
    When I inspect the infra/dev directory
    Then "infra/dev/organiclever-www/docker-compose.yml" exists
    And the organiclever-www pipeline runs its e2e against that stack
```

```gherkin
Feature: app tier gated promotion

  Scenario: App groups test locally then deploy to staging
    Given the app tier after this plan
    When I inspect each app workflow caller
    Then "organiclever-app-test-local-deploy-stag.yml" runs be+fe integration and e2e via docker-compose
    And on success it deploys to "stag-organiclever-app-web"
    And "ose-app-test-local-deploy-stag.yml" deploys to "stag-ose-app-web"

  Scenario: Staging gate stops short of production
    Given "organiclever-app-test-stag-deploy-prod.yml"
    When its staging e2e passes
    Then it does not push any prod branch
    And the dispatch-only "deploy-organiclever-web-to-production.yml" no longer exists
```

```gherkin
Feature: Unblock the Vercel cutover

  Scenario: wire-vercel no longer edits workflows
    Given the wire-vercel-www-app-cutover plan after this plan
    When I inspect its scope and tech-docs sections
    Then its scope lists no ".github/workflows" editing
    And it points its workflow section at standardize-github-actions-pipeline-naming
```

```gherkin
Feature: Repo gates stay green

  Scenario: Lint and links pass after the rename
    When I run actionlint over .github/workflows
    Then it reports no errors
    And when I run "npx nx run rhino-cli:links:validation" no link resolves to an old workflow filename
```

```gherkin
Feature: Heavy tests stay off the fast feedback path

  Scenario: The PR gate runs no integration or e2e
    Given "commons-quality-gate.yml" after this plan
    When I inspect its job definitions
    Then it invokes no "test:integration" target
    And it invokes no "test:e2e" target
    And it runs only typecheck, lint, test:quick, specs:coverage, and the lint/validation jobs

  Scenario: The git hooks run no integration or e2e
    Given ".husky/pre-commit" and ".husky/pre-push"
    When I inspect their contents
    Then neither invokes "test:integration"
    And neither invokes "test:e2e"

  Scenario: The crane-cli integration workflow is removed
    Given the .github/workflows directory after this plan
    When I list workflow files
    Then "test-crane-cli-integration.yml" no longer exists
    And no workflow runs "crane-cli:test:integration" on "pull_request"

  Scenario: Integration and e2e live in the scheduled service pipelines
    Given the tiered pipelines after this plan
    When I inspect each pipeline's job invocations
    Then "test:integration" and "test:e2e" run only in "*-test-local-*" and "*-test-stag-*"
```

```gherkin
Feature: Tiered env/secret injection

  Scenario: One canonical key set per app injects across platforms
    Given the env injection standard after this plan
    When I inspect the injection matrix and .env.example files
    Then every app-runtime key originates from "apps/<app>/.env.example"
    And no key carries a tier qualifier in its name
    And the same key name is used in the GitHub Environment, the Vercel target, and the k3s secret

  Scenario: CI test-harness keys are registered, not app config
    Given "WEB_BASE_URL" and "VERCEL_AUTOMATION_BYPASS_SECRET"
    When I inspect env-injection.yaml and all apps/.env.example files
    Then they appear in "env-injection.yaml" under "ci-harness"
    And they appear in no "apps/<app>/.env.example"
    And they are bound to the "{group}-app-staging" GitHub Environment

  Scenario: The injection manifest is value-less and statically checked
    Given "env-injection.yaml" at repo root
    And it lists injection homes per app per stage by key name only, never values
    When I run the env injection consistency check
    Then every app-runtime key has a documented home at each stage the app runs
    And the check reads no real secret value

  Scenario: This plan sets no real values
    Given the repo after this plan
    When I inspect all committed files for secret values
    Then no GitHub Environment secret value, Vercel env value, or k3s secret value is created here
    And populating them is left to wire-vercel-www-app-cutover and ose-infra coralpolyp
```

## Product scope

### In scope

- Rename all 15 existing `.github/workflows/*.yml` files to follow the `{domain}-{action-chain}.yml`
  grammar.
- Create 4 new reusable workflows (`_reusable-www-test-local-deploy.yml`,
  `_reusable-app-test-local-deploy-stag.yml`, `_reusable-app-test-stag.yml`,
  `_reusable-be-build-deploy.yml`).
- Create 4 new www caller workflows and split `organiclever-www-e2e` into `be-e2e` + `fe-e2e`.
- Create `infra/dev/organiclever-www/` docker-compose stack.
- Define the tiered env/secret injection standard and the value-less `env-injection.yaml` manifest.
- Extend `rhino-cli env validate` with the manifest-consistency pass.
- Update all governance docs, CI convention docs, and agent definitions to reference the new names.
- Reduce `wire-vercel-www-app-cutover` scope to exclude workflow editing.

### Out of scope

- Standing up Vercel projects, DNS, GitHub Environments, or branches (wire-vercel plan).
- Setting any real env/secret values in GitHub, Vercel, or k3s.
- Production continuous delivery for the app tier (separate plan).
- Backend k3s rollout (ose-infra).
- Changing any test's logic or coverage thresholds.
- CLI-tool CI (crane-cli integration test — deferred).

## Product risks

| Risk                                                                  | Likelihood | Product impact | Mitigation                                                                                   |
| --------------------------------------------------------------------- | ---------- | -------------- | -------------------------------------------------------------------------------------------- |
| Naming-convention adoption friction — contributors use old patterns   | Low        | Medium         | Convention doc codifies the grammar; `rhino-cli:naming:workflows-validation` enforces it     |
| Executor confusion about the two-tier model (www direct vs app gated) | Low        | Low            | `tech-docs.md` per-tier mechanics section explains the distinction clearly                   |
| The `When` step in `env validate` extension changes its CLI contract  | Low        | Medium         | No new Nx target; `env:validation` target signature unchanged; TDD fixture covers regression |
| Wire-vercel executor misses the reduced scope — still edits workflows | Low        | Medium         | This plan explicitly edits wire-vercel plan docs (Phase 7.4) to remove workflow items        |

## Validation commands

```bash
actionlint .github/workflows/*.yml
npx nx run rhino-cli:links:validation
npx nx run rhino-cli:headings:hierarchy-validation
npm run lint:md
git grep -nE 'test-and-deploy-(ose|ayokoding|wahidyankf)-web|prod-(ose|ayokoding|wahidyankf)-web|stag-organiclever-web|pr-quality-gate\.yml|validate-markdown\.yml' -- ':!plans/done/**'
# env injection: manifest present, value-less, and statically consistent
test -f env-injection.yaml
npx nx run rhino-cli:env:validation        # extended with the manifest-consistency pass
git grep -nE 'WEB_BASE_URL|VERCEL_AUTOMATION_BYPASS_SECRET' -- 'apps/*/.env.example'  # expect: no hits
```
