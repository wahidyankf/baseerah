---
title: "Large-Scale Rename Plan Link Management"
description: Rules for keeping markdown links and path-referencing strings consistent through a multi-phase repo-wide identifier rename plan, where the pre-push `md links validate` hook checks the whole tree regardless of which phase owns a given file
category: explanation
subcategory: development
tags:
  - workflow
  - rename
  - markdown-links
  - pre-push
  - plan-authoring
created: 2026-08-01
---

# Large-Scale Rename Plan Link Management

Rules learned from executing a multi-phase, repo-wide identifier rename plan (renaming a product
identity across `AGENTS.md`, `docs/`, `specs/`, `apps/`, CI workflows, and agent definitions, phase by
phase). They generalize to any future plan that renames a widely-referenced string or path across many
phases.

## The Core Fact: `md links validate` Is Repo-Wide and Ownership-Blind

The pre-push husky hook runs `md links validate` across the **whole tree**, on every push, regardless
of which phase "owns" the file doing the referencing. A phase that only intends to touch its own
directory can still break the push if some other, unrelated file elsewhere in the repo links to a path
that phase just renamed.

## Rule 1: A `git mv` Breaks Repo-Wide Inbound Links, Not Just the Renaming Phase's Own Set

Renaming a file or directory (`git mv old new`, or a content rename that changes what a heading/anchor
resolves to) can leave links dangling in files that have nothing to do with the phase doing the
rename — agent-fleet mirrors, sibling idea briefs, unrelated README indexes, this plan's own
`README.md`/`brd.md`.

**Do this in the same commit as every `git mv`**: repoint every repo-wide inbound link to the old
path with a targeted, single-string sed — not a full content sweep of the referencing file's prose
(that sweep belongs to whichever phase owns that file's full rename). Verify with
`git grep -l "<old-path>"` across the whole repo before push, and re-run `md links validate` after
every phase that moves a path, not just at the phase whose own gate happens to mention it.

## Rule 2: An Outbound Link's Target Text Can Get Renamed Before the Target Itself Moves

The mirror-image of Rule 1: a phase's content sed (e.g. a blanket find-and-replace of the old
identifier for the new one) can rewrite a link's **target path text** to the new name before the file
at that path has actually been `git mv`'d — because the move is scheduled for a later phase. The link
breaks immediately, since the new path doesn't exist yet.

**Fix**: revert that one link's text back to the old path in the current phase, and add an explicit
repoint step to the later phase that actually performs the `git mv`, so the link's target text changes
in lockstep with the real move.

## Rule 3: Deferring a Push Cadence Defers EVERY Pre-Push-Only Check, Not Just the One That Motivated the Deferral

If a plan collapses its push cadence across several phases (e.g. because a deliberate cross-phase
RED/GREEN design conflicts with `test:quick` being part of the pre-push hook — see
[Test-Driven Development Convention § Cross-phase RED spans and the pre-push hook](./test-driven-development.md)),
remember that `.husky/pre-push` runs **more** than just `test:quick` — `md links validate` lives there
too, and it is invisible to any `nx affected -t ...` command run manually as a stand-in "local quality
gate" during the deferred stretch. Deferring the push defers link validation for the entire stretch,
even though each phase's own `git mv` steps were individually careful about the links they touched.

**Before deciding to defer a push cadence, explicitly enumerate every check that lives only in the
pre-push hook** (not just the one motivating the deferral), and either run that check manually after
every phase in the deferred stretch (`cargo run ... -- md links validate` directly, without waiting for
the collapsed push) or accept that the accumulated stretch may surface several broken links all at
once when the collapsed push finally runs.

## Summary Checklist for Any Rename-Heavy Phase

- [ ] After every `git mv`, `git grep -l "<old-path>"` across the whole repo (not just the phase's own
      directory) and repoint every hit with a targeted sed.
- [ ] After every content sed that touches a markdown link, check whether the link's _target_ has
      already moved — if not, revert that one link's text and schedule its repoint for the phase that
      performs the actual move.
- [ ] If deferring a push cadence across phases, run `md links validate` manually after each phase in
      the deferred stretch rather than waiting for the collapsed push to surface everything at once.

## Related Documentation

- [Test-Driven Development Convention](./test-driven-development.md) — the companion rule about
  cross-phase RED spans conflicting with the same pre-push hook
- [CI Post-Push Verification Convention](./ci-post-push-verification.md)
- [Markdown Quality Convention](../quality/markdown.md)
