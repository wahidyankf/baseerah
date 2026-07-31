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

## Per-Project Coverage Thresholds

**Single source of truth** — every current or planned project appears exactly once below, with its
tool, format, and enforced threshold. This resolves the 80%/88%/95% drift the retired apps left
behind (tech-docs Decision 11 of the `baseerah-repo-reset` plan): new projects use **90% line**,
matching the `nx-targets.md` governance rule, rather than reproducing the old F# backends' 95% or
the old web apps' 70–88%.

| Project        | Status                      | Tool                           | Format        | Threshold                                                                                            |
| -------------- | --------------------------- | ------------------------------ | ------------- | ---------------------------------------------------------------------------------------------------- |
| `rhino-cli`    | Current                     | `cargo llvm-cov`               | LCOV          | 90% line                                                                                             |
| `rust-commons` | Current                     | `cargo llvm-cov`               | LCOV          | 90% line                                                                                             |
| `web-ui`       | Current                     | Vitest + `@vitest/coverage-v8` | LCOV          | 70% line (preexisting; not retroactively raised)                                                     |
| `web-ui-token` | Current                     | N/A                            | N/A           | N/A — deliberately omitted; the single vitest-cucumber scenario already covers this token-export lib |
| `baseerah-be`  | Planned, not yet scaffolded | NUnit/xUnit + Coverlet         | Cobertura XML | 90% line (new project — Decision 11)                                                                 |
| `baseerah-fe`  | Planned, not yet scaffolded | Vitest + `@vitest/coverage-v8` | LCOV          | 90% line (new project — Decision 11)                                                                 |

### Enforcement commands by tool

```bash
# Rust (rhino-cli, rust-commons)
cargo llvm-cov --lib --fail-under-lines 90

# TypeScript (web-ui: 70%; baseerah-fe once scaffolded: 90%)
npx vitest run --coverage --coverage.thresholds.lines=<threshold>

# F# (baseerah-be once scaffolded)
dotnet test --collect:"XPlat Code Coverage" \
  /p:Threshold=90 /p:ThresholdType=line /p:ThresholdStat=Total
```

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
