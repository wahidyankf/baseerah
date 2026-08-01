# BeaverNest Specs

Gherkin behavioral specifications and C4 architecture documentation for
[BeaverNest](../../../README.md) — the hello-world quad (`beaver-nest-be`, `beaver-nest-fe`) proving the
engineering harness end-to-end.

## Purpose

These specs define the **observable behavior** of the BeaverNest hello-world quad and the C4
architecture it sits inside. They are the single source of truth for correctness and serve as the
contract between the `beaver-nest-be`/`beaver-nest-fe` implementations and their consumers.

## Structure

```
specs/apps/beaver-nest/
├── README.md
├── product/          # PM-first hello-world scope framing
├── system-context/    # C4 L1 — actors and external systems
├── containers/        # C4 L2 — deployable units + beaver-nest-contracts OpenAPI spec
├── components/        # C4 L3 — component-level detail (deferred to Phase 6/8)
└── behavior/           # Cross-cutting Gherkin
    ├── beaver-nest-be/gherkin/   # health/, hello/ — 2 feature files
    └── beaver-nest-fe/gherkin/   # hello/ — 1 feature file
```

## Running the Tests

```bash
# Validate the spec tree structure (domain subdirs, naming, README index)
npx nx run rhino-cli:specs:structure-validation

# Lint and bundle the OpenAPI contract
npx nx run beaver-nest-contracts:lint
npx nx run beaver-nest-contracts:bundle
```

## Adding New Specs

1. Create `specs/apps/beaver-nest/behavior/<product>-<surface>/gherkin/<domain>/<feature>.feature`
2. Update the relevant index (`behavior/README.md` or the surface's `gherkin/README.md`) with the
   new feature file
3. Verify: `npx nx run rhino-cli:specs:structure-validation`

## Related

- [product/](./product/README.md) — hello-world scope
- [system-context/](./system-context/README.md) — C4 L1
- [containers/](./containers/README.md) — C4 L2
- [components/](./components/README.md) — C4 L3
- [behavior/](./behavior/README.md) — Gherkin scenarios
