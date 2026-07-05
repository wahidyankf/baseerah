# Worktree-to-PR Default Delivery Mode

Change the DEFAULT plan-delivery mode across all three sibling repos
(`ose-public`, `ose-primer`, `ose-infra`) from the current "worktree → push to `main`" to
"worktree → Pull Request", and introduce a small named vocabulary of delivery modes with a clear
override precedence.

## Status

- **Stage**: In Progress
- **Delivery Mode**: `worktree-to-pr` (this plan dogfoods the new default)
- **Repos**: `ose-public` (canonical), `ose-primer` (parity), `ose-infra` (private, outside parity loop)

## Context

Today the repositories document a single implicit delivery posture: work happens in a git worktree
(or the `main` checkout) and commits push **directly to `origin main`** with no pull request unless
one is explicitly requested. This is captured across
[`trunk-based-development.md`](../../../repo-governance/development/workflow/trunk-based-development.md)
[Repo-grounded], [`git-push-default.md`](../../../repo-governance/development/workflow/git-push-default.md)
[Repo-grounded], and the [plan-execution workflow](../../../repo-governance/workflows/plan/plan-execution.md)
Step 0 [Repo-grounded].

This plan makes **worktree → PR** the default while keeping direct-push available as an explicitly
selectable mode. It also names four delivery modes as a closed vocabulary and defines the precedence
by which a mode is selected — mirroring the existing work-branch precedence already documented in
plan-execution Step 0 (user-at-invocation > plan docs > default).

## Scope

**In scope** — governance/docs edits (no application or library source code, no UI) across three repos:

- The delivery-mode vocabulary and precedence, added to conventions, workflows, agent definitions,
  the plan-authoring skill, and the root instruction files (`AGENTS.md`, `CLAUDE.md`).
- Reconciling Trunk-Based-Development language so worktree → PR via short-lived plan branches is
  framed as a valid TBD flavor (short-lived branches, frequent integration), not an abandonment of TBD.
- Re-sync of `.opencode/` and `.amazonq/` bindings after any `.claude/**` edit.

**Out of scope** — see [`prd.md`](./prd.md#product-scope). Notably: no new `rhino-cli` structural
validator (enforcement is via agent-checker prose), no changes to environment/deploy branches, no
retroactive rewrite of already-archived plans.

## Approach Summary

1. Author all edits in `ose-public` first (the canonical scaffolding source), grouped by governance
   layer: conventions → workflows → agents/skill/root-instructions.
2. Deliver `ose-public` via the new `worktree-to-pr` mode: one worktree, one PR, AI drives all gates
   green, a `[HUMAN]` merge gate closes it.
3. Replicate the identical change to `ose-primer` (parity) and `ose-infra` (private), each in its own
   worktree with its own PR and its own `[HUMAN]` merge gate — three worktrees, three PRs total.

## Documents

- [`brd.md`](./brd.md) — WHY: business/process motivation (reviewability, safer trunk, PR-based CI gating)
- [`prd.md`](./prd.md) — WHAT: the four-mode vocabulary, precedence, acceptance criteria, scope, exemptions
- [`tech-docs.md`](./tech-docs.md) — HOW: per-file impact across all three repos, precedence algorithm, rollback, open questions
- [`delivery.md`](./delivery.md) — DO: phased, per-repo checklist with gates

## Dependency Position

This plan executes **SECOND**. The sibling plan `plan-execution-knowledge-capture` executes **FIRST**;
because that plan lands the Knowledge Capture requirement (an open-ended triage rubric plus the
repo-relevance and secret/sensitivity safety gates) into the repo before this plan runs, this plan
both **depends on** it and must itself honor it — see the final Knowledge Capture phase in
[`delivery.md`](./delivery.md). This plan is still delivered via the worktree → PR mode it establishes
(dogfooding).

```mermaid
%% Dependency ordering — the knowledge-capture plan lands first, this plan follows
flowchart LR
  A["plan-execution-knowledge-capture<br/>(FIRST — lands the Knowledge Capture requirement)"]:::first
  B["worktree-to-pr-default-delivery-mode<br/>(SECOND — this plan; must honor Knowledge Capture)"]:::now
  A -->|"establishes Knowledge Capture requirement"| B

  classDef first fill:#E69F00,stroke:#7a5300,color:#000000;
  classDef now fill:#0072B2,stroke:#023858,color:#ffffff;
```

## Delivery-Mode Selection (at a glance)

```mermaid
%% Precedence for selecting the delivery mode (mirrors work-branch precedence)
flowchart TD
  Start(["Plan execution begins"]) --> Q1{"Mode given as<br/>invocation argument?"}
  Q1 -->|"yes"| UseArg["Use invocation-argument mode"]:::win
  Q1 -->|"no"| Q2{"Plan has a<br/>## Delivery Mode field?"}
  Q2 -->|"yes"| UsePlan["Use plan's declared mode"]:::win
  Q2 -->|"no"| UseDefault["Use default:<br/>worktree-to-pr"]:::def

  classDef win fill:#009E73,stroke:#004d38,color:#ffffff;
  classDef def fill:#0072B2,stroke:#023858,color:#ffffff;
```
