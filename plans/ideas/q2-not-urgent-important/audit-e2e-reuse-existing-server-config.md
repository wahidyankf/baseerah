# `reuseExistingServer` Can Silently Swap a Stale Server Into an E2E Run

One-line summary: Playwright's `reuseExistingServer: true` adopts **any** process already bound to
the target port and skips the configured `webServer.command` entirely — but in this repo the pattern
survives in exactly one config (`libs/web-ui/e2e/playwright.config.ts`, line 19), driven by a target
CI never runs, so the originally-proposed `!process.env.CI` gate would be a no-op here.

> Demoted from a full `backlog/` plan to a two-pager on 2026-08-05. The full plan carried a five-document
> split — `README.md`, `brd.md`, `prd.md`, `tech-docs.md`, a three-phase gated `delivery.md`
> (CI-runner-persistence investigation → remedy selection → knowledge capture and archival), and an
> empty `learnings.md`. It was filed from a Knowledge Capture learning during another repo's
> `ayokoding-www-tools-ai-benchmark` Phase 10 Rule-15 retest, and had already been rescoped once
> (2026-07-31) after the repo reset deleted the apps it originally targeted. The re-derivation below
> shrinks it further, which is why it belongs here rather than in `backlog/`.

## Problem / context

The original incident is real and worth recording. A long-lived `next dev` process, started hours
earlier and before that session's code changes, was already listening on an app's e2e port.
`reuseExistingServer: true` found it, skipped the configured `webServer.command`, and the suite
silently exercised stale dev-mode code instead of the production build the config specifies
(`NODE_ENV: "production"`, a standalone server, e2e-specific env vars such as
`AYOKODING_WEB_MANIFESTS_DIR`). A later full run against that same stale server produced a wall of
unrelated-looking failures, every one tracing back to the reused server never having the e2e fixture
manifests directory wired in. A repo-wide grep on 2026-07-30 found the setting hardcoded `true` in
six configs (`ayokoding-www-fe-e2e`, `ayokoding-www-be-e2e`, `organiclever-www-fe-e2e`,
`wahidyankf-www-fe-e2e`, `ose-www-fe-e2e`, `ose-www-be-e2e`) plus one that already gated it correctly
(`organiclever-app-web-e2e`).

**Re-derived against this repo on 2026-08-05, the problem is almost entirely absent here.** BeaverNest
has a different app set — all seven of those projects were deleted by the
[repo reset](../../done/2026-07-31__baseerah-repo-reset/README.md). A fresh search finds **three**
`playwright.config.ts` files in total, and only **one** mentions `reuseExistingServer` at all:

- `libs/web-ui/e2e/playwright.config.ts` — line 19, `reuseExistingServer: true`, inside a `webServer`
  block running `npx storybook dev -p 6006 --no-open --ci` on port 6006. The one genuine instance.
- `apps/beaver-nest-fe-e2e/playwright.config.ts` — **no `webServer` block at all**; targets
  `process.env.WEB_BASE_URL || "http://localhost:19310"` (line 19).
- `apps/beaver-nest-be-e2e/playwright.config.ts` — **no `webServer` block at all**; targets
  `process.env.API_BASE_URL || "http://localhost:19320"` (line 18); its `test:e2e` target shells out
  to `apps/beaver-nest-be/scripts/run-e2e.sh`.

