# Baseerah — Containers (C4 L2)

Container-level specifications for the Baseerah hello-world quad.

## Baseerah Hello-World Quad

- [container.md](./container.md) — container list (see this README)
- [contracts/](./contracts/README.md) — OpenAPI 3.1 contract spec (`baseerah-contracts` Nx
  project) defining the two `baseerah-be` routes `baseerah-fe` consumes

Two runtime containers exist: `baseerah-fe` (Next.js, port 19310) and `baseerah-be` (F#/Giraffe,
port 19320). A dedicated `container.md` diagram and `deployment.md` topology doc are deferred until
Phase 6/8 land the actual runtimes — see [product/](../product/README.md) for the deferred-scope
list.

## Related

- [system-context/](../system-context/README.md) — C4 L1
- [components/](../components/README.md) — C4 L3
- [behavior/](../behavior/README.md) — Gherkin scenarios
