---
title: "Anti-Patterns in AI Agents Development"
description: ""
category: explanation
subcategory: development
tags: []
created: 2026-05-12
---

# Anti-Patterns in AI Agents Development

> **Companion Document**: For positive guidance on what to do, see [Best Practices](./best-practices.md)

## Overview

Understanding common mistakes in AI agent development helps teams build more maintainable, secure, and effective automation. These anti-patterns cause complexity, security risks, and maintenance burden.

## Purpose

This document provides:

- Common anti-patterns in agent development
- Examples of problematic implementations
- Solutions and corrections for each anti-pattern
- Security and maintenance considerations

## Principles Implemented/Respected

This companion document respects:

- **[Simplicity Over Complexity](../../principles/general/simplicity-over-complexity.md)**: Provides practical examples of simple vs complex approaches
- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**: Makes patterns and anti-patterns explicit through clear examples

## Conventions Implemented/Respected

This companion document supports the conventions in this directory by providing practical examples and guidance.

## Common Anti-Patterns

### Anti-Pattern 1: God Agent

**Problem**: Single agent tries to handle too many responsibilities.

**Bad Example:**

```yaml
---
name: super-agent
description: Validates docs, creates content, deploys apps, manages files, runs tests
tools: [Read, Write, Edit, Glob, Grep, Bash, WebFetch, WebSearch, Task]
---
```

**Solution**: Decompose into focused agents:

```yaml
---
name: docs-checker
description: Validates documentation quality
tools: [Read, Glob, Grep, Write]
---
---
name: docs-maker
description: Creates documentation content
tools: [Read, Write, Glob]
---
---
name: apps-deployer
description: Deploys applications to production
tools: [Bash, Grep]
---
```

**Rationale:**

- Easier to test and maintain
- Clear responsibility boundaries
- Simpler permission model
- Better reusability

### Anti-Pattern 2: Requesting Excessive Tool Permissions

**Problem**: Agent requests tools it does not actually use.

**Bad Example:**

```yaml
---
name: link-checker
description: Validates links in markdown files
tools: [Read, Write, Edit, Glob, Grep, Bash, WebFetch, WebSearch, Task]
# Only needs: Read, Glob, Grep, WebFetch, Write
---
```

**Solution:**

```yaml
---
name: link-checker
description: Validates links in markdown files
tools: [Read, Glob, Grep, WebFetch, Write] # Only what is needed
---
```

**Rationale:**

- Reduces security risk
- Faster user approval
- Clear capability boundaries
- Easier auditing

### Anti-Pattern 3: Vague or Generic Descriptions

**Problem**: Agent description does not clearly communicate what it does or when to use it.

**Bad Example:**

```yaml
---
name: checker
description: Checks things
---
```

**Solution:**

```yaml
---
name: docs-tutorial-checker
description: >
  Validates tutorial quality focusing on pedagogical structure,
  narrative flow, visual completeness, and hands-on elements.
  Use when reviewing tutorial documentation.
---
```

**Rationale:**

- Clear purpose and scope
- Better discoverability
- Users know when to invoke

### Anti-Pattern 4: Hardcoded Paths and Values

**Problem**: Agent has hardcoded paths or values that break when structure changes.

**Bad Example:**

```yaml
---
context: |
  Always write reports to /home/user/repos/project/generated-reports/
  Check files in /home/user/repos/project/docs/
---
```

**Solution:**

```yaml
---
context: |
  Write reports to generated-reports/ (relative to repo root)
  Check files in docs/ directory
  Use Glob to find files dynamically
---
```

**Rationale:**

- Portable across environments
- Works on different machines
- Resilient to restructuring

### Anti-Pattern 5: No Error Handling Guidance

**Problem**: Agent does not document how to handle errors or edge cases.

**Bad Example:**

```yaml
---
description: Processes files and generates reports
# No mention of error handling
---
```

**Solution:**

```yaml
---
description: >
  Processes markdown files and generates reports.
  Handles missing files gracefully with warnings.
  Skips binary files. Creates output directory if missing.
---
```

**Rationale:**

- Clear error behavior
- Graceful degradation
- Better user experience

