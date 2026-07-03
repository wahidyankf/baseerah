# Delivery Checklist — Enforce Identical, Fully-Enforcing rhino-cli Gherkin

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.

This is **one 3-repo execution plan** authored in `ose-public`. `ose-public` is canonical (Phases 0–2);
`ose-primer` (Phase 3) and `ose-infra` (Phase 4) receive verbatim propagation; Phase 5 verifies
cross-repo byte-identity, arms the anti-drift gate, and pushes all three. Sibling repos are at
`/Users/wkf/ose-projects/{ose-primer,ose-infra}` (same parent as this repo).

## Worktree

Worktree path: `worktrees/enforce-identical-rhino-cli-gherkin/`

Optional manual pre-provisioning (run from repo root):

```bash
claude --worktree enforce-identical-rhino-cli-gherkin
```

The plan-execution Step 0 gate enters this worktree by default: it auto-provisions from the latest
`origin/main` when missing, syncs with `origin/main` before implementing, and prompts before deleting the
worktree after the plan is archived and pushed. Phases 3 (`ose-primer`) and 4 (`ose-infra`) operate in
each sibling repo's own tree on `main` (Trunk Based Development); where a hook-safety check needs a
worktree, use that repo's `worktrees/<name>/` convention.

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

---

## Phase 0 — Baseline, Audit & Cross-Repo Census (ose-public)

- [ ] [AI] Provision + initialize toolchain: from repo root run `npm install && npm run doctor -- --fix`.
      Acceptance: doctor reports all tools OK (0 missing, 0 warning).
- [ ] [AI] Record clean baseline: `cargo test --release -p rhino-cli --no-fail-fast > audit/00-baseline.txt 2>&1`.
      Acceptance: exit 0; file committed.
