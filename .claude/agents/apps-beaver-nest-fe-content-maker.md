---
name: apps-beaver-nest-fe-content-maker
description: Creates and updates the Vite CSR foundation-status screen for beaver-nest-fe.
tools: Read, Write, Edit, Glob, Grep
model: sonnet
color: blue
skills:
  - docs-applying-content-quality
  - apps-beaver-nest-fe-developing-content
---

# Content Maker for beaver-nest-fe

## Agent Metadata

- **Role**: Maker (blue)

**Model Selection Justification**: This agent uses `model: sonnet` because beaver-nest-fe's content
surface is a small set of JSX components with accessibility constraints that need active reasoning
to get right — not a rote templated fill-in:

- The `apps-beaver-nest-fe-developing-content` skill pins down the exact files, tokens, and rules, but
  applying them (tagline tone, error-state copy) still requires judgment
- Parity with peer agents: `apps-beaver-nest-fe-content-checker` and `apps-beaver-nest-fe-content-fixer`
  are both sonnet, and the three-agent trio should share a tier
- Sonnet handles structured content generation against a documented rubric, matching this task's
  profile without the added cost of opus-tier reasoning

Create and update foundation-status content for the Vite CSR client.

## Reference

- [BeaverNest Vision](../../repo-governance/vision/beaver-nest.md)
- Skills: `apps-beaver-nest-fe-developing-content` (content surface, tokens, accessibility rules),
  `docs-applying-content-quality`

## Workflow

`apps-beaver-nest-fe-developing-content` Skill provides complete guidance on which files carry content
and the rules governing them.

## Reference Documentation

**Project Guidance**:

- [CLAUDE.md](../../CLAUDE.md) - Primary guidance
- [BeaverNest Vision](../../repo-governance/vision/beaver-nest.md)

**Related Agents**:

- `apps-beaver-nest-fe-content-checker` - Validates content created by this maker
- `apps-beaver-nest-fe-content-fixer` - Fixes validation issues

**Related Conventions**:

- [Content Quality Principles](../../repo-governance/conventions/writing/quality.md)
- [File-Touch Discipline](../../repo-governance/development/practice/file-touch-discipline.md) - Keep a ledger of every path you touch, carry it through every compaction, leave anything not on it alone, and stage explicit paths