### Anti-Pattern 6: Missing Tool Usage Documentation

**Problem**: Agent frontmatter does not explain how tools are used.

**Bad Example:**

```yaml
---
name: validator
tools: [Read, Write, Bash, WebFetch]
# No explanation of tool usage
---
```

**Solution:**

```markdown
## Tool Usage

- **Read**: Scan files for validation
- **Write**: Generate audit reports
- **Bash**: Execute git commands for file operations
- **WebFetch**: Verify external references
```

**Rationale:**

- Transparent behavior
- Security clarity
- Easier troubleshooting

### Anti-Pattern 7: Using Wrong Model for Task

**Problem**: Using execution-grade model for simple tasks or fast model for complex reasoning.

**Bad Example:**

```yaml
---
name: simple-link-checker
model: sonnet # Overkill for simple link validation
---
---
name: complex-architectural-analyzer
model: haiku # Insufficient for deep reasoning
---
```

**Solution:**

```yaml
---
name: simple-link-checker
model: haiku # Sufficient for validation
---
---
name: complex-architectural-analyzer
model: sonnet # Needed for deep reasoning
---
```

**Rationale:**

- Cost optimization
- Performance optimization
- Appropriate capability match

### Anti-Pattern 8: No Testing Before Deployment

**Problem**: Deploying agents without testing edge cases and error scenarios.

**Bad Example:**

```markdown
Created new agent, deploying immediately

# No testing performed
```

**Solution:**

```markdown
## Testing Checklist

- [ ] Valid input - passes
- [ ] Invalid input - reports error
- [ ] Empty file - handles gracefully
- [ ] Missing file - reports error
- [ ] Large file - handles pagination
- [ ] Permission denied - reports error clearly
```

**Rationale:**

- Production readiness
- Robust error handling
- Confident deployments

### Anti-Pattern 9: Generic Agent Names

**Problem**: Using non-descriptive agent names that do not indicate purpose.

**Bad Example:**

```
agent1.md
checker.md
validator.md
tool.md
```

**Solution:**

```
docs-tutorial-checker.md
apps-ayokoding-www-deployer.md
plan-execution-checker.md
readme-maker.md
```

**Rationale:**

- Clear categorization
- Easy discovery
- Self-documenting

### Anti-Pattern 10: Enumeration-Based Guards (Denylist Guards That Fail Open)

**Problem**: A safety guard is written as a list of the specific cases it forbids, and is placed in
a section the agent only reaches once it already suspects the hazard. Every axis the guard does not
enumerate is silently permitted, and the guard never fires for an agent that never got to that
section. Each time a hole is discovered, another enumerated clause is appended — and the next
unnamed axis is still open.

**Bad Example** (five consecutive guards, each correct on its own axis, each leaving another open):

```markdown
## Confidence Assessment

...

### Recipe: applying a finding

- Never auto-apply a fix to a step tagged `[HUMAN]`. <!-- axis: tag value -->
- Never DELETE a merge step, only rewrite it. <!-- axis: verb -->
- Never touch merge steps in `*-to-pr` mode. <!-- axis: delivery mode -->
- Never auto-apply at MEDIUM confidence. <!-- axis: confidence level -->
- Never act on a "stale reference" finding here. <!-- axis: finding type -->
```

Nothing in this list protects a merge step against a finding type nobody thought to name — for
example, deletion justified as removing an unverified claim.

**Solution**: Hoist the invariant to the **point of entry** — ahead of every recipe, and wired into
the first assessment step the agent runs — and state it by **what it protects**, not by what it
enumerates:

```markdown
# plan-fixer

## Invariant (read before any recipe below)

The `[HUMAN]` merge gate is the human's sole authority boundary in a `*-to-pr` delivery. NO
finding, of any type, at any confidence, in any delivery mode, may cause this agent to weaken,
delete, retag, or bypass it. Any change that would touch it escalates to the human instead.

...

## Confidence Assessment

1. Re-verify the finding against the current file.
2. Check the change against the Invariant above. If it touches the merge gate — escalate, stop.
3. ...
```

**Rationale:**

- **Placement beats enumeration**: a guard reached only when the hazard was already suspected is
  not a guard. Entry-point placement removes the "did the agent read far enough?" failure mode.
