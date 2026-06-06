# Delivery Checklist — Mermaid Gate Coverage Expansion

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.
>
> **Phase Gate** — every phase ends with a `### Phase N Gate` (must-pass verification) plus a
> `> **Pause Safety**:` note (the safe-to-stop state and the single command to resume). A phase
> is not complete until its gate is green; do not start phase N+1 while any gate check fails.

## Worktree

Worktree path: `worktrees/mermaid-gate-coverage-expansion/`

Provision before execution (run from repo root):

```bash
claude --worktree mermaid-gate-coverage-expansion
```

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md)
and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

## Push / Definition of Done

- **Push target**: `origin main`, **direct** (Trunk Based Development — no PR). [Repo-grounded —
  `main` is the trunk]
- **DoD**: full-repo `validate:mermaid` reports zero blocking findings of every kind (structural,
  off-palette color, AND the six new checks); the gate is enforced across all THREE layers —
  pre-push (Layer 1, any `*.md`), the dedicated `pr-validate-mermaid.yml` on `pull_request` to
  `main` (Layer 2), and the same workflow on `push` to `main` (Layer 3); all existing
  `mermaid.rs` unit tests stay green; new behavior (exemption directive, warnings-blocking,
  `off_palette_color`, per-type dispatch matrix, `allow-color` rejection, the six new checks
  `literal_backslash_n` / `quotes_in_brackets` / `edge_label_too_long` / `non_lr_direction` /
  `palette_comment_count` / `separator_width_mismatch` with their exemptable/non-exemptable
  directives, and the `collect_md_default_dirs()` `apps`/`libs` reconciliation) is fully tested;
  `diagrams.md` is accurate; this plan's own diagrams pass the expanded validator (0 findings of
  every kind); the plan is archived to `plans/done/`.

> **Important (fix-all-issues)**: Fix ALL failures found during quality gates, not just those
> caused by your changes. This follows the root-cause-orientation principle — proactively fix
> preexisting errors encountered during work. Do not defer or skip existing issues. Commit
> preexisting fixes separately with appropriate conventional commit messages.

---

## Phase 0: Environment Setup and Baseline

> _Executor: repo-setup-manager_

- [ ] [AI] Provision the worktree from repo root: `claude --worktree mermaid-gate-coverage-expansion`
      — acceptance: `worktrees/mermaid-gate-coverage-expansion/` exists.
- [ ] [AI] Initialize the toolchain in the **root** worktree: `npm install && npm run doctor -- --fix`
      — acceptance: both exit 0; `node_modules/` synchronized; no unresolved toolchain drift.
- [ ] [AI] Build rhino-cli to confirm the validator compiles:
      `cargo build --release --quiet --manifest-path apps/rhino-cli/Cargo.toml`
      — acceptance: exits 0.
- [ ] [AI] Capture the **current** (pre-change) baseline by running the existing validator over
      its current scope: `npx nx run rhino-cli:validate:mermaid`
      — acceptance: record pass/fail and any findings to the phase notes below.
- [ ] [AI] Re-measure **full-repo** blocking violations with the current validator binary, scoped
      to every target tree, INCLUDING the soon-to-be-blocking warnings, by running:
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-mermaid --output json repo-governance/ .claude/ plans/ docs/ apps/ libs/ AGENTS.md CLAUDE.md README.md`
      — acceptance: capture per-tree counts of `label_too_long`, `width_exceeded`,
      `multiple_diagrams`, AND `complex_diagram` + `subgraph_density` from the JSON. Record these
      numbers verbatim in the phase notes — they are the structural fix-all backlog. Do NOT trust
      the authoring-time estimates; these re-measured numbers are authoritative.
- [ ] [AI] Establish a **provisional off-palette color backlog** with the CURRENT binary (which
      has no color check yet) by grepping the eligible diagram fences for off-palette hex. Run:
      `grep -rniE 'fill:#|stroke:#|color:#' repo-governance/ .claude/ plans/ docs/ apps/ libs/ AGENTS.md CLAUDE.md README.md --include='*.md'`
      and note any hex NOT in the canonical palette
      (`#0173B2 #DE8F05 #029E73 #CC78BC #CA9161 #000000 #FFFFFF #808080`, case-insensitive)
      — acceptance: a provisional per-tree list of suspect off-palette hex recorded in phase notes.
      This is an estimate only; the authoritative off-palette count is re-measured per tree once
      the `off_palette_color` check lands (Phase 3) using `validate-mermaid` itself.
- [ ] [AI] Run the existing rhino-cli unit tests to establish the green baseline:
      `npx nx run rhino-cli:test:quick`
      — acceptance: baseline pass count recorded; all preexisting failures documented.
- [ ] [AI] Resolve all preexisting failures before proceeding
      — acceptance: no preexisting failures remain unresolved.

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [ ] [AI] `cargo build --release ... apps/rhino-cli/Cargo.toml` exits 0.
- [ ] [AI] `npx nx run rhino-cli:test:quick` is green; baseline recorded.
- [ ] [AI] Full-repo re-measured backlog (per tree, including warnings) recorded in phase notes.

> **Pause Safety**: only the toolchain was verified and the baseline + fix-all backlog recorded —
> no source changed. Safe to stop indefinitely. To resume: re-run
> `npx nx run rhino-cli:validate:mermaid` and confirm the baseline is unchanged.

