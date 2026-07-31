# baseerah-be-e2e

Playwright-BDD backend E2E tests for baseerah-be.

## Quick Start

1. Start the stack: `npm run baseerah:dev` (or let `scripts/run-e2e.sh` do it for you)
2. Run E2E: `nx run baseerah-be-e2e:test:e2e`

## Commands

| Command                                     | Description                    |
| ------------------------------------------- | ------------------------------ |
| `nx run baseerah-be-e2e:test:e2e`           | Run BE E2E tests headlessly    |
| `nx run baseerah-be-e2e:test:e2e:ui`        | Run with interactive UI        |
| `nx run baseerah-be-e2e:test:e2e:report`    | View HTML report               |
| `nx run baseerah-be-e2e:specs:e2e:coverage` | Check Gherkin scenario binding |

## Feature Files

- [service-health.feature](../../specs/apps/baseerah/behavior/baseerah-be/gherkin/health/service-health.feature)
- [greeting.feature](../../specs/apps/baseerah/behavior/baseerah-be/gherkin/hello/greeting.feature)
