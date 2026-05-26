---
name: plan-establishment-execution
title: "plan-establishment-execution"
goal: >
  Create a well-researched, grill-validated project plan in plans/in-progress/ from a user prompt
  describing a desired behavior or change, then push it to the confirmed target
termination: >
  Plan exists in plans/in-progress/, passes plan-quality-gate at strict mode, and is pushed to
  the confirmed target
inputs:
  - name: prompt
    type: string
    description: Description of the behavior, change, or convention to adopt in the repository
    required: true
  - name: push-target
    type: string
    description: "Git push destination (e.g., 'origin main'). Confirmed in the Step 1 grill if not provided."
    required: false
    default: "origin main"
outputs:
  - name: plan-path
    type: string
    description: Path to the created plan in plans/in-progress/<identifier>/
  - name: final-status
    type: enum
    values: [pass, partial, fail]
    description: Final status after the quality gate
  - name: final-report
    type: file
    pattern: generated-reports/plan__*__audit.md
    description: Final audit report from plan-quality-gate
---

# Plan Establishment Workflow

**Purpose**: Transform a user prompt describing a desired behavior or change into a
production-ready plan in `plans/in-progress/`, validated by `plan-quality-gate` and pushed to
the confirmed target.

**When to use**:

- When the user describes a new behavior, pattern, or convention to adopt in the repository
- When a vague idea needs to become a structured, executable plan
- When research is needed before writing a plan (library versions, best practices, prior art)
- When the user wants the full plan-creation lifecycle orchestrated automatically

## Execution Mode

**Direct Orchestration** — the calling context (the top-level assistant session) is the
orchestrator. It follows this workflow step-by-step: exploring the repo, conducting grill sessions
via the `grill-me` Skill, delegating research to `web-research-maker` and plan writing to
`plan-maker` via the Agent tool, and running the `plan-quality-gate` workflow inline.

Grill sessions run in the calling context (not delegated) so the user's conversation is preserved
across all turns.

```
User: "Establish a plan to [describe desired change]"
```

## Steps

### 0. Prompt Parsing and Repo Exploration (Sequential)

Before any user interaction, understand the current repo state relative to the prompt.

**Orchestrator action**:

1. Parse the prompt: extract the desired behavior, likely affected areas (governance files,
   agents, workflows, apps, libs), and any explicit constraints
2. Explore the repo:
   - Read relevant `repo-governance/` files (conventions, workflows, development practices that
     overlap with the prompt)
   - Search `plans/in-progress/`, `plans/backlog/`, `plans/done/` for related prior plans
   - `Grep` for existing conventions or code that may already address or conflict with the prompt
   - Read `AGENTS.md` for relevant agent and workflow references
3. Build a context summary: what already exists, what gaps remain, what conflicts with the prompt

**Output**: Repo context loaded. Related prior work and conflicts identified.

**Notes**:

