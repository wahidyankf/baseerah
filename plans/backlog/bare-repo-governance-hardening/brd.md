# Business Requirements — Bare-Repo Governance Hardening

## Business Goal

Make the **bare-repo landing procedure** a written, followable, self-verifying governance document,
so that landing work in `ose-primer` and `ose-infra` stops producing silent ref divergence, stops
producing unreadable `git status` output, and stops inviting agents to ask git the wrong question
about repository topology.

The goal is **not** new capability. It is the removal of a recurring, already-realized failure mode
from a procedure that every parity cycle runs.

## Business Rationale

### The pain, concretely

Two distinct failures occurred on 2026-07-21, hours apart, from the same underlying gap:

1. **Silent ref divergence.** After the Prior-art two-pager landings, `ose-primer` and `ose-infra`
   each sat **4 commits behind / 1 commit ahead** of their own `origin/main` — behind because the
   base-worktree method pushes to origin without touching local `main`, ahead because a duplicate
   commit was then made directly on that stale local `main`. On top of it, roughly a hundred
   uncommitted files, a large share of them **long-lived foreign WIP staged but never committed**
   across many sessions. A naive "commit and push everything" at that moment would have reverted
   newer origin governance content **and** swept another actor's work into an unrelated commit.
   Recovery required a careful, hand-reasoned `reset --soft`.

