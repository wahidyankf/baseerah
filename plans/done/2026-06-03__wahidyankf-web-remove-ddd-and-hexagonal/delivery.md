# Delivery Checklist — Remove DDD and Hexagonal from wahidyankf-web

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.
>
> **Phase Gate** — every phase ends with a `### Phase N Gate` (must-pass verification) plus a
> `> **Pause Safety**:` note (the safe-to-stop state and the single command to resume). A phase
> is not complete until its gate is green; do not start phase N+1 while any gate check fails.

## Worktree

Worktree path: `worktrees/wahidyankf-web-remove-ddd-and-hexagonal/`

Provision before execution (run from repo root):

```bash
claude --worktree wahidyankf-web-remove-ddd-and-hexagonal
```

Then initialize the toolchain in the **root** worktree (not the new worktree):

```bash
npm install && npm run doctor -- --fix
```

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md)
and [Worktree Toolchain Initialization](../../../repo-governance/development/workflow/worktree-setup.md).

## Git Workflow

Trunk Based Development — work directly on `main`, commit and push to `main`, no
PR. (No PR was requested and no PR step appears below.) Commit thematically with
Conventional Commits.

---

## Phase 0: Environment Setup and Baseline

> _Executor: repo-setup-manager_

- [x] [AI] Provision the worktree from repo root:
    `claude --worktree wahidyankf-web-remove-ddd-and-hexagonal`
    — acceptance: directory `worktrees/wahidyankf-web-remove-ddd-and-hexagonal/` exists.
<!-- Date: 2026-06-03 | Status: done | Notes: git worktree add at HEAD e3c045a89 -->
- [x] [AI] Install dependencies: npm install exits 0.
<!-- Date: 2026-06-03 | Status: done -->
- [x] [AI] Converge toolchain: npm run doctor -- --fix exits 0, 20/20 tools OK.
<!-- Date: 2026-06-03 | Status: done -->
- [x] [AI] Baseline: all targets PASS — wahidyankf-web (66 tests, 80.54% coverage) + rhino-cli (777 tests).
<!-- Date: 2026-06-03 | Status: done | Notes: N=3 in allowlist ["organiclever","wahidyankf","ose-app"] -->
- [x] [AI] cargo test baseline: 782 passed.
<!-- Date: 2026-06-03 | Status: done -->
- [x] [AI] No preexisting failures to resolve.
<!-- Date: 2026-06-03 | Status: done -->

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [x] [AI] npm install + doctor clean.
<!-- Date: 2026-06-03 | Status: done -->
- [x] [AI] baseline recorded, zero unresolved failures.
<!-- Date: 2026-06-03 | Status: done -->
- [x] [AI] cargo test baseline green (782 passed).
<!-- Date: 2026-06-03 | Status: done -->

> **Pause Safety**: only the local toolchain was verified and the baseline recorded — no plan
> work exists yet. Safe to stop indefinitely. To resume: re-run the baseline command and confirm
> it is still clean.

---

## Phase 1: Remove DDD Accretion

> Removes the DDD spec tree, the two `ddd` pre-push gates in `project.json`, and
> the `wahidyankf` entry from the rhino-cli allowlist. Behavior-preserving for
> the app's runtime; only build-time gates change.

- [x] [AI] Delete the DDD spec tree: 9 files removed.
<!-- Date: 2026-06-03 | Status: done | Commit: 8adabee3a -->
- [x] [AI] Edit `apps/wahidyankf-web/project.json` `test:quick` target: remove the two `inputs`
      glob lines referencing `specs/apps/wahidyankf/ddd/bounded-contexts.yaml` and
      `specs/apps/wahidyankf/ddd/ubiquitous-language/**/*.md` (currently lines ~60–61); remove the
      two `commands[]` entries running `ddd bc wahidyankf` and `ddd ul wahidyankf` (currently lines
      ~66–67), leaving the `npx vitest run --project unit-fe --coverage && (... test-coverage
validate ... 80)` command as the sole `commands[]` entry. Keep `dependsOn: ["rhino-cli:build"]`,
      the `behavior/web/gherkin/**/*.feature` input, `outputs`, and `parallel: false`.
      — acceptance: `grep -n "ddd bc wahidyankf\|ddd ul wahidyankf\|ddd/bounded-contexts\|ddd/ubiquitous-language" apps/wahidyankf-web/project.json`
      returns no matches; `dependsOn` still lists `rhino-cli:build`.
  - _Suggested executor: `swe-typescript-dev`_
