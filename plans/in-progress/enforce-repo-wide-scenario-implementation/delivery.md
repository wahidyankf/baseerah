# Delivery Checklist — Enforce Repo-Wide Gherkin Scenario Implementation

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret handling).
> `[AI+HUMAN]`: agent prepares, human approves or finishes.

**Precondition (hard gate)**: [`enforce-identical-rhino-cli-gherkin`](../enforce-identical-rhino-cli-gherkin/README.md)
is **DONE and archived** (rhino-cli fully enforcing, `fail_on_skipped` on, `@covers` complete, tiers
wired). Do not start Phase 1 until that plan is in `plans/done/`.

## Worktree

Worktree path: `worktrees/enforce-repo-wide-scenario-implementation/`

Optional manual pre-provisioning (run from repo root):

```bash
claude --worktree enforce-repo-wide-scenario-implementation
```

The plan-execution Step 0 gate enters this worktree by default: it auto-provisions from the latest
`origin/main` when missing, syncs before implementing, and prompts before deleting after archival. The
engine change (Phase 1) propagates to `ose-primer`/`ose-infra` in their own trees on `main`; per-project
rollout batches run in each repo for its own apps/libs.

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

---

## Phase 0 — Audit & Baseline

- [ ] [AI] Provision + toolchain: `npm install && npm run doctor -- --fix`. Acceptance: all tools OK.
- [ ] [AI] Confirm the dependency plan is archived: `test -d plans/done/*enforce-identical-rhino-cli-gherkin`.
      Acceptance: present; otherwise STOP.
- [ ] [AI] **Scenario census**: per project in `repo-config.yml` `coverage.projects`, count scenarios +
      current level tags → `audit/01-scenario-census.md`. Acceptance: every eligible project has a row.
- [ ] [AI] **@covers adoption census**: `git grep -l "@covers " -- apps libs` grouped by project →
      `audit/02-covers-adoption.md`. Acceptance: reproduces the rhino-cli-only finding (or its correction).
- [ ] [AI] **Per-tier skip inventory**: find `.skip`/`.only`/`.todo` (Jest/Vitest/Playwright), F# ignored
      tests, and undefined cucumber steps across the repo → `audit/03-skip-inventory.md`. Acceptance: the
      backlog of currently-skipped tests is quantified.
- [ ] [AI] **behavior-coverage vacuity check**: run `nx run <non-rhino-project>:specs:behavior:coverage`
      for a sample; record whether it passes vacuously (no markers) or fails → `audit/04-vacuity.md`.
      Acceptance: Open Question in tech-docs §7 resolved.
- [ ] [AI] **Reporter availability**: for each tier tool, confirm a machine-readable (JSON/TRX) reporter +
      the fail-on-skip flag via `--help`/docs → `audit/05-reporters.md`. Acceptance: per-tool mechanism
      confirmed (verified, not assumed).

### Phase 0 Gate

- [ ] [AI] `nx affected -t test:quick,lint,typecheck --base=origin/main` — exits 0.
- [ ] [AI] All five `audit/0*.md` committed; the rollout backlog is sized.

> **Pause Safety**: audit-only, no behaviour change. Safe to stop. To resume: re-run the census commands.

---

## Phase 1 — behavior-coverage runtime cross-check (engine)

> Suggested executor: `swe-rust-dev`. rhino-cli's own now-enforcing suite is the first consumer.

- [ ] [AI] **RED**: add a spec + failing test in `apps/rhino-cli` for "a scenario with a valid `@covers`
      marker whose covering test was skipped at runtime FAILS `behavior-coverage`". Command:
      `cargo test -p rhino-cli`. Acceptance: new test fails (cross-check not implemented).
- [ ] [AI] **GREEN**: implement the runtime cross-check in
      `apps/rhino-cli/src/application/behavior_coverage/**` — ingest each tier's JSON run report and assert
      each `@covers` scenario executed AND passed at its level. Command: same. Acceptance: new test passes;
      existing suite green.
- [ ] [AI] **REFACTOR**: factor the per-tier report parsers behind one trait. Command: same. Acceptance:
      all green.
- [ ] [AI] Regenerate rhino-cli golden-master; propagate the byte-identical `apps/rhino-cli` to
      `ose-primer` + `ose-infra`. Acceptance: `diff -rq apps/rhino-cli` across the three shows only
      untracked-artifact/README diffs.

### Phase 1 Gate

- [ ] [AI] `cargo test -p rhino-cli` green in all three repos; golden-master passes.
- [ ] [AI] `apps/rhino-cli` byte-identical across the three repos.

> **Pause Safety**: engine landed + parity-verified; no per-project rollout yet. Safe to stop. To resume:
> `cargo test -p rhino-cli`.

---

## Phase 2 — Per-tier fail-on-skip config (repo-wide)

- [ ] [AI] Jest/Vitest: enable `--forbid-only` (or config) and a skip-guard so `.skip`/`.todo` fail in CI,
      per `audit/05-reporters.md`. Verify by planting a `.skip` and running the affected unit tier — it
      reddens; revert. Acceptance: skip fails the tier.
- [ ] [AI] Playwright: set `forbidOnly: true` + a `test.skip` guard in each `apps/*-e2e/playwright.config.ts`.
      Verify by planting a skip — e2e tier reddens; revert. Acceptance: skip fails the tier.
- [ ] [AI] F#: configure the test runner to fail on ignored/pending tests. Verify with a planted ignore.
      Acceptance: ignored test fails the tier.
