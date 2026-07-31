# Learnings — Baseerah Repo Reset

Running log of generalizable learnings accrued while executing [delivery.md](./delivery.md). Append
entries **in the moment** you notice something worth keeping — never reconstruct this file from
memory at the end.

This file is transient. Phase 11 (Knowledge Capture) drains it: every entry is routed to exactly one
durable home or discarded with a one-line reason, per the
[Knowledge Capture Convention](../../../repo-governance/development/quality/knowledge-capture.md).
Nothing durable may depend on this file surviving archival.

## Format

```markdown
### <short title>

**Noticed**: Phase N, <what you were doing>
**Learning**: <the generalizable claim>
**Candidate home**: <convention | doc | agent | skill | code | test | backlog plan | discard>
```

## Entries

### Coverage thresholds disagree across three sources

**Noticed**: Phase 0, reading the target contract before authoring any new `project.json`
**Learning**: the repo states a line-coverage threshold in three places that do not agree —
`repo-governance/development/infra/nx-targets.md` says ≥ 90%, `docs/reference/code-coverage.md`
tabulates 95% for F# backends and 70–80% for web apps, and the actual `project.json` files passed
`/p:Threshold=80` (F# backends) and `--coverage.thresholds.lines=88` on a project whose own
`vitest.config.ts` said 70. The drift outlived the apps that caused it, so it is a governance defect
rather than an app defect. This plan resolves it at 90% for the new projects
([tech-docs Decision 11](./tech-docs.md#decision-11--resolve-the-coverage-threshold-drift-at-90-line)).
**Candidate home**: a correction inline in `repo-governance/development/infra/nx-targets.md`, plus a
`docs/reference/code-coverage.md` rewrite — both already scheduled in Phase 5, so Phase 11 needs only
to confirm they landed and record where.

### There is no port registry, only prose

**Noticed**: Phase 0, allocating ports for the two new apps
**Learning**: port allocation across this repo _and its three sibling repos_ is documented nowhere
machine-readable — only in a prose table in `docs/reference/monorepo-structure.md`, and only per-repo.
Since `ose-public`, `ose-primer`, `ose-private`, and `baseerah` all live under
`/Users/wkf/ose-projects/` and can run concurrently, collision is a cross-repo concern that no single
repo's docs can settle. This plan works around it by allocating in a band (`19310`/`19320`) that no
sibling touches, and by adding a re-verification step before the allocation is committed.
**Candidate home**: a `plans/backlog/` two-pager proposing a shared, machine-checkable port registry
across the four repos — likely a `repo-config.yml` key plus a `rhino-cli` validator.

### Tracked coverage artifact bakes in checkout-local absolute paths

**Noticed**: Phase 0, running the baseline `test:quick` sweep
**Learning**: `libs/fsharp-crane-core/tests/unit/coverage.json` is git-tracked but is a generated
.NET coverage artifact that embeds absolute filesystem paths from whichever checkout last ran the
test suite. Running the suite from `/Users/wkf/ose-projects/baseerah` regenerated it with this
checkout's paths (previously baked with `ose-public`'s), producing an 11-line diff with zero relation
to any actual code change. This will dirty the tree on every `test:quick` run from any checkout whose
absolute path differs from whoever committed it last — a latent hazard for exactly the kind of
concurrent-clone setup (`ose-public`/`ose-primer`/`ose-private`/`baseerah` all under
`/Users/wkf/ose-projects/`) this workspace already has. Reverted the regenerated file
(`git checkout -- libs/fsharp-crane-core/tests/unit/coverage.json`) as out of this plan's scope
(a pruning/deletion plan, not a coverage-tooling fix).
**Candidate home**: a `plans/backlog/` two-pager proposing either gitignoring generated coverage
artifacts under `libs/*/tests/**/coverage.json` (and any sibling `.NET` coverage output), or fixing
the coverage generator to emit relative paths.

### `infra/dev/organiclever-app/Dockerfile.be.dev` was stale, not just `ose-app`

**Noticed**: Phase 7, recovering `infra/dev/organiclever-app/` as the model for
`infra/dev/baseerah-app/`
**Learning**: delivery.md's guidance to model on `organiclever-app` rather than the known-stale
`ose-app` assumed `organiclever-app`'s own files were current. They were not: at its deletion
commit, `organiclever-be` was already F# (Giraffe/.NET 10, confirmed via its `.fsproj`/`Program.fs`),
but `infra/dev/organiclever-app/Dockerfile.be.dev` still read `FROM rust:1.95-slim` with a
`cargo run` entrypoint — a leftover from before that backend's language migration that nobody
updated. Copied the compose/dev-container _structure_ (bind-mount + healthcheck + long
`start_period` for a cold in-container build) but rewrote `Dockerfile.be.dev` from
`mcr.microsoft.com/dotnet/sdk:10.0` + `dotnet watch run`, matching baseerah-be's actual stack, and
left `baseerah-fe`'s service commented out in both compose files until Phase 8 creates
`apps/baseerah-fe/` (so `docker compose config`/`up -d` don't reference a nonexistent Dockerfile).
**Candidate home**: discard — narrated for auditability, but no durable home is warranted; the fix
is already in `infra/dev/baseerah-app/Dockerfile.be.dev` itself, and there's no reusable convention
to extract from "an old infra artifact for an app that changed language was never updated."

### A stale NuGet HTTP cache, not just a stale `obj/`, can starve `dotnet fsharplint`

**Noticed**: Phase 9, Rule-16 retest fixes — `baseerah-be:lint` failed deterministically (100% of
retries, not just occasionally) with "Package FSharp.Core, version 10.1.302 was not found" even
immediately after a clean `rm -rf obj bin && dotnet restore --force` that itself succeeded and wrote
the correct `FSharp.Core/10.1.302` entry into `obj/project.assets.json`.
**Learning**: `apps/baseerah-be/src/BaseerahBe/BaseerahBe.fsproj` already carries a comment
documenting this exact symptom as a known flip-flop between 10.1.300/10.1.302, mitigated by pinning
the exact version — but that mitigation doesn't cover every trigger. `dotnet fsharplint lint` runs a
Buildalyzer-driven **design-time build** (`ResolveAssemblyReferencesDesignTime;...` targets), a
different MSBuild invocation from the plain `dotnet build` that `typecheck` runs, and it is this
design-time build specifically that failed every time while the plain build kept succeeding on the
same `obj/`. The fix that broke the 100%-repro cycle was `dotnet nuget locals http-cache --clear`
before the next `rm -rf obj bin && dotnet restore --force` — i.e., the on-disk NuGet HTTP
response cache (`~/.local/share/NuGet/http-cache`), not the project's own `obj/`, was the actual
stale state. A plain `dotnet restore` apparently tolerates or bypasses that staleness where the
design-time build does not.
**Candidate home**: append this as a second remediation step to whatever documents the existing
"stale `obj/`/`bin/` → `rm -rf` + `dotnet restore --force`" fix for this repo (if one exists in
`repo-governance/development/quality/` or a troubleshooting doc) — `dotnet nuget locals http-cache
--clear` first, if the plain `rm -rf`+restore fix doesn't resolve a `baseerah-be:lint` failure. If no
such doc exists yet, this and the FSharp.Core pin comment are candidates for a small
troubleshooting note under `repo-governance/development/quality/` or `docs/reference/`.

<!--
Append new entries below this line as you work. Do not delete the two seeded entries — they were
identified during planning and are already carrying a routing obligation into Phase 11.
-->

## Phase 11 Triage Table

| #   | Entry                                                                             | Routed to                                                                                    | Notes                                                                                                                                                                                                                                                                    |
| --- | --------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 1   | Coverage thresholds disagree across three sources                                 | Confirmed landed                                                                             | `repo-governance/development/infra/nx-targets.md` (`test:coverage` row + guidance line) and `docs/reference/code-coverage.md` (lines 39/45-50) both now state 90% line for new projects, per tech-docs.md Decision 11. No further action needed.                         |
| 2   | There is no port registry, only prose                                             | `plans/backlog/cross-repo-port-registry/`                                                    | New backlog plan filed (README/brd/prd/tech-docs/delivery/learnings), proposing a shared, machine-checkable port registry across `ose-public`/`ose-primer`/`ose-private`/`baseerah`.                                                                                     |
| 3   | Tracked coverage artifact bakes in checkout-local absolute paths                  | `plans/backlog/coverage-artifact-relative-paths/`                                            | New backlog plan filed, proposing gitignoring generated coverage artifacts or fixing the generator to emit relative paths.                                                                                                                                               |
| 4   | `infra/dev/organiclever-app/Dockerfile.be.dev` was stale                          | Discard                                                                                      | Already decided at the time the entry was written: the fix landed inline in `infra/dev/baseerah-app/Dockerfile.be.dev` itself; no reusable convention to extract from a one-off stale infra artifact for an app whose language migrated. Narrated for auditability only. |
| 5   | A stale NuGet HTTP cache, not just a stale `obj/`, can starve `dotnet fsharplint` | `docs/explanation/software-engineering/programming-languages/f-sharp/build-configuration.md` | Added a new "Troubleshooting: Stale NuGet HTTP Cache Breaks `fsharplint`" section (before `## Enforcement`) documenting the design-time-build vs. plain-build distinction and the `dotnet nuget locals http-cache --clear` fix.                                          |

**Secret/sensitivity gate**: all 5 entries reviewed — none reference credentials, tokens, internal
hostnames, or personal data. All are safe for their routed destinations (2 public backlog plans, 1
public docs page, 1 discard, 1 confirmed-already-public governance doc).

**Repo-relevance gate**: all 5 entries concern tooling/process/governance generalizable beyond this
plan's specific deletion-and-rebuild scope (coverage thresholds, port allocation, coverage-artifact
hygiene, infra staleness, and F# toolchain flakiness) — none are `baseerah-repo-reset`-specific
implementation details that would be meaningless outside this plan.