- [x] [AI] **RED** — Edit `apps/rhino-cli/src/internal/allowlist.rs` `mod tests::membership`:
      decrement the `assert_eq!(v.len(), N)` count by one (at authoring time `5` → `4`; use the
      value present at execution time minus one). Run
      `cargo test --manifest-path apps/rhino-cli/Cargo.toml internal::allowlist`
      — acceptance: the `membership` test now FAILS because `apps_with_ddd()` still returns the old
      count (length mismatch).
  - _Suggested executor: `swe-rust-dev`_
- [x] [AI] **GREEN** — In the same file, remove the `"wahidyankf",` entry from `apps_with_ddd()`,
      remove the `//!   - wahidyankf: ...` rustdoc line from the top doc-comment, and remove any
      `assert!(v.contains(&"wahidyankf"))` assertion if one is present (none at authoring time). Run
      `cargo test --manifest-path apps/rhino-cli/Cargo.toml internal::allowlist`
      — acceptance: the `membership` test passes; `grep -n "wahidyankf" apps/rhino-cli/src/internal/allowlist.rs`
      returns no matches.
  - _Suggested executor: `swe-rust-dev`_
- [x] [AI] **REFACTOR** — Rebuild rhino-cli and re-run its full test suite:
      `cargo build --release --manifest-path apps/rhino-cli/Cargo.toml && cargo test --manifest-path apps/rhino-cli/Cargo.toml`
      — acceptance: build exits 0; all cargo tests pass.
  - _Suggested executor: `swe-rust-dev`_
- [x] [AI] Verify `test:quick` still runs for wahidyankf-web without the DDD gates:
      `npx nx run wahidyankf-web:test:quick --skip-nx-cache`
      — acceptance: exits 0; unit tests pass; coverage ≥ 80%; no `ddd bc`/`ddd ul` invocation in output.

### Local Quality Gates (Before Push) — Phase 1

- [x] [AI] `npx nx affected -t typecheck` — exits 0.
- [x] [AI] `npx nx affected -t lint` — exits 0.
- [x] [AI] `npx nx affected -t test:quick` — exits 0.
- [x] [AI] `npx nx affected -t spec-coverage` — exits 0.
- [x] [AI] Fix ALL failures found — including preexisting issues not caused by these changes.

> **Important**: Fix ALL failures found during quality gates, not just those caused by your
> changes. This follows the root cause orientation principle — proactively fix preexisting errors
> encountered during work. Commit preexisting fixes separately with appropriate conventional
> commit messages.

### Commit Guidelines — Phase 1

- [x] [AI] Commit thematically, e.g.
      `chore(wahidyankf-web): remove DDD bounded-context registry and pre-push gates` (spec tree +
      project.json) and `refactor(rhino-cli): drop wahidyankf from DDD allowlist` (allowlist.rs),
      as separate commits by domain.

### Post-Push CI Verification — Phase 1

- [x] [AI] Push to `main`.
<!-- Date: 2026-06-03 | Status: done | Notes: pushed commits 8adabee3a + 003f34d2d -->
- [x] [AI] Monitor CI — triggered correct run 26858329603 (wahidyankf-web workflow 262956551).
<!-- Date: 2026-06-03 | Status: done -->
- [x] [AI] ALL CI checks pass — run 26858329603 completed/success (all 7 jobs).
<!-- Date: 2026-06-03 | Status: done -->
- [x] [AI] CI green — proceeding to Phase 2.
<!-- Date: 2026-06-03 | Status: done -->

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [x] [AI] `test ! -d specs/apps/wahidyankf/ddd` is true.
<!-- Date: 2026-06-03 | Status: done -->
- [x] [AI] grep ddd bc/ul project.json = nothing.
<!-- Date: 2026-06-03 | Status: done -->
- [x] [AI] grep wahidyankf allowlist.rs = nothing.
<!-- Date: 2026-06-03 | Status: done -->
- [x] [AI] cargo test green (782 passed).
<!-- Date: 2026-06-03 | Status: done -->
- [x] [AI] test:quick green + CI 26858329603 green on main.
<!-- Date: 2026-06-03 | Status: done -->

