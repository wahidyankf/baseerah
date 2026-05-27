# Business Requirements Document

## Business Goal

Remove all remnant artifacts (docs, agents, skills, CI jobs, toolchain scripts, config entries)
for tech stacks no longer active in `ose-public`. Retain what serves active stacks: TypeScript,
Go, Rust, and F#/C# (.NET — crane-cli is F#; C# retained for potential dotnet interop).

## Business Impact

After migrations — .NET → Rust for `ose-app-be`, and the polyglot demo extraction to
`ose-primer` — inactive stacks left behind:

- **Documentation files** across 6 language directories (Java, Kotlin, Elixir, Clojure, Dart,
  Python), describing stacks with no active apps
- **6 agent files + 6 skill directories** for languages with no active apps in ose-public
- **CI gate jobs** (JVM, Python) that always skip but add workflow complexity
- **Broken infra** (`infra/dev/ose-app/Dockerfile.be.dev` still references old dotnet SDK for
  `ose-app-be`, which is now Rust/Axum)
- **Stale solution file** (`open-sharia-enterprise.sln`) has no project references

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

- `grep -r "lang:java\|lang:kotlin\|lang:elixir\|lang:clojure\|lang:dart\|lang:python" .github/`
  returns zero results outside of `archived/` and `plans/done/`
- `ls .claude/agents/ | grep "swe-"` shows only: golang, typescript, rust, e2e, csharp, fsharp
- `ls docs/explanation/software-engineering/programming-languages/` shows: c-sharp/, f-sharp/,
  golang/, rust/, typescript/, README.md — and no directory for java, kotlin, elixir, clojure,
  dart, or python
- `grep "F#\|Giraffe\|dotnet" infra/dev/ose-app/Dockerfile.be.dev` returns nothing
- `dotnet sln open-sharia-enterprise.sln list` shows crane-cli project references

## Business Non-Goals

- **No apps/libs source changes** — this is purely a remnant sweep
- **No content changes to ayokoding-web** — language tutorials there are educational content, not tooling remnants
- **No ose-primer changes** — that repo now owns the inactive lang content
- **No historical record removal** — `plans/done/` entries and `archived/` stay

## Business Risks

| Risk                                                                          | Mitigation                                                                    |
| ----------------------------------------------------------------------------- | ----------------------------------------------------------------------------- |
| Removing agent/skill needed for content creation tasks                        | N/A — C#/F# agents and skills are retained; only JVM/ose-primer langs removed |
| Breaking `infra/dev/ose-app/docker-compose.yml` by changing Dockerfile.be.dev | Replacement Dockerfile uses same Rust image pattern as organiclever-be        |
| CI workflow syntax errors after job removal                                   | Local `act` or post-push CI verification catches this                         |
| Broken internal links after doc removal                                       | `npm run lint:md` and the docs-link-checker catch dead references             |
