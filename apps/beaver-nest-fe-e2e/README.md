# beaver-nest-fe-e2e

Playwright-BDD frontend E2E tests for beaver-nest-fe, run against the full stack
(`beaver-nest-fe` on :19310 fetching from `beaver-nest-be` on :19320).

## Quick Start

1. Start the stack: `docker compose -f infra/dev/beaver-nest-app/docker-compose.yml up -d`
2. Run E2E: `nx run beaver-nest-fe-e2e:test:e2e`

## Commands

| Command                                        | Description                    |
| ---------------------------------------------- | ------------------------------ |
| `nx run beaver-nest-fe-e2e:test:e2e`           | Run FE E2E tests headlessly    |
| `nx run beaver-nest-fe-e2e:test:e2e:ui`        | Run with interactive UI        |
| `nx run beaver-nest-fe-e2e:test:e2e:report`    | View HTML report               |
| `nx run beaver-nest-fe-e2e:specs:e2e:coverage` | Check Gherkin scenario binding |

## Feature Files

- [landing-page.feature](../../specs/apps/beaver-nest/behavior/beaver-nest-fe/gherkin/hello/landing-page.feature)