- Purely exploratory — no user interaction in this step
- Thorough exploration reduces grill time in Step 1 (pre-read the repo so you can answer "does X
  already exist?" without asking the user)

### 1. First Grill — Scope, Constraints, Push Target (Sequential, Hard Gate)

Invoke the `grill-me` Skill to resolve all open design decisions before research begins.

**Orchestrator action**:

Invoke the `grill-me` Skill (`.claude/skills/grill-me/SKILL.md`). Present Step 0 findings.
Resolve ALL of the following:

1. **Scope**: What is the exact behavior to adopt? What is explicitly out-of-scope?
2. **Affected files**: Which governance files, agents, or workflows will change?
3. **Conflicts**: Does any current convention already address this, conflict with it, or need
   updating?
4. **Constraints**: Backwards compatibility, multi-harness binding implications (if the plan
   touches `.claude/agents/`, `.opencode/agents/`, or `repo-governance/` paths, confirm that
   changes remain vendor-neutral per the
   [Governance Vendor-Independence Convention](../../conventions/structure/governance-vendor-independence.md)),
   tool dependencies
5. **Plan identifier**: What slug should the plan folder use (e.g., `add-foo-convention`)?
6. **Push target**: Confirm where the finished plan should be pushed (default: `origin main`).
   Record — used verbatim in Step 7 without re-asking.
7. **PR vs. direct push**: Is a PR needed, or direct push to `main`?
8. **Definition of done**: What must the finished plan contain for the user to consider it ready?
9. **Research needed**: Are there external claims (library versions, third-party best practices,
   API behavior) that require verification before writing?

**Do NOT proceed to Step 2** until:

- All design-decision branches are resolved
- Push target and plan identifier are explicitly confirmed
- Definition of done is agreed upon
- Whether research is needed is established (determines Step 2 skip condition)

**Output**: Push target confirmed. Plan identifier confirmed. All decisions resolved.
Research-needed flag set.

**On failure to resolve**: Do not proceed. Remain in grill until resolved or user cancels.

### 2. Web Research (Sequential, Conditional)

Delegate external research to `web-research-maker` to verify claims and gather authoritative
sources.

**Skip condition**: Skip if ALL hold:

1. The prompt describes a purely internal governance or structural change with no external claims
2. No library versions, API signatures, tool behavior, or third-party conventions need verification
3. The user confirmed in Step 1 that no research is needed

If skipping: emit `Step 2 skipped — no external research needed (confirmed in Step 1).`

**If NOT skipping**:

Invoke `web-research-maker` via the Agent tool. Provide a focused research prompt covering:

- Best practices or authoritative sources for the proposed approach
- Library or tool behavior referenced in the prompt (versions, API signatures, caveats)
- Prior art: has anyone formalized this pattern? Known failure modes?
- Risks or caveats not mentioned in the prompt

**Agent**: `web-research-maker`

**Output**: Cited, structured research findings. Passed to Step 3 grill and included in the
plan-maker handoff in Step 4.

### 3. Second Grill — Post-Research Validation (Sequential)

Present research findings and grill again to validate direction and close new branches.

**Orchestrator action**:

1. Summarize research findings from Step 2 (or confirm skipped)
2. Invoke the `grill-me` Skill. Cover:
   - Do the research findings change any decision from Step 1?
   - Are there new constraints or trade-offs surfaced by the research?
   - Does the proposed approach still hold after authoritative sources?
   - Are there risks the user wants to explicitly accept or mitigate in the plan?
3. Confirm the updated direction before proceeding

**Do NOT proceed to Step 4** until mutual understanding is confirmed, incorporating research.

**Notes**:

- If research was skipped in Step 2, this is a brief confirmation pass, not a full grill session
- All new branches must be resolved before calling `plan-maker`

**Output**: Final direction confirmed. Research findings integrated into design decisions.

### 4. Plan Creation (Sequential)

Invoke `plan-maker` to write the plan in `plans/in-progress/`.

**Agent**: `plan-maker`

Delegate via the Agent tool. Provide a self-contained handoff prompt containing ALL of:

1. Original user prompt (verbatim)
2. Resolved design decisions from Steps 1 and 3 (numbered decision list)
3. Research findings from Step 2 (cited) — or note that research was skipped
4. Confirmed plan identifier (target folder: `plans/in-progress/<identifier>/`)
5. Confirmed push target
6. Definition of done (from Step 1)
7. **Explicit instruction**: write the plan directly to `plans/in-progress/<identifier>/` — do
   NOT create in `backlog/`. This workflow places plans in `in-progress/` immediately.

**Note on plan-maker's own grill protocol**: `plan-maker` mandates a pre-write grill (Step 1) and
a post-write grill (Step 8). When invoked by `plan-establishment`, these become
**validation passes** — macro-decisions are already resolved. Micro-decisions (exact Gherkin
phrasing, section ordering, step granularity) are still resolved by plan-maker's grills.

**Output**: Plan files created in `plans/in-progress/<identifier>/`.

**On failure**: Terminate with status `fail`. Surface the error.

### 5. Plan Review (Sequential)

Read the created plan files and verify structural completeness before the quality gate.

**Orchestrator action**:

1. Read all plan files in `plans/in-progress/<identifier>/`
2. Verify `## Worktree` section exists in `delivery.md` (multi-file) or `README.md` (single-file)
3. Verify delivery checklist has at least one `- [ ]` checkbox
4. Verify Gherkin acceptance criteria present in `prd.md` (multi-file) or condensed PRD
5. Verify the worktree path in the plan matches `<identifier>` confirmed in Step 1
6. Verify delivery checklist starts with **Phase 0: Environment Setup and Baseline**
7. If structural gaps found: provide a focused prompt to `plan-maker` or fix trivially via `Edit`

**Output**: Plan structurally complete. Ready for quality gate.

**On failure after one retry**: Terminate with status `fail`.

### 6. Quality Gate (Sequential)

Run the `plan-quality-gate` workflow at `strict` mode.

Follow the [plan-quality-gate workflow](./plan-quality-gate.md) with:

- **Input** `scope`: `plans/in-progress/<identifier>/`
- **Input** `mode`: `strict`
- **Output**: `final-status`, `final-report`

**Success criteria**: `plan-quality-gate` returns `pass` (zero CRITICAL/HIGH/MEDIUM on two
consecutive checks).

**On `partial` or `fail`**: Investigate the final report. Apply targeted fixes. Re-run
`plan-quality-gate` up to 2 additional times. If still not `pass`, terminate with status
`partial` and surface the final report.

### 7. Push and Verify (Sequential)

Commit and push the plan to the confirmed target.

**Orchestrator action**:

1. Stage all plan files: `git add plans/in-progress/<identifier>/`
2. Commit: `chore(plans): establish <identifier> plan`
3. Push to the confirmed target from Step 1: `git push <confirmed-target>`
4. Monitor GitHub Actions: `gh run list --limit 5` — verify all triggered workflows complete
   with `completed/success` conclusion
5. If a CI workflow fails: diagnose root cause, fix, push a follow-up commit, re-monitor
6. Emit a user-visible summary: plan path, quality gate status, push target, CI status

**Output**: `plan-path`, `final-status`, `final-report`.

**On push failure**: Surface the error. Do NOT retry automatically — conflicts require human
resolution.

## Principles Implemented/Respected

- **[Deliberate Problem-Solving](../../principles/general/deliberate-problem-solving.md)**:
  Two grill sessions and a research step ensure the plan is built on verified understanding, not
  assumptions
- **[Root Cause Orientation](../../principles/general/root-cause-orientation.md)**: Repo
  exploration in Step 0 prevents duplicating existing conventions and surfaces conflicts early
- **[Automation Over Manual](../../principles/software-engineering/automation-over-manual.md)**:
  The full research → grill → write → validate → push lifecycle is orchestrated without manual
  intervention at each step
- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**:
  Push target, plan identifier, and definition of done are confirmed explicitly in Step 1 before
  any work begins

## Conventions Implemented/Respected

- **[Plans Organization Convention](../../conventions/structure/plans.md)**: Creates plans in
  `plans/in-progress/` with correct identifier format and worktree specification
- **[Governance Vendor-Independence Convention](../../conventions/structure/governance-vendor-independence.md)**:
  Step 1 grill includes an explicit harness-neutrality checkpoint for plans touching agents,
  skills, or `repo-governance/` paths
- **[Web Research Delegation Convention](../../conventions/writing/web-research-delegation.md)**:
  External research delegated to `web-research-maker`
- **[Commit Messages Convention](../../development/workflow/commit-messages.md)**: Conventional
  Commits format in Step 7
- **[CI Post-Push Verification Convention](../../development/workflow/ci-post-push-verification.md)**:
  Step 7 monitors GitHub Actions after push

## Related Workflows

- [Plan Quality Gate](./plan-quality-gate.md) — called in Step 6
- [Plan Execution](./plan-execution.md) — next workflow after plan-establishment

## Related Documentation

- [Plans Organization Convention](../../conventions/structure/plans.md)
- [Governance Vendor-Independence Convention](../../conventions/structure/governance-vendor-independence.md)
- [grill-me Skill](../../../.claude/skills/grill-me/SKILL.md) — Steps 1 and 3
- [plan-maker Agent](../../../.claude/agents/plan-maker.md) — Step 4
- [web-research-maker Agent](../../../.claude/agents/web-research-maker.md) — Step 2
- [repo-setup-manager Agent](../../../.claude/agents/repo-setup-manager.md) — Phase 0 of plans
  created by this workflow