- **Allowlists fail closed and loudly; denylists fail open and silently**. This mirrors established
  security guidance — see the OWASP Developer Guide's security principles (fail securely, positive
  security model) and NIST SP 800-207 / SP 800-167 (deny-by-default policy enforcement).
- **Stated by what it protects**, an invariant covers axes that do not exist yet. Stated by
  enumeration, it covers only the axes already known to have failed.

**Detection heuristic**: if the fix for a guard hole is "add one more clause to the list", the guard
is enumeration-based and the next hole is already open. Rewrite it as a protected invariant instead.

### Anti-Pattern 11: Verification Prompts That Presuppose Their Conclusion

**Problem**: A verification or re-review prompt asserts the answer it wants confirmed. The reviewing
agent, having no license to disagree, manufactures consensus — it finds evidence for the stated
hypothesis and stops looking.

**Bad Example:**

```markdown
The previous fix to `plan-fixer.md` introduced a regression in the merge-gate guard.
Confirm the regression and describe it.
```

**Solution**: State the hypothesis as a hypothesis, explicitly license the negative finding, and
name agreement itself as a failure mode:

```markdown
Hypothesis (may be WRONG — treat it as a lead, not a conclusion): the previous fix to
`plan-fixer.md` introduced a regression in the merge-gate guard.

Investigate independently. Reporting "the hypothesis is wrong, and here is the evidence" is a
FULLY VALID and equally valuable outcome. Reflexive agreement is the failure mode being guarded
against — if the guard is sound, say so and cite why, then keep looking for defects elsewhere.
```

**Rationale:**

- A prompt that presupposes its conclusion measures compliance, not correctness.
- Explicitly licensing the negative finding is what makes an independent verification pass
  independent — observed in practice to redirect a reviewer from a false lead onto a real defect
  elsewhere in the same file.
- This applies to every re-review, self-check, and fixer re-validation prompt, not only to
  formal review cycles.

## Summary of Anti-Patterns

| Anti-Pattern                   | Problem                                       | Solution                                                |
| ------------------------------ | --------------------------------------------- | ------------------------------------------------------- |
| **God Agent**                  | Too many responsibilities                     | Decompose into focused agents                           |
| **Excessive Tool Permissions** | Requesting unused tools                       | Request only necessary tools                            |
| **Vague Descriptions**         | Unclear purpose                               | Clear, actionable descriptions                          |
| **Hardcoded Paths**            | Breaks in different environments              | Use relative paths                                      |
| **No Error Handling Guidance** | Unclear error behavior                        | Document error handling                                 |
| **Missing Tool Usage Docs**    | Unclear how tools are used                    | Document tool usage                                     |
| **Wrong Model Selection**      | Cost/performance mismatch                     | Match model to task complexity                          |
| **No Testing**                 | Production issues                             | Test edge cases before deployment                       |
| **Generic Names**              | Hard to discover and categorize               | Use descriptive, categorized names                      |
| **Enumeration-Based Guards**   | Denylist guard fails open on any unnamed axis | Hoist an invariant to entry, stated by what it protects |
| **Presupposing Verification**  | Prompt asserts its own conclusion             | State a hypothesis; license the negative finding        |

## Related Documentation

- [AI Agents Convention](./ai-agents.md) - Complete agent development standards
- [Best Practices](./best-practices.md) - Recommended patterns
- [Skill Context Architecture](./skill-context-architecture.md) - Skill integration patterns
- [Agent Workflow Orchestration Convention](./agent-workflow-orchestration.md) - How agents plan, verify, and self-improve during multi-step tasks
- [Agents Index](../../../.claude/agents/README.md) - All available agents

## Conclusion

Avoiding these anti-patterns ensures:

- Focused, single-responsibility agents
- Appropriate tool permissions
- Clear communication of purpose
- Autonomous operation patterns
- Portable, resilient implementations
- Robust error handling
- Transparent tool usage
- Cost-effective model selection
- Production-ready agents
- Discoverable agent library

When implementing agents, ask: **Am I adding clarity or complexity?** If complexity, refactor to follow agent development best practices.
