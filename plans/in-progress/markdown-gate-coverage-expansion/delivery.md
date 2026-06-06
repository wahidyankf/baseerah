# Delivery Checklist — Markdown Gate Coverage Expansion

> **Legend** — `[AI]`: an agent performs the step (the default; unmarked steps are `[AI]`).
> `[HUMAN]`: only a human can do it (physical action, out-of-band approval, real-secret or
> privileged-credential handling). `[AI+HUMAN]`: agent prepares, human approves or finishes.
>
> **Phase Gate** — every phase ends with a `### Phase N Gate` (must-pass verification) plus a
> `> **Pause Safety**:` note (the safe-to-stop state and the single command to resume). A phase
> is not complete until its gate is green; do not start phase N+1 while any gate check fails.

## Worktree

Worktree path: `worktrees/markdown-gate-coverage-expansion/`

Provision before execution (run from repo root):

```bash
claude --worktree markdown-gate-coverage-expansion
```

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md)
and
[Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

## Push / Definition of Done

- **Push target**: `origin main`, **direct** (Trunk Based Development — no PR). [Repo-grounded —
  `main` is the trunk]
- **DoD**: all three markdown gates report zero blocking findings within their scopes (mermaid
  repo-wide−exclusions; links repo-wide−exclusions with anchors validated; heading-hierarchy on the
  prose allowlist); the gates are enforced across all THREE layers — pre-commit staged-only
  (Layer 1), the consolidated `validate-markdown.yml` on `pull_request` to `main` (Layer 2), and
  the same workflow on `push` to `main` (Layer 3); the mermaid trigger is removed from
  `.husky/pre-push`; `pr-validate-links.yml` is deleted and migrated; all existing `links.rs` /
  `heading_hierarchy.rs` / `mermaid.rs` unit tests stay green; new behavior (link `--exclude`,
  repo-wide scan, `broken-anchor` anchor validation, shared heading parser, heading prose allowlist
  - `--exclude`, staged-only pre-commit steps) is fully tested; `diagrams.md` / `quality.md` /
    `linking.md` / check-inventory docs are accurate; this plan's own diagrams, links, anchors, and
    prose headings pass; the plan is archived to `plans/done/`.

> **Important (fix-all-issues)**: Fix ALL failures found during quality gates, not just those
> caused by your changes. This follows the root-cause-orientation principle — proactively fix
> preexisting errors encountered during work. Do not defer or skip existing issues. Commit
> preexisting fixes separately with appropriate conventional commit messages.

---

## Phase 0: Environment Setup and Baseline

> _Executor: repo-setup-manager_

- [ ] [AI] Provision the worktree from repo root: `claude --worktree markdown-gate-coverage-expansion`
      — acceptance: `worktrees/markdown-gate-coverage-expansion/` exists.
- [ ] [AI] Initialize the toolchain in the **root** worktree: `npm install && npm run doctor -- --fix`
      — acceptance: both exit 0; `node_modules/` synchronized; no unresolved toolchain drift.
- [ ] [AI] Build rhino-cli to confirm all three validators compile:
      `cargo build --release --quiet --manifest-path apps/rhino-cli/Cargo.toml`
      — acceptance: exits 0.
- [ ] [AI] Capture the **mermaid** baseline (current scope):
      `npx nx run rhino-cli:validate:mermaid`
      — acceptance: record pass/fail + findings to phase notes.
- [ ] [AI] Capture the **current link** baseline (current 3-dir scope):
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-links --output json`
      — acceptance: record `total_files`, `broken_count`, and any broken links to phase notes.
- [ ] [AI] Establish a **provisional repo-wide link backlog** with the CURRENT binary (3-dir scope
      only) by separately grepping for relative markdown links in the not-yet-scanned trees, so the
      eventual full-scan backlog is not a surprise. Run:
      `grep -rnoE '\]\([^)#][^)]*\.md(#[^)]*)?\)' plans/ apps/ libs/ AGENTS.md CLAUDE.md README.md --include='*.md' | head -100`
      — acceptance: a provisional per-tree list of relative links (with `#anchor` ones flagged)
      recorded in phase notes. Estimate only; authoritative backlog is re-measured per tree once
      the widened link checker + anchor validation land (Phase 1).
- [ ] [AI] Establish a **provisional prose-heading backlog** with the CURRENT heading validator,
      scoped to the prose allowlist, to gauge Phase 3's fix-all:
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-heading-hierarchy --output json docs/ repo-governance/ plans/`
      — acceptance: per-finding list recorded in phase notes. Note: this includes `plans/done/`
      (the allowlist-minus-done filter does not exist yet); discount `plans/done/` findings.
- [ ] [AI] Confirm which `.claude/`/`.opencode/`/`SKILL.md` files have ≠1 H1 (the files the prose
      allowlist MUST protect), for the Phase 2 allowlist test fixtures:
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-heading-hierarchy --output json .claude/ .opencode/ | head -60`
      — acceptance: record that these trees produce findings TODAY (proof the allowlist is needed).
- [ ] [AI] Run the existing rhino-cli unit tests to establish the green baseline:
      `npx nx run rhino-cli:test:quick`
      — acceptance: baseline pass count recorded; all preexisting failures documented.
- [ ] [AI] Resolve all preexisting failures before proceeding
      — acceptance: no preexisting failures remain unresolved.

### Phase 0 Gate

> All checks below must pass before starting Phase 1.

- [ ] [AI] `cargo build --release ... apps/rhino-cli/Cargo.toml` exits 0.
- [ ] [AI] `npx nx run rhino-cli:test:quick` is green; baseline recorded.
- [ ] [AI] Provisional per-tree link, anchor, and prose-heading backlogs recorded in phase notes.

> **Pause Safety**: only the toolchain was verified and baselines recorded — no source changed.
> Safe to stop indefinitely. To resume: re-run `npx nx run rhino-cli:test:quick` and confirm it is
> still green.

**Phase 0 notes** (executor fills in): _baseline results, per-tree provisional counts._

---

## Phase 1: Link Checker — `--exclude`, Repo-Wide Scan, Anchor Validation (TDD)

> _Suggested executor: `swe-rust-dev`_

Implement DD-2 (`--exclude` flag), DD-3 (repo-wide walk minus noise dirs), DD-5 (shared fence-aware
heading parser), and DD-4 (`broken-anchor` anchor validation via GitHub-slugify). The
`.claude/skills/` hard-skip stays.

- [ ] [AI] **RED** — Add failing unit tests in `apps/rhino-cli/src/internal/docs/links.rs` and
      `apps/rhino-cli/src/commands/docs_validate_links.rs` covering:
      (a) `--exclude plans/done` removes a broken link under `plans/done` from results while a
      broken link elsewhere is still reported;
      (b) a repo-wide scan finds a broken link under `libs/` (not in today's 3-dir set) and skips
      a file under `node_modules/`;
      (c) a link `[X](./target.md#missing-section)` where `target.md` exists but has no heading
      slugging to `missing-section` yields a `broken-anchor` finding;
      (d) a link `[X](./target.md#real-section)` where `target.md` has `## Real Section` yields NO
      anchor finding;
      (e) the slug helper maps duplicate `Setup` headings to `setup` and `setup-1`;
      (f) a same-file anchor `[Y](#own-section)` with no matching heading yields a `broken-anchor`.
      Run `npx nx run rhino-cli:test:quick` — acceptance: all new tests FAIL; all preexisting
      `links.rs` tests still pass.
- [ ] [AI] **GREEN** — Implement in `apps/rhino-cli/src/internal/docs/links.rs` and
      `apps/rhino-cli/src/commands/docs_validate_links.rs`:
      (1) add `--exclude` (`#[arg(long = "exclude")] pub exclude: Vec<String>`) and thread into
      `ScanOptions.skip_paths` (replace the hardcoded `Vec::new()` at `docs_validate_links.rs:37`);
      (2) change `get_all_markdown_files` to a whole-repo `walkdir` walk with a `filter_entry` that
      drops noise-skip dirs (`node_modules, dist, target, .next, coverage, generated-reports,
local-temp, archived, apps-labs`, `.git`);
      (3) **Prerequisite — remove the pure-anchor skip in `extract_links`**: `links.rs:245`
      currently has `|| url.starts_with('#')` which causes `extract_links` to skip all
      `[text](#fragment)` links before they reach any validator. Remove (or condition) this skip so
      that same-file anchor links are extracted and passed through the anchor validation logic —
      without this change, scenario (f) above can never fail in RED and the same-file anchor
      acceptance criterion in DD-4 / prd.md scenario 7 is untestable.
      Then add a GitHub-slug helper (lowercase, strip non-alnum-except-hyphen, spaces→hyphens, `-N`
      collisions) + an anchor check that captures the `#fragment` before `resolve_link` strips it,
      parses the target file's headings via the shared parser (step 4), and emits a `BrokenLink`
      with `category = "broken-anchor"` when the slug is absent;
      (4) **shared parser (DD-5)** — extract heading parsing into a reusable
      `pub(crate)` helper consumed by both modules, WITHOUT changing `heading_hierarchy.rs`
      behavior.
      Run `npx nx run rhino-cli:test:quick` — acceptance: all tests (new + preexisting, including
      `heading_hierarchy.rs`) pass.
- [ ] [AI] **REFACTOR** — Consolidate the slug + anchor + walk helpers; keep the shared heading
      parser in one place. Run `npx nx run rhino-cli:lint && npx nx run rhino-cli:test:quick`
      — acceptance: both exit 0; no clippy warnings introduced.

### Phase 1 Gate

> All checks below must pass before starting Phase 2.

- [ ] [AI] `npx nx run rhino-cli:test:quick` is green (new link/anchor/exclude tests + all
      preexisting `links.rs` and `heading_hierarchy.rs` tests).
- [ ] [AI] `npx nx run rhino-cli:lint` exits 0.

> **Pause Safety**: the link checker binary now supports `--exclude`, repo-wide scan, and anchor
> validation, but it is NOT yet wired into any hook/CI (Phase 4) — repo enforcement is unchanged.
> Safe to stop. To resume: `npx nx run rhino-cli:test:quick`.

---

## Phase 2: Heading-Hierarchy — Prose Allowlist + `--exclude` (TDD)

> _Suggested executor: `swe-rust-dev`_

Implement DD-6: a `is_prose_allowlisted` predicate inside the validator file selection
(`docs/`, `repo-governance/`, `plans/`−`done/`, root `*.md`; default-deny everything else) plus a
repeatable `--exclude` flag.

- [ ] [AI] **RED** — Add failing unit tests in
      `apps/rhino-cli/src/internal/docs/heading_hierarchy.rs` and
      `apps/rhino-cli/src/commands/docs_validate_heading_hierarchy.rs` covering:
      (a) a `docs/` file with two H1s yields a `duplicate-h1` finding (allowlist runs);
      (b) a `.claude/agents/` file with zero H1s yields NO finding (default-deny);
      (c) a `SKILL.md` under `.claude/skills/` with many H1s yields NO finding;
      (d) a `plans/done/` file with a skipped level yields NO finding (`done` excluded);
      (e) a `plans/in-progress/` file with a duplicate H1 yields a finding (in allowlist);
      (f) an `apps/example/README.md` with a skipped level yields NO finding (`apps` excluded);
      (g) `--exclude docs` suppresses findings in the `docs` tree while other allowlist trees still
      report.
      Run `npx nx run rhino-cli:test:quick` — acceptance: new tests FAIL; preexisting
      `heading_hierarchy.rs` tests still pass.
- [ ] [AI] **GREEN** — Implement in
      `apps/rhino-cli/src/internal/docs/heading_hierarchy.rs`:
      (1) add `is_prose_allowlisted(repo_rel: &str) -> bool` returning true only for the allowlist
      trees (and false for `plans/done/`);
      (2) apply the predicate to every candidate file in `walk_heading_hierarchy_path` (compute
      each entry's repo-relative path);
      and in `apps/rhino-cli/src/commands/docs_validate_heading_hierarchy.rs`:
      (3) add `--exclude` (`Vec<String>`) and subtract excluded prefixes AFTER the allowlist;
      (4) ensure the allowlist applies even when positional paths or staged files are passed.
      Run `npx nx run rhino-cli:test:quick` — acceptance: all tests (new + preexisting) pass.
- [ ] [AI] **REFACTOR** — Keep the allowlist + exclude logic in one cohesive place; align doc
      comments with module style. Run `npx nx run rhino-cli:lint && npx nx run rhino-cli:test:quick`
      — acceptance: both exit 0; no clippy warnings introduced.

### Phase 2 Gate

> All checks below must pass before starting Phase 3.

- [ ] [AI] `npx nx run rhino-cli:test:quick` is green (allowlist + exclude tests + all preexisting).
- [ ] [AI] `npx nx run rhino-cli:lint` exits 0.
- [ ] [AI] Spot-check: `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-heading-hierarchy .claude/`
      reports ZERO findings (allowlist protects agent/skill files) — acceptance: exits 0.

> **Pause Safety**: the heading validator now self-scopes to prose and protects agent/skill files,
> but it is NOT yet wired into any hook/CI (Phase 4). Safe to stop. To resume:
> `npx nx run rhino-cli:test:quick`.

---

## Phase 3: Pre-Commit Staged-Only Steps (Mermaid + Heading) (TDD)

> _Suggested executor: `swe-rust-dev`_

Implement DD-7: add staged-only mermaid + heading steps to the `rhino-cli git pre-commit` suite in
`apps/rhino-cli/src/internal/git.rs`, mirroring the existing `step7_validate_links`. The heading
step applies the prose allowlist (Phase 2). The link step already exists — extend it to pass the
three `--exclude` named exclusions.

- [ ] [AI] **RED** — Add failing unit tests in `apps/rhino-cli/src/internal/git.rs` covering:
      (a) a staged `*.md` with a malformed flowchart makes the new mermaid step return an error;
      (b) a staged `docs/` file with a duplicate H1 makes the new heading step return an error;
      (c) a staged `SKILL.md` (under `.claude/skills/`) with many H1s makes the heading step
      return OK (allowlist protects it);
      (d) the existing link step now excludes a staged `plans/done/` broken link.
      Run `npx nx run rhino-cli:test:quick` — acceptance: new tests FAIL; preexisting pre-commit
      tests still pass.
- [ ] [AI] **GREEN** — Implement in `apps/rhino-cli/src/internal/git.rs`:
      (1) add a staged-only mermaid step (collect staged `*.md`, run the mermaid validator over
      them minus the 3 named exclusions + noise dirs, block on findings);
      (2) add a staged-only heading step (collect staged `*.md`, keep only
      `is_prose_allowlisted` survivors, run the heading validator, block on findings);
      (3) extend `step7_validate_links` to pass the three named exclusions via `skip_paths` — the
      current call at `git.rs:414` already sets `skip_paths: vec![".claude/worktrees/".to_string()]`;
      ADD the three new entries rather than replacing the vec, so the final value is
      `vec![".claude/worktrees/".to_string(), "plans/done".to_string(), "apps/ayokoding-web/content".to_string(), "apps/ose-web/content".to_string()]`
      (do NOT drop the existing `.claude/worktrees/` entry — omitting it would cause the link
      checker to scan worktree directories and emit spurious broken-link findings);
      (4) register both new steps in `run(deps)`.
      Run `npx nx run rhino-cli:test:quick` — acceptance: all tests (new + preexisting) pass.
- [ ] [AI] **REFACTOR** — Factor staged-file collection shared by the three steps; align step
      numbering/comments. Run `npx nx run rhino-cli:lint && npx nx run rhino-cli:test:quick`
      — acceptance: both exit 0; no clippy warnings introduced.

### Phase 3 Gate

> All checks below must pass before starting Phase 4.

- [ ] [AI] `npx nx run rhino-cli:test:quick` is green (pre-commit step tests + all preexisting).
- [ ] [AI] `npx nx run rhino-cli:lint` exits 0.

> **Pause Safety**: the pre-commit suite binary now contains all three staged-only steps, but the
> `.husky/` hooks and CI are not yet rewired (Phase 4). The installed git hook still calls the old
> binary path; nothing in the repo's enforcement has visibly changed until rebuild + Phase 4. Safe
> to stop. To resume: `npx nx run rhino-cli:test:quick`.

---

## Phase 4: Wire Enforcement — Pre-Push, Nx Targets, Consolidated CI

> _Suggested executor: `swe-rust-dev`_
>
> Wires all THREE layers (DD-1/DD-8/DD-9): Layer 1 = pre-commit (the rebuilt suite, already
> implemented in Phase 3) + remove mermaid from pre-push; Layer 2 = `validate-markdown.yml` on
> `pull_request` to `main`; Layer 3 = the same workflow on `push` to `main`.

- [ ] [AI] **Layer 1 (pre-push removal)** — Edit `.husky/pre-push`: remove the mermaid trigger
      block (the `if echo "$CHANGED" | grep -qE '^(repo-governance/|\.claude/).*\.md$'` block that
      runs `npx nx run rhino-cli:validate:mermaid`, lines ~23-25). Verify by inspection
      — acceptance: `.husky/pre-push` contains no reference to `validate:mermaid`.
- [ ] [AI] Add `validate:links` and `validate:heading-hierarchy` Nx targets to
      `apps/rhino-cli/project.json` (DD-9). Read the existing `validate:mermaid` entry
      (`project.json` lines ~167-178) and mirror its JSON structure exactly (`executor`,
      `options.command`, `cache: true`, `inputs` array). Use these entries:

  ```json
  "validate:links": {
    "executor": "nx:run-commands",
    "options": {
      "command": "cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-links --exclude plans/done --exclude apps/ayokoding-web/content --exclude apps/ose-web/content"
    },
    "cache": true,
    "inputs": [
      "{projectRoot}/src/**/*.rs",
      "{workspaceRoot}/**/*.md"
    ]
  },
  "validate:heading-hierarchy": {
    "executor": "nx:run-commands",
    "options": {
      "command": "cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-heading-hierarchy"
    },
    "cache": true,
    "inputs": [
      "{projectRoot}/src/**/*.rs",
      "{workspaceRoot}/**/*.md"
    ]
  }
  ```

  Verify: `npx nx run rhino-cli:validate:links` and
  `npx nx run rhino-cli:validate:heading-hierarchy` execute (pass/fail acceptable here)
  — acceptance: both targets resolve and run; `files`/findings reported.

- [ ] [AI] **Layers 2 & 3 (consolidated CI)** — Create the NEW file
      `.github/workflows/validate-markdown.yml`, MIRRORING the structure of the existing
      `.github/workflows/pr-validate-links.yml` (read it first for grounding: `actions/checkout@v6`
      → `./.github/actions/setup-rust`, `ubuntu-latest`). Differences:
  - `on:` block has BOTH triggers (mirroring `crane-cli-integration.yml`):

    ```yaml
    on:
      pull_request:
        branches: [main]
      push:
        branches: [main]
    ```

  - run all three validators full-scan (add `./.github/actions/setup-node` before `setup-rust` if
    invoking via `nx`; check out with `fetch-depth: 0`):
    - mermaid: `npx nx run rhino-cli:validate:mermaid`
    - links: `npx nx run rhino-cli:validate:links`
    - heading-hierarchy: `npx nx run rhino-cli:validate:heading-hierarchy`
      Verify: `npx prettier --check .github/workflows/validate-markdown.yml` exits 0; run
      `actionlint`/`yamllint` if available (skip gracefully if not) — acceptance: the file exists;
      prettier passes; the `on:` block has BOTH `pull_request: branches: [main]` AND
      `push: branches: [main]`; all three validators are invoked.

- [ ] [AI] **Migrate the legacy link workflow** — Delete
      `.github/workflows/pr-validate-links.yml` (its link check now runs inside
      `validate-markdown.yml`). Verify by inspection — acceptance: the file no longer exists; the
      consolidated workflow runs `validate:links`.
- [ ] [AI] Rebuild the pre-commit binary so the installed git hook picks up the Phase 3 steps:
      `cargo build --release --quiet --manifest-path apps/rhino-cli/Cargo.toml`
      — acceptance: exits 0; a scratch staged malformed-diagram commit is blocked by the local
      pre-commit hook (then unstage the scratch change).
- [ ] [AI+HUMAN] **Behavioral acceptance (observed at execution)** — Confirm a deliberately-broken
      markdown change makes the CI check FAIL. This requires a real GitHub Actions run (an actual
      PR/push event, not fully simulatable locally): on a throwaway branch or scratch commit,
      introduce one broken relative link (or a broken `#anchor`, or a duplicate H1 in a `docs/`
      file), open a PR to `main` (or push to `main` where safe), and observe `validate-markdown`
      go RED; then revert. Acceptance: the `validate-markdown` check reports failure on the broken
      markdown and passes once reverted. (Agent prepares the scratch change + PR; human confirms
      the observed CI result and authorizes the throwaway push if a real event is required.)

### Phase 4 Gate

> All checks below must pass before starting Phase 5. The validators are EXPECTED to report
> findings here (the fix-all has not run) — that is acceptable for this gate; what must hold is
> that the wiring is correct across all three layers.

- [ ] [AI] `.husky/pre-push` contains no `validate:mermaid` reference (Layer 1 removal, inspection).
- [ ] [AI] `npx nx run rhino-cli:validate:links` and
      `npx nx run rhino-cli:validate:heading-hierarchy` execute against full scope (pass/fail
      acceptable).
- [ ] [AI] `.github/workflows/validate-markdown.yml` exists;
      `npx prettier --check .github/workflows/validate-markdown.yml` exits 0; `on:` has BOTH
      `pull_request: branches: [main]` AND `push: branches: [main]`; all three validators invoked;
      `pr-validate-links.yml` deleted (inspection).

> **Pause Safety**: wiring is in place but the repo has known markdown findings — do NOT push from
> here, because pre-commit/CI would now block on the unfixed backlog. This is a coherent **local**
> stopping point (no half-edited files). To resume: re-run the three validators and proceed to
> per-tree cleanup.

---

## Per-Tree Fix-All Phases (gated)

> For EACH tree below: re-measure with all THREE expanded validators (within scope), then for every
> blocking finding apply ONE of — (mermaid) shorten labels / restructure / add a justified inline
> exemption; (link) fix the path or correct the target; (anchor) fix the `#fragment` to match a
> real slugified heading or update the destination heading; (heading) restructure to a single H1
> with non-skipping nesting. Re-measure each tree at execution — do NOT rely on authoring-time
> counts. Heading findings apply ONLY to prose-allowlist trees.
> _Suggested executor per tree: `swe-rust-dev` for `apps/`/`libs/` (code-adjacent); `docs-maker`
> for `docs/` content; `repo-rules-maker` for `repo-governance/`; otherwise a generic edit._

### Phase 5: Fix-all `repo-governance/`

- [ ] [AI] Re-measure all three gates for this tree:
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-mermaid --output json repo-governance/` ;
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-links --output json --exclude plans/done --exclude apps/ayokoding-web/content --exclude apps/ose-web/content repo-governance/` ;
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-heading-hierarchy --output json repo-governance/`
      — acceptance: per-finding lists recorded (mermaid / broken-link / broken-anchor / heading).
- [ ] [AI] For each finding listed in the Phase 5 re-measure output: apply the resolution per the
      Per-Tree Fix-All preamble — (mermaid) shorten label / restructure diagram / add a
      `%% rhino-cli:exempt` inline exemption with justification; (link) correct the relative path
      or update the link target; (anchor) fix the `#fragment` to match a real heading slug or
      rename the destination heading; (heading) restructure to a single H1 with non-skipping
      nesting. After each fix, re-run the applicable validator for that file to confirm the finding
      is resolved. Acceptance: re-running all three measurement commands from Phase 5 step 1 shows
      zero findings for `repo-governance/`.
  - _Suggested executor: `repo-rules-maker`._

### Phase 5 Gate

> All checks below must pass before starting Phase 6.

- [ ] [AI] `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-mermaid --output json repo-governance/`
      exits 0 — zero mermaid findings for `repo-governance/`.
- [ ] [AI] `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-links --output json --exclude plans/done --exclude apps/ayokoding-web/content --exclude apps/ose-web/content repo-governance/`
      exits 0 — zero link/anchor findings for `repo-governance/`.
- [ ] [AI] `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-heading-hierarchy --output json repo-governance/`
      exits 0 — zero heading findings for `repo-governance/`.

> **Pause Safety**: `repo-governance/` is clean under the new rules; other trees may still have
> findings (don't push yet). Safe to stop. To resume: re-run the three commands above.

### Phase 6: Fix-all `docs/`

- [ ] [AI] Re-measure all three gates for `docs/` (mermaid + links-with-excludes + heading-hierarchy,
      JSON output) — acceptance: per-finding lists recorded.
  - _Suggested executor: `docs-maker` for content-bearing edits._
- [ ] [AI] For each finding listed in the Phase 6 re-measure output: apply the resolution per the
      Per-Tree Fix-All preamble. After each fix, re-run the applicable validator for that file.
      Acceptance: re-running all three measurement commands from Phase 6 step 1 shows zero findings
      for `docs/`.
  - _Suggested executor: `docs-maker` for content-bearing edits._

### Phase 6 Gate

> All checks below must pass before starting Phase 7.

- [ ] [AI] `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-mermaid --output json docs/`
      exits 0 — zero mermaid findings for `docs/`.
- [ ] [AI] `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-links --output json --exclude plans/done --exclude apps/ayokoding-web/content --exclude apps/ose-web/content docs/`
      exits 0 — zero link/anchor findings for `docs/`.
- [ ] [AI] `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-heading-hierarchy --output json docs/`
      exits 0 — zero heading findings for `docs/`.

> **Pause Safety**: `docs/` clean; remaining trees pending. Safe to stop. To resume: re-run the
> three commands above.

### Phase 7: Fix-all `plans/` (excludes `plans/done/`; includes this plan — dogfooding)

- [ ] [AI] Re-measure all three gates for `plans/` (mermaid + links + heading-hierarchy). For links,
      pass `--exclude plans/done`; heading-hierarchy already excludes `plans/done/` via the
      allowlist — acceptance: per-finding lists recorded.
- [ ] [AI] For each finding listed in the Phase 7 re-measure output: apply the resolution per the
      Per-Tree Fix-All preamble. After each fix, re-run the applicable validator for that file.
      Acceptance: re-running all three measurement commands from Phase 7 step 1 shows zero findings
      for `plans/` (excluding `plans/done/`), including this plan's own five docs (dogfooding).

### Phase 7 Gate

> All checks below must pass before starting Phase 8.

- [ ] [AI] `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-mermaid --output json plans/`
      exits 0 — zero mermaid findings for `plans/`.
- [ ] [AI] `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-links --output json --exclude plans/done plans/`
      exits 0 — zero link/anchor findings for `plans/` (excluding `plans/done/`), including this plan.
- [ ] [AI] `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-heading-hierarchy --output json plans/`
      exits 0 — zero heading findings for `plans/` (allowlist excludes `plans/done/` automatically).

> **Pause Safety**: `plans/` clean; `apps/`, `libs/`, root pending. Safe to stop. To resume: re-run
> the three commands above.

### Phase 8: Fix-all `apps/` and `libs/` (mermaid + links only; heading-hierarchy excludes these)

> Heading-hierarchy does NOT run on `apps/`/`libs/` (outside the prose allowlist). For links, pass
> `--exclude apps/ayokoding-web/content --exclude apps/ose-web/content` (those trees own their
> validation). _Suggested executor: `swe-rust-dev` (diagrams/links in app READMEs are
> code-adjacent)._

- [ ] [AI] Re-measure mermaid + links for `apps/` and `libs/`:
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-mermaid --output json apps/ libs/` ;
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-links --output json --exclude apps/ayokoding-web/content --exclude apps/ose-web/content apps/ libs/`
      — acceptance: per-finding lists recorded.
- [ ] [AI] For each finding listed in the Phase 8 re-measure output: apply the resolution per the
      Per-Tree Fix-All preamble (mermaid or link/anchor only — heading-hierarchy is out of scope for
      these trees). After each fix, re-run the applicable validator for that file. Acceptance:
      re-running both measurement commands from Phase 8 step 1 shows zero findings for `apps/` and
      `libs/`.

### Phase 8 Gate

> All checks below must pass before starting Phase 9.

- [ ] [AI] `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-mermaid --output json apps/ libs/`
      exits 0 — zero mermaid findings for `apps/` and `libs/`.
- [ ] [AI] `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-links --output json --exclude apps/ayokoding-web/content --exclude apps/ose-web/content apps/ libs/`
      exits 0 — zero link/anchor findings for `apps/` and `libs/`.

> **Pause Safety**: `apps/`/`libs/` clean; only root files pending. Safe to stop. To resume: re-run
> the two commands above.

### Phase 9: Fix-all root instruction files (`AGENTS.md`, `CLAUDE.md`, root `README.md`)

> These ARE in the prose allowlist (root `*.md`), so all three gates apply.

- [ ] [AI] Re-measure all three gates for the root files:
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-mermaid --output json AGENTS.md CLAUDE.md README.md` ;
      links + heading-hierarchy over the same files — acceptance: per-finding lists recorded.
- [ ] [AI] For each finding listed in the Phase 9 re-measure output: apply the resolution per the
      Per-Tree Fix-All preamble. After each fix, re-run the applicable validator for that file.
      Acceptance: re-running all three measurement commands from Phase 9 step 1 shows zero findings
      for the root files.

### Phase 9 Gate

> All checks below must pass before starting Phase 10.

- [ ] [AI] `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-mermaid --output json AGENTS.md CLAUDE.md README.md`
      exits 0 — zero mermaid findings for root files.
- [ ] [AI] `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-links --output json --exclude plans/done --exclude apps/ayokoding-web/content --exclude apps/ose-web/content AGENTS.md CLAUDE.md README.md`
      exits 0 — zero link/anchor findings for root files.
- [ ] [AI] `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-heading-hierarchy --output json AGENTS.md CLAUDE.md README.md`
      exits 0 — zero heading findings for root files.

> **Pause Safety**: all trees individually clean. The full-repo gates should now pass. Safe to
> stop. To resume: re-run the three commands above.

---

## Phase 10: Update Governance Docs (convention accuracy)

> _Suggested executor: `repo-rules-maker` (governance conventions)._

- [ ] [AI] Edit `repo-governance/conventions/formatting/diagrams.md`: update the mermaid-enforcement
      description to state the gate runs at **pre-commit staged-only** + the consolidated CI
      workflow (NOT pre-push) — acceptance: the doc matches the Phase 4 wiring; no stale pre-push
      claim remains.
- [ ] [AI] Edit `repo-governance/conventions/writing/quality.md`: note that single-H1 and
      non-skipping heading nesting are now **machine-enforced for prose** via
      `rhino-cli docs validate-heading-hierarchy`, scoped to `docs/`, `repo-governance/`,
      `plans/`(−`done/`), and root `*.md`, and explicitly exempt for `.claude/**`/`.opencode/**`
      prompt/skill artifacts — acceptance: the scope + exemption are stated.
- [ ] [AI] Edit `repo-governance/conventions/formatting/linking.md`: note that `#fragment` anchors
      are now validated against the target file's headings (`broken-anchor` finding) — acceptance:
      anchor enforcement is documented.
- [ ] [AI] Update the check-inventory / `repo-governance/development/quality/repository-validation.md`
      doc(s) to list the three markdown gates and the consolidated `validate-markdown.yml` workflow
      — acceptance: the three gates and the workflow are listed.
- [ ] [AI] If any `.claude/` agent/skill text changed as part of governance propagation, run
      `npm run generate:bindings` to re-sync the secondary bindings — acceptance: `git status`
      shows the generated `.opencode/`/`.amazonq/` mirrors updated in lockstep (or no `.claude/`
      change occurred and this is a no-op).
- [ ] [AI] Verify the governance docs themselves pass all three gates:
      `cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- docs validate-heading-hierarchy repo-governance/conventions/`
      and the mermaid + link gates over the edited files — acceptance: all exit 0.

### Phase 10 Gate

- [ ] [AI] `npm run lint:md` passes for the edited governance docs.
- [ ] [AI] All documented facts (mermaid at pre-commit+CI, prose heading enforcement + exemption,
      anchor validation, the three gates + consolidated workflow in the check inventory) are present
      (review).

> **Pause Safety**: governance docs now match the tooling. Safe to stop. To resume: re-run the
> three validators full-scan.

---

## Phase 11: Full-Repo Verification, Quality Gates, Push, CI, Archival

### Repo-Rules Quality Gate (strict, double-zero)

- [ ] [AI] Run the strict repo-rules quality gate to confirm governance changes are consistent and
      propagated, requiring a **double-zero** pass (zero checker findings AND zero fixer changes on
      a clean re-run): invoke `repo-rules-quality-gate` (strict) over the changed governance
      surface (`diagrams.md`, `quality.md`, `linking.md`, check-inventory docs, and any `.claude/`
      bindings) — acceptance: the checker reports zero findings and a follow-up fixer pass produces
      zero changes.
  - _Suggested executor: `repo-rules-checker` then `repo-rules-fixer` (double-zero), per the
    `repo-rules-quality-gate`._

### Local Quality Gates (Before Push)

- [ ] [AI] Run all three markdown gates full-scan:
      `npx nx run rhino-cli:validate:mermaid` ;
      `npx nx run rhino-cli:validate:links` ;
      `npx nx run rhino-cli:validate:heading-hierarchy`
      — acceptance: all three exit 0 (zero findings within scope).
- [ ] [AI] Run affected typecheck: `npx nx affected -t typecheck` — acceptance: exits 0.
- [ ] [AI] Run affected linting: `npx nx affected -t lint` — acceptance: exits 0.
- [ ] [AI] Run affected quick tests: `npx nx affected -t test:quick` — acceptance: exits 0.
- [ ] [AI] Run affected spec coverage: `npx nx affected -t spec-coverage` — acceptance: exits 0.
- [ ] [AI] Run markdown lint: `npm run lint:md` — acceptance: exits 0.
- [ ] [AI] Fix ALL failures — including preexisting issues not caused by these changes — and re-run
      the failing checks to confirm resolution. Verify zero failures before pushing.

### Commit Guidelines

- [ ] [AI] Commit changes thematically (Conventional Commits `<type>(<scope>): <description>`),
      split by concern, for example:
  - `feat(rhino-cli): add --exclude flag and repo-wide scan to validate-links`
  - `feat(rhino-cli): validate markdown anchors against target headings`
  - `refactor(rhino-cli): share fence-aware heading parser between links and heading-hierarchy`
  - `feat(rhino-cli): scope heading-hierarchy to a prose allowlist (default-deny)`
  - `feat(rhino-cli): add staged-only mermaid and heading pre-commit steps`
  - `chore(husky): remove mermaid trigger from pre-push`
  - `feat(rhino-cli): add validate:links and validate:heading-hierarchy Nx targets`
  - `ci: consolidate markdown gates into validate-markdown workflow`
  - `ci: remove migrated pr-validate-links workflow`
  - `fix(<scope>): clean markdown gate violations in <tree>` (one per tree as appropriate)
  - `docs(governance): document pre-commit mermaid, prose heading rules, and anchor validation`
  - Preexisting fixes get their own separate commits.
    — acceptance: no unrelated changes bundled into a single commit.

### Push and Post-Push CI Verification

- [ ] [AI] Push directly to `main`: `git push origin main`
      — acceptance: push succeeds (pre-commit hook green for the staged set; pre-push green).
- [ ] [AI] Monitor ALL GitHub Actions workflows triggered by the push (poll every 3 minutes; one
      `gh run view --json status,conclusion` per wakeup; do NOT use `gh run watch`)
      — acceptance: every workflow run observed to completion, INCLUDING the new `validate-markdown`
      workflow (Layer 3 fires on this `push` to `main`).
- [ ] [AI] Verify the `validate-markdown` workflow run passes and ALL other CI checks pass
      — acceptance: zero failures; the `validate-markdown` run is green.
- [ ] [AI] If any CI check fails, investigate root cause, fix, and push a follow-up commit; repeat
      until ALL GitHub Actions are green — acceptance: full CI green.

### Plan Archival

- [ ] [AI] Verify ALL delivery checklist items are ticked.
- [ ] [AI] Verify ALL quality gates pass (local + CI).
- [ ] [AI] Verify all three markdown gates report zero findings within scope.
- [ ] [AI] Move:
      `git mv plans/in-progress/markdown-gate-coverage-expansion plans/done/2026-06-06__markdown-gate-coverage-expansion`
      (use the actual completion date, NOT the creation date).
- [ ] [AI] Update `plans/in-progress/README.md` — remove the `markdown-gate-coverage-expansion`
      entry (note: the current row still uses the old `mermaid-gate-coverage-expansion` label and
      link — replace or remove it).
- [ ] [AI] Update `plans/done/README.md` — add the plan entry with completion date.
- [ ] [AI] Update any other READMEs that reference this plan (e.g. `plans/README.md`).
- [ ] [AI] Commit the archival: `chore(plans): move markdown-gate-coverage-expansion to done`, then
      push to `origin main`.

### Phase 11 Gate

> All checks below must pass — this is the final gate.

- [ ] [AI] `npx nx run rhino-cli:validate:mermaid`, `:validate:links`, and
      `:validate:heading-hierarchy` all exit 0 (full repo clean within scope).
- [ ] [AI] `npx nx affected -t typecheck lint test:quick spec-coverage` exits 0 and
      `npm run lint:md` passes.
- [ ] [AI] The `repo-rules-quality-gate` double-zero pass is clean.
- [ ] [AI] All GitHub Actions for the push are green, including the new `validate-markdown`
      workflow run (push-to-main trigger).
- [ ] [AI] Plan archived to `plans/done/` and READMEs updated.

> **Pause Safety**: work is complete, pushed, CI green, plan archived. This is the terminal state.
> To re-verify at any later time: run the three markdown validators full-scan.
