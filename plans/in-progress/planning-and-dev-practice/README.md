# Planning and Dev Practice Improvement

**Status**: In Progress
**Created**: 2026-05-25
**Priority**: MEDIUM
**Scope**: Improve planning quality and development discipline across skill, governance, and
workflow layers

## Summary

Three improvements to planning and development practice:

1. **Grill-Me Skill** — a structured interrogation skill that stress-tests plans by asking one
   focused question at a time, presenting choices like Claude Code's `AskUserQuestion`, and walking
   down the decision tree until shared understanding is reached. Adapted from
   [mattpocock/skills grill-me](https://github.com/mattpocock/skills/blob/main/skills/productivity/grill-me/SKILL.md)
   with inspiration from [obra/superpowers](https://github.com/obra/superpowers) brainstorming and
   writing-plans skills.

2. **TDD Mandate** — formalize RED-GREEN-REFACTOR as the required shape for all code delivery steps
   in plan checklists, making test-first discipline explicit at the plan level.

3. **Harness-Neutral Plan Quality Gate** — extend `repo-governance/workflows/plan/plan-quality-gate.md`
   to check harness-neutrality when a plan touches agents, skills, rules, or governance docs, ensuring
   no vendor lock-in is introduced by plan execution.

## Documents

| Document                     | Purpose                                      |
| ---------------------------- | -------------------------------------------- |
| [brd.md](brd.md)             | Business rationale and goals                 |
| [prd.md](prd.md)             | Requirements and Gherkin acceptance criteria |
| [tech-docs.md](tech-docs.md) | Technical design and skill file content      |
| [delivery.md](delivery.md)   | TDD-shaped delivery checklist                |

## Worktree

**Path**: `worktrees/planning-and-dev-practice/` — see [delivery.md §Worktree](delivery.md#worktree)
for the provisioning command and convention references.
