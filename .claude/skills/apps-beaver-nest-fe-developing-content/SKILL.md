---
name: apps-beaver-nest-fe-developing-content
description: Structure, tone, and accessibility rules for the BeaverNest Vite CSR foundation-status screen.
---

# Developing Content for beaver-nest-fe

## Purpose

`beaver-nest-fe` is a Vite CSR foundation-status client, not a marketing site, blog, or CMS. It is
served by the combined ASP.NET runtime and uses same-origin readiness requests.

## Content Surface

| File             | Content                                                                               |
| ---------------- | ------------------------------------------------------------------------------------- |
| `src/App.tsx`    | BeaverNest heading, foundation-status labels, readiness feedback, and refresh control |
| `src/styles.css` | Client stylesheet and token imports                                                   |
| `src/theme.ts`   | External system-theme bootstrap                                                       |

## Rules

- Keep copy direct and operational; do not add promotional calls to action or greeting content.
- Use shared semantic design tokens, never raw colors or arbitrary values.
- Preserve one visible `h1`, readable status feedback, and an explicitly labelled refresh control.
- Test content changes with the Vite unit/integration and Gherkin gates.

## Reference

- [Content Quality Principles](../../../repo-governance/conventions/writing/quality.md)
- `docs-applying-content-quality` for universal content rules
- `specs/apps/beaver-nest/behavior/beaver-nest-fe/gherkin/` for the bound browser behavior
