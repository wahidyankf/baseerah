# Learnings: adopt-cursor-platform-binding

Transient running log. The executor appends one entry per generalizable learning **as it surfaces**,
sanitized before it is ever written. Phase 8 (Knowledge Capture) drains this file: every entry is
routed to a durable home or discarded with a reason, and this file is archived with the plan.

## How to use this file

Append an entry the moment something generalizable surfaces — do not reconstruct the log at the end.
Apply the **secret/sensitivity gate** at write time: no token, key, session identifier, account
identifier, absolute home path, or private hostname is ever written here; replace each with a
`<placeholder>`. Apply the **repo-relevance gate** at routing time: infra-private content stays in
`ose-infra` only.

The litmus test for keeping an entry: _would a durable surface catch this automatically next time?_
If no durable surface could, the entry is an anecdote and gets discarded with a one-line reason.

Code-homed learnings (`apps/`, `libs/`, tests) are **ALWAYS** filed as a separate
`plans/backlog/<slug>/` plan and **NEVER** landed inline in this plan's commits. The sole carve-out
is a blocker genuinely required to finish this plan's own scope.

## Entry shape

```markdown
## Learning: <one-line summary>

- **Context**: what was being done when this surfaced
- **Observation**: what was noticed (sanitized)
- **Why it might generalize**: the litmus reasoning
- **Routing**: filled in during Phase 8 — durable home, backlog slug, or discard reason
```

## Likely sources in this plan

Named here as prompts, not as pre-written entries. Do not tick anything off this list; it exists only
so the executor recognises a learning when it appears.

- Phase 1 — what the four unknowns actually resolved to, and whether the fallback had to be taken.
- Phase 2 — whether the `FieldAction` / `FieldPolicy` extraction revealed a shape the OpenCode
  converter had been carrying implicitly.
- Phase 3 — the ordering constraint between the catalog row and the first generation, and whether
  the Prettier check landed on the ignore branch or the no-op branch.
- Phase 4 — any `ose-public` governance surface stating the Cursor rule that the S1-S10 / P1-P13
  verdict tables missed, and whether `validate_catalog_coverage`'s coarse substring match let a real
  gap through.
- Phase 5 — whether the live probe matched the frontmatter, and what the session record actually
  reported.
- Phase 6 — anything about `ose-primer`'s topology, its divergent governance-target naming
  (`vendor-audit` versus `vendor validate`), or the byte-identity payload that the shared
  preconditions did not anticipate.
- Phase 7 — anything about `ose-infra`'s topology, its missing catalog table (row I7), or the
  pre-existing `ci-monitor-subagent.md` orphan (row I14) that must be routed to backlog rather than
  fixed inline.

## Entries

<!-- Append entries below this line, newest last. -->

## Learning: Shared CARGO_TARGET_DIR causes cross-repo E0460

- **Context**: Running `rhino-cli` release builds in ose-primer/ose-infra worktrees after copying from ose-public
- **Observation**: `serde_norway` E0460 when sandbox/shared cache mixes artifacts from different repo checkouts
- **Why it might generalize**: Any sibling landing that rsyncs rhino-cli while sharing `CARGO_TARGET_DIR` or ose-public cache paths will hit stale rlib mismatches
- **Routing**: docs/how-to/worktree-setup.md or rhino-cli README — note per-worktree `apps/rhino-cli/target` isolation

## Learning: ose-primer pre-push blocked on broken anchor in copied platform-bindings.md

- **Context**: Phase 6 push after copying ose-public governance verbatim
- **Observation**: `#platform-binding-color-translation` anchor missing in ose-primer's `ai-agents.md`; correct anchor is `#color-translation-table`
- **Why it might generalize**: Shared-surface copies from ose-public must not assume identical heading anchors across repos
- **Routing**: plan backlog — add cross-repo link validation to parity workflow

## Learning: composer-2.5-fast grep false positive in generated checker agent

- **Context**: Phase 5 repo-local assertion `grep -r composer-2.5-fast .cursor/agents/`
- **Observation**: Documentation in `repo-harness-compatibility-checker.md` contained the literal string, failing the gate
- **Why it might generalize**: Any grep-based "must not exist" gate applies to prose mentions, not just frontmatter values
- **Routing**: model-selection.md prohibition section — reference `^model:` scoped grep pattern
