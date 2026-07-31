# Baseerah API Contract

OpenAPI 3.1 specification for the Baseerah hello-world quad's REST API.

## Purpose

This contract defines the exact shape of every request and response for `baseerah-be`, consumed
by `baseerah-fe`. It is the single source of truth for API types for this phase.

## Quick Start

```bash
# Lint the contract
nx run baseerah-contracts:lint

# Bundle into a single resolved YAML
nx run baseerah-contracts:bundle
```

## File Structure

```
contracts/
├── README.md
├── openapi.yaml     # Full spec: health, hello, and the shared Error schema
├── project.json     # Nx project targets
└── generated/        # Output (gitignored)
    └── openapi-bundled.yaml
```

## Rules

- Exactly two `GET` routes (`/api/v1/health`, `/api/v1/hello`) — no write operations this phase
- Every schema has a `description`
- Changes to this contract should stay in lockstep with the Gherkin scenarios in
  [../../behavior/baseerah-be/gherkin/](../../behavior/baseerah-be/gherkin/README.md)

## Related

- [../](../README.md) — containers index (C4 L2)
