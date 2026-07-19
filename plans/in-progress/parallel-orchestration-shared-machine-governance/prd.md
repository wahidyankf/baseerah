# Product Requirements — Parallel-Orchestration & Shared-Machine Governance

## Product overview

A governance/documentation change (plus a small CI-config change) delivering these rule outcomes,
propagated identically across the three OSE repositories:

1. **N+1 parallel-orchestration model** replaces the fixed "2 background / 3 total" cap; **default
   N=3 specifically to bound token/compute-budget burn**.
2. **`worktree-to-pr`** reinforced as the default delivery mode and the parallelism mechanism, with
   the **PR** (not just the worktree) sharpened as the independent merge point.
3. **Same-machine, concurrent-actors assumption** made explicit across the orchestration surface.
4. **No-destructive-git-operations** convention (local/shared-machine destructiveness) +
   **worktree-and-artifact cleanup** convention (safe, self-scoped disk hygiene).
5. **DAG-first orchestration**: every non-trivial task list and delivery checklist declares an
   explicit dependency DAG; independent nodes parallelize up to N, dependent nodes serialize.
6. **PR merge preconditions** (hardened done-gate): 3 review cycles + branch up-to-date with latest
   `origin/main` + all PR quality gates green + the surface-conditional tester gates of outcome 11
   run and their defect findings resolved.
7. **main-ci on a 4×/day schedule** (not per-push) in all three repos.
8. **Background-slot preference**: fill background agent slots up to N and keep the main thread as
   vacant/responsive as possible — bounded by the DAG (never force parallelism onto dependent nodes).
9. **Status-update cadence**: while task-list items are active, the orchestrator updates the user
   every 3-5 minutes (not faster), tied to task-list-discipline.
10. **Per-phase PR delivery + feature flags**: decompose plans so each applicable phase / independent
    DAG node lands as its own PR (strict 1-PR ↔ 1-worktree), using feature flags to keep partial work
    merged-but-dark on `main`; inseparable dependent phases stay one PR (DAG governs).
11. **Surface-conditional UI / API tester gates**: UI-bearing plans run both UI gates
    (`ui/ui-quality-gate.md` static components + `web/web-ux-test-fixing-planning.md` running triad);
    API/BE-bearing plans run the **new** `api/api-quality-gate.md`; both → both; neither → an
    **explicitly stated** exemption. Binds during plan creation/update/execution AND as a merge
    precondition. Closes a verified gap: `api-exploratory-tester` exists as an agent but had no
    workflow gating it.

The "product" is the governance text and its wiring (indexes, `AGENTS.md`, `CLAUDE.md`, agent/skill/
workflow surfaces, regenerated bindings) plus the `main-ci.yml` trigger change, consumed by AI agents
and human contributors across all three repos. Governance is written **vendor-neutrally and
capability-gated** so it holds across every supported harness (see the per-harness compatibility
note placeholder in [tech-docs.md](./tech-docs.md)).

## Personas

- **Orchestrator agent (main thread)** — decides how many background agents to run under the N+1
  model and when to raise/lower N.
- **Background subagent** — runs inside its own worktree, bound by N+1 accounting and the
  no-destructive-git + cleanup rules.
- **`repo-rules-maker` (governance author)** — authors and propagates the rule text.
- **Human maintainer** — reads the rules, merges the per-repo PRs, tunes N in practice.

## User stories

- As an **orchestrator agent**, I want a single adjustable N+1 concurrency model so that I can use
  available machine capacity without guessing at an asymmetric fixed cap.
- As a **background subagent**, I want an explicit list of forbidden destructive git operations so
  that I never wipe out a concurrent actor's uncommitted work on the shared machine.
- As a **plan executor**, I want a mandatory, safe cleanup gate so that the worktrees and build
  artifacts my plan created do not fill the shared disk — without ever deleting shared caches.
- As a **governance author**, I want the same rule text in all three repos so that agents behave
  identically regardless of which repo they operate in.
- As a **human maintainer**, I want the same-machine assumption stated explicitly so that all
  orchestration guidance is written to be safe under concurrent, shared-machine work.
- As an **orchestrator agent**, I want to declare an explicit dependency DAG so that I fan out only
  genuinely-independent work and serialize dependent work for safety.
- As an **orchestrator agent**, I want to keep the main thread vacant and push work to background
  slots so that I stay responsive to the user without forcing artificial parallelism.
- As a **human maintainer**, I want a progress update every 3-5 minutes while work is active so that
  I stay informed without being spammed on every micro-event.
- As a **reviewer / merge authority**, I want a PR to merge only when it passed 3 review cycles, is
  up-to-date with latest `origin/main`, and has green gates so that no stale or unreviewed change lands.
