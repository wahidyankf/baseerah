# PRD — Specs Tree Uniformity Pass

## Product Overview

A structural cleanup of `specs/` that brings every app's spec tree onto the canonical
five-folder C4-aware layout and updates documentation so it matches reality. The product
output is **a uniform spec tree across the repo**, verified by every `rhino-cli specs
validate-*` Nx target.

## Personas

| Persona                             | Need                                                                                                         |
| ----------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| New contributor authoring a feature | Open `specs/README.md`, learn the layout, navigate to the right folder once                                  |
| Validator maintainer                | `validate:specs-*` succeeds across the entire repo, not just the allowlist subset                            |
| Repo-rules checker (agent)          | Five-folder tree is universal — no per-app exceptions to encode                                              |
| PM / TPM reading spec docs          | Same `product/` → `system-context/` → `containers/` → `components/` → `behavior/` reading order in every app |

## User Stories

### US-1 — Uniform root navigation

**As** a new contributor,
**I want** `specs/README.md` to document the canonical five-folder tree and list every current
app under `specs/apps/`,
**so that** my first read accurately maps to what I see on disk.

### US-2 — Crane on canonical layout

**As** the `validate:specs-tree` validator,
**I want** `specs/apps/crane/` to use `behavior/cli/gherkin/` instead of root-level `gherkin/`,
**so that** I can stop emitting `HIGH: flat-root artifact (gherkin/)` findings.

### US-3 — Rhino on full CLI-only surface profile

**As** a maintainer auditing the rhino-cli spec tree,
**I want** `specs/apps/rhino/` to include `product/`, `system-context/`, `containers/`, and
`components/cli/` (each with a `README.md`),
**so that** the tree matches the CLI-only row of the Per-Surface Variants table.

### US-4 — Ayokoding `build-tools/` slug retirement

**As** the ayokoding spec maintainer,
**I want** `specs/apps/ayokoding/build-tools/` either migrated under `behavior/build-tools/gherkin/`
or formally documented as a permanent perspective slug in
[Specs Directory Structure Convention](../../../repo-governance/conventions/structure/specs-directory-structure.md),
**so that** the "Out of scope for this spec tree (preserved unchanged as legacy slugs)" note in
[`specs/apps/ayokoding/README.md`](../../../specs/apps/ayokoding/README.md) lines 45–53 stops
being a TODO.

### US-5 — Allowlist coverage decision

**As** the validator-runner,
**I want** `apps/rhino-cli/src/internal/allowlist.rs` `AppsWithDDD` either to include `ose-app`
(when its DDD registry has ≥1 populated BC) or to carry an inline comment documenting its
intentional exclusion,
**so that** allowlist membership is principled rather than incidental.

### US-6 — Stale references in root README purged

**As** a reader of `specs/README.md`,
**I want** the libs section to list only libs that exist in `libs/` AND have specs (`golang-commons`,
`hugo-commons`, `web-ui`) with accurate descriptions, plus a note if `hugo-commons` is pending
decommission review,
**so that** I do not chase phantom paths.

### US-7 — Contracts location documented accurately

**As** a contract-author,
**I want** `specs/README.md` to state contracts live at `containers/contracts/`, not at app root
`contracts/`,
**so that** my mental model matches the on-disk pattern used by `organiclever` and `ose-app`.

### US-8 — Domain subdirs under every CLI `gherkin/`

**As** a contributor reading any app's `behavior/cli/gherkin/`,
**I want** `.feature` files grouped under domain subdirectories (matching the existing
organiclever `behavior/be/gherkin/<domain>/` pattern),
**so that** the layout is uniform across BE, web, and CLI surfaces — eliminating the
"CLI is special" carve-out that today's convention preserves at
[specs-directory-structure.md lines 184–193](../../../repo-governance/conventions/structure/specs-directory-structure.md).
This applies to crane (11 features), rhino (44 features), ayokoding-cli (3 features), and
ose-platform-cli (1 feature). The convention itself must be updated to drop the CLI-flat
exception; this is the governance-side change propagated by `repo-rules-maker`.

### US-9 — Every related .md file updated for uniform structure

**As** a contributor adding a brand-new app to the monorepo,
**I want** every governance doc, convention, agent definition, skill file, per-app README,
new-app how-to, and CI/Nx reference doc that mentions the spec tree to reflect the
post-migration domain-subdir layout,
**so that** the uniform structure propagates automatically into the next app I author — no
re-discovery, no consulting a historical migration plan, no second-class CLI layout. This is
the "make the new project inherit the rule" condition that closes the loop on this plan.

