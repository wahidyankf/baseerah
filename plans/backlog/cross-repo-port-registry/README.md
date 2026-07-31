# Cross-Repo Port Registry

> **Status**: Backlog (not started). Filed from a Knowledge Capture learning surfaced during
> `baseerah-repo-reset`'s Phase 0 (port allocation for `baseerah-be`/`baseerah-fe`).

## Context

Port allocation across `/Users/wkf/ose-projects/` is documented nowhere machine-readable — only in
a prose table in each repo's own `docs/reference/monorepo-structure.md`, and only per-repo.
`ose-public`, `ose-primer`, `ose-private`, and `baseerah` all live under that same parent directory
and can run concurrently, so a port collision is a cross-repo concern that no single repo's docs
can settle. `baseerah-repo-reset` worked around this by allocating a band (`19310`/`19320`) that no
sibling repo's table claims, plus a manual re-verification step before committing — a one-time
workaround, not a fix.

## Scope

**In scope**: design and place a shared, machine-checkable port registry spanning the four sibling
repos (`ose-public`, `ose-primer`, `ose-private`, `baseerah`) so a new app's port allocation can be
validated automatically instead of by manual prose-table review.

**Out of scope**: re-litigating `baseerah-be`'s (`19320`) or `baseerah-fe`'s (`19310`) already-
allocated ports; any change to those two apps.

## Navigation

- [brd.md](./brd.md) — WHY: business rationale, impact, risk.
- [prd.md](./prd.md) — WHAT: user story, Gherkin acceptance criteria, product scope.
- [tech-docs.md](./tech-docs.md) — HOW: the defect class, the proposed investigation, the open
  scope/home decisions the investigation phase resolves.
- [delivery.md](./delivery.md) — DO: phased, gated delivery checklist, quality gates, verification.
- [learnings.md](./learnings.md) — Knowledge Capture running log for this plan's own execution.

## Delivery Mode

`worktree-to-pr` (the repo default) — this is a tooling/governance change, so it is filed as its
own plan rather than folded into any single app's plan.