- As a **maintainer of shared runners**, I want main-ci to run on a schedule rather than per-push so
  that PR-gated changes do not redundantly burn the self-hosted runners.
- As a **contributor on any harness**, I want the orchestration rules written vendor-neutrally and
  capability-gated so that harnesses without background agents still execute the same DAG serially.
- As a **plan author**, I want each phase / independent DAG node to land as its own PR (one PR ↔ one
  worktree) with feature flags for partial work so that work integrates into `main` early and
  independent phases merge in parallel with minimal conflict surface.

## Acceptance criteria (Gherkin)

Each scenario uses exactly one primary `Given`, one `When`, one `Then`; extras chain via `And`.

### N+1 parallel-orchestration model

```gherkin
Scenario: Concurrency surface states the N+1 model instead of the fixed cap
  Given the governance surface previously stated "background agents cap at 2, 3 total including main"
  When the parallel-orchestration rule is updated in AGENTS.md and the three concurrency conventions
  Then every updated surface states "1 main thread + N background agents = N+1 total, default N=3"
  And no updated surface still asserts a fixed "cap of 2 background / 3 total" as the standing model
  And the mtime/staleness relaunch guidance is preserved
```

```gherkin
Scenario: N is adjustable per-plan and along the way
  Given the N+1 model with default N=3
  When a plan declares a different N, or capacity/pressure changes mid-execution
  Then the governance text permits raising N when independent work and machine capacity allow
  And it permits lowering N under runner or disk pressure
  And it still forbids an agent silently self-promoting beyond the declared N without cause
```

### Worktree-to-PR as the parallelism mechanism

```gherkin
Scenario: worktree-to-pr reinforced as the default and the parallelism mechanism
  Given worktree-to-pr is already the default delivery mode
  When the orchestration rationale is updated
  Then the governance text explains each plan or unit gets its own worktree plus PR
  And it states this isolation is what lets independent work proceed concurrently without collision
```

### Same-machine, concurrent-actors assumption

```gherkin
Scenario: Same-machine assumption is made explicit
  Given orchestration guidance that did not state where concurrent actors run
  When the assumption is added to the orchestration surface
  Then the text states agents, engineers, and processes may run simultaneously on the same machine
  And it states they share the disk, git object store, worktrees, and CI runners
  And it requires task, plan, and execution guidance to be safe under that assumption
```

### No-destructive-git-operations convention

```gherkin
Scenario: Local destructive git operations are forbidden on shared state
  Given a new no-destructive-git-operations convention exists
  When an agent considers a locally destructive git command on the shared machine
  Then the convention forbids reset --hard, checkout -f/--force discarding changes, and clean -fd
  And it forbids branch -D on shared branches, history rewrite on shared branches, worktree remove
    --force on worktrees the agent did not create, work-swallowing stash, and shared-object-store
    pruning while others hold worktrees
  And it requires additive, own-worktree-scoped operations and explicit-path staging instead of
    git add -A
```

```gherkin
Scenario: The local rule complements the existing remote-push safety rule
  Given the existing git-push-safety convention covering force-push and --no-verify
  When the new no-destructive-git-operations convention is added
  Then the new convention cross-links git-push-safety as the remote-side companion
  And it scopes itself to local and shared-machine destructiveness rather than restating push safety
```

### Worktree-and-artifact cleanup convention

```gherkin
Scenario: Plan-end cleanup is mandatory but self-scoped and non-destructive to others
  Given a plan that created one or more worktrees and build artifacts
  When the plan reaches its mandatory cleanup gate
  Then the convention requires removing only the worktrees and artifacts that plan created
  And it requires verifying each artifact is not in use by another session before deleting
  And it forbids deleting shared caches other sessions depend on, naming the shared cargo target
    directory as the canonical example
  And it treats the cleanup gate itself as non-destructive to other actors
```

### DAG-first orchestration

```gherkin
Scenario: Non-trivial work declares an explicit dependency DAG
  Given a non-trivial task list or delivery checklist
  When the orchestration rule is applied
  Then the work declares nodes (tasks/items) and edges (blocks/blockedBy)
  And genuinely-independent nodes run in parallel up to N
  And dependent nodes serialize
  And the DAG's independent-node width is what determines the fan-out, capped at N
```

```gherkin
Scenario: The default N is justified by token/compute budget
  Given the N+1 model with default N=3
  When the rationale for the default is documented
  Then the text states N=3 defaults specifically to bound token/compute-budget burn
  And it states raising N is deliberate and justified by independent work plus capacity plus budget headroom
  And it states lowering N is required under budget, runner, or disk pressure
```

### Per-phase PR delivery + feature flags + 1-PR ↔ 1-worktree

