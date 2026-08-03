# beaver-nest-be — Gherkin

Behavioral scenarios for the `beaver-nest-be` F#/Giraffe REST API, organized by domain.

## Feature Files

- [health/liveness.feature](./health/liveness.feature) — liveness without persistence detail
- [health/readiness-ready.feature](./health/readiness-ready.feature) — ready database state
- [health/readiness-unready.feature](./health/readiness-unready.feature) — safe unavailable state
- [hello/greeting.feature](./hello/greeting.feature) — greeting + unknown-route handling
  (2 scenarios)
- [persistence/](./persistence/) — migration and SQLite safety behavior
- [recovery/](./recovery/) — verified online backup and restore behavior

Copied verbatim from [prd.md US-4](../../../../../../plans/done/2026-07-31__baseerah-repo-reset/prd.md#us-4--serve-hello-world-from-baseerah-be).

## Related

- [behavior/README.md](../../README.md) — behavior index
- [../../../containers/contracts/](../../../containers/contracts/README.md) — the OpenAPI
  contract these scenarios exercise
