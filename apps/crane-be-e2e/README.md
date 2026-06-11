# crane-be-e2e

Playwright-BDD E2E test runner for `crane-be`. Drives the running containerised service
over real HTTP (Playwright) and real NATS (`@nats-io/transport-node`), asserting all
`@e2e` scenarios from the crane-be Gherkin tree.

## Stack

- **Playwright-BDD** 8.5.1 consuming
  [specs/…/crane-be/gherkin/](../../specs/apps/crane/behavior/crane-be/gherkin/README.md)
  `@e2e` scenarios
- **@nats-io/transport-node** 3.3.1 for NATS request/reply
- **docker-compose.e2e.yml**: two NATS servers (organiclever + ose-app) + crane-be

## Running

```bash
# Start the compose stack
docker compose -f apps/crane-be-e2e/docker-compose.e2e.yml up -d

# Run all @e2e scenarios
nx run crane-be-e2e:test:e2e

# Stop the stack
docker compose -f apps/crane-be-e2e/docker-compose.e2e.yml down -v
```

## Commands

| Nx target                           | What it does                                  |
| ----------------------------------- | --------------------------------------------- |
| `nx run crane-be-e2e:test:quick`    | bddgen + tsc --noEmit + oxlint                |
| `nx run crane-be-e2e:test:e2e`      | All `@e2e` scenarios (requires compose stack) |
| `nx run crane-be-e2e:test:e2e:ui`   | Playwright UI mode                            |
| `nx run crane-be-e2e:spec-coverage` | Gherkin step coverage (rhino-cli)             |

## Specification coverage

```bash
nx run crane-be-e2e:spec-coverage
```

Covers all four gherkin domains: `health/`, `media/`, and `messaging/` (including the
NATS request/reply, error-envelope, and dual-connection-isolation scenarios excluded from
the F# integration suite by the strict no-network rule).

## Related

- [crane-be](../crane-be/README.md) — the service under test
- [specs/apps/crane/](../../specs/apps/crane/README.md) — full spec tree