### US-10 — Governance and agents propagated by `repo-rules-maker`

**As** an AI agent (or human contributor) reading `repo-governance/`, agent definitions, or
`AGENTS.md` after this plan completes,
**I want** every cross-reference to crane / rhino / ayokoding-build-tools spec paths to reflect
the post-migration uniform layout,
**so that** governance, agents, and the codebase tell one story instead of three. Propagation
runs through `repo-rules-maker` (the only agent authorized to write `repo-governance/`) and is
validated by `repo-rules-checker`.

## Gherkin Acceptance Criteria

### AC-1 — Root README accuracy

```gherkin
Feature: specs/README.md reflects canonical structure

  Scenario: README documents the five-folder tree
    Given the file specs/README.md
    When I read the "Standard Folder Pattern" section
    Then it documents product/, system-context/, containers/, components/, behavior/
    And it documents contracts living at containers/contracts/
    And it does NOT reference a flat be/fe/fs/cli/gherkin/ pattern

  Scenario: README lists every current app
    Given the file specs/README.md
    When I read the "App Specs" section
    Then it lists ayokoding, crane, organiclever, ose-app, ose-platform, rhino, wahidyankf
    And every listed app has a working relative link to its README.md

  Scenario: README libs section matches libs/
    Given the file specs/README.md
    When I read the "Library Specs" section
    Then it lists exactly the libs present under specs/libs/
    And it does NOT list libs that do not exist in libs/

  Scenario: Cross-references resolve
    Given the file specs/README.md
    When `nx run rhino-cli:validate:specs-links` runs against the root
    Then it exits 0
```

### AC-2 — Crane migrated to canonical CLI layout

```gherkin
Feature: crane uses canonical CLI-only spec tree

  Scenario: No flat-root gherkin folder remains
    Given the directory specs/apps/crane
    When I list its top-level entries
    Then there is no entry named `gherkin/`
    And there is an entry named `behavior/cli/gherkin/`

  Scenario: All crane feature files live under the canonical path
    Given the directory specs/apps/crane
    When I `find . -name '*.feature'` from the crane app root
    Then every result lives under `behavior/cli/gherkin/`

  Scenario: Crane README documents the canonical layout
    Given the file specs/apps/crane/README.md
    When I read the "Structure" section
    Then it shows the five-folder tree with `behavior/cli/gherkin/`
    And it does NOT show `gherkin/` at the app root
```

### AC-3 — Rhino CLI-only surface profile populated

```gherkin
Feature: rhino spec tree matches the CLI-only surface profile

  Scenario: Required folders exist with README.md
    Given the directory specs/apps/rhino
    Then product/README.md exists
    And system-context/README.md exists
    And containers/README.md exists
    And components/cli/README.md exists
    And behavior/cli/gherkin/README.md exists

  Scenario: Tree validator passes for rhino
    Given the working tree
    When I run `nx run rhino-cli:validate:specs-tree --apps rhino`
    Then it exits 0
```

### AC-4 — Ayokoding build-tools slug resolved

```gherkin
Feature: ayokoding build-tools is either migrated or explicitly permanent

  Scenario: Migration path chosen — build-tools under behavior/
    Given the migration decision documented in tech-docs.md is "migrate under behavior/"
    Then specs/apps/ayokoding/build-tools/ does not exist
    And specs/apps/ayokoding/behavior/build-tools/gherkin/index-generation/ exists
    And specs/apps/ayokoding/README.md no longer carries the "legacy slugs" warning

  Scenario: Permanence path chosen — build-tools as a perspective slug
    Given the permanence decision documented in tech-docs.md
    Then specs-directory-structure.md adds `build-tools` to the list of perspective slugs
    And rhino-cli `validate-tree` accepts `build-tools/` at the app root for ayokoding
    And specs/apps/ayokoding/README.md replaces the "legacy slugs" warning with permanent-slug language
```

### AC-5 — Allowlist decision recorded

