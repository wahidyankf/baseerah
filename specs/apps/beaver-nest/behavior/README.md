# BeaverNest — Behavior

Cross-cutting Gherkin scenarios for the BeaverNest hello-world quad, covering both C4 L2 containers.

## Structure

```
behavior/
├── beaver-nest-be/gherkin/    # 2 feature files, 3 scenarios (US-4)
│   ├── health/service-health.feature
│   └── hello/greeting.feature
└── beaver-nest-fe/gherkin/    # 1 feature file, 2 scenarios (US-5)
    └── hello/landing-page.feature
```

3 feature files, 5 scenarios total.

## Surfaces

- [beaver-nest-be/gherkin/](./beaver-nest-be/gherkin/README.md) — backend liveness, greeting, and
  unknown-route handling
- [beaver-nest-fe/gherkin/](./beaver-nest-fe/gherkin/README.md) — landing page content and accessibility

## Running the Tests

```bash
npx nx run rhino-cli:specs:structure-validation
```

## Related

- [../containers/](../containers/README.md) — C4 L2, hosts the OpenAPI contract these scenarios
  exercise
- [../system-context/](../system-context/README.md) — C4 L1
