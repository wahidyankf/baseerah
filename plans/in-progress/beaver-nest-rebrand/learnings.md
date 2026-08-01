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

## Phase 3: content sed can rename an outbound link's target path before the target file itself moves

`docs/reference/system-architecture/deployment.md` linked to
`../../../plans/ideas/baseerah-first-deploy.md`. Phase 3's blind `<CANONICAL-SED>` pass rewrote the
link text to `beaver-nest-first-deploy.md`, but that file isn't `git mv`'d until Phase 4 — so the
link broke immediately, failing `md links validate` before Phase 3 could even push. Reverted the one
link back to `baseerah-first-deploy.md` (mirroring the Decision-12 GitHub-URL pattern: keep the old
path text until the real move happens), and added a step to Phase 4's idea-brief-rename item to
repoint this same link once the file actually moves — the mirror-image of the Phase 1 rule (that
rule was about inbound links breaking when a path moves; this is an outbound link's target text being
renamed before the path moves). **Generalizable rule**: whenever a phase's content sweep touches a
markdown link whose _target_ is renamed by a _later_ phase, revert that one link's text in the
current phase and add an explicit repoint step to the later phase's `git mv` item.

## Phase 4: renaming a file leaves repo-wide inbound links dangling, same as Phase 1

Confirmed the same class of bug documented in the Phase 1 entry above, this time for a `git mv`
(not just a content rename): after `git mv plans/ideas/baseerah-first-deploy.md
plans/ideas/beaver-nest-first-deploy.md`, `md links validate` found 2 more repo-wide inbound links
(`apps/README.md`, `plans/in-progress/beaver-nest-rebrand/brd.md`) beyond the one already caught in
Phase 3 (`docs/reference/system-architecture/deployment.md`, reverted there pending this phase).
Fixed all three with a targeted path-only sed (not a full prose sweep of the referencing files,
since e.g. `apps/README.md` still legitimately says `baseerah-fe`/`baseerah-be` elsewhere until
Phases 8-11). **Reconfirms the Phase 1 rule**: every phase with a `git mv` (Phases 6, 8, 9, 10, 11, 12) must re-run `git grep -l "<old-path>"` across the whole repo, not just its own file set, before
pushing.

## Phase 2: BSD `xargs` on macOS has no `-a` flag

The plan's own reference commands use `xargs -a <file> -I{} ...` to feed a captured citation-file
list into a revert command. This is GNU-xargs syntax; BSD `xargs` (macOS, this dev machine) has no
`-a` option and errors immediately (`xargs: invalid option -- a`). Portable equivalent:
`< <file> xargs -I{} ...` (redirect stdin instead of `-a`). Every later phase's citation-revert step
that copies this exact command needs this substitution when executing on macOS.
