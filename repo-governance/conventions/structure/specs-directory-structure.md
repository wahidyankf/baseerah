---
title: "Specs Directory Structure Convention"
description: Canonical C4-aware five-folder directory structure for specs/ — Gherkin feature files, C4 architecture diagrams, DDD artifacts, and OpenAPI contracts
category: explanation
subcategory: conventions
tags:
  - conventions
  - specs
  - gherkin
  - directory-structure
  - organization
  - c4-diagrams
  - openapi
  - c4
created: 2026-04-02
---

# Specs Directory Structure Convention

The `specs/` directory contains all behavioral specifications (Gherkin feature files), architectural diagrams (C4), domain design artifacts (DDD), and API contracts (OpenAPI) for the monorepo. This convention codifies the canonical C4-aware five-folder directory structure that all projects must follow.

The authoritative combined convention — covering what content belongs in app READMEs vs `specs/`, the five-folder tree shape, PM-readability requirements, and BDD/DDD/Contracts adoption expectations — is [App README vs Specs Convention](./app-readme-vs-specs.md). This document describes the canonical path patterns and domain subdirectory rules within the `behavior/` tree in detail, and defines how the overall spec tree is organized.

## Principles Implemented/Respected

This convention implements the following core principles:

- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**: The directory structure communicates spec scope through path segments. Reading a path like `specs/apps/organiclever/behavior/organiclever-be/gherkin/expenses/expense-management.feature` immediately reveals the project, C4 level, layer, domain, and feature without any external metadata.

- **[Simplicity Over Complexity](../../principles/general/simplicity-over-complexity.md)**: Every surface (BE, web, CLI) uses domain subdirectories under `gherkin/`. Single-feature domains are permitted for CLI surfaces with a small command surface area — the domain name still communicates the command group without requiring multiple files.

- **[Documentation First](../../principles/content/documentation-first.md)**: The specs directory serves as living documentation of system behavior. Gherkin features describe what the system does in human-readable language, C4 diagrams describe architectural context at three zoom levels, and OpenAPI contracts describe API surfaces.

## Conventions Implemented/Respected

This convention implements/respects the following conventions:

- **[App README vs Specs Convention](./app-readme-vs-specs.md)**: This directory structure is the canonical layout produced by applying the Content Split Rule from that convention. The five-folder tree IS the spec tree shape described there.

- **[Specs-Application Sync Convention](../../development/quality/specs-application-sync.md)**: The directory structure enables bidirectional sync between specs and application code. The path pattern mirrors the app/lib structure in the workspace.

- **[BDD Spec-Test Mapping](../../development/infra/bdd-spec-test-mapping.md)**: The Gherkin directory structure directly supports the mapping between feature files and test implementations across all three testing levels.

- **[Three-Level Testing Standard](../../development/quality/three-level-testing-standard.md)**: All three test levels (unit, integration, E2E) consume the same Gherkin specs from this directory structure. Only step implementations differ.

## Purpose

This convention establishes the canonical directory layout for the `specs/` directory. It defines how Gherkin feature files, C4 architecture diagrams, DDD artifacts, and OpenAPI contracts are organized across apps and libs, ensuring consistency, discoverability, and correct tool integration. The layout uses a C4-aware five-folder tree at the app level that maps directly to the C4 model zoom levels, with a `product/` folder for PM-first content and a `behavior/` folder for Gherkin that cuts across all C4 levels.

## Scope

### What This Convention Covers

- The C4-aware five-folder tree for app spec areas
- **Gherkin feature file placement** for apps (BE, FE/web, CLI) and libs, within the `behavior/` tree
- **Domain subdirectory rules** for grouping related feature files
- **C4 diagram placement** within `system-context/`, `containers/`, and `components/`
- **DDD artifact placement** within `components/<surface>/ddd/`
- **OpenAPI contract placement** within `containers/contracts/`
- **README.md index files** at each navigational level
- **Per-surface variants** (full-stack, web-only, CLI-only, multi-CLI)
- **Migration path** from flat-root layouts to the C4-aware tree

### What This Convention Does NOT Cover

- **Gherkin writing standards** (covered by [Acceptance Criteria Convention](../../development/infra/acceptance-criteria.md))
- **C4 diagram content** (covered by C4 model documentation within each project)
- **OpenAPI spec authoring** (covered by contract project documentation)
- **Test implementation** (covered by [Three-Level Testing Standard](../../development/quality/three-level-testing-standard.md))
- **Content split decisions** (what belongs in app README vs specs/) — see [App README vs Specs Convention](./app-readme-vs-specs.md)
- **PM-readability requirements** for spec files — see [App README vs Specs Convention](./app-readme-vs-specs.md)

