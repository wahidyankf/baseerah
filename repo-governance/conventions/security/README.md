---
title: "Security Conventions"
description: Repository security conventions governing agent behavior and data protection
category: explanation
subcategory: conventions
tags:
  - index
  - security
  - conventions
created: 2026-05-24
---

# Security Conventions

Security conventions governing how agents and contributors interact with sensitive repository
artifacts. These conventions define rules that reduce the risk of secret exposure, accidental
commits of sensitive files, and related security events.

**Governance**: All conventions in this directory implement the [Core Principles](../../principles/README.md)
(Layer 1) and are part of the [six-layer governance architecture](../../repository-governance-architecture.md).

## Conventions

- [No Secrets in Git](./no-secrets-in-git.md) — The hard iron rule that no system secret (SSH/private
  keys, passwords, API tokens, privileged usernames, certificates, connection strings, and similar)
  may ever be committed to any git-tracked file in this repository. Real secret values belong in
  uncommitted `.env*` files (except `.env.example`) or other gitignored files. The broad governing
  rule that `guard-env-file-access` partially enforces.
- [Environment File Access](./env-file-access.md) — The `guard-env-file-access` policy. AI agents
  must not directly read, write, edit, or commit any `.env*` file except `.env.example`. Covers the
  script carve-out, trust boundary, git-commit prevention (gitignore + pre-commit guard), cross-platform
  enforcement paths, and known gaps with accepted compensating controls.

## Related Documentation

- [Conventions Index](../README.md) — All conventions organized by category
- [Repository Governance Architecture](../../repository-governance-architecture.md) — Six-layer hierarchy
- [Core Principles](../../principles/README.md) — Foundational values governing all conventions