- [ ] [AI] **Command census**: recurse `rhino-cli … --help` for every group/subcommand and write the full
      leaf-command tree to `audit/01-command-census.md`. Cross-check against
      [tech-docs §1.5](./tech-docs.md#15-canonical-command-surface-aligned-with-the-2026-07-01--2026-07-03-plans)
      and the [2026-07-03 synthesis ledger](../../done/2026-07-03__unify-rhino-cli-sdlc-parity/audit/synthesis-ledger.md).
      Acceptance: every leaf command listed; any drift from §1.5 flagged.
- [ ] [AI] **Hollow-scenario census**: parse the baseline output into `audit/02-hollow-census.md` —
      per-binary passed/skipped counts + the exact `.feature:line` of every skipped scenario and the
      step string that failed to match. Acceptance: total skipped count equals the baseline's summary
      (expected 121 at authoring time — re-derive, do not assume).
- [ ] [AI] **Unbound-dir census**: list every `gherkin/<dir>` and the `tests/*.rs` binary (if any) that
      binds it (`grep -rn 'join(".*gherkin/' apps/rhino-cli/tests`), into `audit/03-unbound-dirs.md`.
      Acceptance: the 4 unbound dirs (`ddd`, `git`, `specs`, `test-coverage`) confirmed or corrected.
- [ ] [AI] **Command↔feature map**: in `audit/04-coverage-map.md`, map each leaf command to its covering
      `.feature`(s) and mark: enforcing / hollow / absent. Acceptance: every leaf command has a row; the
      gap set (absent + hollow) is enumerated.
- [ ] [AI] **Cross-repo diff**: write `audit/05-cross-repo-diff.md` — `md5` manifest of every `.feature` + behaviour-`README.md` in all three repos and a `diff -rq` summary. Acceptance: reproduces the
      public=infra / primer-stale finding (or its current-state correction).

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [ ] [AI] `cargo test --release -p rhino-cli` — exits 0 (green baseline recorded).
- [ ] [AI] All six `audit/0*.md` files exist and are committed.
- [ ] [AI] `nx affected -t test:quick,lint,typecheck --base=origin/main` — exits 0.

> **Pause Safety**: baseline green, audit evidence committed, no source or spec changes applied. ose-public
> passes its own affected pre-push gate. Safe to stop. To resume: `cargo test --release -p rhino-cli`.

---

## Phase 1 — De-Hollow + Wire + Gap-Fill the Canonical Tree (ose-public)

> Every code-touching item is TDD-shaped. "De-hollow" = a scenario moves from `skipped` → `passed` in the
> cucumber summary. Suggested executor for all `tests/*.rs` + `src/` edits: `swe-rust-dev`.

### 1·0. Rename feature dirs to match command groups (prerequisite — do first)

- [ ] [AI] Apply the Phase-0 rename mapping (`audit/04-coverage-map.md`) with `git mv`: confirmed
      `specs/apps/rhino/behavior/rhino-cli/gherkin/docs/` → `…/gherkin/md/` and `…/gherkin/agents/` →
      `…/gherkin/harness/`, plus any other dir whose name mismatches its command group. Retarget the
      matching `feature_dir()` binding(s) in `apps/rhino-cli/tests/*.rs` (e.g. `tests/docs.rs`,
      `tests/agents.rs`). Command: `cargo test --release -p rhino-cli --no-fail-fast`. Acceptance: suite
      still builds and runs (skip counts unchanged — pure rename, no vocab change yet); no dir name
      mismatches its command group in `audit/04-coverage-map.md`.
  - _Suggested executor: `swe-rust-dev`_

> The de-hollow subsections below operate on the **renamed** dirs (e.g. `gherkin/md/`, `gherkin/harness/`).

### 1·0b. Introduce the mock I/O seam + reclassify test:unit (prerequisite — Decision 5)

- [ ] [AI] **RED**: add trait seam(s) (`Fs`, `GitRepo` or similar) in `apps/rhino-cli/src` with a real
      (imperative-shell) impl and a `Mock*` impl for tests; add one core validator unit test that calls a
      validator in-process with a mocked FS. Command: `cargo test --manifest-path apps/rhino-cli/Cargo.toml --lib`.
      Acceptance: new mocked unit test fails (validator not yet dependency-injected).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: thread the seam through the core validators (functional-core/imperative-shell) so
      they accept injected `Fs`/`GitRepo`. Command: same. Acceptance: mocked unit test passes; existing
      `--lib` + `--tests` still green.
- [ ] [AI] **REFACTOR**: converge duplicated I/O call sites onto the seam. Command: `cargo test --release -p rhino-cli`.
      Acceptance: all tiers still green.
- [ ] [AI] Edit `apps/rhino-cli/project.json`: point `test:unit` at the **mocked in-process behaviour
      suite** (so the pre-push `test:quick` gate runs it) and keep `test:integration` as the temp-fixture
      (`--tests`) suite. Acceptance: `nx run rhino-cli:test:unit` runs the mocked behaviour scenarios;
      `nx run rhino-cli:test:integration` runs the temp-fixture suite; `repo-config.yml` keeps
      `rhino-cli levels: [unit, integration]`.

### 1a. De-hollow `repo_governance` (61/61 skipped → 0)

- [ ] [AI] **RED**: in `apps/rhino-cli/tests/repo_governance.rs`, align every `#[given]/#[when]/#[then]`
      string to the canonical feature step text in `specs/apps/rhino/behavior/rhino-cli/gherkin/repo-governance/**`
      and ensure each `#[when]` body invokes the real current command (e.g. `repo-governance workflows naming validate`,
      `repo-governance vendor validate`, `repo-governance audit`). Run `cargo test --release -p rhino-cli --test repo_governance`.
      Acceptance: scenarios now **execute** (summary shows passed/failed, not skipped) — failures here are real behaviour assertions to satisfy.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: resolve each executing failure at root cause (fix the step body / fixture, or the
      validator if a genuine bug surfaces — never re-skip). Command: `cargo test --release -p rhino-cli --test repo_governance`.
      Acceptance: `61 scenarios (61 passed)`, `0 skipped`.
- [ ] [AI] **REFACTOR**: dedupe shared step helpers in `tests/repo_governance.rs`. Command: same.
      Acceptance: still `0 skipped`, all passed.

### 1b. De-hollow `docs` (43/69 skipped → 0)

- [ ] [AI] **RED**: align step strings in `apps/rhino-cli/tests/docs.rs` to the `md` command names
      (`md links validate`, `md mermaid validate`, `md heading-hierarchy validate`, `md naming validate`,
      `md frontmatter validate`, `md frontmatter-dates validate`, `md readme-index validate`, `md audit`).
      Command: `cargo test --release -p rhino-cli --test docs`. Acceptance: scenarios execute (not skipped).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: satisfy each executing scenario at root cause. Command: same.
      Acceptance: `69 scenarios (69 passed)`, `0 skipped`.
- [ ] [AI] **REFACTOR**: dedupe helpers. Command: same. Acceptance: `0 skipped`.

### 1c. De-hollow `agents` (13/28 skipped → 0)

- [ ] [AI] **RED**: align step strings in `apps/rhino-cli/tests/agents.rs` to `harness` command names
      (`harness bindings generate/validate`, `harness naming validate`, `harness duplication validate`,
      `harness sync`, `harness audit`, `harness instruction-size validate`, `harness claude …`).
      Command: `cargo test --release -p rhino-cli --test agents`. Acceptance: scenarios execute.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: satisfy each. Command: same. Acceptance: `28 scenarios (28 passed)`, `0 skipped`.
- [ ] [AI] **REFACTOR**: dedupe. Command: same. Acceptance: `0 skipped`.

### 1d. De-hollow `workflows` (4/4 skipped → 0)

- [ ] [AI] **RED**: change the `#[when]` string in `apps/rhino-cli/tests/workflows.rs:151` from
      `the developer runs workflows validate-naming` to `the developer runs repo-governance workflows naming validate`
      (matching the feature) and invoke that command. Command: `cargo test --release -p rhino-cli --test workflows`.
      Acceptance: 4 scenarios execute (not skipped).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: satisfy assertions. Command: same. Acceptance: `4 scenarios (4 passed)`, `0 skipped`.
- [ ] [AI] **REFACTOR**: none needed unless duplication appears. Command: same. Acceptance: `0 skipped`.

### 1e. Wire the 4 unbound feature dirs (ddd / git / specs / test-coverage)

- [ ] [AI] For each of `ddd`, `git`, `specs`, `test-coverage`: add a cucumber `[[test]]` binary
      (`harness = false`) in `apps/rhino-cli/Cargo.toml` + a `tests/<name>.rs` following the exact pattern
      of an existing binary (async `main()` → `World::run(feature_dir())` bound to that dir + step defs).
      Per [tech-docs §1.5](./tech-docs.md), `test-coverage` diff/merge scenarios assert **internal**
      behaviour (`application/testcoverage/{diff,merge}.rs`) or scope to `test-coverage validate` — no
      invented CLI verb.
  - [ ] [AI] **RED**: register the binary + empty step scaffold; `cargo test --release -p rhino-cli --test <name>`.
        Acceptance: scenarios execute and fail/undefined (proving the dir is now bound).
  - [ ] [AI] **GREEN**: implement step defs against real commands/internal behaviour; same command.
        Acceptance: all scenarios in the dir pass, `0 skipped`.
  - _Suggested executor: `swe-rust-dev`_

### 1f. Gap-fill uncovered leaf commands

- [ ] [AI] **RED**: from `audit/04-coverage-map.md`, for each leaf command with no scenario add a
      `.feature` in its existing domain dir. Priority gap: create
      `specs/apps/rhino/behavior/rhino-cli/gherkin/specs/gherkin-cardinality.feature` for `specs gherkin-cardinality`
      (modernize primer's stale `repo-governance-gherkin-keyword-cardinality.feature` content to the new command), + a step def in the relevant binary. Command: `cargo test --release -p rhino-cli --test <binary>`.
      Acceptance: new scenarios execute and fail (RED).
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: make each gap-fill scenario pass against the real command. Command: same.
      Acceptance: all pass, `0 skipped`; every leaf command in `audit/04-coverage-map.md` now marked enforcing.
- [ ] [AI] Reconcile the env-validate app-drift behaviour (AC-6): ensure `env validate` declared-but-unread /
      read-but-undeclared behaviour (today only in the plain `tests/env_validate_integration.rs`) owns an
      **executing cucumber scenario** under `env/` or `env-contract/`. Command: `cargo test --release -p rhino-cli --test env`.
      Acceptance: the drift behaviour has a passing cucumber scenario.

### 1g. Enable cucumber fail-on-skip (lock 0-skip — Decision 6)

- [ ] [AI] Configure every cucumber World runner (`apps/rhino-cli/tests/*.rs`) with `.fail_on_skipped()`
      (or the 0.23 equivalent) so an undefined/skipped step **fails** the run. Command:
      `cargo test --release -p rhino-cli --no-fail-fast`. Acceptance: with all scenarios de-hollowed the
      suite still exits 0; introduce a temporary bogus step to confirm it now **fails** (then revert).
  - _Suggested executor: `swe-rust-dev`_

### 1h. @covers completeness for rhino-cli (Decision 7)

- [ ] [AI] Ensure every rhino-cli scenario carries its `@unit`/`@integration` level tag(s) and a matching
      `// @covers <spec-path>:<scenario-title>` marker at each declared level (per-scenario envelope from
      `audit/04-coverage-map.md`). Command: `rhino-cli specs behavior-coverage validate`. Acceptance: exit 0
      with no untagged/uncovered/orphan findings.
  - _Suggested executor: `swe-rust-dev`_

### 1i. Regenerate golden-master

- [ ] [AI] Regenerate `apps/rhino-cli/tests/golden-master/**` from the canonical binary per the
      predecessor's method (`{{TMPDIR}}` sentinel + `--no-color`). Command: `cargo test --release -p rhino-cli --test golden_master`.
      Acceptance: golden-master test passes; review the diff for intent before freezing.

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [ ] [AI] `cargo test --release -p rhino-cli --no-fail-fast` — exits 0 **with `0 skipped` in every binary**
      (grep the output: `grep -c "skipped)"` returns 0). **This is the core acceptance of the whole plan.**
- [ ] [AI] `nx run rhino-cli:test:unit` (mocked, in-process) runs the behaviour suite and passes — proving
      it now executes inside the pre-push `test:quick` gate; `nx run rhino-cli:test:integration` (temp-fixture)
      also green.
- [ ] [AI] Cucumber `fail_on_skipped` is active (a bogus undefined step reddens the build).
- [ ] [AI] `nx affected -t lint,typecheck --base=origin/main` — exits 0 (public's strict clippy/doc lints pass).
- [ ] [AI] `rhino-cli specs structure validate` + `rhino-cli specs behavior-coverage validate` — exit 0.

> **Pause Safety**: the canonical tree is fully enforcing (0 skipped), golden-master regenerated, ose-public
> green on its own gate. Safe to stop. To resume: `cargo test --release -p rhino-cli`.

---

## Phase 2 — Freeze Canonical + Author Anti-Drift Gate (ose-public)

- [ ] [AI] Freeze the propagation source: record `audit/06-canonical-manifest.md` = `md5` of every
      `apps/rhino-cli` tracked file + every `gherkin/**/*.feature` + `gherkin/**/README.md`.
      Acceptance: manifest committed.
- [ ] [AI] Extend the SDLC parity gate: edit
      [`docs/reference/sdlc-gate-standard.md`](../../../docs/reference/sdlc-gate-standard.md) — add
      `specs/apps/rhino/behavior/rhino-cli/gherkin/**` (`.feature` + `README.md`) to the rhino-cli
      byte-identity boundary section. Acceptance: the path appears in the boundary definition.
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] Add a verification step to
      [`repo-governance/workflows/plan/plan-multi-repo-parity-planning.md`](../../../repo-governance/workflows/plan/plan-multi-repo-parity-planning.md)
      that diffs the Gherkin tree md5-manifest across the three repos. Acceptance: the step is present with an explicit command.
  - _Suggested executor: `repo-workflow-maker`_
- [ ] [AI] Update the byte-identity notes in `AGENTS.md` + `CLAUDE.md` to mention the Gherkin tree is
      in-boundary. Acceptance: both files reference it; `npm run generate:bindings` re-synced.
- [ ] [AI] **Re-place the repo-config schema gate** (Decision 8, closes the 2026-07-03 Decision-5 gap) per
      [tech-docs §1.6](./tech-docs.md#16-repo-config-schema-parity-gate-is-missing-at-pre-commit):
      (a) add the staged-gated step to `ose-public/.husky/pre-commit` after the `env staged-guard` step
      (`git diff --cached --name-only … | grep '^repo-config\.yml$'` → `rhino-cli repo-config validate`);
      (b) add a standalone `rhino-cli repo-config validate` step to `.github/workflows/pr-quality-gate.yml`
      and `.github/workflows/main-ci.yml`; (c) **remove** the `repo-config validate` line from
      `.husky/pre-push` (`ose-public` `:10`). Verify: staging a bogus-key `repo-config.yml` + `sh .husky/pre-commit`
      rejects it (revert after); `grep -c "repo-config validate" .husky/pre-push` returns 0. Acceptance: gate
      fires at pre-commit/PR/main, absent from pre-push.
- [ ] [AI] Run `rhino-cli md links validate` + `rhino-cli md readme-index validate` over the new plan +
      edited docs. Acceptance: exit 0.

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [ ] [AI] `sh .husky/pre-push` (ose-public root) — exits 0.
- [ ] [AI] `audit/06-canonical-manifest.md` exists; boundary doc + parity workflow updated.

> **Pause Safety**: ose-public is fully at target — enforcing suite, frozen manifest, anti-drift gate
> authored, own pre-push green. Safe to stop. To resume: `sh .husky/pre-push`.

---

## Phase 3 — Propagate to ose-primer

- [ ] [AI] Copy canonical `apps/rhino-cli/` (excluding `target/`, `dist/`, `cover.out`, `lcov.info`) from
      ose-public into `/Users/wkf/ose-projects/ose-primer/apps/rhino-cli/`. Acceptance:
      `diff -rq --exclude=target --exclude=dist ose-public/apps/rhino-cli ose-primer/apps/rhino-cli` shows
      only untracked-artifact/README differences (zero source/tests/feature diffs).
- [ ] [AI] Replace primer's `specs/apps/rhino/behavior/rhino-cli/gherkin/` tree with the canonical tree
      (`.feature` + behaviour-`README.md`); delete the 2 stale files (`env/env-validate.feature`,
      `repo-governance/repo-governance-gherkin-keyword-cardinality.feature`). Acceptance:
      `diff -rq` of the gherkin `.feature`+README set between public and primer is empty.
- [ ] [AI] Propagate the boundary/workflow/AGENTS/CLAUDE edits **and the pre-commit `repo-config validate`
      staged-gate step** from Phase 2 into primer; run `npm run generate:bindings`. Acceptance: bindings
      synced; primer's `.husky/pre-commit` step is byte-identical to public's.
- [ ] [AI] Run `cargo test --release -p rhino-cli --no-fail-fast` in ose-primer. Acceptance: exit 0, `0 skipped`.

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [ ] [AI] `md5` manifest of primer's gherkin tree == `audit/06-canonical-manifest.md`. Acceptance: identical.
- [ ] [AI] `sh .husky/pre-push` (ose-primer root) — exits 0 with `0 skipped`.

> **Pause Safety**: primer's rhino-cli + gherkin tree are byte-identical to public and fully enforcing;
> primer passes its own pre-push. Safe to stop. To resume: `sh .husky/pre-push` (primer root).

---

## Phase 4 — Propagate to ose-infra

- [ ] [AI] Copy canonical `apps/rhino-cli/` from ose-public into
      `/Users/wkf/ose-projects/ose-infra/apps/rhino-cli/` (same exclusions). Acceptance:
      `diff -rq` shows only untracked-artifact/README differences.
- [ ] [AI] Sync infra's gherkin tree to canonical (`.feature` + behaviour-`README.md`). Since infra's
      `.feature` set was already identical to public pre-plan, this applies the de-hollow/gap-fill deltas.
      Acceptance: `diff -rq` of the `.feature`+README set between public and infra is empty.
- [ ] [AI] Propagate the boundary/workflow/AGENTS/CLAUDE edits **and the pre-commit `repo-config validate`
      staged-gate step** into infra; run `npm run generate:bindings`. Acceptance: bindings synced; infra's
      `.husky/pre-commit` step is byte-identical to public's (accounting for infra's existing hook-mechanism
      divergence — the added step invokes the same `cargo run … repo-config validate`).
- [ ] [AI] Run `cargo test --release -p rhino-cli --no-fail-fast` in ose-infra. Acceptance: exit 0, `0 skipped`.

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [ ] [AI] `md5` manifest of infra's gherkin tree == `audit/06-canonical-manifest.md`. Acceptance: identical.
- [ ] [AI] `sh .husky/pre-push` (ose-infra root) — exits 0 with `0 skipped`.

> **Pause Safety**: infra's rhino-cli + gherkin tree are byte-identical to public and fully enforcing;
> infra passes its own pre-push. Safe to stop. To resume: `sh .husky/pre-push` (infra root).

---

## Phase 5 — Cross-Repo Verification, Push & Archival

### Local Quality Gates (Before Push)

> **Important**: Fix ALL failures found during quality gates, not just those caused by your changes
> (root cause orientation — proactively fix preexisting errors encountered).

- [ ] [AI] **Cross-repo byte-identity matrix**: for all three repos, verify `apps/rhino-cli` (excl.
      artifacts) and the gherkin `.feature`+README set are byte-identical (`diff -rq` pairwise + md5
      manifests all equal `audit/06-canonical-manifest.md`). Acceptance: zero differences across all three.
- [ ] [AI] **Enforcement matrix**: `cargo test --release -p rhino-cli --no-fail-fast` in each repo reports
      `0 skipped` and exit 0. Acceptance: identical scenario counts, all passed, none skipped, in all three.
- [ ] [AI] Per repo: `nx affected -t typecheck,lint,test:quick,specs:coverage --base=origin/main` — exits 0.

### Commit Guidelines

- [ ] [AI] Commit thematically per repo (Conventional Commits), staging **explicit paths only** (never
      `git add -A` — sibling repos may carry unrelated WIP). Suggested split per repo:
      `test(rhino-cli): de-hollow + wire gherkin so all behaviour is enforced`,
      `test(specs): make rhino-cli gherkin tree byte-identical across repos`,
      `docs(governance): bring rhino-cli gherkin tree into the SDLC parity boundary`.
- [ ] [AI] Verify each repo's staged set contains only rhino-cli + specs/gherkin + governance-doc paths.

### Post-Push Verification

- [ ] [AI] Push `ose-public` → `origin main`. Monitor GitHub Actions; verify green (poll every 2 min, one
      `gh run view --json status,conclusion` per wakeup). If red, root-cause + fix before proceeding.
- [ ] [AI] Push `ose-primer` → `origin main`. Verify CI green.
- [ ] [AI] Push `ose-infra` → `origin main`. Verify CI green.
- [ ] [AI] Do NOT mark the plan done until all three repos' CI is green.

> Manual UI/API verification (Playwright/curl), Rule-15 web-triad, and Rule-16 API retest are **Not
> applicable** — this plan touches only CLI/tooling source, specs, and governance docs (no web UI, no HTTP API).

### Phase 5 Gate

- [ ] [AI] All three repos converged, byte-identity matrix all-green, every suite `0 skipped`, all three
      `main` CI runs green.

> **Pause Safety**: all three repos converged, parity-verified, fully enforcing, and CI-green; nothing
> half-applied. Safe to stop. To resume: re-run the byte-identity + enforcement matrices (this phase's
> first two items) and confirm all-green.

### Plan Archival

- [ ] [AI] Verify ALL delivery checklist items are ticked.
- [ ] [AI] Verify ALL quality gates pass (local + CI in all three repos).
- [ ] [AI] Verify the byte-identity + `0 skipped` enforcement matrices are green across all three repos.
- [ ] [AI] Move plan folder: `git mv plans/in-progress/enforce-identical-rhino-cli-gherkin plans/done/2026-07-03__enforce-identical-rhino-cli-gherkin` (use the actual completion date).
- [ ] [AI] Update `plans/in-progress/README.md` — remove the plan entry.
- [ ] [AI] Update `plans/done/README.md` — add the plan entry with completion date.
- [ ] [AI] Commit: `chore(plans): move enforce-identical-rhino-cli-gherkin to done`.

## Validation Checklist

- [ ] All TDD cycles complete (RED→GREEN→REFACTOR for every `tests/*.rs`/`src` change)
- [ ] `0 skipped` scenarios in the rhino-cli suite in all three repos
- [ ] Gherkin `.feature` + behaviour-`README.md` byte-identical across all three repos
- [ ] `apps/rhino-cli` byte-identical across all three repos (zero carve-outs)
- [ ] Every leaf command maps to ≥ 1 executing scenario (`audit/04-coverage-map.md`)
- [ ] Anti-drift gate armed (SDLC boundary doc + parity workflow step)
- [ ] `repo-config validate` wired at pre-commit (staged-gated) byte-identical in all three repos
- [ ] All three repos' CI green
