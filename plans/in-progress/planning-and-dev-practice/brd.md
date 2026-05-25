# Business Requirements — Planning and Dev Practice Improvement

## Problem

Three gaps in current development practice:

1. **Planning without stress-testing**: Plans get created and immediately executed without structured
   interrogation. Unresolved design branches surface during implementation, causing rework.
2. **Inconsistent TDD discipline**: Code delivery steps in plans don't always prescribe test-first
   cycles. Tests are sometimes written after code, reducing their value as design instruments.
3. **Harness-neutrality blind spot in plan quality gate**: The plan quality gate does not check
   whether plans introducing agents, skills, rules, or governance changes are harness-neutral.
   Vendor-specific assumptions can slip through undetected and lock the repo to a single coding
   harness.

## Business Rationale

### Grill-Me

When planning sessions are relentlessly questioned before implementation begins, design flaws
surface cheaply. The `grill-me` skill
[Web-cited: https://github.com/mattpocock/skills/blob/main/skills/productivity/grill-me/SKILL.md,
accessed: 2026-05-25, excerpt: "Ask the questions one at a time."]
provides a structured interview process: one question at a time, choices presented with
recommendations, decision tree walked to full resolution.

Presenting choices as multiple-option questions with trade-offs (aligned with Claude Code's
`AskUserQuestion` pattern) reduces ambiguity and forces explicit trade-off thinking before any code
is written. The `brainstorming` skill from
[obra/superpowers](https://github.com/obra/superpowers)
[Web-cited: https://github.com/obra/superpowers, accessed: 2026-05-25,
excerpt: "prefer multiple choice, YAGNI ruthlessly"] reinforces this with its
"prefer multiple choice, YAGNI ruthlessly" approach.

Grill-me applies whenever planning is underway — at plan creation, design review, or any moment
when the user requests interrogation. It is a complement to existing planning workflows
[Repo-grounded: `repo-governance/workflows/plan/plan-execution.md`], not a replacement.

### TDD Mandate

RED-GREEN-REFACTOR
[Repo-grounded: `repo-governance/development/workflow/test-driven-development.md`] produces code
that is testable by design, minimal, and understood before written. Making TDD mandatory in
delivery checklists enforces this discipline at the plan level, not just the coding level. The
`test-driven-development` skill from obra/superpowers
[Web-cited: https://github.com/obra/superpowers/blob/main/skills/test-driven-development/SKILL.md,
accessed: 2026-05-25, excerpt: "NO PRODUCTION CODE WITHOUT A FAILING TEST FIRST"] captures this as
the "Iron Law": no production code without a failing test first.

### Harness-Neutral Plan Quality Gate

The repo operates across multiple AI coding harnesses (Claude Code, OpenCode, Amazon Q, Codex,
Cursor, and others) [Repo-grounded: `repo-governance/conventions/structure/multi-harness-binding.md`].
When a plan introduces or modifies agents, skills, rules, or governance documents, the plan quality
gate should verify that those changes follow harness-neutral conventions — no Claude-Code-only
syntax in shared governance, skill body in harness-independent markdown, agent mirrors generated
correctly. This closes the harness-neutrality blind spot in `plan-quality-gate.md`.

### Propagation to Repo Rules

Adding new skills and planning conventions requires propagating updates across related governance
documents, agent definitions, workflow documentation, and rules — so the convention is coherent
across the repo's six-layer governance hierarchy
[Repo-grounded: `repo-governance/repository-governance-architecture.md`].

## Business Impact

### Pain Points

- **Design flaws surfacing late**: Plans are executed without structured interrogation, causing rework
  when unresolved design branches emerge during implementation.
- **Inconsistent TDD discipline**: Code delivery steps do not mandate test-first cycles, reducing
  tests' value as design instruments and increasing post-hoc test writing.
- **Harness-neutrality blind spot**: The plan quality gate has no check for vendor-specific
  assumptions in plans that touch agents, skills, or governance docs.

### Expected Benefits

- **Cheaper defect detection**: Stress-testing plans before execution surfaces design flaws at
  planning cost, not implementation cost.
- **Higher plan executability**: Delivery checklists with explicit RED-GREEN-REFACTOR cycles are
  unambiguous for execution-grade agents and developers.
- **Multi-harness safety**: The harness-neutrality check closes the gap that allows vendor-specific
  content to slip through the quality gate undetected.

## Non-Goals

This plan explicitly does not address:

- **Automated TDD enforcement tooling**: Enforcement is convention-based only; no linter or
  pre-commit hook enforces the RED-GREEN-REFACTOR shape in delivery checklists.
- **Enforcement outside plan checklists**: TDD mandate applies to plan delivery steps only; it
  does not change how developers write code outside a plan context.
- **Changes to harness bindings beyond governance docs**: No changes to `.opencode/`, `.amazonq/`,
  or other secondary bindings beyond what `repo-rules-maker` propagates naturally.
- **Retrospective updates to plans in `done/` or `backlog/`**: Only in-progress and future plans
  are affected.

## Risks

| Risk                                                              | Likelihood | Impact | Mitigation                                                                                                     |
| ----------------------------------------------------------------- | ---------- | ------ | -------------------------------------------------------------------------------------------------------------- |
| Grill-me skill created but not invoked in practice                | Medium     | Low    | Skill frontmatter uses broad trigger phrases; plan-execution.md will reference it explicitly                   |
| TDD mandate misapplied to non-code steps (doc edits)              | Low        | Low    | Convention text explicitly exempts non-code steps; plan-checker validates TDD shape only for code steps        |
| Harness-neutrality check generates false positives on valid plans | Low        | Medium | Check is conditional and scoped to plans touching agents/skills/governance; plan-fixer handles false positives |
| Governance propagation via repo-rules-maker incomplete            | Medium     | Medium | Step 3.3 (repo-rules-quality-gate) verifies coherence post-propagation; must pass before work is done          |
| Changes revert without updating this plan                         | Low        | Low    | Plan archived to `done/` only after all delivery items are ticked and CI passes                                |

## Goals

1. `grill-me` skill file exists and activates on trigger words and planning contexts
2. Skill presents questions as multiple-choice options, marks recommended answers, explores
   codebase before asking answerable questions
3. All code delivery steps in plan checklists express cycles as RED → GREEN → REFACTOR
4. `repo-governance/workflows/plan/plan-quality-gate.md` includes a harness-neutrality check
   step for plans that touch agents, skills, rules, or governance docs
5. All related governance documents, agent definitions, and workflow docs updated consistently
6. Repo rules are coherent after changes (repo-rules-quality-gate passes)

## Success Criteria

- `.claude/skills/grill-me/SKILL.md` exists with correct frontmatter and choice-format body
- Skill activates, asks one question at a time, presents 2-4 options, marks recommendation
- `plan-quality-gate.md` has a harness-neutrality check step (CRITICAL finding if plans touch
  agents/skills/rules without checking vendor-independence)
- All related `.md` files updated (AGENTS.md, workflows, docs, rules, skill references)
- `npm run lint:md` passes with zero violations
- `repo-rules-quality-gate` passes with zero CRITICAL/HIGH findings
- [Judgment call] Future plans reviewed via the grill-me skill before execution encounter fewer
  mid-execution design pivots, as design branches are resolved at planning time rather than
  implementation time

## Affected Roles

- **Primary**: Development team (all agents and human developers)
- **Secondary**: Plan reviewers (`plan-checker`, `plan-execution-checker`, `repo-rules-checker`)
