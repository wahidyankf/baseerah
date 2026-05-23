# Delivery — Specs Tree Uniformity Pass

## Worktree

Worktree path: `worktrees/specs-tree-uniform/`

Provision before execution (run from repo root):

```bash
claude --worktree specs-tree-uniform
```

See [Worktree Path Convention](../../../repo-governance/conventions/structure/worktree-path.md)
and [Plans Organization Convention §Worktree Specification](../../../repo-governance/conventions/structure/plans.md#worktree-specification).

## Phase 0 — Environment Setup and Decisions

- [ ] Provision worktree: `claude --worktree specs-tree-uniform` — creates
      `worktrees/specs-tree-uniform/` in repo root.
  - _Suggested executor: default plan-execution orchestrator_
- [ ] Initialize toolchain in the root worktree (not the new worktree):
      `npm install && npm run doctor -- --fix` — exits 0; see
      [Worktree Toolchain Initialization](../../../repo-governance/development/workflow/worktree-setup.md).
- [ ] `cd worktrees/specs-tree-uniform/` and verify the working tree:
      `git status` reports a clean tree on the worktree branch.
- [ ] Re-read [Specs Directory Structure Convention](../../../repo-governance/conventions/structure/specs-directory-structure.md)
      and [App README vs Specs Convention](../../../repo-governance/conventions/structure/app-readme-vs-specs.md)
      end-to-end. Confirm: nothing in the convention has changed since plan-authoring
      (2026-05-23) that would invalidate the gap inventory in
      [tech-docs.md §Gap Inventory](./tech-docs.md#gap-inventory). If anything changed, update
      the plan first; do not migrate against a stale convention.
- [ ] Confirm exact current state via filesystem:
      `find specs -maxdepth 4 -type d | sort > /tmp/specs-tree-before.txt`.
      Inspect to verify GAP-1 through GAP-8 still match.
- [ ] Confirm exact crane feature-file list:
      `ls specs/apps/crane/gherkin/`. Compare against the file list in
      [tech-docs.md §R1](./tech-docs.md#r1--crane-flat-gherkin--behaviorcligherkin).
      Update R1's mv block if filenames have drifted. [Repo-grounded check]
- [ ] Resolve Decision D1 (ayokoding `build-tools/` slug) per
      [tech-docs.md §D1](./tech-docs.md#d1--ayokoding-build-tools-slug-fate). Record the
      decision verbatim at the top of `delivery.md` (this file) as a callout:
      `> D1 resolution (YYYY-MM-DD): chose option A/B/C because ...`.
- [ ] Confirm `apps/rhino-cli/src/internal/allowlist.rs` location and exact constant name:
      `grep -n 'WithDDD\|with_ddd\|AppsWithDDD' apps/rhino-cli/src/internal/allowlist.rs`.
      Update [tech-docs.md §R6](./tech-docs.md#r6--allowlist-update) if the Rust constant
      name differs from the assumed `APPS_WITH_DDD`. [Repo-grounded check]

## Phase 1 — Root README rewrite

- [ ] Edit `specs/README.md`: replace the "Standard Folder Pattern" section (currently lines
      46–73) with content matching the five-folder layout from
      [specs-directory-structure.md §Five-Folder Layout](../../../repo-governance/conventions/structure/specs-directory-structure.md#five-folder-layout).
      Show the canonical tree (product/, system-context/, containers/, components/, behavior/)
      with `containers/contracts/` and `behavior/<surface>/gherkin/<domain>/<feature>.feature`
      paths. Acceptance: section no longer mentions `be/fe/fs/cli/gherkin/` as a top-level
      structure.
  - _Suggested executor: `docs-maker`_
- [ ] Edit `specs/README.md` "App Specs" list: replace current entries with full alphabetized
      list — `ayokoding`, `crane`, `organiclever`, `ose-app`, `ose-platform`, `rhino`,
      `wahidyankf`. Each entry: relative link to `./apps/<name>/README.md` + one-line
      description matching the per-app README's first line.
- [ ] Edit `specs/README.md` "Library Specs" list: list exactly `golang-commons`, `hugo-commons`,
      `web-ui` with relative links. Add inline note next to `hugo-commons`:
      `_Hugo agent is deprecated; lib retention under separate review — see CLAUDE.md._`
      [Repo-grounded — CLAUDE.md confirms `swe-hugo-dev` deprecation]
- [ ] Edit `specs/README.md`: ensure the "Standards" link block remains intact pointing to
      `docs/explanation/software-engineering/development/behavior-driven-development-bdd/`
      and `repo-governance/development/infra/bdd-spec-test-mapping.md`. Verify links resolve.
- [ ] Run `npm run lint:md` against `specs/README.md` — exits 0.
- [ ] Run `nx run rhino-cli:validate:specs-links` — exits 0; no broken links from root README.
- [ ] Commit: `git add specs/README.md && git commit -m "docs(specs): rewrite root README to
match canonical five-folder tree"`.

## Phase 2 — Crane migration (atomic commit per R1)

- [ ] Create destination tree:
      `mkdir -p specs/apps/crane/{product,system-context,containers,components/cli,behavior/cli/gherkin}`.
- [ ] Author skeleton READMEs per [tech-docs.md §R3](./tech-docs.md#r3--skeleton-readme-template)
      at:
      `specs/apps/crane/product/README.md`,
      `specs/apps/crane/system-context/README.md`,
      `specs/apps/crane/containers/README.md`,
      `specs/apps/crane/components/cli/README.md`.
      Each file: ~5 lines per template. Verify relative-link depth resolves via `validate:specs-links`.
  - _Suggested executor: `specs-maker`_
- [ ] Execute the `git mv` block from
      [tech-docs.md §R1 Step 2](./tech-docs.md#r1--crane-flat-gherkin--behaviorcligherkin)
      verbatim against the on-disk file list (re-confirmed in Phase 0). Acceptance: every
      `.feature` file and `gherkin/README.md` lives under
      `specs/apps/crane/behavior/cli/gherkin/`; `specs/apps/crane/gherkin/` no longer exists.
- [ ] Run the path-reference sweep — execute the bash block verbatim from
      [tech-docs.md §R1 Step 3](./tech-docs.md#r1--crane-flat-gherkin--behaviorcligherkin)
      (`grep -rln ... | xargs sed -i.bak ...; find . -name '*.bak' -delete`).
      Acceptance: `grep -rln 'specs/apps/crane/gherkin[^/c]' . | wc -l` returns 0 (the
      negative-lookahead pattern excludes the new path).
- [ ] Edit `specs/apps/crane/README.md`: rewrite the "Structure" block to show the canonical
      CLI-only five-folder tree. Update the "Running the Tests" code block step paths from
      `apps/crane-cli/tests/unit/steps/` references if needed (verify paths unchanged).
  - _Suggested executor: `specs-maker`_
- [ ] Verify locally inside the worktree:
      `nx run rhino-cli:validate:specs-tree --apps crane && nx run rhino-cli:validate:specs-counts --apps crane && nx run rhino-cli:validate:specs-links --apps crane`
      — all three exit 0.
- [ ] Verify crane unit + integration tests still pass:
      `nx run crane-cli:test:unit && nx run crane-cli:test:integration` — both exit 0.
- [ ] Commit atomically:
      `git add -A && git commit -m "refactor(specs/crane): migrate to canonical CLI-only five-folder tree"`.

## Phase 3 — Rhino fill-out (atomic commit per R2)

- [ ] Create folders:
      `mkdir -p specs/apps/rhino/{product,system-context,containers,components/cli}`.
- [ ] Author skeleton READMEs per [tech-docs.md §R3](./tech-docs.md#r3--skeleton-readme-template)
      at:
      `specs/apps/rhino/product/README.md`,
      `specs/apps/rhino/system-context/README.md`,
      `specs/apps/rhino/containers/README.md`,
      `specs/apps/rhino/components/cli/README.md`.
  - _Suggested executor: `specs-maker`_
- [ ] Edit `specs/apps/rhino/README.md`: update the "Structure" block to show all five
      top-level folders, not just `behavior/cli/gherkin/`.
- [ ] Verify:
      `nx run rhino-cli:validate:specs-tree --apps rhino && nx run rhino-cli:validate:specs-counts --apps rhino && nx run rhino-cli:validate:specs-links --apps rhino`
      — all three exit 0.
- [ ] Verify rhino-cli unit + integration tests:
      `nx run rhino-cli:test:quick && nx run rhino-cli:test:integration` — both exit 0.
- [ ] Commit atomically:
      `git add -A && git commit -m "refactor(specs/rhino): add missing CLI-only surface folders"`.

## Phase 4 — Ayokoding build-tools resolution

> Branch on D1 resolution recorded in Phase 0.

### Phase 4.A — If D1 == A (migrate under `behavior/build-tools/gherkin/`)

- [ ] Locate the surface-allowlist constant in `apps/rhino-cli/src/specs/`:
      `grep -rn '"cli"\|"be"\|"web"' apps/rhino-cli/src/specs | grep -i 'surface\|allow' | head`
      Identify the canonical file (likely `apps/rhino-cli/src/specs/validate_tree.rs` or sibling).
- [ ] Edit the surface enum/allowlist to add `"build-tools"` as a valid surface. Acceptance:
      `cargo check --manifest-path apps/rhino-cli/Cargo.toml` exits 0.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Add unit test in the same Rust file: scenario "build-tools surface accepted by validate-tree".
      Run `nx run rhino-cli:test:quick` — new test passes.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Execute the migration block (`mkdir -p`, `git mv`, `rmdir`) verbatim from
      [tech-docs.md §R4](./tech-docs.md#r4--ayokoding-build-tools-migration-assuming-d1a).
      Acceptance: `specs/apps/ayokoding/build-tools/` no longer exists and
      `specs/apps/ayokoding/behavior/build-tools/gherkin/index-generation/` does.
- [ ] Path-reference sweep — execute the `grep | xargs sed; find -name '*.bak' -delete` block
      from [tech-docs.md §R4](./tech-docs.md#r4--ayokoding-build-tools-migration-assuming-d1a).
      Acceptance: `grep -rln 'specs/apps/ayokoding/build-tools[^/]' . | wc -l` returns 0.
- [ ] Edit `specs/apps/ayokoding/README.md`: remove the "Out of scope for this spec tree
      (preserved unchanged as legacy slugs)" note (currently lines 45–53) referencing
      `build-tools/`. Update the "Structure" tree block to include
      `behavior/build-tools/gherkin/`.
  - _Suggested executor: `specs-maker`_
- [ ] Verify:
      `nx run rhino-cli:validate:specs-tree --apps ayokoding && nx run rhino-cli:validate:specs-counts --apps ayokoding && nx run rhino-cli:validate:specs-links --apps ayokoding`
      — all three exit 0.
- [ ] Verify ayokoding-web tests still pass:
      `nx run ayokoding-web:test:quick` — exits 0.
- [ ] Commit atomically:
      `git add -A && git commit -m "refactor(specs/ayokoding): migrate build-tools slug under behavior/"`.

### Phase 4.B — If D1 == B (promote build-tools to permanent perspective slug)

- [ ] Edit
      [`repo-governance/conventions/structure/specs-directory-structure.md`](../../../repo-governance/conventions/structure/specs-directory-structure.md):
      add `build-tools` to the list of permitted perspective slugs in the Canonical App Spec
      Tree section. Document the rationale (build-time scripts vs runtime CLI commands).
  - _Suggested executor: `repo-rules-maker`_
- [ ] Edit `specs/apps/ayokoding/README.md`: convert the "Out of scope" note into a
      "Permanent perspective slug" subsection citing the updated convention.
- [ ] Verify:
      `nx run rhino-cli:validate:specs-tree --apps ayokoding` — exits 0.
- [ ] Commit atomically:
      `git add -A && git commit -m "docs(specs): formalize build-tools as permanent perspective slug"`.

### Phase 4.C — If D1 == C (inline under existing behavior/cli/gherkin/)

- [ ] Move feature files into `specs/apps/ayokoding/behavior/cli/gherkin/build-tools-*.feature`
      (rename each with `build-tools-` prefix to preserve discoverability).
- [ ] Path-reference sweep + README update + verify (same shape as Phase 4.A's last three steps).
- [ ] Commit atomically:
      `git add -A && git commit -m "refactor(specs/ayokoding): inline build-tools features into cli surface"`.

## Phase 5 — ose-app PM section + allowlist update

- [ ] Edit `specs/apps/ose-app/README.md`: add a "For Product / Project Managers" section
      modeled on
      [`specs/apps/organiclever/README.md`](../../../specs/apps/organiclever/README.md)
      lines 168–197 — Audience note, Reading order (1–5), "In plain language" bullet list.
      Adapt prose to ose-app's regulatory-gap-analysis domain.
  - _Suggested executor: `specs-maker`_
- [ ] Verify: `nx run rhino-cli:validate:specs-links --apps ose-app` — exits 0.
- [ ] Commit: `git add specs/apps/ose-app/README.md && git commit -m "docs(specs/ose-app): add
PM-readable reading-order section"`.
- [ ] Resolve Decision D2 per
      [tech-docs.md §D2](./tech-docs.md#d2--allowlist-policy-for-appswithddd). Record the
      decision at the top of this delivery file as a callout.
- [ ] If D2 == A (add ose-app to allowlist): edit `apps/rhino-cli/src/internal/allowlist.rs`
      to add `"ose-app"` to the `APPS_WITH_DDD` (or actual constant name from Phase 0 grep).
      Add inline `//` comment block above the constant documenting the inclusion criterion.
      Acceptance: `cargo check --manifest-path apps/rhino-cli/Cargo.toml` exits 0 AND
      `nx run rhino-cli:test:quick` exits 0.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Run `nx run rhino-cli:validate:specs-tree && nx run rhino-cli:validate:specs-adoption &&
nx run rhino-cli:validate:specs-counts && nx run rhino-cli:validate:specs-links` — all
      four exit 0 with no `--apps` flag.
      **Expected**: ose-app DDD entries with empty BC content may surface adoption findings.
      Address each finding by either populating the BC field or removing the BC entry from
      `ddd/bounded-contexts.yaml` (consult the user before deleting entries).
- [ ] If D2 == B (exclude ose-app): add `//` comment block above the constant in
      `allowlist.rs` documenting the exclusion criterion (zero populated BC entries today).
- [ ] Commit: `git add apps/rhino-cli/src/internal/allowlist.rs && git commit -m
"feat(rhino-cli): document AppsWithDDD allowlist policy"` (or `feat(rhino-cli): add
ose-app to AppsWithDDD allowlist`).

## Phase 6 — Governance Propagation (repo-rules-maker)

After structural migrations land (Phases 2–5), propagate the new uniform state into governance
and agent documentation so future contributors and agents read a consistent story. This phase
is delegated to the `repo-rules-maker` agent — it owns `repo-governance/` and is the only
agent authorized to write rules and conventions there per
[Agent Naming Convention](../../../repo-governance/conventions/structure/agent-naming.md).

- [ ] Invoke `repo-rules-maker` with the following propagation brief (paste verbatim into the
      agent invocation):

      > Propagation brief for repo-rules-maker — driven by plan
      > `plans/in-progress/specs-tree-uniform/`. Phases 2–5 of that plan have landed: crane is
      > now CLI-canonical, rhino has the full CLI-only surface profile, ayokoding `build-tools`
      > is resolved per Decision D1 (see callout at top of `delivery.md`), and the
      > `AppsWithDDD` allowlist policy is settled per Decision D2. Update governance to match:
      >
      > 1. **`repo-governance/conventions/structure/specs-directory-structure.md`** —
      >    (a) Append a dated migration-history note in the §Migration Path section recording
      >    the crane + rhino + ayokoding/build-tools moves (mirror the existing
      >    "DDD relocation (2026-05-09)" note style at lines 273–278).
      >    (b) If D1 == A: add `build-tools` to the `<surface>` enum description (currently
      >    "be, web, or cli") and document the rationale.
      >    (c) If D1 == B: add `build-tools` to the canonical perspective-slug list (sibling
      >    of `api`) with rationale.
      > 2. **`repo-governance/conventions/structure/app-readme-vs-specs.md`** — refresh the
      >    Adoption Matrix and any per-app examples that cite crane, rhino, ayokoding or
      >    `ose-app` if they still reference pre-migration paths.
      > 3. **`AGENTS.md` Project Structure tree** — update `specs/` block if it documents
      >    legacy paths; cross-check against the new root `specs/README.md`.
      > 4. **`.claude/agents/specs-checker.md`** — refresh Category 1 (Structural Completeness)
      >    enumeration of required folders and Category 8 (Spec Tree Shape Compliance) if it
      >    cites flat-root forms that are now eliminated.
      > 5. **`.claude/agents/specs-maker.md` and `.claude/agents/specs-fixer.md`** — refresh
      >    any path examples that cited the legacy crane/rhino layouts.
      > 6. **`.claude/skills/repo-syncing-with-ose-primer/SKILL.md`** — confirm the extraction
      >    scope for crane/rhino/ayokoding paths still resolves; update if any old path is
      >    referenced.
      > 7. **`docs/reference/related-repositories.md` and `docs/reference/platform-bindings.md`** —
      >    quick grep for any stale path references to `specs/apps/crane/gherkin/` or
      >    `specs/apps/ayokoding/build-tools/`; update if found.
      >
      > **Out of scope**: do NOT re-author the migration recipes (they live in this plan's
      > `tech-docs.md`); do NOT modify any `specs/` file (already migrated); do NOT introduce
      > new conventions, only update existing ones.

- [ ] Verify `repo-rules-maker` only modified files under `repo-governance/`, `AGENTS.md`,
      `.claude/agents/`, `.claude/skills/`, or `docs/reference/`. If it touched anything else,
      reject and re-invoke with tighter scope.
- [ ] Run `npm run sync:claude-to-opencode` to mirror `.claude/agents/` changes into
      `.opencode/agents/`. Acceptance: exit code 0; diff shows only mechanical
      Claude-Code-to-OpenCode translations (color tokens, tool array → boolean flags).
- [ ] Run `nx run rhino-cli:validate:specs-links` — exits 0 (governance updates may have
      changed cross-link targets).
- [ ] Run `npm run lint:md` — exits 0.
- [ ] Invoke `repo-rules-checker` to validate the propagated changes for consistency,
      contradictions, and Skill/agent duplication. Acceptance: exits 0 OR all findings are
      pre-existing and unrelated to this propagation.
- [ ] Address any HIGH/CRITICAL findings from `repo-rules-checker` via `repo-rules-fixer` (or
      manually if the fix is trivial).
- [ ] Commit governance + agent changes as one or two thematic commits per
      [Commit Messages Convention](../../../repo-governance/development/workflow/commit-messages.md): - `docs(repo-governance): propagate specs-tree-uniform changes to conventions and agents` - `chore(agents): sync .opencode mirror after specs propagation` (only if sync diff is non-empty)

## Phase 7 — Local Quality Gates (Before Push)

- [ ] Run affected typecheck: `npx nx affected -t typecheck` — exits 0.
- [ ] Run affected lint: `npx nx affected -t lint` — exits 0.
- [ ] Run affected quick tests: `npx nx affected -t test:quick` — exits 0.
- [ ] Run affected spec coverage: `npx nx affected -t spec-coverage` — exits 0.
- [ ] Run markdown lint: `npm run lint:md` — exits 0.
- [ ] Fix ALL failures found — including preexisting issues not caused by this plan
      (per the root-cause-orientation principle in
      [AGENTS.md](../../../AGENTS.md#conventions)).
- [ ] All four `validate:specs-*` Nx targets exit 0 with no `--apps` flag:
      `nx run rhino-cli:validate:specs-tree && nx run rhino-cli:validate:specs-counts &&
nx run rhino-cli:validate:specs-links && nx run rhino-cli:validate:specs-adoption`.

> **Important**: Fix ALL failures found during quality gates, not just those caused by this
> plan's changes.

### Commit Guidelines

- [ ] Commit changes thematically — each phase produces one or two atomic commits per
      [tech-docs.md §Path-Reference Sweep Discipline](./tech-docs.md#path-reference-sweep-discipline).
- [ ] Follow Conventional Commits format: `<type>(<scope>): <description>`.
- [ ] Do NOT bundle Phase 2 (crane), Phase 3 (rhino), Phase 4 (ayokoding), Phase 5 (ose-app /
      allowlist) into a single commit — each is its own atomic unit.

## Phase 8 — Post-Push Verification

- [ ] Push the worktree branch (or its commits merged back to main per
      [Trunk Based Development](../../../repo-governance/development/workflow/trunk-based-development.md)):
      `git push origin main`.
- [ ] Monitor GitHub Actions workflows triggered by the push.
- [ ] Verify all CI checks pass.
- [ ] If any CI check fails, fix immediately and push a follow-up commit; do NOT proceed to
      Plan Archival until CI is green.
- [ ] Verify the four `validate:specs-*` jobs in `pr-quality-gate.yml` and
      `_reusable-test-and-deploy.yml` are green for this push.

## Plan Archival

- [ ] Verify ALL delivery checklist items above are ticked.
- [ ] Verify ALL quality gates pass (local + CI).
- [ ] `git mv plans/in-progress/specs-tree-uniform plans/done/YYYY-MM-DD__specs-tree-uniform`
      using today's actual completion date.
- [ ] Update `plans/in-progress/README.md` — remove the `specs-tree-uniform` entry (added
      during plan creation, see Plan-creation steps below).
- [ ] Update `plans/done/README.md` — add `specs-tree-uniform` entry with completion date and
      one-line summary.
- [ ] Update any other READMEs cross-referencing this plan.
- [ ] Commit: `chore(plans): move specs-tree-uniform to done`.

## Plan-creation steps (out-of-band — applied at authoring time, 2026-05-23)

The following one-time steps are applied by the plan author when this plan folder is created.
They are NOT executed during plan execution; they were performed at plan-authoring time:

- [x] Create `plans/in-progress/specs-tree-uniform/` directory.
- [x] Author README.md, brd.md, prd.md, tech-docs.md, delivery.md.
- [ ] Add `specs-tree-uniform` entry to `plans/in-progress/README.md` active plans list
      (this is the one outstanding plan-creation step; will be ticked when the plan is
      first read by an execution context).