## Canonical App Spec Tree

### Five-Folder Layout

Every app spec area under `specs/apps/<app-family>/` uses the following five-folder layout. Apps create only the folders they need — do not pre-create empty folders.

```
specs/apps/<app-family>/
├── README.md
├── product/                        # PM-first content (not a C4 level)
│   ├── README.md
│   └── overview.md
├── system-context/                 # C4 L1
│   ├── README.md
│   └── context.md
├── containers/                     # C4 L2
│   ├── README.md
│   ├── container.md
│   ├── contracts/                  # OpenAPI specs (full-stack only)
│   │   ├── README.md
│   │   ├── openapi.yaml
│   │   ├── paths/
│   │   ├── schemas/
│   │   └── generated/
│   └── deployment.md
├── components/                     # C4 L3
│   ├── README.md
│   ├── be/                         # Full-stack only
│   │   ├── README.md
│   │   ├── component-be.md
│   │   └── api.md
│   └── web/                        # Web and full-stack
│       ├── README.md
│       ├── component-web.md
│       ├── architecture.md
│       ├── design-system.md
│       └── routes-and-screens.md
├── ddd/                            # App-level DDD (when adopted)
│   ├── README.md
│   ├── bounded-contexts.yaml
│   ├── bounded-context-map.md
│   └── ubiquitous-language/
│       ├── README.md
│       └── <bc>.md
└── behavior/                       # Cross-cutting Gherkin (all C4 levels)
    ├── README.md
    └── <product>-<surface>/         # e.g., organiclever-be, ayokoding-www, rhino-cli
        └── gherkin/
            ├── README.md
            └── <domain>/            # Domain subdir — required for all surfaces
                └── <feature>.feature
```

### Folder Purposes

| Folder            | Reader question it answers                            | Why top-level                                                    |
| ----------------- | ----------------------------------------------------- | ---------------------------------------------------------------- |
| `product/`        | "What does this product do for the user?"             | PM-first content — not architecture, not behavior                |
| `system-context/` | "What is the system boundary? Who interacts with it?" | C4 L1                                                            |
| `containers/`     | "What runtime processes exist?"                       | C4 L2 — hosts API contracts and deployment topology              |
| `components/`     | "What is inside each container?"                      | C4 L3 — bounded contexts are components                          |
| `behavior/`       | "Does the system do what the specs say?"              | Gherkin cuts across all C4 levels — orthogonal to zoom hierarchy |

### Per-Surface Variants

| Surface profile                   | Folders populated                                                                                                                                    | Folders absent or empty                         |
| --------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------- |
| Full-stack (e.g., `organiclever`) | All five; `components/be/` + `components/web/` + `containers/contracts/`; `behavior/organiclever-be/gherkin/` + `behavior/organiclever-web/gherkin/` | None                                            |
| Web-only (e.g., `wahidyankf`)     | `product/`, `system-context/`, `containers/`, `components/web/`, `behavior/wahidyankf-www/gherkin/`                                                  | `containers/contracts/`, `components/be/`       |
| CLI-only (e.g., `rhino`)          | `product/`, `system-context/`, `containers/`, `components/cli/`, `behavior/rhino-cli/gherkin/`                                                       | `components/{be,web}/`, `containers/contracts/` |
| Multi-CLI (e.g., `ayokoding`)     | Same as CLI-only, plus web layers if applicable                                                                                                      | Nothing additional omitted                      |

## Gherkin Feature File Placement

Gherkin feature files live inside the `behavior/` tree at `specs/apps/<app-family>/behavior/<product>-<surface>/gherkin/`.

### Canonical Path Pattern

```
specs/apps/<app-family>/behavior/<product>-<surface>/gherkin/{domain}/{feature}.feature
```

Where:

- **`<app-family>`** = project name (e.g., `organiclever`, `ayokoding`, `rhino`)
- **`<product>-<surface>`** = flat slug combining product name and perspective (e.g.,
  `organiclever-be`, `ayokoding-www`, `rhino-cli`, `ayokoding-build-tools`)
- **`{domain}`** = business domain grouping folder (all surfaces, including CLI)
- **`{feature}`** = feature file name in kebab-case

Deprecated slugs (bare `be`, `web`, `cli`, `api`) must not be used for new surfaces; use the
`<product>-<surface>` compound form instead.

