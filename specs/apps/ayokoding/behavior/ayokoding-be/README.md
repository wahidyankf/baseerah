# ayokoding-be — Behavior Surface

tRPC HTTP-semantic Gherkin scenarios for the AyoKoding API surface.

## Contents

- **[gherkin/](./gherkin/README.md)** — Feature files organized by bounded context domain.
  Consumed by `apps/ayokoding-www-be-e2e` (Playwright BE E2E).

## Note

`ayokoding-be` is a **perspective slug**, not a container. The tRPC procedures run inside
the same `web` Next.js container. The surface exists so API contract behavior can be
specified separately from UI behavior.

## Background step

All scenarios use: `Given the API is running`

## Related

- [Parent behavior README](../README.md)
- [Gherkin specs](./gherkin/README.md)
