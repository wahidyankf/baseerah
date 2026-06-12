# Delivery Checklist — Mermaid State Diagram Validation (ose-public)

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.
>
> **Phase Gate** — every phase ends with a `### Phase N Gate` (must-pass verification) plus a
> `> **Pause Safety**:` note (the safe-to-stop state and the single command to resume). A phase
> is not complete until its gate is green; do not start phase N+1 while any gate check fails.

## Worktree

Worktree path: `worktrees/mermaid-state-diagram-validation/`

Optional manual pre-provisioning (run from repo root):

```bash
claude --worktree mermaid-state-diagram-validation
```

The plan-execution Step 0 gate enters this worktree by default: it auto-provisions from the latest
`origin/main` when missing, syncs with `origin/main` before implementing, and prompts before
deleting the worktree after the plan is archived and pushed.

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md) and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

## Phase 0: Environment Setup and Baseline

> _Executor: repo-setup-manager_

- [ ] [AI] Install dependencies in the root worktree: `npm install`
      — acceptance: exits 0, `node_modules/` synchronized
- [ ] [AI] Converge the toolchain in the root worktree: `npm run doctor -- --fix`
      — acceptance: exits 0 with no unresolved drift (Rust/cargo present)
- [ ] [AI] Build rhino-cli to confirm a clean baseline: `npx nx run rhino-cli:build`
      — acceptance: exits 0
- [ ] [AI] Record the existing test baseline: `npx nx run rhino-cli:test:unit`
      — acceptance: pass/fail counts recorded; all 30 existing mermaid tests pass
      [Repo-grounded: 30 `#[test]` in `apps/rhino-cli/src/internal/mermaid.rs`]
- [ ] [AI] Record the existing mermaid gate baseline: `npx nx run rhino-cli:validate:mermaid`
      — acceptance: current pass/fail recorded (flowchart-only behavior today)
- [ ] [AI] Resolve any preexisting failures before proceeding
      — acceptance: no preexisting failures remain unresolved

### Phase 0 Gate

> All checks below must pass before starting Phase A.

- [ ] [AI] `npm install` exited 0 and `npm run doctor -- --fix` reports no unresolved drift
- [ ] [AI] `npx nx run rhino-cli:build` exits 0
- [ ] [AI] `npx nx run rhino-cli:test:unit` baseline recorded; every preexisting failure resolved

> **Pause Safety**: only the local toolchain and baseline were verified — no source changes exist.
> Safe to stop indefinitely. To resume: re-run `npx nx run rhino-cli:test:unit` and confirm it is
> still clean.

## Phase A: Unify validator onto the fresh module design (behavior-preserving)

> _Suggested executor: `swe-rust-dev`_
>
> Pure refactor of `apps/rhino-cli/src/internal/mermaid.rs` into the `mermaid/` directory module.
> No behavior change. Every existing flowchart test stays green.

