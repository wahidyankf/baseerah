# Coverage Artifact Relative Paths

> **Status**: Backlog (not started). Filed from a Knowledge Capture learning surfaced during
> `baseerah-repo-reset`'s Phase 0 baseline `test:quick` sweep.

## Context

`libs/fsharp-crane-core/tests/unit/coverage.json` is git-tracked but is a generated .NET coverage
artifact that embeds absolute filesystem paths from whichever checkout last ran the test suite.
Running the suite from `/Users/wkf/ose-projects/baseerah` regenerated it with this checkout's
paths (previously baked with `ose-public`'s), producing an 11-line diff with zero relation to any
actual code change. This dirties the tree on every `test:quick` run from any checkout whose
absolute path differs from whoever committed it last — a latent hazard for exactly the kind of
concurrent-clone setup (`ose-public`/`ose-primer`/`ose-private`/`baseerah` all under
`/Users/wkf/ose-projects/`) this workspace already has. The regenerated file was reverted
(`git checkout -- libs/fsharp-crane-core/tests/unit/coverage.json`) as out of `baseerah-repo-reset`
scope (a pruning/deletion plan, not a coverage-tooling fix).

## Scope

**In scope**: either (a) gitignore generated coverage artifacts under `libs/*/tests/**/coverage.json`
(and any sibling .NET coverage output) so they stop being tracked at all, or (b) fix the coverage
generator/config to emit relative paths so tracking them stays harmless across checkouts. Phase 1
investigation picks between the two.

**Out of scope**: any change to `libs/fsharp-crane-core`'s own test content or coverage thresholds.

## Navigation

- [brd.md](./brd.md) — WHY: business rationale, impact, risk.
- [prd.md](./prd.md) — WHAT: user story, Gherkin acceptance criteria, product scope.
- [tech-docs.md](./tech-docs.md) — HOW: the defect class, the proposed investigation, the open
  scope/home decisions the investigation phase resolves.
- [delivery.md](./delivery.md) — DO: phased, gated delivery checklist, quality gates, verification.
- [learnings.md](./learnings.md) — Knowledge Capture running log for this plan's own execution.

## Delivery Mode

`worktree-to-pr` (the repo default) — this is a tooling/CI-hygiene change, so it is filed as its
own plan rather than folded into any single lib's plan.
