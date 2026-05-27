# Business Requirements Document

## Business Goal

Remove all remnant artifacts (docs, agents, skills, CI jobs, toolchain scripts, config entries)
for tech stacks no longer active in `ose-public`. Retain only what serves the three active stacks:
TypeScript, Go, and Rust.

## Problem

After several migrations — .NET → Rust for `ose-app-be`, F# → Rust for `crane-cli`, and the
polyglot demo extraction to `ose-primer` — inactive stacks left behind:

- **124 documentation files** [Repo-grounded] across 8 language directories, describing stacks with no active apps
- **8 agent files + 8 skill directories** [Repo-grounded] for languages never used by ose-public's production code
- **CI gate jobs** (dotnet, JVM, Python) that always skip but add workflow complexity
- **Toolchain scripts and config** (`.sln`, `format-csharp.sh`, lint-staged C# entry)
- **Broken infra** (`Dockerfile.be.dev` still uses dotnet SDK for an app now on Rust)

This creates:

- Confusion about which stacks are actually supported
- False signals in docs (F# listed as "Planned" when it was removed)
- Longer CI workflow parse time
- Maintenance surface for code that will never run

## Affected Roles

- **Contributors** — cleaner signal about which stacks are in use
- **AI agents** — no stale agent definitions or skills triggering on non-existent apps
- **CI system** — leaner workflow with fewer no-op jobs

## Business Success Metrics

[Judgment call] After completion:

- `grep -r "lang:fsharp\|lang:csharp\|lang:java\|lang:kotlin\|lang:elixir\|lang:clojure\|lang:dart\|lang:python" .github/` returns zero results outside of archived/ and plans/done/
- `ls .claude/agents/ | grep "swe-"` shows only active-stack agents (golang, typescript, rust, e2e)
- `ls docs/explanation/software-engineering/programming-languages/` shows only: golang/, rust/,
  typescript/, README.md — and no directory for c-sharp, f-sharp, java, kotlin, elixir, clojure,
  dart, or python

## Business Non-Goals

- **No apps/libs source changes** — this is purely a remnant sweep
- **No content changes to ayokoding-web** — language tutorials there are educational content, not tooling remnants
- **No ose-primer changes** — that repo now owns the inactive lang content
- **No historical record removal** — `plans/done/` entries and `archived/` stay

## Business Risks

| Risk                                                                          | Mitigation                                                             |
| ----------------------------------------------------------------------------- | ---------------------------------------------------------------------- |
| Removing agent/skill needed for content creation tasks                        | Verify: no active apps use these agents; ose-primer owns them          |
| Breaking `infra/dev/ose-app/docker-compose.yml` by changing Dockerfile.be.dev | Replacement Dockerfile uses same Rust image pattern as organiclever-be |
| CI workflow syntax errors after job removal                                   | Local `act` or post-push CI verification catches this                  |
| Broken internal links after doc removal                                       | `npm run lint:md` and the docs-link-checker catch dead references      |
