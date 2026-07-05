# Delivery Checklist

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.

## Worktree

Worktree path: `worktrees/upgrade-opencode-go-models/`

Optional manual pre-provisioning (run from repo root):

```bash
claude --worktree upgrade-opencode-go-models
```

The plan-execution Step 0 gate enters this worktree by default: it auto-provisions from the latest
`origin/main` when missing, syncs with `origin/main` before implementing, and prompts before
deleting the worktree after the plan is archived and pushed. Phases 0-3 run inside this `ose-public`
worktree. Phase 4's `ose-primer`/`ose-infra` work runs directly in each of those repos' own `main`
trees (matching the precedent set by
[`enforce-repo-wide-scenario-implementation`](../../done/2026-07-04__enforce-repo-wide-scenario-implementation/delivery.md) —
this is a small, tightly-scoped cross-repo config/engine parity change, not a long-lived isolated
feature branch).

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

---

## Phase 0 — Environment Setup and Baseline

- [ ] [AI] Initialize the toolchain in the (freshly auto-provisioned) worktree root: run
      `npm install && npm run doctor -- --fix`. Acceptance: both commands exit 0 (per
      [Worktree Toolchain Initialization](../../../repo-governance/development/workflow/worktree-setup.md)
      — required before any `nx run` command below will work reliably).
- [ ] [AI] Confirm the live `opencode-go` roster is unchanged from this plan's research snapshot:
      run `opencode models | grep opencode-go`. Acceptance: output contains both `opencode-go/glm-5.2`
      and `opencode-go/minimax-m3`, and does NOT contain a bare `opencode-go/glm-5` (no suffix). If
      the roster has changed since 2026-07-05 (either target model retired, or a new model added
      that changes the rankings), STOP and re-run the `web-researcher` benchmark comparison from
      `tech-docs.md` against the new roster before proceeding — do not blindly continue with a stale
      target ID.
- [ ] [AI] Confirm clean git state in all 3 repos before starting: run `git status --short` in
      `/Users/wkf/ose-projects/ose-public`, `/Users/wkf/ose-projects/ose-primer`,
      `/Users/wkf/ose-projects/ose-infra`. Acceptance: all three print no output (clean working
      tree) and `git rev-list --left-right --count origin/main...HEAD` prints `0 0` in each.
- [ ] [AI] Investigate `ose-infra`'s `.opencode/opencode.json` provider divergence (Decision 3,
      `tech-docs.md`). Run `cd /Users/wkf/ose-projects/ose-infra && git log -p --follow -- .opencode/opencode.json | head -200`
      and read the commit message(s) that introduced `zai-coding-plan/*`. Acceptance: record in
      this checklist item's own completion note either (a) "no rationale found — proceeding with
      reconciliation to opencode-go/glm-5.2 + opencode-go/minimax-m3 per Decision 3's default" or
      (b) the specific rationale found, plus whether it still holds today.
- [ ] [AI] Baseline `ose-public`: run `nx run rhino-cli:test:quick`. Acceptance: passes cleanly
      (0 pre-existing failures) — this is the baseline the Phase 1 RED step will intentionally
      break.
- [ ] [AI] Re-confirm the docs-refresh file list from `tech-docs.md`'s File Impact tables is still
      accurate (Confirmed Decision 8, `README.md` — this repeats a check already done once during
      plan-authoring on 2026-07-05, guarding against further drift before execution touches these
      files). Run, in each of `ose-primer` and `ose-infra`:
      `grep -n "opencode-go/minimax-m2\.7\|opencode-go/glm-5\b" CLAUDE.md AGENTS.md repo-governance/development/agents/model-selection.md repo-governance/development/agents/ai-agents.md repo-governance/conventions/structure/governance-vendor-independence.md docs/reference/platform-bindings.md docs/reference/ai-model-benchmarks.md 2>/dev/null`.
      Acceptance: line numbers match `tech-docs.md`'s File Impact table (`ose-primer`:
      `CLAUDE.md:52`, `AGENTS.md:319`, `model-selection.md:269-272`, `ai-agents.md:66,155,2505-2506`,
      `governance-vendor-independence.md:167`, `platform-bindings.md:181-183`; `ose-infra`:
      `model-selection.md:262-265,268,272-273`, `platform-bindings.md:187-189`, no hits in the other
      4 files) — if a line number has shifted or a new hit/miss appears, update `tech-docs.md`'s File
      Impact tables before Phase 4 uses them.
