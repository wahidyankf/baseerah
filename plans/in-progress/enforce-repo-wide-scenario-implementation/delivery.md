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

## Delivery Phase Flow

```mermaid
%% Color palette: Blue #0173B2, Orange #DE8F05, Teal #029E73, Brown #CA9161 (color-blind friendly)
flowchart TD
  Pre["Precondition:<br/>sibling plan DONE<br/>and archived"]:::brown --> P0["Phase 0<br/>Audit and Baseline"]:::blue
  P0 --> G0{"Phase 0 Gate"}:::orange
  G0 -->|"pass"| P1["Phase 1<br/>Engine: runtime cross-check"]:::blue
  P1 --> G1{"Phase 1 Gate"}:::orange
  G1 -->|"pass"| P2["Phase 2<br/>Per-tier fail-on-skip config"]:::blue
  P2 --> G2{"Phase 2 Gate"}:::orange
  G2 -->|"pass"| P3["Phase 3..N<br/>Per-project rollout<br/>batched"]:::blue
  P3 --> GN{"Phase N Gate<br/>each batch"}:::orange
  GN -->|"more batches"| P3
  GN -->|"all batches done"| PF["Final Phase<br/>Wire and Cross-Repo<br/>Verify and Archive"]:::blue
  PF --> GF{"Final Gate"}:::orange
  GF -->|"pass"| Done["Plan Archived"]:::teal

  classDef blue fill:#0173B2,stroke:#000000,color:#FFFFFF,stroke-width:2px
  classDef orange fill:#DE8F05,stroke:#000000,color:#FFFFFF,stroke-width:2px
  classDef teal fill:#029E73,stroke:#000000,color:#FFFFFF,stroke-width:2px
  classDef brown fill:#CA9161,stroke:#000000,color:#FFFFFF,stroke-width:2px
```

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
- [ ] [AI] **behavior-coverage vacuity check**: run `nx run organiclever-be:specs:behavior:coverage` (or
      any other non-rhino project from `repo-config.yml` `coverage.projects`, as a sample); record whether
      it passes vacuously (no markers) or fails → `audit/04-vacuity.md`. Acceptance: Open Question in
      tech-docs §7 resolved.
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