> **Pause Safety**: DDD accretion fully removed; the app still builds and tests pass; the
> `contexts/` layout is untouched and coherent. Safe to stop. To resume:
> `npx nx run-many -t typecheck lint test:quick -p wahidyankf-web rhino-cli`.

---

## Phase 2: Flatten `app-shell` context

> Move app-shell first because every other context imports `Navigation` from it.
> Moving it first lets later phases update fewer cross-references per step.

- [x] [AI] **RED/baseline** — Confirm the suite is green before the move:
      `npx nx run wahidyankf-web:test:unit` — acceptance: exits 0 (this is the behavior guard the
      refactor must keep green).
  - _Suggested executor: `swe-typescript-dev`_
- [x] [AI] **GREEN (move + rewrite)** — Create `apps/wahidyankf-web/src/features/app-shell/` and
      `git mv` the four files (collapsing the `presentation/` layer):
      `Navigation.tsx`, `Navigation.unit.test.tsx`, `style.ts`, `style.unit.test.ts` from
      `src/contexts/app-shell/presentation/` to `src/features/app-shell/`. Delete the empty stubs
      `src/contexts/app-shell/{domain,application,infrastructure}/index.ts` via `git rm`. Then
      rewrite every importer of app-shell to `@/features/app-shell/Navigation`:
      `src/contexts/home/presentation/HomeContent.tsx`,
      `src/contexts/personal-projects/presentation/PersonalProjectsContent.tsx`,
      `src/contexts/cv/presentation/CvContent.tsx`, `src/app/page.unit.test.tsx`,
      `src/app/cv/page.unit.test.tsx`, `src/app/personal-projects/page.unit.test.tsx`. Run
      `npx nx run wahidyankf-web:typecheck && npx nx run wahidyankf-web:test:unit`
      — acceptance: both exit 0; no `@/contexts/app-shell` import remains
      (`grep -rn "@/contexts/app-shell" apps/wahidyankf-web` returns nothing).
  - _Suggested executor: `swe-typescript-dev`_
- [x] [AI] **REFACTOR** — Run `npx nx run wahidyankf-web:lint` and fix any oxlint findings
      introduced by the move — acceptance: lint exits 0.
  - _Suggested executor: `swe-typescript-dev`_

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [x] [AI] `test -d apps/wahidyankf-web/src/features/app-shell` and
      `test ! -d apps/wahidyankf-web/src/contexts/app-shell` both true.
- [x] [AI] `grep -rn "@/contexts/app-shell" apps/wahidyankf-web` returns no matches.
- [x] [AI] `npx nx run wahidyankf-web:typecheck` and `:test:unit` and `:lint` all exit 0.

> **Pause Safety**: `app-shell` lives at `features/app-shell/`; the remaining four contexts still
> live under `contexts/` and import from `features/app-shell` — the tree compiles and tests pass.
> Safe to stop. To resume: `npx nx run wahidyankf-web:typecheck`.

---

## Phase 3: Flatten `search` context

> `search` is imported by home, cv, and personal-projects; flatten it next so
> later phases reference its final path.

- [x] [AI] **RED/baseline** — Confirm the suite is green before the move:
      `npx nx run wahidyankf-web:test:unit` — acceptance: exits 0 (this is the behavior guard the
      refactor must keep green).
  - _Suggested executor: `swe-typescript-dev`_