- [ ] [AI] **RED**: Create the directory-module skeleton — move
      `apps/rhino-cli/src/internal/mermaid.rs` to `apps/rhino-cli/src/internal/mermaid/mod.rs`
      via `git mv`, then split into empty `types.rs`, `extractor.rs`, `diagram.rs`, `flowchart.rs`,
      `graph.rs`, `validator.rs`, `reporter.rs` siblings (content not yet moved). Run
      `npx nx run rhino-cli:test:unit`
      — acceptance: compilation FAILS (unresolved items) — confirms the split is wired but
      incomplete
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: Move the type definitions (`Direction`, `Node`, `Edge`, `Subgraph`,
      `ParsedDiagram`, `Violation`, `Warning`, `ViolationKind`, `WarningKind`, `ValidateOptions`,
      `MermaidBlock`) into `types.rs`; move `extract_blocks` into `extractor.rs`; move flowchart
      `parse_diagram` logic into `flowchart.rs`; move rank/width/depth (`max_width`, `depth`) into
      `graph.rs`; move `validate_blocks`/`default_validate_options` into `validator.rs`; move
      `format_text`/`format_json`/`format_markdown` into `reporter.rs`; re-export the public API
      from `mod.rs`. Run `npx nx run rhino-cli:test:unit`
      — acceptance: all 30 preexisting tests pass; no test bodies changed
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **RED**: Add a unit test in `apps/rhino-cli/src/internal/mermaid/diagram.rs`
      asserting `detect_kind("flowchart TB ...")` returns `DiagramKind::Flowchart` and
      `detect_kind("stateDiagram-v2 ...")` returns `DiagramKind::State`. Run
      `npx nx run rhino-cli:test:unit`
      — acceptance: test FAILS (`DiagramKind` and `detect_kind` not yet defined)
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: Add `DiagramKind { Flowchart, State }` to `types.rs` and a
      `detect_kind`/dispatch function in `diagram.rs` that returns `Flowchart` for `flowchart`/
      `graph` headers and routes parsing through `flowchart.rs` (state arm returns an empty
      `ParsedDiagram` for now). Run `npx nx run rhino-cli:test:unit`
      — acceptance: all tests pass; flowchart dispatch unchanged
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: Confirm the kind-agnostic core (`graph.rs`, `validator.rs`, `reporter.rs`)
      references no flowchart-specific symbols; remove any dead `pub` left by the move; run
      `cargo fmt`. Run `npx nx run rhino-cli:lint && npx nx run rhino-cli:test:unit`
      — acceptance: lint exits 0 (clippy `-D warnings`), all tests pass
  - _Suggested executor: `swe-rust-dev`_

### Local Quality Gates (Before Push) — Phase A

- [ ] [AI] Run affected typecheck: `npx nx affected -t typecheck`
- [ ] [AI] Run affected linting: `npx nx affected -t lint`
- [ ] [AI] Run affected quick tests: `npx nx affected -t test:quick`
      — acceptance: rhino-cli library coverage stays `≥90`
- [ ] [AI] Run affected spec coverage: `npx nx affected -t spec-coverage`
- [ ] [AI] Fix ALL failures — including preexisting issues not caused by your changes
- [ ] [AI] Re-run failing checks to confirm resolution; verify zero failures before pushing

> **Important**: Fix ALL failures found during quality gates, not just those caused by your
> changes. This follows the root cause orientation principle — proactively fix preexisting errors
> encountered during work. Commit preexisting fixes separately with appropriate conventional
> commit messages.

### Commit Guidelines — Phase A

- [ ] [AI] Commit thematically: `refactor(rhino-cli): split mermaid validator into unified module`
- [ ] [AI] Follow Conventional Commits; keep the pure refactor in its own commit separate from any
      preexisting fixes

### Phase A Gate

> All checks below must pass before starting Phase B.

- [ ] [AI] `npx nx run rhino-cli:test:unit` — expected: all 30 preexisting flowchart tests pass
- [ ] [AI] `npx nx run rhino-cli:lint` — expected: exits 0
- [ ] [AI] `npx nx run rhino-cli:validate:mermaid` — expected: identical result to the Phase 0
      baseline (flowchart behavior unchanged)

> **Pause Safety**: the validator is a clean directory module with byte-for-byte flowchart
> behavior; no state code yet. Safe to stop. To resume: `npx nx run rhino-cli:test:unit`.

## Phase B: State front-end + shared golden corpus

> _Suggested executor: `swe-rust-dev`_
>
> Add `state.rs`, wire it into `diagram.rs`, apply width + label rules, land the golden corpus.

