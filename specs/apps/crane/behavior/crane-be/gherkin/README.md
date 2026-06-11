# crane-be Gherkin

Behaviour specifications for the `crane-be` HTTP + NATS service.

## Domains

- `health/` — Liveness check
- `media/` — HTTP PDF-to-Markdown conversion endpoint
- `messaging/` — NATS `crane.convert` request/reply subscriber

## Test levels consuming this tree

- **Unit** (`@unit`): TickSpec F# in `apps/crane-be/tests/unit/`
- **Integration** (`@integration`): TickSpec F# in `apps/crane-be/tests/integration/`
- **E2E** (`@e2e`): Playwright-BDD TypeScript in `apps/crane-be-e2e/`
