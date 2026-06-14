# AyoKoding DDD Artifacts

Domain-Driven Design artifacts for the `ayokoding-www` bounded-context architecture.
These files are the machine-readable source of truth consumed by `rhino-cli ddd bc` and
`rhino-cli ddd ul` during `nx run ayokoding-www:test:quick`.

## Structure

```
specs/apps/ayokoding/ddd/
├── README.md                  # This file
├── bounded-contexts.yaml      # Registry — bounded contexts with layers, paths, relationships
├── bounded-context-map.md     # Visual bounded-context map with Mermaid diagrams
└── ubiquitous-language/       # Per-context glossaries (one .md per bounded context)
    ├── README.md              # Authoring rules and index
    └── *.md                   # One glossary file per bounded context
```

## Files

- **[bounded-contexts.yaml](./bounded-contexts.yaml)** — Declares every bounded context
- **[bounded-context-map.md](./bounded-context-map.md)** — Visual bounded-context map
- **[ubiquitous-language/](./ubiquitous-language/README.md)** — One Markdown glossary per bounded context

## Related

- [specs/apps/ayokoding/README.md](../README.md)
- [rhino-cli ddd commands](../../../../apps/rhino-cli/README.md)
