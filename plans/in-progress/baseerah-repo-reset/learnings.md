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

<!--
Append new entries below this line as you work. Do not delete the two seeded entries — they were
identified during planning and are already carrying a routing obligation into Phase 11.
-->
