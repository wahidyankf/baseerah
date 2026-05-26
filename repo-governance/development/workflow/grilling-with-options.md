---
title: "Grilling-With-Options Convention"
description: >
  Every grill question in plan creation, plan establishment, and plan execution must present
  2-4 concrete options with trade-off descriptions. Open-ended questions without options are
  forbidden.
category: explanation
subcategory: development
tags:
  - planning
  - grill-me
  - user-interaction
  - plan-maker
created: 2026-05-26
---

# Grilling-With-Options Convention

Every question asked during a grill session — whether in pre-write, post-write, or pre-execution
contexts — MUST present 2-4 concrete options with trade-off descriptions. Open-ended questions
without options are forbidden.

## Principles Implemented/Respected

- **[Deliberate Problem-Solving](../../principles/general/deliberate-problem-solving.md)**: Presenting
  concrete options forces the asker to explore the solution space before asking. This prevents lazy
  open-ended prompting and ensures the user receives actionable choices.
- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**:
  Explicit options surface assumptions, trade-offs, and design boundaries that would otherwise remain
  implicit in an open-ended question.
- **[Simplicity Over Complexity](../../principles/general/simplicity-over-complexity.md)**: A bounded
  list of options reduces cognitive load. The user selects from prepared choices rather than having to
  generate an answer from scratch.

## Conventions Implemented/Respected

- **[Convention Writing Convention](../../conventions/writing/conventions.md)**: This document follows
  the standard Purpose / Standards / Examples / Validation structure.
- **[Plans Organization Convention](../../conventions/structure/plans.md)**: This convention serves
  the plan creation lifecycle described there — grilling is the first gate before any plan files
  are written.
- **[Governance Vendor-Independence Convention](../../conventions/structure/governance-vendor-independence.md)**:
  All tool references in this document use vendor-neutral language; platform-specific tooling
  (e.g., `AskUserQuestion`) is referenced only in skill and agent files under `.claude/`.

## Standards

### The HARD RULE

Every grill question MUST present **2-4 concrete options**, each with a brief trade-off description.
Open-ended questions without options are **FORBIDDEN**, no exceptions.

If you cannot enumerate options for a question, read the codebase first and synthesize options from
what you find before asking. A question becomes askable only once options are grounded in repo reality.

### Recommended Option

One option MUST be marked as the recommended choice (e.g., `**(Recommended)**`). This prevents the
user from facing a forced binary choice while still keeping the list bounded.

### Tool Preference

When the coding agent provides an interactive multiple-choice UI (e.g., via the `AskUserQuestion`
tool), use it for each question. The interactive UI shows the user exactly which options are
available and lets them select with a single click. Fall back to the markdown format (see Examples)
only when no interactive selection tool is available.

### One Question at a Time

Ask questions **one at a time** — never bundle multiple questions in a single message. Each question
must be fully resolved before the next is asked.

### Explore Before Asking

Before asking a question that the codebase can answer, read the codebase. Resolving a decision by
reading files instead of asking the user is always preferred. Reserve grill questions for genuinely
ambiguous decisions that only the user can resolve.

### Continue Until Resolved

Continue the grill until all decision branches are resolved. Do not proceed to plan writing,
execution, or implementation while any design decision remains open.

## When This Rule Applies

This convention governs the `grill-me` skill and any context that invokes it:

- **`plan-maker` Step 1 (pre-write grill)**: resolve all open design decisions before reading the
  codebase or creating files.
- **`plan-maker` Step 8 (post-write grill)**: validate the written plan with the user before
  signaling done.
- **`plan-establishment-execution` workflow Step 1 (first grill)**: resolve scope, constraints,
  push target, and definition of done before research.
- **`plan-establishment-execution` workflow Step 3 (post-research grill)**: validate direction after
  research findings are presented.
- **`plan-execution` workflow (pre-execution grill)**: stress-test any unresolved design decisions
  in the plan before execution begins.

## Examples

### Correct: Multiple-Choice Question (markdown fallback)

> **Where should the new convention document live?**
>
> - **Option A**: `repo-governance/conventions/writing/` — groups it with other writing
>   conventions; suits if the rule governs how content is written. — trade-off: less visible to
>   workflow consumers.
> - **Option B**: `repo-governance/development/workflow/` — groups it with process conventions;
>   suits if the rule governs how agents conduct their work. **(Recommended)**
> - **Option C**: `repo-governance/conventions/structure/` — groups it with structural conventions;
>   suits if the rule governs file or folder organisation. — trade-off: weakest fit; grilling is a
>   process, not a structure concern.
>
> **Recommendation**: Option B because grilling is a process the agent follows during plan creation,
> making it a development workflow convention rather than a writing or structure convention.

### Incorrect: Open-Ended Question (FORBIDDEN)

> **Where should the new convention document live?**

No options presented. The user must generate the answer from scratch. This pattern is forbidden.

### Correct: Interactive Multiple-Choice Tool (preferred when available)

When the coding agent supports interactive selection (e.g., via an `AskUserQuestion`-style tool),
use it with 2-4 `options` entries. The platform renders the choices as a single-click selection UI.

## Validation

A grill question is valid when ALL of the following hold:

- [ ] It presents exactly 2-4 concrete options
- [ ] Each option has a trade-off description (even a brief one)
- [ ] One option is marked **(Recommended)**
- [ ] The question addresses exactly one decision
- [ ] Options are grounded in codebase reality (not invented)
- [ ] An interactive multiple-choice tool is used when the coding agent supports it

A grill question is invalid when ANY of the following hold:

- No options are presented (open-ended)
- Only one option is presented (not a real choice)
- More than four options are presented (too many; simplify)
- Options are not grounded in codebase reality
- Multiple decisions are bundled into one question

## Related Documentation

- **[grill-me Skill](../../../.claude/skills/grill-me/SKILL.md)** — The authoritative implementation;
  this convention codifies its HARD RULES at the governance layer
- **[plan-maker Agent](../../../.claude/agents/plan-maker.md)** — Invokes grill-me in Steps 1 and 8
- **[plan-establishment-execution Workflow](../../workflows/plan/plan-establishment-execution.md)** —
  Invokes grill-me in Steps 1 and 3
- **[plan-execution Workflow](../../workflows/plan/plan-execution.md)** — Invokes grill-me before
  execution begins
- **[Plans Organization Convention](../../conventions/structure/plans.md)** — Plan structure this
  grilling process serves
