# Delivery — Standardize App Spec Trees

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.

This plan touches only spec files, project configuration, governance docs, and agent definitions
— no production application code. Delivery steps are therefore direct action + acceptance
criterion (no Red→Green→Refactor cycles); the green gates are the existing `spec-coverage`,
`test:quick`, and e2e suites.

## Worktree

Worktree path: `worktrees/standardize-app-spec-trees/`

Optional manual pre-provisioning (run from repo root):

```bash
claude --worktree standardize-app-spec-trees
```

The plan-execution Step 0 gate enters this worktree by default: it auto-provisions from the latest
`origin/main` when missing, syncs with `origin/main` before implementing, and prompts before
deleting the worktree after the plan is archived and pushed.

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

## Phase 0: Environment Setup and Baseline

- [ ] [AI] Provision worktree: `claude --worktree standardize-app-spec-trees` (creates
      `worktrees/standardize-app-spec-trees/`). Acceptance: worktree directory exists.
- [ ] [AI] Initialize toolchain in the root worktree: `npm install && npm run doctor -- --fix`.
      Acceptance: `doctor` reports all required tools present.
- [ ] [AI] Record baseline: `npx nx run-many -t spec-coverage,test:quick --projects=ose-app-be,ose-app-web,ose-web,ose-cli`
      and run affected e2e (`ose-app-be-e2e`, `ose-app-web-e2e`, `ose-web-fe-e2e`,
      `ose-web-be-e2e`). Acceptance: capture the pass/fail state in writing; all targets that
      pass today are recorded as the green baseline.
- [ ] [AI] Confirm the consumer reference inventory in
      [tech-docs.md](./tech-docs.md#consumer-reference-impact) is still accurate:
      `grep -rn "specs/apps/ose-app\|specs/apps/ose-platform" apps/ specs/ repo-governance/ docs/ AGENTS.md .claude/`.
      Acceptance: every hit maps to a row in the impact tables; add any newly found references.

### Phase 0 Gate

> All checks below must pass before starting Phase A.

- [ ] [AI] Baseline recorded and reference inventory reconciled. Acceptance: a written baseline
      note exists and the grep returns no unmapped references.

> **Pause Safety**: No files moved yet; repo is at clean `origin/main`. Safe to stop. To resume:
> re-run the Phase 0 baseline command.

## Phase A: Migrate `ose-app` → `specs/apps/ose/` (app surfaces)

- [ ] [AI] Create target tree and move app-be behavior:
      `git mv specs/apps/ose-app/behavior/be/gherkin specs/apps/ose/behavior/app-be/gherkin`
      (create intermediate dirs as needed). Acceptance: `git status` shows renames, not
      delete+add.
- [ ] [AI] Move app-web behavior:
      `git mv specs/apps/ose-app/behavior/web/gherkin specs/apps/ose/behavior/app-web/gherkin`.
      Acceptance: renames tracked.
- [ ] [AI] Move contracts project: `git mv specs/apps/ose-app/containers/contracts specs/apps/ose/containers/contracts`.
      Acceptance: renames tracked.
- [ ] [AI] Edit `specs/apps/ose/containers/contracts/project.json`: set `"name": "ose-contracts"`,
      `"root": "specs/apps/ose/containers/contracts"`, and rewrite every `specs/apps/ose-app/containers/contracts`
      path in the `lint`/`bundle`/`docs` commands to `specs/apps/ose/containers/contracts`.
      Verify: `npx nx run ose-contracts:lint` — exits 0; `git diff --exit-code` clean on generated
      bundle.
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] Rewrite `apps/ose-app-be/project.json` (contracts input L13; spec-coverage inputs
      L112–114 `be/gherkin`→`app-be/gherkin` and `ddd/...`→`specs/apps/ose/ddd/...`; command L127;
      inputs L130). Verify: `npx nx run ose-app-be:spec-coverage` — exits 0.
- [ ] [AI] Rewrite `apps/ose-app-be-e2e/project.json` (L29, L44), `playwright.config.ts` (L5–6),
      and `Covers:` comments in `steps/bounded-contexts.steps.ts` (L5–8) + `steps/health.steps.ts`
      (L4) to `app-be/gherkin`. Verify: `npx nx run ose-app-be-e2e:test:e2e` — passes (or matches
      recorded baseline if env-gated).
