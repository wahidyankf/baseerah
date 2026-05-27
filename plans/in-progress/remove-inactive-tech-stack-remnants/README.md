# Remove Inactive Tech Stack Remnants

## Context

Several tech stacks previously planned or briefly active in `ose-public` have been superseded:

- **C# (.NET)** — `ose-app-be` was ported from .NET to Rust (2026-05-27). C# has no active apps
  but is **retained** for potential dotnet interop with `crane-cli`.
- **F# (.NET)** — `crane-cli` was rewritten in F# (2026-05-27) and is **active**. F# agents,
  skills, docs, and CI gate are kept.
- **Java, Kotlin, Elixir, Clojure, Dart, Python** — polyglot demo apps were extracted to
  [`ose-primer`](https://github.com/wahidyankf/ose-primer) on 2026-04-18. No active apps for
  these languages remain in `ose-public`.

Despite this, the JVM/ose-primer-lang docs, agents, skills, CI gates, and config references
remain in the repo, creating noise, longer CI runtimes, and confusing signals for contributors.
Additionally, `infra/dev/ose-app/` still references the old F#/Giraffe backend (now Rust/Axum),
and `open-sharia-enterprise.sln` has no project references.

**Active tech stacks in `ose-public`**: TypeScript, Go, Rust, F#/C# (.NET).

## Scope

### In-Scope

- Delete language-specific docs (`docs/explanation/software-engineering/programming-languages/`)
  for: Java, Kotlin, Elixir, Clojure, Dart, Python
- Delete agent files (`.claude/agents/`, `.opencode/agents/`) for 6 inactive langs (Java,
  Kotlin, Elixir, Clojure, Dart, Python)
- Delete skill directories (`.claude/skills/swe-programming-*/`) for 6 inactive langs
- Remove CI detection and gate jobs for JVM, Python from `pr-quality-gate.yml`
- Remove vestigial detection for Elixir, Clojure, Dart from `pr-quality-gate.yml`
- Update `open-sharia-enterprise.sln` — add crane-cli project references
- Replace `infra/dev/ose-app/Dockerfile.be.dev` dotnet image with Rust equivalent
- Remove `ASPNETCORE_URLS` from `infra/dev/ose-app/docker-compose.ci.yml`
- Update `AGENTS.md` — remove 6 inactive lang agents from Development list
- Update `docs/.../programming-languages/README.md` — remove sections for 6 inactive langs
- Update `infra/dev/ose-app/README.md` — change "F#/Giraffe" → "Rust/Axum"
- Sync OpenCode bindings: `npm run generate:bindings` after agent removals

### Out-of-Scope

- Apps and libs code (no changes to `apps/`, `libs/` source files)
- `archived/` directory (preserved for reference)
- `plans/done/` entries that mention inactive langs (historical records)
- `ose-primer` repository (owns inactive lang content going forward)
- Ayokoding-web educational content about these languages (separate concern)

## Approach

Sequential cleanup in four phases grouped by concern:

1. **Dotnet cleanup** — F#/C# artifacts (most impactful, most files)
2. **JVM cleanup** — Java/Kotlin artifacts
3. **Other ose-primer langs** — Elixir, Clojure, Dart, Python artifacts
4. **Cross-cutting** — docs README rewrite, AGENTS.md update, final link check

Each phase ends with local quality gates before proceeding.

## Documents

- [Business Rationale](./brd.md) — why this cleanup
- [Product Requirements](./prd.md) — acceptance criteria, Gherkin scenarios
- [Technical Documentation](./tech-docs.md) — exact file inventory, design decisions
- [Delivery Checklist](./delivery.md) — phased execution checklist
