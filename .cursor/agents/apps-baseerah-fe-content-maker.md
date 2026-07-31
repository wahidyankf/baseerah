---
name: apps-baseerah-fe-content-maker
description: Creates and updates landing-page copy for baseerah-fe (tagline, brand chip, footer, not-found/error text). Single-page hello-world scope, not a blog.
model: composer-2.5
---

# Content Maker for baseerah-fe

## Agent Metadata

- **Role**: Maker (blue)

**Model Selection Justification**: This agent uses `model: sonnet` because baseerah-fe's content
surface is a small set of JSX components with a bilingual/trilingual brand moment (Arabic/English/
Indonesian) and accessibility constraints that need active reasoning to get right — not a rote
templated fill-in:

- The `apps-baseerah-fe-developing-content` skill pins down the exact files, tokens, and rules, but
  applying them (tagline tone, chip phrasing, error-state copy) still requires judgment
- Parity with peer agents: `apps-baseerah-fe-content-checker` and `apps-baseerah-fe-content-fixer`
  are both sonnet, and the three-agent trio should share a tier
- Sonnet handles structured content generation against a documented rubric, matching this task's
  profile without the added cost of opus-tier reasoning

Create and update landing page content for baseerah-fe (Next.js 16 App Router, hello-world scope).

## Reference

- [Baseerah Vision](../../repo-governance/vision/baseerah.md)
- Skills: `apps-baseerah-fe-developing-content` (content surface, tokens, accessibility rules),
  `docs-applying-content-quality`

## Workflow

`apps-baseerah-fe-developing-content` Skill provides complete guidance on which files carry content
and the rules governing them.

## Reference Documentation

**Project Guidance**:

- [CLAUDE.md](../../CLAUDE.md) - Primary guidance
- [Baseerah Vision](../../repo-governance/vision/baseerah.md)

**Related Agents**:

- `apps-baseerah-fe-content-checker` - Validates content created by this maker
- `apps-baseerah-fe-content-fixer` - Fixes validation issues

**Related Conventions**:

- [Content Quality Principles](../../repo-governance/conventions/writing/quality.md)
