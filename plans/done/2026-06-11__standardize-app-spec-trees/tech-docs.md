# Tech Docs — Standardize App Spec Trees

All paths below are `[Repo-grounded]` (verified via `Glob`/`Grep`/`Read` at authoring, 2026-06-11)
unless labelled otherwise. Line numbers reflect the working tree at authoring time; the Phase 0
gate re-greps to reconcile any drift before edits begin.

## Cross-Repo Deviation Matrix

This plan is one of three parallel parity plans. The matrix below records every dimension where the
three repos align or deviate, with justification. It is embedded here verbatim per the multi-repo
parity workflow.

| Dimension                          | ose-public                                                      | ose-primer                                                                | ose-infra                         | Resolution + justification                                     |
| ---------------------------------- | --------------------------------------------------------------- | ------------------------------------------------------------------------- | --------------------------------- | -------------------------------------------------------------- |
| Multi-product family consolidation | ose-app + ose-platform -> ose                                   | none                                                                      | none                              | per-repo deviation: only ose-public has a multi-product family |
| Surface naming scheme              | flat product-surface                                            | flat product-surface                                                      | flat product-surface              | align: identical rule all repos                                |
| Backend perspective name           | be (rename api: ose-platform + ayokoding)                       | be (no api present)                                                       | be (no api present)               | align: be everywhere; only ose-public has api to fix           |
| Families restructured              | ose, organiclever, ayokoding, crane, rhino, wahidyankf          | crud, rhino                                                               | coralpolyp, rhino                 | per-repo: each repo restructures its own families              |
| Convention text authoring          | source-of-truth (authors amendment)                             | identical text (bidirectional/identity)                                   | independent copy, adapted wording | align public/primer; infra adapts (outside sync loop)          |
| Delivery mode                      | main-to-main                                                    | main-to-main (deviation from primer PR-only default, accepted: docs-only) | main-to-main                      | deviation recorded for primer                                  |
| Rationale doc location             | docs/explanation/standardize-app-spec-trees-parity-decisions.md | same                                                                      | same                              | align                                                          |
| Contracts project rename           | ose-app-contracts -> ose-contracts                              | none                                                                      | none                              | per-repo: only ose has a contracts rename                      |

## Current State (ose-public)

The current convention names behavior dirs bare-perspective:
`specs/apps/<family>/behavior/<surface>/gherkin/` where `<surface> = be | web | cli`
([`specs-directory-structure.md` L168](../../../repo-governance/conventions/structure/specs-directory-structure.md)).
On-disk behavior surfaces per family `[Repo-grounded]`:

```
specs/apps/ose-app/        behavior/{be,web}/gherkin/        + product/ system-context/ containers/contracts/ components/ ddd/ README
specs/apps/ose-platform/   behavior/{api,web,cli}/gherkin/   + product/ system-context/ containers/ components/ ddd/ README
specs/apps/organiclever/   behavior/{be,web}/gherkin/
specs/apps/ayokoding/      behavior/{api,web,cli,build-tools}/gherkin/
specs/apps/crane/          behavior/cli/gherkin/
specs/apps/rhino/          behavior/cli/gherkin/
specs/apps/wahidyankf/     behavior/web/gherkin/
```

Two trees serve `apps/ose-*`; two families use the non-standard `api` perspective
(`ose-platform`, `ayokoding`); ayokoding still keeps a separate `build-tools` surface on disk.

## Target State — Flat Product-Surface Scheme

Behavior dirs become `specs/apps/<family>/behavior/<product>-<surface>/gherkin/`:

```
specs/apps/ose/        behavior/{app-be,app-web,platform-be,platform-web,cli}/gherkin/
specs/apps/organiclever/ behavior/{organiclever-be,organiclever-web}/gherkin/
specs/apps/ayokoding/  behavior/{ayokoding-be,ayokoding-web,ayokoding-cli,ayokoding-build-tools}/gherkin/
specs/apps/crane/      behavior/crane-cli/gherkin/
specs/apps/rhino/      behavior/rhino-cli/gherkin/
specs/apps/wahidyankf/ behavior/wahidyankf-web/gherkin/
```

Consolidated OSE tree:

