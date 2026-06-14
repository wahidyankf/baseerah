# GitHub Actions Workflows

CI/CD workflows for the monorepo. Filenames follow the
[GitHub Actions Workflow Naming Convention](../../repo-governance/development/infra/github-actions-workflow-naming.md);
the [CI Conventions](../../repo-governance/development/infra/ci-conventions.md) define the
reusable-workflow pattern and the twice-daily WIB CRON schedule.

## Reusable

| Workflow                        | Role                                                                                                                                                                        |
| ------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `_reusable-test-and-deploy.yml` | Full test pipeline (lint, typecheck, `test:quick`, E2E) then force-push to a prod branch. Called by the `www`-tier deploy workflows with `app-name` / `prod-branch` inputs. |

## PR and repo-wide gates

| Workflow                | Trigger             | Role                                                                                    |
| ----------------------- | ------------------- | --------------------------------------------------------------------------------------- |
| `pr-quality-gate.yml`   | Pull request        | Typecheck, lint, `test:quick`, `specs:coverage`, naming validation (Node + .NET + Rust) |
| `validate-markdown.yml` | PR + push to `main` | Mermaid, link, and heading-hierarchy validation via `rhino-cli`                         |
| `validate-env.yml`      | PR + push           | Environment-variable contract validation                                                |

## www tier — direct deploy (scheduled callers of `_reusable-test-and-deploy.yml`)

| Workflow                             | Site                                |
| ------------------------------------ | ----------------------------------- |
| `test-and-deploy-ayokoding-web.yml`  | ayokoding-www → ayokoding.com       |
| `test-and-deploy-ose-web.yml`        | ose-www → oseplatform.com           |
| `test-and-deploy-wahidyankf-web.yml` | wahidyankf-www → www.wahidyankf.com |

## app-web tier — gated promotion (dev → staging → dispatch promotion)

| Workflow                                           | Stage                                        |
| -------------------------------------------------- | -------------------------------------------- |
| `test-and-deploy-organiclever-web-development.yml` | Test organiclever-app-web, deploy to staging |
| `test-organiclever-web-staging.yml`                | Staging health check (organiclever-app-web)  |
| `deploy-organiclever-web-to-production.yml`        | Dispatch-only promotion staging → production |
| `test-and-deploy-ose-app-web-development.yml`      | Test ose-app-web, deploy to staging          |
| `test-ose-app-web-staging.yml`                     | Staging health check (ose-app-web)           |
| `deploy-ose-app-web-to-production.yml`             | Dispatch-only promotion staging → production |

## Backend images and CLIs

| Workflow                         | Role                                                                                                         |
| -------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| `publish-images.yml`             | Build and push `organiclever-be` / `ose-be` images to GHCR (deployed by the ose-infra k3s plans, not Vercel) |
| `test-crane-cli-integration.yml` | `crane-cli` integration tests (OCR) on `apps/crane-cli/**` and `specs/apps/crane/**` changes                 |

> **In-flight rename:** the
> [wire-vercel-www-app-cutover plan](../../plans/in-progress/wire-vercel-www-app-cutover/README.md)
> repoints the `www` callers to `prod-*-www`, adds an `organiclever-www` caller, and renames the
> OrganicLever app-web branches/environments to the `*-app-web-*` form.
