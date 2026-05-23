# Tech Docs — Specs Tree Uniformity Pass

## Gap Inventory

Numbered references trace back to acceptance criteria in [prd.md](./prd.md).

| ID    | Location                                                                | Current state                                                   | Target state                                                                                                       | Severity | Source                                                                                                               |
| ----- | ----------------------------------------------------------------------- | --------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ | -------- | -------------------------------------------------------------------------------------------------------------------- |
| GAP-1 | `specs/README.md` "Standard Folder Pattern" section                     | Documents flat `be/fe/fs/cli/gherkin/`                          | Canonical five-folder tree + `behavior/<surface>/gherkin/`                                                         | HIGH     | [Repo-grounded — `specs/README.md` lines 46–73]                                                                      |
| GAP-2 | `specs/README.md` "App Specs" + "Library Specs" + "Standards" lists     | Missing: ose-app, wahidyankf, crane. Lib list partially correct | Lists every app present under `specs/apps/`                                                                        | HIGH     | [Repo-grounded — `specs/README.md` lines 29–45 vs `find specs/apps -maxdepth 1 -type d`]                             |
| GAP-3 | `specs/README.md` line 67                                               | "Contracts live at `specs/apps/{domain}/contracts/`"            | Contracts live at `specs/apps/{domain}/containers/contracts/`                                                      | HIGH     | [Repo-grounded — `specs-directory-structure.md` line 271 + `specs/apps/organiclever/containers/contracts/README.md`] |
| GAP-4 | `specs/apps/crane/`                                                     | Flat `gherkin/<feature>.feature` at app root                    | `behavior/cli/gherkin/<feature>.feature`                                                                           | HIGH     | [Repo-grounded — `specs/apps/crane/README.md` lines 16–31 vs `specs-directory-structure.md` lines 184–193]           |
| GAP-5 | `specs/apps/rhino/`                                                     | Only `behavior/cli/gherkin/` populated                          | CLI-only surface profile: `product/`, `system-context/`, `containers/`, `components/cli/`, `behavior/cli/gherkin/` | MEDIUM   | [Repo-grounded — `specs/apps/rhino/README.md` lines 18–24 vs `specs-directory-structure.md` line 151]                |
| GAP-6 | `specs/apps/ayokoding/build-tools/`                                     | Legacy flat-root slug containing `gherkin/index-generation/`    | Migrated under `behavior/build-tools/gherkin/` OR documented as permanent slug                                     | MEDIUM   | [Repo-grounded — `specs/apps/ayokoding/README.md` lines 45–53]                                                       |
| GAP-7 | `apps/rhino-cli/src/internal/allowlist.rs` `AppsWithDDD`                | Lists `organiclever`, `wahidyankf`, `ose-platform`, `ayokoding` | Inline-commented rationale for include/exclude per app; ose-app evaluated                                          | LOW      | [Repo-grounded — `apps/rhino-cli/src/internal/allowlist.rs` exists per `find` output]                                |
| GAP-8 | `specs/apps/ose-app/README.md` "For Product / Project Managers" section | Absent (organiclever has equivalent section)                    | Present with reading-order guidance                                                                                | LOW      | [Repo-grounded — `specs/apps/ose-app/README.md` lines 1–66 vs `specs/apps/organiclever/README.md` lines 168–197]     |

## Target Structure per App

