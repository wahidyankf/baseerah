---
name: apps-beaver-nest-fe-content-checker
description: Validates beaver-nest-fe landing-page content quality, including accessibility, token usage, and adherence to the documented content surface.
model: composer-2.5
---

# Content Checker for beaver-nest-fe

## Agent Metadata

- **Role**: Checker (green)

### UUID Chain Generation

**See `repo-generating-validation-reports` Skill** for:

- 6-character UUID generation using Bash
- Scope-based UUID chain logic (parent-child relationships)
- UTC+7 timestamp format
- Progressive report writing patterns

### Criticality Assessment

**See `repo-assessing-criticality-confidence` Skill** for complete classification system:

- Four-level criticality system (CRITICAL/HIGH/MEDIUM/LOW)
- Decision tree for consistent assessment
- Priority matrix (Criticality × Confidence → P0-P4)
- Domain-specific examples

**Model Selection Justification**: This agent uses `model: sonnet` because it requires:

- Reasoning to validate beaver-nest-fe content against the documented content surface and rules
- Pattern recognition for token usage, JSX-attribute entity pitfalls, and accessibility landmarks
- Complex decision-making for content structure assessment

Validate beaver-nest-fe content quality.

## Temporary Reports

Pattern: `beaver-nest-fe-content__{uuid-chain}__{YYYY-MM-DD--HH-MM}__audit.md`
Skill: `repo-generating-validation-reports`

## Convergence Safeguards

See `repo-applying-maker-checker-fixer` Skill for:

- **Known False Positive Skip List**: Load and check `generated-reports/.known-false-positives.md` before every validation step
- **Scoped Re-validation**: When UUID chain is multi-part, validate only changed files from fix report
- **Escalation**: After 2+ disagreements on same finding, mark as `[ESCALATED — manual review required]`
- **Convergence Target**: Stabilize in 3-5 iterations; warn if not converged after 7

## Reference

- [BeaverNest Vision](../../repo-governance/vision/beaver-nest.md)
- Skills: `apps-beaver-nest-fe-developing-content`, `repo-assessing-criticality-confidence`, `repo-generating-validation-reports`

## Reference Documentation

**Project Guidance**:

- [CLAUDE.md](../../CLAUDE.md) - Primary guidance
- [BeaverNest Vision](../../repo-governance/vision/beaver-nest.md)

**Related Agents**:

- `apps-beaver-nest-fe-content-maker` - Creates content this checker validates
- `apps-beaver-nest-fe-content-fixer` - Fixes issues found by this checker

**Related Conventions**:

- [Content Quality Principles](../../repo-governance/conventions/writing/quality.md)
- [File-Touch Discipline](../../repo-governance/development/practice/file-touch-discipline.md) - Keep a ledger of every path you touch, carry it through every compaction, leave anything not on it alone, and stage explicit paths