- [x] [AI] **GREEN (move + rewrite)** — Create `src/features/search/` and `git mv`
      `src/contexts/search/application/search.ts`, `search.unit.test.ts`, and
      `src/contexts/search/presentation/SearchSection.tsx` into it. `git rm` the empty
      `src/contexts/search/{domain,infrastructure}/index.ts` stubs. Rewrite importers of
      `@/contexts/search/application/search` → `@/features/search/search` in:
      `src/contexts/home/presentation/HomeContent.tsx`,
      `src/contexts/personal-projects/application/projects.ts`,
      `src/contexts/cv/presentation/CvContent.tsx`, `src/app/page.unit.test.tsx`,
      `src/app/cv/page.unit.test.tsx`, `src/app/personal-projects/page.unit.test.tsx`. Run
      `npx nx run wahidyankf-web:typecheck && npx nx run wahidyankf-web:test:unit`
      — acceptance: both exit 0; `grep -rn "@/contexts/search" apps/wahidyankf-web` returns nothing.
  - _Suggested executor: `swe-typescript-dev`_
- [x] [AI] **REFACTOR** — `npx nx run wahidyankf-web:lint` — acceptance: exits 0.
  - _Suggested executor: `swe-typescript-dev`_

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [x] [AI] `test -d apps/wahidyankf-web/src/features/search` and
      `test ! -d apps/wahidyankf-web/src/contexts/search` both true.
- [x] [AI] `grep -rn "@/contexts/search" apps/wahidyankf-web` returns no matches.
- [x] [AI] `:typecheck`, `:test:unit`, `:lint` all exit 0.

> **Pause Safety**: `app-shell` and `search` live under `features/`; `cv`, `home`,
> `personal-projects` still under `contexts/`. Tree compiles, tests green. Safe to stop. To resume:
> `npx nx run wahidyankf-web:typecheck`.

---

## Phase 4: Flatten `cv` context

- [x] [AI] **RED/baseline** — Confirm the suite is green before the move:
      `npx nx run wahidyankf-web:test:unit` — acceptance: exits 0 (this is the behavior guard the
      refactor must keep green).
  - _Suggested executor: `swe-typescript-dev`_
- [x] [AI] **GREEN (move + rewrite)** — Create `src/features/cv/` and `git mv` the cv files
      (collapsing `application/` and `presentation/`): `data.ts`, `data.unit.test.ts`,
      `markdown.tsx`, `markdown.unit.test.tsx` from `src/contexts/cv/application/`, and
      `CvContent.tsx` from `src/contexts/cv/presentation/`, into `src/features/cv/`. `git rm` the
      empty `src/contexts/cv/{domain,infrastructure}/index.ts` stubs. Inside the moved
      `CvContent.tsx`, rewrite its imports: `@/contexts/cv/application/data` → `@/features/cv/data`
      and `@/contexts/cv/application/markdown` → `@/features/cv/markdown`. Rewrite external
      importers of cv: `src/contexts/home/presentation/HomeContent.tsx` (`@/contexts/cv/application/data`
      → `@/features/cv/data`, `@/contexts/cv/application/markdown` → `@/features/cv/markdown`),
      `src/app/cv/page.tsx` (`@/contexts/cv/presentation/CvContent` → `@/features/cv/CvContent`),
      `src/app/page.unit.test.tsx` and `src/app/cv/page.unit.test.tsx`
      (`@/contexts/cv/application/data` → `@/features/cv/data`). Run
      `npx nx run wahidyankf-web:typecheck && npx nx run wahidyankf-web:test:unit`
      — acceptance: both exit 0; `grep -rn "@/contexts/cv" apps/wahidyankf-web` returns nothing.
  - _Suggested executor: `swe-typescript-dev`_
- [x] [AI] **REFACTOR** — `npx nx run wahidyankf-web:lint` — acceptance: exits 0.
  - _Suggested executor: `swe-typescript-dev`_

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [x] [AI] `test -d apps/wahidyankf-web/src/features/cv` and
      `test ! -d apps/wahidyankf-web/src/contexts/cv` both true.
- [x] [AI] `grep -rn "@/contexts/cv" apps/wahidyankf-web` returns no matches.
- [x] [AI] `:typecheck`, `:test:unit`, `:lint` all exit 0.