**Phase 0 notes** (executor fills in): _baseline result, per-tree re-measured counts._

---

## Phase 1: Inline Exemption Directive Parser (TDD)

> _Suggested executor: `swe-rust-dev`_

Implement the `%% validate-mermaid: allow-<kind>` + mandatory `%% reason:` directive. Flowchart
blocks only; structure kinds only (`allow-width`, `allow-complexity`, `allow-subgraph-density`); an
`allow-*` without an adjacent `reason:` is a hard blocking error (new
`ViolationKind::ExemptionMissingReason` or equivalent).

- [ ] [AI] **RED** — Add failing unit tests in `apps/rhino-cli/src/internal/mermaid.rs` covering:
      (a) `allow-width` + `reason:` suppresses a `width_exceeded` violation;
      (b) `allow-width` without `reason:` produces a blocking missing-reason error;
      (c) `allow-label` + `reason:` does NOT suppress `label_too_long`;
      (d) an allow directive on a `sequenceDiagram` block yields zero findings.
      Run `npx nx run rhino-cli:test:quick` — acceptance: the four new tests FAIL; all
      preexisting tests still pass.
- [ ] [AI] **GREEN** — Implement directive parsing in `apps/rhino-cli/src/internal/mermaid.rs`:
      detect `%%`-comment directive lines inside the fence, pair each `allow-*` with the adjacent
      `%% reason:` line, thread the exemption set into the per-block validate step (suppress only
      structure-kind violations; emit the missing-reason blocking error when a reason is absent;
      never suppress `label_too_long` / `multiple_diagrams`; ignore directives on non-flowchart
      blocks). Run `npx nx run rhino-cli:test:quick` — acceptance: all tests (new + preexisting)
      pass.
- [ ] [AI] **REFACTOR** — Extract directive parsing into a cohesive helper; ensure naming and doc
      comments match the existing module style. Run
      `npx nx run rhino-cli:lint && npx nx run rhino-cli:test:quick` — acceptance: both exit 0;
      no clippy warnings introduced.

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [ ] [AI] `npx nx run rhino-cli:test:quick` is green (new exemption tests + all preexisting).
- [ ] [AI] `npx nx run rhino-cli:lint` exits 0.

> **Pause Safety**: the parser is additive — exemptions are recognized but no warning is yet
> blocking, so behavior for existing diagrams is unchanged. Safe to stop. To resume:
> `npx nx run rhino-cli:test:quick`.

---

## Phase 2: Promote Complexity Warnings to Blocking (TDD)

> _Suggested executor: `swe-rust-dev`_

Promote `complex_diagram` and `subgraph_dense` to blocking, integrated with the Phase 1 exemption
mechanism. Add the flowchart-only non-regression test.

- [ ] [AI] **RED** — Add failing unit tests in `apps/rhino-cli/src/internal/mermaid.rs`:
      (a) a flowchart exceeding both width and depth, with no exemption, produces a blocking
      `complex_diagram` finding and a non-zero result;
      (b) a subgraph exceeding `max-subgraph-nodes`, with no exemption, produces a blocking
      `subgraph_dense` finding;
      (c) the same complex diagram with `allow-complexity` + `reason:` passes;
      (d) **flowchart-only invariant** — a `sequenceDiagram`, an `erDiagram`, and a `gantt` block
      each yield zero findings.
      Run `npx nx run rhino-cli:test:quick` — acceptance: new tests FAIL; preexisting tests pass.
- [ ] [AI] **GREEN** — Update the blocking/exit logic so `complex_diagram` and `subgraph_dense`
      contribute to the non-zero exit in
      `apps/rhino-cli/src/commands/docs_validate_mermaid.rs` (exit condition at ~lines 104-106)
      and the core in `apps/rhino-cli/src/internal/mermaid.rs`; update reporter wording in
      `format_text` so these print as blocking errors, not advisories. Run
      `npx nx run rhino-cli:test:quick` — acceptance: all tests pass.
- [ ] [AI] **REFACTOR** — Consolidate the blocking-kind set so violation/warning promotion is
      expressed in one place; align reporter messages. Run
      `npx nx run rhino-cli:lint && npx nx run rhino-cli:test:quick` — acceptance: both exit 0.

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [ ] [AI] `npx nx run rhino-cli:test:quick` is green (warnings-blocking + flowchart-only
      invariant tests + all preexisting).
- [ ] [AI] `npx nx run rhino-cli:lint` exits 0.

> **Pause Safety**: the validator binary now treats complexity findings as blocking, but the
> **scan scope is still the old `repo-governance/ .claude/`** (Phase 4 not yet applied), so the
> pre-push/CI behavior for the repo at large is unchanged. Safe to stop. To resume:
> `npx nx run rhino-cli:test:quick`.

---

## Phase 3: Off-Palette Color Check + Per-Type Dispatch Matrix (TDD)

> _Suggested executor: `swe-rust-dev`_

Add the new blocking `ViolationKind::OffPaletteColor` (code `off_palette_color`), the per-diagram-
type dispatch matrix (DD-7), and the `allow-color`/`allow-label` rejection rule. The color
allowlist MUST be extracted from `repo-governance/conventions/formatting/diagrams.md` (Accessible
Color Palette, the 8 canonical colors), NOT hardcoded from memory. Color is **never exemptable**.

