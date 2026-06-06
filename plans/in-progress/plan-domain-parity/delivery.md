# Delivery Checklist — Plan Domain Parity (ose-public)

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.
>
> **Phase Gate** — every phase ends with a `### Phase N Gate` (must-pass verification) plus a
> `> **Pause Safety**:` note (the safe-to-stop state and the single command to resume). A phase
> is not complete until its gate is green; do not start phase N+1 while any gate check fails.

## Worktree

Worktree path: `worktrees/plan-domain-parity/`

This plan was authored in that worktree and is executed in it. The worktree already exists
(branch `plan-domain-parity` cut from `main`; remote `origin` =
`git@github.com:wahidyankf/ose-public.git` `[Repo-grounded]`). Push target: `origin main`.

Provision before execution if absent (run from repo root):

```bash
claude --worktree plan-domain-parity
```

Equivalent manual provisioning (the merged plan-establishment default, matrix row 3):

```bash
git worktree add -b plan-domain-parity worktrees/plan-domain-parity main
cd worktrees/plan-domain-parity && npm install && npm run doctor -- --fix
```

See the [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md),
[Worktree Toolchain Initialization](../../../repo-governance/development/workflow/worktree-setup.md),
and [Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

## Git Workflow

Trunk Based Development, worktree-to-main: thematic Conventional Commits inside the
worktree; one delivery push `git push origin HEAD:main` in Phase 7; **no PR** (no explicit
PR instruction exists). Worktree removed after archival.

### Commit Guidelines (apply in every phase)

> **Commit Policy**: Commit thematically with `<type>(<scope>): <description>` Conventional
> Commits format. Split different domains/concerns into separate commits (docs merges ≠
> rhino-cli code ≠ regenerated mirrors). Preexisting fixes get their own commits, separate
> from plan work. Never bundle unrelated changes into a single commit.
>
> **Important**: Fix ALL failures found during quality gates, not just those caused by your
> changes. This follows the root cause orientation principle — proactively fix preexisting
> errors encountered during work. Do not defer or skip existing issues. Commit preexisting
> fixes separately with appropriate conventional commit messages.

## Phase 0: Environment Setup and Baseline

> _Executor: repo-setup-manager_

- [ ] [AI] Verify the worktree exists: `git -C worktrees/plan-domain-parity status` (run
      from the main checkout root) — acceptance: exits 0 on branch `plan-domain-parity`.
      If absent, provision per the `## Worktree` section above.
- [ ] [AI] Install dependencies in the worktree: `npm install` (run inside
      `worktrees/plan-domain-parity/`) — acceptance: exits 0, `node_modules/` synchronized
- [ ] [AI] Converge the full polyglot toolchain: `npm run doctor -- --fix` — acceptance:
      exits 0 with no unresolved drift (Rust toolchain available for `apps/rhino-cli`)
- [ ] [AI] Verify sibling merge inputs are readable:
      `test -d /Users/wkf/ose-projects/ose-primer/repo-governance/workflows/plan && test -d /Users/wkf/ose-projects/ose-infra/repo-governance/workflows/plan`
      — acceptance: exits 0
- [ ] [AI] Run the rhino-cli baseline: `npx nx run rhino-cli:test:quick` — acceptance:
      baseline pass/fail count recorded in implementation notes; all preexisting failures
      documented
- [ ] [AI] Run the markdown baseline: `npm run lint:md` and
      `npx nx run rhino-cli:validate:links` — acceptance: exit codes recorded; preexisting
      failures documented
- [ ] [AI] Resolve all preexisting failures before proceeding — acceptance: no preexisting
      failures remain unresolved (separate commits per the guidelines above)

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [ ] [AI] `npm install` exited 0 and `npm run doctor -- --fix` reports no unresolved drift
- [ ] [AI] `npx nx run rhino-cli:test:quick`, `npm run lint:md`, and
      `npx nx run rhino-cli:validate:links` baselines recorded and every preexisting failure
      resolved (zero unresolved)

> **Pause Safety**: only the local toolchain was verified and the baseline recorded — no
> parity work exists yet. Safe to stop indefinitely. To resume: re-run
> `npx nx run rhino-cli:test:quick` and confirm it is still green.

## Phase 1: Plan-Domain Workflow Merges (matrix rows 2–6)

> _Suggested executor: `repo-workflow-maker` (workflow docs); merge-input paths below are
> the same relative path under `/Users/wkf/ose-projects/ose-primer/` and
> `/Users/wkf/ose-projects/ose-infra/` unless noted._

- [ ] [AI] Merge `repo-governance/workflows/plan/plan-establishment-execution.md` (row 3):
      produce 3-way diffs first —
      `diff repo-governance/workflows/plan/plan-establishment-execution.md /Users/wkf/ose-projects/ose-primer/repo-governance/workflows/plan/plan-establishment-execution.md`
      and the same against
      `/Users/wkf/ose-projects/ose-infra/repo-governance/workflows/plan/plan-establishment-execution.md`;
      fold every sibling improvement into the public copy; keep the `target-stage` input —
      acceptance: each sibling-only improvement is merged or recorded as deliberately
      excluded (with reason) in implementation notes; `grep -c "target-stage"` on the file
      returns ≥ 1.
- [ ] [AI] Add the new worktree default to the merged file (row 3, per tech-docs D2): amend
      `## Execution Mode`, `### 4. Plan Creation (Sequential)`, and
      `### 7. Push and Verify (Sequential)` to document — author in `worktrees/<identifier>/`;
      provision if absent via `git worktree add -b <identifier> worktrees/<identifier> main` + `npm install` + `npm run doctor -- --fix`; commit in worktree; push `HEAD` to the
      confirmed push target (default `origin main`); remove the worktree after delivery —
      acceptance: `grep -F "git worktree add -b" repo-governance/workflows/plan/plan-establishment-execution.md`
      returns ≥ 1 hit and the push-target default is stated.
- [ ] [AI] Merge `repo-governance/workflows/plan/plan-execution.md` (row 4) using the same
      3-way diff procedure — acceptance: public-specific agent-selection lists preserved
      verbatim; sibling improvements merged or recorded as excluded.
- [ ] [AI] Merge `repo-governance/workflows/meta/execution-modes.md` (row 6) using the same
      3-way diff procedure — acceptance: sibling improvements merged or recorded as
      excluded; file passes the markdown gates below.
- [ ] [AI] Restructure `repo-governance/workflows/plan/plan-multi-repo-parity-planning.md`
      (row 2): the `## Steps` section becomes, in order — Step 1 Survey; Step 2 Matrix;
      Step 3 First Grill (hard gate, blocks authoring until every matrix row is resolved);
      Step 4 Web Research via `web-research-maker` (conditional); Step 5 Second Grill
      (post-research); Step 6 Author; Step 7 Gate; Step 8 Deliver (absorbing the current
      Step 7 Finalization content). Update the `## Grilling Contract`,
      `## Termination Criteria`, and `## Sibling Plans` cross-references to the renumbered
      steps — acceptance: the eight step headings appear in the stated order;
      `npx nx run rhino-cli:validate:links` exits 0 (no broken intra-file fragments).
- [ ] [AI] Align `repo-governance/workflows/plan/README.md` (row 5): verify all four plan
      workflows remain indexed (establishment, execution, parity, quality-gate) and refresh
      descriptions to match the merged/restructured content — acceptance: four workflow
      links present; descriptions mention the two-grill parity structure.
- [ ] [AI] Refresh the plan-domain rows in `repo-governance/workflows/README.md` if step
      naming or descriptions changed — acceptance: no stale step names remain
      (`grep -n "Relentless Grilling" repo-governance/workflows/README.md` returns 0 hits or
      only deliberate historical mentions).
- [ ] [AI] Run the docs gates: `npm run format:md`, `npm run lint:md`,
      `npx nx run rhino-cli:validate:links`,
      `npx nx run rhino-cli:validate:heading-hierarchy`,
      `npx nx run rhino-cli:validate:mermaid` — acceptance: all exit 0.
- [ ] [AI] Commit: `docs(workflows): merge plan-domain workflow canon and restructure parity workflow` —
      acceptance: commit exists; `git status` clean for the workflow files.

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [ ] [AI] `grep -F "git worktree add -b" repo-governance/workflows/plan/plan-establishment-execution.md` — ≥ 1 hit
- [ ] [AI] `grep -c "target-stage" repo-governance/workflows/plan/plan-establishment-execution.md` — ≥ 1
- [ ] [AI] `grep -n "^### Step [0-9]" repo-governance/workflows/plan/plan-multi-repo-parity-planning.md` — returns exactly 8 step headings in the order: Survey, Matrix, First Grill, Web Research, Second Grill, Author, Gate, Deliver
- [ ] [AI] `npm run lint:md && npx nx run rhino-cli:validate:links && npx nx run rhino-cli:validate:heading-hierarchy && npx nx run rhino-cli:validate:mermaid` — all exit 0

> **Pause Safety**: workflow docs are merged and committed; no agent, skill, or code files
> touched yet — the repo is coherent. Safe to stop. To resume: re-run
> `npm run lint:md` and confirm green.

## Phase 2: Plan-Agent Definition Merges (matrix rows 7–11)

> _Suggested executor: `agent-maker` (agent definition files)_

- [ ] [AI] Merge `.claude/agents/plan-maker.md` (row 7) via 3-way diff against
      `/Users/wkf/ose-projects/ose-primer/.claude/agents/plan-maker.md` and
      `/Users/wkf/ose-projects/ose-infra/.claude/agents/plan-maker.md` — acceptance:
      sibling improvements merged or recorded as excluded; repo-specific references (app
      names, paths) preserved.
- [ ] [AI] Merge `.claude/agents/plan-checker.md` (row 8), same procedure — acceptance: same
      criteria.
- [ ] [AI] Merge `.claude/agents/plan-fixer.md` (row 9), same procedure — acceptance: same
      criteria.
- [ ] [AI] Merge `.claude/agents/plan-execution-checker.md` (row 10), same procedure —
      acceptance: same criteria.
- [ ] [AI] Verify `.claude/agents/repo-setup-manager.md` (row 11):
      `diff .claude/agents/repo-setup-manager.md /Users/wkf/ose-projects/ose-infra/.claude/agents/repo-setup-manager.md`
      — acceptance: zero changed lines pub↔infra (survey fact); primer's 3-line drift is
      `rhino-cli-rust` naming (repo-specific, primer-plan concern) — record the verification
      result in implementation notes; no public edit expected.
- [ ] [AI] Regenerate the four touched OpenCode mirrors: `npm run generate:bindings` —
      acceptance: exits 0; `.opencode/agents/plan-{maker,checker,fixer}.md` and
      `.opencode/agents/plan-execution-checker.md` updated.
- [ ] [AI] Validate mirror parity: `npm run validate:sync` — acceptance: exits 0.
- [ ] [AI] Run the docs gates (same five commands as Phase 1) — acceptance: all exit 0.
- [ ] [AI] Commit in two parts: `docs(agents): merge plan-domain agent canon` (hand-edited
      `.claude/agents/`) and `chore(bindings): resync opencode mirrors` (generated files) —
      acceptance: both commits exist; `git status` clean.

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [ ] [AI] `npm run validate:sync` — exits 0
- [ ] [AI] `npm run lint:md && npx nx run rhino-cli:validate:links` — exit 0
- [ ] [AI] `grep -c "implementation notes\|deliberately excluded\|merged or recorded" plans/in-progress/plan-domain-parity/delivery.md` — ≥ 4 hits (confirms merge/exclude rationale is recorded inline for each of the four agents; also read the Phase 2 implementation notes for plan-maker, plan-checker, plan-fixer, and plan-execution-checker to confirm each has a recorded merge/exclude decision)

> **Pause Safety**: agent canon merged, mirrors in sync, all committed. Safe to stop. To
> resume: re-run `npm run validate:sync` and confirm green.

## Phase 3: Skill and Convention Merges (matrix rows 12–16)

> _Suggested executor: `repo-rules-maker` (conventions); `agent-maker` (skills)_

- [ ] [AI] Merge `.claude/skills/plan-creating-project-plans/SKILL.md` (row 12) via 3-way
      diff (siblings at the same relative path) — acceptance: infra's mandatory pre-write
      AND post-write grilling gates present in the merged text; the 2–4-options hard rule
      stated; sibling improvements merged or recorded as excluded.
- [ ] [AI] Merge `.claude/skills/plan-writing-gherkin-criteria/SKILL.md` (row 13, trivial
      2–10 line drift) — acceptance: merged or recorded; gates pass.
- [ ] [AI] Merge `.claude/skills/grill-me/SKILL.md` (row 14) — acceptance: merged or
      recorded; the one-question-at-a-time and 2–4-options rules retained.
- [ ] [AI] Merge `repo-governance/development/workflow/grilling-with-options.md` (row 15):
      3-way inputs are the public file, primer **none** (no input), and infra
      `/Users/wkf/ose-projects/ose-infra/repo-governance/development/workflow/grilling.md`
      (different name, broader wording); fold infra's broader wording into the public file;
      the public path and name are kept — acceptance: merged file remains at
      `repo-governance/development/workflow/grilling-with-options.md`; infra-only
      improvements present or recorded as excluded.
- [ ] [AI] Merge `repo-governance/conventions/structure/plans.md` (row 16) via 3-way diff —
      acceptance: sibling improvements merged or recorded; Worktree-Specification,
      Executor-Tagging, Phase-Gate, and Execution-Grade-Clarity sections intact.
- [ ] [AI] Run the docs gates (same five commands as Phase 1) — acceptance: all exit 0.
- [ ] [AI] Commit: `docs(governance): merge plan-domain skills and conventions canon` —
      acceptance: commit exists; `git status` clean.

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [ ] [AI] Merged skill contains both grilling gates:
      `grep -ci "post-write" .claude/skills/plan-creating-project-plans/SKILL.md` — ≥ 1
- [ ] [AI] `test -f repo-governance/development/workflow/grilling-with-options.md` — exits 0
- [ ] [AI] `npm run lint:md && npx nx run rhino-cli:validate:links && npx nx run rhino-cli:validate:heading-hierarchy` — exit 0

> **Pause Safety**: all fourteen doc merges (rows 2–16) are complete and committed; code
> streams untouched. Safe to stop. To resume: re-run `npm run lint:md` and confirm green.

## Phase 4: rhino-cli OpenCode Permission Emitter (matrix row 18, TDD)

> _Suggested executor: `swe-rust-dev`_

- [ ] [AI] **RED** — add failing unit tests to the inline `#[cfg(test)]` module of
      `apps/rhino-cli/src/internal/agents/converter.rs` (_New tests_):
      `convert_permission_maps_tools_to_allow` (input `["Read", "Write"]` → map
      `{read: "allow", write: "allow"}`) and `encode_emits_permission_block_not_tools`
      (encoded YAML contains a `permission:` block with `read: allow` and contains no
      boolean `tools:` map). Run
      `cargo test --manifest-path apps/rhino-cli/Cargo.toml convert_permission` —
      acceptance: the new tests FAIL (compile error or assertion failure) proving RED.
- [ ] [AI] **GREEN** — implement in `apps/rhino-cli/src/internal/agents/converter.rs`:
      rename the `OpenCodeAgent.tools: BTreeMap<String, bool>` field to
      `permission: BTreeMap<String, String>`; replace `convert_tools` with
      `convert_permission` mapping each trimmed, lower-cased, non-empty Claude tool to the
      value `allow` (unlisted tools omitted per tech-docs D3); update
      `encode_opencode_agent` to emit `permission:` in the position `tools:` occupied
      (empty input emits `permission: {}`); update the field-order doc comments and all
      existing tests referencing `tools`. Run
      `npx nx run rhino-cli:test:unit` — acceptance: exits 0, including the two new tests.
- [ ] [AI] **REFACTOR** — clean up naming/doc comments; run
      `npx nx run rhino-cli:lint` and `npx nx run rhino-cli:fmt:check` — acceptance: both
      exit 0 with no behavioral diff (`npx nx run rhino-cli:test:unit` still green).
- [ ] [AI] Regenerate all mirrors: `npm run generate:bindings` — acceptance: exits 0; spot
      check `head -15 .opencode/agents/plan-maker.md` shows a `permission:` block and no
      boolean `tools:` map.
- [ ] [AI] Sweep for stragglers:
      `grep -rln "^tools:" .opencode/agents/` — acceptance: 0 files.
- [ ] [AI] Validate parity: `npm run validate:sync` — acceptance: exits 0.
- [ ] [AI] Commit in two parts: `feat(rhino-cli): emit opencode permission object instead of deprecated tools flags`
      (code + tests) and `chore(bindings): regenerate opencode mirrors in permission format`
      (the ~70 regenerated files) — acceptance: both commits exist; `git status` clean.

### Phase 4 Gate

> All checks below must pass before starting Phase 5.

- [ ] [AI] `npx nx run rhino-cli:test:quick` — exits 0
- [ ] [AI] `npm run validate:sync` — exits 0
- [ ] [AI] `grep -rln "^tools:" .opencode/agents/` — 0 files
- [ ] [AI] `ls .claude/agents/*.md | wc -l` equals `ls .opencode/agents/*.md | wc -l`

> **Pause Safety**: emitter and all mirrors moved to the `permission` format atomically and
> are committed; validator and emitter share the converter so parity holds. Safe to stop. To
> resume: re-run `npm run validate:sync` and confirm green.

## Phase 5: Codex Consolidation and Guard (matrix row 19, TDD)

> _Suggested executor: `swe-rust-dev` (guard); main context (config migration)_

- [ ] [AI] Verify sub-table key support (tech-docs D4): single WebFetch of
      <https://developers.openai.com/codex/config-reference>; determine whether
      `developer_instructions` may be inlined in `[agents.<name>]` — acceptance: the
      decision (inline vs relocated `config_file`) recorded in implementation notes with the
      cited excerpt and access date.
- [ ] [AI] Migrate `.codex/config.toml`: per the D4 decision, either inline the
      `developer_instructions` content from `.codex/agents/ci-monitor-subagent.toml` into
      `[agents.ci-monitor-subagent]`, or move that file to
      `.codex/ci-monitor-subagent.toml` and update `config_file` accordingly — acceptance:
      `python3 -c "import tomllib; tomllib.load(open('.codex/config.toml','rb'))"` exits 0
      (valid TOML) and the sub-table carries the agent config; pre/post content diff shows
      no instruction text lost.
- [ ] [AI] Remove the unofficial directory: `git rm -r .codex/agents/` — acceptance:
      `test ! -d .codex/agents` exits 0.
- [ ] [AI] **RED** — add a failing unit test to the inline `#[cfg(test)]` module of
      `apps/rhino-cli/src/internal/agents/bindings.rs` (_New test_):
      `validate_fails_when_codex_agents_dir_exists` — in a tempdir with valid bridge files
      and full catalog, create `.codex/agents/` and assert `validate_bindings` reports a
      failed check whose advice mentions `config.toml` sub-tables. Run
      `cargo test --manifest-path apps/rhino-cli/Cargo.toml validate_fails_when_codex_agents_dir_exists`
      — acceptance: test FAILS (no such check yet), proving RED.
- [ ] [AI] **GREEN** — implement in `apps/rhino-cli/src/internal/agents/bindings.rs`: add a
      check to `validate_bindings` (alongside the catalog-coverage checks) that fails when
      `<repo_root>/.codex/agents` exists, with advice text
      "migrate per-agent Codex config to .codex/config.toml agents.<name> sub-tables". Run
      `npx nx run rhino-cli:test:unit` — acceptance: exits 0 including the new test, and the
      existing test `validate_passes_when_catalog_references_all_present_dirs` is updated if
      it materializes `.codex/agents` (it currently creates only `.codex/`
      `[Repo-grounded]`).
- [ ] [AI] **REFACTOR** — tidy check naming/messages; `npx nx run rhino-cli:lint` and
      `npx nx run rhino-cli:fmt:check` — acceptance: both exit 0;
      `npx nx run rhino-cli:test:unit` still green.
- [ ] [AI] Run the guard end-to-end: `npm run validate:harness-bindings` — acceptance:
      exits 0 against the migrated repo (no `.codex/agents/`).
- [ ] [AI] Commit in two parts:
      `feat(rhino-cli): guard against unofficial .codex/agents directory` and
      `chore(codex): consolidate per-agent config into config.toml sub-tables` —
      acceptance: both commits exist; `git status` clean.

### Phase 5 Gate

> All checks below must pass before starting Phase 6.

- [ ] [AI] `test ! -d .codex/agents` — exits 0
- [ ] [AI] `npx nx run rhino-cli:test:quick` — exits 0
- [ ] [AI] `npm run validate:harness-bindings` — exits 0

> **Pause Safety**: Codex surface consolidated, guard active, all committed; OpenCode and
> docs streams already coherent from earlier gates. Safe to stop. To resume: re-run
> `npm run validate:harness-bindings` and confirm green.

## Phase 6: Full Binding Audit and Harness-Doc Updates (matrix rows 17, 20)

- [ ] [AI] Final regeneration: `npm run generate:bindings` then `git status --short` —
      acceptance: exits 0 and reports no unexpected drift (idempotent).
- [ ] [AI] Audit agent×binding coverage: `ls .claude/agents/*.md | wc -l` vs
      `ls .opencode/agents/*.md | wc -l` — acceptance: equal counts (70/70 at authoring
      time `[Repo-grounded]`; equality is the criterion, not the literal number).
- [ ] [AI] Run the full validation set: `npm run validate:sync`,
      `npm run validate:harness-bindings`, and
      `npx nx run rhino-cli:validate:cross-vendor-parity` — acceptance: all exit 0.
- [ ] [AI] Verify row 20 (no change needed):
      `grep -F "cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml" package.json`
      — acceptance: ≥ 1 hit in the `generate:bindings` script; record in implementation
      notes that ose-public already matches the aligned invocation.
- [ ] [AI] Update `CLAUDE.md`: rewrite the OpenCode format bullet ("OpenCode uses boolean
      flags `{ read: true, write: true }`") to describe the `permission` object as current
      and the boolean form as deprecated/legacy — acceptance:
      `grep -n "permission" CLAUDE.md` shows the new wording in the multi-harness section.
- [ ] [AI] Update `repo-governance/development/agents/ai-agents.md` (3 known hits at lines
      ~73, ~2571, ~2619 `[Repo-grounded]`): same deprecated-form reframing for tool-format
      descriptions and the Platform Binding translation sections — acceptance: a repo-wide
      `grep -rn "boolean flags" repo-governance/ AGENTS.md CLAUDE.md docs/ --include="*.md"`
      shows every remaining hit framed as deprecated/legacy/historical.
- [ ] [AI] Update `docs/reference/platform-bindings.md`: Codex row (line ~31) drops the
      `config_file` pointer into `.codex/agents/<name>.toml`; the `.codex/agents/`
      provenance note (line ~70) is rewritten to record the directory's removal; OpenCode
      row/format wording mentions the `permission` object — acceptance:
      `grep -n ".codex/agents" docs/reference/platform-bindings.md` returns only
      removal/historical framing (or zero hits).
- [ ] [AI] Update `repo-governance/conventions/structure/multi-harness-binding.md`: sweep
      for boolean-tools and `.codex/agents/` references; reframe per the new canon —
      acceptance: same grep criteria as above applied to this file.
- [ ] [AI] Repo-wide stale-reference sweep:
      `grep -rn ".codex/agents" --include="*.md" . | grep -v "plans/done\|archived\|node_modules\|local-temp\|worktrees\|plan-domain-parity"`
      — acceptance: every remaining hit is deliberate historical/removal framing; fix any
      that present `.codex/agents/` as a live config surface.
- [ ] [AI] Run the docs gates (same five commands as Phase 1) — acceptance: all exit 0.
- [ ] [AI] Commit: `docs(governance): update harness binding docs for permission format and codex consolidation` —
      acceptance: commit exists; `git status` clean.

### Phase 6 Gate

> All checks below must pass before starting Phase 7.

- [ ] [AI] `npm run validate:sync && npm run validate:harness-bindings` — exit 0
- [ ] [AI] `npx nx run rhino-cli:validate:cross-vendor-parity` — exits 0
- [ ] [AI] `grep -rn "boolean flags" repo-governance/ AGENTS.md CLAUDE.md docs/ --include="*.md"` — every hit framed as deprecated/legacy/historical; AND `grep -rn ".codex/agents" --include="*.md" . | grep -v "plans/done\|archived\|node_modules\|local-temp\|worktrees\|plan-domain-parity"` — every remaining hit is removal/historical framing
- [ ] [AI] `npm run lint:md && npx nx run rhino-cli:validate:links` — exit 0

> **Pause Safety**: every binding surface is regenerated, audited, and documented; the repo
> tells one consistent story. Safe to stop. To resume: re-run
> `npm run validate:harness-bindings` and confirm green.

## Phase 7: Rationale Doc, Final Gates, Push, and Archival

- [ ] [AI] Create `docs/explanation/plan-domain-parity-decisions.md` (_New file_) explaining
      all 26 matrix rows in plain language — what was decided, why, and what was rejected —
      with dedicated subsections for the deviations: row 19 (including the ose-public nuance
      that rhino-cli never emitted `.codex/agents/`, per tech-docs D5), row 22 (primer
      direct-push deviation), row 23 (primer plan supersession), row 26 (drift guard
      deliberately dropped) — acceptance: all 26 rows covered (one heading or list entry
      each); file passes the docs gates.
  - _Suggested executor: `docs-maker`_
- [ ] [AI] Index it: add the rationale doc to `docs/explanation/README.md` — acceptance:
      link present and `npx nx run rhino-cli:validate:links` exits 0.
- [ ] [AI] Commit: `docs(explanation): add plan-domain-parity decision rationale` —
      acceptance: commit exists.

### Local Quality Gates (Before Push)

- [ ] [AI] Run affected typecheck: `npx nx affected -t typecheck` — exits 0
- [ ] [AI] Run affected linting: `npx nx affected -t lint` — exits 0
- [ ] [AI] Run affected quick tests: `npx nx affected -t test:quick` — exits 0
- [ ] [AI] Run affected spec coverage: `npx nx affected -t spec-coverage` — exits 0
- [ ] [AI] Run markdown gates: `npm run lint:md`, `npx nx run rhino-cli:validate:links`,
      `npx nx run rhino-cli:validate:heading-hierarchy`,
      `npx nx run rhino-cli:validate:mermaid` — all exit 0
- [ ] [AI] Fix ALL failures — including preexisting issues not caused by these changes
      (separate commits) — acceptance: zero failures remain
- [ ] [AI] Re-run any previously failing checks to confirm resolution — acceptance: green

### Post-Push CI Verification

- [ ] [AI] Push from the worktree: `git push origin HEAD:main` — acceptance: push accepted
- [ ] [AI] Monitor ALL GitHub Actions workflows triggered by the push — poll with
      `gh run list`/`gh run view --json status,conclusion` every 3 minutes per the
      [CI Monitoring Convention](../../../repo-governance/development/workflow/ci-monitoring.md)
      (never `gh run watch`) — acceptance: every triggered workflow concludes `success`
- [ ] [AI] If any CI check fails, fix immediately, commit, and push a follow-up — repeat
      until ALL GitHub Actions pass with zero failures (strict double-zero bar, matrix
      row 25)
- [ ] [AI] Do NOT archive until CI is fully green

### Plan Archival

- [ ] [AI] Verify ALL delivery checklist items are ticked
- [ ] [AI] Verify ALL quality gates pass (local + CI)
- [ ] [AI] Rename and move:
      `git mv plans/in-progress/plan-domain-parity plans/done/YYYY-MM-DD__plan-domain-parity`
      using the actual completion date (NOT the creation date)
- [ ] [AI] Update `plans/in-progress/README.md` — remove this plan's entry
- [ ] [AI] Update `plans/done/README.md` — add this plan's entry with the completion date
- [ ] [AI] Update any other READMEs referencing this plan (e.g., `plans/README.md`)
- [ ] [AI] Commit the archival: `chore(plans): move plan-domain-parity to done` and push
      `git push origin HEAD:main`; re-verify CI green
- [ ] [AI] Remove the worktree (run from the main checkout root, after the archival push):
      `git worktree remove worktrees/plan-domain-parity` and
      `git branch -d plan-domain-parity` — acceptance: both exit 0

### Phase 7 Gate

> Final gate — the plan is done only when all checks pass.

- [ ] [AI] `gh run list --branch main --limit 20 --json status,conclusion --jq '.[] | select(.status == "completed") | .conclusion'` — all results are `success`; zero workflows show `failure` or `cancelled`
- [ ] [AI] `ls plans/done/ | grep plan-domain-parity` — shows exactly one entry with a `YYYY-MM-DD__plan-domain-parity` prefix; AND `grep -c "plan-domain-parity" plans/done/README.md` — ≥ 1 hit; AND `grep -c "plan-domain-parity" plans/in-progress/README.md` — 0 hits
- [ ] [AI] Worktree removed; `git worktree list` no longer shows `plan-domain-parity`

> **Pause Safety**: after this gate the parity canon is live on `main`, CI is green, and the
> plan is archived — terminal state. Sibling plans (primer, infra) may now execute their
> adoption work. To re-verify at any time: `npm run validate:harness-bindings` on `main`.
