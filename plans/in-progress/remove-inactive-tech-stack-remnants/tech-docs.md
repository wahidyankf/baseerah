# Technical Documentation

## Complete File Inventory

All paths verified [Repo-grounded] via `ls`/`find` at 2026-05-27.

### Phase 1: Dotnet (F# / C#) Artifacts

#### Delete entirely

| Path                                                                   | Type                                          |
| ---------------------------------------------------------------------- | --------------------------------------------- |
| `open-sharia-enterprise.sln`                                           | Empty VS solution file                        |
| `.github/actions/setup-dotnet/action.yml` (+ dir)                      | CI action for dotnet toolchain                |
| `scripts/format-csharp.sh`                                             | C# formatter wrapper script                   |
| `docs/explanation/software-engineering/programming-languages/c-sharp/` | 14 files (13 standards + templates/README.md) |
| `docs/explanation/software-engineering/programming-languages/f-sharp/` | 14 files (13 standards + templates/README.md) |
| `.claude/agents/swe-csharp-dev.md`                                     | C# agent definition                           |
| `.claude/agents/swe-fsharp-dev.md`                                     | F# agent definition                           |
| `.opencode/agents/swe-csharp-dev.md`                                   | OpenCode mirror                               |
| `.opencode/agents/swe-fsharp-dev.md`                                   | OpenCode mirror                               |
| `.claude/skills/swe-programming-csharp/`                               | C# skill directory                            |
| `.claude/skills/swe-programming-fsharp/`                               | F# skill directory                            |

#### Replace

| Path                                  | Current                                         | Replace With                                            |
| ------------------------------------- | ----------------------------------------------- | ------------------------------------------------------- |
| `infra/dev/ose-app/Dockerfile.be.dev` | `FROM mcr.microsoft.com/dotnet/sdk:10.0-alpine` | `FROM rust:1.95-slim` (matches organiclever-be pattern) |

#### Modify

| Path                                                                    | Change                                                                                                                                                                                               |
| ----------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `package.json`                                                          | Remove `"*.cs": "scripts/format-csharp.sh"` from lint-staged                                                                                                                                         |
| `.github/workflows/pr-quality-gate.yml`                                 | Remove: `has-dotnet` output, `lang:fsharp\|lang:csharp` detect case, dotnet job block, `dotnet` from quality-gate needs + for-loop, `tag:lang:fsharp,tag:lang:csharp` from TypeScript exclusion list |
| `.github/workflows/crane-cli-integration.yml`                           | Remove `- uses: ./.github/actions/setup-dotnet` step                                                                                                                                                 |
| `infra/dev/ose-app/docker-compose.ci.yml`                               | Remove `ASPNETCORE_URLS: "http://+:8302"`                                                                                                                                                            |
| `infra/dev/ose-app/README.md`                                           | Change "F#/Giraffe REST API backend" → "Rust/Axum REST API backend"                                                                                                                                  |
| `AGENTS.md`                                                             | Remove `swe-csharp-dev, swe-fsharp-dev` from Development agents list                                                                                                                                 |
| `docs/explanation/software-engineering/programming-languages/README.md` | Remove C# and F# sections (see Phase 4 for full README rewrite)                                                                                                                                      |

### Phase 2: JVM (Java / Kotlin) Artifacts

#### Delete entirely

| Path                                                                  | Type                    |
| --------------------------------------------------------------------- | ----------------------- |
| `docs/explanation/software-engineering/programming-languages/java/`   | Java docs directory     |
| `docs/explanation/software-engineering/programming-languages/kotlin/` | Kotlin docs directory   |
| `.claude/agents/swe-java-dev.md`                                      | Java agent definition   |
| `.claude/agents/swe-kotlin-dev.md`                                    | Kotlin agent definition |
| `.opencode/agents/swe-java-dev.md`                                    | OpenCode mirror         |
| `.opencode/agents/swe-kotlin-dev.md`                                  | OpenCode mirror         |
| `.claude/skills/swe-programming-java/`                                | Java skill directory    |
| `.claude/skills/swe-programming-kotlin/`                              | Kotlin skill directory  |

#### Modify

| Path                                    | Change                                                                                                                    |
| --------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| `.github/workflows/pr-quality-gate.yml` | Remove: `has-jvm` output, `lang:java\|lang:kotlin` detect case, `jvm` job block, `jvm` from quality-gate needs + for-loop |
| `AGENTS.md`                             | Remove `swe-java-dev, swe-kotlin-dev` from Development agents list                                                        |

### Phase 3: Other ose-primer Langs (Elixir, Clojure, Dart, Python)

#### Delete entirely