- [ ] [AI] **RED**: Add a unit test in `apps/rhino-cli/src/internal/mermaid/diagram.rs` asserting
      `detect_kind` returns `State` for both `stateDiagram-v2` and `stateDiagram` headers. Run
      `npx nx run rhino-cli:test:unit`
      — acceptance: test FAILS (`State` not yet returned)
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: Implement state-header detection in `diagram.rs` for `stateDiagram-v2` and
      `stateDiagram`. Run `npx nx run rhino-cli:test:unit`
      — acceptance: detection test passes
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **RED**: Add a unit test in `apps/rhino-cli/src/internal/mermaid/state.rs` parsing an
      11-state `direction LR` chain and asserting 11 `Node`s with the chain shape. Run
      `npx nx run rhino-cli:test:unit`
      — acceptance: test FAILS (`state.rs` parser absent)
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: Implement the `state.rs` parser per `tech-docs.md` pinned grammar facts —
      bare ids, `id : desc`, `state "desc" as id`, `[*]`, stereotype states (`<<choice>>`/
      `<<fork>>`/`<<join>>` and `[[...]]`) as `Node`s; `A --> B : lbl` as `Edge`; composite
      `state X { }` as `Subgraph` (recursed); skip notes/comments/`--`; match `-->` before `--`;
      `direction` accepts `TB|BT|LR|RL` only (reject `TD`). Run `npx nx run rhino-cli:test:unit`
      — acceptance: the 11-node parse test passes
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **RED**: Add a unit test asserting the 11-state `direction LR` chain yields a
      `width_exceeded` violation with width 11 through `validate_blocks`. Run
      `npx nx run rhino-cli:test:unit`
      — acceptance: test FAILS (state edges not yet fed to `graph.rs` width core)
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: Wire the state `ParsedDiagram` through the shared `graph.rs` width core so
      `LR`/`RL` map to the depth-as-horizontal axis like flowcharts. Run
      `npx nx run rhino-cli:test:unit`
      — acceptance: `width_exceeded` width-11 test passes
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **RED**: Add unit tests in `validator.rs` for label rules — a `>30`-char state display
      label and a `>30`-char transition label (`A --> B : <long>`) each yield `label_too_long`; a
      short colon label yields none. Run `npx nx run rhino-cli:test:unit`
      — acceptance: tests FAIL (transition-label check absent)
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: Extend `validator.rs` to check both state display labels and transition-edge
      labels against `max_label_len` using the existing `effective_label_len` per-segment measure
      [Repo-grounded: `effective_label_len` at `apps/rhino-cli/src/internal/mermaid.rs:670`]. Run
      `npx nx run rhino-cli:test:unit`
      — acceptance: all three label tests pass
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **RED**: Add unit tests for D-MAP/D-STEREO — a rank holding `[*]`, `<<choice>>`,
      `<<fork>>`, `<<join>>` plus one more yields `width_exceeded` (5 nodes); a composite
      `state Outer { Inner1 --> Inner2 }` is recorded as a `Subgraph`. Run
      `npx nx run rhino-cli:test:unit`
      — acceptance: tests FAIL
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: Implement pseudostate/stereotype node-counting and composite-as-subgraph
      recursion in `state.rs`. Run `npx nx run rhino-cli:test:unit`
      — acceptance: both tests pass
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **RED**: Add a unit test asserting a block with a multiline `note right of X ... end
    note`, a `%%` comment, and a `--` separator produces zero violations and zero spurious nodes.
      Run `npx nx run rhino-cli:test:unit`
      — acceptance: test FAILS
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: Implement note/comment/`--` skipping in `state.rs`. Run
      `npx nx run rhino-cli:test:unit`
      — acceptance: the free-text test passes (note text exempt from the label rule)
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **RED**: Add the corpus test harness in `apps/rhino-cli/tests/` (confirm exact subdir
      against existing `tests/**/*.rs` layout — e.g. `apps/rhino-cli/tests/mermaid_golden_corpus.rs`)
      that iterates over fixture `.md` files in a `fixtures/state/` subdirectory and asserts actual
      violation JSON equals expected JSON companion files. Run `npx nx run rhino-cli:test:unit`
      — acceptance: test FAILS because fixture directory is empty or absent
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **GREEN**: Land the shared golden corpus — create fixture `.md` files + expected
      violation JSON under `apps/rhino-cli/tests/` (confirm exact subdir against existing
      `tests/**/*.rs` layout) covering over-wide LR chain, compliant narrow chain, long state
      label, long transition label, `[*]`/stereotype counting, composite-as-subgraph, and
      note/comment/`--` exemption; add a corpus test that asserts each fixture's actual violations
      equal its expected JSON. Run `npx nx run rhino-cli:test:unit`
      — acceptance: corpus test passes; this exact fixture set is the one mirrored to ose-primer
      and ose-infra
  - _Suggested executor: `swe-rust-dev`_
