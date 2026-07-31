# Baseerah — Behavior

Cross-cutting Gherkin scenarios for the Baseerah hello-world quad, covering both C4 L2 containers.

## Structure

```
behavior/
├── baseerah-be/gherkin/    # 2 feature files, 3 scenarios (US-4)
│   ├── health/service-health.feature
│   └── hello/greeting.feature
└── baseerah-fe/gherkin/    # 1 feature file, 2 scenarios (US-5)
    └── hello/landing-page.feature
```

3 feature files, 5 scenarios total.

## Surfaces

- [baseerah-be/gherkin/](./baseerah-be/gherkin/README.md) — backend liveness, greeting, and
  unknown-route handling
- [baseerah-fe/gherkin/](./baseerah-fe/gherkin/README.md) — landing page content and accessibility

## Running the Tests

```bash
npx nx run rhino-cli:specs:structure-validation
```

## Related

- [../containers/](../containers/README.md) — C4 L2, hosts the OpenAPI contract these scenarios
  exercise
- [../system-context/](../system-context/README.md) — C4 L1
