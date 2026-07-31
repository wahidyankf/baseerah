# baseerah-fe-e2e

Playwright-BDD frontend E2E tests for baseerah-fe, run against the full stack
(`baseerah-fe` on :19310 fetching from `baseerah-be` on :19320).

## Quick Start

1. Start the stack: `docker compose -f infra/dev/baseerah-app/docker-compose.yml up -d`
2. Run E2E: `nx run baseerah-fe-e2e:test:e2e`

## Commands

| Command                                     | Description                    |
| ------------------------------------------- | ------------------------------ |
| `nx run baseerah-fe-e2e:test:e2e`           | Run FE E2E tests headlessly    |
| `nx run baseerah-fe-e2e:test:e2e:ui`        | Run with interactive UI        |
| `nx run baseerah-fe-e2e:test:e2e:report`    | View HTML report               |
| `nx run baseerah-fe-e2e:specs:e2e:coverage` | Check Gherkin scenario binding |

## Feature Files

- [landing-page.feature](../../specs/apps/baseerah/behavior/baseerah-fe/gherkin/hello/landing-page.feature)
