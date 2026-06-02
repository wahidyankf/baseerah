# Delivery — ose-web-remove-ddd

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.
>
> **Phase Gate** — every phase ends with a `### Phase N Gate` (must-pass verification) plus a
> `> **Pause Safety**:` note (the safe-to-stop state and the single command to resume). A phase is
> not complete until its gate is green; do not start phase N+1 while any gate check fails.

## Worktree

Worktree path: `worktrees/ose-web-remove-ddd/`

Provision before execution (run from repo root):

```bash
claude --worktree ose-web-remove-ddd
```

Then initialize the toolchain (run in the **root** worktree, per
[Worktree Toolchain Initialization](../../../repo-governance/development/workflow/worktree-setup.md)):

```bash
npm install && npm run doctor -- --fix
```

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

---

## Phase 0: Environment Setup and Baseline

> _Executor: repo-setup-manager_

- [ ] [AI] Provision the worktree from repo root: `claude --worktree ose-web-remove-ddd`
      — acceptance: `worktrees/ose-web-remove-ddd/` exists and is a valid git worktree.
- [ ] [AI] Install dependencies in the root worktree: `npm install`
      — acceptance: exits 0, `node_modules/` synchronized.
- [ ] [AI] Converge the toolchain in the root worktree: `npm run doctor -- --fix`
      — acceptance: exits 0 with no unresolved drift (Rust + Node toolchains present).
- [ ] [AI] Record the current `apps_with_ddd()` length for the relative edit: run
      `grep -n "v.len()" apps/rhino-cli/src/internal/allowlist.rs`
      — acceptance: the expected-length integer N is recorded in the execution log (today N = 5,
      verify at execution time). [Repo-grounded]
- [ ] [AI] Establish the `ose-web` + `rhino-cli` baseline:
      `npx nx run-many -t typecheck lint test:quick spec-coverage -p ose-web rhino-cli`
      — acceptance: baseline pass/fail recorded; all preexisting failures documented.
- [ ] [AI] Establish the `rhino-cli` cargo baseline:
      `cargo test --manifest-path apps/rhino-cli/Cargo.toml`
      — acceptance: pass/fail recorded; `membership` test currently passes.
- [ ] [AI] Resolve all preexisting failures before proceeding
      — acceptance: no preexisting failures remain unresolved.

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [ ] [AI] `npm install` exited 0 and `npm run doctor -- --fix` reports no unresolved drift.
- [ ] [AI] `npx nx run-many -t typecheck lint test:quick spec-coverage -p ose-web rhino-cli`
      baseline recorded and every preexisting failure resolved (zero unresolved).
- [ ] [AI] `cargo test --manifest-path apps/rhino-cli/Cargo.toml` baseline green.
- [ ] [AI] The current `apps_with_ddd()` expected length N is recorded.

> **Pause Safety**: only the local toolchain was verified and the baseline recorded — no feature
> work exists yet. Safe to stop indefinitely. To resume: re-run the two baseline commands and
> confirm they are still clean.

---

## Phase 1: Remove rhino-cli DDD allowlist entry (RED→GREEN)

> _Suggested executor: `swe-rust-dev`_

This phase is a natural RED→GREEN: removing the slice entry makes the `membership` test fail (RED);
decrementing the expected length makes it pass (GREEN).

- [ ] [AI] **RED** — Edit `apps/rhino-cli/src/internal/allowlist.rs`: remove the line
      `"ose-platform",` from the `apps_with_ddd()` slice (line ~23). Run
      `cargo test --manifest-path apps/rhino-cli/Cargo.toml membership`
      — acceptance: the `membership` test FAILS with a length mismatch (slice now has N-1 entries
      but the assertion still expects N). This confirms the test guards the entry.