- [ ] [AI] **REFACTOR**: Deduplicate any shared parsing helpers between `flowchart.rs` and
      `state.rs` into a small shared util in `diagram.rs`; run `cargo fmt`. Run
      `npx nx run rhino-cli:lint && npx nx run rhino-cli:test:unit`
      — acceptance: lint exits 0, all tests pass
  - _Suggested executor: `swe-rust-dev`_

### Local Quality Gates (Before Push) — Phase B

- [ ] [AI] Run `npx nx affected -t typecheck`
- [ ] [AI] Run `npx nx affected -t lint`
- [ ] [AI] Run `npx nx affected -t test:quick` — acceptance: library coverage stays `≥90`
- [ ] [AI] Run `npx nx affected -t spec-coverage`
- [ ] [AI] Fix ALL failures — including preexisting issues; re-run to confirm zero failures

### Commit Guidelines — Phase B

- [ ] [AI] Commit thematically: `feat(rhino-cli): validate mermaid state diagrams`
- [ ] [AI] Keep the golden corpus in its own commit:
      `test(rhino-cli): add shared state-diagram golden corpus`

### Phase B Gate

> All checks below must pass before starting Phase C.

- [ ] [AI] `npx nx run rhino-cli:test:unit` — expected: all new state tests + 30 flowchart tests
      pass
- [ ] [AI] `npx nx run rhino-cli:test:quick` — expected: coverage `≥90`, exits 0
- [ ] [AI] `npx nx run rhino-cli:lint` — expected: exits 0

> **Pause Safety**: the validator now parses and validates state diagrams; flowchart behavior is
> unchanged; the gate has NOT yet been run repo-wide (cleanup pending). Safe to stop. To resume:
> `npx nx run rhino-cli:test:unit`.

## Phase C: Repo-wide state-diagram cleanup (clean-then-gate)

> Per D-CLEAN, fix every violating state diagram repo-wide INCLUDING `plans/done/` and otherwise
> gate-excluded paths. Diagram-only edits.

- [ ] [AI] Enumerate every violating state diagram: run the new validator without exclusions —
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-mermaid`
      and additionally scan `plans/done` and excluded paths explicitly (no `--exclude` flags)
      — acceptance: a complete list of `width_exceeded`/`label_too_long` state-diagram findings is
      produced
- [ ] [AI] Fix each `width_exceeded` state diagram using the width-fix strategies in
      `repo-governance/conventions/formatting/diagrams.md §Width Violation Fix Strategy Guide`
      (direction flip, sequential chaining, splitting) — edit each offending `.md` file
      — acceptance: re-running the validator on each fixed file reports no `width_exceeded`
- [ ] [AI] Fix each `label_too_long` state diagram by shortening state/transition labels per
      `§Strategy 4 — Label Shortening` — edit each offending `.md` file
      — acceptance: re-running the validator on each fixed file reports no `label_too_long`
- [ ] [AI] Verify the gate-scoped scan is clean:
      `npx nx run rhino-cli:validate:mermaid`
      — acceptance: zero state-diagram violations in gate scope
- [ ] [AI] Verify the full repo-wide scan (including `plans/done`) is clean:
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-mermaid`
      — acceptance: zero state-diagram violations anywhere

### Local Quality Gates (Before Push) — Phase C

- [ ] [AI] Run `npm run lint:md` — acceptance: exits 0, no markdownlint violations in edited
      files
