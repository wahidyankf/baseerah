---
name: web-exploratory-and-usability-test-fixing-planning
title: "web-exploratory-and-usability-test-fixing-planning"
goal: >
  Run spec-aware exploratory testing and spec-blind heuristic-usability testing against the same
  live URL(s) and goal, then synthesize both result sets into one fix-ready plan whose findings
  section keeps the two sources clearly separated (exploratory EWT-### vs usability UWT-###) and
  which carries a tech-docs.md (root-cause + fix approach) and a TDD-shaped delivery.md describing
  how to fix every finding. The deliverable is the plan, never the fixes.
termination: >
  A grill-validated plan exists under plans/in-progress/<identifier>/ containing README.md, brd.md,
  prd.md, findings.md (with separate Exploratory and Usability sections), tech-docs.md, and
  delivery.md, passes plan-quality-gate at strict mode, and is pushed to the requested git target.
  No application or library source under apps/ or libs/ is modified by this workflow.
inputs:
  - name: target-urls
    type: string
    description: >
      One or more live URLs to test (comma-separated). The same set is handed to both testers so the
      exploratory and usability passes judge identical surfaces. The running dev/preview server must
      already be reachable (HTTP 200) before the workflow starts.
    required: true
  - name: testing-goal
    type: string
    description: >
      The shared charter/goal forwarded verbatim to both testers (e.g. "thoroughly test the
      cost-of-living calculator tool page"). Each tester interprets it through its own lens —
      exploratory hunts correctness/spec defects, usability judges first-time-user friction.
    required: true
  - name: plan-mode
    type: enum
    values: [new, merge]
    description: >
      Whether to create a brand-new plan (default) or merge the combined findings into an existing
      plan folder. "merge" requires target-plan-path.
    required: false
    default: new
  - name: plan-identifier
    type: string
    description: >
      Slug for the new plan folder under plans/in-progress/ (no date prefix per Plans convention).
      Default is derived from the target (e.g. "<app>-<feature>-test-fixing"). Ignored when
      plan-mode=merge.
    required: false
  - name: target-plan-path
    type: string
    description: >
      When plan-mode=merge, the existing plan folder under plans/in-progress/ to merge the combined
      findings into. Required when plan-mode=merge; ignored otherwise.
    required: false
  - name: breakpoints
    type: string
    description: >
      Optional comma-separated viewport widths (px) to exercise responsive behaviour. Forwarded to
      both testers. Default is the testers' own standard set (e.g. 320, 375, 768, 1024, 1280, 1440).
    required: false
  - name: locales
    type: string
    description: >
      Optional comma-separated locale path segments to cover (e.g. "en, id"). Forwarded to both
      testers. Default is whatever the testers infer from target-urls.
    required: false
  - name: mode
    type: enum
    values: [lax, normal, strict, ocd]
    description: "Quality threshold for the nested plan-quality-gate. Default: strict."
    required: false
    default: strict
  - name: max-concurrency
    type: number
    description: "Maximum testers/agents run in parallel. Default: 2 (the two testers run concurrently)."
    required: false
    default: 2
  - name: push-target
    type: string
    description: "Git push destination for the finished plan. Default: origin main."
    required: false
    default: "origin main"
outputs:
  - name: plan-path
    type: string
    description: Path to the created or updated plan under plans/in-progress/<identifier>/
  - name: exploratory-findings-count
    type: number
    description: Number of EWT-### findings carried into the combined plan
  - name: usability-findings-count
    type: number
    description: Number of UWT-### findings carried into the combined plan
  - name: final-status
    type: enum
    values: [pass, partial, fail]
    description: Final status after the plan's quality gate
---

# Web Exploratory and Usability Test-Fixing Planning Workflow

**Purpose**: Test a live website from two complementary angles in one pass — spec-aware exploratory
(`web-exploratory-tester`) and spec-blind heuristic-usability (`web-usability-tester`) — then fold
both result sets into a single fix-ready plan whose findings stay attributed to their source and
which spells out, in `tech-docs.md` and a TDD-shaped `delivery.md`, exactly how to fix what was found.

> **The outcome is the plan, not the implementation.** This workflow never edits app/lib source,
> never runs a fix, and never lands behaviour changes. It produces a proposal under
> `plans/in-progress/`. The actual fixes happen later, only after a human reviews the plan and runs
> the [Plan Execution workflow](../plan/plan-execution.md). `delivery.md` becomes the executable
> checklist then, not now.

This is a `planning`-type workflow: a single forward procedure whose terminal deliverable is a plan
document. It is **not** an iterative quality gate over the site.

## Execution Mode

**Agent Delegation (preferred)** — the calling context orchestrates the phases, delegating the two
testing passes to `web-exploratory-tester` and `web-usability-tester` via the Agent tool, running
the synthesis and plan authoring through `plan-maker`, and gating with `plan-checker` / `plan-fixer`.
The human grill checkpoint runs inline so the user's conversation is preserved.

**Manual Orchestration (fallback)** — when those agents are unavailable as delegated agent types,
the assistant executes each phase directly using the testers' and plan agents' documented procedures
with Read/Write/Edit tools.

## When to use

- You have a running site (dev, preview, or production) and want both a correctness sweep and a
  first-time-user usability read, delivered as one actionable fix plan rather than two disconnected
  reports.
- Before hardening a user-facing feature: capture defects and friction together so the fix plan
  addresses both in one delivery checklist.
- To refresh an existing findings plan: re-run both testers and merge the new results into the
  prior plan folder (`plan-mode=merge`).

## Inputs at a glance

| Input              | Required | Default               | Notes                                      |
| ------------------ | -------- | --------------------- | ------------------------------------------ |
| `target-urls`      | yes      | —                     | Same set handed to both testers            |
| `testing-goal`     | yes      | —                     | Shared charter, interpreted per lens       |
| `plan-mode`        | no       | `new`                 | `new` creates a plan; `merge` updates one  |
| `plan-identifier`  | no       | derived from target   | New-plan slug (no date prefix)             |
| `target-plan-path` | no       | —                     | Required when `plan-mode=merge`            |
| `breakpoints`      | no       | testers' standard set | Responsive viewports                       |
| `locales`          | no       | inferred from URLs    | Locale path segments                       |
| `mode`             | no       | `strict`              | Threshold for the nested plan-quality-gate |
| `push-target`      | no       | `origin main`         | Git destination for the finished plan      |

## Phases

### 0. Pre-flight (Sequential)

**Actions**:

- Confirm the `ose-public` working tree is clean (`git status --porcelain` empty).
- Verify every URL in `target-urls` returns HTTP 200 (curl). If the server is down, abort and ask
  the user to start it — the testers cannot run against a dead target.
- Resolve `plan-mode`. For `new`, resolve `plan-identifier` (input, else derive from the target,
  e.g. `ayokoding-www-calc-test-fixing`). For `merge`, require `target-plan-path` to point at an
  existing folder under `plans/in-progress/`; abort if absent.
- Resolve `breakpoints` and `locales` (defaults = testers' own standard coverage).

**Output**: Targets reachable; plan destination resolved.

**On failure**: Dirty tree → ask the user to commit/stash first. Unreachable URL or missing
merge target → abort with a clear message.

### 1. Dual Testing (Parallel, delegated)

Run both testers concurrently against the identical `target-urls` + `testing-goal`, capped at
`max-concurrency` (default 2) per the
[Subagent Orchestration Convention](../../development/agents/subagent-orchestration.md). Both are
**non-destructive / passive** — they read, click, resize, and probe but never mutate server state.

**Agent**: `web-exploratory-tester` — spec-aware. Compares live behaviour against existing
`specs/**` Gherkin; produces a findings catalog `EWT-###` (functional, behavioural-consistency,
UI/UX, responsive, accessibility, URL/IA, passive security) plus spec-gap proposals `SG-###`.

**Agent**: `web-usability-tester` — spec-blind. Deliberately ignores specs/source/mockups; judges
only first-time-user perception against Nielsen's 10 heuristics (0–4 severity), cognitive walkthrough,
information scent, and responsive usability; produces a findings catalog `UWT-###`. Emits no
spec-gaps (proposing spec coverage requires reading the spec, which it refuses).

- **Args (both)**: `target-urls: {input.target-urls}`, `testing-goal: {input.testing-goal}`,
  `breakpoints: {input.breakpoints}`, `locales: {input.locales}`.
- **Output**: Each tester returns its full findings set as structured text (README/brd/prd/findings
  bodies, plus `spec-gaps` for exploratory and `walkthrough` for usability). Subagents cannot write
  under `plans/` directly, so the orchestrator captures the returned text for Phase 2 rather than
  letting each tester file a separate plan.

**Success criteria**: Both testers return a findings catalog (possibly empty).
**On failure**: If one tester fails, continue with the other's results and record the gap prominently
in the combined plan's README; do not silently drop a perspective.

### 2. Synthesis & Plan Authoring (Sequential, delegated)

Compose **one** plan from both result sets. The findings MUST stay attributed to their source — a
reader must always be able to tell an exploratory finding from a usability finding.

**Agent**: `plan-maker` — grills the user (multiple-choice, per the
[Grilling-With-Options Convention](../../development/workflow/grilling-with-options.md)) on scope,
prioritization, and any ambiguous fixes, then authors the plan. Hand it a self-contained brief
containing both testers' returned catalogs and this **required document set**:

- `README.md` — target URL(s)/environment, both testing goals, coverage map, and a combined risk
  summary that labels each top risk `[Exploratory]` or `[Usability]`.
- `brd.md` — business framing: who is affected, cost of the defects + friction, success metrics.
- `prd.md` — personas, user stories, and Gherkin acceptance criteria for the corrected behaviours.
- `findings.md` — **two clearly separated sections**: `## Exploratory findings (EWT-###)` and
  `## Usability findings (UWT-###)`, each with severity and steps-to-reproduce, preserving the
  testers' original IDs. A short cross-reference note flags where an EWT and a UWT describe the same
  underlying defect (e.g. the `html lang="en"` locale issue seen by both) so the same root cause is
  fixed once.
- `tech-docs.md` — root-cause analysis and the chosen fix approach per finding (or per finding
  cluster), naming the affected files/components and the design-system primitives involved.
- `delivery.md` — TDD-shaped delivery checklist (RED/GREEN/REFACTOR per code item, file path +
  verbatim command + acceptance criterion), tagged `[AI]`/`[HUMAN]`, with Phase 0 first and the
  **Specs & Gherkin completeness** coverage steps that fold the exploratory `SG-###` proposals into
  `specs/**` Gherkin (per [feature-change-completeness](../../development/quality/feature-change-completeness.md)).

- **plan-mode=new**: author the full set under `plans/in-progress/<plan-identifier>/`.
- **plan-mode=merge**: integrate the new results into `target-plan-path` by ID continuation (do not
  renumber prior findings); re-verify prior findings as STILL-PRESENT / FIXED and record the result,
  then extend `tech-docs.md` and `delivery.md` to cover the newly added findings.

**Output**: Plan document set written under `plan-path`; `exploratory-findings-count` and
`usability-findings-count` tallied.

### 3. Plan Quality Gate (Nested Workflow)

**Workflow**: `plan/plan-quality-gate`

- **Args**: `scope: {plan-path}, mode: {input.mode}`
- **Output**: `{final-status}`

Iterates `plan-checker` → `plan-fixer` to double-zero at the requested mode, confirming the plan's
requirements completeness, technical clarity, and delivery-checklist executability (including the
TDD shape and specs-coverage steps).

**Success criteria**: `plan-quality-gate` returns `pass`.
**On failure**: If it returns `partial` after max-iterations, surface the residual findings to the
user before pushing.

### 4. Push & Hand-back (Sequential)

- Stage the explicit plan paths and the workflow/governance edits only (never `git add -A`; sibling
  repos carry unrelated WIP). Commit with a Conventional Commit message and push to `push-target`.
- Emit a user-visible summary: `plan-path`, `exploratory-findings-count`, `usability-findings-count`,
  `final-status`, and a reminder that the plan is a **snapshot of the site as tested** — re-run both
  testers if the site changes materially before the plan is executed.

**Output**: `plan-path`, `final-status`, pushed commit.

## Gherkin Success Criteria

```gherkin
Feature: web exploratory and usability test-fixing planning

Scenario: One run produces one combined, source-attributed plan
  Given a reachable live URL and a testing goal
  And the ose-public working tree is clean
  When the workflow runs to completion in plan-mode=new
  Then a plan exists at plans/in-progress/<identifier>/
  And the plan contains README.md, brd.md, prd.md, findings.md, tech-docs.md, and delivery.md
  And findings.md has a separate "Exploratory findings (EWT-###)" section and "Usability findings (UWT-###)" section
  And delivery.md is TDD-shaped with Specs & Gherkin coverage steps
  And the plan passes plan-quality-gate at strict mode
  And no file under apps/ or libs/ source is modified

Scenario: Merge mode extends an existing findings plan
  Given an existing plan folder under plans/in-progress/
  When the workflow runs in plan-mode=merge against that folder
  Then prior findings keep their original IDs and gain a re-verification result
  And new findings are appended by ID continuation
  And tech-docs.md and delivery.md are extended to cover the new findings

Scenario: Unreachable target aborts before testing
  Given a target URL that does not return HTTP 200
  When the workflow starts
  Then it aborts in pre-flight with a message to start the server
  And no plan is authored
```

## Related Documents

- [web-exploratory-tester Agent](../../../.claude/agents/web-exploratory-tester.md) — Phase 1 spec-aware pass.
- [web-usability-tester Agent](../../../.claude/agents/web-usability-tester.md) — Phase 1 spec-blind pass.
- [plan-maker Agent](../../../.claude/agents/plan-maker.md) — Phase 2 synthesis + tech-docs/delivery authoring.
- [Plan Quality Gate workflow](../plan/plan-quality-gate.md) — Phase 3 nested gate.
- [Plan Execution workflow](../plan/plan-execution.md) — runs the plan later, after human review.
- [Feature Change Completeness](../../development/quality/feature-change-completeness.md) — the specs+Gherkin rule the delivery checklist must honour.
- [Plans Organization Convention](../../conventions/structure/plans.md) — in-progress plans use the date-free `<identifier>/` folder form.

## Principles Implemented/Respected

- **[Deliberate Problem-Solving](../../principles/general/deliberate-problem-solving.md)**: Two independent perspectives are gathered and reconciled before any fix is proposed; the plan-maker grill forces explicit scope decisions.
- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**: Findings stay attributed to their source (EWT vs UWT); the fix approach and delivery steps are written down before execution.
- **[Simplicity Over Complexity](../../principles/general/simplicity-over-complexity.md)**: One plan, one delivery checklist — shared root causes are fixed once via the cross-reference note.
- **[Automation Over Manual](../../principles/software-engineering/automation-over-manual.md)**: Testing and authoring are delegated to specialized agents; the gate iterates automatically.
- **[No Time Estimates](../../principles/content/no-time-estimates.md)**: Outcomes, not durations.

## Conventions Implemented/Respected

- **[Workflow Naming Convention](../../conventions/structure/workflow-naming.md)**: Basename `web-exploratory-and-usability-test-fixing-planning` parses as scope=`web`, qualifier=`exploratory-and-usability-test-fixing`, type=`planning`.
- **[Plans Organization Convention](../../conventions/structure/plans.md)**: The plan lands at `plans/in-progress/<identifier>/` with no date prefix.
- **[Feature Change Completeness](../../development/quality/feature-change-completeness.md)**: The delivery checklist carries the specs+Gherkin coverage steps for the exploratory spec-gap proposals.
- **[Subagent Orchestration Convention](../../development/agents/subagent-orchestration.md)**: The two testers run capped at 2 concurrent.
- **[Linking Convention](../../conventions/formatting/linking.md)**: Cross-references use GitHub-compatible markdown links with `.md` extensions.
