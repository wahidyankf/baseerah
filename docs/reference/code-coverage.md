---
title: Code Coverage Reference
description: How code coverage is measured, validated, and reported across all projects in the monorepo
category: reference
tags:
  - coverage
  - testing
  - quality
created: 2026-03-22
---

# Code Coverage Reference

How code coverage is measured and validated across all projects in the monorepo.

> **Note**: The polyglot demo apps (`a-demo-be-*`, `a-demo-fe-*`) and their
> per-language coverage tooling were extracted to
> [ose-primer](https://github.com/wahidyankf/ose-primer) on 2026-04-18. That
> repository is the authoritative reference for polyglot coverage patterns
> (Java/JaCoCo, Kotlin/Kover, Python/coverage.py, Rust/cargo-llvm-cov,
> Elixir/excoveralls, C#/Coverlet, Clojure/cloverage, Dart/flutter test).

## Coverage Algorithm

Coverage is measured natively by each project's test runner. The standard
line-based algorithm counts:

- **COVERED**: hit count > 0 AND all branches taken (or no branches)
- **PARTIAL**: hit count > 0 but some branches not taken
- **MISSED**: hit count = 0
- **Coverage %** = `covered / (covered + partial + missed)`

Partial lines count as NOT covered.

## Per-Project Coverage Details

### Rust Projects

**Tool**: `cargo llvm-cov`
**Format**: LCOV at project `lcov.info`
**Threshold**: 90% line coverage

```bash
cargo llvm-cov --lib --fail-under-lines 90
```

### TypeScript Projects

**Tool**: Vitest with `@vitest/coverage-v8`
**Format**: LCOV at `coverage/lcov.info`

| Project      | Threshold | Exclusions                                                                                        |
| ------------ | --------- | ------------------------------------------------------------------------------------------------- |
| web-ui       | 70%       | None                                                                                              |
| web-ui-token | N/A       | Coverage deliberately omitted — the single vitest-cucumber scenario already covers this token lib |

> The prior TypeScript apps (`organiclever-app-web`, `ayokoding-www`, `ose-www`, `wahidyankf-www`)
> were removed along with their coverage configs in the `baseerah-repo-reset`. `baseerah-fe` (planned
> Next.js frontend, not yet scaffolded) will get its own threshold row once it exists.

### F# Projects

No F# projects exist in the repo currently — `organiclever-be` and `ose-be` (both F#/Giraffe, 95%
threshold via Coverlet) were removed along with their apps in the `baseerah-repo-reset`.
`baseerah-be` — the planned backend (port 19320, likely F#/Giraffe) — is expected to follow the same
pattern once scaffolded, but no project exists yet to threshold.

**Tool** (standing convention, applies once an F# project exists): NUnit / xUnit + Coverlet
**Format**: Cobertura XML (enforced via Coverlet threshold flags)

```bash
dotnet test --collect:"XPlat Code Coverage" \
  /p:Threshold=95 /p:ThresholdType=line /p:ThresholdStat=Total
```

## Thresholds

| Project Type          | Threshold | Rationale                                                         |
| --------------------- | --------- | ----------------------------------------------------------------- |
| CLI tools (Rust)      | >= 90%    | Core business logic (`rhino-cli`)                                 |
| Rust libraries        | >= 90%    | Shared utilities (`rust-commons`)                                 |
| web-ui (TS lib)       | >= 70%    | Shared UI component library with rendering code                   |
| web-ui-token (TS lib) | N/A       | Token-export lib; coverage deliberately omitted (see table above) |

`baseerah-fe` (planned Next.js frontend) and `baseerah-be` (planned backend) will each get a
threshold row once scaffolded; neither exists yet.

## CI Integration

Coverage is measured during `test:quick` (part of the pre-push hook and main CI)
via the native `test:coverage` Nx target per project.

### Pipeline Flow

1. `test:unit` runs tests and generates the coverage file
2. `test:coverage` enforces the threshold natively (per-project tool)
3. Both steps run sequentially inside `test:quick`

## Troubleshooting

### Coverage drops after adding a new file

New source files with no test coverage appear as 0% in the coverage report. Either
write tests or add the file to the appropriate exclusion config (language
tool config).

### Exclusions

Configure exclusions in each project's native coverage tool:

- **Rust**: `--ignore-filename-regex` flag in `cargo llvm-cov`
- **TypeScript**: `exclude` array in `vitest.config.ts`
- **C#/F#**: `[ExcludeFromCodeCoverage]` attribute on classes/methods

## Related Documentation

- [Three-Level Testing Standard](../../repo-governance/development/quality/three-level-testing-standard.md) - Coverage thresholds and testing levels
- [Project Dependency Graph](./project-dependency-graph.md) - Which projects depend on rhino-cli
- [Nx Configuration](./nx-configuration.md) - How test:quick targets are configured