- [ ] [AI] **RED**: add a new scenario to
      `specs/apps/rhino/behavior/rhino-cli/gherkin/specs/behavior-coverage.feature` ("a scenario with a
      valid `@covers` marker whose covering test was skipped at runtime FAILS `behavior-coverage`") and a
      matching failing `#[test]` in the `mod tests` block of
      `apps/rhino-cli/src/application/behavior_coverage/validator.rs` (sibling to `mod.rs`/`types.rs`),
      tagged `// @covers specs/apps/rhino/behavior/rhino-cli/gherkin/specs/behavior-coverage.feature:<scenario
title>`. Command: `cargo test -p rhino-cli`. Acceptance: new test fails (cross-check not
      implemented).
  - **Gherkin (binds) →** "A marked-but-unexecuted scenario fails the central gate" (AC-2)

    ```gherkin
    Scenario: A marked-but-unexecuted scenario fails the central gate
      Given a scenario with a valid @covers marker whose covering test is skipped at runtime
      When rhino-cli specs behavior-coverage validate runs with the runtime cross-check
      Then the gate fails and names the scenario as marked-but-not-executed
      And the gate passes only when every @covers scenario executed and passed at each declared level
    ```

- [ ] [AI] **GREEN**: implement the runtime cross-check in a new sibling file
      `apps/rhino-cli/src/application/behavior_coverage/runtime_check.rs` (declared via
      `pub mod runtime_check;` in `apps/rhino-cli/src/application/behavior_coverage/mod.rs`) — ingest each
      tier's JSON run report and assert each `@covers` scenario executed AND passed at its level. Command:
      same. Acceptance: new test passes; existing suite green.
- [ ] [AI] **REFACTOR**: factor the per-tier report parsers behind one trait in
      `apps/rhino-cli/src/application/behavior_coverage/runtime_check.rs`. Command: same. Acceptance: all
      green.
- [ ] [AI] Regenerate the golden-master: `cargo test --release -p rhino-cli --test golden_master` (per
      `enforce-identical-rhino-cli-gherkin/delivery.md` §"1i. Regenerate golden-master"); review the diff
      for intent before freezing. Propagate the byte-identical `apps/rhino-cli/` to ose-primer and
      ose-infra using the dependency plan's exact Phase 3/Phase 4 commands. Command (ose-primer):
      `rsync -a --delete --exclude=target --exclude=dist --exclude=cover.out --exclude=lcov.info
/Users/wkf/ose-projects/ose-public/apps/rhino-cli/ /Users/wkf/ose-projects/ose-primer/apps/rhino-cli/`.
      Command (ose-infra):
      `rsync -a --delete --exclude=target --exclude=dist --exclude=cover.out --exclude=lcov.info
/Users/wkf/ose-projects/ose-public/apps/rhino-cli/ /Users/wkf/ose-projects/ose-infra/apps/rhino-cli/`.
      Acceptance: `diff -rq --exclude=target --exclude=dist apps/rhino-cli
../ose-primer/apps/rhino-cli` and the equivalent comparison against `ose-infra` show only
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
- [ ] [AI] Playwright: `forbidOnly: !!process.env.CI` is already set in all 11
      `apps/*-e2e/playwright.config.ts` — add only the missing `test.skip` guard/reporter to each.
      Verify by planting a skip — e2e tier reddens; revert. Acceptance: skip fails the tier.
- [ ] [AI] F#: xunit v3 (all four F# test surfaces —
      `apps/organiclever-be/tests/{unit,integration}/*.fsproj`,
      `apps/ose-be/tests/{unit,integration}/*.fsproj`, `apps/crane-cli/tests/{unit,integration}/*.fsproj`,
      `libs/fsharp-crane-core/tests/unit/*.fsproj`) has no CI-forbid-only equivalent, so add a fail-on-skip
      guard as a grep check for the xunit `Skip =` attribute: `grep -rn 'Skip\s*=' apps/organiclever-be/tests
apps/ose-be/tests apps/crane-cli/tests libs/fsharp-crane-core/tests` must return 0 matches, wired into
      each project's test target per `audit/05-reporters.md`. Verify by planting `[Fact(Skip = "temp")]` in
      one test file — the grep check catches it and fails the tier; revert. Acceptance: ignored test fails
      the tier.
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
      level, for every scenario whose behaviour **already exists** (marker-only path — no TDD cycle
      needed since the test is added against passing production code). Command:
      `nx run <project>:test:unit` (+`:test:integration`/`:test:e2e` as applicable) then
      `nx run <project>:specs:behavior:coverage`. Acceptance: cross-check passes for these scenarios.
