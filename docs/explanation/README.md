---
title: Explanation
description: Conceptual documentation for open-sharia-enterprise
category: explanation
tags:
  - index
  - explanation
  - concepts
created: 2025-11-22
---

# Explanation

**Understanding-oriented documentation** that provides background, context, and conceptual knowledge about the open-sharia-enterprise project.

## What is Explanation Documentation?

Per the [Diátaxis framework](../../repo-governance/conventions/structure/diataxis-framework.md), Explanation documentation:

- **Deepens understanding** of concepts, design decisions, and systems
- **Answers "why?"** questions and provides context
- **Clarifies background** and alternative approaches
- **Discusses trade-offs** and decision rationale

This is distinct from:

- **Tutorials** (learning by doing)
- **How-to Guides** (solving specific problems)
- **Reference** (technical specifications)

## Documentation Scope

This directory contains conceptual documentation across multiple areas:

- **Repository Governance** - How we organize, validate, and enforce standards
- **Software Design** - System design and technical decisions
- **Domain Concepts** - Shariah-compliant enterprise principles and Islamic business foundations
- **Technical Background** - Technologies, patterns, and frameworks used in the project

---

## 🏛️ Repository Governance

The repository follows a **six-layer governance architecture** (Vision → Principles → Conventions → Development → Agents → Workflows) where each layer builds on the foundation above.

**See [Rules](../../repo-governance/README.md)** for governance overview with architecture diagram, layer descriptions, and decision trees.

**See [Repository Governance Architecture](../../repo-governance/repository-governance-architecture.md)** for comprehensive explanation with traceability examples, usage guidance, and verification methods.

---

## 📋 Documentation Index

### Repository Governance

- **[Rules](../../repo-governance/README.md)** - All governance layers (Vision, Principles, Conventions, Development, Workflows)
- **[Repository Governance Architecture](../../repo-governance/repository-governance-architecture.md)** - Comprehensive architecture deep-dive

### Software Engineering

- **[Software Engineering](./software-engineering/README.md)** - Complete index of programming languages, frameworks, architecture patterns, and development practices
- **[C4 Architecture Model](./software-engineering/architecture/c4-architecture-model/README.md)** - Visualizing software architecture through hierarchical abstraction levels
- **[Domain-Driven Design (DDD)](./software-engineering/architecture/domain-driven-design-ddd/README.md)** - Strategic and tactical patterns for modeling complex business domains

### Decision Logs

- **[Plan Domain Parity — Design Decisions (2026-06-06)](./plan-domain-parity-decisions.md)** - All 26 decisions from the 2026-06-06 cross-repo parity effort: what was resolved, why, and what was rejected across ose-public, ose-primer, and ose-infra
- **[Gherkin Step-Keyword Cardinality — Parity Decisions (2026-06-07)](./gherkin-step-keyword-cardinality-parity-decisions.md)** - All 13 decisions from the 2026-06-07 cross-repo parity effort: canonical rule, four deliberate deviations (primer dual-CLI, sibling Step 0.5 preflight port, per-repo CI wiring, primer main-push), and aligned decisions across ose-public, ose-primer, and ose-infra

### Post-Mortems

- **[Post-Mortems](./post-mortems/README.md)** - Blameless incident retrospectives (CI/CD failures, Vercel outages, dependency-bump and parity-guard regressions). Writer-facing template and index; governed by the [Post-Mortem Convention](../../repo-governance/conventions/structure/post-mortems.md)

### Domain Concepts

_Documentation for Shariah-compliant enterprise principles and Islamic business foundations to be added as the project evolves._

### Technical Background

_Documentation for key technologies, patterns, and frameworks to be added as the project evolves._