```
specs/apps/ose/
├── README.md                            # unified OSE-family index (merged from both)
├── product/                             # unified product framing (app + platform sections)
├── system-context/                      # unified C4 L1
├── containers/
│   └── contracts/                       # Nx project "ose-contracts" (moved + renamed)
├── components/                          # unified C4 L3 (per-surface)
├── ddd/                                 # unified DDD (all bounded contexts + one map)
└── behavior/
    ├── app-be/gherkin/{ai-orchestration,gap-analysis,health,internal-policy,regulatory-source}
    ├── app-web/gherkin/{smoke}
    ├── platform-be/gherkin/{content,health,rss-feed,search,seo}   # was platform "api"
    ├── platform-web/gherkin/{app-shell,landing}
    └── cli/gherkin/{links}              # normalized single canonical location
```

## Behavior Surface Migration Map (all families)

| Source                                               | Target (`git mv`)                                              |
| ---------------------------------------------------- | -------------------------------------------------------------- |
| `specs/apps/ose-app/behavior/be/gherkin/`            | `specs/apps/ose/behavior/app-be/gherkin/`                      |
| `specs/apps/ose-app/behavior/web/gherkin/`           | `specs/apps/ose/behavior/app-web/gherkin/`                     |
| `specs/apps/ose-platform/behavior/api/gherkin/`      | `specs/apps/ose/behavior/platform-be/gherkin/`                 |
| `specs/apps/ose-platform/behavior/web/gherkin/`      | `specs/apps/ose/behavior/platform-web/gherkin/`                |
| `specs/apps/ose-platform/behavior/cli/gherkin/`      | `specs/apps/ose/behavior/cli/gherkin/`                         |
| `specs/apps/ose-app/containers/contracts/`           | `specs/apps/ose/containers/contracts/`                         |
| `specs/apps/organiclever/behavior/be/gherkin/`       | `specs/apps/organiclever/behavior/organiclever-be/gherkin/`    |
| `specs/apps/organiclever/behavior/web/gherkin/`      | `specs/apps/organiclever/behavior/organiclever-web/gherkin/`   |
| `specs/apps/ayokoding/behavior/api/gherkin/`         | `specs/apps/ayokoding/behavior/ayokoding-be/gherkin/`          |
| `specs/apps/ayokoding/behavior/web/gherkin/`         | `specs/apps/ayokoding/behavior/ayokoding-web/gherkin/`         |
| `specs/apps/ayokoding/behavior/cli/gherkin/`         | `specs/apps/ayokoding/behavior/ayokoding-cli/gherkin/`         |
| `specs/apps/ayokoding/behavior/build-tools/gherkin/` | `specs/apps/ayokoding/behavior/ayokoding-build-tools/gherkin/` |
| `specs/apps/crane/behavior/cli/gherkin/`             | `specs/apps/crane/behavior/crane-cli/gherkin/`                 |
| `specs/apps/rhino/behavior/cli/gherkin/`             | `specs/apps/rhino/behavior/rhino-cli/gherkin/`                 |
| `specs/apps/wahidyankf/behavior/web/gherkin/`        | `specs/apps/wahidyankf/behavior/wahidyankf-web/gherkin/`       |

## Consumer Reference Impact — OSE

Located via `Grep` for `specs/apps/ose-app` / `specs/apps/ose-platform` (2026-06-11).

### Phase A — `ose-app` consumers (→ `app-be` / `app-web` / `ose-contracts`)

| File                                                   | What changes                                                                                                                    |
| ------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------- |
| `apps/ose-app-be/project.json`                         | contracts input (L13), spec-coverage inputs (L112–114: `be/gherkin`→`app-be/gherkin`, `ddd/...`), command (L127), inputs (L130) |
| `apps/ose-app-be/README.md`                            | spec links (L70, L75, L76)                                                                                                      |
| `apps/ose-app-be-e2e/project.json`                     | feature globs (L29, L44) `be/gherkin`→`app-be/gherkin`                                                                          |
| `apps/ose-app-be-e2e/README.md`                        | feature link (L19)                                                                                                              |
| `apps/ose-app-be-e2e/playwright.config.ts`             | `featuresRoot`/`features` (L5–6)                                                                                                |
| `apps/ose-app-be-e2e/steps/bounded-contexts.steps.ts`  | `Covers:` comments (L5–8)                                                                                                       |
| `apps/ose-app-be-e2e/steps/health.steps.ts`            | `Covers:` comment (L4)                                                                                                          |
| `apps/ose-app-web-e2e/project.json`                    | feature globs (L22, L44) `web/gherkin`→`app-web/gherkin`                                                                        |
| `apps/ose-app-web-e2e/README.md`                       | feature link (L20)                                                                                                              |
| `apps/ose-app-web-e2e/playwright.config.ts`            | `featuresRoot`/`features` (L5–6)                                                                                                |
| `apps/ose-app-web-e2e/steps/smoke.steps.ts`            | `Covers:` comment (L4)                                                                                                          |
| `apps/ose-app-web/project.json`                        | codegen `-i` path (L10), input (L14), spec-coverage cmd (L108: `web/gherkin`→`app-web/gherkin`), input (L111)                   |
| `apps/ose-app-web/README.md`                           | spec link (L38)                                                                                                                 |
| `apps/ose-app-web/src/contexts/*/README.md` (4 files)  | `ddd/ubiquitous-language` references                                                                                            |
| `specs/apps/ose-app/containers/contracts/project.json` | `name`→`ose-contracts`, `root` + all command paths → `specs/apps/ose/containers/contracts`                                      |