- [ ] [AI] For every scenario the runtime cross-check reveals as **unimplemented** (behaviour missing, not
      merely untested), run a full TDD cycle instead of the marker-only path above:
  - [ ] [AI] **Conditional UI-design-funnel**: if this project is `ose-www`, `ose-app-web`,
        `organiclever-www`, `organiclever-app-web`, or one of their `-e2e` counterparts AND the missing
        behaviour requires building a genuinely new user-facing screen or component (not merely new
        backend/CLI logic behind an existing screen), run the UI-design-funnel (diverge → narrow → select
        → justify, per the
        [UI Mockups in Plan Docs convention](../../../repo-governance/conventions/formatting/diagrams.md#ui-mockups-in-plan-docs))
        BEFORE the RED step below, recording it in this plan's `prd.md` and `assets/`: ≥2 named low-fi
        alternatives, 2 hi-fi `.excalidraw.png` finalists, an explicit selection + rationale, a stated
        mobile/tablet/desktop responsive strategy, an R5 grounding note (survey existing `libs/web-ui`
        and sibling screens; name any net-new component), and an R7 prior-art citation (a `web-researcher`
        survey of comparable tools). Not applicable when the marker-only path (the earlier
        `@covers`-marker checkbox) was used instead, or when the missing behaviour reuses an existing
        screen/component with no net-new UI. Acceptance: the funnel record is committed in `prd.md` before
        RED is written, or this checkbox is ticked with an explicit one-line "N/A — <reason>" note.
  - [ ] [AI] **RED**: write the failing test for the scenario in the project's test suite at its declared
        level(s), tagged `// @covers <spec-path>:<scenario-title>`. Command:
        `nx run <project>:test:unit` (or the scenario's declared-level target). Acceptance: test fails,
        naming the missing behaviour.
  - [ ] [AI] **GREEN**: implement the minimum production code in the project's source to make the test
        pass. Command: same. Acceptance: test passes; no other tests broken.
  - [ ] [AI] **REFACTOR**: clean up the new implementation and test. Command: same, then
        `nx run <project>:specs:behavior:coverage`. Acceptance: all green, cross-check passes — every
        scenario executed and passed at its levels; zero silent skips.
  - [ ] [AI] **Conditional Rule-15/16 retest**: if this project is `ose-www`, `ose-app-web`,
        `organiclever-www`, `organiclever-app-web`, or one of their `-e2e` counterparts AND the behaviour
        just built above is genuinely new user-facing UI behaviour, run the Rule-15 three-tester retest
        (`web-exploratory-tester` + `web-usability-tester` + `web-design-tester` via the
        `web-ux-test-fixing-planning` workflow, `output-mode: delivery`, this plan's `plan-path`) against
        the running app before this batch's gate passes; fix every `EWT-###`/`UWT-###`/`DWT-###` defect
        finding. If this project is `ose-be` or `organiclever-be` AND the behaviour just built
        exposes/changes a REST or GraphQL endpoint, run `api-exploratory-tester` instead
        (`output-mode: delivery`, this plan's `plan-path`) and fix every `AET-###` defect finding. Not
        applicable when the marker-only path (the earlier `@covers`-marker checkbox) was used instead (no
        behaviour change) or when the built behaviour has no UI/API surface (e.g. a pure lib). Acceptance:
        retest ran and every defect finding is fixed and ticked, or this checkbox is ticked with an
        explicit one-line "N/A — <reason>" note.

### Phase N Gate (each batch)

- [ ] [AI] `nx run <project>:specs:behavior:coverage` — exit 0, non-vacuous (markers present).
- [ ] [AI] `nx affected -t test:quick,specs:behavior:coverage --base=origin/main` — exits 0.
- [ ] [AI] **Zero deferrals**: the project has no `@wip`, no `.skip`/`.only`/`.todo`, no
      marker-without-a-real-test — every scenario executed and passed (`grep`-proof recorded in
      `audit/07-no-defer-proof.md`).
- [ ] [AI] **Conditional Rule-15/16 gate**: if this batch's no-defer TDD path built new user-facing UI
      behaviour in a UI-bearing project (`ose-www`, `ose-app-web`, `organiclever-www`,
      `organiclever-app-web`, or their `-e2e` counterparts), the Rule-15 three-tester retest ran and every
      `EWT-###`/`UWT-###`/`DWT-###` defect finding is fixed and ticked; if it built/changed a REST or
      GraphQL endpoint (`ose-be`, `organiclever-be`), the Rule-16 `api-exploratory-tester` retest ran and
      every `AET-###` defect finding is fixed and ticked. N/A otherwise (marker-only batch, or no UI/API
      surface touched).
- [ ] [AI] **Conditional UI-design-funnel gate**: if this batch's no-defer TDD path built a genuinely new
      user-facing screen or component in a UI-bearing project (`ose-www`, `ose-app-web`,
      `organiclever-www`, `organiclever-app-web`, or their `-e2e` counterparts), the UI-design-funnel
      record (diverge/narrow/select/justify + responsive strategy) is committed in `prd.md` and predates
      the RED step for that scenario. N/A otherwise (marker-only batch, no net-new UI, or existing-screen
      reuse).

> **Pause Safety**: the completed batches are fully enforced; remaining projects are untouched and still
> pass their existing gates. Safe to stop between batches. To resume: pick the next batch.

---

## Final Phase — Wire, Cross-Repo Verify & Archival

### Local Quality Gates (Before Push)

> **Important**: Fix ALL failures found during quality gates, not just those caused by your changes.

- [ ] [AI] Verify the runtime cross-check runs for every affected project — **no file edit is needed**:
      Phase 1's engine change lands inside `apps/rhino-cli/src/application/behavior_coverage/`, and every
      project's `specs:behavior:coverage` Nx target already invokes the rhino-cli binary directly, so the
      cross-check propagates automatically via the existing chain — pre-push:
      `.husky/pre-push` runs `nx affected -t test:quick`, whose `dependsOn`/command chain
      (`test:quick` → `test:specs` → `specs:behavior:coverage`, confirmed in
      `apps/organiclever-be/project.json`) already reaches it; CI: `.github/workflows/main-ci.yml` and
      `.github/workflows/pr-quality-gate.yml` already run
      `nx run-many`/`nx affected -t … specs:behavior:coverage` directly. Acceptance: plant a
      marked-but-skipped scenario in any eligible project, confirm it fails both
      `nx affected -t test:quick` and the CI `specs:behavior:coverage` step, then revert the plant.
- [ ] [AI] Per repo: `nx run-many --all -t typecheck,lint,test:quick,specs:behavior:coverage` — exits 0,
      non-vacuous, zero silent skips.
- [ ] [AI] Cross-repo: `apps/rhino-cli` (engine) byte-identical across the three repos.

### Commit Guidelines

- [ ] [AI] Commit thematically, explicit paths only (never `git add -A`). Split: engine
      (`feat(rhino-cli): behavior-coverage runtime cross-check`), per-tier config
      (`test: fail-on-skip across tiers`), per-project rollout (`test(<project>): @covers + level tags`).

### Post-Push Verification

- [ ] [AI] Push each repo → `origin main`; monitor CI (poll every 2 min, one `gh run view` per wakeup);
      verify green; fix any failure before proceeding.

> Manual UI/API verification, Rule-15 web-triad, Rule-16 API retest: **conditionally applicable**. Most
> batches only add `@covers` markers/level tags to already-passing tests (no behaviour change) and remain
> exempt. **If** a Phase 3..N batch's no-defer TDD path (Decision 4) built genuinely new user-facing
> behaviour to satisfy a previously-unimplemented scenario in a UI-bearing project (`ose-www`,
> `ose-app-web`, `organiclever-www`, `organiclever-app-web`, or their `-e2e` counterparts), that batch's
> Phase N Gate required the Rule-15 three-tester retest before being marked done (see the "Conditional
> Rule-15/16 retest" checkbox in Phase 3..N). If the built behaviour instead exposed/changed a REST or
> GraphQL endpoint (`ose-be`, `organiclever-be`), that batch's gate required the Rule-16
> `api-exploratory-tester` retest instead.
>
> The UI-design-funnel is **conditionally applicable** on the same basis: most batches remain exempt, but
> if that same no-defer TDD path built a genuinely new user-facing screen or component (not merely new
> backend/CLI logic behind an existing screen), that batch's Phase N Gate required the funnel record
> (diverge/narrow/select/justify + responsive strategy) committed in `prd.md` before RED, predating the
> RED step for that scenario (see the "Conditional UI-design-funnel" checkbox and gate in Phase 3..N).

### Final Gate

- [ ] [AI] Every eligible project: `specs:behavior:coverage` non-vacuous + runtime cross-check green;
      every tier fails on skip; all three repos' CI green.

> **Pause Safety**: repo-wide enforcement live and honest; nothing half-applied. Safe to stop. To resume:
> re-run `nx run-many --all -t specs:behavior:coverage`.

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