> **Pause Safety**: `app-shell`, `search`, `cv` under `features/`; `home`, `personal-projects`
> still under `contexts/`. Tree compiles, tests green. Safe to stop. To resume:
> `npx nx run wahidyankf-web:typecheck`.

---

## Phase 5: Flatten `home` context

- [x] [AI] **RED/baseline** — Confirm the suite is green before the move:
      `npx nx run wahidyankf-web:test:unit` — acceptance: exits 0 (this is the behavior guard the
      refactor must keep green).
  - _Suggested executor: `swe-typescript-dev`_
- [x] [AI] **GREEN (move + rewrite)** — Create `src/features/home/` and `git mv`
      `src/contexts/home/presentation/HomeContent.tsx` → `src/features/home/HomeContent.tsx`.
      `git rm` the empty `src/contexts/home/{domain,application,infrastructure}/index.ts` stubs.
      (HomeContent's own imports of app-shell/search/cv were already rewritten to `@/features/...`
      in Phases 2–4.) Rewrite the page importer `src/app/page.tsx`
      (`@/contexts/home/presentation/HomeContent` → `@/features/home/HomeContent`). Run
      `npx nx run wahidyankf-web:typecheck && npx nx run wahidyankf-web:test:unit`
      — acceptance: both exit 0; `grep -rn "@/contexts/home" apps/wahidyankf-web` returns nothing.
  - _Suggested executor: `swe-typescript-dev`_
- [x] [AI] **REFACTOR** — `npx nx run wahidyankf-web:lint` — acceptance: exits 0.
  - _Suggested executor: `swe-typescript-dev`_

### Phase 5 Gate

> All checks below must pass before starting Phase 6.

- [x] [AI] `test -d apps/wahidyankf-web/src/features/home` and
      `test ! -d apps/wahidyankf-web/src/contexts/home` both true.
- [x] [AI] `grep -rn "@/contexts/home" apps/wahidyankf-web` returns no matches.
- [x] [AI] `:typecheck`, `:test:unit`, `:lint` all exit 0.

> **Pause Safety**: only `personal-projects` remains under `contexts/`. Tree compiles, tests green.
> Safe to stop. To resume: `npx nx run wahidyankf-web:typecheck`.

---

## Phase 6: Flatten `personal-projects` context

- [x] [AI] **RED/baseline** — Confirm the suite is green before the move:
      `npx nx run wahidyankf-web:test:unit` — acceptance: exits 0 (this is the behavior guard the
      refactor must keep green).
- [x] [AI] **GREEN (move + rewrite)** — Create `src/features/personal-projects/` and `git mv`
      `src/contexts/personal-projects/application/projects.ts` and
      `src/contexts/personal-projects/presentation/PersonalProjectsContent.tsx` into it. `git rm`
      the empty `src/contexts/personal-projects/{domain,infrastructure}/index.ts` stubs. Inside the
      moved `PersonalProjectsContent.tsx`, rewrite
      `@/contexts/personal-projects/application/projects` → `@/features/personal-projects/projects`.
      (Its app-shell/search imports were already rewritten in earlier phases.) Rewrite the page
      importer `src/app/personal-projects/page.tsx`
      (`@/contexts/personal-projects/presentation/PersonalProjectsContent` →
      `@/features/personal-projects/PersonalProjectsContent`). Run
      `npx nx run wahidyankf-web:typecheck && npx nx run wahidyankf-web:test:unit`
      — acceptance: both exit 0; `grep -rn "@/contexts/personal-projects" apps/wahidyankf-web`
      returns nothing.
  - _Suggested executor: `swe-typescript-dev`_
- [x] [AI] **REFACTOR** — Remove the now-empty `src/contexts/` directory tree:
      `git rm -r apps/wahidyankf-web/src/contexts 2>/dev/null; rmdir apps/wahidyankf-web/src/contexts 2>/dev/null || true`
      then `npx nx run wahidyankf-web:lint`
      — acceptance: `test ! -d apps/wahidyankf-web/src/contexts` is true; lint exits 0.
  - _Suggested executor: `swe-typescript-dev`_

### Local Quality Gates (Before Push) — Phases 2–6