```gherkin
Scenario: A plan is decomposed into per-phase PRs with a strict worktree mapping
  Given a plan with multiple applicable phases or independent DAG nodes
  When the plan is authored under the planning-granularity rule
  Then each applicable phase or independent DAG node is delivered as its own pull request
  And each PR maps to exactly one worktree (one worktree → one branch → one PR → one phase/node)
  And genuinely inseparable dependent phases stay a single PR rather than being force-split
  And the worktree is the unit cleaned up when its PR lands
```

```gherkin
Scenario: Feature flags keep partially-built work merged-but-dark on main
  Given a multi-phase plan where later phases are not yet complete
  When feature-flagging is applied wherever possible
  Then partially-built work is merged to main early behind a feature flag
  And incomplete phases integrate continuously instead of accumulating in a long-lived branch
  And independent phases can review, gate, and merge in parallel with reduced conflict surface
```

### Background-slot preference and status cadence

```gherkin
Scenario: The orchestrator prefers background slots and keeps the main thread responsive
  Given independent work is available under the DAG
  When the orchestrator schedules the work
  Then it fills background agent slots up to N and keeps the main thread as vacant as possible
  And it does not force parallelism onto dependent nodes just to raise utilization
  And the main thread stays responsive to the user as an orchestrator rather than a long-running worker
```

```gherkin
Scenario: Status updates arrive on a bounded cadence
  Given active or open task-list items
  When the orchestrator reports progress
  Then it updates the user every 3 to 5 minutes and no faster
  And it does not emit an update on every micro-event
  And the cadence is tied to the task-list-discipline convention
```

### PR = independent merge point and hardened merge preconditions

```gherkin
Scenario: N parallel units become N independently-mergeable PRs
  Given N parallel units of work under the DAG
  When each unit that produces changes is delivered
  Then each unit gets its own worktree and its own PR
  And the PRs review, gate, and merge independently without blocking each other
  And the governance text names the PR (not just the worktree) as the parallelism enabler
```

```gherkin
Scenario: A PR merges only when all hardened preconditions hold
  Given a PR under a *-to-pr delivery mode
  When merge is considered
  Then the PR must have passed the pr-review-maker to pr-review-fixer cycle for 3 cycles
  And the branch must be up-to-date with the latest origin/main at merge time
  And all PR quality gates must be green
  And a branch that is behind origin/main is brought up-to-date non-destructively before merge
  And the surface-conditional UI/API tester gates have been run with their defect findings resolved
```

### Surface-conditional UI / API tester gates

```gherkin
Scenario: A UI-bearing plan runs both UI gates at authoring and at merge
  Given a plan that adds or changes user-facing screens under apps/ or libs/web-ui
  When the plan is created, updated, or executed
  Then the delivery checklist runs ui/ui-quality-gate.md for the built components
  And it runs web/web-ux-test-fixing-planning.md for the running UI triad
  And the same gates are re-asserted as a pre-merge precondition on the PR
```

```gherkin
Scenario: An API-bearing plan runs the new API quality gate
  Given a plan that adds or changes a REST or GraphQL endpoint in a backend app
  When the plan is created, updated, or executed
  Then the delivery checklist runs repo-governance/workflows/api/api-quality-gate.md
  And that workflow drives api-exploratory-tester against a live endpoint with the contract as ground truth
  And the gate is re-asserted as a pre-merge precondition on the PR
```

```gherkin
Scenario: A plan touching neither surface records the exemption explicitly
  Given a pure docs or governance plan touching no UI and no API
  When the plan documents its applicable gates
  Then tech-docs.md states the UI and API gate exemption explicitly
  And the exemption is never left implicit by silent omission
```

```gherkin
Scenario: The missing API workflow half is created and registered
  Given repo-governance/workflows/api/ does not exist while api-exploratory-tester does
  When the plan executes its Delta 11 sub-block
  Then repo-governance/workflows/api/api-quality-gate.md and api/README.md exist in all three repos
  And the new workflow declares a max-concurrency input consistent with the N+1 alignment
  And repo-governance/workflows/README.md registers the api category alongside ui
```

```gherkin
Scenario: The three UI-related gates are documented as complementary, not interchangeable
  Given plan-checker Step 5k, ui/ui-quality-gate.md, and the web tester triad all concern UI
  When the governance text describes them
  Then Step 5k is stated to gate the UI design funnel in prd.md before anything is built
  And ui-quality-gate.md is stated to gate the built components statically without a browser
  And web-ux-test-fixing-planning.md is stated to gate the running UI in a browser
  And no gate is presented as a substitute for another
```

### main-ci on a schedule

```gherkin
Scenario: main-ci runs on a schedule instead of per-push
  Given main-ci.yml currently triggers on push to main
  When the trigger is changed in all three repositories
  Then main-ci triggers only on a 4-times-daily schedule and workflow_dispatch
  And the push-to-main trigger is removed
  And actionlint reports the workflow as valid
```