### Phase B — `ose-platform` consumers (→ `platform-be` / `platform-web` / `cli`)

| File                                              | What changes                                                             |
| ------------------------------------------------- | ------------------------------------------------------------------------ |
| `apps/ose-web-fe-e2e/project.json`                | feature glob (L43) `web/gherkin`→`platform-web/gherkin`                  |
| `apps/ose-web-fe-e2e/playwright.config.ts`        | `features` (L9)                                                          |
| `apps/ose-web-be-e2e/playwright.config.ts`        | `api/gherkin`→`platform-be/gherkin`                                      |
| `apps/ose-web-be-e2e/.features-gen/**`            | **regenerate** (playwright-bdd output) — do not hand-edit                |
| `apps/ose-web/test/unit/be-steps/search.steps.ts` | feature path (L11) `api/gherkin`→`platform-be/gherkin`                   |
| `apps/ose-cli/README.md`                          | feature paths (L62, L102, L105) → `specs/apps/ose/behavior/cli/gherkin/` |

> **Resolved (was open question)**: `apps/ose-cli/README.md:62` cites `specs/apps/ose-platform/cli/`
> (a top-level `cli/` dir). `find specs/apps/ose-platform -type d -name cli` at authoring shows ONLY
> `specs/apps/ose-platform/behavior/cli`. The README L62 reference is stale — there is no second
> `cli/` location. Phase B fixes the README to the canonical `specs/apps/ose/behavior/cli/gherkin/`.
> `[Repo-grounded]`

### Phase C — Framing merge + index (OSE)

| File                                                                                                 | What changes                                                            |
| ---------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------- |
| `specs/apps/ose/{product,system-context,containers,components,ddd}/`                                 | author unified docs merging both products' content as labelled sections |
| `specs/apps/ose/README.md`                                                                           | unified family index (merge `ose-app` + `ose-platform` READMEs)         |
| `specs/README.md`                                                                                    | index rows (L32–33) `ose-app` + `ose-platform` → single `ose`           |
| `specs/apps/ose-app/ddd/bounded-contexts.yaml`, `specs/apps/ose-platform/ddd/bounded-context-map.md` | merge into `specs/apps/ose/ddd/`                                        |

## Consumer Reference Impact — organiclever (→ `organiclever-be` / `organiclever-web`)

Located via `Grep specs/apps/organiclever/behavior` (2026-06-11). `[Repo-grounded]`

