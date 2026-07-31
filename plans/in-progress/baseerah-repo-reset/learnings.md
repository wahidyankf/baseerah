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

<!--
Append new entries below this line as you work. Do not delete the two seeded entries — they were
identified during planning and are already carrying a routing obligation into Phase 11.
-->
