# 6 · Version Control & Git (By Example, Git †)

**prd row**: Pass 1 · Core Foundations · By Example · Git † · Learn 106 / Drill 206 · Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: Git as the working engineer's memory and collaboration substrate — building an
object-model intuition (commits/trees/blobs/refs), everyday CLI fluency, branching with merge and
rebase, and a pull-request / trunk-based flow. `†`: everything is driven from the `git` command line,
no GUI. The build-your-own-Git pass that reconstructs the content-addressed object store from scratch
lives at [`86-build-your-own-git`](./86-build-your-own-git.md).

## Why this exists · the big idea

- **The problem before the solution**: before version control, coordinating change meant zipped
  folders, `final_v2_REALLY_final` filenames, and silently overwriting a colleague's work with no way
  back — there was no shared, trustworthy record of who changed what and why.
- **Keep-this-if-you-forget-everything**: a commit is an immutable snapshot of the whole tree,
  identified by the hash of its content, and a branch is just a movable pointer to one commit — once
  that clicks, merge, rebase, reset, and reflog stop being magic and become graph operations.
- **Big ideas touched**: `coupling-vs-cohesion` (a branch-and-PR flow keeps changes that belong
  together in one reviewable commit and unrelated work apart), `correctness-vs-pragmatism` (rebase
  buys a clean linear history but rewrites shared state — the disciplined compromise is rebase local
  work, merge shared work).

## Prerequisites

- **Prior topics**: [topic 4 Just Enough Python](./04-just-enough-python.md) and
  [topic 5 Just Enough Bash](./05-just-enough-bash.md).
- **Tools & environment**: a macOS/Linux terminal; **Git** at a recent stable release; a GitHub (or
  equivalent) account for the remote/PR flow; Neovim/VSCode with Git integration for diffs and blame
  (DD-17).
- **Assumed knowledge**: navigating a filesystem and running CLI tools (topic 05); reading a small
  script well enough to understand a commit hook (topic 04).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: Git's core CLI (`add`/`commit`/`branch`/`merge`/`rebase`/`log`/`reflog`) and
  object model (blob/tree/commit/tag, hash-addressed storage) are stable and correctly left
  version-unpinned. Git's SHA-1 → SHA-256 object-format transition is still opt-in/experimental —
  describe the store as hash-addressed without asserting the default hash is already SHA-256.
- 2026-07-12 — verified: "trunk-based development" and the pull-request review flow described here
  match current mainstream practice; there is no version claim to pin.

## Items

- The three states + object model: working tree, staging area (index), and the commit graph, and how
  blobs/trees/commits/refs relate.
- Everyday CLI fluency: `status`/`add`/`commit`/`diff`/`log`, staging in hunks, amending, and writing
  a good commit message.
- Branching and integration: create/switch branches, fast-forward vs three-way merge, and resolving a
  conflict deliberately.
- Rebase vs merge: interactive rebase to curate local history, when rebasing is safe, and the "never
  rebase shared history" rule.
- The undo toolkit: `reset` (soft/mixed/hard), `revert`, `restore`, `stash`, and `reflog` as the
  safety net.
- Remotes and collaboration: `fetch`/`pull`/`push`, tracking branches, and a pull-request +
  trunk-based flow.

## Tensions & trade-offs — when NOT to reach for this

- **Rebase vs merge is not free**: rebasing rewrites commit identity, so rebasing a branch others have
  already pulled forces everyone to reconcile — the clean linear history is worth it locally and a
  liability once shared.
- **History rewriting has a blast radius**: `reset --hard`, force-push, and history-rewriting filters
  can destroy work; the reflog rescues you locally but not a force-pushed remote. When in doubt prefer
  additive operations (`revert`) over destructive ones.
- **Git is not a large-binary store**: it snapshots whole file content, so large binaries bloat every
  clone forever. Reach for LFS or an artifact store instead — this is a "when not to put it in Git"
  boundary, not a tuning knob.

## Lineage — why it beat the alternative

- Centralized version control (CVS, then Subversion) put history on one server: every commit needed
  the network, branching was expensive, and the server was a single point of failure. Git (2005, built
  for Linux-kernel development) inverted this — every clone is a full repository with the entire
  history, commits are local and cheap, and content-addressed storage makes integrity and
  de-duplication fall out for free. Distributed, near-free branching is precisely what made the
  pull-request and trunk-based workflows practical. This topic hands its object-model intuition to
  [`86-build-your-own-git`](./86-build-your-own-git.md), which rebuilds the content-addressed store,
  and its collaboration flow to [`52-cicd-and-release-engineering`](./52-cicd-and-release-engineering.md),
  which automates the path from commit to production.

## Worked examples

Colocated under `version-control-and-git/learning/code/`; each step is a real repo you build and
inspect from the `git` CLI (DD-20/DD-30).

- **beginner** — initialize a repo, stage in hunks, and craft a clean series of commits; inspect the
  resulting object graph with `git log --graph` and `git cat-file`.
- **intermediate** — branch, create a merge conflict on purpose and resolve it, then redo the same
  integration with an interactive rebase and compare the two histories.
- **advanced** — simulate a mistake (a bad `reset --hard`), recover it via `reflog`; then run a small
  PR/trunk-based flow against a remote with a pre-commit hook.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: take a small project from empty directory to a reviewed, merged change on trunk — a
  curated commit history, a resolved conflict, a recovered mistake, and a pull-request merge — proving
  Git fluency end to end.
- **Concepts exercised**: [ ] object-model inspection (`cat-file`/`log --graph`) [ ] commits staged in
  hunks with good messages [ ] a branch + three-way merge with a resolved conflict [ ] an interactive
  rebase [ ] a `reflog` recovery [ ] a PR/trunk-based merge to a remote.
- **Ordered steps**:
  1. `.../learning/capstone/code/` — init the repo and build a three-to-four-commit history staged in
     hunks. Verify `git log --graph` shows the intended graph and `git cat-file -p` resolves a commit
     to its tree and blobs.
  2. Branch, force a conflict against trunk, and resolve it with a merge. Verify the merge commit has
     two parents and the resulting tree is correct.
  3. On a second branch, curate history with an interactive rebase, then recover a deliberate
     `reset --hard` via `reflog`. Verify the "lost" commit is restored.
  4. Push to a remote and land the change through a pull request onto trunk with a passing pre-commit
     hook. Verify trunk contains the change and the history is intact.
- **Acceptance criteria**: the object graph matches intent; the conflict resolves correctly; the rebase
  produces the curated history; the reflog recovery restores the commit; the PR merges to trunk.
- **Done bar**: runnable end-to-end + web-verified.

## Read more

**Books**

- **Pro Git** — Scott Chacon, Ben Straub (2nd ed.). Canonical free book on Git: data model, branching,
  workflows, internals. <https://git-scm.com/book/en/v2>

**Papers & articles**

- **Git Reference Documentation** — The Git Project. Official command reference plus the "Git
  Internals" plumbing/porcelain and object-storage chapters. <https://git-scm.com/docs>
- **"A Note About Git Commit Messages"** — Tim Pope (2008). Widely cited article establishing the
  seven-rules commit-message convention.
  <https://tbaggery.com/2008/04/19/a-note-about-git-commit-messages.html>

---

← Previous: [5 · Just Enough Bash](./05-just-enough-bash.md) · Next: [7 · Data Structures & Algorithms Essentials](./07-data-structures-and-algorithms-essentials.md) →