- [x] [AI] `npx nx affected -t typecheck` — exits 0.
- [x] [AI] `npx nx affected -t lint` — exits 0.
- [x] [AI] `npx nx affected -t test:quick` — exits 0 (unit tests + coverage ≥ 80%).
- [x] [AI] `npx nx affected -t spec-coverage` — exits 0.
- [x] [AI] Fix ALL failures found — including preexisting issues not caused by these changes.

### Commit Guidelines — Phases 2–6

- [x] [AI] Commit each context flatten as its own thematic commit, e.g.
      `refactor(wahidyankf-web): flatten <ctx> context into src/features`.

### Post-Push CI Verification — Phases 2–6

- [x] [AI] Push to `main`: `git push origin main`.
- [x] [AI] Monitor ALL GitHub Actions workflows (poll every 3 min via
      `gh run view --json status,conclusion`; do not use `gh run watch`).
- [x] [AI] Verify ALL CI checks pass; fix and re-push on any failure.
- [x] [AI] Do NOT proceed to Phase 7 until CI is fully green.

### Phase 6 Gate

> All checks below must pass before starting Phase 7.

- [x] [AI] `test -d apps/wahidyankf-web/src/features/personal-projects` and
      `test ! -d apps/wahidyankf-web/src/contexts` both true.
- [x] [AI] `grep -rn "@/contexts" apps/wahidyankf-web` returns NO matches (all five contexts moved).
- [x] [AI] `npx nx run wahidyankf-web:typecheck`, `:lint`, `:test:quick`, `:spec-coverage` all exit 0.
- [x] [AI] CI green on `main`.

> **Pause Safety**: the entire `contexts/` tree is gone; all five features live under `features/`;
> the app builds, tests pass, coverage holds. Code work is complete and coherent. Safe to stop. To
> resume: `npx nx run-many -t typecheck lint test:quick spec-coverage -p wahidyankf-web`.

---

## Phase 7: Governance Opt-Out, Docs, Verification, Archival

> Documentation + governance + final verification. No app code changes.