- [ ] [AI] **RED** — Add failing unit tests in `apps/rhino-cli/src/internal/mermaid.rs` covering:
      (a) a `flowchart` block with `classDef x fill:#0072B2` produces a blocking
      `off_palette_color` violation and a non-zero result;
      (b) a `flowchart` block using only `fill:#0173B2,stroke:#000000,color:#FFFFFF` produces NO
      `off_palette_color` violation;
      (c) a `classDiagram` block with `style A fill:#D55E00` produces a blocking
      `off_palette_color` violation;
      (d) a `classDiagram` with a >30-char label and a wide class arrangement produces NO
      `label_too_long` / `width_exceeded` / `complex_diagram` / `subgraph_dense` finding
      (color-only dispatch);
      (e) a `quadrantChart` with inline `color:#009E73` on a point produces a blocking
      `off_palette_color` violation;
      (f) a `stateDiagram-v2` and a `requirementDiagram` each with an off-palette fill report
      `off_palette_color`;
      (g) a `sequenceDiagram` and a `gantt` block report ZERO findings of any kind;
      (h) the allowlist parsed from `diagrams.md` equals exactly the 8 canonical colors;
      (i) hex normalization — `#0173b2` (lowercase) and `#FFF` (shorthand) compare as on-palette;
      (j) an `allow-color` directive (with a reason) on an off-palette flowchart is REJECTED as an
      invalid/unsupported directive (hard error) AND the `off_palette_color` violation still blocks;
      (k) an `allow-label` directive is REJECTED as invalid.
      Run `npx nx run rhino-cli:test:quick` — acceptance: all new tests FAIL; preexisting tests
      still pass.
- [ ] [AI] **GREEN** — Implement in `apps/rhino-cli/src/internal/mermaid.rs`:
      (1) extract the canonical hex allowlist from `diagrams.md` (parse the Accessible Color
      Palette section, or embed it with a unit test asserting equality to the documented set);
      (2) add `ViolationKind::OffPaletteColor` (code `off_palette_color`) to the enum and `code()`
      mapping (siblings at lines 65-67) and `format_text` reporter wording;
      (3) add a color scanner that, for eligible blocks, finds every `fill:`/`stroke:`/`color:` hex
      (in `classDef`, `style`, `:::`-class defs, and quadrant inline `color:`), normalizes case +
      shorthand, and emits `off_palette_color` for any non-member;
      (4) add a diagram-type classifier (sibling to `flowchart_re()`) recognizing `classDiagram`,
      `stateDiagram`/`stateDiagram-v2`, `requirementDiagram`, `quadrantChart` and route them to a
      **color-only** validate path; keep all structural checks behind the flowchart classifier;
      leave all other types at the count-0 skip;
      (5) extend the DD-2 directive guard so any `allow-*` not in the structural exemptable
      allowlist (`allow-width`, `allow-complexity`, `allow-subgraph-density`) is rejected as an invalid
      directive (hard blocking error).
      Run `npx nx run rhino-cli:test:quick` — acceptance: all tests (new + preexisting) pass.
- [ ] [AI] **REFACTOR** — Extract the palette-allowlist + hex-normalization + type-classifier into
      cohesive helpers; ensure the exemptable-kind allowlist is defined in ONE place shared by the
      directive guard. Run `npx nx run rhino-cli:lint && npx nx run rhino-cli:test:quick`
      — acceptance: both exit 0; no clippy warnings introduced.

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [ ] [AI] `npx nx run rhino-cli:test:quick` is green (off-palette + dispatch-matrix +
      allow-color-rejection tests + all preexisting).
- [ ] [AI] `npx nx run rhino-cli:lint` exits 0.

> **Pause Safety**: the validator binary now also blocks off-palette colors and applies the
> per-type dispatch matrix, but the **scan scope is still the old `repo-governance/ .claude/`**
> (Phase 4 not yet applied), so repo-wide pre-push/CI behavior is unchanged. Safe to stop. To
> resume: `npx nx run rhino-cli:test:quick`.

---

## Phase 3B: Six New Flowchart-Only Checks + Default-Dir Reconciliation (TDD)

> _Suggested executor: `swe-rust-dev`_

Add the six new flowchart-only checks (Amendment A / DD-9) and reconcile
`collect_md_default_dirs()` (Amendment B / DD-8). All six new checks apply to `flowchart`/`graph`
blocks ONLY. Exemptable: `edge_label_too_long` (`allow-edge-label`), `non_lr_direction`
(`allow-direction`), `separator_width_mismatch` (`allow-separator`). Never exemptable:
`literal_backslash_n`, `quotes_in_brackets`, `palette_comment_count` (their `allow-*` directives
are rejected as invalid).