- [ ] [AI] Confirm `pi` is not installed and no `.pi/` directory exists yet in `ose-public`: run
      `which pi; ls -la .pi/ 2>&1`. Acceptance: `which pi` prints nothing (or "not found") and `ls`
      reports "No such file or directory" — confirms Phase 2's `.pi/settings.json` step is creating
      a genuinely new file, not overwriting an existing one.
- [ ] [AI] Re-confirm no `opencode-go` roster model clears Claude Opus 4.8's SWE-bench Pro bar
      (69.2%) since this plan's research snapshot: compare the live `opencode models` roster from
      the first item above against `tech-docs.md`'s benchmark table. Acceptance: `glm-5.2` remains
      the strongest confirmed roster model (62.1% SWE-bench Pro) and still does not clear 69.2% — if
      it now does (or a new model does), STOP and update `tech-docs.md`'s "Correcting 'Opus 5'"
      section and Decision 1 before proceeding to Phase 1, since the thinking-tier target would
      change from a collapse-onto-execution-tier design to a genuinely distinct model.

### Phase 0 Gate

- [ ] [AI] All 8 items above ticked with their acceptance evidence recorded inline.

> **Pause Safety**: no code changed yet. Safe to stop and resume anytime; nothing to revert.

---

## Phase 1 — TDD the Engine Change: 3-Branch `convert_model()` (`ose-public`)

