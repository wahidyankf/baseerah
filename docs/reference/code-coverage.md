---
title: Code Coverage Reference
description: How code coverage is measured, validated, and reported across all projects in the monorepo
category: reference
tags:
  - coverage
  - testing
  - rhino-cli
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

All projects use `rhino-cli test-coverage validate` which applies a standard
line-based algorithm:

- **COVERED**: hit count > 0 AND all branches taken (or no branches)
- **PARTIAL**: hit count > 0 but some branches not taken
- **MISSED**: hit count = 0
- **Coverage %** = `covered / (covered + partial + missed)`

Partial lines count as NOT covered.

## Supported Formats

`rhino-cli` auto-detects the coverage format from the file:

| Format       | Detection                                                                | Used By             |
| ------------ | ------------------------------------------------------------------------ | ------------------- |
| Go cover.out | Default (no other match)                                                 | Go projects         |
| LCOV (.info) | Filename ends in `.info` or contains `lcov`                              | TypeScript, Rust    |
| JaCoCo XML   | Filename ends in `.xml` containing `jacoco`, or XML with `<report>` root | (none in this repo) |

## Thresholds

| Project Type     | Threshold | Rationale                               |
| ---------------- | --------- | --------------------------------------- |
| CLI tools (Rust) | >= 90%    | Core business logic                     |
| Rust libraries   | >= 90%    | Shared utilities                        |
| organiclever-be  | >= 90%    | Rust/Axum backend API                   |
| organiclever-web | >= 70%    | Frontend app with MSW integration tests |
| ayokoding-web    | >= 80%    | Content platform with UI rendering code |
| ose-web          | >= 80%    | Content platform with UI rendering code |
| wahidyankf-web   | >= 80%    | Personal portfolio (Next.js)            |

## Per-Project Coverage Details

### Go Projects

**Tool**: `go test -coverprofile=cover.out`
**Format**: Go cover.out (statement-based, mode: set)

| Project       | Coverage File | Threshold | Exclusions |
| ------------- | ------------- | --------- | ---------- |
| rhino-cli     | `cover.out`   | 90%       | None       |
| ayokoding-cli | `cover.out`   | 90%       | None       |
| ose-cli       | `cover.out`   | 90%       | None       |
| rust-commons  | `lcov.info`   | 90%       | None       |

### TypeScript Projects

**Tool**: Vitest with `@vitest/coverage-v8`
**Format**: LCOV at `coverage/lcov.info`

| Project          | Threshold | Exclusions |
| ---------------- | --------- | ---------- |
| organiclever-web | 70%       | None       |
| ayokoding-web    | 80%       | None       |
| ose-web          | 80%       | None       |
| wahidyankf-web   | 80%       | None       |

### Rust Projects

**Tool**: cargo-llvm-cov with `--test unit --ignore-filename-regex 'main\.rs'`
**Format**: LCOV via `rhino-cli test-coverage validate`

| Project         | Threshold | Notes                                              |
| --------------- | --------- | -------------------------------------------------- |
| organiclever-be | 90%       | Line coverage via cargo-llvm-cov                   |
| ose-app-be      | 90%       | Line coverage via cargo-llvm-cov; main.rs excluded |

## CI Integration

Coverage is measured during `test:quick` (part of the pre-push hook and main CI).

### Pipeline Flow

1. `test:unit` runs tests and generates the coverage file
2. `rhino-cli test-coverage validate <file> <threshold>` checks locally
3. Both steps are combined in `test:quick`

## Troubleshooting

### Coverage drops after adding a new file

New source files with no test coverage appear as 0% in rhino-cli. Either
write tests or add the file to the appropriate exclusion config (language
tool config).

### `rhino-cli --exclude` flag

`rhino-cli test-coverage validate` supports `--exclude` glob patterns for
runtime exclusion without modifying the coverage file. Note: glob matching
may not work with Go's full module paths in `cover.out` — use `grep -v`
for Go projects instead.

## Related Documentation

- [Three-Level Testing Standard](../../repo-governance/development/quality/three-level-testing-standard.md) - Coverage thresholds and testing levels
- [Project Dependency Graph](./project-dependency-graph.md) - Which projects depend on rhino-cli
- [Nx Configuration](./nx-configuration.md) - How test:quick targets are configured