### Domain Subdirectory Rules

**Every surface** (BE, web, CLI) uses domain subdirectories under `gherkin/`. Each domain folder groups related feature files by business domain or command group, not by technical concern. Single-feature domains are permitted when the surface area is small.

Build-time features for `ayokoding` live under their own surface `ayokoding-build-tools/` —
renamed from the old bare `build-tools/` slug during the `standardize-app-spec-trees` plan.

```
specs/apps/organiclever/behavior/organiclever-be/gherkin/expenses/expense-management.feature
specs/apps/organiclever/behavior/organiclever-be/gherkin/authentication/password-login.feature
specs/apps/organiclever/behavior/organiclever-web/gherkin/authentication/google-login.feature
specs/apps/ayokoding/behavior/ayokoding-www/gherkin/accessibility/accessibility.feature
```

A domain folder may contain one or many feature files.

**CLI specs** use the same domain subdirectory rule as BE and web. Group features by command domain (e.g., `system/`, `env/`, `links/`). Single-feature domains are fine when the CLI surface area is small:

```
specs/apps/rhino/behavior/rhino-cli/gherkin/system/doctor.feature
specs/apps/rhino/behavior/rhino-cli/gherkin/env/env-backup.feature
specs/apps/rhino/behavior/rhino-cli/gherkin/spec-coverage/spec-coverage-validate.feature
specs/apps/ayokoding/behavior/ayokoding-cli/gherkin/links/links-check.feature
```

`rhino-cli specs validate-tree` enforces this rule: a `.feature` file placed directly under `behavior/<product>-<surface>/gherkin/` (with no domain subdirectory) is a HIGH finding.

## Lib Spec Structure

Library specs use a simpler layout with no five-folder tree — libs do not have C4 levels or behavioral architecture in the same sense as deployed apps.

```
specs/libs/<lib-name>/
├── README.md
└── gherkin/
    └── <package>/           # Package or module subdirectories
        └── <feature>.feature
```

**Examples:**

```
specs/libs/rust-commons/gherkin/links/check-links.feature
```

## Full Directory Structure

The complete `specs/` directory follows this layout:

```
specs/
├── README.md
├── apps/
│   └── <app-family>/         # C4-aware five-folder tree (per app above)
├── libs/
│   └── <lib-name>/
│       ├── README.md
│       └── gherkin/
│           └── <package>/
│               └── <feature>.feature
└── apps-labs/
    └── README.md             # Placeholder for experimental apps
```

### Which Projects Have Which Directories

Not every project has all directories. Presence of subdirectories depends on the project's surface profile:

- **`containers/contracts/`**: Present for apps with OpenAPI contract specs (e.g., `organiclever`)
- **`components/be/`**: Present for apps with a backend container (e.g., `organiclever`)
- **`ddd/`**: Present when DDD is adopted (lives at the app root, not under `components/web/`, because the ubiquitous language belongs to the bounded context, not to one implementation surface)
- **`behavior/<product>-be/gherkin/`**: Present for apps with backend Gherkin specs (e.g., `behavior/organiclever-be/gherkin/`)
- **`behavior/<product>-cli/gherkin/`**: Present for CLI apps (e.g., `behavior/rhino-cli/gherkin/`)

## README Index Files

Every directory within a spec area must contain a `README.md` index file. README files serve as entry points when browsing on GitHub, providing context about what specifications exist at each level. This follows the same pattern used throughout the repository — see [File Naming Convention](./file-naming.md).

The order of folders in any README listing follows the canonical order: `product/`, `system-context/`, `containers/`, `components/`, `behavior/`.

## Migration Path (Flat-Root to C4-Aware)

For existing spec trees with a flat-root layout (`be/`, `web/`, `cli/`, `c4/`, `contracts/` at the `specs/apps/<app-family>/` root):

1. Create the five top-level folders with placeholder `README.md` files.
2. In ONE atomic commit: `git mv` all old subfolders to their new positions. Update ALL path references in the same commit — rhino-cli path constants, Nx `project.json` `inputs`, step definition files, governance cross-links.
3. Update `specs/apps/<app-family>/README.md` to reflect the new tree.
4. Verify with `rhino-cli specs validate-tree <app>` and `npm run lint:md`.

**Flat-root to C4-aware path mapping:**