- [ ] [AI] **RED** — Add failing unit tests in `apps/rhino-cli/src/internal/mermaid.rs` covering:
      (a) a `flowchart` node label containing a literal `\n` produces a blocking
      `literal_backslash_n` violation; an `allow-backslash-n` + reason is REJECTED as invalid;
      (b) a `flowchart` node label `[He said "hi"]` produces a blocking `quotes_in_brackets`
      violation; an `allow-quotes` + reason is REJECTED as invalid;
      (c) a `flowchart` edge label >20 chars produces a blocking `edge_label_too_long` violation;
      the same with `allow-edge-label` + `%% reason:` is suppressed;
      (d) a `flowchart TD` block produces a blocking `non_lr_direction` violation; the same with
      `allow-direction` + `%% reason:` is suppressed; a `flowchart LR` block produces none;
      (e) a `flowchart` block with zero palette comments and one with two duplicate palette
      comments each produce a blocking `palette_comment_count` violation; exactly one passes; an
      `allow-palette-comment` + reason is REJECTED as invalid;
      (f) a `flowchart` node with a box-drawing separator whose length does not match its longest
      text line produces a blocking `separator_width_mismatch`; the same with `allow-separator` +
      `%% reason:` is suppressed;
      (g) **flowchart-only invariant** — a `classDiagram` and a `sequenceDiagram` block yield NONE
      of the six new findings;
      (h) **default-dir reconciliation** — `collect_md_default_dirs()` returns files under `apps`
      and `libs` (plus the existing `docs`/`repo-governance`/`.claude`/`plans`) and excludes the
      extended `SKIP_DIRS` (`dist`, `target`, `.opencode`).
      Run `npx nx run rhino-cli:test:quick` — acceptance: all new tests FAIL; preexisting tests
      still pass.
- [ ] [AI] **GREEN** — Implement in `apps/rhino-cli/src/internal/mermaid.rs`:
      (1) add the six new `ViolationKind` variants and their `code()` + `format_text` reporter
      wording (`literal_backslash_n`, `quotes_in_brackets`, `edge_label_too_long`,
      `non_lr_direction`, `palette_comment_count`, `separator_width_mismatch`);
      (2) implement each detector inside the flowchart-only validate path (the `non_lr_direction`
      check reads the direction token already captured by `flowchart_re()`);
      (3) extend the DD-2 exemptable-kind allowlist (single source of truth) to add
      `allow-edge-label`, `allow-direction`, `allow-separator`; ensure `allow-backslash-n`,
      `allow-quotes`, `allow-palette-comment` (and any unknown token) are rejected as invalid;
      and in `apps/rhino-cli/src/commands/docs_validate_mermaid.rs`:
      (4) ensure the six new blocking kinds contribute to the non-zero exit;
      (5) reconcile `collect_md_default_dirs()` (line ~186) to add `"apps"` and `"libs"` to the dir
      list, and extend `SKIP_DIRS` (line ~46) to add `"dist"`, `"target"`, `".opencode"`.
      Run `npx nx run rhino-cli:test:quick` — acceptance: all tests (new + preexisting) pass.
- [ ] [AI] **REFACTOR** — Extract the six detectors into cohesive helpers; keep the exemptable-kind
      allowlist defined in ONE place shared by the directive guard. Run
      `npx nx run rhino-cli:lint && npx nx run rhino-cli:test:quick` — acceptance: both exit 0; no
      clippy warnings introduced.

### Phase 3B Gate

> All checks below must pass before starting Phase 4.

- [ ] [AI] `npx nx run rhino-cli:test:quick` is green (six-new-checks + flowchart-only invariant +
      default-dir reconciliation tests + all preexisting).
- [ ] [AI] `npx nx run rhino-cli:lint` exits 0.

> **Pause Safety**: the validator binary now also blocks the six new kinds and the default-dir
> list covers `apps`/`libs`, but the **wired Nx-target scan scope is still the old
> `repo-governance/ .claude/`** (Phase 4 not yet applied), so repo-wide pre-push/CI behavior is
> unchanged. Safe to stop. To resume: `npx nx run rhino-cli:test:quick`.

---

## Phase 4: Widen Scan Scope + Triple-Layer Enforcement (Nx target + pre-push hook + PR/push CI)

> _Suggested executor: `swe-rust-dev`_
>
> This phase wires all THREE enforcement layers (DD-4): Layer 1 = pre-push hook (any `*.md`);
> Layer 2 = dedicated `pr-validate-mermaid.yml` on `pull_request` to `main`; Layer 3 = the same
> workflow on `push` to `main`.

- [ ] [AI] Edit `apps/rhino-cli/project.json` (`validate:mermaid`, ~line 167): change the command
      positional paths from `repo-governance/ .claude/` to
      `repo-governance/ .claude/ plans/ docs/ apps/ libs/ AGENTS.md CLAUDE.md README.md`, and
      widen `inputs` to add `{workspaceRoot}/plans/**/*.md`, `{workspaceRoot}/docs/**/*.md`,
      `{workspaceRoot}/apps/**/*.md`, `{workspaceRoot}/libs/**/*.md`, `{workspaceRoot}/AGENTS.md`,
      `{workspaceRoot}/CLAUDE.md`, `{workspaceRoot}/README.md`.
      Verify: `npx nx run rhino-cli:validate:mermaid` now scans the widened set — acceptance: the
      command's reported `files_scanned` count is markedly higher than the Phase 0 baseline
      (exact number recorded in notes); the command may FAIL here because the fix-all phases have
      not run yet — that failure is expected and resolved in Phases 5-11.
- [ ] [AI] **Layer 1 (pre-push)** — Edit `.husky/pre-push` (mermaid trigger, line ~22): change the
      trigger regex from `'^(repo-governance/|\.claude/).*\.md$'` to `'\.md$'` so the gate fires on
      ANY markdown change. Verify by inspection — acceptance: the `grep -qE` for the mermaid block
      matches any `*.md` path; the conditional still calls `npx nx run rhino-cli:validate:mermaid`.