```gherkin
Feature: AppsWithDDD allowlist is principled

  Scenario: ose-app added to allowlist
    Given specs/apps/ose-app/ddd/bounded-contexts.yaml has ≥1 fully populated BC entry
    When I read apps/rhino-cli/src/internal/allowlist.rs
    Then the AppsWithDDD constant lists ose-app
    And `nx run rhino-cli:validate:specs-tree` (no --apps flag) exits 0

  Scenario: ose-app intentionally excluded
    Given specs/apps/ose-app/ddd/bounded-contexts.yaml has zero populated BC entries
    When I read apps/rhino-cli/src/internal/allowlist.rs
    Then the AppsWithDDD constant does NOT list ose-app
    And the file carries a comment above the constant explaining the exclusion criterion
```

### AC-6 — Validator gates green across changed apps

```gherkin
Feature: All four validate:specs-* targets pass at plan completion

  Scenario Outline: Each validator exits 0 against the AppsWithDDD allowlist
    Given the working tree at plan completion
    When I run `nx run rhino-cli:validate:specs-<target>`
    Then it exits 0 with no findings

    Examples:
      | target    |
      | adoption  |
      | tree      |
      | counts    |
      | links     |
```

### AC-7 — Pre-push hook green

```gherkin
Feature: pre-push validator does not regress

  Scenario: Pre-push runs cleanly for affected projects after migrations
    Given changes from this plan are committed
    When the pre-push hook runs `npx nx affected -t typecheck lint test:quick spec-coverage`
    Then it exits 0
```

### AC-8 — Domain subdirs under every CLI `gherkin/`

```gherkin
Feature: CLI gherkin trees use domain subdirectories

  Scenario Outline: Every CLI .feature file lives under a domain subdir
    Given the directory <cli-gherkin-root>
    When I list its direct children
    Then no entry is a `.feature` file
    And every `.feature` file lives at least one level deeper inside a domain subdirectory

    Examples:
      | cli-gherkin-root                                          |
      | specs/apps/crane/behavior/cli/gherkin                     |
      | specs/apps/rhino/behavior/cli/gherkin                     |
      | specs/apps/ayokoding/behavior/cli/gherkin                 |
      | specs/apps/ose-platform/behavior/cli/gherkin              |

  Scenario: Convention drops the CLI-flat exception
    Given the file repo-governance/conventions/structure/specs-directory-structure.md
    When I read the "Domain Subdirectory Rules" section
    Then it states CLI specs use domain subdirectories under `gherkin/`
    And the prior "CLI specs use a flat structure" rule (lines 184–193) is removed
    And the historical exception is documented in §Migration Path as a dated retirement note

  Scenario: validate-tree enforces the new shape
    Given the working tree at plan completion
    When I run `nx run rhino-cli:validate:specs-tree`
    Then it exits 0
    And the validator emits HIGH if any future commit places a `.feature` directly under
        any `behavior/<surface>/gherkin/`
```

### AC-9 — Repo-wide .md files reference uniform structure only

```gherkin
Feature: Every .md file in the repo reflects the post-migration spec structure

  Scenario: Discovery grep returns zero unintentional hits
    Given Phase 7 propagation has completed
    When I run from repo root:
      """
      grep -rln --include='*.md' \
        -e 'cli/gherkin/' \
        -e 'flat structure' \
        -e 'flat-root' \
        -e 'specs/apps/crane/gherkin' \
        -e 'specs/apps/ayokoding/build-tools' \
        -e 'no domain dirs' \
        . | grep -v node_modules | grep -v '/.next/' | grep -v generated-reports
      """
    Then every remaining hit is in one of the documented exclusion zones
        (plans/done/, apps/ayokoding-web/content/ educational tutorials,
         repo-governance/conventions/hugo/ stale Hugo content)
    And no hit is a live canonical reference an onboarding doc, agent, or skill would consume

  Scenario: New-app onboarding doc teaches uniform structure
    Given Phase 7 propagation has completed
    When I read docs/how-to/add-new-app.md
    Then the document shows the canonical five-folder tree with
        `behavior/<surface>/gherkin/<domain>/<feature>.feature`
    And it does NOT show a flat CLI gherkin example as canonical

  Scenario: Per-app READMEs show post-migration paths
    Given Phase 7 propagation has completed
    When I read each of apps/{crane-cli,rhino-cli,ayokoding-cli,ose-cli}/README.md
    Then every `specs/apps/...` path reference resolves to a real file on disk
    And no path cites the pre-migration flat layout

  Scenario: Agent / skill files cite uniform layout
    Given Phase 7 propagation has completed
    When I read each of .claude/agents/{specs-checker,specs-maker,specs-fixer}.md and
        .claude/skills/repo-syncing-with-ose-primer/SKILL.md
    Then no example path or validation rule references the retired CLI-flat layout
```