2. **Topology misread.** A scoping agent concluded `ose-primer`'s `main` was un-merged because it
   ran `git rev-parse --is-bare-repository` from inside a linked worktree, where the documented
   answer is `false`. The agent's question was wrong, not git's answer
   ([F3](./tech-docs.md#research-findings)) — but nothing in any of the three repos told it so.

Neither failure was exotic. Both are the **default outcome** of following an undocumented procedure
correctly.

### Why the fix is documentation, not tooling

Three independent strands converge on "write it down, do not automate it":

- **No hook exists.** git ships **no `post-push` client hook**
  ([S1](./tech-docs.md#research-findings), verified against `githooks(5)`'s full enumerated list).
  `pre-push` fires before the transfer and therefore cannot observe post-push drift. Any lag guard
  would have to be a wrapper script, never a hook — a strictly larger commitment than a documented
  step.
- **No tool sees staleness.** `git diff --cached --exit-code` and `git status --porcelain` report
  **state, not duration** ([S6](./tech-docs.md#research-findings)). Nothing can distinguish "staged
  five seconds ago" from "staged for six weeks", so the WIP-parking rule is inherently a judgment
  call and must be advisory (**DD-2**).
- **Adopting a tool is worse than writing a paragraph.** Every candidate was rejected on inspection
  ([S4](./tech-docs.md#research-findings)); `git-extras`' `git sync` was **disqualified on safety**
  because its shell source runs an unconditional `git reset --hard` plus `git clean -d -f -x` —
  in this plan's own originating scenario it would have destroyed the very WIP the plan protects,
  and it is forbidden outright by the
  [No Destructive Git Operations Convention](../../../repo-governance/development/workflow/no-destructive-git-operations.md).

The honest counter-argument is recorded rather than suppressed: documentation-over-automation has a
real failure mode, articulated by Rahul Garg (Thoughtworks) in _Encoding Team Standards_
(martinfowler.com, published 2026-03-31,
<https://martinfowler.com/articles/reduce-friction-ai/encoding-team-standards.html>, accessed
2026-07-21) — _"A checklist on a wiki depends on someone reading it,
remembering it, and applying it consistently under time pressure"_ — with an explicit variance
threshold: _"Teams of five may not need this. Teams of fifteen almost certainly do."_ [Web-cited]
This repo is a solo maintainer plus agents, sitting well under that threshold, and the piece's frame
is AI-assisted code generation rather than git hygiene, so the principle transfers while the
specifics do not. The trade-off is taken deliberately, not by omission.

## Business Impact

| Dimension               | Before                                                                                       | After                                                                                         |
| ----------------------- | -------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------- |
| Sibling landing         | Tacit procedure; local `main` silently lags after every landing                              | Written procedure whose **terminal step** reconciles local `main`                             |
| Recovery cost           | Hand-reasoned `reset --soft` recovery when divergence is finally noticed                     | Divergence never accumulates, so there is nothing to recover                                  |
| `git status` legibility | Unreadable — landed-and-stale dups mixed with months-old foreign WIP                         | Advisory parking rule keeps the index to genuinely-active work                                |
| Topology questions      | Agents ask `--is-bare-repository` and get a correct answer to the wrong question             | Two prescribed checks, provenance-labelled; the misleading question is explicitly forbidden   |
| Delivery-mode selection | Docs offer `main-to-*` for repos that have no primary checkout, contradicting their own text | The restriction is stated where the modes are listed, and the contradicting option is removed |

## Affected Roles

Solo-maintainer repository — these are hats the maintainer wears and agents that consume the files,
not sign-off parties.

- **Maintainer, landing parity work** — runs the base-worktree method by hand across three repos
  every parity cycle. Primary beneficiary of C1's written procedure.
- **`plan-multi-repo-parity-planning` workflow** — consumes the Delivery Mode table and the
  bare-repo grill question (C3, C4); currently contradicts itself between its own line 202 note and
  its meta-question #1.
- **`plan-idea-promotion-planning` workflow** — already links the phrase "bare-repo git-ops method"
  to a document that does not contain it (C6); the link resolves to real content after this plan.
- **Any scoping or execution agent operating across the three repos** — consumes the bareness-check
  prescription (C1, C6) instead of guessing.
- **`plan-checker` / `repo-rules-checker`** — read the amended conventions when validating future
  plans.

## Success Signals

Deliberately expressed as observable checks or explicit judgment, never as fabricated measurements.

- **Observable** — after any sibling landing performed per C1, the command
  `git rev-list --left-right --count origin/main...main` prints `0` and `0` in that sibling,
  without a manual recovery step. (Note:
  `git status --porcelain=v2 --branch`, which also emits `# branch.ab`, does **not** run in a bare
  repo — [S4](./tech-docs.md#research-findings) — so the `rev-list` form is the portable one.)
- **Observable** — no governance document in any of the three repos offers a `main-to-*` delivery
  mode for a bare repo without stating the restriction.
- **Observable** — every one of the three repos contains a prescribed bareness-verification method,
  and every one of them explicitly forbids `git rev-parse --is-bare-repository` for that purpose.
- **Observable** — the "bare-repo git-ops method" cross-link in `plan-idea-promotion-planning.md`
  resolves to a document that actually defines the method.
- **Judgment call** — a bare sibling's `git status` becomes readable again, in the sense that a
  reader can account for every listed path. This is a legibility claim, not a metric; no
  file-count target is asserted because none was measured against a defined baseline.

## Business-Scope Non-Goals

- **Not** building a lag detector, a WIP-staleness checker, or any `rhino-cli` subcommand
  (**DD-2**; [S1](./tech-docs.md#research-findings), [S6](./tech-docs.md#research-findings)).
- **Not** adopting `git-town`, `git-machete`, Graphite, Jujutsu, `git-absorb`, or `git-extras`
  ([S4](./tech-docs.md#research-findings)).
- **Not** migrating either sibling away from the bare layout.
- **Not** altering what any delivery mode _does_ — only which ones a bare repo can be offered.
- **Not** touching `apps/`, `libs/`, `specs/`, or any executable code path.

## Business Risks and Mitigations

| Risk                                                                                                | Likelihood     | Mitigation                                                                                                                                                                                                         |
| --------------------------------------------------------------------------------------------------- | -------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| The written method is read once and then not followed, and divergence recurs                        | Judgment: real | The terminal reconcile is authored as a **numbered step of the method itself**, not as an appendix — skipping it means visibly skipping a step                                                                     |
| The advisory WIP rule is ignored because nothing enforces it                                        | Judgment: real | Accepted deliberately (**DD-2**); [S5](./tech-docs.md#research-findings) shows the failure is recoverable — `git add`-ed blobs survive `reset --hard` as dangling objects within `gc.pruneExpire`'s 2-week default |
| The three repos drift apart again after propagation                                                 | Judgment: real | Propagation is **in-plan** (**DD-8**) and sequential from a single source of truth, not a follow-up promise                                                                                                        |
| The `core.bare` config read is mistaken for an upstream-prescribed method                           | Low            | **DD-7** requires the derived form to be labelled **derived from documented mechanics, not upstream-prescribed** wherever it appears                                                                               |
| Framing the `--is-bare-repository` behaviour as a git bug propagates a false claim into three repos | Low            | [F3](./tech-docs.md#research-findings) settles it: the behaviour is **documented and intentional**; the rule is framed as scoping semantics                                                                        |
| Documentation-only change is judged insufficient later                                              | Judgment: real | The counter-argument is recorded above with its citation and its threshold, so a future re-open starts from the evidence rather than from scratch                                                                  |

## Related Documents

- [prd.md](./prd.md) — the testable scenarios these business claims imply
- [tech-docs.md](./tech-docs.md) — design decisions DD-1..DD-8 and research findings F1-F4, S1-S8
- [No Destructive Git Operations Convention](../../../repo-governance/development/workflow/no-destructive-git-operations.md)
- [Worktree and Artifact Cleanup Convention](../../../repo-governance/development/workflow/worktree-and-artifact-cleanup.md)