- [ ] [AI] Run `npx nx run rhino-cli:validate:mermaid` — acceptance: exits 0 with zero
      state-diagram violations
- [ ] [AI] Run `npx nx run rhino-cli:validate:links` — acceptance: exits 0, no broken links
      introduced
- [ ] [AI] Fix ALL failures — including preexisting issues not caused by your changes

### Commit Guidelines — Phase C

- [ ] [AI] Commit thematically: `docs: fix over-wide and over-long mermaid state diagrams`
- [ ] [AI] If cleanup spans many domains, split commits by domain (plans vs docs vs governance)

### Phase C Gate

> All checks below must pass before starting Phase D.

- [ ] [AI] `npx nx run rhino-cli:validate:mermaid` — expected: exits 0, zero violations in gate
      scope
- [ ] [AI] `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-mermaid`
      — expected: zero state-diagram violations repo-wide including `plans/done`

> **Pause Safety**: all state diagrams repo-wide are compliant and the validator enforces them.
> Safe to stop. To resume: `npx nx run rhino-cli:validate:mermaid`.

## Phase D: Confirm the gate now covers state diagrams

> No new gate wiring is needed — state diagrams ride the existing `validate:mermaid` target,
> pre-commit hook, and CI workflow. This phase confirms they are now in scope. [Repo-grounded:
> > target at `apps/rhino-cli/project.json:167`; gate location documented in
> > `repo-governance/conventions/formatting/diagrams.md §Automated enforcement`]

