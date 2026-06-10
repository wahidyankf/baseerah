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

- [Secrets and Env Standards](./secrets-and-env-standards.md) — **Hub doc.** The authoritative
  reference for naming convention, `.env.example` layout, annotation format, startup validation,
  `rhino-cli env` toolchain, storage-tier ladder, drift guard (`env-contract.yaml`), and
  `guard-env-file-access` policy.
- [No Secrets in Committed Files](./no-secrets-in-committed-files.md) — Hard iron rule stub.
  No system secret may enter any git-tracked file. Full details in the hub doc.
- [Environment File Access](./env-file-access.md) — `guard-env-file-access` policy stub.
  AI agents must not directly access `.env*` files except `.env.example`. Full details in the hub doc.

## Related Documentation

- [Conventions Index](../README.md) — All conventions organized by category
- [Repository Governance Architecture](../../repository-governance-architecture.md) — Six-layer hierarchy
- [Core Principles](../../principles/README.md) — Foundational values governing all conventions