- [ ] [AI] **GREEN** — In the same file, decrement the `assert_eq!(v.len(), N);` in the
      `#[cfg(test)] mod tests::membership` test by 1 (line ~38), using the N recorded in Phase 0
      (today: `5` → `4`). Run `cargo test --manifest-path apps/rhino-cli/Cargo.toml membership`
      — acceptance: the `membership` test PASSES; it still asserts `contains` for `organiclever`,
      `ayokoding`, `ose-app`. Do NOT add an `ose-platform`-related assertion. [Repo-grounded]
- [ ] [AI] **REFACTOR** — In the same file, remove the verbatim doc-comment line `//!   - ose-platform: bounded-contexts.yaml + feature files present` from the module doc block (line ~10 of `apps/rhino-cli/src/internal/allowlist.rs`). Run
      `cargo test --manifest-path apps/rhino-cli/Cargo.toml` — acceptance: the full `rhino-cli` test suite passes; `grep -n "ose-platform" apps/rhino-cli/src/`
      returns zero matches.
- [ ] [AI] Rebuild `rhino-cli` (it is a pre-push dependency for other apps):
      `npx nx build rhino-cli`
      — acceptance: build exits 0.

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [ ] [AI] `cargo test --manifest-path apps/rhino-cli/Cargo.toml` — all tests pass.
- [ ] [AI] `grep -n "ose-platform" apps/rhino-cli/src/` — zero matches.
- [ ] [AI] `npx nx build rhino-cli` — exits 0.

> **Pause Safety**: `rhino-cli` no longer lists `ose-platform` as a DDD app, its tests are green,
> and it is rebuilt. `ose-web` still references the (still-present) DDD specs, so the tree is
> coherent. Safe to stop. To resume: re-run `cargo test --manifest-path apps/rhino-cli/Cargo.toml`.

---

## Phase 2: De-DDD the ose-web pre-push gate (RED→GREEN)

> _Suggested executor: `swe-typescript-dev`_

- [ ] [AI] **RED** — Confirm the guard target exists: run
      `grep -nE "ddd (bc|ul) ose-platform|specs/apps/ose-platform/ddd/" apps/ose-web/project.json`
      — acceptance: exactly the two command lines (`ddd bc ose-platform`, `ddd ul ose-platform`)
      and the two `inputs` globs match. This documents the old behavior to be removed.
- [ ] [AI] **GREEN** — Edit `apps/ose-web/project.json`: in the `test:quick` target's `commands`
      array, delete the two lines
      `"(cd ../../apps/rhino-cli && cargo run --release --quiet -- ddd bc ose-platform)",` and
      `"(cd ../../apps/rhino-cli && cargo run --release --quiet -- ddd ul ose-platform)",`
      (lines ~72-73). Keep the vitest+coverage line (threshold `86`) and the `ose-cli links check`
      line intact. Then in the `test:quick` `inputs` array delete the two globs
      `"{workspaceRoot}/specs/apps/ose-platform/ddd/bounded-contexts.yaml",` and
      `"{workspaceRoot}/specs/apps/ose-platform/ddd/ubiquitous-language/**/*.md",` (lines ~86-87).
      Keep the `src`, `test`, `content`, `vitest`, `behavior/web/gherkin`, and `behavior/api/gherkin`
      inputs unchanged. Verify with
      `grep -nE "ddd (bc|ul) ose-platform|specs/apps/ose-platform/ddd/" apps/ose-web/project.json`
      — acceptance: zero matches.
- [ ] [AI] **REFACTOR** — Validate the JSON is still well-formed and Nx can read the target:
      `npx nx show project ose-web --json | jq -e '.targets."test:quick"'`
      — acceptance: command exits 0 and prints the `test:quick` target (no parse error).

### Phase 2 Gate

> All checks below must pass before starting Phase 3. (`test:quick` will still fail here because the
> `ddd/` specs the registry referenced are deleted only in Phase 3 — so this gate validates only
> config integrity, not a full `test:quick` run.)

- [ ] [AI] `grep -nE "ddd (bc|ul) ose-platform|specs/apps/ose-platform/ddd/" apps/ose-web/project.json`
      — zero matches.
- [ ] [AI] `npx nx show project ose-web --json | jq -e '.targets."test:quick".options.commands | length'`
      — prints the reduced command count (two fewer than baseline).