- [ ] [AI] **Layers 2 & 3 (PR + push CI)** — Create the NEW file
      `.github/workflows/pr-validate-mermaid.yml`, MIRRORING the structure of the existing
      `.github/workflows/pr-validate-links.yml` (read it first for grounding: `actions/checkout@v6`
      → `./.github/actions/setup-rust`, `ubuntu-latest`, single job running a rhino-cli docs
      validator). Differences from the links template:
  - `on:` block has BOTH triggers (mirroring `.github/workflows/crane-cli-integration.yml`):

    ```yaml
    on:
      pull_request:
        branches: [main]
      push:
        branches: [main]
    ```

  - the single job runs `npx nx run rhino-cli:validate:mermaid` (the widened full-repo target), so
    add `./.github/actions/setup-node` before `./.github/actions/setup-rust` (the target is invoked
    via `nx`), and check out with `fetch-depth: 0`.
    Verify: `npx prettier --check .github/workflows/pr-validate-mermaid.yml` exits 0; if
    `actionlint` and/or `yamllint` are available on PATH, run
    `actionlint .github/workflows/pr-validate-mermaid.yml` and
    `yamllint .github/workflows/pr-validate-mermaid.yml` and confirm clean (skip gracefully if
    not installed) — acceptance: the file exists; prettier passes; `actionlint`/`yamllint` are
    clean when available; the `on:` block contains BOTH `pull_request: branches: [main]` AND
    `push: branches: [main]`; the job invokes `npx nx run rhino-cli:validate:mermaid`.

- [ ] [AI+HUMAN] **Behavioral acceptance (observed at execution)** — Confirm a deliberately-malformed
      diagram makes the PR check FAIL. This is a manual/observed verification performed against a
      real GitHub Actions run (it requires an actual PR/push event, so it cannot be fully simulated
      locally): on a throwaway branch or scratch commit, introduce one off-palette `fill:#0072B2`
      (or a `flowchart TD`) into a markdown file, open a PR to `main` (or push to `main` on a fork
      where safe), and observe the `pr-validate-mermaid` workflow run go RED; then revert the
      scratch change. Acceptance: the `pr-validate-mermaid` check reports failure on the malformed
      diagram and passes once reverted. (Agent prepares the scratch diagram + PR; human confirms the
      observed CI result and authorizes the throwaway push if a real event is required.)

### Phase 4 Gate

> All checks below must pass before starting Phase 5. The validator is EXPECTED to report
> findings here (the fix-all has not run) — that is acceptable for this gate; what must hold is
> that the wiring is correct across all three layers.

- [ ] [AI] `npx nx run rhino-cli:validate:mermaid` executes against the widened scope (confirmed
      by `files_scanned` >> baseline), regardless of pass/fail.
- [ ] [AI] `.husky/pre-push` mermaid trigger matches any `*.md` (Layer 1, inspection).
- [ ] [AI] `.github/workflows/pr-validate-mermaid.yml` exists;
      `npx prettier --check .github/workflows/pr-validate-mermaid.yml` exits 0; the `on:` block has
      BOTH `pull_request: branches: [main]` (Layer 2) AND `push: branches: [main]` (Layer 3); the
      job invokes `npx nx run rhino-cli:validate:mermaid` (inspection; `actionlint`/`yamllint` clean
      when available).

> **Pause Safety**: wiring is in place but the repo has known diagram findings — do NOT push from
> here, because pre-push would now block on the unfixed backlog. This is a coherent **local**
> stopping point (no half-edited files). To resume:
> `npx nx run rhino-cli:validate:mermaid` and proceed to per-tree cleanup.

---

## Per-Tree Fix-All Phases (gated)

> For EACH tree below: re-measure with the expanded validator (warnings now blocking,
> `off_palette_color` active, AND the six new checks active), then for every blocking finding apply
> ONE of — (1) shorten labels to ≤30 chars; (2) restructure the over-wide flowchart per the
> width-fix strategies in `repo-governance/conventions/formatting/diagrams.md` (LR orientation,
> subgraph splits); (3) for a genuinely inherently-complex diagram, add a justified
> `%% validate-mermaid: allow-<kind>` + `%% reason: <text>` exemption (exemptable kinds:
> `allow-width`, `allow-complexity`, `allow-subgraph-density`, `allow-edge-label`,
> `allow-direction`, `allow-separator`); (4) for an `off_palette_color` finding, **replace** the
> off-palette hex with the nearest canonical palette color
> (`#0173B2 #DE8F05 #029E73 #CC78BC #CA9161 #000000 #FFFFFF #808080`) across ALL five eligible
> diagram types; (5) for a `non_lr_direction` finding, **convert** the flowchart to `LR` (preferred)
> or add a justified `allow-direction` + reason (expected the LARGEST single contributor);
> (6) for a `literal_backslash_n` finding, replace `\n` with `<br/>` (node labels) or shorten
> (edge labels); (7) for a `quotes_in_brackets` finding, remove the quote or rephrase the label;
> (8) for an `edge_label_too_long` finding, shorten to ≤20 chars or add `allow-edge-label` + reason;
> (9) for a `palette_comment_count` finding, ensure exactly one palette comment; (10) for a
> `separator_width_mismatch` finding, match the separator to the longest text line or add
> `allow-separator` + reason. NEVER exempt `label_too_long`, `multiple_diagrams`, `off_palette_color`,
> `literal_backslash_n`, `quotes_in_brackets`, or `palette_comment_count` (these are non-exemptable).
> Re-measure each tree at execution — do NOT rely on authoring-time counts.
> _Suggested executor per tree: `swe-rust-dev` for `apps/`/`libs/` (code-adjacent diagrams);
> otherwise the content-owning agent for that tree, or a generic edit for simple label shortening._

