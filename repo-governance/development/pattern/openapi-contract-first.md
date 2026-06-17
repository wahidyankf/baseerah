---
title: OpenAPI Contract-First Development
description: Spec-first API development — the OpenAPI YAML is the single source of truth; code is generated from it, not the reverse
category: explanation
subcategory: development
tags:
  - openapi
  - contract-first
  - codegen
  - api
  - drift-enforcement
created: 2026-05-26
---

# OpenAPI Contract-First Development

Contract-first development means the OpenAPI YAML specification is written before any implementation code. The spec is
the single source of truth for every API contract. Generated code follows from the spec; the spec never follows from
the code.

## Principles Implemented/Respected

- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**: Every endpoint,
  request body, response schema, and error type is declared explicitly in the YAML before it exists in any
  implementation. No undocumented behaviour can accumulate silently.

- **[Reproducibility First](../../principles/software-engineering/reproducibility.md)**: Codegen runs from a
  committed YAML file. The same spec always produces the same generated types. CI enforces that generated files match
  the spec — no drift is tolerated.

- **[Automation Over Manual](../../principles/software-engineering/automation-over-manual.md)**: Type definitions,
  serialisers, and route skeletons are generated automatically. Manual synchronisation between spec and code is
  eliminated.

- **[Simplicity Over Complexity](../../principles/general/simplicity-over-complexity.md)**: A single YAML file is the
  authoritative interface description. Frontend, backend, and integration tests all read from the same source.

## Conventions Implemented/Respected

- **[Hexagonal Architecture + DDD — Backend Apps](./hexagonal-architecture-be.md)**: Generated types land in the
  `api/http/` inbound adapter layer. Domain types are hand-authored; generated request/response types stay at the
  boundary.

## Overview

Each BE↔client pair maintains an OpenAPI 3.1 YAML spec. Codegen tooling reads that spec and emits typed client code
(TypeScript) and server scaffolding (Rust). CI runs codegen on every push and fails the build if the generated
output differs from the committed output. This makes spec drift a hard CI failure rather than a silent runtime
divergence.

## Spec Location

Specs live under the `specs/` tree, organised by app and container:

```
specs/
└── apps/
    └── <app-name>/
        └── containers/
            └── contracts/
                └── openapi.yaml
```

| BE App            | Spec Path                                                   |
| ----------------- | ----------------------------------------------------------- |
| `organiclever-be` | `specs/apps/organiclever/containers/contracts/openapi.yaml` |
| `ose-be`          | `specs/apps/ose/containers/contracts/openapi.yaml`          |

The spec file is the only artefact that humans edit. Generated files are never edited by hand.

## Codegen Tooling

| Target                                            | Tool                  | Output Path                | Notes                                     |
| ------------------------------------------------- | --------------------- | -------------------------- | ----------------------------------------- |
| TypeScript client (`organiclever-web`, `ose-www`) | `@hey-api/openapi-ts` | `src/generated-contracts/` | Emits typed fetch client + schema types   |
| F# server (`organiclever-be`)                     | `nswag` (F# target)   | `generated-contracts/`     | Emits Giraffe handler types + model types |
| F# server (`ose-be`)                              | `nswag` (F# target)   | `generated-contracts/`     | Emits Giraffe handler types + model types |

Generated directories are committed to the repository. The CI drift check (see below) compares the freshly generated
output against the committed files and fails if they differ.

## Nx Targets

Each app that participates in contract-first development exposes these Nx targets in its `project.json`:

| Target    | App                      | Command                                                      |
| --------- | ------------------------ | ------------------------------------------------------------ |
| `codegen` | `organiclever-web`       | Runs `@hey-api/openapi-ts` against the contracts spec        |
| `codegen` | `organiclever-be`        | Runs `nswag` F# target                                       |
| `codegen` | `ose-www`                | Runs `@hey-api/openapi-ts` against the contracts spec        |
| `codegen` | `ose-be`                 | Runs `nswag` F# target                                       |
| `lint`    | `organiclever-contracts` | Validates and bundles the OpenAPI spec (Redocly or Spectral) |
| `docs`    | `organiclever-contracts` | Generates browsable API documentation                        |

Run codegen for a specific app:

```bash
nx run organiclever-web:codegen
nx run organiclever-be:codegen
nx run ose-www:codegen
nx run ose-be:codegen
```

Validate the spec itself:

```bash
nx run organiclever-contracts:lint
```

## Drift Enforcement

CI enforces that committed generated files match the spec. After running `codegen`, any non-empty `git diff` in the
generated output directory fails the build.

The CI step for each app follows this pattern:

```bash
# 1. Run codegen from the committed spec
nx run <app>:codegen

# 2. Fail if generated output differs from committed files
git diff --exit-code src/generated-contracts/
# (Rust apps use generated-contracts/ without the src/ prefix)
```

A non-zero exit code from `git diff --exit-code` means the spec was updated but codegen was not re-run before commit,
or vice versa. The fix is always to re-run `nx run <app>:codegen` and commit the updated generated files together with
the spec change.

## Scope

Contract-first development covers these BE↔client pairs:

| Backend           | Client             | Spec                                                        |
| ----------------- | ------------------ | ----------------------------------------------------------- |
| `organiclever-be` | `organiclever-web` | `specs/apps/organiclever/containers/contracts/openapi.yaml` |
| `ose-be`          | `ose-www`          | `specs/apps/ose/containers/contracts/openapi.yaml`          |

Apps outside this table (CLI tools, content-only web apps such as `ayokoding-www` and `ose-www`) do not participate
in contract-first codegen.

## Related

- **[Hexagonal Architecture + DDD — Backend Apps](./hexagonal-architecture-be.md)** — Where generated types land in
  the layer structure (`api/http/` boundary); domain types are never generated
- **[Functional Core / Imperative Shell — Web Apps](./functional-core-imperative-shell-web.md)** — Where generated
  TypeScript client types land in the web app structure (`features/<name>/shell/`, the imperative shell)
