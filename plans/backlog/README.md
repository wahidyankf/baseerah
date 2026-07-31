# Backlog Plans

Full, ready-to-execute plans waiting to start. A plan lands here only when it has been **promoted
from a two-pager** in [`../ideas/`](../ideas/README.md) — i.e. its open questions have shrunk to ones
that genuinely need a full plan's depth to answer.

## Planned Projects

- [audit-e2e-reuse-existing-server-config](./audit-e2e-reuse-existing-server-config/README.md)
  — Audits whether `reuseExistingServer: true` (hardcoded unconditionally in six `*-e2e`
  `playwright.config.ts` files) risks silently reusing a stale, unrelated server, and applies a
  CI-conditional gate, doc caveat, or automated check depending on runner persistence.
- [cross-repo-governance-link-parity](./cross-repo-governance-link-parity/README.md)
  — Validates shared governance doc anchors across `ose-public`, `ose-primer`, and `ose-private`
  during multi-repo landings.
- [merge-queue-adoption](./merge-queue-adoption/README.md)
  — Hardens merge-precondition (c) under concurrent integration; owns the merge-queue work deferred
  from `worktree-to-pr-hardening`.
- [ose-private-opencode-ci-monitor-orphan](./ose-private-opencode-ci-monitor-orphan/README.md)
  — Removes or restores the source mirror for a stale `.opencode/agents/ci-monitor-subagent.md` in
  the sibling `ose-private` repo.
- [vitest-glob-coverage-guard](./vitest-glob-coverage-guard/README.md)
  — Designs a durable, automated guard against test files landing outside every configured Vitest
  project's `include` glob, after a regression test silently executed zero times due to exactly this
  gap.

Other candidate work lives as two-pager idea briefs in [`../ideas/`](../ideas/README.md); promote one
here when it is ripe.

## Instructions

**Idea Capture**: For ideas not ready for formal planning, write a two-pager in
[`../ideas/`](../ideas/README.md) — not here.

**Naming**: Plans in `backlog/` use NO date prefix — just the slug (e.g.,
`doc-command-existence-validation/`). A date prefix is applied only when a plan is archived to
`done/`, where it records the completion date.

When promoting a two-pager to a plan:

1. Create folder: `[project-identifier]/`
2. Add standard files: README.md, brd.md, prd.md, tech-docs.md, delivery.md — carrying the
   two-pager's problem, scope, and open questions forward
3. Add the plan to this list
4. Delete the two-pager from `../ideas/` and drop its line from `../ideas/README.md`
