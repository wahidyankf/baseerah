# Technical Documentation

## Architecture

This plan introduces no new architectural components. It is a pure cleanup sweep: delete
inactive-lang docs, agents, skills, and CI jobs; correct stale infra references; and update the
VS solution file to include the active F# project. No source code, application structure, or
runtime behaviour changes.

## Complete File Inventory

All paths verified [Repo-grounded] via `ls`/`find` at 2026-05-27.

### Phase 1: .NET Artifacts — Retain C#/F#; Correct ose-app Infra

> **Decision**: C# and F# artifacts are **retained**. `crane-cli` is active F#; C# retained for
> potential dotnet interop. Only `infra/dev/ose-app/` infra files and `open-sharia-enterprise.sln`
> need action.

#### Keep (no change)

| Path                                                                   | Reason                                 |
| ---------------------------------------------------------------------- | -------------------------------------- |
| `.github/actions/setup-dotnet/action.yml` (+ dir)                      | crane-cli CI dependency                |
| `scripts/format-csharp.sh`                                             | C# tooling retained for dotnet interop |
| `docs/explanation/software-engineering/programming-languages/c-sharp/` | C# retained                            |
| `docs/explanation/software-engineering/programming-languages/f-sharp/` | crane-cli is F#                        |
| `.claude/agents/swe-csharp-dev.md` + `.opencode/agents/`               | C# retained                            |
| `.claude/agents/swe-fsharp-dev.md` + `.opencode/agents/`               | crane-cli is F#                        |
| `.claude/skills/swe-programming-csharp/`                               | C# retained                            |
| `.claude/skills/swe-programming-fsharp/`                               | crane-cli is F#                        |
| `package.json` `"*.cs"` lint-staged entry                              | C# retained                            |
| `lang:fsharp\|lang:csharp` detect + dotnet job in pr-quality-gate.yml  | dotnet gate needed for crane-cli       |
| `setup-dotnet` step in `crane-cli-integration.yml`                     | crane-cli CI                           |

#### Update

| Path                         | Change                                                                                       |
| ---------------------------- | -------------------------------------------------------------------------------------------- |
| `open-sharia-enterprise.sln` | Add crane-cli project references via `dotnet sln add` (see delivery.md Phase 1a for command) |

#### Replace

| Path                                  | Current                                         | Replace With                                            |
| ------------------------------------- | ----------------------------------------------- | ------------------------------------------------------- |
| `infra/dev/ose-app/Dockerfile.be.dev` | `FROM mcr.microsoft.com/dotnet/sdk:10.0-alpine` | `FROM rust:1.95-slim` (matches organiclever-be pattern) |

#### Modify

| Path                                      | Change                                                                        |
| ----------------------------------------- | ----------------------------------------------------------------------------- |
| `infra/dev/ose-app/docker-compose.ci.yml` | Remove `ASPNETCORE_URLS: "http://+:8302"` (line 4)                            |
| `infra/dev/ose-app/README.md`             | Change "F#/Giraffe REST API backend" → "Rust/Axum REST API backend" (line 10) |

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
| `docs/explanation/software-engineering/programming-languages/README.md` | Remove all sections for the 6 removed langs (Java, Kotlin, Elixir, Clojure, Dart, Python). Update "Skills Available" list, "Quick Decision" table, "Current Language Usage" table. Active langs: Go, Rust, TypeScript, F#/C# (.NET). |
| `AGENTS.md`                                                             | Final pass — confirm all 6 removed agents are gone from **Development** list. Verify C#/F# agents still present. Update skill references in AI Agents section if any.                                                                |

#### Run generate:bindings (Phase 3c — see D2)

```bash
npm run generate:bindings
```

This syncs `.opencode/agents/` from `.claude/agents/`. Run after Phase 3 deletions as the
authoritative sync — verifies all 6 inactive lang agents are removed from `.opencode/agents/`
while C#/F# mirrors remain.

## Design Decisions

### D1: Replacement Dockerfile pattern

`infra/dev/organiclever/Dockerfile.be.dev` uses `FROM rust:1.95-slim` with no additional
instructions (the docker-compose `volumes` mount mounts the workspace at `/workspace`). The
ose-app-be docker-compose uses the same volumes pattern, so the same minimal Dockerfile works.
[Repo-grounded]

### D2: Single generate:bindings call — Phase 3c only

C#/F# agents are retained (no Phase 1 agent deletion). Phase 3c runs `npm run generate:bindings`
as the authoritative sync after all 6 inactive lang agent deletions. This verifies all 6 inactive
agents are removed from `.opencode/agents/` while C#/F# mirrors remain intact. [Judgment call —
single call sufficient since Phase 1 makes no agent changes]

### D3: TypeScript exclusion list in pr-quality-gate.yml

The TypeScript gate excludes other lang tags to avoid double-running on polyglot projects:
`--exclude='tag:lang:golang,...'`. After cleanup, `tag:lang:golang`, `tag:lang:rust`, and
`tag:lang:fsharp`/`tag:lang:csharp` remain valid exclusions (dotnet gate kept for crane-cli).

### D4: Python detection removal

Python was detected (`has-python`, `lang:python` gate job) despite no Python apps in ose-public.
Remove entirely. If Python is ever added, the gate can be re-added at that time.

### D5: Elixir/Clojure/Dart had detection but no gate jobs

These had `has-X` outputs and detect cases but no corresponding `X:` job blocks and were not in
the quality-gate needs list. They are purely vestigial — remove from detect step only.

### D6: C# retained for dotnet interop

C# has no active apps in ose-public, but `crane-cli` is F# and .NET interop between F# and C#
is zero-cost (same runtime). Retaining C# agents, skills, docs, `format-csharp.sh`, and the
lint-staged `*.cs` entry costs nothing and avoids a painful re-add if C# interop is ever needed.
[Judgment call — retention cost near-zero; removal cost non-trivial if reversed]

## Rollback

This is a documentation/config cleanup with no runtime dependencies. Rollback = `git revert`
of the relevant commits. Each phase is a separate commit for granular rollback.

## Impact on ose-primer

Zero. `ose-primer` owns these stacks going forward. No sync needed — this plan only removes
from ose-public, not from ose-primer.