| Old path                               | New path                                           |
| -------------------------------------- | -------------------------------------------------- |
| `specs/apps/<app>/be/gherkin/`         | `specs/apps/<app>/behavior/<product>-be/gherkin/`  |
| `specs/apps/<app>/web/gherkin/`        | `specs/apps/<app>/behavior/<product>-web/gherkin/` |
| `specs/apps/<app>/cli/gherkin/`        | `specs/apps/<app>/behavior/<product>-cli/gherkin/` |
| `specs/apps/<app>/ddd/`                | `specs/apps/<app>/ddd/`                            |
| `specs/apps/<app>/c4/context.md`       | `specs/apps/<app>/system-context/context.md`       |
| `specs/apps/<app>/c4/container.md`     | `specs/apps/<app>/containers/container.md`         |
| `specs/apps/<app>/c4/component-be.md`  | `specs/apps/<app>/components/be/component-be.md`   |
| `specs/apps/<app>/c4/component-web.md` | `specs/apps/<app>/components/web/component-web.md` |
| `specs/apps/<app>/contracts/`          | `specs/apps/<app>/containers/contracts/`           |

**DDD relocation (2026-05-09).** An interim layout placed DDD artefacts at
`specs/apps/<app>/components/web/ddd/` (the row above used to point there). They were lifted
back to the app root because the ubiquitous language belongs to the bounded context, not to one
implementation surface. The current canonical location is `specs/apps/<app>/ddd/`. Apps still
on the interim path apply the same atomic-commit migration recipe (rhino-cli constants, Nx
inputs, every cross-link, governance) to relocate.

The atomic commit is mandatory — splitting the move and the path updates causes test failures between commits.

**CLI-flat exception retired (2026-05-23)**: crane, rhino, ayokoding-cli, and ose-cli
all regrouped under `behavior/<product>-cli/gherkin/<domain>/` during the `specs-tree-uniform`
pass. `rhino-cli specs validate-tree` now enforces domain subdirs for every surface.

**CLI domain-subdir moves (2026-05-23)**. As part of the `specs-tree-uniform` plan, four CLI
apps completed migration to the universal domain-subdir layout, then renamed to
`<product>-<surface>` during the `standardize-app-spec-trees` plan (2026-06-11):

- `crane` — regrouped into domain subdirs (`pdf/`, `content/`, `media/`, `reporting/`,
  `system/`); bare `cli/` renamed to `crane-cli/`.
- `rhino` — regrouped into domain subdirs (`agents/`, `system/`, `env/`, `git/`, `ddd/`,
  `docs/`, `spec-coverage/`, `test-coverage/`, `repo-governance/`, `workflows/`); bare `cli/`
  renamed to `rhino-cli/`.
- `ayokoding-cli` — features at `ayokoding-cli/gherkin/links/`; bare `cli/` renamed to
  `ayokoding-cli/`.
- `ose-cli` — bare `cli/` renamed to `ose-cli/` under `specs/apps/ose/behavior/`.

The bare `build-tools` surface was renamed to `ayokoding-build-tools` during the
`standardize-app-spec-trees` plan to follow `<product>-<surface>` naming; it remains an active
surface.

`ose` (merged from `ose-app` + `ose-platform`) was added to the `AppsWithDDD` allowlist. The
single source of truth is `apps/rhino-cli/src/internal/allowlist.rs`.

## Adding New Specs

### Adding a Feature File to an Existing Project

1. Identify the correct `<product>-<surface>` slug (e.g., `organiclever-be`, `ayokoding-www`,
   `rhino-cli`). For ayokoding build-time features, use `ayokoding-build-tools`
2. Place the file in the appropriate domain subdirectory under
   `behavior/<product>-<surface>/gherkin/<domain>/`, creating the domain folder if it does not exist
3. For CLI: choose a domain that matches the command group (e.g., `system/`, `env/`, `links/`); single-feature domains are permitted
4. Update the relevant `README.md` index file

### Adding Specs for a New Project

1. Create the project directory under `specs/apps/<app-family>/`
2. Create `README.md` at the project level
3. Determine the surface profile (full-stack, web-only, CLI-only, multi-CLI)
4. Create only the folders the project needs — see per-surface variant table
5. Create `README.md` index files at each folder level
6. Run `rhino-cli specs validate-tree <app>` to verify the layout

### Adding Specs for a New Lib

1. Create `specs/libs/<lib-name>/`
2. Create `README.md` at the lib level
3. Create `gherkin/` directly under the lib name (no five-folder tree)
4. Create package subdirectories under `gherkin/` matching the lib's module structure

## Enforcement

### Deterministic Validation (rhino-cli)