The five-folder tree row in
[specs-directory-structure.md §Per-Surface Variants](../../../repo-governance/conventions/structure/specs-directory-structure.md#per-surface-variants)
is authoritative. Mapping every in-scope app to its declared profile:

| App          | Surface profile              | Required folders                                                                                         | Has today                                 | Action                                                                               |
| ------------ | ---------------------------- | -------------------------------------------------------------------------------------------------------- | ----------------------------------------- | ------------------------------------------------------------------------------------ |
| ayokoding    | Multi-CLI                    | `product/`, `system-context/`, `containers/`, `components/{web,api}/`, `behavior/{web,api,cli}/gherkin/` | All present + legacy `build-tools/`       | Resolve `build-tools/` per Decision D1 below; otherwise no structural action         |
| crane        | CLI-only                     | `product/`, `system-context/`, `containers/`, `components/cli/`, `behavior/cli/gherkin/`                 | `gherkin/<feature>.feature` only          | Full migration + new product/system-context/containers/components READMEs            |
| organiclever | Full-stack                   | All five plus `containers/contracts/`, `components/{be,web}/`, `behavior/{be,web}/gherkin/`              | Compliant                                 | No structural action                                                                 |
| ose-app      | Full-stack                   | Same as organiclever                                                                                     | Compliant                                 | Add "For Product / Project Managers" section (GAP-8); evaluate allowlist add (GAP-7) |
| ose-platform | Web-only + perspective `api` | `product/`, `system-context/`, `containers/`, `components/{web,api}/`, `behavior/{web,api}/gherkin/`     | Compliant (legacy `cli/` already retired) | No structural action                                                                 |
| rhino        | CLI-only                     | `product/`, `system-context/`, `containers/`, `components/cli/`, `behavior/cli/gherkin/`                 | Only `behavior/cli/gherkin/`              | Add four missing folders, each with `README.md` skeleton                             |
| wahidyankf   | Web-only                     | `product/`, `system-context/`, `containers/`, `components/web/`, `behavior/web/gherkin/`                 | Compliant                                 | No structural action                                                                 |

## Decisions

### D1 — Ayokoding `build-tools/` slug fate

**Options**:

- **D1.A — Migrate under `behavior/build-tools/gherkin/`.** Mechanically simplest; consistent with the
  "all Gherkin under `behavior/<surface>/gherkin/`" rule. Risk: `build-tools` is not in the canonical
  surface enum (`be`, `web`, `cli`) so `validate-tree` may reject it as a non-canonical surface.
  Requires rhino-cli code change to accept `build-tools` as a valid surface.
- **D1.B — Promote to permanent perspective slug in convention.** Update
  [specs-directory-structure.md](../../../repo-governance/conventions/structure/specs-directory-structure.md)
  to list `build-tools` alongside `api` as a permitted perspective slug at app root. No code change needed.
  Risk: deviates from the "all behavior under `behavior/`" rule and re-opens the precedent the
  Migration Path section explicitly closed.
- **D1.C — Inline migration into existing `behavior/cli/gherkin/`** since build-tools scripts execute as
  CLI invocations under the same `ayokoding-cli` binary. Risk: conflates two different test surfaces
  (binary CLI command behavior vs build-time index generation).

**Recommendation**: **D1.A**. Add `build-tools` to the canonical surface enum in `rhino-cli`'s
`validate-tree` (one line change in surface allowlist; locate via
`grep -rn 'be\|web\|cli' apps/rhino-cli/src/specs`). The "behavior cuts across all C4 levels"
principle from [specs-directory-structure.md line 143](../../../repo-governance/conventions/structure/specs-directory-structure.md)
extends naturally to a build-time surface. [Judgment call]

**Resolution required before Step 2 of delivery.md.** Default to D1.A unless validator-runner objects
during execution.

### D2 — Allowlist policy for `AppsWithDDD`

**Options**:

- **D2.A — Add `ose-app` to allowlist.** Surfaces any latent findings. Aligned with "every full-stack
  app with a DDD registry is validated".
- **D2.B — Exclude `ose-app` until BC content lands.** Avoids noise from empty BCs (all four show
  `--` feature counts).

**Recommendation**: **D2.A** — add ose-app to the allowlist BUT in a separate commit AFTER all
other migrations land, so any latent findings can be triaged in isolation. Add inline `//`
comment in `allowlist.rs` documenting both included apps' rationale.
[Repo-grounded — `specs/apps/ose-app/README.md` lines 46–52 show all BCs at `--`]

**Resolution required at delivery.md Phase 5.**

### D3 — Crane's missing C4 layers

CLI-only profile requires `product/`, `system-context/`, `containers/`, `components/cli/`. Crane has
none. Authoring full content for these is out of scope per BRD Non-Goals.

**Recommendation**: Create each folder with a `README.md` skeleton that:

1. States the folder's purpose per convention.
2. Cites the canonical convention link.
3. Marks content as `_To be populated in a follow-up authoring plan_` per
   [plan-anti-hallucination.md](../../../repo-governance/development/quality/plan-anti-hallucination.md)
   refuse-on-uncertainty rule.

This satisfies `validate-counts` (folders exist) and `validate-tree` (canonical shape) without
forcing this structural plan to author technical content. [Judgment call]

### D4 — Rhino's missing C4 layers

Same recommendation as D3. Skeleton READMEs only; behavior preserved as the source of truth for
rhino-cli command contracts.

## Migration Recipes

### R1 — Crane: flat `gherkin/` → `behavior/cli/gherkin/`

```bash
# All commands run inside the worktree at worktrees/specs-tree-uniform/
cd worktrees/specs-tree-uniform

# Step 1 — create destination
mkdir -p specs/apps/crane/behavior/cli/gherkin
mkdir -p specs/apps/crane/product
mkdir -p specs/apps/crane/system-context
mkdir -p specs/apps/crane/containers
mkdir -p specs/apps/crane/components/cli

# Step 2 — git mv every .feature plus the gherkin README
git mv specs/apps/crane/gherkin/README.md         specs/apps/crane/behavior/cli/gherkin/README.md
git mv specs/apps/crane/gherkin/figure-check.feature      specs/apps/crane/behavior/cli/gherkin/figure-check.feature
git mv specs/apps/crane/gherkin/heading-check.feature     specs/apps/crane/behavior/cli/gherkin/heading-check.feature
git mv specs/apps/crane/gherkin/mermaid-validate.feature  specs/apps/crane/behavior/cli/gherkin/mermaid-validate.feature
git mv specs/apps/crane/gherkin/nesting-check.feature     specs/apps/crane/behavior/cli/gherkin/nesting-check.feature
git mv specs/apps/crane/gherkin/ocr-quality.feature       specs/apps/crane/behavior/cli/gherkin/ocr-quality.feature
git mv specs/apps/crane/gherkin/pdf-commands.feature      specs/apps/crane/behavior/cli/gherkin/pdf-commands.feature
git mv specs/apps/crane/gherkin/report-management.feature specs/apps/crane/behavior/cli/gherkin/report-management.feature
git mv specs/apps/crane/gherkin/skiplist-management.feature specs/apps/crane/behavior/cli/gherkin/skiplist-management.feature
git mv specs/apps/crane/gherkin/table-check.feature       specs/apps/crane/behavior/cli/gherkin/table-check.feature
git mv specs/apps/crane/gherkin/text-check.feature        specs/apps/crane/behavior/cli/gherkin/text-check.feature

# Step 3 — sweep all path references in the same commit
grep -rln 'specs/apps/crane/gherkin' apps libs .github .husky docs repo-governance \
  | xargs -I {} sed -i.bak 's|specs/apps/crane/gherkin|specs/apps/crane/behavior/cli/gherkin|g' {}
find . -name '*.bak' -delete

# Step 4 — author skeleton READMEs (see R3 template)
# Step 5 — update specs/apps/crane/README.md "Structure" block
# Step 6 — verify
nx run rhino-cli:validate:specs-tree --apps crane
nx run rhino-cli:validate:specs-counts --apps crane
nx run rhino-cli:validate:specs-links --apps crane

# Step 7 — atomic commit
git add -A
git commit -m "refactor(specs/crane): migrate to canonical CLI-only five-folder tree"
```

**Pre-flight verification** (mandatory before `git mv`): confirm the exact feature-file list with
`ls specs/apps/crane/gherkin/`. The list above is from a 2026-05-23 `find` and may drift before
execution. [Repo-grounded — verify at execution start]

### R2 — Rhino: add missing top-level folders

```bash
mkdir -p specs/apps/rhino/product
mkdir -p specs/apps/rhino/system-context
mkdir -p specs/apps/rhino/containers
mkdir -p specs/apps/rhino/components/cli

# Author skeleton READMEs via R3 template
# Update specs/apps/rhino/README.md "Structure" block
# Verify
nx run rhino-cli:validate:specs-tree --apps rhino
nx run rhino-cli:validate:specs-counts --apps rhino
```

### R3 — Skeleton README template

Verbatim contents for each placeholder `README.md` created in R1 and R2. Replace `<APP>` and
`<FOLDER>` per file.

```markdown
# <APP> — <FOLDER>

<one-line description of this C4 level for <APP>>

> _Skeleton placeholder. Substantive content to be authored in a follow-up plan._

See [Specs Directory Structure Convention](../../../../repo-governance/conventions/structure/specs-directory-structure.md)
for the canonical purpose of this folder.
```

Relative-link depth (`../../../../`) assumes the folder is one level below the app spec root.
For deeper folders (e.g., `components/cli/`) adjust to `../../../../../`. Verify resolution with
`validate:specs-links`.

### R4 — Ayokoding `build-tools/` migration (assuming D1.A)

```bash
# Add 'build-tools' to rhino-cli surface enum first (one-line edit)
# Locate via: grep -rn '"cli"\|"be"\|"web"' apps/rhino-cli/src/specs
# Add entry to the surface-allowlist constant; re-build rhino-cli; re-run validator

mkdir -p specs/apps/ayokoding/behavior/build-tools/gherkin
git mv specs/apps/ayokoding/build-tools/gherkin/* specs/apps/ayokoding/behavior/build-tools/gherkin/
rmdir specs/apps/ayokoding/build-tools/gherkin
rmdir specs/apps/ayokoding/build-tools

# Sweep references
grep -rln 'specs/apps/ayokoding/build-tools' apps libs .github .husky docs repo-governance \
  | xargs -I {} sed -i.bak 's|specs/apps/ayokoding/build-tools/gherkin|specs/apps/ayokoding/behavior/build-tools/gherkin|g' {}
find . -name '*.bak' -delete

# Update specs/apps/ayokoding/README.md — remove the "Out of scope" note for build-tools

# Verify
nx run rhino-cli:validate:specs-tree --apps ayokoding
nx run rhino-cli:validate:specs-counts --apps ayokoding
nx run rhino-cli:validate:specs-links --apps ayokoding
```

### R5 — Root README rewrite

In-place rewrite of `specs/README.md` Sections "Standard Folder Pattern", "App Specs",
"Experimental App Specs", "Library Specs". New content sketched in
[delivery.md Phase 1](./delivery.md#phase-1--root-readme-rewrite).

### R6 — Allowlist update

```rust
// apps/rhino-cli/src/internal/allowlist.rs
// Apps with a populated DDD bounded-context registry.
// Inclusion criterion: specs/apps/<app>/ddd/bounded-contexts.yaml exists AND has ≥1
// BC entry whose `code:` path resolves to actual layered source.
// ose-app: included as of <commit-sha> — BC content authoring tracked separately.
pub const APPS_WITH_DDD: &[&str] = &[
    "organiclever",
    "wahidyankf",
    "ose-platform",
    "ayokoding",
    "ose-app",  // added by specs-tree-uniform plan
];
```

[Repo-grounded — `apps/rhino-cli/src/internal/allowlist.rs` path verified via `find`. Constant
name and exact syntax MUST be confirmed by reading the file at execution start; the snippet
above is illustrative.]

## File Impact

| File                                                                            | Action                                                             |
| ------------------------------------------------------------------------------- | ------------------------------------------------------------------ |
| `specs/README.md`                                                               | Rewrite Standard Folder Pattern, App Specs, Library Specs sections |
| `specs/apps/crane/README.md`                                                    | Update Structure block + Running the Tests paths                   |
| `specs/apps/crane/gherkin/`                                                     | Deleted via `git mv`                                               |
| `specs/apps/crane/behavior/cli/gherkin/**`                                      | New location for every feature file                                |
| `specs/apps/crane/{product,system-context,containers,components/cli}/README.md` | New skeleton files                                                 |
| `specs/apps/rhino/{product,system-context,containers,components/cli}/README.md` | New skeleton files                                                 |
| `specs/apps/rhino/README.md`                                                    | Update Structure block to show full CLI-only tree                  |
| `specs/apps/ayokoding/README.md`                                                | Remove "Out of scope" legacy slug warning for `build-tools/`       |
| `specs/apps/ayokoding/build-tools/`                                             | Deleted via `git mv` (if D1.A chosen)                              |
| `specs/apps/ayokoding/behavior/build-tools/gherkin/**`                          | New location (if D1.A chosen)                                      |
| `specs/apps/ose-app/README.md`                                                  | Add "For Product / Project Managers" section                       |
| `apps/rhino-cli/src/internal/allowlist.rs`                                      | Add `ose-app` to allowlist + inline rationale comment              |
| `apps/rhino-cli/src/specs/<surface-enum>.rs` (if D1.A)                          | Add `build-tools` to canonical surface enum                        |
| `apps/crane-cli/tests/unit/steps/**`                                            | Update any hardcoded `specs/apps/crane/gherkin` path references    |
| `apps/crane-cli/project.json`                                                   | Update Nx target `inputs` referencing the spec path                |

The exact set of step-definition files and Nx config files touched by path sweeps is determined
by `grep -rln 'specs/apps/crane/gherkin' .` AND `grep -rln 'specs/apps/ayokoding/build-tools' .`
at execution start. Both greps are part of delivery.md Step 0.

## Path-Reference Sweep Discipline

Per [Specs Directory Structure Convention §Migration Path](../../../repo-governance/conventions/structure/specs-directory-structure.md#migration-path-flat-root-to-c4-aware):

> The atomic commit is mandatory — splitting the move and the path updates causes test failures
> between commits.

Mechanical rule: in any commit that runs `git mv` on a spec path, **the same commit MUST contain
all `sed`-driven path updates** for that path. Do not push between `git mv` and the sed sweep.

## Rollback

Each migration commit is atomic, so rollback is `git revert <commit-sha>` for any one of:

- Root README rewrite
- Crane migration
- Rhino fill-out
- Ayokoding build-tools migration
- Allowlist update

Reverting one commit does not require touching the others. Validator state returns to pre-commit
baseline because each commit's path references are self-contained. [Judgment call — assumes
sed sweep is exhaustive]

## Verification

Per AC-6 in [prd.md](./prd.md):

```bash
nx run rhino-cli:validate:specs-adoption
nx run rhino-cli:validate:specs-tree
nx run rhino-cli:validate:specs-counts
nx run rhino-cli:validate:specs-links
```

All four must exit 0. If any emits HIGH findings, fix the offending app inside the worktree
before moving to the next phase.

Additionally:

```bash
npm run lint:md
npx nx affected -t typecheck lint test:quick spec-coverage
```

Both must exit 0 before push (pre-push hook also enforces).

## Open Questions

- _Should `apps-labs/README.md` move to a different location or be deleted entirely now that it
  documents itself as empty?_ — `_Unverified — confirm with maintainer at execution start._`
- _Is `libs/hugo-commons` still actively used by any Nx target?_ — `_Unverified — out of scope
for this plan; opened as a follow-up if specs/libs/hugo-commons cross-references break._`
