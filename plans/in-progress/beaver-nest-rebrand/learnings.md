<!-- Knowledge Capture running log — append entries during execution. -->
<!-- Triage every entry (or record the explicit "none" escape) before archival. -->

# Learnings: beaver-nest-rebrand

## Phase 1: pre-push `md links validate` blocks on ANY repo-wide broken link, not just the current phase's file set

Renaming `repo-governance/vision/baseerah.md` → `beaver-nest.md` immediately broke 27 inbound
markdown links from files entirely outside Phase 1's scope (agent-fleet mirrors under
`.claude/agents/`, `.cursor/agents/`, `.opencode/agents/`, `plans/ideas/*`,
`specs/apps/baseerah/product/README.md`, and this plan's own `README.md`/`brd.md`). The repo's
pre-push husky hook runs `md links validate` across the whole tree and fails the push on any
broken link, regardless of which phase "owns" the referencing file's full content sweep.

**Generalizable rule for every later phase that `git mv`s a path** (Phases 6, 8, 9, 10, 11, 12 all
rename directories): in the SAME commit as the rename, also repoint (not fully content-sweep) every
repo-wide inbound link to the old path — a targeted single-string sed scoped to just that link
string, leaving the referencing file's own full `baseerah`→`beaver-nest` prose sweep to its
designated phase. Verify with `git grep -l "<old-path>"` before push to catch every reference, and
re-run `md links validate` before every push, not just at the phase whose own gate mentions it.
