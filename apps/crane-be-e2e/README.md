# crane-be-e2e

Playwright-BDD end-to-end test runner for `apps/crane-be`. Drives the running service over real
HTTP (Playwright) and real NATS (`@nats-io/transport-node`), asserting all `@e2e` scenarios from
the crane-be Gherkin tree.

## Running

```bash
# Start services
docker compose -f docker-compose.e2e.yml up -d

# Run tests
npx nx run crane-be-e2e:test:e2e

# Stop services
docker compose -f docker-compose.e2e.yml down -v
```
