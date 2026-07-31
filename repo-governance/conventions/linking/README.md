---
title: "Linking Conventions"
description: ""
category: explanation
subcategory: conventions
tags: []
created: 2026-05-12
---

# Linking Conventions

This category contains standards for cross-referencing and internal linking in markdown documentation.

## Purpose

Linking conventions ensure consistent, maintainable internal references across the repository. These standards govern how markdown files reference other files, ensuring links remain functional and follow repository conventions.

## Conventions

### [Documentation Linking Convention](../formatting/linking.md)

Standards for linking between documentation files across the repository, including relative path
conventions, `.md` extension requirements, and cross-directory reference patterns — for example,
linking from `rhino-cli` CLI docs into shared `repo-governance/` conventions, or from a future
`baseerah-fe`/`baseerah-be` app's docs back into the same conventions tree.

## Principles Implemented/Respected

- **[Simplicity Over Complexity](../../principles/general/simplicity-over-complexity.md)**: Direct markdown links, no complex indirection
- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**: Links are explicit paths, not computed references
- **[Documentation First](../../principles/content/documentation-first.md)**: Clear linking standards prevent broken references

## Related Conventions

- [File Naming Convention](../structure/file-naming.md) - Correct file names enable accurate linking
- [Diátaxis Framework](../structure/diataxis-framework.md) - Documentation organization affects link paths
