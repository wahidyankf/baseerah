# crane — DDD Artifacts

DDD adoption status for the `crane-cli` app.

## Adoption Status

`crane-cli` is a single-surface PDF-to-Markdown conversion pipeline CLI. Per the adoption
matrix, DDD adoption is **LOW expectation** for CLI apps — there is no multi-bounded-context
domain model warranting a full DDD registry.

`bounded-contexts.yaml` is present with `contexts: []` to satisfy the `rhino-cli specs
validate adoption` gate. No DDD layers, glossaries, or context maps are required for crane.

## Files

- **[bounded-contexts.yaml](./bounded-contexts.yaml)** — Empty registry (no contexts declared)

## Related

- [crane specs root](../README.md)
- [Adoption matrix](../../../../repo-governance/conventions/structure/specs-directory-structure.md)
