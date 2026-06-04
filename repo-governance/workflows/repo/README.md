---
title: "Repository Workflows"
description: "Orchestrated repository-level governance workflows — rules consistency, harness compatibility (parity + external drift), ose-primer content synchronization, and dependency bump planning."
category: explanation
subcategory: workflows
tags: []
created: 2026-05-12
---

# Repository Workflows

Orchestrated workflows for repository-level governance — validation, binding-file health, content synchronization with the `ose-primer` template, and policy-compliant dependency bump planning.

## Purpose

These workflows define **WHEN and HOW to validate and synchronize repository artifacts**, orchestrating agents for three concerns: repository rules consistency (repo-rules-checker, repo-rules-fixer), harness compatibility including cross-vendor parity and external drift (repo-harness-compatibility-checker, repo-harness-compatibility-fixer), and ose-primer content sync (repo-ose-primer-adoption-maker, repo-ose-primer-propagation-maker).

## Scope

**✅ Workflows Here:**

- Repository-wide consistency validation
- Cross-layer governance checking
- Agent standards enforcement
- Iterative check-fix-verify cycles

**❌ Not Included:**

- Content quality validation (that's docs/)
- ayokoding-web content validation (that's ayokoding-web/)
- Plan validation (that's plan/)

## Workflows

- [Repository Rules Validation](./repo-rules-quality-gate.md) - Validate repository consistency across all layers (principles, conventions, development, agents) and apply fixes iteratively until ZERO findings. Supports four strictness modes (lax, normal, strict, ocd)
- [ose-primer Sync Execution](./repo-ose-primer-sync-execution.md) - Single-pass sync orchestration between `ose-public` and `ose-primer`. Dispatches the adoption-maker or propagation-maker agent, collects its report, and (in apply mode) surfaces the resulting primer PR URL.
- [ose-primer Extraction Execution](./repo-ose-primer-extraction-execution.md) - One-time orchestration for Phase 8 of the 2026-04-18 ose-primer-separation plan. Runs the primer-parity gate, a bounded catch-up loop, and ten ordered extraction commits (A → J) with per-commit CI verification.
- [Harness Compatibility Quality Gate](./repo-harness-compatibility-quality-gate.md) - Validates five deterministic cross-vendor parity invariants (Phase 0) then verifies the platform-binding catalog and committed binding files still match each supported harness's current upstream conventions (Phase 1); fixes drift iteratively to double-zero.
- [Dependency Bump Planning](./repo-dependency-bump-planning.md) - Surveys every dependency manifest across `apps/` and `libs/`, classifies each candidate bump per the Dependency Bump Stability & Safety Policy (three-path tree + Rule 5a/5b), and produces a validated **backlog** plan (via `plan-establishment-execution` with `target-stage=backlog`) that will perform the bumps. Deliverable is the plan, not the dependency edits.

## Related Documentation

- [Workflows Index](../README.md) - All orchestrated workflows
- [Repository Architecture](../../repository-governance-architecture.md) - Six-layer governance model these workflows enforce
- [Maker-Checker-Fixer Pattern](../../development/pattern/maker-checker-fixer.md) - Core workflow pattern
- [Core Principles](../../principles/README.md) - Layer 1 governance