The following `rhino-cli specs` commands validate the directory structure mechanically:

| Command                                    | What it checks                                                             |
| ------------------------------------------ | -------------------------------------------------------------------------- |
| `rhino-cli specs validate-tree <app>`      | Top-level folders match the canonical five — no flat-root artifacts remain |
| `rhino-cli specs validate-counts <folder>` | README count claims match actual `.feature` file counts                    |
| `rhino-cli specs validate-links <folder>`  | Markdown link integrity within the spec tree                               |
| `rhino-cli specs validate-adoption <app>`  | BDD/DDD/Contracts adoption gaps per surface profile                        |

These commands run as part of the `specs-quality-gate` workflow deterministic-offload pass. See [Deterministic Offload](#deterministic-offload) below.

#### Allowlist-driven default app selection

`validate-adoption`, `validate-tree`, `validate-counts`, and `validate-links` all accept the same three calling shapes:

- Positional `<folder>` or `<app>` — single-target legacy behavior preserved.
- `--apps <csv>` — multi-app validation across an explicit list.
- No positional, no flag — defaults to the `AppsWithDDD` allowlist (`organiclever`, `ose`).

The single source of truth for the allowlist is `apps/rhino-cli/src/internal/allowlist.rs`. Pre-push and CI surfaces invoke the four targets without arguments so adding a new app is a one-line edit there.

#### Per-bounded-context `code_lang:` field (DDD validators)

`specs/apps/<app>/ddd/bounded-contexts.yaml` accepts an optional `code_lang: [<lang>, ...]` field per BC. Glossary code-identifier checks compute the file-extension glob list as the union of `SupportedLangGlobs[<lang>]` for every declared lang (e.g., `[fs]` → `*.fs`; `[ts, fs]` → `*.ts *.fs`). When omitted, the loader defaults to `[ts, tsx]` to preserve historical TS-only behaviour. Supported lang tags: `ts`, `tsx`, `fs`, `go`, `py`, `java`, `kt`, `rs`, `ex`, `exs`, `cs`, `clj`, `dart`.

#### Multi-perspective `gherkin: []string` schema

`specs/apps/<app>/ddd/bounded-contexts.yaml` accepts both scalar and list forms for the `gherkin:` field. A scalar auto-converts to a single-element list at load time:

```yaml
gherkin: behavior/organiclever-web/gherkin/content # scalar (most BCs)
gherkin: # list (multi-perspective BCs)
  - behavior/organiclever-web/gherkin/content
  - behavior/organiclever-be/gherkin/content
```

The validator iterates every declared path in `checkGherkin`, `registeredGherkin`, and `gherkinRoots`. Glossary `Used in features` lookups resolve under any declared path (first-match-wins). This unblocks BCs that legitimately have both web and be gherkin trees (e.g., ayokoding's `content`, `search`, `i18n`, `navigation`).

#### Severity audit log + env var

`OSE_RHINO_DDD_SEVERITY=warn` downgrades all `ddd bc` and `ddd ul` findings to warnings (exit 0 even when findings exist). Every honored downgrade emits a stderr audit line:

```
WARN: severity downgraded to "warn" via OSE_RHINO_DDD_SEVERITY env var
```

The legacy `ORGANICLEVER_RHINO_DDD_SEVERITY` env var was removed without a deprecation period in this same plan; every in-tree reference was renamed atomically. The flag form `--severity=warn|error` takes precedence over the env var.

#### Reverse-direction step orphan check (Fix #15)

Every `rhino-cli specs coverage` invocation enforces both directions:

- **Forward**: every Gherkin step has a matching impl.
- **Reverse**: every impl matcher has at least one matching Gherkin step.

Orphan impls fail the gate non-zero. There is no `--allow-orphan-steps` flag and no env var escape hatch — any orphan is either real drift or an extractor bug that must be fixed at source. The pre-flight audit ran across all 15 specs:coverage-wired projects in worktree as part of this plan and reached `FAIL=0` before merge.

The validator handles Scenario Outline forms in both directions: outline steps are emitted with `<placeholder>` tokens intact for forward matching against vitest-cucumber per-scenario impls, and Examples-table-expanded variants feed both directions so playwright-bdd regex-pattern impls binding expanded values count as covered. Comments in `.ts/.tsx` source are stripped before extraction (line comments only when at line start to preserve regex literals; block comments anywhere; strings preserved verbatim) so commented-out placeholder doc lines do not become false-positive orphan matches.

#### Combined gherkin scopes per app

`rhino-cli specs coverage` accepts a variadic specs-dirs list (`specs coverage <specs-dir> [<specs-dir>...] <app-dir>`). Apps with multiple gherkin perspectives (ose-www has web + api; ayokoding-www has web + api + cli) declare a single combined run in `project.json` so impls shared across scopes don't false-positive on per-scope orphan checks.

#### Expanded relationship symmetry (DDD validators)

`bcregistry/validator.go` flags asymmetric relationships for `customer-supplier`, `conformist`, `partnership`, and `shared-kernel` kinds. `anticorruption-layer` and `open-host-service` are intentionally one-way and silent. Unknown relationship kinds (e.g., typos like `shered-kernel`) produce an explicit "unknown relationship kind" finding via the new `checkRelationshipKinds` pass.

#### Drift detection

Drift detection commands (`drift-routes`, `drift-endpoints`, `drift-contracts`) are **not currently implemented**. The placeholder command files were removed in the BDD+DDD tooling gap-fill plan (2026-05) because reservation-pattern stubs that print "Not yet implemented" mislead callers into believing functionality exists. If drift detection is later required, a new plan adds those commands back as real implementations rather than stubs. Track via the tooling backlog.

#### Pre-push + CI gating surfaces

Every `validate:specs-*` target runs on all four gating surfaces — no surface lags behind:

- `.husky/pre-push` (every developer push, single line)
- `.github/workflows/commons-quality-gate.yml` (every PR, dedicated `specs-gate` job in `quality-gate.needs:`)
- `.github/workflows/_reusable-www-test-local-deploy.yml` (called by the www cron deploys, `specs-gate` job in `deploy.needs:`)
- `.github/workflows/organiclever-app-test-local-deploy-stag.yml` (cron on `main`, `specs-gate` job in `deploy.needs:`)

`docs validate-links` is NOT gated by this plan — it scans the entire repo's markdown (repo-governance/, docs/, app READMEs) and is owned by a separate planned validator-unification effort.

After this plan ships, **zero specs/BDD/DDD scripts in `rhino-cli` are dead** — every command file under `apps/rhino-cli/src/commands/specs_*.rs`, `src/commands/ddd_*.rs`, and `src/commands/spec_coverage*.rs` is invoked by at least one Nx target or pre-push surface.

### LLM Semantic Validation (specs-checker)

`specs-checker` validates categories that require semantic judgment: narrative coherence, terminology drift, C4 diagram consistency, cross-folder contradictions, and PM-readability compliance. See [Specs Validation Workflow](../../workflows/specs/specs-quality-gate.md).

### Deterministic Offload

The reasoning split between deterministic and LLM checks follows the principle that counting, path comparison, and file-system walking belong in Rust/deterministic tooling, not in LLM context. Categories tagged `[Deterministic]` in `specs-checker` shell out to `rhino-cli`; categories tagged `[LLM]` keep LLM-driven reasoning.

### Manual Verification Checklist

When reviewing changes to the `specs/` directory, verify:

- [ ] App spec tree uses the five-folder layout at the top level
- [ ] No flat-root artifacts remain (`be/`, `web/`, `cli/`, `c4/`, `contracts/` at root)
- [ ] BE, web, and CLI specs use domain subdirectories (never flat under `gherkin/`)
- [ ] Lib specs use package subdirectories under `gherkin/`
- [ ] `README.md` index files exist at every directory level
- [ ] New projects include only the folders their surface profile needs
- [ ] Folder listing in README follows canonical order: `product/`, `system-context/`, `containers/`, `components/`, `behavior/`

## Related Documentation

- [App README vs Specs Convention](./app-readme-vs-specs.md) — combined convention: content split rule, PM-readability contract, BDD/DDD/Contracts adoption
- [Specs-Application Sync Convention](../../development/quality/specs-application-sync.md) — bidirectional sync between specs and application code
- [BDD Spec-Test Mapping](../../development/infra/bdd-spec-test-mapping.md) — how specs map to test implementations
- [Three-Level Testing Standard](../../development/quality/three-level-testing-standard.md) — unit, integration, and E2E testing levels
- [Acceptance Criteria Convention](../../development/infra/acceptance-criteria.md) — Gherkin writing standards for feature files
- [File Naming Convention](./file-naming.md) — general file naming patterns
- [Plans Organization Convention](./plans.md) — similar convention for plans/ directory structure
- [Specs Validation Workflow](../../workflows/specs/specs-quality-gate.md) — iterative validation workflow
