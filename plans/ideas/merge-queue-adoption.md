# Hardening merge-precondition (c) under concurrent merges

One-line summary: a merge queue would make merge-precondition (c) — "the branch is non-destructively
up to date with the latest `origin/main`" — hold under concurrent `worktree-to-pr` merges rather than
only when PRs land one at a time, but GitHub's native queue is gated on organization ownership, and
the only owner-type probes on record covered three sibling repos that do not include this one.

> Provenance: demoted from a full `backlog/` plan to a two-pager on 2026-08-05. The full plan carried
> the five-document set — `README.md`, `brd.md`, `prd.md`, `tech-docs.md`, and a nine-phase
> `delivery.md` (Phases 0-8: availability investigation, CI trigger, precondition reword plus an
> operations doc, finalization, per-repo enablement, two propagation phases, knowledge capture, and
> archival) — plus seven Gherkin acceptance criteria, seven user stories, an availability matrix, a
> mechanism-comparison table, a file-impact list, and two Mermaid diagrams. It was itself split out
> of `worktree-to-pr-hardening`, where the queue was researched as decisions D7 and D10 and then
> dropped from scope.

## Problem / context

This repo's default delivery mode is `worktree-to-pr`, and its stated rationale is maximum
parallelization: N independent units become N PRs that review, gate, and merge independently
([AGENTS.md](../../AGENTS.md) §Delivery Mode). A PR merges only when all five hardened preconditions
(a)-(e) hold, and (c) requires the branch to be non-destructively up to date with the latest
`origin/main` at merge time
([PR Merge Protocol](../../repo-governance/development/workflow/pr-merge-protocol.md)).

A static, per-PR "branch up to date" check cannot guarantee (c) under concurrency. PR-A and PR-B are
each green against base `X`; A merges, `main` becomes `X+A`, and B is now silently stale — possibly
carrying a semantic rather than textual conflict that no per-PR check ever saw. The more the repo
leans on its parallel-by-default posture, the more often two PRs are ready at overlapping times,
which is exactly the window (c) is weakest in.

A merge queue closes that window structurally: a ready PR is enqueued rather than merged, the queue
builds a speculative merge onto the current queue head and runs CI on that artifact (the GitHub
`merge_group` event), and a PR whose speculative CI fails is auto-evicted with `main` untouched. Each
PR stays an independent merge point, so the strict 1-PR ↔ 1-worktree model survives intact.

The blocking discovery is what makes this a brief rather than a plan. The maintainer originally
reported being unable to find a merge-queue toggle in branch settings, and that report was factually
correct rather than a UI-navigation mistake: GitHub gates merge queue on repository **owner type**,
not visibility or plan tier, and it is not offered at all to repositories owned by a personal (User)
account. On 2026-07-23 the plan recorded live `gh api repos/<owner>/<repo> --jq '.owner.type'` probes
returning `User` for `ose-public`, `ose-primer`, and `ose-private` — all three unavailable, one
shared blocker rather than three independent gaps.

Two caveats specific to this repo, which the inherited plan text does not cover:

- **This repo was never probed.** Every owner-type verdict in the source documents names one of the
  three sibling repos. No `.owner.type` result for `beaver-nest` appears anywhere in them, so its
  availability is genuinely unknown here, not merely assumed-unavailable.
- **This repo is outside the parity loop.** [AGENTS.md](../../AGENTS.md) §Related Repositories states
  that `ose-private` and `beaver-nest` do not participate in the three-repo parity loop, so the
  plan's "identical scaffolding in all three repos" posture does not transfer — any adoption here is
  a local decision.

On the CI side, the local facts are concrete. `.github/workflows/pr-quality-gate.yml` is the workflow
that triggers on `pull_request` (types `opened`, `synchronize`, `reopened`) plus `push` to `main`, so
it is the candidate that would receive a `merge_group` trigger here. `.github/workflows/main-ci.yml`
triggers only on `schedule` and `workflow_dispatch` and would gate nothing in a queue.

## Why now

Nothing forces this now, and that is itself the finding. The GitHub-native path is blocked until the
ownership model changes, and the recommended resolution in the source plan was explicitly to keep
merge queue deferred. What is worth doing now is preserving the research so the next person does not
re-derive the owner-type gate from scratch — the search cost has already been paid once — and
recording that the CI-trigger and protocol scaffolding is harmless to land early, since a
`merge_group` trigger stays inert until a queue actually exists. The natural trigger to revisit is
any move of this repo under an organization, or any moment when two PRs here actually race and (c)
fails in practice.

## Prior art / precedents

- **[PR Merge Protocol](../../repo-governance/development/workflow/pr-merge-protocol.md)** — the
  normative home of the five preconditions. In this repo it restates (c) four times (the rule, the
  before-merging agent workflow, the precondition-summary example, and the PASS worked example), and
  any reword has to keep all four congruent.
- **[PR Review Quality Gate](../../repo-governance/workflows/pr/pr-review-quality-gate.md)** — the
  workflow the PR-review agents actually read the preconditions from; it restates (c) too, as does
  [plan-quality-gate.md](../../repo-governance/workflows/plan/plan-quality-gate.md) and
  [plans.md](../../repo-governance/conventions/structure/plans.md) §Delivery Mode.
