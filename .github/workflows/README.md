# GitHub Actions Workflows

CI/CD workflows for the monorepo. Filenames follow the
[GitHub Actions Workflow Naming Convention](../../repo-governance/development/infra/github-actions-workflow-naming.md);
the [CI Conventions](../../repo-governance/development/infra/ci-conventions.md) define the
reusable-workflow pattern and the twice-daily WIB CRON schedule (with a 2.5-hour staging→prod gap).

## Reusable

| Workflow                                   | Role                                                                                                     |
| ------------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| `_reusable-www-test-local-deploy.yml`      | Full local-stack test pipeline (lint, unit, integration, E2E) then force-push to a `prod-*-www` branch.  |
| `_reusable-app-test-local-deploy-stag.yml` | App-group local-stack pipeline; on pass force-pushes BOTH the `stag-*-app-web` and `stag-*-be` branches. |
| `_reusable-app-test-stag.yml`              | FE E2E gate against the deployed staging URL (Vercel bypass secret). Stops on pass — no promote.         |
| `_reusable-be-build-deploy.yml`            | Build a backend image and push it to GHCR (rolled out by ose-infra `coralpolyp`).                        |

## PR and repo-wide gates

| Workflow                | Trigger             | Role                                                                                    |
| ----------------------- | ------------------- | --------------------------------------------------------------------------------------- |
| `pr-quality-gate.yml`   | Pull request        | Typecheck, lint, `test:quick`, `specs:coverage`, naming validation (Node + .NET + Rust) |
| `validate-markdown.yml` | PR + push to `main` | Mermaid, link, and heading-hierarchy validation via `rhino-cli`                         |
| `validate-env.yml`      | PR + push           | Environment-variable contract validation                                                |

## www tier — direct deploy (scheduled callers of `_reusable-www-test-local-deploy.yml`)

| Workflow                                      | Site                                |
| --------------------------------------------- | ----------------------------------- |
| `ayokoding-www-test-local-deploy-prod.yml`    | ayokoding-www → ayokoding.com       |
| `ose-www-test-local-deploy-prod.yml`          | ose-www → oseplatform.com           |
| `organiclever-www-test-local-deploy-prod.yml` | organiclever-www → organiclever.com |
| `wahidyankf-www-test-local-deploy-prod.yml`   | wahidyankf-www → www.wahidyankf.com |

## app tier — gated promotion (local-deploy-stag → test-stag; prod CD deferred)

| Workflow                                      | Stage                                                            |
| --------------------------------------------- | ---------------------------------------------------------------- |
| `organiclever-app-test-local-deploy-stag.yml` | Test the organiclever app group, force-push web + be stag branch |
| `organiclever-app-test-stag-deploy-prod.yml`  | FE E2E gate vs staging (+2.5h); stops on pass                    |
| `ose-app-test-local-deploy-stag.yml`          | Test the ose-app group, force-push web + be stag branch          |
| `ose-app-test-stag-deploy-prod.yml`           | FE E2E gate vs staging (+2.5h); stops on pass                    |

## Backend images and CLIs

| Workflow                         | Role                                                                                                         |
| -------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| `publish-images.yml`             | Build and push `organiclever-be` / `ose-be` images to GHCR (deployed by the ose-infra k3s plans, not Vercel) |
| `test-crane-cli-integration.yml` | `crane-cli` integration tests (OCR) on `apps/crane-cli/**` and `specs/apps/crane/**` changes                 |

> **In-flight cutover:** the
> [wire-vercel-www-app-cutover plan](../../plans/in-progress/wire-vercel-www-app-cutover/README.md)
> creates the `prod-*-www`, `stag-*-app-web`, and `stag-*-be` branches and populates the Vercel +
> GitHub Environment values these workflows reference. Until then a scheduled run's `git push` to a
> not-yet-created branch fails loudly (expected, non-destructive).