### Phase 5: Fix-all `repo-governance/`

- [ ] [AI] Re-measure (structural + color): `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-mermaid --output json repo-governance/`
      — acceptance: per-finding list recorded (note any `off_palette_color`).
- [ ] [AI] Resolve every blocking finding in `repo-governance/` (shorten / restructure / exempt
      structural with reason; replace off-palette hex with canonical palette) — acceptance: each
      finding addressed, including zero `off_palette_color`.

### Phase 5 Gate

- [ ] [AI] `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-mermaid repo-governance/`
      exits 0 (zero blocking findings — all kinds: structural, off-palette color, and the six new checks — for this tree).

> **Pause Safety**: `repo-governance/` is clean under the new rules; other trees may still have
> findings (don't push yet). Safe to stop. To resume: re-run the Phase 5 gate command.

### Phase 6: Fix-all `.claude/`

- [ ] [AI] Re-measure (structural + color): `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-mermaid --output json .claude/`
      — acceptance: per-finding list recorded (note any `off_palette_color`).
- [ ] [AI] Resolve every blocking finding in `.claude/` (including off-palette colors) — acceptance:
      each finding addressed; zero `off_palette_color`.

### Phase 6 Gate

- [ ] [AI] `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-mermaid .claude/`
      exits 0 (all kinds: structural, off-palette color, and the six new checks).

> **Pause Safety**: `.claude/` clean; remaining trees pending. Safe to stop. To resume: re-run the
> Phase 6 gate command.

### Phase 7: Fix-all `plans/` (includes the trigger case)

- [ ] [AI] Re-measure: `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-mermaid --output json plans/`
      — acceptance: per-finding list recorded.
- [ ] [AI] Fix the 7 over-long labels in
      `plans/in-progress/gherkin-step-keyword-cardinality/README.md` (the diagram that started
      this) — acceptance: those labels are ≤30 chars and the file passes.
- [ ] [AI] Resolve every other blocking finding in `plans/` (including off-palette colors) —
      acceptance: each addressed. Confirm THIS plan's own diagrams pass (dogfooding) — acceptance:
      every diagram across this plan's five docs reports zero findings (0 structural / 0
      `off_palette_color`).

### Phase 7 Gate

- [ ] [AI] `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-mermaid plans/`
      exits 0 (all kinds: structural, off-palette color, and the six new checks; includes this plan and the gherkin-cardinality README).

> **Pause Safety**: `plans/` clean; `docs/`, `apps/`, `libs/`, root pending. Safe to stop. To
> resume: re-run the Phase 7 gate command.

### Phase 8: Fix-all `docs/`

- [ ] [AI] Re-measure (structural + color): `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-mermaid --output json docs/`
      — acceptance: per-finding list recorded (note any `off_palette_color`).
  - _Suggested executor: `docs-maker` for content-bearing diagram edits._
- [ ] [AI] Resolve every blocking finding in `docs/` (including off-palette colors) — acceptance:
      each addressed; zero `off_palette_color`.

### Phase 8 Gate

- [ ] [AI] `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-mermaid docs/`
      exits 0 (all kinds: structural, off-palette color, and the six new checks).

> **Pause Safety**: `docs/` clean; `apps/`, `libs/`, root pending. Safe to stop. To resume: re-run
> the Phase 8 gate command.

### Phase 9: Fix-all `apps/`

> This tree is the largest backlog (authoring-time estimate ≈ 529 structural violation-lines across
> `apps/`+`libs/`, EXCLUDING soon-to-be-blocking warnings AND off-palette colors — real count is
> HIGHER and must be re-measured with both warnings AND the color check on). _Suggested executor:
> `swe-rust-dev` (diagrams in app READMEs are code-adjacent)._

- [ ] [AI] Re-measure (structural + color): `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-mermaid --output json apps/`
      — acceptance: per-finding list recorded; total count noted (note any `off_palette_color`).
- [ ] [AI] Resolve every blocking finding in `apps/` — acceptance: each addressed (prefer label
      shortening and LR/subgraph restructuring; replace off-palette hex with canonical palette;
      reserve structural exemptions for genuinely complex diagrams).

### Phase 9 Gate

- [ ] [AI] `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-mermaid apps/`
      exits 0 (all kinds: structural, off-palette color, and the six new checks).

> **Pause Safety**: `apps/` clean; `libs/` and root pending. Safe to stop. To resume: re-run the
> Phase 9 gate command.

### Phase 10: Fix-all `libs/`

- [ ] [AI] Re-measure (structural + color): `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-mermaid --output json libs/`
      — acceptance: per-finding list recorded (note any `off_palette_color`).
- [ ] [AI] Resolve every blocking finding in `libs/` (including off-palette colors) — acceptance:
      each addressed; zero `off_palette_color`.