| File                                                      | What changes                                                                                                                                                                                                                                                                                                                                                   |
| --------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `apps/organiclever-be/project.json`                       | spec-coverage input (L112), command (L127), inputs (L130) `be/gherkin`→`organiclever-be/gherkin`                                                                                                                                                                                                                                                               |
| `apps/organiclever-be/README.md`                          | Gherkin spec link (L59)                                                                                                                                                                                                                                                                                                                                        |
| `apps/organiclever-be-e2e/project.json`                   | feature globs (L29, L44) `be/gherkin`→`organiclever-be/gherkin`                                                                                                                                                                                                                                                                                                |
| `apps/organiclever-be-e2e/README.md`                      | spec links (L7, L61)                                                                                                                                                                                                                                                                                                                                           |
| `apps/organiclever-be-e2e/playwright.config.ts`           | `featuresRoot`/`features` (L5–6)                                                                                                                                                                                                                                                                                                                               |
| `apps/organiclever-web/project.json`                      | inputs (L67, L88, L99), spec-coverage command (L122), inputs (L125) `web/gherkin`→`organiclever-web/gherkin`                                                                                                                                                                                                                                                   |
| `apps/organiclever-web/README.md`                         | Gherkin spec link (L60)                                                                                                                                                                                                                                                                                                                                        |
| `apps/organiclever-web-e2e/project.json`                  | feature globs (L22, L44) `web/gherkin`→`organiclever-web/gherkin`                                                                                                                                                                                                                                                                                              |
| `apps/organiclever-web-e2e/README.md`                     | spec links (L7, L59)                                                                                                                                                                                                                                                                                                                                           |
| `apps/organiclever-web-e2e/playwright.config.ts`          | `featuresRoot`/`features` (L5–6)                                                                                                                                                                                                                                                                                                                               |
| `apps/organiclever-web-e2e/steps/*.steps.ts` (14 files)   | `Covers:` comments citing `web/gherkin` — `routing` (L4, L6), `workout-session` (L4), `accessibility` (L4), `app-shell` (L4), `settings` (L5–7), `landing` (L4), `routine-management` (L4), `disabled-routes` (L4), `system-status-be` (L4), `progress-screen` (L4), `home-screen` (L4), `entry-loggers` (L4), `history-screen` (L4), `journal-mechanism` (L4) |
| `specs/apps/organiclever/behavior/web/gherkin/README.md`  | self-reference (L10) — moves with the dir; update embedded path                                                                                                                                                                                                                                                                                                |
| `specs/apps/organiclever/components/be/api.md`            | path ref (L60)                                                                                                                                                                                                                                                                                                                                                 |
| `specs/apps/organiclever/components/be/README.md`         | path refs (L62, L92, L100)                                                                                                                                                                                                                                                                                                                                     |
| `specs/apps/organiclever/components/be/component-be.md`   | path ref (L31)                                                                                                                                                                                                                                                                                                                                                 |
| `specs/apps/organiclever/components/web/README.md`        | path refs (L65, L84)                                                                                                                                                                                                                                                                                                                                           |
| `specs/apps/organiclever/components/web/component-web.md` | path ref (L116)                                                                                                                                                                                                                                                                                                                                                |
| `specs/apps/organiclever/ddd/bounded-context-map.md`      | path refs (L174, L197)                                                                                                                                                                                                                                                                                                                                         |
| `specs/apps/organiclever/containers/container.md`         | path refs (L47, L49)                                                                                                                                                                                                                                                                                                                                           |
| `specs/apps/organiclever/system-context/context.md`       | path ref (L30) `behavior/{be,web}/gherkin/`                                                                                                                                                                                                                                                                                                                    |

## Consumer Reference Impact — ayokoding (→ `ayokoding-be` / `ayokoding-web` / `ayokoding-cli` / `ayokoding-build-tools`)

Located via `Grep specs/apps/ayokoding/behavior` (2026-06-11). `[Repo-grounded]`
Note: there is **no `ayokoding-be` app** — the `api` surface is consumed by the fullstack
`ayokoding-web` Next.js app and its BE e2e suite. `[Repo-grounded]`