### AC-10 — Governance propagated to match migrated state

```gherkin
Feature: repo-rules-maker propagates the new uniform state into governance

  Scenario: Convention migration-history note appended
    Given Phases 2–6 of this plan have landed
    When repo-rules-maker completes its propagation per delivery.md Phase 7
    Then repo-governance/conventions/structure/specs-directory-structure.md §Migration Path
      contains a dated note recording the crane, rhino, ayokoding/build-tools, and
      CLI-domain-subdir migrations
    And the note follows the style of the existing "DDD relocation (2026-05-09)" entry

  Scenario: Agents and skills cite post-migration paths only
    Given repo-rules-maker has completed Phase 7
    When I `grep -rn 'specs/apps/crane/gherkin\|specs/apps/ayokoding/build-tools' \
      .claude/agents .claude/skills AGENTS.md repo-governance`
    Then the only hits are inside historical migration notes
    And no hit is a live path reference an agent or skill would consume

  Scenario: OpenCode mirror stays in sync
    Given any .claude/agents/*.md file was updated in Phase 7
    When I run `npm run sync:claude-to-opencode`
    Then it exits 0
    And `.opencode/agents/` carries the mechanical translation of the changed agents

  Scenario: repo-rules-checker validates the propagation
    Given Phase 7 governance commits are in the working tree
    When I run `repo-rules-checker` against the repo
    Then it exits 0 OR all findings are pre-existing and unrelated to this propagation
```

## Product Scope

### In Scope

- `specs/README.md` rewrite
- `specs/apps/crane/` migration to `behavior/cli/gherkin/<domain>/`
- `specs/apps/rhino/` fill-out to full CLI-only surface profile **plus** domain regrouping under
  `behavior/cli/gherkin/<domain>/`
- `specs/apps/ayokoding/build-tools/` decision + migration (or convention update)
- `specs/apps/ayokoding/behavior/cli/gherkin/` domain regrouping
- `specs/apps/ose-platform/behavior/cli/gherkin/` domain regrouping
- `repo-governance/conventions/structure/specs-directory-structure.md` — drop the CLI-flat
  exception, require domain subdirs for every surface
- `apps/rhino-cli/src/internal/allowlist.rs` allowlist update with inline rationale
- `apps/rhino-cli/src/specs/` validator update — flat `.feature` directly under
  `behavior/<surface>/gherkin/` now emits HIGH
- Per-app README updates for crane, rhino, ayokoding, ose-platform to reflect post-migration state
- README link integrity sweep across all migrated paths
- Repo-wide .md sweep — every governance doc, convention file, agent/skill definition,
  per-app README, BDD/testing reference, new-app how-to, monorepo-structure reference, and
  related markdown file updated to reflect the uniform structure. Discovered list in
  [tech-docs.md File Impact](./tech-docs.md#file-impact) + the repo-wide grep block in
  [delivery.md Phase 7 propagation brief](./delivery.md#propagation-brief)

### Out of Scope (Product Scope)

- Authoring new Gherkin scenarios (Coverage of ose-app BCs is a separate authoring plan)
- Renaming the `ose-platform` spec folder
- `libs/hugo-commons` decommissioning
- Specs-checker / specs-maker / specs-fixer agent rewrites
- Bumping `rhino-cli specs validate-*` target semantics
- Cleanup of `archived/rhino-cli/`

## Product Risks

- **Crane step definitions reference flat-root paths.** Any step-def file at
  `apps/crane-cli/tests/unit/steps/*` reading specs by glob will break if it hardcodes
  `gherkin/`. Mitigation: Grep `apps/crane-cli/` for `specs/apps/crane/gherkin` before the
  migration commit and update in the same atomic commit. [Repo-grounded]
- **Rhino tooling references the spec path** via Nx target inputs. Mitigation: search
  `apps/rhino-cli/project.json` and `tests/cli/` for `specs/apps/rhino/behavior/cli/gherkin/` —
  these paths do not change in the rhino migration so risk is low. [Repo-grounded]
- **Build-tools decision blocks ayokoding migration.** Resolved by treating decision as
  Step 0 of delivery.md — must close before any rename is committed.
- **Allowlist broadening surfaces real DDD findings** in ose-app. Treated as expected — finding
  triage is part of validator pass execution.