### Scope sweep across agents, skills, and workflows

```gherkin
Scenario: Related agents, skills, and workflows are swept for stale orchestration text
  Given agent, skill, and workflow files may reference the old cap numbers or orchestration rules
  When the scope sweep runs
  Then every .claude/agents/*.md, .claude/skills/*/SKILL.md, and repo-governance/workflows/** file
    referencing the old cap, orchestration, worktree, git-safety, or cleanup rules is updated
  And the .opencode/ and .amazonq/ mirrors are regenerated via npm run generate:bindings
  And a completeness gate confirms no stale reference remains
```

### Complete plan-workflow and max-concurrency coverage

```gherkin
Scenario: Every plan workflow file is reviewed and updated
  Given repo-governance/workflows/plan/ contains seven files
  When the cross-surface sweep runs
  Then all seven files are updated to the new orchestration model
  And multi-plans-execution.md no longer asserts its superseded "cap 3 concurrent / background cap 2" language
  And plan-quality-gate.md carries both the aligned max-concurrency default and the hardened merge preconditions
  And both multi-repo-parity workflows reflect worktree-to-PR, per-phase PR, and the parallel propagation shape
```

```gherkin
Scenario: The repo-wide max-concurrency input is aligned without breaking a deliberate serialization
  Given twenty workflow files carry a max-concurrency frontmatter input
  When the sweep aligns them with the N+1 model
  Then the nineteen files defaulting to 2 reference the N+1 model rather than a bare fixed cap
  And web-ux-test-fixing-planning.md remains at Default 1 with a justification citing DAG-governed serialization
  And a repo-wide grep for the superseded cap phrasing returns zero unannotated hits
```

### Vendor-neutral, capability-gated orchestration

```gherkin
Scenario: Orchestration rules hold across harnesses of differing capability
  Given harnesses that differ in background-agent support
  When the orchestration governance is written
  Then it is expressed vendor-neutrally and capability-gated
  And where a harness supports background agents it fans out to N
  And where a harness does not, it executes the same DAG serially
```

### Tri-repo propagation

```gherkin
Scenario: Identical rule text lands in all three repositories
  Given ose-public is authored first as the source of truth
  When the rules are propagated to ose-primer and ose-infra
  Then the concurrency model, no-destructive-git, and cleanup rule text are identical across repos
  And apps/rhino-cli/** and the rhino gherkin tree are left untouched by this change
```

### Self-consistency (the plan obeys its own rules)

```gherkin
Scenario: The plan obeys the rules it introduces
  Given this plan introduces non-destructive git, explicit-path staging, and self-scoped cleanup
  When the plan is executed
  Then no delivery step uses a forbidden destructive git operation
  And every staging step stages explicit paths rather than git add -A
  And the final cleanup gate removes only this plan's own worktrees and self-created artifacts
```

## Product scope

**In scope**: all rule outcomes above (N+1 + DAG + background-slot preference + status cadence + PR
merge preconditions), their wiring into `AGENTS.md` / `CLAUDE.md` / the workflow, agents, and practice
indexes, a sweep of related `.claude/agents/*.md`, `.claude/skills/*/SKILL.md`, and
`repo-governance/workflows/**` (including `pr-review-quality-gate.md`), the `main-ci.yml` trigger
change in all three repos, regenerated `.opencode/` + `.amazonq/` bindings, and tri-repo propagation.

**Out of scope**: automated enforcement tooling for the new rules, app/lib code, rhino-cli surfaces,
`.github/workflows/**` job content beyond the `main-ci.yml` trigger (ose-infra keeps its coralpolyp
jobs), and any convention not required by the outcomes.

## Product-level risks

- **Over-broad reading of "no destructive git"** blocking legitimate self-teardown — mitigated by
  scoping the ban to shared/others' state and permitting own-worktree additive ops.
- **Cleanup deleting a shared cache** — mitigated by the verify-not-in-use gate and the explicit
  shared-cargo-`target/` carve-out.
- **Stale numbers left behind** in a surface not enumerated — mitigated by a grep sweep for the old
  "cap of 2"/"3 total" phrasing plus the agents/skills/workflows completeness gate in each repo.
- **Forced parallelism** on dependent work — mitigated by the DAG governing fan-out; "maximize
  background utilization" is explicitly bounded by real node independence, not artificial splitting.
- **Update-storming** the user — mitigated by the 3-5 minute bounded status cadence (not faster).
- **Removing push-CI leaves main unguarded** — mitigated by the fact that every change is already
  PR-gated under `worktree-to-pr`; the 4×/day schedule is a health-check safety net, not the primary gate.
- **Harness incompatibility** — mitigated by vendor-neutral, capability-gated wording; a web-researcher
  is separately confirming expressibility across all supported harnesses (see the tech-docs placeholder).