| File                                                                                    | What changes                                                                                                                                                                                                                                                         |
| --------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `apps/ayokoding-web/project.json`                                                       | inputs (L86: `api/gherkin`→`ayokoding-be/gherkin`; L87: `web/gherkin`→`ayokoding-web/gherkin`), spec-coverage command (L111: all three `--shared-steps` paths → `ayokoding-web/gherkin`, `ayokoding-be/gherkin`, `ayokoding-build-tools/gherkin`), inputs (L115–117) |
| `apps/ayokoding-web-be-e2e/project.json`                                                | feature glob (L43) `api/gherkin`→`ayokoding-be/gherkin`                                                                                                                                                                                                              |
| `apps/ayokoding-web-be-e2e/playwright.config.ts`                                        | `features` (L9) `api/gherkin`→`ayokoding-be/gherkin`                                                                                                                                                                                                                 |
| `apps/ayokoding-web-fe-e2e/project.json`                                                | feature glob (L43) `web/gherkin`→`ayokoding-web/gherkin`                                                                                                                                                                                                             |
| `apps/ayokoding-web-fe-e2e/playwright.config.ts`                                        | `features` (L9) `web/gherkin`→`ayokoding-web/gherkin`                                                                                                                                                                                                                |
| `apps/ayokoding-web/test/unit/be-steps/*.steps.ts`                                      | feature paths: `search-api` (L8), `navigation-api` (L8), `i18n-api` (L7), `content-api` (L8), `health-check` (L7) `api/gherkin`→`ayokoding-be/gherkin`; `index-generation` (L11) `build-tools/gherkin`→`ayokoding-build-tools/gherkin`                               |
| `apps/ayokoding-web/test/integration/be-steps/*.steps.ts`                               | feature paths: `search-api` (L11), `content-api` (L8), `navigation-api` (L8), `i18n-api` (L7), `health-check` (L7) `api/gherkin`→`ayokoding-be/gherkin`                                                                                                              |
| `apps/ayokoding-cli/README.md`                                                          | test-mapping path (L218) `cli/gherkin`→`ayokoding-cli/gherkin`                                                                                                                                                                                                       |
| `specs/apps/ayokoding/behavior/web/gherkin/README.md`                                   | self-reference (L10)                                                                                                                                                                                                                                                 |
| `specs/apps/ayokoding/behavior/api/gherkin/README.md`                                   | self-reference (L15)                                                                                                                                                                                                                                                 |
| `specs/apps/ayokoding/components/web/component-web.md`                                  | path ref (L109)                                                                                                                                                                                                                                                      |
| `specs/apps/ayokoding/components/web/README.md`                                         | path refs (L39, L59)                                                                                                                                                                                                                                                 |
| `specs/apps/ayokoding/components/api/component-api.md`                                  | path ref (L99)                                                                                                                                                                                                                                                       |
| `specs/apps/ayokoding/components/api/README.md`                                         | path refs (L48, L80, L86)                                                                                                                                                                                                                                            |
| `apps/ayokoding-web-fe-e2e/.features-gen/**`, `playwright-report/**`, `test-results/**` | regenerated artifacts — re-run e2e, do not hand-edit `[Repo-grounded]`                                                                                                                                                                                               |

## Consumer Reference Impact — crane (→ `crane-cli`)

Located via `Grep specs/apps/crane/behavior` (2026-06-11). `[Repo-grounded]`

| File                                        | What changes                                                                           |
| ------------------------------------------- | -------------------------------------------------------------------------------------- |
| `apps/crane-cli/project.json`               | inputs (L72, L85, L101), spec-coverage command (L98) `cli/gherkin`→`crane-cli/gherkin` |
| `apps/crane-cli/tests/unit/Suite.fs`        | default gherkin path (L12) `cli/gherkin`→`crane-cli/gherkin`                           |
| `apps/crane-cli/tests/integration/Suite.fs` | default gherkin path (L12) `cli/gherkin`→`crane-cli/gherkin`                           |

## Consumer Reference Impact — rhino (→ `rhino-cli`)

Located via `Grep specs/apps/rhino/behavior` (2026-06-11). `[Repo-grounded]` rhino-cli Rust source
hardcodes default spec paths — these are **code changes** (TDD-shaped), not doc edits.

| File                                                    | What changes                                                                                                                                                                          |
| ------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `apps/rhino-cli/project.json`                           | inputs (L73) `cli/gherkin`→`rhino-cli/gherkin`                                                                                                                                        |
| `apps/rhino-cli/README.md`                              | spec links (L9, L86)                                                                                                                                                                  |
| `apps/rhino-cli/src/commands/spec_coverage_validate.rs` | test default paths (L146, L160, L177) `cli/gherkin`→`rhino-cli/gherkin` — **code + test** change                                                                                      |
| `apps/rhino-cli/src/internal/specs.rs`                  | test fixture paths (L572, L586) use `behavior/cli/gherkin` placeholder — confirm whether these are family-agnostic fixtures (path `specs/apps/x/...`) or need updating `[Unverified]` |
| `specs/apps/rhino/README.md`                            | scaffold instruction path (L71) `cli/gherkin`→`rhino-cli/gherkin`                                                                                                                     |

> **Note on `specs.rs` L572/L586** `[Unverified]`: these use a synthetic `specs/apps/x/behavior/cli/gherkin`
> fixture inside a tempdir, not the real rhino tree. They likely do NOT need changing (the literal
> `cli` is the test author's arbitrary surface name, not the rhino family's surface). Confirm at
> execution: if the fixture is decoupled from the family naming scheme, leave it; otherwise rename.