| Path                                                                   | Type                                                                                                                                                                        |
| ---------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `docs/explanation/software-engineering/programming-languages/elixir/`  | Elixir docs                                                                                                                                                                 |
| `docs/explanation/software-engineering/programming-languages/clojure/` | Clojure docs                                                                                                                                                                |
| `docs/explanation/software-engineering/programming-languages/dart/`    | Dart docs                                                                                                                                                                   |
| `docs/explanation/software-engineering/programming-languages/python/`  | Python docs                                                                                                                                                                 |
| `.claude/agents/swe-elixir-dev.md`                                     | Elixir agent                                                                                                                                                                |
| `.claude/agents/swe-clojure-dev.md`                                    | Clojure agent                                                                                                                                                               |
| `.claude/agents/swe-dart-dev.md`                                       | Dart agent                                                                                                                                                                  |
| `.claude/agents/swe-python-dev.md`                                     | Python agent                                                                                                                                                                |
| `.opencode/agents/swe-elixir-dev.md`                                   | OpenCode mirror                                                                                                                                                             |
| `.opencode/agents/swe-clojure-dev.md`                                  | OpenCode mirror                                                                                                                                                             |
| `.opencode/agents/swe-dart-dev.md`                                     | OpenCode mirror                                                                                                                                                             |
| `.opencode/agents/swe-python-dev.md`                                   | OpenCode mirror                                                                                                                                                             |
| `.claude/skills/swe-programming-elixir/`                               | Elixir skill                                                                                                                                                                |
| `.claude/skills/swe-programming-clojure/`                              | Clojure skill                                                                                                                                                               |
| `.claude/skills/swe-programming-dart/`                                 | Dart skill                                                                                                                                                                  |
| `.claude/skills/swe-programming-python/`                               | Python skill                                                                                                                                                                |
| `libs/clojure-openapi-codegen/`                                        | Clojure lib — source already removed; only `LICENSE` tracked + gitignored artifacts (`.cpcache/`, `classes/`, `coverage/`) remain. Not in Nx workspace (no `project.json`). |

#### Modify

| Path             | Change                                                                                                            |
| ---------------- | ----------------------------------------------------------------------------------------------------------------- |
| `.gitignore`     | Remove line `# Clojure classpath cache` + `.cpcache/` (Clojure-specific; no Clojure code remains after this plan) |
| `libs/README.md` | Remove `clojure-openapi-codegen/` entry from the libs listing                                                     |

#### Modify

| Path                                    | Change                                                                                                                                                                                                                                                                                                                                              |
| --------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `.github/workflows/pr-quality-gate.yml` | Remove: `has-python` output, `lang:python` detect case, `python` job block, `python` from quality-gate needs + for-loop. Remove vestigial: `has-elixir`, `has-clojure`, `has-dart` outputs and their detect cases (these had no gate jobs). Remove `tag:lang:python,tag:lang:elixir,tag:lang:clojure,tag:lang:dart` from TypeScript exclusion list. |
| `AGENTS.md`                             | Remove `swe-elixir-dev, swe-dart-dev, swe-kotlin-dev, swe-clojure-dev` from Development agents list (some may already be removed in Phase 2; skip duplicates)                                                                                                                                                                                       |

### Phase 4: Cross-Cutting

#### Rewrite

| Path                                                                    | Change                                                                                                                                                                                                                               |
| ----------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `docs/explanation/software-engineering/programming-languages/README.md` | Remove all sections for the 8 removed langs. Update "Skills Available" list, "Quick Decision" table, "Current Language Usage" table, "Domain-Specific Standards Pattern" example list. Active langs remaining: Go, Rust, TypeScript. |
| `AGENTS.md`                                                             | Final pass — confirm all 8 removed agents are gone from **Development** list. Update skill references in the AI Agents section if any.                                                                                               |

#### Run generate:bindings (two calls — see D2)

```bash
npm run generate:bindings
```

This syncs `.opencode/agents/` from `.claude/agents/`. Run after Phase 1 deletions to validate
dotnet agent removal, and again after Phase 3 deletions to catch all remaining removals in one
pass.

## Design Decisions

### D1: Replacement Dockerfile pattern

`infra/dev/organiclever/Dockerfile.be.dev` uses `FROM rust:1.95-slim` with no additional
instructions (the docker-compose `volumes` mount mounts the workspace at `/workspace`). The
ose-app-be docker-compose uses the same volumes pattern, so the same minimal Dockerfile works.
[Repo-grounded]

### D2: Two generate:bindings calls — Phase 1d and Phase 3c

Phase 1d runs `npm run generate:bindings` as a parity-validation check after manual C#/F# agent
deletion. Phase 3c runs it as the authoritative sync after all 8 agent deletions. Running it
twice is intentional: Phase 1d catches any sync gap early (immediately after the highest-volume
deletion), while Phase 3c provides the definitive verification that all 8 inactive agents are
removed from `.opencode/agents/`. [Judgment call — generate:bindings in Phase 3c provides
explicit verification regardless of pre-commit hook behavior]

### D3: TypeScript exclusion list in pr-quality-gate.yml

The TypeScript gate excludes other lang tags to avoid double-running on polyglot projects:
`--exclude='tag:lang:golang,...'`. After cleanup, only `tag:lang:golang` and `tag:lang:rust`
remain valid exclusions (TS projects won't have other tags anyway, but keep golang + rust for
correctness).

### D4: Python detection removal

Python was detected (`has-python`, `lang:python` gate job) despite no Python apps in ose-public.
Remove entirely. If Python is ever added, the gate can be re-added at that time.

### D5: Elixir/Clojure/Dart had detection but no gate jobs

These had `has-X` outputs and detect cases but no corresponding `X:` job blocks and were not in
the quality-gate needs list. They are purely vestigial — remove from detect step only.

## Rollback

This is a documentation/config cleanup with no runtime dependencies. Rollback = `git revert`
of the relevant commits. Each phase is a separate commit for granular rollback.

## Impact on ose-primer

Zero. `ose-primer` owns these stacks going forward. No sync needed — this plan only removes
from ose-public, not from ose-primer.
