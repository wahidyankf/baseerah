# Delivery — Learn-Tree Reorganization

The executor's working file. Each `- [ ]` item is an atomic step. Tick it the moment it is done; do not batch.

Three iron rules:

1. **One phase per commit group**. A phase ends with a commit (or small commit cluster) and the three validation commands at exit 0.
2. **Never edit the main checkout**. All work happens inside `worktrees/ayokoding-web-learn-reorg/`.
3. **`git mv`, always**. No plain `mv` followed by `git add`.

## Phase 0 — Worktree and Baseline

- [ ] Create the worktree: `cd /Users/wkf/ose-projects/ose-public && claude --worktree ayokoding-web-learn-reorg`
- [ ] Inside the worktree run `npm install`
- [ ] Inside the worktree run `npm run doctor -- --fix`
- [ ] Inside the worktree run the three validation commands and capture exit codes as baseline:
  - [ ] `cd apps/ayokoding-web && ../../apps/ayokoding-cli/dist/ayokoding-cli links check --content content` (expect exit 0)
  - [ ] `cd apps/ayokoding-web && node --import tsx src/scripts/generate-indexes.ts --validate` (expect exit 0)
  - [ ] `nx run ayokoding-web:test:quick` (expect exit 0)
- [ ] Inventory Gherkin specs that reference `/en/learn/...` URLs: `rg "/en/learn/" apps/ayokoding-web/specs > /tmp/learn-reorg-spec-refs.txt`
- [ ] Inventory governance docs that reference learn paths: `rg "platform-linux\|platform-web\|platform-mobile\|/en/learn/human" repo-governance docs > /tmp/learn-reorg-gov-refs.txt`
- [ ] Commit baseline notes if any tooling tweaks needed: `chore(ayokoding-web): prepare for learn-tree reorg`

## Phase 1 — Redirect Plumbing Skeleton (Make-It-Fail First)

The redirect file lands before any rename. Empty/skeleton, but wired in — so phase 2's renames have a target to push entries into immediately.

- [ ] Create `apps/ayokoding-web/src/redirects/learn-reorg.ts` with `export const learnReorgRedirects: Array<{source: string; destination: string; permanent: boolean}> = [];`
- [ ] Edit `apps/ayokoding-web/next.config.ts` to import and spread `learnReorgRedirects` into `redirects()`
- [ ] Run `nx build ayokoding-web` to confirm Next.js still builds with the empty array
- [ ] Add a unit-style smoke test or at least one Playwright spec asserting that a known unredirected path still 200s (e.g., `/en/learn/software-engineering/programming-languages/typescript`) — establishes the "no redirect yet" baseline
- [ ] Run `nx run ayokoding-web:test:quick`
- [ ] Commit: `feat(ayokoding-web): scaffold learn-reorg redirect map`

## Phase 2 — Platforms Rename (`platform-*` → `platforms/*`)

Most-used area in software-engineering. Doing it first proves the mechanics on a high-traffic surface.

- [ ] `git mv apps/ayokoding-web/content/en/learn/software-engineering/platform-linux apps/ayokoding-web/content/en/learn/software-engineering/platforms/linux` (creates `platforms/` directory)
- [ ] `git mv apps/ayokoding-web/content/en/learn/software-engineering/platform-web apps/ayokoding-web/content/en/learn/software-engineering/platforms/web`
- [ ] `git mv apps/ayokoding-web/content/en/learn/software-engineering/platform-mobile apps/ayokoding-web/content/en/learn/software-engineering/platforms/mobile`
- [ ] Repeat the three `git mv` calls for `content/id/learn/software-engineering/platform-*` paths if they exist (check with `ls apps/ayokoding-web/content/id/learn/software-engineering/ 2>/dev/null`)
- [ ] Add `platforms/_index.md` and `platforms/overview.md` (curated, not generated) explaining the area
- [ ] Cross-link rewrite for each old prefix:
  - [ ] `/en/learn/software-engineering/platform-linux` → `/en/learn/software-engineering/platforms/linux`
  - [ ] `/en/learn/software-engineering/platform-web` → `/en/learn/software-engineering/platforms/web`
  - [ ] `/en/learn/software-engineering/platform-mobile` → `/en/learn/software-engineering/platforms/mobile`
  - [ ] Repeat for `/id/learn/...` if applicable
- [ ] Append three redirect entries to `learn-reorg.ts` (en) and three more (id) if Indonesian content was touched
- [ ] `node --import tsx apps/ayokoding-web/src/scripts/generate-indexes.ts`
- [ ] Run link-check; fix any miss
- [ ] Run `nx run ayokoding-web:test:quick`
- [ ] Sample-check `git log --follow apps/ayokoding-web/content/en/learn/software-engineering/platforms/web/_index.md` reaches the pre-rename history
- [ ] Commit: `refactor(ayokoding-web): rename platform-* to platforms/{linux,web,mobile}`

## Phase 3 — `algorithm-and-data-structures` Grammar Fix