### Phase 10 Gate

- [ ] [AI] `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-mermaid libs/`
      exits 0 (all kinds: structural, off-palette color, and the six new checks).

> **Pause Safety**: `libs/` clean; only root instruction files pending. Safe to stop. To resume:
> re-run the Phase 10 gate command.

### Phase 11: Fix-all root instruction files

> Authoring-time estimate: 0 structural violation-lines in `AGENTS.md` / `CLAUDE.md` / root
> `README.md` (label/width/multiple only; warnings + color excluded). Re-measure to confirm with
> blocking warnings AND the color check on.

- [ ] [AI] Re-measure (structural + color): `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-mermaid --output json AGENTS.md CLAUDE.md README.md`
      — acceptance: per-finding list recorded.
- [ ] [AI] Resolve every blocking finding in the root files (including off-palette colors) —
      acceptance: each addressed (likely none).

### Phase 11 Gate

- [ ] [AI] `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-mermaid AGENTS.md CLAUDE.md README.md`
      exits 0 (all kinds: structural, off-palette color, and the six new checks).

> **Pause Safety**: all trees individually clean. The full-repo gate should now pass. Safe to
> stop. To resume: `npx nx run rhino-cli:validate:mermaid`.

---

## Phase 12: Update `diagrams.md` (convention accuracy)

> _Suggested executor: `repo-rules-maker` (preferred — governance convention) or `docs-maker`._

- [ ] [AI] Edit `repo-governance/conventions/formatting/diagrams.md`: correct the coverage claim
      (~line 438) to list the actual scanned trees (`repo-governance/`, `.claude/`, `plans/`,
      `docs/`, `apps/`, `libs/`, root `AGENTS.md`/`CLAUDE.md`/`README.md`; note `.opencode/`
      excluded as generated, `.next`/`node_modules`/`.git` skipped) — acceptance: the claim
      matches the Phase 4 target command.
- [ ] [AI] Document the **triple-layer enforcement** in `diagrams.md`: the gate runs in (Layer 1)
      the `.husky/pre-push` hook on any `*.md` change; (Layer 2) the dedicated
      `.github/workflows/pr-validate-mermaid.yml` workflow on `pull_request` to `main` (blocks PRs);
      and (Layer 3) the same workflow on `push` to `main` (gates direct trunk pushes) — acceptance:
      all three layers and the dual `pull_request`/`push` trigger are described.
- [ ] [AI] Document the exemption directive in `diagrams.md`: syntax
      (`%% validate-mermaid: allow-<kind>` + mandatory `%% reason:`), exemptable kinds
      (`width_exceeded`→`allow-width`, `complex_diagram`→`allow-complexity`,
      `subgraph_dense`→`allow-subgraph-density`, `edge_label_too_long`→`allow-edge-label`,
      `non_lr_direction`→`allow-direction`, `separator_width_mismatch`→`allow-separator`),
      never-exemptable kinds (`label_too_long`, `multiple_diagrams`, `off_palette_color`,
      `literal_backslash_n`, `quotes_in_brackets`, `palette_comment_count`), the flowchart-only
      scope, that an `allow-*` without a `reason:` is a hard error, and that any `allow-*` naming a
      non-exemptable kind (e.g. `allow-color`, `allow-label`, `allow-quotes`) is rejected as an
      invalid directive — acceptance: all facts present.
- [ ] [AI] Document the **six new checks** in `diagrams.md`: `literal_backslash_n` (no literal
      `\n`; use `<br/>`), `quotes_in_brackets` (no `"` inside `[...]`), `edge_label_too_long`
      (edge labels ≤20 chars, exemptable), `non_lr_direction` (`flowchart LR` default, exemptable),
      `palette_comment_count` (exactly one palette comment, never exemptable),
      `separator_width_mismatch` (separator length matches longest text line ≤20, exemptable); state
      that all six are **flowchart-only** (extending the bracket-label checks to other diagram types
      is future work) — acceptance: all six checks documented with their blocking/exemptable status
      and flowchart-only scope.
  - _Suggested executor: `repo-rules-maker` (governance convention)._
- [ ] [AI] Document the **palette-enforcement scope** in `diagrams.md`: the new blocking
      `off_palette_color` check validates every `fill:`/`stroke:`/`color:` hex against the canonical
      palette (which `diagrams.md` itself defines — it is the source of truth); the check applies to
      the **five eligible diagram types** (flowchart/graph, classDiagram, stateDiagram/v2,
      requirementDiagram, quadrantChart); non-flowchart eligible types receive **color-only**
      validation (no label/width/structural checks — those stay flowchart-only); `erDiagram` and C4
      color are deferred; theme-only types (sequenceDiagram, gantt, pie, gitGraph, journey,
      timeline, mindmap) are unchecked; and color is **never exemptable** — acceptance: all five
      facts present.
- [ ] [AI] Document that `complex_diagram` and `subgraph_dense` are now **blocking** (no longer
      advisory) — acceptance: the convention states the blocking behavior.
  - _Suggested executor: `repo-rules-maker` (governance convention)._
