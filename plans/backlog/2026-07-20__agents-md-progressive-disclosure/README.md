# AGENTS.md Progressive-Disclosure Refactor

> **Status**: Backlog — filed by the Knowledge Capture phase of
> [`parallel-orchestration-shared-machine-governance`](../../done/) (merged as `60d53119b`).
>
> **Delivery Mode**: `worktree-to-pr` (repo default)
>
> **Urgency**: `AGENTS.md` is currently **29,995 bytes** against a **30,000-byte fail** threshold.
> The next governance addition of any size fails the gate.

`AGENTS.md` is the most-loaded instruction surface in the repository and sits five bytes below its
hard ceiling. This plan moves detail behind progressive disclosure to restore working headroom —
without deleting a rule, compressing to dense prose, or pointing at an incomplete target.

## Context

Measured at filing time: [Repo-grounded]

| Surface                          | Size      | Warn     | Fail     |
| -------------------------------- | --------- | -------- | -------- |
| `AGENTS.md`                      | 29,995 B  | 27,000 B | 30,000 B |
| `CLAUDE.md`                      | 7,373 B   | 8,000 B  | 10,000 B |
| Resolved tree (root `CLAUDE.md`) | ~37,368 B | 34,000 B | 38,000 B |

Both `AGENTS.md` and the resolved tree are **over warn** and near **fail**. This is not new debt
introduced by any one plan: `AGENTS.md` was already 28,333 bytes at the originating plan's baseline
commit, before that plan touched it.

**The structural tension**: governance plans exist to thread new rules through this exact file,
while the
[Instruction-File Size Budget Convention](../../../repo-governance/conventions/structure/instruction-file-size-budget.md)
names progressive disclosure as the sole sanctioned remediation. Neither side is wrong — they were
authored independently, and nothing forces a plan author to notice the collision until a gate
fires mid-execution.

## The Hazard This Plan Must Not Reproduce

A previous compression pass on this file replaced an inline environment-branch enumeration with a
pointer to a table that was **not complete**, leaving three deploy targets uncovered by a "never
commit directly" rule — one of which an agent force-pushes to. That failure is now recorded as
**Forbidden Anti-Fix 4 ("Point at an incomplete target")** in the budget convention.

**Binding constraints on execution**:

1. Before every `See`-link replacement, **diff the target against ground truth** and prove the
   target covers every case the inline text covered. Text search cannot find omissions — see
   [Absence and Completeness Claims](../../../repo-governance/development/quality/plan-anti-hallucination.md#absence-and-completeness-claims-hard).
2. **Never compress a safety guardrail to save bytes** — the secrets/`.env` rules, the Git Identity
   Guardrail, and the environment-branch rule are trimmed last and only via a complete target.
3. Prefer restating a rule **as a pattern rather than an enumeration**, which is both shorter and
   structurally complete. The environment-branch rule's current form is the worked example: "every
   `prod-*` and `stag-*` ref is a deploy target — never commit directly; `git branch -r` is
   authoritative."
4. No rule may be deleted, and no content may be moved into another auto-loaded file (that merely
   relocates bytes within the resolved tree).

## Scope

**In scope**:

- Identify the highest-byte inline-expanded sections of `AGENTS.md` that have a complete canonical
  home in `repo-governance/`, `docs/`, or a per-app `README.md`.
- Replace each with a one-line summary plus a `See` link, after proving target completeness.
- Convert enumerations to patterns where a pattern is both complete and shorter.
- Re-measure and confirm `AGENTS.md` and the resolved tree land back in the **target** band, not
  merely under fail.

**Out of scope**: raising the thresholds. The budget convention explicitly forbids adjusting
thresholds to paper over a bloated file.

## Acceptance Criteria

```gherkin
Feature: AGENTS.md fits its size budget without losing rules

  Scenario: The size gate passes with headroom
    Given the refactored AGENTS.md
    When nx run rhino-cli:instruction-size:validation runs
    Then the command exits 0
    And AGENTS.md is at or under its 24000-byte target threshold
    And no warn-level message is emitted for AGENTS.md or the resolved tree

  Scenario: Every See-link target is complete
    Given each section replaced by a summary plus a See link
    When the link target is diffed against the ground truth for that section
    Then every case covered by the removed inline text is covered by the target

  Scenario: No rule was deleted
    Given the pre-refactor and post-refactor AGENTS.md
    When each removed rule is traced
    Then it resolves to a reachable canonical home
    And no rule is absent from both AGENTS.md and every link target

  Scenario: Safety guardrails remain inline and complete
    Given the refactored AGENTS.md
    When the secrets, env-file, and git-identity guardrails are read
    Then each is stated in full
    And none was replaced by a link

  Scenario: The environment-branch rule covers every deploy target
    Given the refactored AGENTS.md
    When "git branch -r" is diffed against the branches the rule covers
    Then every prod-* and stag-* ref is covered by the rule
```

The third and fifth scenarios are the falsifiability controls — a refactor that shrank the file by
dropping rules would pass the first scenario and fail these.
