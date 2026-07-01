# web-ui-token Specs

Gherkin behavioral specifications for
[web-ui-token](../../../libs/web-ui-token/README.md), the shared design-token package.

## Purpose

These specs define the **observable behavior** of the token package: which structural design
tokens (color, spacing, radius, typography) it exports and are consumable by downstream apps.

## Structure

```
specs/libs/web-ui-token/
├── README.md
├── product/               # C4 L1 product framing
├── system-context/        # C4 L1 actors and consumers
├── containers/            # C4 L2 deployable units
├── components/            # C4 L3 component catalogue
└── behavior/
    └── gherkin/           # Gherkin feature files
        └── tokens/
```

## Status

`test:unit` is currently an `echo` placeholder (no test runner is configured for this package
yet) — this spec tree is scaffolded ahead of that work so the C4 structure and `specs:*`
validators pass uniformly across every project.
