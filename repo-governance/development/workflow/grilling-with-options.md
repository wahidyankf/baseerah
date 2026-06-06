---
title: "Grilling-With-Options Convention"
description: >
  Agents and workflows MUST resolve open design decisions using structured multiple-choice
  questions, not open-ended prose prompts. Every grilling question presents 2-4 concrete
  options, each with its trade-off, and exactly one option marked Recommended. Applies to
  all agent and workflow contexts: plan creation, design review, stress-testing, and
  requirements clarification.
category: explanation
subcategory: development
tags:
  - planning
  - grill-me
  - user-interaction
  - plan-maker
  - design-decisions
  - interaction
  - agents
created: 2026-05-26
---

# Grilling-With-Options Convention

When an agent or workflow must resolve open design decisions with the user — during plan
creation, design review, stress-testing, or requirements clarification — it MUST present
structured multiple-choice questions, not open-ended prose prompts. This convention defines
the required format, mechanism, and scope for all such interactions.

## Principles Implemented/Respected

- **[Deliberate Problem-Solving](../../principles/general/deliberate-problem-solving.md)**:
  Structured options force the agent to understand the design space before asking, and force
  the user to choose deliberately rather than free-associate. Reversible design branches are
  made explicit so users can reason about them.
- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**:
  Each option names its trade-off. The Recommended option names its rationale. Nothing is
  left to the user's imagination or to silent agent inference.
- **[Simplicity Over Complexity](../../principles/general/simplicity-over-complexity.md)**: A
  bounded list of options reduces cognitive load. The user selects from prepared choices
  rather than having to generate an answer from scratch.
- **[Progressive Disclosure](../../principles/content/progressive-disclosure.md)**: Options
  start simple (2-3 choices covering 90% of cases) and expose complexity only when the
  user's prior answer opens a new branch. Unrelated decisions never appear in the same
  question.
- **[Accessibility First](../../principles/content/accessibility-first.md)**: Users are never
  locked into a predefined set — the free-form "Other / write-in" path is always available.
  Questions are self-contained so screen-reader users and harness users without rich
  rendering experience the same choice surface.

## Conventions Implemented/Respected

- **[Convention Writing Convention](../../conventions/writing/conventions.md)**: This document
  follows the standard Purpose / Standards / Examples / Validation structure.
- **[Plans Organization Convention](../../conventions/structure/plans.md)**: This convention
  serves the plan creation lifecycle described there — grilling is the first gate before any
  plan files are written.
- **[Governance Vendor-Independence Convention](../../conventions/structure/governance-vendor-independence.md)**:
  All vendor-specific tool names are confined to the Platform Binding Examples section.
- **[Linking Convention](../../conventions/formatting/linking.md)**: All cross-references use
  GitHub-compatible markdown with `.md` extensions.
- **[Content Quality Principles](../../conventions/writing/quality.md)**: Active voice, single
  H1, proper heading nesting.

## Purpose

Open-ended questions ("What approach do you want?") produce vague answers, require follow-up
clarification, and shift cognitive burden from the agent to the user. Structured
multiple-choice grilling resolves this by:

1. Requiring the agent to explore the repo and enumerate concrete options before asking.
2. Giving the user a small, well-defined decision surface rather than a blank canvas.
3. Making trade-offs explicit so decisions are reversible and auditable in the plan.
4. Using the harness's native interactive tool when available, so options are rendered as
   selectable choices rather than prose the user must parse.

## Scope

### What This Convention Covers

- All grilling interactions during plan creation (pre-write and post-write grill sessions).
- All grilling steps in the plan establishment workflow (Steps 1 and 3).
- Pre-execution grilling of unresolved design decisions before plan execution begins.
- Any "grill me" design-review or stress-testing session invoked explicitly by the user.
- The interaction format, option structure, Recommended marking, and mechanism (native tool
  vs. markdown fallback).

### What This Convention Does Not Cover

- The content of individual grilling questions (domain-specific; defined by the agent or
  workflow that invokes the grill).
- Plan document structure (see
  [Plans Organization Convention](../../conventions/structure/plans.md)).
- Agent file structure and frontmatter (see
  [AI Agents Convention](../agents/ai-agents.md)).
