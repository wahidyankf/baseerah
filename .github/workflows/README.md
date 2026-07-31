# GitHub Actions Workflows

CI/CD workflows for the monorepo. Filenames follow the
[GitHub Actions Workflow Naming Convention](../../repo-governance/development/infra/github-actions-workflow-naming.md);
the [CI Conventions](../../repo-governance/development/infra/ci-conventions.md) define the
reusable-workflow pattern and the twice-daily WIB CRON schedule (with a 2.5-hour staging→prod gap).

## Shared-architecture invariant (do not "tidy up" the uncalled templates)

This repo's CI/CD design is intentionally kept consistent with the `ose-public`/`ose-primer`/
`ose-private` sibling repos rather than invented fresh. Four core workflows —
`main-ci.yml`, `pr-quality-gate.yml`, `deps-audit.yml`, `validate-env.yml` — and their non-language
job sets stay byte-identical in shape across every sibling; the language jobs (`typescript`,
`dotnet`, `rust`) track the languages actually present in the repo, unchanged by this reset because
Baseerah's language set is identical to `ose-public`'s. The three `_reusable-*.yml` templates below
are kept even though **no caller currently references them**: they are fully parameterised, name no
app, and map exactly onto the `fe`/`be` app-group shape this repo will grow into (Phases 7 and 9 add
Baseerah callers with the same shape). Deleting them as "dead code" would silently diverge from the
shared architecture — see tech-docs Decision 15 for the full invariant list and verification
commands.

## Reusable (awaiting Baseerah callers)

| Workflow                                   | Role                                                                                                     |
| ------------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| `_reusable-app-test-local-deploy-stag.yml` | App-group local-stack pipeline; on pass force-pushes BOTH the `stag-*-app-web` and `stag-*-be` branches. |
| `_reusable-app-test-stag.yml`              | FE E2E gate against the deployed staging URL (Vercel bypass secret). Stops on pass — no promote.         |
| `_reusable-be-build-deploy.yml`            | Build a backend image and push it to GHCR.                                                               |

## PR and repo-wide gates

| Workflow              | Trigger                | Role                                                                                                                              |
| --------------------- | ---------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| `pr-quality-gate.yml` | PR + push              | Typecheck, lint, `test:quick`, `compat:min-version`, naming, md-links, harness-duplication, governance validation (all languages) |
| `validate-env.yml`    | PR + push              | Environment-variable contract validation                                                                                          |
| `main-ci.yml`         | 4x/day CRON + dispatch | Same as PR gate but runs across all projects (`nx run-many --all`) — no push trigger                                              |
| `deps-audit.yml`      | Nightly CRON           | Language-native dependency audit (npm audit, cargo deny, dotnet vulnerable) — CRON-only                                           |

## Backend images

| Workflow             | Role                                                                                                                                                                       |
| -------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `publish-images.yml` | Detects affected backend image projects and publishes them to GHCR — currently a skeleton with no case arm; Phase 7 registers the `baseerah-be` image and its publish job. |