- [x] [AI] Add an `## Exemptions` section to
      `repo-governance/development/pattern/hexagonal-architecture-web.md` (insert before the
      existing `## Related` section, ~line 187). State: trivially-small static content sites with
      no IO ports and no business rules MAY use a flat `src/features/<name>/` layout instead of the
      hexagonal `contexts/` layout, documented per-app; cite `wahidyankf-web` as the documented
      example. Keep the wording vendor-neutral (no tool/brand names; not under any "Platform Binding
      Examples" heading — the file has none).
      — acceptance: `grep -n "Exemptions" repo-governance/development/pattern/hexagonal-architecture-web.md`
      matches; the new text mentions `src/features/` and "no IO ports"/"no business rules"; a
      vendor-term scan (`grep -in "claude\|opencode\|amazon q\|cursor" <the new section>`) is clean.
  - _Suggested executor: `repo-rules-maker`_
- [x] [AI] Rewrite the Architecture/Specs/Structure sections of
      `apps/wahidyankf-web/README.md`: replace the "Five-folder C4 + DDD tree" / bounded-context /
      `contexts/` (DDD bounded contexts) descriptions with the flat `src/features/<ctx>/` layout;
      remove the `rhino-cli ddd bc wahidyankf` / `rhino-cli ddd ul wahidyankf` "first pre-push gate"
      lines and the `ddd bc → ddd ul → unit tests` description in the test:quick comment; remove the
      DDD-registry and ubiquitous-language bullets that reference `specs/apps/wahidyankf/ddd`. Keep
      the Gherkin/behavior and C4 references. Update the `## Structure` tree to show `features/`.
      — acceptance: `grep -in "DDD\|bounded context\|hexagonal\|ddd bc\|ddd ul\|specs/apps/wahidyankf/ddd"
apps/wahidyankf-web/README.md` returns no matches; the README's structure tree shows
      `src/features/`.
  - _Suggested executor: `readme-maker`_
- [x] [AI] Repo-wide grep-clean verification:
      `grep -rn "@/contexts" apps/wahidyankf-web` AND
      `grep -rn "specs/apps/wahidyankf/ddd" . --include='*.md' --include='*.json' --include='*.rs' --include='*.ts' --include='*.tsx'`
      — acceptance: both return no matches (excluding this plan folder's own descriptive text).
- [x] [AI] Full quality gate across both affected projects:
      `npx nx run-many -t typecheck lint test:quick spec-coverage -p wahidyankf-web rhino-cli`
      and `npx nx build wahidyankf-web`
      — acceptance: all exit 0.
- [x] [AI] rhino-cli cargo tests: `cargo test --manifest-path apps/rhino-cli/Cargo.toml`
      — acceptance: green.

### Manual UI Verification (Playwright MCP)

- [x] [AI] Start dev server: `npx nx dev wahidyankf-web` (port 3201).
- [x] [AI] `browser_navigate` to `http://localhost:3201/`, `.../cv`, `.../personal-projects`.
- [x] [AI] `browser_snapshot` each page — verify correct rendering (nav, content present).
- [x] [AI] Exercise the search box via `browser_fill_form` / `browser_click` on the relevant page —
      verify filtering behaves as before.
- [x] [AI] `browser_console_messages` on every page — acceptance: zero console errors.
- [x] [AI] `browser_take_screenshot` of each page for the record.
- [x] [AI] Document verification results in this checklist.

### Local Quality Gates (Before Push) — Phase 7

- [x] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` — exits 0.
- [x] [AI] `npm run lint:md:fix` then `npm run lint:md` — markdown clean.
- [x] [AI] Fix ALL failures found — including preexisting issues.

> **Important**: Fix ALL failures found during quality gates, not just those caused by your
> changes (root cause orientation). Commit preexisting fixes separately.

### Commit Guidelines — Phase 7

- [x] [AI] Thematic commits, e.g. `docs(governance): add static-site exemption to hexagonal-web pattern`
      and `docs(wahidyankf-web): describe flat features layout in README` as separate commits.

### Post-Push CI Verification — Phase 7

- [x] [AI] Push to `main`: `git push origin main`.
- [x] [AI] Monitor ALL GitHub Actions workflows (poll every 3 min via
      `gh run view --json status,conclusion`; do not use `gh run watch`).
- [x] [AI] Verify ALL CI checks pass; fix and re-push on any failure until fully green.

### Phase 7 Gate

> All checks below must pass before archival.

- [x] [AI] Governance exemption clause present and vendor-neutral.
- [x] [AI] `grep -in "DDD\|bounded context\|hexagonal\|ddd bc\|ddd ul" apps/wahidyankf-web/README.md`
      returns no matches.
- [x] [AI] `grep -rn "@/contexts" apps/wahidyankf-web` and
      `grep -rn "specs/apps/wahidyankf/ddd"` (code/docs) both clean.
- [x] [AI] Full gate + `nx build wahidyankf-web` + rhino-cli cargo tests green; CI green on `main`.
- [x] [AI] Playwright-MCP smoke recorded with zero console errors.

> **Pause Safety**: all code, governance, and docs changes are complete and verified; CI is green.
> Only plan archival remains. Safe to stop. To resume: proceed to Plan Archival.

---

### Plan Archival

- [x] [AI] Verify ALL delivery checklist items are ticked.
- [x] [AI] Verify ALL quality gates pass (local + CI).
- [x] [AI] Verify ALL manual assertions pass (Playwright MCP).
- [x] [AI] Rename and move:
      `git mv plans/in-progress/wahidyankf-web-remove-ddd-and-hexagonal/ plans/done/YYYY-MM-DD__wahidyankf-web-remove-ddd-and-hexagonal/`
      substituting `YYYY-MM-DD` with the actual completion date at archival time.
- [x] [AI] Update `plans/in-progress/README.md` — remove this plan's entry.
- [x] [AI] Update `plans/done/README.md` — add this plan with completion date.
- [x] [AI] Update any other READMEs that reference this plan (e.g. `plans/README.md`).
- [x] [AI] Commit the archival: `chore(plans): move wahidyankf-web-remove-ddd-and-hexagonal to done`.
