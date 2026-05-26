# Product Requirements Document

## Product Overview

Governance rule updates to existing files that tighten plan-creation and plan-execution workflows,
plus a new `plan-establishment` workflow that orchestrates the full prompt-to-pushed-plan lifecycle,
markdown linter archive exclusions, harness-neutrality authoring reminders, and a new
`repo-setup-manager` agent that standardizes Phase 0 across all plans.

## Personas

- **Plan executor** (AI orchestrator following `plan-execution` workflow)
- **Plan author** (AI `plan-maker` agent)
- **Maintainer** (human reviewing plans and execution output)

## User Stories

**Worktree auto-provisioning**:

As a plan executor,
I want the worktree to be provisioned automatically when I detect a CWD mismatch,
So that plan execution does not require a manual user interrupt for a mechanical setup step.

**RED/GREEN/REFACTOR as separate items**:

As a maintainer watching task-list progress,
I want each TDD phase to be its own checkbox,
So that I can see which phase the executor is on and the task list stays 1:1 with delivery.md.

**Mandatory grill before and after plan creation**:

As a maintainer accepting a freshly created plan,
I want to know the plan-maker grilled me before writing and reviewed the plan with me after writing,
So that design decisions are resolved before execution starts.

**Guided prompt-to-plan pipeline**:

As a maintainer with a vague idea for a behavioral change,
I want a single workflow that handles repo exploration, research, grilling, plan writing, quality
validation, and pushing,
So that I do not have to manually coordinate each step or risk shipping an under-researched plan.

**Archive exclusion for markdown link checking**:

As a maintainer pushing commits that touch active files,
I want the markdown link checker to skip `plans/done/` and `archived/`,
So that stale links in frozen historical content do not block my push.

**Harness-neutrality awareness at authoring time**:

As a plan author,
I want plan-establishment and plan-maker to remind me of harness-neutrality constraints when my
plan touches agents, skills, or `repo-governance/` paths,
So that I don't unknowingly introduce vendor-specific content into governance files.

**Standardized Phase 0 in every plan**:

As a plan executor,
I want every delivery checklist to begin with a Phase 0 that installs dependencies, runs doctor,
and resolves all preexisting failures,
So that I start from a clean, known-good baseline before doing any plan work.

## Acceptance Criteria

### AC-1: Worktree Auto-Provisioning

```gherkin
Scenario: CWD does not match declared worktree path
  Given a plan with a valid ## Worktree section declaring worktrees/foo/
  And the orchestrator's CWD is the repo root (not worktrees/foo/)
  When the orchestrator reaches Step 0 of plan-execution
  Then it runs `git worktree add worktrees/foo HEAD` from the repo root
  And runs `npm install && npm run doctor -- --fix` in the root worktree
  And emits a user-visible confirmation line
  And proceeds with plan execution from the worktree path
  And does NOT terminate with fail

Scenario: ## Worktree section is missing from the plan
  Given a plan with no ## Worktree section
  When the orchestrator reaches Step 0 of plan-execution
  Then it terminates immediately with status fail
  And emits the existing missing-section error message
```

### AC-2: RED/GREEN/REFACTOR Separate Items

```gherkin
Scenario: Plan author writes a TDD delivery item
  Given a delivery checklist item that ships code
  When the plan-maker writes the TDD steps
  Then RED is a standalone - [ ] checkbox
  And GREEN is a standalone - [ ] checkbox
  And REFACTOR is a standalone - [ ] checkbox
  And no single checkbox combines two or more TDD phases

Scenario: test-driven-development.md contains the HARD RULE
  Given the updated test-driven-development.md
  When a reader searches for "HARD RULE" in the TDD Shape section
  Then the document explicitly prohibits combining RED, GREEN, REFACTOR into one checkbox
```

### AC-3: Mandatory Grill Protocol in plan-maker

```gherkin
Scenario: plan-maker starts creating a plan
  Given a user request to create a plan
  When plan-maker begins its workflow
  Then it invokes the grill-me skill BEFORE reading the codebase or writing any files
  And resolves all open design-decision branches before proceeding

Scenario: plan-maker finishes creating a plan
  Given all plan files have been written
  When plan-maker completes its workflow
  Then it invokes the grill-me skill AFTER writing all files
  And presents the finished plan for user validation
  And resolves any remaining questions before signalling done
```

### AC-4: plan-establishment Workflow

```gherkin
Scenario: User invokes plan-establishment with a prompt
  Given a user prompt describing a desired behavioral change
  When the orchestrator follows the plan-establishment workflow
  Then it explores the repo for related conventions and prior plans (Step 0)
  And it grills the user to resolve scope, push target, plan identifier, and definition of done (Step 1)
  And if research is needed, it invokes web-research-maker (Step 2)
  And it grills the user again to integrate research findings (Step 3)
  And it invokes plan-maker to create the plan in plans/in-progress/<identifier>/ (Step 4)
  And it reviews the created plan for structural completeness (Step 5)
  And it runs plan-quality-gate at strict mode until pass (Step 6)
  And it commits and pushes to the confirmed target (Step 7)

Scenario: Research is not needed (internal governance change)
  Given a prompt for a purely internal governance change
  And the user confirms in Step 1 that no external research is needed
  When the orchestrator reaches Step 2
  Then it skips Step 2 and emits "Step 2 skipped — no external research needed"
  And Step 3 is a brief confirmation pass, not a full grill session

Scenario: push target is not origin main
  Given the user specifies a non-default push target during the Step 1 grill
  When the orchestrator reaches Step 7
  Then it pushes to the user-confirmed target, not origin main
  And it does NOT re-ask for the push target in Step 7
```

