---
title: "Parallel-by-Default Practice"
description: When doing work with independent sub-units (tool calls, file reads/edits, searches, or delegated agents), default to running them in parallel rather than serially, capped at three concurrent units of work
category: explanation
subcategory: development
tags:
  - parallelism
  - concurrency
  - performance
  - ai-agents
  - efficiency
created: 2026-06-23
---

# Parallel-by-Default Practice

When independent units of work are ready, run them in parallel. Serial execution of independent work wastes throughput and adds latency with no benefit. The deliberate cap of **three** simultaneous units preserves meaningful speedup while staying below the token-burn and Claude API per-minute rate-limit threshold.

## Principles Implemented/Respected

This practice respects the following core principles:

- **[Deliberate Problem-Solving](../../principles/general/deliberate-problem-solving.md)**: The cap of three is a deliberate, pre-decided constraint — not a reactive limit set after hitting errors. Acting from a bounded model prevents speculative over-parallelism and the cascading failures it causes.

- **[Simplicity Over Complexity](../../principles/general/simplicity-over-complexity.md)**: One number — three — governs all parallel work. No adaptive scheduling, no per-task caps, no context-dependent arithmetic. A single fixed cap is simple enough to apply consistently without thought.

- **[Automation Over Manual](../../principles/software-engineering/automation-over-manual.md)**: Running independent tool calls in a single turn, or independent subagents in background, automates what would otherwise require manually sequenced round-trips. Parallel-by-default is the automated form of efficient execution.

- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**: The cap and its rationale are stated explicitly in this document. Agents apply the value here — they do not infer limits from context or self-promote based on observed headroom.

## Conventions Implemented/Respected

This practice implements/respects the following conventions:

- **[Content Quality Principles](../../conventions/writing/quality.md)**: This document follows active voice, proper heading hierarchy, and accessible examples throughout.

- **[File Naming Convention](../../conventions/structure/file-naming.md)**: This document uses a lowercase kebab-case filename consistent with repository naming rules.

## Purpose

Two failure modes emerge when agents treat serial execution as the default:

1. **Unnecessary latency**: Reading five independent files one at a time takes five round-trips. Reading them in a single parallel turn takes one. The agent adds latency for every independently-readable file, search, or tool call that waits behind a previous unrelated operation.

2. **Wasted throughput**: Parallel capacity exists. Leaving it idle while independent work queues serially is waste — the kind that compound across every multi-file task and multi-agent batch an agent runs.

This practice eliminates both failure modes by inverting the default: parallel unless dependent.

## Scope

### What This Practice Covers

- Independent Bash/tool calls batched together in a single conversation turn (e.g., reading multiple unrelated files, running multiple independent searches)
- Delegated Agent-tool spawns running in background (covered in detail by [Subagent Orchestration Convention](../agents/subagent-orchestration.md))
- Any work where sub-units do not depend on each other's output

### What This Practice Does NOT Cover

- Dependent work, where a later step requires an earlier step's result — those stay sequential
- Intra-agent concurrency inside a subagent's own execution (governed by that agent's own behavior)
- Bash-level pipeline parallelism (e.g., `&` / `wait` in shell scripts)

## Standards

### Standard 1 — Parallel Unless Dependent

The default execution model is **parallel**. An agent MUST run multiple independent units of work in the same turn rather than issuing them one at a time when:

- The outputs of the units do not depend on each other
- All inputs needed to launch the units are already known

The burden of proof is on serialization: an agent that runs independent work serially must have an explicit reason (dependency, ordering constraint, tool conflict). Absence of a reason means parallel.

### Standard 2 — Cap at Three

No more than **three** independent units of work run simultaneously at any point in a turn, unless the user explicitly raises the cap for a specific batch or session. After a unit completes, a new one may start immediately to refill the slot — the cap governs the instantaneous maximum, not the batch total.

**Why three**: Three concurrent units deliver meaningful parallel speedup over serial execution while staying below the token-burn rate and Claude API per-minute quota threshold that cause degraded performance or rate-limit errors. Fewer than three under-uses available throughput; more than three risks rate-limit cascades. Three is the deliberate optimum.

**Override rule**: The user may raise the cap for a named session or batch (e.g., "read all eight of these files at once"). The override applies for that session or batch only. The agent MUST NOT self-promote the cap based on its own assessment of available headroom.

### Standard 3 — Subagent Specialization

For delegated Agent-tool spawns, the same cap-3 norm applies, but the mechanics are different: background subagents require polling, stuck detection, and relaunch procedures. The [Subagent Orchestration Convention](../agents/subagent-orchestration.md) owns those mechanics. This practice sets the general norm; that convention is the concrete specialization for background agent spawning.

## Anti-Patterns

### Serial Execution of Independent Reads

**Problem**: The agent reads five unrelated files one at a time, waiting for each response before issuing the next read.

**Why it fails**: Each read is independent. Five sequential round-trips add latency proportional to the number of files. The batch could complete in one turn with five parallel reads (or two turns at cap-3).

**Fix**: Batch all independent reads into a single turn, up to three at a time.

---

### Serial Execution of Independent Searches

**Problem**: The agent runs three independent grep/glob searches in sequence, waiting for each result before issuing the next.

**Why it fails**: The searches are independent. Running them in parallel reduces total elapsed time to the duration of the slowest single search.

**Fix**: Issue all three searches in the same turn.

---

### Self-Promoting the Cap

**Problem**: The agent raises the parallel limit to five or six because the first few units completed quickly and the API feels responsive.

**Why it fails**: Response latency varies across a batch. Units that start fast can converge on rate-limit boundaries as they all hit their heaviest phases simultaneously. The cap is set to stay safely below saturation at all batch phases, not just the opening ones.

**Fix**: The cap is three. Only explicit user instruction raises it.

---

### Parallelizing Dependent Work

**Problem**: The agent runs step B in parallel with step A even though step B requires step A's output as its input.

**Why it fails**: Step B will read stale or missing data. The result is either a tool error or incorrect output that must be redone.

**Fix**: Identify dependencies before batching. Dependent steps run sequentially. Independent steps run in parallel.

## References

**Related Principles:**

- [Deliberate Problem-Solving](../../principles/general/deliberate-problem-solving.md) - Bounded, pre-decided constraints over reactive improvisation
- [Simplicity Over Complexity](../../principles/general/simplicity-over-complexity.md) - One fixed cap over adaptive scheduling
- [Automation Over Manual](../../principles/software-engineering/automation-over-manual.md) - Automated parallel execution over manually sequenced round-trips
- [Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md) - Documented constants, not inferred limits

**Related Practices:**

- [Subagent Orchestration Convention](../agents/subagent-orchestration.md) - Concrete specialization of this norm for background Agent-tool spawns; owns polling, stuck detection, and relaunch mechanics
- [Agent Workflow Orchestration Convention](../agents/agent-workflow-orchestration.md) - Broader agent task management strategy of which parallel-by-default is one component

**Agents:**

- `repo-rules-checker` - Validates convention compliance
- `repo-rules-maker` - Creates and updates conventions