Two further facts collapse the original plan's central open question. First, every `runs-on:` in
`.github/workflows/` is `ubuntu-latest` — GitHub-hosted and ephemeral per job, with no self-hosted
runner anywhere in this repo. Second, the only consumer of the offending config, `libs/web-ui`'s
`test:visual` target, is invoked by **neither** `main-ci.yml` nor `pr-quality-gate.yml`; both run
only `typecheck`, `lint`, `test:quick`, `specs:behavior:coverage`, `compat:min-version`, and
`specs:structure-validation`. (`libs/web-ui`'s `test:e2e` is a no-op `echo` stub.) So the remedy the
plan was built to choose between — a `!process.env.CI` gate — would guard a code path CI never
reaches. The residual risk here is strictly local: a developer or agent with a stray Storybook on
port 6006 gets a visual-regression run against the wrong build.

The interesting leftover is not the config the plan named. It is the two app configs, which have no
`webServer` at all and therefore test whatever happens to answer on ports 19310/19320 with no
`webServer.command` to skip and no signal that anything was substituted — the same hazard class, one
layer lower, and not covered by any `reuseExistingServer` rule.

## Why now

Not urgent, and deliberately less urgent than the original plan assumed. Nothing is broken in CI, and
the one real instance is behind a target CI does not run. The reason to keep the brief alive rather
than discard it is cheapness and timing: the correct moment to decide the standing rule is **before**
BeaverNest grows its next `*-e2e` project or wires `test:visual` into a workflow, either of which
would turn a one-line local convenience into a real gate question. Recording the re-derivation now
also stops a future reader from re-running the same seven-app search and re-discovering that all
seven apps are gone.

## Prior art / precedents

- **Playwright configuration reference** — the repo's own authoritative notes on `webServer` and
  config structure; the natural home for any documented caveat this brief produces.
  [configuration](../../../docs/explanation/software-engineering/automation-testing/tools/playwright/configuration.md)
- **Playwright best practices** — where a "never reuse a server you did not start" convention would sit
  alongside the existing guidance.
  [best-practices](../../../docs/explanation/software-engineering/automation-testing/tools/playwright/best-practices.md)
- **`ci-checker`** — the existing agent that already validates projects against mandatory Nx targets,
  E2E pairing, and env-variable compliance; the obvious host for an automated guard if one is warranted.
  [ci-checker](../../../.claude/agents/ci-checker.md)
- **`nx-affected-cross-worktree-contamination` two-pager** — same underlying class: ambient
  machine-local state (there, uncommitted WIP; here, a stray listening process) silently changing what
  a nominally hermetic run actually exercises.
  [nx-affected-cross-worktree-contamination](https://github.com/wahidyankf/ose-public/blob/main/plans/ideas/q2-not-urgent-important/nx-affected-cross-worktree-contamination.md)
- **`acceptance-clause-vacuity` two-pager** — directly relevant to the trap this re-derivation caught:
  gating a config path that never executes produces a check that cannot fail and certifies nothing.
  [acceptance-clause-vacuity](https://github.com/wahidyankf/ose-public/blob/main/plans/ideas/q1-urgent-important/acceptance-clause-vacuity.md)

## Proposed direction (sketch)

Three small, independent moves, in increasing cost:

- **Change the one config.** Set `libs/web-ui/e2e/playwright.config.ts` to
  `reuseExistingServer: !process.env.CI`. Cheap and harmless, but be honest that it is defensive
  future-proofing rather than a fix — CI does not run `test:visual` today, so the gate changes no
  current behaviour.
- **Write the caveat where it will be read.** Add a short note to the Playwright configuration or
  best-practices doc: a reused server is never verified to match the config that would have started
  it, so treat an unexplained wall of e2e failures as a "check what is on the port" signal first.
- **Decide whether a guard is warranted at all**, and if so, scope it wider than the original plan did
  — covering both `reuseExistingServer: true` and a `webServer`-less config pointed at a fixed
  localhost port, since the latter is the shape this repo actually has two of.

## Rough scope & non-goals

In scope: `libs/web-ui/e2e/playwright.config.ts`'s `reuseExistingServer` setting; a documented caveat
for local e2e runs; a decision on whether an automated guard is worth building and what shape it takes;
and — new to this re-derivation — whether the two `webServer`-less app e2e configs deserve the same
treatment.

Out of scope (carried forward verbatim from the source plan, plus one addition):

- Any change to the e2e test scenarios or assertions themselves.
- Re-litigating the already-fixed `ayokoding-www-tools-ai-benchmark` incident this was filed from.
- The six deleted apps' configs, moot now that those apps are gone.
- **Added here**: wiring `libs/web-ui`'s `test:visual` into CI. That is a separate decision with its own
  cost, and this brief must not smuggle it in as a side effect of a config gate.

## Risks & open questions

- Is a `!process.env.CI` gate on a config CI never runs worth landing at all, or does it just add an
  unexercised branch that reads as protection without providing any? (open)
- Should the two `webServer`-less app configs gain their own `webServer` blocks so that an e2e run
  provably starts the build it tests, or is the externally-started-server model (`WEB_BASE_URL`,
  `API_BASE_URL`, `run-e2e.sh`) a deliberate choice that a `webServer` block would fight? (open)
- Is a guard justified for a repo with three Playwright configs, one of which is affected? The
  guard's value scales with the number of future `*-e2e` projects, which is currently unknown. (open)
- Resolved by this re-derivation, recorded so it is not re-asked: CI runners are GitHub-hosted
  `ubuntu-latest`, ephemeral per job, so the shared-runner port-collision scenario the source plan's
  Phase 1 was built to investigate does not apply to this repo.

## What success looks like + promotion signal

Success is modest and mostly documentary: no Playwright config in this repo silently substitutes an
unverified server for the build under test, and the reasoning above is written down where the next
person to hit a wall of unexplained e2e failures will find it.

**Promotion signal** — promote to a full `backlog/` plan when **either** of these becomes true:

1. A fourth Playwright config lands in this repo, or `libs/web-ui`'s `test:visual` target is wired
   into `main-ci.yml` or `pr-quality-gate.yml` — at which point the gate stops being a no-op and the
   guard question acquires real stakes.
2. A concrete local incident recurs in **this** repo (an e2e or visual run demonstrably exercising a
   stale server), giving the brief a second data point of its own rather than one inherited from a
   deleted app set.

Absent either, the right outcome is the two-line config change plus the doc caveat, landed inline
under the direct-change path — not a plan.