- [ ] [AI] **RED**: edit
      `specs/apps/rhino/behavior/rhino-cli/gherkin/harness/agents-sync.feature:33` — change
      `the corresponding .opencode/ agent uses the "opencode-go/minimax-m2.7" model identifier` to
      `the corresponding .opencode/ agent uses the "opencode-go/glm-5.2" model identifier` (this
      scenario documents the execution tier — `sonnet`/omitted). Add a NEW scenario immediately
      after it explicitly naming the `opus` alias (thinking tier), per Decision 2 (`tech-docs.md`):
      `Given a Claude Code agent with model "opus" / When rhino-cli's Claude-to-OpenCode sync runs /
Then the corresponding .opencode/ agent uses the "opencode-go/glm-5.2" model identifier` — with
      a comment or scenario description noting this is the thinking tier, collapsed onto the
      execution tier's target per Decision 1 (no roster model clears Opus 4.8 separately). Then
      update `apps/rhino-cli/tests/agents.rs`'s matching scenario-text-quoting assertion at line 267
      and its other 7 hard-coded `opencode-go/minimax-m2.7` fixture strings (lines 233, 273, 288,
      305, 457, 479, 495 — all non-haiku-tier fixtures) to `opencode-go/glm-5.2`; add a NEW fixture/
      assertion covering an explicit `opus`-tagged agent if none of the existing 7 already uses
      `model: opus` literally (check first — some fixtures may already use `opus` and rely on the
      `else` branch implicitly; if so, no new fixture is needed, just confirm one exists). Also
      update `apps/rhino-cli/src/application/agents/converter.rs`'s test module: rename the existing
      `convert_model_default` test to `convert_model_sonnet_and_default` and adjust it to assert
      `"opencode-go/glm-5.2"` for `"sonnet"`, `""`, and `"inherit"` only (drop any `opus` case if it
      was previously bundled there); add a NEW test function `convert_model_opus` asserting
      `convert_model("opus") == "opencode-go/glm-5.2"` (thinking tier, explicit branch per
      Decision 1); update `convert_model_haiku` to expect `"opencode-go/minimax-m3"` instead of
      `"opencode-go/glm-5"`; update its fixture strings at lines 507 and 624 (both non-haiku-tier
      fixtures — line 624 is inside `encode_emits_permission_block_not_tools`, unrelated to the
      model-mapping assertions but sharing the same stale literal) → `glm-5.2`. Update
      `apps/rhino-cli/src/application/agents/sync_validator.rs`'s 5 hard-coded
      `opencode-go/minimax-m2.7` fixture strings (lines 447, 505, 535, 550, 565 — all non-haiku-tier
      fixtures — leave line 520's `opencode-go/wrong` untouched, it is a deliberate negative-case
      fixture) to `opencode-go/glm-5.2`. Command: `nx run rhino-cli:test:quick`. Acceptance: build
      fails to compile or tests fail, naming a mismatch between the now-updated expectations
      (including the new `convert_model_opus` test) and `convert_model()`'s still-old
      two-branch implementation.
  - **Gherkin (underpins) →** "Converting a thinking-tier Claude model alias yields the closest
    available OpenCode Go model to Opus tier"; "Converting an execution-tier Claude model alias
    yields the Sonnet-tier-or-above OpenCode Go model"; "Converting a fast-tier Claude model alias
    yields the closest OpenCode Go model to Sonnet tier without exceeding it" (all three titles
    verbatim from `prd.md`'s Gherkin Acceptance Criteria) — `convert_model()` is a pure data-mapping
    function (Claude alias in, OpenCode model ID string out); per the Gherkin-Tagged Delivery Steps
    pure-core (`underpins`) exception, this single RED step supplies the data-mapping test coverage
    all three scenarios rely on, rather than one `binds` cycle per scenario.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: edit `apps/rhino-cli/src/application/agents/converter.rs`'s `convert_model()`
      function per `tech-docs.md` Decision 1 — restructure from the two-branch `if m == "haiku" {
... } else { ... }` to an explicit three-branch `if m == "haiku" { ... } else if m == "opus" {
... } else { ... }`, with the `haiku` branch returning `"opencode-go/minimax-m3"`, the `opus`
      branch returning `"opencode-go/glm-5.2"`, and the `else` branch (sonnet/omitted/inherit)
      returning `"opencode-go/glm-5.2"` — update the doc comment per Decision 1's full text
      (explaining the collapse and why it's intentional). Command: `nx run rhino-cli:test:quick`.
      Acceptance: all tests pass, including `convert_model_opus` and the other updated tests from
      the RED step above.
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: run `cargo clippy --all-targets --all-features -- -D warnings` from
      `apps/rhino-cli/`. Acceptance: zero warnings. If clippy flags the `opus`/`else` branches as
      `if_same_then_else` (identical bodies), add `#[allow(clippy::if_same_then_else)]` directly
      above the `if` with a one-line comment pointing at `tech-docs.md` Decision 1 — do NOT collapse
      the branches back to silence the lint; the explicit three-way structure is intentional.
  - _Suggested executor: `swe-rust-dev`_

### Phase 1 Gate

- [ ] [AI] `nx run rhino-cli:test:quick` — exits 0.
- [ ] [AI] `nx run rhino-cli:specs:behavior:coverage` — exits 0, non-vacuous (both the `opus` and
      `sonnet`/omitted Gherkin scenarios resolve to real, passing tests, not just one shared test).

> **Pause Safety**: engine change complete and tested in `ose-public`; not yet propagated to
> `ose-primer`/`ose-infra`, and no config/doc files changed yet. Safe to stop. To resume: proceed to
> Phase 2.

---

## Phase 2 — Config Bump + Regenerate Bindings + Pi Model Pin (`ose-public`)

- [ ] [AI] Edit `.opencode/opencode.json`: change `"model": "opencode-go/minimax-m2.7"` to
      `"model": "opencode-go/glm-5.2"` and `"small_model": "opencode-go/glm-5"` to
      `"small_model": "opencode-go/minimax-m3"`. Acceptance: `model` reads `opencode-go/glm-5.2`
      (covers both thinking + execution tiers — OpenCode's own config has only 2 slots, per
      `tech-docs.md`'s File Impact note) and `small_model` reads `opencode-go/minimax-m3` (fast).
- [ ] [AI] Run `npm run generate:bindings` from the repo root. Acceptance: command exits 0 and
      reports converting 74 agents (`ls .claude/agents/*.md | wc -l` returns 75 — one of the 75
      glob matches is `README.md`, which `convert_all_agents()` intentionally skips via its explicit
      `name == "README.md"` exclusion; this count can drift as agents are added/removed) with 0
      failures.
- [ ] [AI] Run `npm run validate:sync`. Acceptance: exits 0 — every `.opencode/agents/*.md` file's
      `model:` field matches `convert_model()`'s new output. Confirm the split explicitly: run
      `grep -L "model: opencode-go/minimax-m3" .opencode/agents/*.md | grep -v README.md | xargs grep -L "model: opencode-go/glm-5.2"`
      and expect zero output (every real agent file matches one of the two IDs;
      `.opencode/agents/README.md` is excluded because it is a hand-authored catalog file with no
      `model:` field, never touched by `convert_all_agents()`), then
      `grep -l "model: opencode-go/minimax-m3" .opencode/agents/*.md | wc -l` should equal the count
      of `.claude/agents/*.md` files with `model: haiku` (11, per Phase 0's finding).
- [ ] [AI] Create `.pi/settings.json` (new file, `ose-public` only per `tech-docs.md` Decision 5)
      with this exact content:

  ```json
  {
    "defaultProvider": "opencode-go",
    "defaultModel": "glm-5-2",
    "enabledModels": ["opencode-go/glm-5-2", "opencode-go/minimax-m3"]
  }
  ```

  Record in this checklist item's own completion note: "`defaultModel: glm-5-2` and the
  `glm-5-2` entry in `enabledModels` are `[Needs Verification]` per `tech-docs.md` Decision 6 —
  Pi's own catalog renders this ID hyphenated rather than dotted (`glm-5.2`); not locally verified
  against a live `pi` session (user directive, 2026-07-05: trust research). `defaultModel` covers
  both the thinking and execution tiers (collapsed per Decision 1); `enabledModels` additionally
  lists the fast tier (`minimax-m3`) so a Pi user can manually cycle to it via Ctrl+P, since Pi's
  schema has only one `defaultModel` slot (Decision 5)." Acceptance:
  `cat .pi/settings.json | python3 -m json.tool` exits 0 (valid JSON) and prints all three fields
  with the values above.

- [ ] [AI] Confirm `docs/reference/platform-bindings.md`'s Pi row `Status` column is untouched
      (still `Reserved`, not flipped to `Active`) — per `tech-docs.md` Decision 5, this plan does not
      change Pi's adoption status. Acceptance: `grep -n "Pi (pi.dev)" docs/reference/platform-bindings.md`
      output unchanged from Phase 0's baseline (no diff in that line from this plan's edits).

### Phase 2 Gate

- [ ] [AI] `npm run validate:sync` — exits 0.
- [ ] [AI] `git diff --stat .opencode/agents/` shows only `model:` line changes (74 files —
      `README.md` is untouched by the regeneration and shows no diff), no unrelated diffs (confirms
      the sync regeneration touched nothing else).
- [ ] [AI] `git status --short .pi/` shows `.pi/settings.json` as a new (`??` or `A`) file.

> **Pause Safety**: config, generated bindings, and Pi's model pin updated and validated in
> `ose-public`. Safe to stop. To resume: proceed to Phase 3.

---

## Phase 3 — Docs Refresh (`ose-public`)

- [ ] [AI] Edit `CLAUDE.md:45` — update the sentence describing the OpenCode mapping to reflect the
      3-tier design: thinking (`opus`) and execution (`sonnet`/omitted) both → `opencode-go/glm-5.2`
      (explicitly noting the collapse is intentional, not an oversight), fast (`haiku`) →
      `opencode-go/minimax-m3`. Acceptance: line reads accurately; `npm run lint:md:fix` run
      afterward reports no violations introduced.
- [ ] [AI] Edit `repo-governance/development/agents/model-selection.md`: update the terminology
      note's example ID (line 18) from `opencode-go/minimax-m2.7` to `opencode-go/glm-5.2`, and
      rewrite the `### Model ID Mapping` table plus the following `### 3-to-2 Tier Collapse` prose
      (lines 279-297 — the full section, not just the table) to show the 3-tier mapping as 3
      explicit rows (thinking/`opus`, execution/`sonnet`+omitted, fast/`haiku`), even though thinking
      and execution show the identical target — with the current SWE-bench Pro figures from
      `tech-docs.md` (62.1% for glm-5.2 vs. both Sonnet-5 63.2% and Opus-4.8 69.2%; 59.0% for
      minimax-m3) and an explicit note that neither the thinking nor fast tier clears its respective
      Claude bar (Opus 4.8 / N/A for fast — fast is deliberately below-tier by design). Also decide
      whether the `### 3-to-2 Tier Collapse` heading itself should be renamed (e.g. to
      "Tier Collapse") now that the design is an explicit 3-branch structure, not a 3-to-2 collapse.
      Acceptance: no remaining reference to `opencode-go/minimax-m2.7` or unsuffixed
      `opencode-go/glm-5` in this file (`grep -c "minimax-m2.7\|opencode-go/glm-5\b"` returns 0).
- [ ] [AI] Edit `repo-governance/development/agents/ai-agents.md`: update line 75's model-selection
      bullet and lines 2577-2578's frontmatter example comments to the 3-tier mapping (thinking/
      execution both `opencode-go/glm-5.2`, fast `opencode-go/minimax-m3`). Acceptance:
      `grep -c "minimax-m2.7\|opencode-go/glm-5\b" repo-governance/development/agents/ai-agents.md`
      returns 0.
- [ ] [AI] Edit `repo-governance/conventions/structure/governance-vendor-independence.md:168` —
      update the example `model: opencode-go/minimax-m2.7` to `model: opencode-go/glm-5.2`.
      Acceptance: line reflects the new ID.
- [ ] [AI] Edit `docs/reference/platform-bindings.md` (lines 172-174) — consolidate the
      `omit (inherit)`/`sonnet` rows (both currently pointing at the same `opencode-go/minimax-m2.7`
      target) into a single `sonnet`/omitted execution-tier row, add a new `opus` thinking-tier row,
      and keep the `haiku` row — 3 rows total, matching the plan's 3-tier design. Acceptance:
      `grep -c "minimax-m2.7\|opencode-go/glm-5\b" docs/reference/platform-bindings.md` returns 0.
- [ ] [AI] Refresh `docs/reference/ai-model-benchmarks.md` per `tech-docs.md` Decision 7: replace
      the `OpenCode Go Models` roster table and per-model detail sections (~lines 282-593) with the
      current 13-model roster and the benchmark table from `tech-docs.md`'s Current State section,
      including the Opus-4.8 comparison column (not just Sonnet-5); add the "Correcting 'Opus 5'"
      explanation (no such model exists; Opus 4.8 is the real thinking-tier bar; Fable 5 exists but
      is out of scope) so a reader of the benchmarks doc understands the thinking-tier collapse;
      add the **standard per-token API pricing** table from `tech-docs.md`'s "Standard API pricing
      per model" section, each figure carrying its retrieval date (2026-07-05); add the NEW
      **frontier/big-brand model reference table** (Anthropic/OpenAI/Google current flagships,
      informational only, explicitly labeled as not available via `opencode-go`); update the
      `Claude-to-OpenCode mapping` table (~lines 556-593) to the 3-tier mapping; update the
      document's "Last updated" date (line 14) to this phase's completion date; update any
      Claude-model reference rows elsewhere in the file citing Sonnet 4.6/Opus 4.7 to Sonnet 5/Opus
      4.8 with their current benchmark figures from `tech-docs.md`. Acceptance:
      `grep -c "minimax-m2.7\|opencode-go/glm-5\b\|Sonnet 4.6\|Opus 4.7\|Opus 5" docs/reference/ai-model-benchmarks.md`
      returns 0 (excluding an explicit "superseded"/"does not exist" historical/corrective note if
      one is kept for context), the file's own roster table lists all 13 current
      `opencode-go` models from `tech-docs.md`, the new pricing table and frontier reference table
      are both present with retrieval-date notes, and every model/pricing figure in the refreshed
      sections carries an inline date (publish date or "retrieved YYYY-MM-DD").
- [ ] [AI] Run `npm run lint:md:fix` repo-wide. Acceptance: exits 0, no markdown violations in any
      file touched above.

### Phase 3 Gate

- [ ] [AI] `grep -rn "opencode-go/minimax-m2\.7\|opencode-go/glm-5\b" --include="*.md" .` from the
      repo root (excluding `node_modules`, `target`, `dist`) returns zero hits anywhere in
      `ose-public`'s documentation.
- [ ] [AI] `npx nx affected -t lint` — exits 0 (confirms no markdown/lint regressions from the
      docs refresh, scoped to this plan's actual blast radius rather than the whole workspace).

> **Pause Safety**: all `ose-public` docs, code, config, generated bindings, and Pi's model pin now
> consistent with the new 3-tier mapping, but not yet committed — this plan batches all commits per
> repo in the Final Phase (see `tech-docs.md`'s Rollback section), not per phase. Safe to stop with
> the working tree uncommitted; `ose-primer`/`ose-infra` are independent repos and this phase does
> not depend on them. To resume: proceed to Phase 4.

---

## Phase 4 — Propagate to `ose-primer` and `ose-infra`

- [ ] [AI] In `/Users/wkf/ose-projects/ose-primer`, copy the byte-identical engine change: apply the
      same edits as Phase 1 to
      `apps/rhino-cli/src/application/agents/converter.rs`,
      `apps/rhino-cli/src/application/agents/sync_validator.rs`, `apps/rhino-cli/tests/agents.rs`,
      and `specs/apps/rhino/behavior/rhino-cli/gherkin/harness/agents-sync.feature`. Acceptance:
      `diff /Users/wkf/ose-projects/ose-public/apps/rhino-cli/src/application/agents/converter.rs /Users/wkf/ose-projects/ose-primer/apps/rhino-cli/src/application/agents/converter.rs`
      (and the same for `sync_validator.rs`/`tests/agents.rs`/the `.feature` file) report "Files
      are identical" for all four pairs.
- [ ] [AI] In `ose-primer`, run `nx run rhino-cli:test:quick` and `nx run rhino-cli:specs:behavior:coverage`.
      Acceptance: both exit 0.
- [ ] [AI] In `ose-primer`, edit `.opencode/opencode.json`: `model` → `opencode-go/glm-5.2`,
      `small_model` → `opencode-go/minimax-m3`. Run `npm run generate:bindings` then
      `npm run validate:sync`. Acceptance: `validate:sync` exits 0.
- [ ] [AI] In `ose-primer`, refresh the 7 governance/reference docs per `tech-docs.md`'s File Impact
      table (`ose-primer` section) to the 3-tier mapping: `CLAUDE.md:52`, `AGENTS.md:319`,
      `repo-governance/development/agents/model-selection.md` (lines 269-272, now 3 explicit rows),
      `repo-governance/development/agents/ai-agents.md` (lines 66, 155, 2505-2506),
      `repo-governance/conventions/structure/governance-vendor-independence.md:167`,
      `docs/reference/platform-bindings.md` (lines 181-183, now 3 explicit rows), and a full refresh
      of `docs/reference/ai-model-benchmarks.md` (same shape as `ose-public`'s Phase 3 step,
      including the pricing table and frontier reference table with retrieval dates; this repo's
      "Last updated" line is at line 15 and currently reads "2026-04-19"). Run
      `npm run lint:md:fix`. Acceptance:
      `grep -rn "opencode-go/minimax-m2\.7\|opencode-go/glm-5\b" CLAUDE.md AGENTS.md repo-governance/ docs/ --include="*.md"`
      returns zero hits; `npx nx affected -t lint` exits 0.
- [ ] [AI] In `/Users/wkf/ose-projects/ose-infra`, copy the same byte-identical engine change to the
      same 4 files. Acceptance: same byte-identity diff check as above, for all four files, against
      `ose-public`'s versions.
- [ ] [AI] In `ose-infra`, run `nx run rhino-cli:test:quick` and `nx run rhino-cli:specs:behavior:coverage`.
      Acceptance: both exit 0.
- [ ] [AI] In `ose-infra`, resolve `.opencode/opencode.json` per Phase 0's investigation finding: if
      no rationale was found for the `zai-coding-plan/*` divergence, edit `model` →
      `opencode-go/glm-5.2` and `small_model` → `opencode-go/minimax-m3`, and remove the
      `zai-coding-plan` provider block if one exists elsewhere in the file; if a valid rationale WAS
      found, tick this item "N/A — see Phase 0 finding" and leave the file unchanged. Run
      `npm run generate:bindings` then `npm run validate:sync`. Acceptance: `validate:sync` exits 0
      either way.
- [ ] [AI] In `ose-infra`, refresh `repo-governance/development/agents/model-selection.md` (lines
      262-265, 268, 272-273) and `docs/reference/platform-bindings.md` (lines 187-189) to the 3-tier
      mapping (3 explicit rows); refresh `docs/reference/ai-model-benchmarks.md` in full (same shape
      as `ose-public`'s Phase 3 step, including the pricing table and frontier reference table with
      retrieval dates), even though it does not currently cite the stale IDs directly, for
      roster/Claude-reference-point consistency. `CLAUDE.md`, `AGENTS.md`, `ai-agents.md`, and
      `governance-vendor-independence.md` need no edit in this repo (Phase 0 re-confirmed zero
      stale-ID hits) — tick as "N/A — no hits, confirmed Phase 0" if that still holds, otherwise
      apply the same edit shape as `ose-primer`. Run `npm run lint:md:fix`. Acceptance:
      `grep -rn "opencode-go/minimax-m2\.7\|opencode-go/glm-5\b" repo-governance/ docs/ --include="*.md"`
      returns zero hits; `npx nx affected -t lint` exits 0.
- [ ] [AI] Confirm `.pi/settings.json` was NOT created in `ose-primer` or `ose-infra` — per
      `tech-docs.md` Decision 5, Pi's model pin is `ose-public`-only. Acceptance:
      `ls /Users/wkf/ose-projects/ose-primer/.pi/ /Users/wkf/ose-projects/ose-infra/.pi/ 2>&1` both
      report "No such file or directory".

### Phase 4 Gate

- [ ] [AI] `apps/rhino-cli/` (`src/`, `Cargo.toml`, `Cargo.lock`, `project.json`,
      `specs/apps/rhino/behavior/rhino-cli/gherkin/**`) byte-identical across all 3 repos — same
      diff-pairwise check used by prior cross-repo plans.
      Acceptance: zero diffs, all 3 pairs.
- [ ] [AI] All 3 repos' `npm run validate:sync` exit 0.
- [ ] [AI] All 3 repos' governance/reference docs (`CLAUDE.md`, `AGENTS.md`, `model-selection.md`,
      `ai-agents.md`, `governance-vendor-independence.md`, `platform-bindings.md`,
      `ai-model-benchmarks.md`) contain zero references to `opencode-go/minimax-m2.7`, unsuffixed
      `opencode-go/glm-5`, or a fabricated "Opus 5" model name.

> **Pause Safety**: all 3 repos now consistent. Safe to stop between repos within this phase if
> needed — each repo's edit is independent of the others once Phase 1-3's `ose-public` reference
> implementation exists. To resume: continue with whichever repo's items remain unticked.

---

## Final Phase — Cross-Repo Verification, Commit, Push & Archival

### Local Quality Gates (Before Push)

> **Important**: Fix ALL failures found during quality gates, not just those caused by your changes.

- [ ] [AI] Per repo: `npx nx affected -t typecheck,lint,test:quick,specs:behavior:coverage` —
      exits 0 in all 3 repos.
- [ ] [AI] Repo-wide grep, all 3 repos: `grep -rn "opencode-go/minimax-m2\.7\|opencode-go/glm-5\b" . --include="*.md" --include="*.rs" --include="*.json" --include="*.feature"`
      (excluding `node_modules`, `target`, `dist`, `.venv`) returns zero hits, except any file
      explicitly preserved as historical record (e.g. dated changelog entries) which must be
      individually confirmed as out-of-scope, not silently skipped.
- [ ] [AI] `ose-public`: `cat .pi/settings.json` shows `"defaultProvider": "opencode-go"`,
      `"defaultModel": "glm-5-2"`, and `"enabledModels"` containing both `opencode-go/glm-5-2` and
      `opencode-go/minimax-m3`; `ose-primer`/`ose-infra` have no `.pi/` directory.

### Commit Guidelines

- [ ] [AI] Commit thematically per repo, explicit paths only (never `git add -A`). Suggested split
      per repo: engine (`fix(rhino-cli): map opus/sonnet/haiku to explicit opencode-go tiers`),
      config + generated bindings (`chore(opencode): bump model mapping to current opencode-go models`),
      Pi model pin — `ose-public` only (`chore(pi): pin default model to opencode-go/glm-5.2`),
      docs (`docs: refresh OpenCode Go model references, benchmarks, and frontier comparison`).

### Post-Push Verification

- [ ] [AI] Push each repo → `origin main`; monitor the `main-ci` workflow
      (`.github/workflows/main-ci.yml`, triggered on push to `main` in all 3 repos — especially its
      `rust` job ["Rust quality gate (all projects)"] and `markdown-per-file` job ["Markdown
      per-file validators (all files)"], the two most relevant to this plan's `rhino-cli`/docs
      changes) via `gh run list --workflow=main-ci.yml --limit 1` then `gh run view <run-id>` (poll
      every 2 min, one `gh run view` per wakeup, never `gh run watch`); verify green; fix any failure
      before proceeding.

### Final Gate

- [ ] [AI] Every OpenCode alternative in use (top-level config + every synced agent) resolves to
      either `opencode-go/glm-5.2` (thinking + execution) or `opencode-go/minimax-m3` (fast) in all
      3 repos, confirmed via the Phase 2/4 `validate:sync` runs.
- [ ] [AI] Zero references to a retired (`opencode-go/glm-5` unsuffixed) or below-Sonnet-tier
      (`opencode-go/minimax-m2.7`) model ID, or to a fabricated "Opus 5" model, remain in any config,
      code, or doc across all 3 repos (Final Phase's repo-wide grep, all 3 repos, zero hits).
- [ ] [AI] Every repo's own `docs/reference/ai-model-benchmarks.md` "Last updated" date reflects
      this plan's execution date and cites Claude Sonnet 5/Opus 4.8 as the current reference points
      (with Claude Fable 5 noted as existing but out of scope), with the standard-API-pricing table
      and the frontier/big-brand reference table both present and every figure carrying its
      retrieval/publish date (user directive, 2026-07-05).
- [ ] [AI] `ose-infra`'s provider divergence is resolved one way or the other (reconciled, or
      explicitly documented as intentional) — not left silently unexplained.
- [ ] [AI] `ose-public`'s `.pi/settings.json` exists, pins the `opencode-go` provider/model, and
      lists both tier targets in `enabledModels`; `docs/reference/platform-bindings.md`'s Pi row
      `Status` is still `Reserved` (not flipped to `Active`); `ose-primer`/`ose-infra` have no
      `.pi/` directory.

### Plan Archival

- [ ] [AI] Verify ALL delivery items ticked and ALL gates pass (local + CI, all three repos).
- [ ] [AI] Move plan: `git mv plans/in-progress/upgrade-opencode-go-models plans/done/<completion-date>__upgrade-opencode-go-models`.
- [ ] [AI] Update `plans/in-progress/README.md` (remove entry) + `plans/done/README.md` (add entry
      summarizing the 3-tier model-mapping change, the Pi model pin, the Opus-5-doesn't-exist
      correction, and the `ose-infra` divergence finding).
- [ ] [AI] Commit: `docs(plans): move upgrade-opencode-go-models to done`.

> **Pause Safety**: fully enforced and consistent across all 3 repos; nothing half-applied. Safe to
> stop. To resume: re-run `npm run validate:sync` in each repo.

## Validation Checklist

- [ ] All TDD cycles complete for the 3-branch engine change (RED→GREEN→REFACTOR), `ose-public`
- [ ] Engine byte-identical across all 3 repos
- [ ] Every OpenCode alternative (config + all synced agents, all 3 repos) resolves to
      `opencode-go/glm-5.2` (thinking `opus` + execution `sonnet`/omitted) or
      `opencode-go/minimax-m3` (fast `haiku`)
- [ ] Zero references anywhere to `opencode-go/minimax-m2.7`, unsuffixed `opencode-go/glm-5`, or a
      fabricated "Opus 5" model (excluding explicitly-preserved historical records)
- [ ] `ai-model-benchmarks.md` refreshed with current roster + current Claude reference points
      (Sonnet 5 AND Opus 4.8) + standard API pricing table + frontier/big-brand reference table
      (every figure dated), in all 3 repos, explicitly noting the thinking-tier collapse and the
      fast tier's gap below Sonnet-5 rather than glossing over either
- [ ] `ose-infra` provider divergence resolved or explicitly documented
- [ ] `ose-public`'s `.pi/settings.json` pins `opencode-go`/`glm-5-2` with `enabledModels` covering
      both tiers; Pi's catalog Status stays `Reserved`; no `.pi/` in `ose-primer`/`ose-infra`
- [ ] All 3 repos' CI green