## Consumer Reference Impact — wahidyankf (→ `wahidyankf-web`)

Located via `Grep specs/apps/wahidyankf/behavior` (2026-06-11). `[Repo-grounded]`

| File                                                       | What changes                                                                                                                                                                 |
| ---------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `apps/wahidyankf-web/project.json`                         | inputs (L47, L57, L70), spec-coverage command (L77), inputs (L80) `web/gherkin`→`wahidyankf-web/gherkin`                                                                     |
| `apps/wahidyankf-web/README.md`                            | spec links (L52, L69)                                                                                                                                                        |
| `apps/wahidyankf-web/test/unit/steps/*.steps.ts` (7 files) | feature paths: `cv` (L5), `accessibility` (L5), `search` (L5), `home` (L5), `personal-projects` (L7), `theme` (L5), `responsive` (L5) `web/gherkin`→`wahidyankf-web/gherkin` |
| `apps/wahidyankf-web-fe-e2e/project.json`                  | feature globs (L22, L44) `web/gherkin`→`wahidyankf-web/gherkin`                                                                                                              |
| `apps/wahidyankf-web-fe-e2e/playwright.config.ts`          | `featuresRoot` (L5) `web/gherkin`→`wahidyankf-web/gherkin`                                                                                                                   |
| `specs/apps/wahidyankf/behavior/web/gherkin/README.md`     | self-reference (L12)                                                                                                                                                         |

## Governance / Docs Cross-Ref Sweep (Phase G)

These governance and docs files cite old bare-surface or `api` paths and must be rewritten to the
flat product-surface form. Located via `Grep` across `repo-governance/`, `docs/`, `AGENTS.md`,
`.claude/` (2026-06-11). `[Repo-grounded]`

| File                                                                               | Lines / what changes                                                                                                                   |
| ---------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| `repo-governance/conventions/structure/specs-directory-structure.md`               | the amendment target (see Phase G) + example paths L179–193                                                                            |
| `repo-governance/conventions/structure/app-readme-vs-specs.md`                     | cross-check for surface-path examples; update any present                                                                              |
| `repo-governance/development/infra/bdd-spec-test-mapping.md`                       | rhino example paths L97–99, L144, L182, L223 `cli/gherkin`→`rhino-cli/gherkin`                                                         |
| `repo-governance/development/infra/ci-conventions.md`                              | organiclever paths L187–188, L378 `be/gherkin`→`organiclever-be/gherkin` and `web/gherkin`→`organiclever-web/gherkin`                  |
| `repo-governance/development/quality/specs-application-sync.md`                    | organiclever L159–160, L194; ayokoding L170 (note: cites `behavior/be/gherkin` but ayokoding uses `api` — reconcile to `ayokoding-be`) |
| `repo-governance/development/quality/feature-change-completeness.md`               | organiclever L145, L166                                                                                                                |
| `repo-governance/workflows/specs/specs-quality-gate.md`                            | organiclever L57                                                                                                                       |
| `repo-governance/conventions/structure/deterministic-vs-ai-validation-split.md`    | rhino L113                                                                                                                             |
| `repo-governance/conventions/writing/dynamic-collection-references.md`             | organiclever L166                                                                                                                      |
| `docs/explanation/software-engineering/automation-testing/tools/playwright/bdd.md` | organiclever L87–88, L295                                                                                                              |
| `.claude/agents/specs-checker.md`                                                  | example paths L63, L182 (also rule additions — see Phase G)                                                                            |
| `.claude/agents/specs-maker.md`                                                    | example path L58 + profile templates (see Phase G)                                                                                     |
| `.claude/agents/specs-fixer.md`                                                    | L46 bare-surface examples in "Missing top-level README.md" fix category; L127 fix report example path `behavior/be/README.md`          |
| `docs/reference/monorepo-structure.md`                                             | L50 tree diagram `behavior/cli/gherkin/` example → flat product-surface example (e.g. `behavior/rhino-cli/gherkin/`)                   |
| `docs/reference/project-dependency-graph.md`                                       | L200 `specs/apps/ose-platform/` → `specs/apps/ose/`                                                                                    |
| `docs/how-to/add-new-app.md`                                                       | L336 `behavior/be/gherkin/**/*.feature` template → flat product-surface example                                                        |
| `repo-governance/conventions/structure/ose-primer-sync.md`                         | L106 `specs/apps/ose-platform/**` → `specs/apps/ose/**`                                                                                |
| `repo-governance/development/infra/nx-targets.md`                                  | L561 `ose-cli` target glob `specs/apps/ose-platform/**/*.feature` → `specs/apps/ose/**/*.feature`                                      |
| `repo-governance/development/pattern/openapi-contract-first.md`                    | L67, L137 `specs/apps/ose-app/` → `specs/apps/ose/`                                                                                    |
| `.claude/skills/apps-organiclever-web-developing-content/SKILL.md`                 | L103, L399 `behavior/web/gherkin` → `behavior/organiclever-web/gherkin`                                                                |
| `.claude/skills/repo-syncing-with-ose-primer/reference/transforms.md`              | L27 `specs/apps/ose-platform/` → `specs/apps/ose/` in product-paths no-propagation list                                                |
| `.claude/agents/repo-ose-primer-propagation-maker.md`                              | L93 `specs/apps/ose-platform/` → `specs/apps/ose/` in no-neither-propagation safety invariant                                          |