- [ ] `git mv apps/ayokoding-web/content/en/learn/software-engineering/algorithm-and-data-structures apps/ayokoding-web/content/en/learn/software-engineering/algorithms-and-data-structures`
- [ ] Repeat for `content/id/learn/...` if applicable
- [ ] Cross-link rewrite for the prefix change
- [ ] Append redirect entry to `learn-reorg.ts`
- [ ] Regenerate indexes, run link-check, run `test:quick`
- [ ] Commit: `refactor(ayokoding-web): rename algorithm-and-data-structures to algorithms-and-data-structures`

## Phase 4 — `human/` → `personal-development/` Domain Rename

- [ ] `git mv apps/ayokoding-web/content/en/learn/human apps/ayokoding-web/content/en/learn/personal-development`
- [ ] Repeat for `content/id/learn/human` if applicable
- [ ] Cross-link rewrite for `/en/learn/human` → `/en/learn/personal-development` (and id)
- [ ] Append redirect entry to `learn-reorg.ts`
- [ ] Update `apps/ayokoding-web/content/en/learn/_index.md` "Human" entry wording to "Personal Development" if hand-curated text exists post-regen
- [ ] Update `learn/overview.md` "Human Development" line to "Personal Development"
- [ ] Regenerate indexes, run link-check, run `test:quick`
- [ ] Commit: `refactor(ayokoding-web): rename human domain to personal-development`

## Phase 5 — Information-Security Track Normalization

Three sub-moves; order matters because `concepts/explanation/` must move before `concepts/` is deleted.

- [ ] `git mv apps/ayokoding-web/content/en/learn/information-security/concepts/explanation apps/ayokoding-web/content/en/learn/information-security/by-concept` (creates `by-concept/` directory if absent)
- [ ] Remove now-empty `information-security/concepts/` if it has no remaining content; if it has, fold remaining files into `information-security/by-concept/` via `git mv` per file and then remove
- [ ] `git mv apps/ayokoding-web/content/en/learn/information-security/foundations/by-example apps/ayokoding-web/content/en/learn/information-security/by-example/foundations`
- [ ] Remove now-empty `information-security/foundations/` (move any non-`by-example` content into `by-concept/foundations/` first)
- [ ] Cross-link rewrite for the three old prefixes
- [ ] Append redirect entries to `learn-reorg.ts`
- [ ] Update `information-security/_index.md` curated TOC entries (post-regen diff)
- [ ] Regenerate indexes, run link-check, run `test:quick`
- [ ] Commit: `refactor(ayokoding-web): fold information-security concepts and foundations into canonical tracks`

## Phase 6 — Infrastructure `concepts/` Fold-In

- [ ] Inventory `software-engineering/infrastructure/concepts/` contents: `find apps/ayokoding-web/content/en/learn/software-engineering/infrastructure/concepts -type f`
- [ ] Classify each file as conceptual (→ `by-concept/`) or action-oriented (→ `by-example/`); the existing `concepts/how-to/` sub-tree is action-oriented by definition
- [ ] `git mv apps/ayokoding-web/content/en/learn/software-engineering/infrastructure/concepts/how-to apps/ayokoding-web/content/en/learn/software-engineering/infrastructure/by-example` (merging contents; resolve any name collisions explicitly)
- [ ] `git mv` remaining `concepts/*` files into `infrastructure/by-concept/`
- [ ] Remove the now-empty `infrastructure/concepts/`
- [ ] Cross-link rewrite for the two old prefixes
- [ ] Append redirect entries to `learn-reorg.ts`
- [ ] Regenerate indexes, run link-check, run `test:quick`
- [ ] Commit: `refactor(ayokoding-web): fold infrastructure concepts into canonical tracks`

## Phase 7 — `cases/` Subfolders Into `by-example/cases/`

- [ ] `git mv apps/ayokoding-web/content/en/learn/software-engineering/software-architecture/cases apps/ayokoding-web/content/en/learn/software-engineering/software-architecture/by-example/cases`
- [ ] `git mv apps/ayokoding-web/content/en/learn/software-engineering/system-design/cases apps/ayokoding-web/content/en/learn/software-engineering/system-design/by-example/cases`
- [ ] Cross-link rewrite for the two old prefixes
- [ ] Append redirect entries to `learn-reorg.ts`
- [ ] Regenerate indexes, run link-check, run `test:quick`
- [ ] Update `software-architecture/overview.md` and `system-design/overview.md` to cross-link and state the split per PRD §FR-4
- [ ] Commit: `refactor(ayokoding-web): move architecture and system-design cases under by-example`

## Phase 8 — Governance and Specs Sweep

By now content is reshaped. Governance docs, agent skills, and Gherkin specs likely still reference old paths.