### AC-5: Archive Exclusion for Markdown Link Checking

```gherkin
Scenario: Markdown linter runs on a repo with stale links in plans/done/
  Given links inside plans/done/ reference moved or renamed files
  When npm run lint:md is executed
  Then no violations are reported for files under plans/done/
  And no violations are reported for files under archived/

Scenario: Active files outside archived directories are still checked
  Given a broken internal link in plans/in-progress/
  When npm run lint:md is executed
  Then the violation is reported and the command exits non-zero
```

### AC-6: Harness-Neutrality Awareness at Authoring Time

```gherkin
Scenario: plan-establishment workflow touches agent or skill paths
  Given a plan-establishment run where the scope includes .claude/agents/ changes
  When the orchestrator reaches Step 1 (first grill)
  Then the grill includes a harness-neutrality checkpoint question
  And the workflow documentation reminds authors that governance files must be vendor-neutral

Scenario: plan-maker writes a delivery checklist for a plan touching repo-governance/
  Given a plan whose scope includes repo-governance/ file changes
  When plan-maker writes the delivery checklist
  Then it includes a reminder to run the harness-neutrality scan (plan-quality-gate Step 5g)
  And plan-maker's workflow documentation references the harness-neutrality constraint
```

### AC-7: Standardized Phase 0 in Every Plan

```gherkin
Scenario: plan-maker creates a new delivery checklist
  Given a user request to create a plan
  When plan-maker writes the delivery.md
  Then the first phase is Phase 0: Environment Setup and Baseline
  And Phase 0 contains: npm install, npm run doctor -- --fix, baseline test run, and preexisting
  failure resolution steps
  And no plan phase work begins before Phase 0 completes

Scenario: repo-setup-manager agent exists
  Given the repo-setup-manager agent definition at .claude/agents/repo-setup-manager.md
  When plan-maker or plan-execution references Phase 0
  Then repo-setup-manager is the designated executor for Phase 0 tasks
  And its definition is synced to .opencode/agents/repo-setup-manager.md
```

## Product Scope

**In-scope features**:

- Updated Step 0 in `plan-execution.md` with auto-provisioning logic
- HARD RULE addition in `test-driven-development.md` TDD Shape section
- Mandatory Step 1 (pre-write grill) in `plan-maker.md` Planning Workflow
- Mandatory Step 8 (post-write grill) in `plan-maker.md` Planning Workflow
- Phase 0 mandate in `plan-maker.md` delivery checklist template
- Summary updates in `AGENTS.md`
- New `repo-governance/workflows/plan/plan-establishment.md` with 8-step prompt-to-pushed-plan lifecycle (including harness-neutrality checkpoint)
- Updated `repo-governance/workflows/plan/README.md` workflow index
- Archive exclusions in `.markdownlintignore` and `.markdownlint-cli2.jsonc`
- Archive exclusion policy documented in `repo-governance/development/quality/markdown.md`
- New `.claude/agents/repo-setup-manager.md` agent definition

**Out-of-scope features**:

- `plan-checker` / `plan-fixer` enforcement of the new TDD rule (future plan)
- Changes to the `grill-me` skill itself
- Broad harness-neutrality audit of all governance files (only plan-related files in scope)

## Product Risks

| Risk                                                                                                                                             | Likelihood | Impact | Mitigation                                                                                                                                  |
| ------------------------------------------------------------------------------------------------------------------------------------------------ | ---------- | ------ | ------------------------------------------------------------------------------------------------------------------------------------------- |
| Mandatory grill steps (Steps 1 + 8 in plan-maker) increase time for simple plans where design is obvious                                         | Low        | Low    | Grill-me resolves in one pass when no open branches exist; not a blocking ceremony                                                          |
| Archive exclusion allows stale links to accumulate silently in `plans/done/` over time                                                           | Medium     | Low    | Archived content is frozen; stale links are expected and acceptable; active content remains fully linted                                    |
| Phase 0 baseline resolution uncovers many preexisting failures, stalling plan execution                                                          | Low        | Medium | Root cause orientation principle already requires resolving preexisting failures; Phase 0 makes this explicit rather than adding new burden |
| `repo-setup-manager` agent is newly defined with no prior usage history; executors may not know when to delegate to it vs. run commands directly | Low        | Low    | Agent description explicitly lists Phase 0 as its scope; plan-execution workflow Step 1b will reference it                                  |
| plan-establishment workflow's two grill sessions (Steps 1 and 3) may feel redundant when research is skipped                                     | Low        | Low    | Step 3 is a brief confirmation pass when Step 2 is skipped, not a full grill                                                                |