- [ ] [AI] Rewrite `apps/ose-app-web/project.json` (codegen `-i` L10; input L14; spec-coverage cmd
      L108 `web/gherkin`→`app-web/gherkin`; input L111). Verify:
      `npx nx run ose-app-web:codegen` then `npx nx run ose-app-web:spec-coverage` — both exit 0.
  - _Suggested executor: `swe-typescript-dev`_
- [ ] [AI] Rewrite `apps/ose-app-web-e2e/project.json` (L22, L44), `playwright.config.ts` (L5–6),
      `steps/smoke.steps.ts` (L4) to `app-web/gherkin`. Verify:
      `npx nx run ose-app-web-e2e:test:e2e` — passes (or matches baseline).
- [ ] [AI] Rewrite README references: `apps/ose-app-be/README.md` (L70, L75, L76),
      `apps/ose-app-be-e2e/README.md` (L19), `apps/ose-app-web-e2e/README.md` (L20),
      `apps/ose-app-web/README.md` (L38), and `apps/ose-app-web/src/contexts/*/README.md` (4 files)
      to the new `specs/apps/ose/...` paths. Verify:
      `npx nx run rhino-cli:validate:links` — no broken links in touched files.

### Phase A Gate

> All checks below must pass before starting Phase B.

- [ ] [AI] `npx nx run-many -t spec-coverage --projects=ose-app-be,ose-app-web` — exits 0.
- [ ] [AI] `npx nx run-many -t test:e2e --projects=ose-app-be-e2e,ose-app-web-e2e` — passes or
      matches recorded baseline.
- [ ] [AI] `grep -rn "specs/apps/ose-app" apps/ specs/` returns only not-yet-migrated framing
      paths (`product`, `system-context`, `containers` non-contracts, `components`, `ddd`) — no
      stale `behavior`/`contracts` references.

> **Pause Safety**: `ose-app` behavior + contracts fully migrated and green; `ose-platform`
> untouched. Safe to stop. To resume: `npx nx run-many -t spec-coverage --projects=ose-app-be,ose-app-web`.

## Phase B: Migrate `ose-platform` → `specs/apps/ose/` (platform surfaces + cli)

- [ ] [AI] Move platform backend behavior with `api`→`be` rename:
      `git mv specs/apps/ose-platform/behavior/api/gherkin specs/apps/ose/behavior/platform-be/gherkin`.
      Acceptance: renames tracked; no `behavior/.../api/gherkin` path remains for OSE.
- [ ] [AI] Move platform web behavior:
      `git mv specs/apps/ose-platform/behavior/web/gherkin specs/apps/ose/behavior/platform-web/gherkin`.
      Acceptance: renames tracked.
