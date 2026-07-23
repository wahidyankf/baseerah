# Product Requirements — Worktree-to-PR Hardening

## Product Overview

Replace the single `pr-review-maker` agent with a **discipline-specialized reviewer set + a mandatory
coordinator**, wire the set into the existing `pr-review-quality-gate` loop, add a small number of
quality-gate enhancements, and ship a governance convention that makes the reviewer boundaries and
the grey-zone tie-breaker explicit. The product surface is entirely **agent-definition markdown +
governance/workflow markdown + register/binding updates** — there is no runtime code.

## Personas (hats the maintainer wears; consuming agents)

- **The Reviewer-set author** — needs each specialist charter to be crisp and non-overlapping enough
  that the same finding is never independently owned by two specialists.
- **The Synthesizer author** — needs an explicit contract for dedup, re-categorization,
  reasonableness-filtering, and tool-verification, plus a model tier justified against the research.
- **`plan-execution` (consuming agent)** — invokes the review workflow at finalization; needs the
  fan-out to be transparent (it still sees "run the review cycle", now producing one consolidated
  review).
- **`pr-review-fixer` (consuming agent)** — must keep receiving a single consolidated, deduplicated
  finding set, unchanged in shape from what it consumes today.
- **The Maintainer running the cutover** — retires the monolith at cutover and needs post-cutover
  monitoring with a documented rollback trigger, so a regression can be caught and reverted to the
  monolith from git history.
- **The Trunk integrator** — needs merge-precondition (c) to hold under concurrent worktree-to-PR
  merges, which a merge queue provides.

## User Stories

- **US-1** — As the reviewer-set author, I want each specialist reviewer to have a written,
  non-overlapping charter, so that findings are filed by discipline and duplication is minimized.
- **US-2** — As the synthesizer author, I want a mandatory coordinator with an explicit
  dedup/re-categorize/filter/verify contract, so that a naive fan-out does not regress review quality.
- **US-3** — As the workflow author, I want the specialists + coordinator slotted into the existing
  3-cycle loop, so that `pr-review-fixer` and the 5 hardened merge preconditions are preserved.
- **US-4** — As the governance author, I want a written boundary tie-breaker rule, so that grey-zone
  findings (architecture vs. correctness vs. governance) are categorized consistently.
- **US-5** — As the maintainer, I want each new agent registered and mirrored across harnesses, so
  that the bindings never drift.
- **US-6** — As the maintainer running the cutover, I want the monolith retired at cutover plus a
  post-cutover monitoring plan with a rollback trigger, so that a regression is caught and reverted
  rather than silently shipped.
- **US-7** — As the quality-gate owner, I want calibration, selective adversarial verification, and a
  CRITICAL-requires-reproduction rule, so that high-confidence and high-severity findings are
  trustworthy.
- **US-8** — As the trunk integrator, I want a merge queue adopted so that merge-precondition (c)
  holds under concurrent worktree-to-PR merges, not only when PRs merge one at a time.

## Acceptance Criteria

Each acceptance criterion below follows the step-keyword cardinality HARD rule: exactly one primary
`Given`, one `When`, one `Then`; extras chain with `And`/`But`.

### AC-1: Every specialist reviewer has a non-overlapping charter

```gherkin
Scenario: Specialist charters partition the review disciplines
  Given the seven specialist reviewer-maker agent files exist under .claude/agents/
  And a written reviewer-discipline convention defines each discipline
  When a reviewer opens each specialist's "Core Responsibility" section
  Then each specialist names exactly one discipline it owns
  And no discipline is claimed by two specialists
  And each specialist inherits the confidence >= 80 bar, evidence, anti-sycophancy, scope-guard, and untrusted-input rules from the retired monolith
```

### AC-2: The coordinator is mandatory and has an explicit contract

```gherkin
Scenario: The synthesizer consolidates specialist findings before the fixer sees them
  Given the seven specialists have each posted their raw findings for one review cycle
  When pr-review-synthesis-maker runs against those raw findings
  Then it deduplicates overlapping findings into one
  And it re-categorizes any misfiled finding to the correct discipline
  And it drops speculative, nitpick, false-positive, or convention-contradicted findings
  And it re-reads source to tool-verify any finding it is uncertain about
  And it emits exactly one consolidated review for pr-review-fixer to consume
```

### AC-3: The split slots into the existing loop without breaking the merge gate

```gherkin
Scenario: The 3-cycle loop and 5 merge preconditions survive the decomposition
  Given pr-review-quality-gate.md has been revised for fan-out then synthesize then fixer
  When the workflow runs one full cycle against a PR
  Then each cycle fans out to the specialists, synthesizes one review, and hands it to the unchanged pr-review-fixer
  And the 3-cycle hard ceiling with no early exit is preserved
  And the five hardened merge preconditions (a) through (e) remain the merge gate verbatim
```

### AC-4: The boundary tie-breaker rule is documented and applied