- [ ] [AI] (cucumber-rs already fail-on-skip via the dependency plan — confirm still active.)

### Phase 2 Gate

- [ ] [AI] Each tier reddens on a planted skip (evidence in `audit/06-fail-on-skip-proof.md`).
- [ ] [AI] `nx affected -t test:quick --base=origin/main` — exits 0 (no unexpected skips remain in-scope).

> **Pause Safety**: every tier now fails on skip; `@covers` rollout not yet begun. Safe to stop. To
> resume: re-run the planted-skip proofs.

---

## Phase 3..N — Per-project @covers + level-tag rollout (batched)

> Repeat this phase per project batch from `audit/01`/`02` (one bounded group per phase — e.g. one domain
> or one lib per phase — so each is a natural pause). Suggested executor: the project's language dev agent.

For each project in the batch:

- [ ] [AI] Level-tag every scenario in the project's `specs/**` features (`@unit`/`@integration`/`@e2e`)
      per its `coverage.projects` envelope. **No defer, no shortcut** (Decision 4): no scenario is
      `@wip`-tagged, skipped, or parked — all are implemented in this batch. Command:
      `rhino-cli specs behavior-coverage validate`. Acceptance: no untagged findings; zero `@wip`.
- [ ] [AI] Add `// @covers <spec-path>:<scenario-title>` markers to the project's tests at each declared
      level; **write the real test** where the runtime cross-check reveals a scenario is unimplemented.
      Command: `nx run <project>:test:unit` (+`:test:integration`/`:test:e2e` as applicable) then
      `nx run <project>:specs:behavior:coverage`. Acceptance: cross-check passes — every scenario executed
      and passed at its levels; zero silent skips.

### Phase N Gate (each batch)

- [ ] [AI] `nx run <project>:specs:behavior:coverage` — exit 0, non-vacuous (markers present).
- [ ] [AI] `nx affected -t test:quick,specs:coverage --base=origin/main` — exits 0.
- [ ] [AI] **Zero deferrals**: the project has no `@wip`, no `.skip`/`.only`/`.todo`, no
      marker-without-a-real-test — every scenario executed and passed (`grep`-proof recorded in
      `audit/07-no-defer-proof.md`).

> **Pause Safety**: the completed batches are fully enforced; remaining projects are untouched and still
> pass their existing gates. Safe to stop between batches. To resume: pick the next batch.

---

## Final Phase — Wire, Cross-Repo Verify & Archival

### Local Quality Gates (Before Push)

> **Important**: Fix ALL failures found during quality gates, not just those caused by your changes.

- [ ] [AI] Wire the runtime cross-check into `specs:coverage` (pre-push) + CI (`main-ci`) so it runs for
      every affected project. Acceptance: a planted marked-but-skipped scenario blocks pre-push and CI.
- [ ] [AI] Per repo: `nx run-many --all -t typecheck,lint,test:quick,specs:coverage` — exits 0,
      non-vacuous, zero silent skips.
- [ ] [AI] Cross-repo: `apps/rhino-cli` (engine) byte-identical across the three repos.

### Commit Guidelines

- [ ] [AI] Commit thematically, explicit paths only (never `git add -A`). Split: engine
      (`feat(rhino-cli): behavior-coverage runtime cross-check`), per-tier config
      (`test: fail-on-skip across tiers`), per-project rollout (`test(<project>): @covers + level tags`).

### Post-Push Verification

- [ ] [AI] Push each repo → `origin main`; monitor CI (poll every 2 min, one `gh run view` per wakeup);
      verify green; fix any failure before proceeding.

> Manual UI/API verification, Rule-15 web-triad, Rule-16 API retest: **Not applicable** — no UI/API
> surface is added (test-wiring + config + CLI engine only).

### Final Gate

- [ ] [AI] Every eligible project: `specs:behavior:coverage` non-vacuous + runtime cross-check green;
      every tier fails on skip; all three repos' CI green.

> **Pause Safety**: repo-wide enforcement live and honest; nothing half-applied. Safe to stop. To resume:
> re-run `nx run-many --all -t specs:coverage`.

### Plan Archival

- [ ] [AI] Verify ALL delivery items ticked and ALL gates pass (local + CI, all three repos).
- [ ] [AI] Verify **zero deferrals** repo-wide: no `@wip`, no `.skip`/`.only`/`.todo`, no
      marker-without-a-real-test anywhere (`audit/07-no-defer-proof.md` shows a clean grep).
- [ ] [AI] Move plan: `git mv plans/in-progress/enforce-repo-wide-scenario-implementation plans/done/<completion-date>__enforce-repo-wide-scenario-implementation`.
- [ ] [AI] Update `plans/in-progress/README.md` (remove entry) + `plans/done/README.md` (add entry).
- [ ] [AI] Commit: `chore(plans): move enforce-repo-wide-scenario-implementation to done`.

## Validation Checklist

- [ ] All TDD cycles complete for the engine cross-check (RED→GREEN→REFACTOR)
- [ ] Every tier fails on skip/only/todo (proofs committed)
- [ ] `behavior-coverage` runtime cross-check live and wired to pre-push + CI
- [ ] `@covers` + level tags on every eligible app/lib; `behavior-coverage` non-vacuous
- [ ] Engine byte-identical across the three repos; all three repos' CI green
- [ ] Zero deferrals repo-wide: no `@wip`, no `.skip`/`.only`/`.todo`, no marker-without-a-real-test