> Some governance docs (e.g. `specs-application-sync.md` L170) describe ayokoding with a
> `behavior/be/gherkin` path that does not exist on disk today (ayokoding uses `api`). The sweep
> normalises these to the correct new flat product-surface path (`ayokoding-be`). `[Repo-grounded]`

## Design Decisions & Rationale

- **Flat product-surface over bare-surface** — a single, self-describing scheme where the dir name
  states both product and perspective. Multi-product families (OSE) need product tokens to avoid
  `web/`/`web/` collisions across products; single-product families use the family name as the
  product token so the scheme is uniform everywhere. Echo families read identically
  (`crane-cli`, `rhino-cli`, `wahidyankf-web`).
- **`be` over `api`** — the app tree already uses `be`; standardising gives every tree one
  vocabulary and lets `specs-checker` enforce a single perspective name.
- **`git mv` for every relocation** — preserves blame/history across the move.
- **Phased per family/group** — each phase is a natural pause: independently green, safe to stop,
  clean to resume. OSE (multi-product, most consumers) is split A/B/C; single-product families are
  grouped to keep each phase reviewable.
- **rhino source-default updates are TDD-shaped** — the Rust unit tests in `spec_coverage_validate.rs`
  hardcode the old path; the RED step names the new path as the expected default before the swap.
- **Convention amendment is additive + reusable** — the flat product-surface subsection replaces the
  bare-surface naming guidance but keeps the five-folder C4 structure; the text is authored so
  ose-primer can adopt it byte-identical.

## Toolchain Notes

- `spec-coverage` is driven by `rhino-cli spec-coverage validate <gherkin-dir> <project>`; it takes
  paths as arguments, so **no `rhino-cli` runtime code change** is required for OTHER families — only
  `project.json` strings. The rhino family itself does need source edits because its own unit-test
  defaults hardcode the rhino gherkin path. `[Repo-grounded]`
- playwright-bdd families (`ose-web-be-e2e`, `ayokoding-web-fe-e2e`, etc.) generate `.features-gen/`
  from `playwright.config.ts`. Regenerate (do not hand-edit) after each rename.
- Contracts codegen: `ose-app-be` (Rust) and `ose-app-web` (`@hey-api/openapi-ts`) both read
  `containers/contracts/generated/openapi-bundled.yaml`; both input paths move with the project.

## Rollback

Each phase is a discrete set of `git mv` + ref rewrites. To roll back a phase before its commit is
pushed: `git restore --staged --worktree specs/apps apps/` for the touched paths, or revert the
phase commit. Because moves use `git mv`, a revert restores the original tree exactly. No data is
deleted — only relocated.

## Open Questions

1. `apps/rhino-cli/src/internal/specs.rs` L572/L586 synthetic fixture (`specs/apps/x/behavior/cli/gherkin`)
   — confirm it is family-agnostic and needs no rename. `[Unverified]`
2. Whether `specs-maker.md` `surface-profile` templates need full rewrites or only example-path
   edits to express the flat product-surface scheme. `[Unverified]` — confirm by reading the agent
   file at execution (the profiles at L71–177 enumerate `behavior/<surface>/` layout).
