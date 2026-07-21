# Bare-repo delivery-mode governance hardening

One-line summary: fix four governance-doc gaps around bare repos and delivery modes that a real scoping
error exposed while merging the sibling-repo parity PRs.

> Surfaced 2026-07-21 during shared-course-library parity work (merging ose-primer/ose-infra PRs #13/#15).

## Problem / context

Four concrete governance-doc defects surfaced while merging the ose-primer/ose-infra
governance-propagation PRs and scoping the ose-public delta. The sharpest one already caused a real
failure: a scoping agent misread ose-primer's merged `main` as un-merged because it used
`git rev-parse --is-bare-repository`, which returns `false` from a linked worktree — the exact trap no
doc warns against.

## Why now

The defects are fresh and concrete (one already caused a misread), and all four are cheap
documentation fixes that then propagate public → siblings before the next parity cycle repeats them.

## Prior art / precedents

- **git worktree (bare repos + linked worktrees)** — the tool whose `--is-bare-repository` returns
  `false` from a linked worktree, the exact trap item 3 codifies against. [git-scm](https://git-scm.com/docs/git-worktree)
- **Plans Organization convention §Delivery Mode** — the table (item 1) that lists the impossible
  `main-to-*` modes for bare repos. [plans](../../repo-governance/conventions/structure/plans.md)
- **PR Merge Protocol** — the enumeration sites item 4 mirrors the saturation-qualifier note into.
  [pr-merge-protocol](../../repo-governance/development/workflow/pr-merge-protocol.md)
- **plan-multi-repo-parity-planning workflow** — carries the name-scoped bare-repo grill question
  (item 2) to be property-bound. [parity-planning](../../repo-governance/workflows/plan/plan-multi-repo-parity-planning.md)

## Proposed direction (sketch)

Four data-pointed fixes:

1. **Impossible delivery modes** — the Delivery Mode table in `plans.md` (~L576-582) lists
   `main-to-origin-main` / `main-to-pr` unconditionally, but a bare repo (ose-primer, ose-infra) has no
   primary checkout and cannot use any `main-to-*` mode. Note the restriction; fix the contradicting
   `plan-multi-repo-parity-planning.md` parenthetical (~L345 vs its own L202 note).
2. **Property-bind the bare-repo grill question** — `plan-multi-repo-parity-planning.md` meta-question
   #1 (~L341) is name-scoped to ose-primer; generalize to "any bare repo with no primary checkout" so
   it fires for ose-infra too.
3. **Codify a bareness-verification method** (absent from all three repos) — prescribe
   `git config --file <common-dir>/config core.bare` and explicitly forbid
   `git rev-parse --is-bare-repository` (false from a linked worktree).
4. **Optional inline saturation qualifier** — mirror the "(default 3)" floor-not-ceiling note from
   `pr-review-quality-gate.md` L328-330 inline at `pr-merge-protocol.md`'s two enumeration sites
   (~L46, ~L169).

## Rough scope & non-goals

In scope: the four documentation fixes above, then propagated to the sibling repos.

Out of scope (for now): changing delivery-mode behaviour itself; automating bareness detection in
tooling.

## Risks & open questions

- Item 4 is a free stylistic add — worth bundling, or drop it to keep the plan tight? (open)

## What success looks like + promotion signal

Success: no doc lists an impossible `main-to-*` mode for a bare repo, the bareness check is codified in
all three repos, and the `--is-bare-repository` trap can't recur. Ready to promote now — these are
well-specified, low-risk doc edits.