- [ ] [AI] Verify `diagrams.md` itself passes the expanded validator:
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-mermaid repo-governance/conventions/formatting/diagrams.md`
      — acceptance: exits 0 (all kinds: structural, off-palette color, and the six new checks).

### Phase 12 Gate

- [ ] [AI] `npm run lint:md` passes for `diagrams.md`.
- [ ] [AI] All documented facts (coverage, exemption directive with the full token vocabulary,
      structural flowchart-only scope, warnings-blocking, the five color-eligible types, the
      non-exemptable `off_palette_color` check, palette source-of-truth, AND the six new checks
      with their blocking/exemptable status and flowchart-only scope) are present in `diagrams.md`
      (review).

> **Pause Safety**: governance docs now match the tool. Safe to stop. To resume:
> `npx nx run rhino-cli:validate:mermaid`.

---

## Phase 13: Full-Repo Verification, Quality Gates, Push, CI, Archival

### Local Quality Gates (Before Push)

- [ ] [AI] Run the full expanded gate: `npx nx run rhino-cli:validate:mermaid`
      — acceptance: exits 0 (zero blocking findings repo-wide — structural AND `off_palette_color`).
- [ ] [AI] Run affected typecheck: `npx nx affected -t typecheck` — acceptance: exits 0.
- [ ] [AI] Run affected linting: `npx nx affected -t lint` — acceptance: exits 0.
- [ ] [AI] Run affected quick tests: `npx nx affected -t test:quick` — acceptance: exits 0.
- [ ] [AI] Run affected spec coverage: `npx nx affected -t spec-coverage` — acceptance: exits 0.
- [ ] [AI] Run markdown lint: `npm run lint:md` — acceptance: exits 0.
- [ ] [AI] Fix ALL failures — including preexisting issues not caused by these changes — and
      re-run the failing checks to confirm resolution. Verify zero failures before pushing.

### Commit Guidelines

- [ ] [AI] Commit changes thematically (Conventional Commits `<type>(<scope>): <description>`),
      split by concern, for example:
  - `feat(rhino-cli): add inline mermaid exemption directive`
  - `feat(rhino-cli): promote complex_diagram and subgraph_dense to blocking`
  - `feat(rhino-cli): add off-palette color check with per-type dispatch matrix`
  - `feat(rhino-cli): add six flowchart checks (\n, quotes, edge-label, direction, palette-comment, separator)`
  - `fix(rhino-cli): reconcile validate-mermaid default dirs to include apps and libs`
  - `feat(rhino-cli): widen validate:mermaid scan to all markdown trees`
  - `feat(husky): fire mermaid gate on any markdown change in pre-push`
  - `ci: add dedicated pr-validate-mermaid workflow (PR + push-to-main triggers)`
  - `fix(<scope>): clean mermaid violations in <tree>` (one per tree as appropriate)
  - `docs(governance): correct mermaid coverage and document exemption + color check`
  - Preexisting fixes get their own separate commits.
    — acceptance: no unrelated changes bundled into a single commit.

### Push and Post-Push CI Verification

- [ ] [AI] Push directly to `main`: `git push origin main`
      — acceptance: push succeeds (pre-push hook green, including the now-widened mermaid gate).
- [ ] [AI] Monitor ALL GitHub Actions workflows triggered by the push (poll every 3 minutes; one
      `gh run view --json status,conclusion` per wakeup; do NOT use `gh run watch`)
      — acceptance: every workflow run is observed to completion, INCLUDING the new
      `pr-validate-mermaid` workflow (Layer 3 fires on this `push` to `main`).
- [ ] [AI] Verify the new `pr-validate-mermaid` workflow run (push-to-main trigger) runs and passes,
      and ALL other CI checks pass — acceptance: zero failures; the `pr-validate-mermaid` run is
      green.
- [ ] [AI] If any CI check fails, investigate the root cause, fix, and push a follow-up commit;
      repeat until ALL GitHub Actions are green — acceptance: full CI green.

### Plan Archival

- [ ] [AI] Verify ALL delivery checklist items are ticked.
- [ ] [AI] Verify ALL quality gates pass (local + CI).
- [ ] [AI] Verify the full-repo `validate:mermaid` reports zero blocking findings (structural AND
      `off_palette_color`).
- [ ] [AI] Move:
      `git mv plans/in-progress/mermaid-gate-coverage-expansion plans/done/2026-06-06__mermaid-gate-coverage-expansion`
      (use the actual completion date, NOT the creation date).
- [ ] [AI] Update `plans/in-progress/README.md` — remove the plan entry.
- [ ] [AI] Update `plans/done/README.md` — add the plan entry with completion date.
- [ ] [AI] Update any other READMEs that reference this plan (e.g. `plans/README.md`).
- [ ] [AI] Commit the archival: `chore(plans): move mermaid-gate-coverage-expansion to done`, then
      push to `origin main`.

### Phase 13 Gate

> All checks below must pass — this is the final gate.

- [ ] [AI] `npx nx run rhino-cli:validate:mermaid` exits 0 (full repo clean — all kinds: structural, off-palette color, and the six new checks).
- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` exits 0 and `npm run lint:md`
      passes.
- [ ] [AI] All GitHub Actions for the push are green, including the new `pr-validate-mermaid`
      workflow run (push-to-main trigger).
- [ ] [AI] Plan archived to `plans/done/` and READMEs updated.

> **Pause Safety**: work is complete, pushed, CI green, plan archived. This is the terminal state.
> To re-verify at any later time: `npx nx run rhino-cli:validate:mermaid`.