- Commit message format for decisions captured in plans (see
  [Commit Message Convention](./commit-messages.md)).

## Standards

### Rule 1 — Explore Before Asking

Before composing any grilling question, the agent MUST read the relevant repo artifacts. If
the answer to a potential question already exists in a convention file, a plan, or the
codebase, the agent MUST use that information directly and MUST NOT ask the user a question
that a file read could answer.

**Rationale**: Every unnecessary question erodes trust and wastes context. The repo is the
ground truth; the user is the tiebreaker for genuinely ambiguous decisions.

### Rule 2 — Structured Options (2-4, Mutually Exclusive)

Every grilling question MUST present between 2 and 4 concrete, mutually exclusive options.
The options MUST collectively cover the realistic decision space. A free-form write-in path
("Other") is always implicitly available; agents MUST acknowledge it when using the markdown
fallback.

**Rationale**: Fewer than 2 options is a binary yes/no (use a confirmation prompt instead,
not a grill). More than 4 options overwhelms the user and signals the agent has not pruned
the decision space sufficiently.

### Rule 3 — Trade-Off Per Option

Each option MUST state its implication or trade-off in one sentence. The trade-off must be
specific to this decision context, not generic filler ("Option A is simpler").

**Good trade-off**: "Adds a new `development/workflow/grilling-with-options.md` convention
— layer-coherent and matches adjacent workflow docs, but requires updating the development/
README."

**Bad trade-off**: "This option is simpler." (non-specific, not actionable)

### Rule 4 — Exactly One Recommended Option

