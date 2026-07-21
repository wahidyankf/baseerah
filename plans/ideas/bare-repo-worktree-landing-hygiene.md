# Bare-repo worktree-landing hygiene

One-line summary: stop local `main` from silently diverging after side-worktree pushes, and stop
long-lived WIP from rotting in the shared index, so a bare sibling's `git status` stays readable.

> Surfaced 2026-07-21 while reconciling ose-primer/ose-infra after the Prior-art two-pager landings.

## Problem / context

Landing on a bare sibling's `origin/main` via the base-worktree re-derive method (`git worktree add
origin/main` → re-apply delta → push `HEAD:main` → remove worktree) advances **origin** but never
touches local `main` or its working tree. After the Prior-art work, both ose-primer and ose-infra
sat **4-behind / 1-ahead** of `origin/main` — behind from the bypassed pushes, ahead from a duplicate
`drop-prefix` commit made directly on local `main` on an already-stale base. On top of that, the
local tree carried ~100 uncommitted files: already-landed restructure edits stranded as stale dups,
piled on long-lived foreign rhino-cli WIP that has been _staged but never committed_ across many
sessions. The result: `git status` is unreadable, and a naive "commit and push all" would have
reverted newer origin governance content or swept in the foreign WIP.

## Why now

The divergence just bit a real "commit and push" request and forced a careful `reset --soft`
recovery; the pattern will recur every parity cycle because the base-worktree method is the standard
way sibling repos land. Codifying one terminal step and one WIP-parking rule is cheap and stops the
whole class.

## Prior art / precedents

- **`git merge --ff-only`** — the exact primitive a terminal reconcile step would use: _"resolve the
  merge as a fast-forward when possible... refuse to merge"_ otherwise. [git-merge](https://git-scm.com/docs/git-merge)
- **`git worktree`** — linked working trees against a bare repo, the mechanism whose pushes bypass
  local `main`. [git-worktree](https://git-scm.com/docs/git-worktree)
- **bare-repo delivery-mode governance hardening** — the sibling brief covering bare-repo _doc_
  gaps; this one covers the disjoint _workflow-hygiene_ gap. [brief](./bare-repo-delivery-mode-governance-hardening.md)
- **No Destructive Git Operations** — the guardrail this extends: `reset --soft` reconciles safely,
  `reset --hard` would destroy the staged WIP. [no-destructive-git-operations](../../repo-governance/development/workflow/no-destructive-git-operations.md)

## Proposed direction (sketch)

Three cheap rules, propagated public → siblings:

1. **Terminal FF step** — make `git fetch && git merge --ff-only origin/main` (or `reset --soft
origin/main` when a stale dup commit blocks the FF) the documented last step of the base-worktree
   landing method, so local `main` never lags origin.
2. **No stale-base commits on local main** — land via the worktree _or_ via an FF'd local `main`,
   never both; the duplicate commit is what created the "+1 ahead."
3. **Park long-lived WIP off the index** — long-lived foreign WIP belongs on a `wip/*` branch or a
   named stash, not staged-uncommitted for sessions on end where one `reset --hard` destroys it and
   it pollutes every status read.

## Rough scope & non-goals

In scope: the three workflow rules above, folded into the bare-repo git-ops method doc and the
worktree-cleanup convention, then propagated to the siblings.

Out of scope (for now): a rhino-cli guard that auto-detects post-push local-main lag (possible
follow-up, not needed to codify the rule); changing the base-worktree method itself.

## Risks & open questions

- Where does rule 3 bind — a convention, or just the git-ops method doc? The foreign rhino-cli WIP is
  someone else's active work, so the rule must be advisory, not an automated stash. (open)
- Is a tooling guard (detect lag / unparked WIP) worth it, or does the documented step suffice? (open)

## What success looks like + promotion signal

Success: after any sibling landing, `origin/main...HEAD` reads `0 0` without manual recovery, and a
bare sibling's `git status` shows only genuinely-active WIP. Ready to promote once rule 3's binding
surface (convention vs. method doc) is decided — the other two are settled doc edits.