- [ ] [AI] Resolve the cli-location open question
      ([tech-docs §Phase B note](./tech-docs.md#phase-b--ose-platform-consumers--platform-be--platform-web--cli)):
      `find specs/apps/ose-platform -type d -name cli`. If a second `cli/` exists, fold it in;
      then `git mv specs/apps/ose-platform/behavior/cli/gherkin specs/apps/ose/behavior/cli/gherkin`.
      Acceptance: all ose-cli Gherkin under one canonical `specs/apps/ose/behavior/cli/gherkin/`.
- [ ] [AI] Rewrite `apps/ose-web-fe-e2e/project.json` (L43) + `playwright.config.ts` (L9)
      `web/gherkin`→`platform-web/gherkin`. Verify: `npx nx run ose-web-fe-e2e:test:e2e` — passes
      or matches baseline.
- [ ] [AI] Rewrite `apps/ose-web-be-e2e/playwright.config.ts` `api/gherkin`→`platform-be/gherkin`,
      then regenerate playwright-bdd artifacts (re-run the e2e target so `.features-gen/`
      rebuilds). Verify: `npx nx run ose-web-be-e2e:test:e2e` — passes or matches baseline;
      `grep -rn "ose-platform" apps/ose-web-be-e2e/.features-gen` returns nothing.
- [ ] [AI] Rewrite `apps/ose-web/test/unit/be-steps/search.steps.ts` (L11)
      `api/gherkin`→`platform-be/gherkin`. Verify: `npx nx run ose-web:test:quick` — passes.
- [ ] [AI] Rewrite `apps/ose-cli/README.md` (L62, L102, L105) to
      `specs/apps/ose/behavior/cli/gherkin/`, and re-grep `apps/ose-cli` for any Go/source spec
      path references and rewrite them. Verify:
      `npx nx run-many -t test:quick,test:integration --projects=ose-cli` — passes or matches
      baseline.

### Phase B Gate

> All checks below must pass before starting Phase C.

- [ ] [AI] `npx nx run-many -t test:e2e --projects=ose-web-fe-e2e,ose-web-be-e2e` — passes or
      matches baseline.
- [ ] [AI] `npx nx run-many -t test:quick --projects=ose-web,ose-cli` — exits 0.
- [ ] [AI] `grep -rn "specs/apps/ose-platform" apps/` returns nothing.

> **Pause Safety**: all OSE behavior surfaces migrated and consumers green; only C4 framing docs
> still live under the old trees. Safe to stop. To resume:
> `npx nx run-many -t test:quick --projects=ose-web,ose-cli`.

## Phase C: Unify C4 framing + index

- [ ] [AI] Author `specs/apps/ose/README.md` by merging `specs/apps/ose-app/README.md` +
      `specs/apps/ose-platform/README.md` into one OSE-family index (app + platform sections).
      Acceptance: single H1, both deployable groups described, links resolve.
  - _Suggested executor: `specs-maker`_
- [ ] [AI] Merge `product/`, `system-context/`, `containers/` (non-contracts), `components/`, and
      `ddd/` from both old trees into `specs/apps/ose/` as unified docs with labelled per-product
      sections (use `git mv` for files that move 1:1; hand-merge files that collide such as
      `ddd/bounded-contexts.yaml` and `ddd/bounded-context-map.md`). Acceptance: no content lost
      vs. the two source trees; `specs/apps/ose-app/` and `specs/apps/ose-platform/` are empty.
  - _Suggested executor: `specs-maker`_
- [ ] [AI] Remove the now-empty old trees: `git rm -r` any residual `specs/apps/ose-app/` and
      `specs/apps/ose-platform/` scaffolding. Acceptance: `ls specs/apps` shows `ose` and no
      `ose-app`/`ose-platform`.
- [ ] [AI] Update `specs/README.md` (L32–33): replace the `ose-app` + `ose-platform` rows with a
      single `ose` row. Verify: `npx nx run rhino-cli:validate:links` — no broken links.
- [ ] [AI] Reconcile any DDD/contract input paths in `apps/ose-app-be/project.json` and
      `apps/ose-app-web/project.json` that point at `ddd/` now that framing has moved. Verify:
      `npx nx run-many -t spec-coverage --projects=ose-app-be,ose-app-web` — exits 0.

### Phase C Gate

> All checks below must pass before starting Phase D.

- [ ] [AI] `test -d specs/apps/ose && ! test -d specs/apps/ose-app && ! test -d specs/apps/ose-platform`
      — true.
- [ ] [AI] `grep -rn "specs/apps/ose-app\|specs/apps/ose-platform" . --exclude-dir=node_modules --exclude-dir=.git --exclude-dir=plans`
      returns nothing (plans archive excepted). Acceptance: zero stale references repo-wide.
- [ ] [AI] `npx nx run-many -t spec-coverage,test:quick --projects=ose-app-be,ose-app-web,ose-web,ose-cli`
      — exits 0.

> **Pause Safety**: single consolidated `specs/apps/ose/` tree exists and all consumers are green;
> the convention has not yet been amended. Safe to stop. To resume: re-run the Phase C grep gate.

## Phase D: Promote to repo-wide standard + conformance audit

- [ ] [AI] Amend
      `repo-governance/conventions/structure/specs-directory-structure.md`: add a
      "Multi-Deployable Family Layout" subsection documenting surface-prefixed `behavior/`
      subtrees (`<deployable>-be`, `<deployable>-web`, `cli`), name `be` as the standard
      backend-HTTP perspective (deprecating `api`), and cite `specs/apps/ose/` as the worked
      example. Acceptance: subsection present; existing single-deployable rule unchanged.
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] Cross-check `repo-governance/conventions/structure/app-readme-vs-specs.md` for OSE path
      references; update any that cite the old trees. Verify:
      `grep -n "ose-app\|ose-platform" repo-governance/conventions/structure/app-readme-vs-specs.md`
      — only app-name references (not spec-tree paths) remain.
- [ ] [AI] Update `.claude/agents/specs-checker.md`: add validation rules — (1) each `apps/`
      family maps to exactly one `specs/apps/<family>/` tree; (2) multi-deployable families use
      surface-prefixed behavior dirs; (3) flag `api`-named perspectives as non-standard. Then run
      `npm run generate:bindings` to sync `.opencode/` + `.amazonq/`. Verify:
      `npx nx run rhino-cli:validate:sync` — passes.
  - _Suggested executor: `agent-maker`_
- [ ] [AI] Read `.claude/agents/specs-maker.md`; if it enumerates per-family layout, add the
      multi-deployable template + re-sync bindings. Acceptance: maker and checker agree on the
      layout, or a note records that specs-maker needs no change.
- [ ] [AI] Conformance audit: for each family (`organiclever`, `ayokoding`, `wahidyankf`, `crane`,
      `rhino`, `ose`) verify exactly one `specs/apps/<family>/` tree and standard perspective
      names. Record results in this plan's implementation notes. Acceptance: all six confirmed
      conformant; any gap filed as a follow-up.
- [ ] [AI] Sweep `AGENTS.md`, `repo-governance/`, and `docs/` for residual
      `specs/apps/ose-app`/`specs/apps/ose-platform` path strings:
      `grep -rn "specs/apps/ose-app\|specs/apps/ose-platform" AGENTS.md repo-governance/ docs/`.
      Rewrite any hits. Acceptance: grep returns nothing.

### Phase D Gate

> All checks below must pass before quality gates / archival.

- [ ] [AI] `npx nx run rhino-cli:validate:sync` — passes (bindings synced).
- [ ] [AI] Conformance audit recorded; all six families conformant.
- [ ] [AI] Repo-wide grep for old spec-tree paths (excluding `plans/`, `node_modules`, `.git`)
      returns nothing.

> **Pause Safety**: convention amended, checker enforces it, all families conformant. Safe to
> stop. To resume: `npx nx run rhino-cli:validate:sync`.

## Local Quality Gates (Before Push)

- [ ] [AI] Run affected typecheck: `npx nx affected -t typecheck`.
- [ ] [AI] Run affected linting: `npx nx affected -t lint`.
- [ ] [AI] Run affected quick tests: `npx nx affected -t test:quick`.
- [ ] [AI] Run affected spec coverage: `npx nx affected -t spec-coverage`.
- [ ] [AI] Run markdown lint: `npm run lint:md`.
- [ ] [AI] Fix ALL failures found — including preexisting issues not caused by these changes.
- [ ] [AI] Verify all checks pass before pushing.

> **Important**: Fix ALL failures found during quality gates, not just those caused by your
> changes. This follows the root cause orientation principle — proactively fix preexisting errors
> encountered during work.

## Post-Push Verification

- [ ] [AI] Push changes to `main`.
- [ ] [AI] Monitor GitHub Actions workflows for the push (3-minute poll interval; do not use
      `gh run watch`).
- [ ] [AI] Verify all CI checks pass.
- [ ] [AI] If any CI check fails, fix immediately and push a follow-up commit.
- [ ] [AI] Do NOT proceed to archival until CI is green.

## Commit Guidelines

- [ ] [AI] Commit thematically — group related changes into logically cohesive commits.
- [ ] [AI] Suggested split: (1) `refactor(specs): migrate ose-app specs into specs/apps/ose`,
      (2) `refactor(specs): migrate ose-platform specs into specs/apps/ose`,
      (3) `refactor(specs): unify ose C4 framing and index`,
      (4) `docs(governance): standardize multi-deployable app spec layout`.
- [ ] [AI] Follow Conventional Commits; do NOT bundle unrelated fixes.

## Validation Checklist

- [ ] [AI] Single `specs/apps/ose/` tree; no `ose-app`/`ose-platform` trees remain.
- [ ] [AI] All affected `spec-coverage`, `test:quick`, and e2e suites pass.
- [ ] [AI] Contracts project renamed to `ose-contracts` and codegen green.
- [ ] [AI] Convention amended and `specs-checker` enforces the standard; bindings synced.
- [ ] [AI] Conformance audit recorded; all families conformant.
- [ ] [AI] All acceptance criteria in [prd.md](./prd.md) verified.

## Plan Archival

- [ ] [AI] Verify ALL delivery checklist items are ticked.
- [ ] [AI] Verify ALL quality gates pass (local + CI).
- [ ] [AI] Move plan folder from `plans/in-progress/` to `plans/done/` via `git mv`, adding the
      completion-date prefix (`YYYY-MM-DD__standardize-app-spec-trees`).
- [ ] [AI] Update `plans/in-progress/README.md` — remove the plan entry.
- [ ] [AI] Update `plans/done/README.md` — add the plan entry with completion date.
- [ ] [AI] Commit: `chore(plans): move standardize-app-spec-trees to done`.
