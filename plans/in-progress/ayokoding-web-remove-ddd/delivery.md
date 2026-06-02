# Delivery Checklist — ayokoding-web Remove DDD

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.
>
> **Phase Gate** — every phase ends with a `### Phase N Gate` (must-pass verification) plus a
> `> **Pause Safety**:` note (the safe-to-stop state and the single command to resume). A phase
> is not complete until its gate is green; do not start phase N+1 while any gate check fails.
>
> **Fix-all-issues**: Fix ALL failures found during quality gates, not just those caused by your
> changes. This follows the root-cause-orientation principle — proactively fix preexisting errors
> encountered during work. Commit preexisting fixes separately with appropriate Conventional
> Commit messages.

## Worktree

Worktree path: `worktrees/ayokoding-web-remove-ddd/`

Provision before execution (run from repo root):

```bash
claude --worktree ayokoding-web-remove-ddd
```

Then initialize the toolchain **in the root worktree** (not the new worktree):

```bash
npm install && npm run doctor -- --fix
```

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md),
[Worktree Toolchain Initialization](../../../repo-governance/development/workflow/worktree-setup.md),
and [Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

---

## Phase 0: Environment Setup and Baseline

> _Executor: repo-setup-manager_

- [ ] [AI] Provision the worktree from repo root: `claude --worktree ayokoding-web-remove-ddd`
      — acceptance: `worktrees/ayokoding-web-remove-ddd/` exists and is a valid git worktree
      (`git worktree list` shows it).
- [ ] [AI] Install dependencies in the root worktree: `npm install`
      — acceptance: exits 0, `node_modules/` synchronized.
- [ ] [AI] Converge the toolchain in the root worktree: `npm run doctor -- --fix`
      — acceptance: exits 0 with no unresolved drift (Rust/cargo, Node, Nx all present).
- [ ] [AI] Establish the ayokoding-web + rhino-cli baseline:
      `npx nx run-many -t typecheck lint test:quick spec-coverage -p ayokoding-web rhino-cli`
      — acceptance: baseline pass/fail recorded; document every preexisting failure verbatim.
- [ ] [AI] Resolve all preexisting failures before proceeding (root-cause-orientation)
      — acceptance: no preexisting failures remain unresolved; if any are fixed, commit them
      separately (`fix(<scope>): ...`).
- [ ] [AI] Confirm the current ground truth still matches `tech-docs.md`:
      `grep -n "ddd bc ayokoding\|ddd ul ayokoding" apps/ayokoding-web/project.json` and
      `grep -n "ayokoding" apps/rhino-cli/src/internal/allowlist.rs`
      — acceptance: the two `ddd` commands and the three `ayokoding` lines (doc, slice, assertion)
      are present; if not, STOP and reconcile the plan before editing.

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [ ] [AI] `npm install` exited 0 and `npm run doctor -- --fix` reports no unresolved drift.
- [ ] [AI] `npx nx run-many -t typecheck lint test:quick spec-coverage -p ayokoding-web rhino-cli`
      baseline recorded and every preexisting failure resolved (zero unresolved).

> **Pause Safety**: only the local toolchain was verified and the baseline recorded — no plan work
> exists yet. Safe to stop indefinitely. To resume: re-run
> `npx nx run-many -t typecheck lint test:quick spec-coverage -p ayokoding-web rhino-cli` and
> confirm it is still clean.

---

## Phase 1: Remove ayokoding from the rhino-cli DDD allowlist

> Natural RED→GREEN: adjust the `membership` test to its post-removal shape first (it fails
> against the current five-entry slice), then remove the slice entry to make it pass.
>
> _Suggested executor: `swe-rust-dev`_

- [ ] [AI] **RED** — Edit the `membership` test in
      `apps/rhino-cli/src/internal/allowlist.rs`: read the current `assert_eq!(v.len(), N);`
      literal, decrement `N` by exactly one, and delete the line
      `assert!(v.contains(&"ayokoding"));`. Do NOT yet touch the `apps_with_ddd()` slice.
      — acceptance: `cargo test --manifest-path apps/rhino-cli/Cargo.toml --lib internal::allowlist`
      FAILS on `membership` (asserted length now mismatches the still-five-entry slice). This
      proves the test guards the change.
- [ ] [AI] **GREEN** — In the same file, remove `"ayokoding",` from the `apps_with_ddd()` slice
      and delete the matching `//!   - ayokoding:    bounded-contexts.yaml + feature files present`
      doc line in the module `//!` block.
      — acceptance: `cargo test --manifest-path apps/rhino-cli/Cargo.toml --lib internal::allowlist`
      PASSES; `grep -n "ayokoding" apps/rhino-cli/src/internal/allowlist.rs` returns nothing.
- [ ] [AI] **REFACTOR** — Verify the remaining assertions still reference apps that are present
      (`organiclever`, `ose-app`) and the `//!` block reads cleanly. Run
      `cargo fmt --manifest-path apps/rhino-cli/Cargo.toml`.
      — acceptance: `cargo fmt --manifest-path apps/rhino-cli/Cargo.toml -- --check` exits 0; no stray blank lines in the slice.
- [ ] [AI] Rebuild rhino-cli (it is a pre-push dependency for other apps):
      `npx nx run rhino-cli:build` and `npx nx run rhino-cli:test:unit`
      — acceptance: both exit 0.
- [ ] [AI] Commit thematically: `git add apps/rhino-cli/src/internal/allowlist.rs && git commit -m "refactor(rhino-cli): drop ayokoding from DDD allowlist"`
      — acceptance: commit created; message follows Conventional Commits.

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [ ] [AI] `cargo test --manifest-path apps/rhino-cli/Cargo.toml --lib` — exits 0.
- [ ] [AI] `npx nx run rhino-cli:build` and `npx nx run rhino-cli:lint` — both exit 0.
- [ ] [AI] `grep -n "ayokoding" apps/rhino-cli/src/internal/allowlist.rs` — returns nothing.

> **Pause Safety**: rhino-cli no longer validates ayokoding's DDD registry, but ayokoding-web's
> `test:quick` still invokes `ddd bc/ul ayokoding` (Phase 2). The tree compiles and all
> rhino-cli tests pass — running `ddd bc ayokoding` now would report "not in allowlist" but
> ayokoding-web's pre-push has not yet been updated, so do not push ayokoding-web changes here.
> Safe to stop. To resume: `cargo test --manifest-path apps/rhino-cli/Cargo.toml --lib`.

---

## Phase 2: Remove DDD validation from ayokoding-web test:quick

> _Suggested executor: `swe-typescript-dev`_

- [ ] [AI] **RED (guard)** — Confirm the two DDD commands are still wired:
      `grep -n "ddd bc ayokoding\|ddd ul ayokoding" apps/ayokoding-web/project.json`
      — acceptance: both lines print (current state). This is the pre-edit guard.
- [ ] [AI] **GREEN** — Edit `apps/ayokoding-web/project.json` `test:quick` target:
      (a) remove the two array entries
      `(cd ../../apps/rhino-cli && cargo run --release --quiet -- ddd bc ayokoding)` and
      `(cd ../../apps/rhino-cli && cargo run --release --quiet -- ddd ul ayokoding)` from
      `options.commands`; (b) remove the two `inputs` globs
      `{workspaceRoot}/specs/apps/ayokoding/ddd/bounded-contexts.yaml` and
      `{workspaceRoot}/specs/apps/ayokoding/ddd/ubiquitous-language/**/*.md`. Leave the vitest +
      coverage-82 command, the `ayokoding-cli links check` command, the
      `generate-indexes --validate` command, `parallel: false`, `cwd`, and `dependsOn` intact.
      — acceptance: `grep -n "ddd " apps/ayokoding-web/project.json` returns nothing; the file is
      valid JSON (`node -e "require('./apps/ayokoding-web/project.json')"` exits 0).
- [ ] [AI] **REFACTOR** — Run the full target to confirm the trimmed command array works:
      `npx nx run ayokoding-web:test:quick`
      — acceptance: exits 0 (vitest + coverage-82 + links + index validation all pass).
- [ ] [AI] Commit thematically:
      `git add apps/ayokoding-web/project.json && git commit -m "chore(ayokoding-web): drop DDD validation from test:quick"`
      — acceptance: commit created.

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [ ] [AI] `npx nx run ayokoding-web:test:quick` — exits 0.
- [ ] [AI] `grep -rn "ddd bc\|ddd ul" apps/ayokoding-web/project.json` — returns nothing.

> **Pause Safety**: ayokoding-web's pre-push no longer runs DDD validation and rhino-cli's
> allowlist no longer lists ayokoding — these two are now consistent. The DDD spec subtree still
> exists on disk but nothing references it for ayokoding. Tree compiles, tests pass. Safe to stop.
> To resume: `npx nx run ayokoding-web:test:quick`.

---

## Phase 3: Delete the DDD spec subtree and empty domain layers

> _Suggested executor: `swe-typescript-dev`_

- [ ] [AI] **RED (guard)** — Confirm the targets exist and have no live importers:
      `test -d specs/apps/ayokoding/ddd && echo PRESENT` and
      `grep -rn "contexts/[a-z-]*/domain" apps/ayokoding-web/src --include="*.ts" --include="*.tsx" | grep -v "/domain/index.ts"`
      — acceptance: first prints `PRESENT`; second returns nothing (zero importers — safe to delete).
- [ ] [AI] **GREEN (spec subtree)** — Delete the DDD spec subtree:
      `git rm -r specs/apps/ayokoding/ddd`
      — acceptance: `test -d specs/apps/ayokoding/ddd` is false; `git status` shows 10 deletions.
- [ ] [AI] **GREEN (domain layers)** — Delete the six empty domain folders:
      `git rm -r apps/ayokoding-web/src/contexts/app-shell/domain apps/ayokoding-web/src/contexts/content/domain apps/ayokoding-web/src/contexts/health/domain apps/ayokoding-web/src/contexts/i18n/domain apps/ayokoding-web/src/contexts/navigation/domain apps/ayokoding-web/src/contexts/search/domain`
      — acceptance: `ls apps/ayokoding-web/src/contexts/*/domain 2>/dev/null` returns nothing.
- [ ] [AI] **REFACTOR (no dangling references)** — Confirm no code/config/doc outside `plans/`
      references the deleted paths:
      `grep -rn "specs/apps/ayokoding/ddd" . --include="*.ts" --include="*.tsx" --include="*.json" --include="*.md" --include="*.rs" | grep -v "plans/"`
      — acceptance: returns nothing. If any non-`plans/` match remains, fix it now (it is in scope
      as a dangling reference).
- [ ] [AI] Typecheck ayokoding-web to confirm the domain deletions broke nothing:
      `npx nx run ayokoding-web:typecheck`
      — acceptance: exits 0.
- [ ] [AI] Stage and commit the spec subtree deletion:
      `git add specs/apps/ayokoding/ddd && git commit -m "chore(specs): remove ayokoding DDD bounded-context registry"`
      — acceptance: `git log --oneline -1` shows this commit message; `git status` shows no staged spec deletions.
- [ ] [AI] Stage and commit the domain-layer deletion:
      `git add apps/ayokoding-web/src/contexts && git commit -m "refactor(ayokoding-web): drop empty domain layers"`
      — acceptance: `git log --oneline -1` shows this commit message; `git status` is clean.

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [ ] [AI] `npx nx run ayokoding-web:typecheck` — exits 0.
- [ ] [AI] `grep -rn "specs/apps/ayokoding/ddd" . --include="*.ts" --include="*.tsx" --include="*.json" --include="*.md" --include="*.rs" | grep -v "plans/"` — returns nothing.
- [ ] [AI] `ls apps/ayokoding-web/src/contexts/*/domain 2>/dev/null` — returns nothing.

> **Pause Safety**: all DDD artifacts except the README prose are gone; the app typechecks and
> the spec tree is consistent. The README still describes the old BC structure (Phase 4). Tree
> compiles. Safe to stop. To resume: `npx nx run ayokoding-web:typecheck`.

---

## Phase 4: Rewrite the README architecture sections

> _Suggested executor: `readme-maker`_

- [ ] [AI] **RED (guard)** — Confirm the DDD/BC language is present:
      `grep -n "bounded context\|DDD registry\|specs/apps/ayokoding/ddd\|ddd bc\|ddd ul" apps/ayokoding-web/README.md`
      — acceptance: matches print (current state).
- [ ] [AI] **GREEN** — Edit `apps/ayokoding-web/README.md`:
      (a) Rename/rewrite the `## Source Layout (BC-organized)` section to describe `src/contexts/`
      as **hexagonal feature modules** per
      [`hexagonal-architecture-web.md`](../../../repo-governance/development/pattern/hexagonal-architecture-web.md),
      with the three layers that actually exist (`application/`, `infrastructure/`,
      `presentation/`) — `domain/` is no longer present in this app. Listing the six feature
      modules in a table is fine, but link the governance doc as the structural authority
      (Dynamic Collection References convention). (b) Remove the paragraph asserting the DDD
      registry is the source of truth and that `rhino-cli ddd bc/ul` enforce it on `test:quick`.
      (c) In `## Related`, change "C4 + DDD + Gherkin specifications" to "C4 + Gherkin
      specifications". Do not introduce any vendor-specific (Claude Code / OpenCode) instructions.
      — acceptance: README reads coherently; describes 3 layers; links the governance doc.
- [ ] [AI] **REFACTOR** — Run markdown quality:
      `npm run lint:md:fix && npm run format:md`
      — acceptance: both exit 0; README passes markdownlint.
- [ ] [AI] Verify no DDD/BC language or ddd path remains:
      `grep -in "bounded context\|DDD registry\|specs/apps/ayokoding/ddd\|ddd bc\|ddd ul" apps/ayokoding-web/README.md`
      — acceptance: returns nothing.
- [ ] [AI] Commit thematically:
      `git add apps/ayokoding-web/README.md && git commit -m "docs(ayokoding-web): describe hexagonal feature modules, drop DDD language"`
      — acceptance: commit created.

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [ ] [AI] `grep -in "bounded context\|DDD registry\|specs/apps/ayokoding/ddd\|ddd bc\|ddd ul" apps/ayokoding-web/README.md` — returns nothing.
- [ ] [AI] `npm run lint:md` — passes for `apps/ayokoding-web/README.md`.

> **Pause Safety**: all five change groups are now applied and the tree is fully consistent —
> docs, tooling, specs, and source all agree. Safe to stop. To resume: proceed to Phase 5
> quality gates.

---

## Phase 5: Quality Gates, Manual Verification, and Push

### Local Quality Gates (Before Push)

- [ ] [AI] Run affected typecheck: `npx nx affected -t typecheck`
- [ ] [AI] Run affected linting: `npx nx affected -t lint`
- [ ] [AI] Run affected quick tests: `npx nx affected -t test:quick`
- [ ] [AI] Run affected spec coverage: `npx nx affected -t spec-coverage`
- [ ] [AI] Run rhino-cli cargo tests: `cargo test --manifest-path apps/rhino-cli/Cargo.toml --lib`
- [ ] [AI] Build the app: `npx nx build ayokoding-web` — exits 0.
- [ ] [AI] Fix ALL failures found — including preexisting issues not caused by these changes;
      commit preexisting fixes separately.
- [ ] [AI] Re-run any failing checks to confirm resolution — acceptance: zero failures remain.

### Manual UI Verification (Playwright MCP)

> ayokoding-web is a web UI; manual behavioral assertion is required.

- [ ] [AI] Start dev server: `npx nx dev ayokoding-web` (serves on port 3101).
- [ ] [AI] Navigate to the home page via `browser_navigate` (`http://localhost:3101/en`).
- [ ] [AI] Inspect DOM via `browser_snapshot` — acceptance: home page renders its expected
      content (header, content listing).
- [ ] [AI] Navigate to one content page via `browser_navigate` (any `/en/<section>/...` route
      present in `content/`).
- [ ] [AI] Inspect DOM via `browser_snapshot` — acceptance: the content page renders its
      markdown body.
- [ ] [AI] Check `browser_console_messages` on both pages — acceptance: **zero** console errors.
- [ ] [AI] Take `browser_take_screenshot` of both pages for the record.
- [ ] [AI] Document the verification result inline in this checklist (pages visited, console
      clean yes/no).

### Commit Guidelines

- [ ] [AI] Confirm commits are thematic and split by domain (rhino-cli / project.json / specs /
      domain folders / README), each Conventional-Commits formatted. Acceptance: `git log --oneline`
      shows distinct, well-scoped commits with no bundled unrelated changes.

### Post-Push CI Verification

- [ ] [AI] Push to `main`: `git push origin main`.
- [ ] [AI] Monitor ALL GitHub Actions workflows triggered by the push (poll every ~3 minutes;
      one `gh run view --json status,conclusion` per wakeup; do NOT use `gh run watch`).
- [ ] [AI] Verify ALL CI checks pass — no exceptions.
- [ ] [AI] If any CI check fails, investigate root cause, fix, and push a follow-up commit;
      repeat until CI is fully green.
- [ ] [AI] Do NOT proceed to archival until CI is fully green.

### Phase 5 Gate

> All checks below must pass before archival.

- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — all green for
      ayokoding-web and rhino-cli.
- [ ] [AI] `npx nx build ayokoding-web` — exits 0.
- [ ] [AI] Manual Playwright smoke: home + one content page render with zero console errors.
- [ ] [AI] `git push origin main` completed and all GitHub Actions workflows are green.

> **Pause Safety**: all changes are committed, pushed, and CI-green. The repo is in its final
> intended state. Safe to stop. To resume (for archival only): `git log --oneline` to confirm
> the pushed commits, then proceed to archival.

---

## Plan Archival

- [ ] [AI] Verify ALL delivery checklist items above are ticked.
- [ ] [AI] Verify ALL quality gates pass (local + CI) and the manual Playwright smoke passed.
- [ ] [AI] Rename and move the plan to done with today's completion date:
      `git mv plans/in-progress/ayokoding-web-remove-ddd plans/done/$(date +%Y-%m-%d)__ayokoding-web-remove-ddd`
      — acceptance: folder now under `plans/done/YYYY-MM-DD__ayokoding-web-remove-ddd/`.
- [ ] [AI] Update `plans/in-progress/README.md` — remove this plan's entry (if present).
- [ ] [AI] Update `plans/done/README.md` — add this plan with its completion date.
- [ ] [AI] Update any other READMEs that reference this plan (e.g. `plans/README.md`).
- [ ] [AI] Commit the archival: `git commit -m "chore(plans): move ayokoding-web-remove-ddd to done"`.
- [ ] [AI] Push the archival commit to `main` and confirm CI green.