- [ ] [AI] `npx nx run ose-web:typecheck` — exits 0 (config edit does not affect typecheck).

> **Pause Safety**: `project.json` no longer invokes the DDD validators and the JSON is valid. The
> `ddd/` spec directory still exists (deleted next phase), so nothing dangles. Safe to stop. To
> resume: re-run the grep guard above.

---

## Phase 3: Delete the DDD spec registry (RED→GREEN)

> _Suggested executor: `specs-fixer`_

- [ ] [AI] **RED** — Confirm the directory still exists:
      `test -d specs/apps/ose-platform/ddd && echo PRESENT`
      — acceptance: prints `PRESENT` (the 11-file registry is still there).
- [ ] [AI] **GREEN** — Delete the whole DDD registry:
      `git rm -r specs/apps/ose-platform/ddd`
      — acceptance: `test -d specs/apps/ose-platform/ddd` exits non-zero; `git status` shows 11
      deleted files (`bounded-contexts.yaml`, `bounded-context-map.md`, `README.md`, and the eight
      `ubiquitous-language/*.md` files). [Repo-grounded]
- [ ] [AI] **REFACTOR** — Verify the rest of the `ose-platform` spec tree is intact and no tracked
      file still references the deleted path:
      `test -d specs/apps/ose-platform/system-context && test -d specs/apps/ose-platform/behavior && git grep -n "ose-platform/ddd" -- ':!worktrees' ':!**/.nx/**'`
      — acceptance: the two `test -d` checks pass; `git grep` returns zero matches outside the
      `apps/ose-web/README.md` (which is rewritten in Phase 5).

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [ ] [AI] `test -d specs/apps/ose-platform/ddd` — exits non-zero.
- [ ] [AI] `git grep -n "ose-platform/ddd" -- ':!worktrees' ':!**/.nx/**' ':!apps/ose-web/README.md'`
      — zero matches (README handled in Phase 5).
- [ ] [AI] `npx nx run ose-web:spec-coverage` — exits 0 (behavior specs untouched).

> **Pause Safety**: the DDD registry is gone, `project.json` no longer references it, and
> `spec-coverage` (behavior-only) still passes. The README still has DDD prose (fixed next).
> Safe to stop. To resume: re-run the `git grep` guard above.

---

## Phase 4: Delete the empty domain layers (RED→GREEN)

> _Suggested executor: `swe-typescript-dev`_

- [ ] [AI] **RED** — Confirm the seven empty `domain/` folders still exist and are stubs:
      `find apps/ose-web/src/contexts -type d -name domain | wc -l` and
      `grep -rL "export {};" apps/ose-web/src/contexts/*/domain/index.ts`
      — acceptance: `find` prints `7`; the `grep -rL` prints nothing (every barrel is exactly the
      stub `export {};`). [Repo-grounded]
- [ ] [AI] **GREEN** — Delete all seven `domain/` folders:
      `git rm -r apps/ose-web/src/contexts/app-shell/domain apps/ose-web/src/contexts/content/domain apps/ose-web/src/contexts/health/domain apps/ose-web/src/contexts/landing/domain apps/ose-web/src/contexts/rss-feed/domain apps/ose-web/src/contexts/search/domain apps/ose-web/src/contexts/seo/domain`
      — acceptance: `find apps/ose-web/src/contexts -type d -name domain | wc -l` prints `0`.
