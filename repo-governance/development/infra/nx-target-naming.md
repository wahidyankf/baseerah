---
title: "Nx Target Naming Convention"
description: Derivation rules for Nx target names, covering the {domain}:{work} scheme for governance and validation targets and the lifecycle naming scheme for build/test targets
category: explanation
subcategory: development
tags:
  - nx
  - targets
  - naming
  - conventions
created: 2026-06-13
---

# Nx Target Naming Convention

Defines how Nx target names are derived for all projects in the workspace. Two naming schemes
apply, depending on the target's purpose: the **lifecycle scheme** for build, test, and
runtime targets, and the **`{domain}:{work}` scheme** for governance, validation, lint, and
format targets.

## Principles Implemented/Respected

- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**:
  Target names encode their scope and operation, making `nx affected -t specs:coverage` more
  self-describing than `nx affected -t spec-coverage`.

- **[Simplicity Over Complexity](../../principles/general/simplicity-over-complexity.md)**:
  Two schemes cover all cases. No per-project inventions. A reader who knows the scheme can
  predict any target name.

## Conventions Implemented/Respected

- **[Nx Target Standards](./nx-targets.md)**: The full required target set per project type
  and caching rules are defined there. This document covers only the naming derivation rule.

## Scheme 1 — Lifecycle Targets

Lifecycle targets describe the project's build and test pipeline. Names are short verbs or
`verb:qualifier` pairs. These are constant across all project types (every project that has
unit tests uses `test:unit`, not `test-unit`, not `unit`, not `unit_tests`).

| Pattern              | Examples                                                                       |
| -------------------- | ------------------------------------------------------------------------------ |
| `{verb}`             | `build`, `lint`, `typecheck`, `dev`, `start`, `run`                            |
| `{verb}:{qualifier}` | `test:quick`, `test:unit`, `test:integration`, `test:e2e`, `test:e2e:ui`       |
| `specs:coverage`     | Special: governance target that behaves like lifecycle (runs in pre-push + CI) |

**Rules**:

- Use `dev` for development server — never `serve` or `start:dev`.
- Use `start` for production server — never `serve`.
- Separate qualifiers with `:` — never `-` or `_`.
- All names are lowercase kebab-case.

## Scheme 2 — `{domain}:{work}` for Governance and Validation Targets

Governance, validation, lint, and format targets use `{domain}:{work}` where:

- **domain**: lowercase noun naming the subject or scope of the check (e.g., `specs`,
  `links`, `mermaid`, `env`, `naming`, `governance`, `cross-vendor`, `harness`,
  `format`, `msrv`).
- **work**: lowercase verb phrase naming the operation. Pure checks end in `-validation`.
  Bare operations use a single verb (`check`).

**Rule**: do not invent `validate:{thing}` prefixes. The old `validate:*` naming scheme was
retired in P10 (2026-06-12); any `validate:` target in `project.json` or a caller script
is a bug.

### Canonical Governance and Validation Targets

All defined on `rhino-cli`. Other projects expose `specs:coverage` only.

| Target                                 | Subject                   | Operation                                                |
| -------------------------------------- | ------------------------- | -------------------------------------------------------- |
| `specs:coverage`                       | Gherkin specs             | Validate every step has a step definition                |
| `specs:tree-validation`                | Specs directory tree      | Validate structure matches app registrations             |
| `specs:links-validation`               | Spec `.md` files          | Validate internal links                                  |
| `specs:counts-validation`              | Spec scenario/step counts | Validate counts meet thresholds                          |
| `specs:adoption-validation`            | App registrations         | Validate every app has a spec directory                  |
| `specs:gherkin-cardinality-validation` | Gherkin keyword usage     | Validate keyword cardinality within bounds               |
| `links:validation`                     | All `.md` files           | Validate internal + anchor links                         |
| `mermaid:validation`                   | Mermaid diagrams          | Validate width, label length, syntax (flowchart + state) |
| `headings:hierarchy-validation`        | Prose `.md` files         | Validate heading nesting on allowlist paths              |
| `env:validation`                       | `.env.example` files      | Validate against `env-contract.yaml`                     |
| `naming:harness-validation`            | Agent definition files    | Validate names match naming convention                   |
| `naming:workflows-validation`          | Workflow files            | Validate names match naming convention                   |
| `governance:vendor-audit-validation`   | `repo-governance/` docs   | Validate no vendor-specific content leakage              |
| `cross-vendor:parity-validation`       | All binding trees         | Validate cross-vendor behavioral parity                  |
| `harness:bindings-validation`          | Binding artifacts         | Validate `.claude/` ↔ `.opencode/` ↔ `.amazonq/` parity  |
| `format:check`                         | Rust source               | `rustfmt --check`                                        |
| `msrv:check`                           | Rust toolchain            | Minimum Supported Rust Version compatibility             |

### Derivation Examples

| Subject scope              | Operation      | Derived target                       |
| -------------------------- | -------------- | ------------------------------------ |
| `specs` (Gherkin)          | check adoption | `specs:adoption-validation`          |
| `links` (markdown)         | validate       | `links:validation`                   |
| `mermaid` (diagrams)       | validate       | `mermaid:validation`                 |
| `governance` (vendor docs) | audit          | `governance:vendor-audit-validation` |
| `format` (Rust fmt)        | check          | `format:check`                       |

### Anti-Patterns

| Forbidden                 | Correct                     | Reason                                 |
| ------------------------- | --------------------------- | -------------------------------------- |
| `validate:mermaid`        | `mermaid:validation`        | `validate:*` prefix abolished          |
| `validate:links`          | `links:validation`          | same                                   |
| `validate:specs-adoption` | `specs:adoption-validation` | same                                   |
| `spec-coverage`           | `specs:coverage`            | Hyphen dropped; domain clarified       |
| `fmt:check`               | `format:check`              | Domain must be the noun (`format`)     |
| `check:msrv`              | `msrv:check`                | Verb follows domain: `{domain}:{verb}` |

## Enforcement

The `validate:*` naming scheme is not validated at the Rust level (no clippy rule), but any
usage of the old form in `project.json` targets, `.husky/` hook files, `.github/workflows/`,
or `package.json` scripts is caught by the plan delivery gate:

```bash
grep -r "validate:" apps/*/project.json libs/*/project.json nx.json .husky/ .github/ package.json
```

A clean output (no matches) is the P10 / P11 gate criterion.

**See also**: [Nx Target Standards](./nx-targets.md) for the full required target set per
project type and caching rules, and [CI/CD Conventions](./ci-conventions.md) for the
Invariant E description.
