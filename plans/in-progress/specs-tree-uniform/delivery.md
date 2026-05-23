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
      Inspect to verify GAP-1 through GAP-9 still match.
- [ ] Confirm exact crane feature-file list:
      `ls specs/apps/crane/gherkin/`. Compare against the file list in
      [tech-docs.md §R1](./tech-docs.md#r1--crane-flat-gherkin--behaviorcligherkindomain).
      Update R1's mv block if filenames have drifted. [Repo-grounded check]
- [ ] Confirm exact rhino feature-file list AND domain-prefix coverage:
      `ls specs/apps/rhino/behavior/cli/gherkin/ | sort`. Compare against the prefix table in
      [tech-docs.md §D5](./tech-docs.md#d5--domain-groupings-for-cli-gherkin-trees). For every
      `.feature` without a prefix-matched domain, assign it to `system/` or add a new domain
      subdir to the D5 table at execution time. [Repo-grounded check]
- [ ] Confirm exact ayokoding-cli + ose-platform-cli feature-file lists:
      `ls specs/apps/ayokoding/behavior/cli/gherkin/ specs/apps/ose-platform/behavior/cli/gherkin/`.
      If files have drifted from the D5 mapping, update D5 before migration.
- [ ] Resolve Decision D1 (ayokoding `build-tools/` slug) per
      [tech-docs.md §D1](./tech-docs.md#d1--ayokoding-build-tools-slug-fate). Record the
      decision verbatim at the top of `delivery.md` (this file) as a callout:
      `> D1 resolution (YYYY-MM-DD): chose option A/B/C because ...`.
- [ ] Resolve Decision D5 (CLI domain groupings) per
      [tech-docs.md §D5](./tech-docs.md#d5--domain-groupings-for-cli-gherkin-trees). Default to
      the table in D5 unless the maintainer rejects a specific app's grouping at execution
      time. Record any overrides in a callout below D1's.
- [ ] Confirm `apps/rhino-cli/src/internal/allowlist.rs` location and exact constant name:
      `grep -n 'WithDDD\|with_ddd\|AppsWithDDD' apps/rhino-cli/src/internal/allowlist.rs`.
      Update [tech-docs.md §R6](./tech-docs.md#r6--allowlist-update) if the Rust constant
      name differs from the assumed `APPS_WITH_DDD`. [Repo-grounded check]
- [ ] Locate the Rust file that owns the `behavior/<surface>/gherkin/` flatness rule:
      `grep -rn 'flat\|domain\|gherkin' apps/rhino-cli/src/specs/`. Record the exact path for
      use in Phase 6 R7.c. Likely `apps/rhino-cli/src/specs/validate_tree.rs` or sibling.
      [Repo-grounded check]

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

## Phase 2 — Crane migration with domain subdirs (atomic commit per R1)

- [ ] Create destination tree with domain subdirs:
      `mkdir -p specs/apps/crane/{product,system-context,containers,components/cli,behavior/cli/gherkin/{pdf,content,media,reporting}}`.
- [ ] Author skeleton READMEs per [tech-docs.md §R3](./tech-docs.md#r3--skeleton-readme-template)
      at:
      `specs/apps/crane/product/README.md`,
      `specs/apps/crane/system-context/README.md`,
      `specs/apps/crane/containers/README.md`,
      `specs/apps/crane/components/cli/README.md`,
      plus a one-paragraph index `README.md` in each new domain subdir
      (`behavior/cli/gherkin/{pdf,content,media,reporting}/README.md`) listing the features it
      contains.
      Each top-level skeleton: ~5 lines per template. Verify relative-link depth via
      `validate:specs-links`.
  - _Suggested executor: `specs-maker`_
- [ ] Execute the per-domain `git mv` block from
      [tech-docs.md §R1 Step 2](./tech-docs.md#r1--crane-flat-gherkin--behaviorcligherkindomain)
      verbatim against the on-disk file list (re-confirmed in Phase 0). Acceptance: every
      `.feature` lives under `specs/apps/crane/behavior/cli/gherkin/<domain>/<feature>.feature`
      and `specs/apps/crane/gherkin/` no longer exists; no `.feature` directly under
      `behavior/cli/gherkin/`.
- [ ] Run the path-reference sweep — execute the bash block verbatim from
      [tech-docs.md §R1 Step 3](./tech-docs.md#r1--crane-flat-gherkin--behaviorcligherkindomain)
      (`grep -rln ... | xargs sed -i.bak ...; find . -name '*.bak' -delete`). Then hand-check
      any per-`.feature` references in `apps/crane-cli/tests/unit/steps/` and rewrite to the
      new `<domain>/` path. Acceptance: `grep -rln 'specs/apps/crane/gherkin[^/c]' . | wc -l`
      returns 0 AND no per-file reference cites the old flat path.
- [ ] Edit `specs/apps/crane/README.md`: rewrite the "Structure" block to show the canonical
      CLI-only five-folder tree with `behavior/cli/gherkin/{pdf,content,media,reporting}/`
      subdirs. Update the "Running the Tests" code block step paths.
  - _Suggested executor: `specs-maker`_
- [ ] Verify locally inside the worktree:
      `nx run rhino-cli:validate:specs-tree --apps crane && nx run rhino-cli:validate:specs-counts --apps crane && nx run rhino-cli:validate:specs-links --apps crane`
      — all three exit 0.
- [ ] Verify crane unit + integration tests still pass:
      `nx run crane-cli:test:unit && nx run crane-cli:test:integration` — both exit 0.
- [ ] Commit atomically:
      `git add -A && git commit -m "refactor(specs/crane): migrate to canonical CLI tree with domain subdirs"`.

## Phase 3 — Rhino fill-out AND domain regrouping (atomic commit per R2)

- [ ] Create missing C4 folders:
      `mkdir -p specs/apps/rhino/{product,system-context,containers,components/cli}`.
- [ ] Create CLI-gherkin domain subdirs:
      `mkdir -p specs/apps/rhino/behavior/cli/gherkin/{agents,ddd,docs,env,git,repo-governance,spec-coverage,test-coverage,workflows,system}`.
      Adjust the subdir list if Phase 0 D5 resolution added or removed any domains.
- [ ] Execute the prefix-driven `git mv` loops from
      [tech-docs.md §R2 Step 3](./tech-docs.md#r2--rhino-add-missing-top-level-folders-and-regroup-feature-files-into-domain-subdirs)
      verbatim. After loops complete, run
      `find specs/apps/rhino/behavior/cli/gherkin -maxdepth 1 -name '*.feature'` —
      output MUST be empty. If any `.feature` remains at the root, hand-place it into the
      correct domain subdir before continuing.
- [ ] Author skeleton READMEs per [tech-docs.md §R3](./tech-docs.md#r3--skeleton-readme-template)
      at:
      `specs/apps/rhino/product/README.md`,
      `specs/apps/rhino/system-context/README.md`,
      `specs/apps/rhino/containers/README.md`,
      `specs/apps/rhino/components/cli/README.md`,
      plus a one-paragraph index `README.md` in each new domain subdir.
  - _Suggested executor: `specs-maker`_
- [ ] Edit `specs/apps/rhino/README.md` and
      `specs/apps/rhino/behavior/cli/gherkin/README.md`: update the "Structure" blocks to
      show all five top-level folders AND the new domain subdir layout.
- [ ] Run the path-reference sweep — capture `grep -rln
'specs/apps/rhino/behavior/cli/gherkin/' apps libs .github .husky docs repo-governance >
/tmp/rhino-spec-refs.txt`, inspect, and rewrite every per-`.feature` reference to its
      new `<domain>/<feature>.feature` path. Pre-push will fail loudly if any reference is
      stale.
- [ ] Verify:
      `nx run rhino-cli:validate:specs-tree --apps rhino && nx run rhino-cli:validate:specs-counts --apps rhino && nx run rhino-cli:validate:specs-links --apps rhino`
      — all three exit 0.
- [ ] Verify rhino-cli unit + integration tests:
      `nx run rhino-cli:test:quick && nx run rhino-cli:test:integration` — both exit 0.
- [ ] Commit atomically:
      `git add -A && git commit -m "refactor(specs/rhino): fill out CLI tree and regroup features into domains"`.

## Phase 4 — Ayokoding build-tools resolution

> Branch on D1 resolution recorded in Phase 0.

### Phase 4.A — If D1 == A (migrate under `behavior/build-tools/gherkin/`)

- [ ] Locate the surface-allowlist constant in rhino-cli. Authoritative search:
      `grep -rn '"cli"\|"be"\|"web"' apps/rhino-cli/src/commands/specs_validate_tree.rs apps/rhino-cli/src/internal/specs.rs`.
      Likely owner: `apps/rhino-cli/src/internal/specs.rs` (helpers) or
      `apps/rhino-cli/src/commands/specs_validate_tree.rs` (validator entry). [Repo-grounded —
      both files confirmed via `find` at plan-authoring time]
- [ ] Edit the surface enum/allowlist to add `"build-tools"` as a valid surface in whichever of
      the two files above owns the rule. Acceptance: `cargo check --manifest-path
apps/rhino-cli/Cargo.toml` exits 0.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Add a `#[cfg(test)]` unit test in the same Rust file: scenario "build-tools surface
      accepted by validate-tree". Run `nx run rhino-cli:test:quick` — new test passes; coverage
      remains ≥90%.
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

## Phase 6 — CLI domain regrouping for ayokoding-cli, ose-platform-cli + validator enforcement (R7)

Three atomic commits — two structural migrations and one validator/convention update —
landing the universal "every `.feature` lives under a `<domain>/` subdir" rule across the
last two CLI trees and hardening rhino-cli so the rule is enforced going forward.

### Phase 6.a — ayokoding-cli domain regrouping (R7.a)

- [ ] Create domain subdirs:
      `mkdir -p specs/apps/ayokoding/behavior/cli/gherkin/{system,links}` (adjust per Phase 0
      D5 override if any).
- [ ] Execute `git mv` per [tech-docs.md §R7](./tech-docs.md#r7--domain-regrouping-for-ayokoding-cli-ose-platform-cli-and-validator-enforcement):
      `check-all.feature` and `version.feature` into `system/`, `links-check.feature` into
      `links/`.
- [ ] Author one-paragraph index `README.md` in each new domain subdir.
  - _Suggested executor: `specs-maker`_
- [ ] Path-reference sweep: `grep -rln 'specs/apps/ayokoding/behavior/cli/gherkin/' apps libs
.github .husky docs repo-governance > /tmp/ayko-cli-refs.txt`. Inspect; hand-rewrite
      every per-`.feature` reference (likely in `apps/ayokoding-cli/`'s step files +
      `project.json` `inputs`).
- [ ] Verify: `nx run rhino-cli:validate:specs-tree --apps ayokoding && nx run
rhino-cli:validate:specs-counts --apps ayokoding && nx run
rhino-cli:validate:specs-links --apps ayokoding && nx run ayokoding-cli:test:quick` —
      all exit 0.
- [ ] Commit atomically: `git add -A && git commit -m "refactor(specs/ayokoding): regroup cli
features into domain subdirs"`.

### Phase 6.b — ose-platform-cli domain regrouping (R7.b)

- [ ] Create domain subdir: `mkdir -p specs/apps/ose-platform/behavior/cli/gherkin/links`
      (single-feature domain).
- [ ] `git mv specs/apps/ose-platform/behavior/cli/gherkin/links-check.feature
specs/apps/ose-platform/behavior/cli/gherkin/links/links-check.feature`.
- [ ] Author one-paragraph index `README.md` in the new `links/` subdir.
  - _Suggested executor: `specs-maker`_
- [ ] Path-reference sweep: `grep -rln 'specs/apps/ose-platform/behavior/cli/gherkin/' apps
libs .github .husky docs repo-governance > /tmp/osep-cli-refs.txt`. Inspect; hand-rewrite
      every per-`.feature` reference (likely in `apps/ose-cli/`'s step files + `project.json`
      `inputs`).
- [ ] Verify: `nx run rhino-cli:validate:specs-tree --apps ose-platform && nx run
rhino-cli:validate:specs-counts --apps ose-platform && nx run
rhino-cli:validate:specs-links --apps ose-platform && nx run ose-cli:test:quick` —
      all exit 0.
- [ ] Commit atomically: `git add -A && git commit -m "refactor(specs/ose-platform): regroup
cli features into domain subdirs"`.

### Phase 6.c — Validator enforcement + convention update (R7.c)

This is the apps/rhino-cli code change that locks in the new "no flat CLI" rule plus the
governance line that authorizes it.

- [ ] Read the current rule sites in rhino-cli: - `apps/rhino-cli/src/commands/specs_validate_tree.rs` — top-level shape validator - `apps/rhino-cli/src/internal/specs.rs` — shared helpers (`required_spec_folders`,
      `walk_feature_files`, etc.)
      Identify where the current CLI-flat carve-out lives (likely a conditional skipping the
      "no .feature directly under gherkin/" check when surface == "cli").
- [ ] Edit the validator so that for ANY surface (be, web, cli, build-tools), a `.feature`
      file directly under `behavior/<surface>/gherkin/` (zero domain levels) emits a HIGH
      finding with category `Spec Tree Shape Compliance` and message
      `"flat feature file at <path>; expected behavior/<surface>/gherkin/<domain>/<feature>.feature"`.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Add a `#[cfg(test)]` unit test in the same file proving the rule fires for a synthetic
      flat CLI tree and stays silent for a domain-grouped tree. Coverage gate (≥90%) must
      still pass via `nx run rhino-cli:test:quick`.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Update `apps/rhino-cli/src/commands/specs_validate_counts.rs` if it carries a separate
      assumption that CLI gherkin can be flat — change to expect each `<domain>/` subdir to
      contain ≥1 `.feature`. Add unit test mirroring the change.
  - _Suggested executor: `swe-rust-dev`_
- [ ] Edit
      [`repo-governance/conventions/structure/specs-directory-structure.md`](../../../repo-governance/conventions/structure/specs-directory-structure.md):
      (a) In the Canonical App Spec Tree code block, replace the line
      `└── <command>.feature    # Flat structure — no domain dirs`
      with
      `└── <domain>/                # Domain subdir, same rule as be/web`
      `└── <command>.feature`.
      (b) In §Domain Subdirectory Rules, replace the CLI-exception paragraph (currently lines
      184–193) with "Every surface (BE, web, CLI) uses domain subdirectories. Single-feature
      domains are permitted when the CLI surface area is small."
      (c) Append a dated §Migration Path retirement note: "CLI-flat exception retired
      (YYYY-MM-DD): crane, rhino, ayokoding-cli, and ose-platform-cli all regrouped under
      `behavior/cli/gherkin/<domain>/`."
  - _Suggested executor: `repo-rules-maker`_
- [ ] Run `nx run rhino-cli:validate:specs-tree` (no `--apps` flag) — exits 0 across every
      app in `AppsWithDDD` plus every other in-scope spec area.
- [ ] Run `npm run lint:md` — exits 0.
- [ ] Commit atomically: `git add -A && git commit -m "feat(rhino-cli): enforce domain
subdirs under every behavior/<surface>/gherkin/"`.

## Phase 7 — Governance Propagation (repo-rules-maker)

After structural migrations land (Phases 2–6), propagate the new uniform state into governance
and agent documentation so future contributors and agents read a consistent story. This phase
is delegated to the `repo-rules-maker` agent — it owns `repo-governance/` and is the only
agent authorized to write rules and conventions there per
[Agent Naming Convention](../../../repo-governance/conventions/structure/agent-naming.md).

- [ ] Invoke `repo-rules-maker` with the brief in [§Propagation Brief](#propagation-brief)
      below. Pass the brief verbatim.
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

### Propagation Brief

Pass the following brief verbatim to `repo-rules-maker` when invoking the propagation step
above.

Driven by plan `plans/in-progress/specs-tree-uniform/`. Phases 2–6 have landed: crane is now
CLI-canonical with domain subdirs, rhino has the full CLI-only surface profile with features
regrouped under `behavior/cli/gherkin/<domain>/`, ayokoding `build-tools` is resolved per
Decision D1 (see callout at top of `delivery.md`), ayokoding-cli and ose-platform-cli CLI
gherkin trees use domain subdirs, the CLI-flat exception has been retired in
`specs-directory-structure.md` by R7.c, and the `AppsWithDDD` allowlist policy is settled per
Decision D2. Update the remaining governance surfaces to match:

1. **`repo-governance/conventions/structure/specs-directory-structure.md`** — already partly
   updated by R7.c (the CLI-flat-exception retirement). In this propagation step also:
   (a) Append a dated migration-history note in §Migration Path recording the crane, rhino,
   ayokoding/build-tools, and CLI-domain-subdir moves (mirror the existing "DDD relocation
   (2026-05-09)" note style at lines 273–278).
   (b) If D1 == A: add `build-tools` to the `<surface>` enum description (currently
   "be, web, or cli") and document the rationale.
   (c) If D1 == B: add `build-tools` to the canonical perspective-slug list (sibling of `api`)
   with rationale.
   (d) Update any remaining examples / per-surface tables that still show flat CLI gherkin
   as canonical.
2. **`repo-governance/conventions/structure/app-readme-vs-specs.md`** — refresh the Adoption
   Matrix and any per-app examples that cite crane, rhino, ayokoding, or `ose-app` if they
   still reference pre-migration paths.
3. **`AGENTS.md` Project Structure tree** — update `specs/` block if it documents legacy
   paths; cross-check against the new root `specs/README.md`.
4. **`.claude/agents/specs-checker.md`** — refresh Category 1 (Structural Completeness)
   enumeration of required folders and Category 8 (Spec Tree Shape Compliance). The
   "CLI Gherkin feature file placed in a domain subdirectory under `behavior/cli/gherkin/`
   (should be flat)" finding (currently HIGH) MUST be flipped to its inverse: "CLI Gherkin
   feature file placed DIRECTLY under `behavior/cli/gherkin/` (should be in a domain subdir)".
5. **`.claude/agents/specs-maker.md` and `.claude/agents/specs-fixer.md`** — refresh any path
   examples that cited the legacy crane/rhino/ayokoding-cli/ose-platform-cli layouts. Examples
   in those agents that explicitly call out "CLI is flat" must be rewritten.
6. **`.claude/skills/repo-syncing-with-ose-primer/SKILL.md`** — confirm the extraction scope
   for crane/rhino/ayokoding paths still resolves; update if any old path is referenced.
7. **`docs/reference/related-repositories.md` and `docs/reference/platform-bindings.md`** —
   quick grep for any stale path references to `specs/apps/crane/gherkin/`,
   `specs/apps/ayokoding/build-tools/`, or any of the four flat CLI gherkin paths; update if
   found.
8. **Repo-wide .md sweep — every other markdown file in the repository.** Run the discovery
   greps below from the repo root and update every hit so future contributors (and every new
   app added to the workspace) inherit the uniform structure by default:

   ```bash
   # Catch every .md that references the old CLI flat pattern, legacy slugs, or stale paths.
   grep -rln --include='*.md' \
     -e 'cli/gherkin/' \
     -e 'flat structure' \
     -e 'flat-root' \
     -e 'specs/apps/crane/gherkin' \
     -e 'specs/apps/ayokoding/build-tools' \
     -e 'no domain dirs' \
     . \
     | grep -v node_modules | grep -v '/.next/' | grep -v generated-reports
   ```

   The files surfaced at plan-authoring time (2026-05-23) include — but are not limited to —
   the following. Re-run the grep at execution time; the live result is authoritative.
   - `docs/reference/monorepo-structure.md`
   - `docs/how-to/add-new-app.md` (new-app onboarding — MUST teach the domain-subdir layout)
   - `docs/reference/project-dependency-graph.md`
   - `docs/explanation/software-engineering/automation-testing/tools/playwright/{bdd,configuration}.md`
   - `docs/explanation/software-engineering/development/test-driven-development-tdd/integration-testing-standards.md`
   - `docs/explanation/software-engineering/programming-languages/typescript/testing.md`
   - `repo-governance/development/infra/{ci-conventions,nx-targets,bdd-spec-test-mapping,temporary-files}.md`
   - `repo-governance/development/quality/{three-level-testing-standard,specs-application-sync,feature-change-completeness}.md`
   - `repo-governance/workflows/specs/specs-quality-gate.md`
   - `repo-governance/workflows/repo/repo-ose-primer-extraction-execution.md`
   - `repo-governance/conventions/structure/{README,deterministic-vs-ai-validation-split,app-readme-vs-specs,ose-primer-sync}.md`
   - `repo-governance/conventions/writing/{dynamic-collection-references,readme-quality}.md`
   - `repo-governance/principles/general/simplicity-over-complexity.md`
   - `repo-governance/conventions/hugo/{ayokoding,ose-platform}.md` (legacy Hugo — may simply
     need stale-flag rather than rewrite)
   - `apps/ayokoding-cli/README.md`, `apps/rhino-cli/README.md`, `apps/ose-cli/README.md`,
     `apps/crane-cli/README.md` — per-app READMEs must show the post-migration spec path
   - `.claude/agents/{specs-checker,specs-maker,specs-fixer,web-research-maker,repo-ose-primer-propagation-maker}.md`
   - `.claude/skills/{repo-syncing-with-ose-primer/SKILL,repo-syncing-with-ose-primer/reference/extraction-scope,repo-syncing-with-ose-primer/reference/transforms,apps-organiclever-web-developing-content/SKILL}.md`

   For each hit:
   - If the file documents the spec tree as canonical (READMEs, conventions, agent
     specifications), rewrite to show the universal `behavior/<surface>/gherkin/<domain>/`
     layout.
   - If the file uses a path as an example in unrelated content (e.g., language tutorials
     mentioning Go monorepos), update the path only if it currently points at a relocated
     file; leave the surrounding prose alone.
   - **Exclusions** (do NOT modify):
     - `apps/ayokoding-web/.next/**` — Next.js build output (regenerated by the dev server).
     - `apps/ayokoding-web/content/**` — educational tutorials whose examples are
       independent of this repo's spec layout. Touch only when a literal `specs/apps/...`
       reference is broken.
     - `generated-reports/**` — historical audit output; preserved as-is.
     - `plans/done/**` — historical plans; rewriting these falsifies history. Add a brief
       note at the top of any obviously misleading file IF and ONLY IF a future reader would
       follow stale guidance.

**Out of scope**: do NOT re-author the migration recipes (they live in this plan's
`tech-docs.md`); do NOT modify any `specs/` file (already migrated); do NOT introduce new
conventions, only update existing ones; do NOT re-edit `apps/rhino-cli/` source (R7.c already
did that — flag any further code change needed as a separate finding for human triage); do
NOT alter `apps/ayokoding-web/content/**` tutorials unless a literal repo-internal path
reference resolves to nothing.

## Phase 8 — Local Quality Gates (Before Push)

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
- [ ] Do NOT bundle phases into a single commit — Phase 2 (crane), Phase 3 (rhino),
      Phase 4 (ayokoding build-tools), Phase 5 (ose-app + allowlist), Phase 6.a (ayokoding-cli
      domain regrouping), Phase 6.b (ose-platform-cli domain regrouping), Phase 6.c
      (validator + convention update), and Phase 7 (governance propagation) each produce
      separate atomic commits.

## Phase 9 — Post-Push Verification

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