- [ ] [AI] **REFACTOR** — Confirm nothing imported from the deleted barrels and typecheck is clean:
      `git grep -n "contexts/[a-z-]*/domain" -- 'apps/ose-web/src' ; npx nx run ose-web:typecheck`
      — acceptance: `git grep` returns zero matches; `typecheck` exits 0.

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [ ] [AI] `find apps/ose-web/src/contexts -type d -name domain | wc -l` — prints `0`.
- [ ] [AI] `npx nx run ose-web:typecheck` — exits 0.
- [ ] [AI] `npx nx run ose-web:test:quick` — exits 0 (now that the DDD validators and `ddd/` specs
      are gone, the full gate runs clean). [Repo-grounded: command array de-DDD'd in Phase 2]

> **Pause Safety**: all DDD scaffolding (specs, validators, domain stubs) is removed and `ose-web`
> typechecks and passes `test:quick`. Only the README prose remains DDD-framed. Safe to stop. To
> resume: re-run `npx nx run ose-web:test:quick`.

---

## Phase 5: Rewrite the ose-web README (RED→GREEN)

> _Suggested executor: `readme-fixer`_

Rewrite the Architecture, Project-Structure, Specs, and Bounded-Contexts sections of
`apps/ose-web/README.md` to describe hexagonal feature modules (three layers: `application`,
`infrastructure`, `presentation`) per
`repo-governance/development/pattern/hexagonal-architecture-web.md`. Remove the `DDD` Architecture
bullet, the "DDD bounded contexts" Project-Structure comment, the entire "Bounded Contexts" table,
the `ddd/bounded-contexts.yaml` and `ddd/ubiquitous-language/` Specs rows, and the "schema v2" and
"Per-BC" phrasing. Update the "test:quick" comment so it no longer says "DDD validators".

- [ ] [AI] **RED** — Document the DDD terms currently present:
      `grep -nE "DDD|bounded context|Bounded Context|Per-BC|schema v2|ddd/bounded-contexts|ddd/ubiquitous-language" apps/ose-web/README.md`
      — acceptance: matches are found (the Architecture DDD bullet, the Bounded Contexts table, the
      two ddd Specs rows, etc.). [Repo-grounded]
- [ ] [AI] **GREEN** — Edit `apps/ose-web/README.md` to remove all DDD framing and describe the
      hexagonal feature-module architecture; add a link to
      `../../repo-governance/development/pattern/hexagonal-architecture-web.md`. Re-run the grep:
      `grep -nE "DDD|bounded context|Bounded Context|Per-BC|schema v2|ddd/bounded-contexts|ddd/ubiquitous-language" apps/ose-web/README.md`
      — acceptance: zero matches; the README references the three real layers and links the
      governance doc.
  - _Suggested executor: `readme-fixer`_
- [ ] [AI] **REFACTOR** — Lint/format the markdown:
      `npm run lint:md:fix && npm run format:md`
      — acceptance: both exit 0; `apps/ose-web/README.md` passes markdownlint.

### Phase 5 Gate

> All checks below must pass before starting Phase 6.

- [ ] [AI] `grep -nE "DDD|bounded context|Per-BC|schema v2|ose-platform/ddd" apps/ose-web/README.md`
      — zero matches.
- [ ] [AI] `grep -n "hexagonal-architecture-web.md" apps/ose-web/README.md` — at least one match.
- [ ] [AI] `npm run lint:md` — exits 0 for `apps/ose-web/README.md`.

> **Pause Safety**: all five change groups are now applied and the README is accurate. The tree is
> fully coherent. Safe to stop. To resume: proceed to the Phase 6 quality gates.

---

## Phase 6: Quality Gates, Manual Verification, Commit, and Push

> _Suggested executor: `ci-fixer` for CI follow-ups_

### Local Quality Gates (Before Push)

> **Important**: Fix ALL failures found during quality gates, not just those caused by your
> changes. This follows the root cause orientation principle — proactively fix preexisting errors
> encountered during work. Commit preexisting fixes separately with appropriate conventional
> commit messages.

- [ ] [AI] Run affected typecheck: `npx nx affected -t typecheck` — exits 0.
- [ ] [AI] Run affected linting: `npx nx affected -t lint` — exits 0.
- [ ] [AI] Run affected quick tests: `npx nx affected -t test:quick` — exits 0.
- [ ] [AI] Run affected spec coverage: `npx nx affected -t spec-coverage` — exits 0.
- [ ] [AI] Explicitly verify both primary projects:
      `npx nx run-many -t typecheck lint test:quick spec-coverage -p ose-web rhino-cli` — all green.
- [ ] [AI] Verify the production build: `npx nx build ose-web` — exits 0.
- [ ] [AI] Verify the rhino-cli cargo suite: `cargo test --manifest-path apps/rhino-cli/Cargo.toml`
      — all tests pass.
- [ ] [AI] Fix ALL failures — including preexisting issues not caused by these changes; re-run
      until zero failures.

### Manual UI Verification (Playwright MCP)

- [ ] [AI] Start dev server: `npx nx dev ose-web` (port 3100).
- [ ] [AI] Navigate to the landing page via `browser_navigate` to `http://localhost:3100/`; inspect
      via `browser_snapshot` — acceptance: hero/landing renders.
- [ ] [AI] Navigate to `http://localhost:3100/updates`; inspect via `browser_snapshot`
      — acceptance: updates list renders.
- [ ] [AI] Navigate to `http://localhost:3100/about`; inspect via `browser_snapshot`
      — acceptance: about page renders.
- [ ] [AI] Check `browser_console_messages` on each of the three pages
      — acceptance: zero console errors.
- [ ] [AI] Take screenshots via `browser_take_screenshot` for the three pages and record results in
      this checklist.

### Commit Guidelines

- [ ] [AI] Commit changes thematically using Conventional Commits; split by concern, for example:
  - `refactor(rhino-cli): drop ose-platform from DDD allowlist`
  - `chore(ose-web): remove rhino-cli ddd validators from test:quick`
  - `chore(specs): delete ose-platform DDD registry`
  - `refactor(ose-web): delete empty domain layer stubs`
  - `docs(ose-web): rewrite README as hexagonal feature modules`
- [ ] [AI] Keep any preexisting fixes in their own separate commits.
- [ ] [AI] Do NOT bundle unrelated changes into a single commit.

### Post-Push CI Verification

- [ ] [AI] Push changes to `main` (Trunk Based Development; no PR — none requested).
- [ ] [AI] Monitor ALL GitHub Actions workflows triggered by the push (poll every 3 minutes; do not
      use `gh run watch`): `gh run list --branch main --limit 5` then
      `gh run view <id> --json status,conclusion`.
- [ ] [AI] Verify ALL CI checks pass — no exceptions.
- [ ] [AI] If any CI check fails, investigate root cause, fix, and push a follow-up commit; repeat
      until ALL GitHub Actions pass.
- [ ] [AI] Do NOT proceed to archival until CI is fully green.

### Phase 6 Gate

> All checks below must pass before archival.

- [ ] [AI] `npx nx run-many -t typecheck lint test:quick spec-coverage -p ose-web rhino-cli` — all green.
- [ ] [AI] `npx nx build ose-web` — exits 0.
- [ ] [AI] Playwright MCP smoke of `/`, `/updates`, `/about` — zero console errors.
- [ ] [AI] All pushed commits' GitHub Actions workflows are green.

> **Pause Safety**: all work is committed, pushed, and CI-green. The repo is in a fully coherent,
> shippable state. Safe to stop. To resume: proceed to Plan Archival.

---

## Plan Archival

- [ ] [AI] Verify ALL delivery checklist items are ticked.
- [ ] [AI] Verify ALL quality gates pass (local + CI).
- [ ] [AI] Verify ALL manual assertions pass (Playwright MCP smoke).
- [ ] [AI] Rename and move using today's completion date:
      `git mv plans/in-progress/ose-web-remove-ddd plans/done/YYYY-MM-DD__ose-web-remove-ddd`
      (substitute today's date).
- [ ] [AI] Update `plans/in-progress/README.md` — remove the `ose-web-remove-ddd` entry.
- [ ] [AI] Update `plans/done/README.md` — add the entry with the completion date and a one-line summary.
- [ ] [AI] Update any other READMEs that reference this plan (e.g. `plans/README.md`).
- [ ] [AI] Commit the archival: `chore(plans): move ose-web-remove-ddd to done`.