- [ ] Compare `/tmp/learn-reorg-gov-refs.txt` (from Phase 0) against current paths; update any remaining old references
- [ ] Search agent definitions for old patterns: `rg "platform-linux\|platform-web\|platform-mobile\|concepts/explanation\|foundations/by-example\|/en/learn/human\|software-architecture/cases\|system-design/cases" .claude/agents .claude/skills .opencode/agents`
- [ ] Update any agent definition that hard-codes an old path
- [ ] Compare `/tmp/learn-reorg-spec-refs.txt` against current paths; update Gherkin specs
- [ ] Run `npm run sync:claude-to-opencode` if any `.claude/agents/` files changed
- [ ] Run `nx affected -t lint typecheck test:quick spec-coverage`
- [ ] Commit: `chore(ayokoding-web): update governance docs, agents, and specs for learn reorg`

## Phase 9 — End-to-End Redirect Verification

- [ ] Run `nx build ayokoding-web` and start `nx serve ayokoding-web` (or `nx dev`)
- [ ] For each redirect entry in `learn-reorg.ts`, run `curl -IL http://localhost:<port>/<old-url>` and confirm:
  - [ ] Status is 308 (Next.js permanent default) or 301 depending on `permanent: true` flag wiring
  - [ ] `Location` header points at the new URL with `:path*` correctly preserved
  - [ ] Following the redirect reaches a 200
- [ ] Random spot-check three nested cases that should cascade (e.g., `platform-linux/<subpath>` and `concepts/explanation/<subpath>`)
- [ ] Commit any redirect-map fixes: `fix(ayokoding-web): repair redirect entries discovered in verification`

## Phase 10 — Final Local Gate

- [ ] `cd apps/ayokoding-web && ../../apps/ayokoding-cli/dist/ayokoding-cli links check --content content` — exit 0
- [ ] `cd apps/ayokoding-web && node --import tsx src/scripts/generate-indexes.ts --validate` — exit 0
- [ ] `nx affected -t typecheck lint test:quick spec-coverage` — all green
- [ ] Sample five random renamed files; for each run `git log --follow --format=%H -- <new-path> | tail -1` and confirm history reaches pre-reorg
- [ ] Tree check: `find apps/ayokoding-web/content -type d \( -name concepts -o -name explanation -o -name foundations -o -name cases -o -name 'platform-*' \) | grep -v by-example/cases` returns empty
- [ ] Tree check: `find apps/ayokoding-web/content/en/learn -type d -name human` returns empty

## Phase 11 — Publish to `main` (Direct-to-Main per TBD)

- [ ] Inside the worktree, ensure all phases committed: `git status` clean
- [ ] From the main checkout (NOT the worktree): `git fetch origin && git checkout main && git pull --ff-only origin main`
- [ ] Merge worktree branch into main fast-forward: `git merge --ff-only worktree-ayokoding-web-learn-reorg`
- [ ] Push: `git push origin main` (pre-push hook runs again as final gate)
- [ ] Wait for hook to pass; if it does not, do NOT `--no-verify` — investigate root cause and add a new phase to fix

## Phase 12 — Promote to Production

- [ ] Delegate to the `apps-ayokoding-web-deployer` agent. Brief: "Reorg landed at SHA `<main-tip>`. Promote `prod-ayokoding-web` to `origin/main`. Vercel will rebuild ayokoding.com."
- [ ] After Vercel build completes, verify in production:
  - [ ] `curl -IL https://ayokoding.com/en/learn/software-engineering/platform-web` returns 308/301 with Location `…/platforms/web`
  - [ ] `curl -IL https://ayokoding.com/en/learn/human` returns 308/301 with Location `…/personal-development`
  - [ ] `curl -IL https://ayokoding.com/en/learn/information-security/concepts/explanation` returns 308/301 with Location `…/information-security/by-concept`
- [ ] Spot-check three pages render correctly in a browser

## Phase 13 — Archive

- [ ] Wait for the gitlink at parent (ose-projects) to track `origin/main` containing the merge SHA (only relevant when parent bump is needed; this plan does not require one because it only modifies subrepo content)
- [ ] `git mv ose-public/plans/in-progress/ayokoding-web-learn-reorg ose-public/plans/done/2026-MM-DD__ayokoding-web-learn-reorg` (use the actual completion date)
- [ ] Commit: `chore(plans): archive ayokoding-web-learn-reorg`
- [ ] Push

## Rollback Plan

If a phase introduces breakage that surfaces after merge to main:

- The phases are designed to be reverted individually. `git revert <phase-commit>` reverses one phase's renames and removes that phase's redirect entries.
- Reverting Phase 2 (`platforms/`) requires reverting Phase 7's `software-architecture/by-example/cases/` only if the cases-under-by-example move depended on a path that no longer exists post-revert — re-check.
- A full rollback is: `git revert <SHA-range>` for all phase commits in reverse order, then `git push origin main`. Vercel re-deploys on push.

## After-Action

- [ ] Open a follow-up plan in `plans/backlog/` if Indonesian content (`content/id/`) needs the same treatment but was deferred
- [ ] Open a follow-up plan if `apps-ayokoding-web-{by-concept,by-example,in-the-field}-checker` agents need new rules that enforce the canonical shape going forward
- [ ] Update `apps-ayokoding-web-developing-content` skill to reference the canonical shape if it does not already