```gherkin
Scenario: A grey-zone finding is routed by the written tie-breaker
  Given the reviewer-discipline convention states the tie-breaker rule
  When a new cross-module dependency is reviewed
  Then a violation of an existing layering rule is routed to governance
  And a genuinely novel boundary judgment is routed to architecture
  And the coordinator's re-categorization function is named as the owner of the architecture-versus-correctness boundary
```

### AC-5: Every new agent is registered and mirrored across harnesses

```gherkin
Scenario: Registers and bindings stay in sync after adding the agents
  Given the eight new agent files have been added under .claude/agents/
  When npm run generate:bindings runs and the registers are updated
  Then AGENTS.md and .claude/agents/README.md list all eight new agents
  And the OpenCode and Amazon-Q bindings mirror them
  And the binding sync-validation check passes with zero drift
```

### AC-6: The monolith is retired at cutover with post-cutover monitoring and a rollback trigger

```gherkin
Scenario: Cutover retires the monolith and arms the rollback trigger
  Given the seven specialists and the coordinator are live in the revised workflow
  When the cutover phase removes and de-registers pr-review-maker
  Then the monolith no longer appears in any register or binding
  And a post-cutover monitoring plan defines precision, acceptance-rate, and Outdated-Rate metrics
  And a documented rollback trigger restores the monolith from git history if metrics regress below the rollback bar
```

### AC-7: Quality-gate enhancements are specified

```gherkin
Scenario: Calibration, adversarial verification, and reproduction rules are documented
  Given the quality-gate enhancement section of the convention is authored
  When a reviewer reads the enhancement rules
  Then a periodic confidence-calibration spot-check procedure is defined
  And selective adversarial verification is scoped to high-risk diffs only
  And a rule requires CRITICAL findings to carry a reproduction, not mere agreement-counting
  And the 3-cycle no-early-exit policy carries an explicit rationale flagged as a predictability choice, not a research-derived one
```

### AC-8: A merge queue is adopted to harden precondition (c)

```gherkin
Scenario: A merge queue governs concurrent worktree-to-PR integration
  Given the merge-queue phase has selected a mechanism and updated pr-merge-protocol.md precondition (c)
  When two worktree-to-PR PRs are ready to merge concurrently
  Then the merge queue serializes their integration with CI on the speculative merge result
  And a PR that fails the queued CI is auto-evicted without breaking main
  And precondition (c) is documented as satisfied by the queue rather than by a manual branch-up-to-date check
```

### AC-9: The change set reaches all three sibling repos in parity

```gherkin
Scenario: The identical shared-scaffolding artifacts propagate from ose-public to both downstream repos
  Given the reviewer agents, coordinator, workflow revision, convention, and merge-queue changes have merged to ose-public main
  When the propagation phases deliver the identical change set to ose-primer and ose-infra, each via its own worktree-to-pr cycle with a per-repo generate:bindings step
  Then all three repos carry byte-parity of the shared PR-review agent/governance/workflow scaffolding
  And no rhino-cli file is touched in any repo, preserving the rhino-cli byte-identity boundary
  And no infra-private content is cross-routed out of ose-infra
```

## Product Scope

**In scope (features)**:

- Eight new agent-definition files (seven specialists + one coordinator).
- One new governance convention (reviewer disciplines + tie-breaker + quality-gate enhancements).
- Revision of `pr-review-quality-gate.md` and cross-references in `pr-merge-protocol.md` where the
  reviewer count/shape is described; the monolith retired and de-registered at cutover.
- Register + binding updates.
- A post-cutover monitoring plan with a documented rollback trigger for the monolith.
- Adoption of a merge queue (delivered phase) to harden merge-precondition (c) under concurrent
  integration, updating `pr-merge-protocol.md` precondition (c) and the CI workflow config.
- A future-work section (bot identity, cost budgeting).
- **Three-repo parity propagation**: after `ose-public` (source of truth) merges, the identical
  shared-scaffolding artifacts are delivered to `ose-primer` and `ose-infra`, each via its own
  `worktree-to-pr` cycle with a per-repo `npm run generate:bindings` step (per the multi-repo parity
  workflow).

**Out of scope (features)**:

- Any `apps/`/`libs/` runtime code (this plan is docs + agent definitions only).
- Provisioning a bot/GitHub-App identity.
- Splitting `pr-review-fixer`.

## Product-Level Risks

- **Charter overlap** — if two specialists can each claim a finding, dedup burden shifts entirely to
  the coordinator. Mitigated by the written partition + tie-breaker (AC-1, AC-4).
- **Coordinator single point of failure** — the whole quality bet rides on the synthesizer. Mitigated
  by giving it the top model tier and the tool-verify step, and by post-cutover monitoring + rollback.
- **Immediate retirement risk (no eval gate)** — retiring the monolith at cutover (D2) means a
  regression ships before it is measured. Mitigated by the documented rollback trigger and the
  monolith's recoverability from git history.
- **Binding drift** — eight new agents across three harnesses. Mitigated by the sync-validation gate
  on every agent-touching phase (AC-5).

The **factual claims / judgments** behind these risks live in
[brd.md §Business Risks](./brd.md#business-risks-and-mitigations); the testable scenarios are the ACs
above.
