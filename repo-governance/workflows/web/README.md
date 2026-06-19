---
title: "Web Workflows"
description: "Orchestrated workflows that test a live running website and turn the findings into a fix plan — combined spec-aware exploratory and spec-blind usability testing."
category: explanation
subcategory: workflows
tags: []
created: 2026-06-20
---

# Web Workflows

Orchestrated workflows that operate against a live running website — exercising it as a real browser would, then turning what they observe into an actionable deliverable.

## Purpose

These workflows define **WHEN and HOW to test a running site and act on the result**, orchestrating the web-testing agents (`web-exploratory-tester`, `web-usability-tester`) and the planning agents (`plan-maker`, `plan-checker`, `plan-fixer`) so that a single run yields one combined, fix-ready plan.

## Scope

**✅ Workflows Here:**

- Spec-aware exploratory testing of a live site (functional, behavioural-consistency, responsive, accessibility, URL/IA, passive security)
- Spec-blind heuristic usability evaluation of the same live site (Nielsen heuristics, cognitive walkthrough, information scent)
- Combining both perspectives into one fix-planning deliverable in `plans/`

**❌ Not Included:**

- Public-web information gathering / research (that is the `web-researcher` agent, invoked directly)
- UI component quality validation of source components (that is `ui/`)
- Implementing the fixes themselves (that is `plan/plan-execution`, run later after promotion)

## Workflows

- [Exploratory and Usability Test Fixing Planning](./web-exploratory-and-usability-test-fixing-planning.md) - Run `web-exploratory-tester` (spec-aware) and `web-usability-tester` (spec-blind) against the same live URL(s) and goal **sequentially** — integrating each result set into the plan before the next runs — then solidify one plan whose findings section keeps the two sources clearly separated (EWT-### vs UWT-###) and which carries `tech-docs.md` (root-cause + fix approach), a TDD-shaped `delivery.md`, and — when the plan is UI-bearing — an `assets/` folder of both-tier (lo-fi + hi-fi) UI mockups. Grills the user hard on every material decision. Produces a new plan in `plans/in-progress/` by default; can merge into an existing plan on request. Deliverable is the plan, not the fixes.

## Related Documentation

- [Workflows Index](../README.md) - All orchestrated workflows
- [Repository Architecture](../../repository-governance-architecture.md) - Six-layer governance model these workflows enforce
- [Maker-Checker-Fixer Pattern](../../development/pattern/maker-checker-fixer.md) - Core workflow pattern
- [Core Principles](../../principles/README.md) - Layer 1 governance