- **GitHub-native merge queue** — the D10 mechanism choice, cheapest given the existing `gh`
  toolchain and no new vendor, but organization-gated
  ([GitHub Blog announcement](https://github.blog/news-insights/product-news/github-merge-queue-is-generally-available/),
  [community discussion #51483](https://github.com/orgs/community/discussions/51483), and the
  [`merge_group` event docs](https://docs.github.com/en/actions/using-workflows/events-that-trigger-workflows#merge_group)).
- **Graphite's stack-aware queue** — the strongest-evidenced third-party alternative; Ramp
  Engineering reported a 74% decrease in median time between merges, with engineers merging PRs up to
  3x faster ([Graphite blog](https://graphite.com/blog/the-first-stack-aware-merge-queue)). Whether
  it works on personal-account repos was left unverified.
- **[sibling-main-ci-never-runs-on-merge](./sibling-main-ci-never-runs-on-merge.md)** — the same
  family of defect: a CI trigger that is configured such that the signal never fires, read as a pass.

## Proposed direction (sketch)

Investigate before adopting, and keep the scaffolding separable from the enablement.

- Probe this repo's own owner type first; that single answer decides whether the native path exists
  here at all. Do not inherit the sibling verdicts.
- If a queue is reachable, add `merge_group` to the `on:` block of the workflow whose checks are
  required for merge — locally that is `pr-quality-gate.yml` — reusing the existing `pull_request`
  job set so queued CI equals branch CI, and keeping the change `actionlint`-clean.
- Reword precondition (c) so it is satisfiable by the queue's speculative merge where a queue is
  enabled, while retaining the manual non-destructive branch-up-to-date form as the fallback
  everywhere else. Preconditions (a), (b), (d), (e) and the (a)-(e) lettering stay verbatim, and the
  reword must land congruently across every surface that restates (c), not just the protocol file.
- Write an operations note covering how the queue interacts with the PR-Review Maker→Fixer Cycle (the
  queue is an integration step after review, never a review step), with the `[AI]`-merges-by-default
  posture (merge means enqueue), and with 1-PR ↔ 1-worktree (the queue orders PRs, it does not merge
  their identities).
- Treat enablement as a `[HUMAN]` settings toggle bracketed by agent preparation and agent
  verification. An agent must never change repository security settings.

## Rough scope & non-goals

In scope: an owner-type availability check for this repo; a `merge_group` trigger on the required
gating workflow; the precondition-(c) reword across its restatement sites; a merge-queue operations
doc; and a `[HUMAN]` enablement runbook with `gh api` verification afterward.

Out of scope, carried verbatim from the source plan:

- Any `apps/` or `libs/` runtime code — this is CI config plus governance docs only.
- The PR-reviewer decomposition, owned by `worktree-to-pr-hardening`.
- Provisioning a bot or GitHub-App identity — a separate idea, filed as
  [pr-review-bot-identity](./pr-review-bot-identity.md).
- Changing any of the other four merge preconditions (a), (b), (d), (e).
- Deciding MQ-1 on the maintainer's behalf: migrating repository ownership to a GitHub organization
  is a significant `[HUMAN]` infrastructure decision and adopting a third-party queue is a vendor
  decision. The brief records the fork and a recommendation, not a pre-made choice.

Additionally out of scope here, because this repo sits outside the parity loop: propagating identical
scaffolding to the sibling repos as a parity obligation
([related repositories](../../docs/reference/related-repositories.md),
[multi-repo parity workflow](../../repo-governance/workflows/plan/plan-multi-repo-parity-planning-and-execution.md)).

## Risks & open questions

- **MQ-1, the blocking fork.** Four options were recorded: (A) migrate to a GitHub organization and
  adopt the native queue; (B) adopt a third-party queue such as Graphite or Aviator; (C) harden (c)
  with a lightweight non-queue guard such as auto-rebase-before-merge or a serialize-merges
  convention; (D, recommended) keep merge queue deferred and leave (c) as the manual check. Unresolved.
  (open)
- **What is this repo's owner type?** The three recorded `User` verdicts are from 2026-07-23 and none
  of them is this repo. Until probed, both "available" and "unavailable" are guesses. (open)
- **Does Graphite actually work on personal-account repositories?** The source marked this
  `[Unverified]` and flagged it as needing a dedicated research pass before Option B could be chosen.
  (open)
- **Does `gh pr merge --auto` enqueue reliably?** The source cited independent reports (for example
  `cli/cli#5653`) that automerge does not behave uniformly with merge queues across `gh` versions and
  configurations. If it does not, the `[AI]`-merges-by-default posture fights the queue. (open)
- **Which checks are actually required on `main` here?** The `merge_group` trigger should go only on
  required-gate workflows (MQ-2's recommended option); confirming the required-check set needs a
  branch-protection read this brief did not perform. (open)
- Queue tuning was pre-answered rather than left open: MQ-3 recommends starting at batch size 1
  (strict serialization) for correctness and tuning later.
- A non-risk worth stating: adding `merge_group` before any queue exists changes nothing, because the
  event only fires once a PR enters a merge queue. Every element of this direction is independently
  revertible, and (c)'s manual fallback is retained throughout rather than replaced.

## What success looks like + promotion signal

Success: whether a merge queue is available for this repo is written down and verified rather than
assumed; if one is available and adopted, two concurrently-ready PRs integrate through it with CI on
each speculative merge result and a failing PR is auto-evicted without breaking `main`; and if it is
not available, the deferral names the exact owner-type limitation and its resume condition instead of
sitting silently as an unmet gate.

Promotion signal: this is ripe the moment `gh api repos/<owner>/beaver-nest --jq '.owner.type'`
returns `Organization` — that single result unblocks Option A and makes the whole scaffolding
executable. Failing that, promote if the maintainer commits to Option B or C, or if precondition (c)
is ever observed to fail in practice on a real pair of racing PRs here, which would convert the
motivating risk from theoretical to measured. Absent all three, "not promoted yet" is the correct
state, and Option D is the standing recommendation.
