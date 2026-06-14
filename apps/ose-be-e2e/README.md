# ose-be-e2e

Playwright-BDD backend E2E tests for ose-be.

## Quick Start

1. Start BE: `nx dev ose-be`
2. Run E2E: `nx run ose-be-e2e:test:e2e`

## Commands

| Command                            | Description                 |
| ---------------------------------- | --------------------------- |
| `nx run ose-be-e2e:test:e2e`       | Run BE E2E tests headlessly |
| `nx run ose-be-e2e:specs:coverage` | Check Gherkin step coverage |

## Feature Files

- [health.feature](../../specs/apps/ose/behavior/be/gherkin/health/health.feature)