Exactly one option MUST be marked as Recommended with a one-line rationale grounded in the
specific context (repo state, existing conventions, the user's stated constraints). Marking
more than one option Recommended is forbidden — if two options are genuinely equal, the
agent must choose one based on context.

**Rationale**: An agent that refuses to recommend abdicates its expertise. Users engage with
AI agents precisely to get a grounded recommendation, not just a menu.

### Rule 5 — One Decision Per Question; Batch Only Tightly Coupled Decisions

Each grilling question MUST resolve exactly one decision branch. Tightly coupled decisions
(where the answer to one necessarily constrains the other) MAY be batched in a single
multi-question prompt. Unrelated decisions MUST NOT be bundled — present them as separate
questions, in the same grill session if needed.

**What counts as tightly coupled**: "Which layer should the convention live in?" and "What
filename should it use?" are tightly coupled (the filename depends on the layer choice).
These may appear together.

**What counts as unrelated**: "Which layer?" and "Should we update the README?" are
independent. Present them separately.

### Rule 6 — Mechanism: Native Interactive Tool First, Markdown Fallback

When the AI coding agent harness provides a native interactive multiple-choice question tool,
grilling MUST use it. The native tool renders options as selectable UI elements and returns
the user's choice as structured data, eliminating parse ambiguity.

When no such native tool is available, fall back to inline markdown options in the following
format:

```markdown
**Question**: [Decision to resolve]

- **Option 1 — [Label]**: [Trade-off sentence] _(Recommended — [rationale])_
- **Option 2 — [Label]**: [Trade-off sentence]
- **Option 3 — [Label]**: [Trade-off sentence]
- **Other**: Write in your own approach.
```

The markdown fallback MUST still satisfy Rules 2–5 (2-4 options, trade-offs, one Recommended,
one decision per question).

### Rule 7 — User Can Always Supply an Unlisted Answer

Options are a structured starting point, not a closed cage. The agent MUST treat a user's
write-in answer with the same weight as a listed option. If the write-in answer opens a new
decision branch, the agent grills on that branch before proceeding.

## When This Convention Applies

Grilling MUST follow this convention in all of the following contexts:

| Context                                 | Trigger                                                         |
| --------------------------------------- | --------------------------------------------------------------- |
| **plan-maker pre-write grill**          | Before writing any plan — resolve macro design decisions        |
| **plan-maker post-write grill**         | After writing the plan — validate and stress-test decisions     |
| **plan-establishment-execution Step 1** | First grill: scope, constraints, push target                    |
| **plan-establishment-execution Step 3** | Second grill: post-research validation                          |
| **plan-execution pre-execution grill**  | Unresolved design decisions in the plan before execution begins |
| **"Grill me" design review**            | Any explicit "grill me" invocation by the user                  |

## Examples

### PASS: Well-formed grilling question (markdown fallback)

```markdown
**Question**: Where should the new convention live?

- **Option 1 — `development/workflow/grilling-with-options.md`**: Layer-coherent (HOW we
  interact during development); matches adjacent workflow docs; requires updating
  `development/README.md`. _(Recommended — grilling is an interaction workflow, not a
  documentation-writing rule; the conventions/ README explicitly scopes that directory to
  documentation standards)_
- **Option 2 — `conventions/writing/grilling-with-options.md`**: Co-located with other
  writing conventions; simpler path for writers who look in conventions/ first; but fails
  the layer-coherence test because conventions/ is scoped to documentation rules, not
  development workflows.
- **Option 3 — `development/agents/grilling-with-options.md`**: Groups with AI agent
  standards; appropriate if grilling is agent-only; but grilling also applies to
  human-facing orchestration steps, so agents/ is too narrow.
- **Other**: Specify a different path.
```

### PASS: Interactive multiple-choice tool (preferred when available)

When the coding agent supports interactive selection (e.g., via an `AskUserQuestion`-style
tool), use it with 2-4 `options` entries. The platform renders the choices as a single-click
selection UI and always includes a free-form "Other" path.

### FAIL: Open-ended prose question

```markdown
What approach do you want for the grilling convention?
```

**Problems**: No options, no trade-offs, no Recommended, no structure. Shifts cognitive
burden entirely to the user.

### FAIL: Too many options (more than 4)

```markdown
**Question**: Which layer?

- Option 1 — conventions/writing/
- Option 2 — conventions/structure/
- Option 3 — development/workflow/
- Option 4 — development/agents/
- Option 5 — development/pattern/
- Option 6 — development/quality/
```

**Problem**: Six options signals the agent has not pruned the decision space. Maximum is 4.
Narrow the options first by exploring the repo and applying the layer-coherence test, then
ask.

### FAIL: Trade-offs are non-specific filler

```markdown
- **Option 1**: This is the simpler approach.
- **Option 2**: This is the more flexible approach.
```

**Problem**: "Simpler" and "more flexible" are meaningless without context. Each trade-off
must name the specific structural, maintenance, or governance implication for this decision.

### FAIL: Two options marked Recommended

```markdown
- **Option 1**: … _(Recommended)_
- **Option 2**: … _(Also Recommended)_
```

**Problem**: Exactly one option may be Recommended. If options are genuinely equal, the
agent must choose one and state why.

### FAIL: Unrelated decisions bundled

```markdown
**Question**: Which layer? And also, should we update the README? And what filename?
```

**Problem**: Three independent decisions bundled into one prompt. Present "Which layer?" and
"What filename?" together (tightly coupled). Present "Should we update the README?"
separately.

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

## Special Considerations

### Grilling Within plan-maker

When `plan-maker` is invoked by `plan-establishment-execution`, the macro design decisions
are already resolved by Steps 1 and 3 of that workflow. The `plan-maker` grilling sessions
in that context become **validation passes** for micro-decisions (exact Gherkin phrasing,
section ordering, step granularity). The structured format (Rules 2–5) still applies, but
questions are narrower.

When `plan-maker` is invoked standalone (not via `plan-establishment`), both the pre-write
and post-write grills are full grill sessions resolving all open decisions.

### Grilling Is a Process Artifact, Not a Document Artifact

Grilling produces answers that are captured in plan documents (resolved decisions in
`tech-docs.md`, design-decision lists in the plan-establishment handoff). The grill session
itself does not produce a standalone artifact. Compliance with this convention is verified
primarily by checking that plan-creation workflows and agent definitions reference it — not
by inspecting a generated file.

`repo-rules-checker`'s general cross-reference and consistency validation flags a
plan-creation touchpoint that drops its reference to this convention. The touchpoints
expected to reference this convention are:

- The plan-establishment workflow's Step 1 and Step 3 sections.
- The plan-execution workflow's pre-execution grill section.
- The Plans Organization Convention, where design-decision resolution is discussed.
- The development/README.md index.
- The `plan-maker` agent (pre-write grill step and post-write grill step).
- The `grill-me` skill (canonical implementation of this convention's rules).
- The `plan-creating-project-plans` skill (pre-write and post-write grill gates in the plan
  lifecycle).

## Tools and Automation

- **`grill-me` skill** — The canonical implementation of this convention. The
  [grill-me Skill](../../../.claude/skills/grill-me/SKILL.md) provides the grilling
  service used by `plan-establishment-execution` and `plan-maker`. This convention governs
  the format and mechanism that `grill-me` MUST use. Platform-specific tool invocations
  live in the Platform Binding Examples section below.
- **`repo-rules-checker`** — Its general cross-reference/consistency validation flags a
  plan-creation touchpoint that has dropped its reference to this convention.
- **`repo-rules-fixer`** — Restores missing convention references to touchpoint files when
  flagged.

## Platform Binding Examples

The content under this heading is intentionally vendor-specific. Per the
[Governance Vendor-Independence Convention](../../conventions/structure/governance-vendor-independence.md),
the vendor-audit scanner skips every line under this heading until the next same-level
heading or end of file.

### Primary Coding Harness — Claude Code

Claude Code exposes `AskUserQuestion` as its native interactive multiple-choice tool. When
grilling inside a Claude Code session, the `grill-me` skill MUST invoke `AskUserQuestion`
with:

- `questions`: 1–4 questions (one per tightly-coupled decision cluster)
- `options` per question: 2–4 selectable options (string labels)
- A free-form "Other" option always included as the last option per question

`AskUserQuestion` returns a structured response the agent uses directly without parsing
free-text. Markdown option lists are only a fallback when `AskUserQuestion` is unavailable.

Example invocation shape:

```binding-example
AskUserQuestion({
  questions: [
    {
      question: "Where should the new convention live?",
      options: [
        "development/workflow/grilling-with-options.md  [RECOMMENDED — layer-coherent, matches adjacent workflow docs]",
        "conventions/writing/grilling-with-options.md  [fails layer-coherence; conventions/ is documentation-scoped]",
        "development/agents/grilling-with-options.md  [too narrow; grilling applies beyond agent-only contexts]",
        "Other — specify below"
      ]
    }
  ]
})
```

### Secondary Harness — OpenCode

OpenCode provides an interactive prompt equivalent. When running in an OpenCode session, use
its interactive prompt API with 2-4 selectable options per question. If the interactive
prompt API is unavailable in the current OpenCode version, fall back to the markdown option
format defined in Rule 6.

### All Other Harnesses

Any harness that does not provide a native interactive multiple-choice tool falls back to the
inline markdown format defined in Rule 6. The structured format requirements (Rules 2–5) are
identical regardless of rendering mechanism.

## Related Documentation

- **[grill-me Skill](../../../.claude/skills/grill-me/SKILL.md)** — The canonical
  implementation of this convention; its HARD RULES reflect the standards defined here
- **[plan-maker Agent](../../../.claude/agents/plan-maker.md)** — Invokes grill-me in Steps
  1 and 8
- **[plan-establishment-execution Workflow](../../workflows/plan/plan-establishment-execution.md)**
  — Invokes grill-me in Steps 1 and 3
- **[plan-execution Workflow](../../workflows/plan/plan-execution.md)** — Invokes grill-me
  before execution begins
- **[Plans Organization Convention](../../conventions/structure/plans.md)** — Plan structure
  this grilling process serves; grilling decisions are captured in plan documents
- **[Governance Vendor-Independence Convention](../../conventions/structure/governance-vendor-independence.md)**
  — Vendor neutrality rules; Platform Binding Examples allowlist mechanism
- **[Multi-Harness Binding Convention](../../conventions/structure/multi-harness-binding.md)**
  — Two-tier binding model used by the Platform Binding Examples section
- **[Agent Workflow Orchestration Convention](../agents/agent-workflow-orchestration.md)** —
  How agents plan and execute multi-step tasks; grilling is the pre-execution decision gate
- **[Implementation Workflow Convention](./implementation.md)** — "Make it work" first;
  grilling resolves the design decisions that define what "work" means