- [ ] [AI] Confirm an intentionally over-wide state fixture is caught by the gate target — add a
      temporary 11-state `direction LR` block to a scratch file under `local-temp/`, run
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-mermaid local-temp/`
      — acceptance: the scratch file reports `width_exceeded`; then delete the scratch file
- [ ] [AI] Confirm the committed default `stateDiagram-v2` example in
      `repo-governance/conventions/formatting/diagrams.md` (the `[*] --> Pending` example) passes
      the validator [Repo-grounded: example at `diagrams.md:350-358`] — run
      `npx nx run rhino-cli:validate:mermaid`
      — acceptance: that example reports no violation (or is fixed in Phase C if it did)

### Phase D Gate

> All checks below must pass before starting Phase E.

- [ ] [AI] `npx nx run rhino-cli:validate:mermaid` — expected: exits 0 with state diagrams in scope
- [ ] [AI] Temporary over-wide scratch fixture confirmed flagged, then removed
      — expected: `local-temp/` scratch file deleted, working tree clean
- [ ] [AI] `npm run lint:md` — expected: exits 0 (no regressions from Phase C edits)

> **Pause Safety**: state diagrams are confirmed in-scope of the live gate with no new wiring. Safe
> to stop. To resume: `npx nx run rhino-cli:validate:mermaid`.

## Phase E: Governance propagation

> _Suggested executor: `repo-rules-maker`_
>
> Propagate the new state-diagram validation rule into governance, then re-sync platform bindings.

- [ ] [AI] Update `repo-governance/conventions/formatting/diagrams.md` so the width/label rules and
      the `validate-mermaid` enforcement sections state that they apply to state diagrams
      (`stateDiagram-v2` + `stateDiagram` v1), documenting: `[*]`/stereotype nodes count toward
      width; composite states are subgraphs; both state labels and transition labels are checked;
      `direction` is `TB|BT|LR|RL` only
      — acceptance: the diagram convention enumerates state diagrams alongside flowcharts in the
      width/label rule and enforcement sections
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] Sweep repo-rules registers/checkers that enumerate Mermaid rules and add the
      state-diagram rule where Mermaid validation is listed (e.g.,
      `repo-governance/development/quality/markdown.md`,
      `repo-governance/development/quality/repository-validation.md`) [Repo-grounded: these files
      reference `validate:mermaid`]
      — acceptance: each register/checker that lists the Mermaid gate notes state diagrams are now
      in scope
  - _Suggested executor: `repo-rules-maker`_
- [ ] [AI] Re-sync platform bindings: `npm run generate:bindings`
      — acceptance: exits 0; `.opencode/` and `.amazonq/` binding artifacts regenerate with no
      drift; `npx nx run rhino-cli:validate:cross-vendor-parity` (if present) passes
  - _Suggested executor: `repo-rules-maker`_

### Local Quality Gates (Before Push) — Phase E

- [ ] [AI] Run `npx nx affected -t typecheck lint test:quick`
- [ ] [AI] Run `npx nx run rhino-cli:validate:mermaid` over the edited governance docs
      — acceptance: governance docs themselves contain no violating diagrams
- [ ] [AI] Run `npm run lint:md` — acceptance: edited markdown passes markdownlint
- [ ] [AI] Fix ALL failures; re-run to confirm zero failures

### Commit Guidelines — Phase E

- [ ] [AI] Commit thematically:
      `docs(governance): document mermaid state-diagram validation`
- [ ] [AI] Keep regenerated binding artifacts in their own commit if `generate:bindings` produces
      changes: `chore(bindings): re-sync after state-diagram rule`

### Phase E Gate

> All checks below must pass before starting Phase F.

- [ ] [AI] `repo-governance/conventions/formatting/diagrams.md` enumerates state diagrams in the
      width/label and enforcement sections — expected: grep confirms `stateDiagram` in those
      sections
- [ ] [AI] `npm run generate:bindings` exits 0 with no unexpected drift
- [ ] [AI] `npm run lint:md` — expected: exits 0

> **Pause Safety**: governance and bindings reflect the new rule; the implementation and cleanup
> are complete and committed. Safe to stop. To resume: `npx nx affected -t lint`.

## Phase F: Verify and archive

- [ ] [AI] Push all phase commits to `main`
      — acceptance: `git push` succeeds; working tree clean
- [ ] [AI] Monitor ALL GitHub Actions workflows triggered by the push (poll every 3 min; one
      `gh run view --json status,conclusion` per wakeup; do NOT use `gh run watch`)
      — acceptance: all CI checks pass with zero failures
- [ ] [AI] If any CI check fails, fix the root cause and push a follow-up commit; repeat until CI is
      fully green
      — acceptance: all GitHub Actions green
- [ ] [AI] Run `plan-execution-checker` against this plan
      — acceptance: zero findings (CRITICAL/HIGH/MEDIUM all clear)
  - _Suggested executor: `plan-execution-checker`_
- [ ] [AI] Address any `plan-execution-checker` findings and re-run until clean
      — acceptance: zero findings

### Plan Archival

- [ ] [AI] Verify ALL delivery checklist items are ticked
- [ ] [AI] Verify ALL quality gates pass (local + CI)
- [ ] [AI] Rename and move:
      `git mv plans/in-progress/mermaid-state-diagram-validation/ plans/done/2026-06-12__mermaid-state-diagram-validation/`
      using today's completion date
- [ ] [AI] Update `plans/in-progress/README.md` — remove the plan entry
- [ ] [AI] Update `plans/done/README.md` — add the plan entry with completion date
- [ ] [AI] Update any other READMEs that reference this plan (e.g., `plans/README.md`)
- [ ] [AI] Commit the archival: `chore(plans): move mermaid-state-diagram-validation to done`

### Phase F Gate

> Final gate. All checks below must pass to consider the plan complete.

- [ ] [AI] All GitHub Actions green for the latest push — expected: `gh run view` shows success
- [ ] [AI] `plan-execution-checker` reports zero findings
- [ ] [AI] Plan folder moved to `plans/done/2026-06-12__mermaid-state-diagram-validation/`
      — expected: `git status` clean after the archival commit

> **Pause Safety**: the objective is delivered, verified, and archived; CI is green. This is the
> terminal safe state. To resume verification: `gh run view --json status,conclusion`.
