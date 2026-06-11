# Tech Docs — Standardize App Spec Trees

All paths below are `[Repo-grounded]` (verified via `Glob`/`Grep`/`Read` at authoring,
2026-06-11) unless labelled otherwise.

## Current State

Two spec trees serve the `apps/ose-*` family:

```
specs/apps/ose-app/                      # GRC application (app.oseplatform.com)
├── product/  system-context/  components/  ddd/
├── containers/contracts/                # Nx project "ose-app-contracts"
└── behavior/
    ├── be/gherkin/{ai-orchestration,gap-analysis,health,internal-policy,regulatory-source}
    └── web/gherkin/{smoke}

specs/apps/ose-platform/                 # marketing site (oseplatform.com) + ose-cli
├── product/  system-context/  components/  ddd/  containers/
└── behavior/
    ├── api/gherkin/{content,health,rss-feed,search,seo}
    ├── web/gherkin/{app-shell,landing}
    └── cli/gherkin/{links}
```

Both trees carry the same top-level C4 folders (`product`, `system-context`, `containers`,
`components`, `ddd`, `README.md`), which **collide** on a flat merge; `behavior/` collides on
`web/`; the platform tree uses `api` where the app tree uses `be`.

## Target State

```
specs/apps/ose/
├── README.md                            # unified OSE-family index (merged from both)
├── product/                             # unified product framing (app + platform sections)
├── system-context/                      # unified C4 L1 (all actors + external systems)
├── containers/                          # unified C4 L2 (all four deployables)
│   └── contracts/                       # Nx project "ose-contracts" (moved + renamed)
├── components/                          # unified C4 L3 (per-surface component docs)
├── ddd/                                 # unified DDD (all bounded contexts + one map)
└── behavior/
    ├── app-be/gherkin/{ai-orchestration,gap-analysis,health,internal-policy,regulatory-source}
    ├── app-web/gherkin/{smoke}
    ├── platform-be/gherkin/{content,health,rss-feed,search,seo}   # was platform "api"
    ├── platform-web/gherkin/{app-shell,landing}
    └── cli/gherkin/{links}              # normalized single canonical location
```

## Behavior Surface Migration Map

| Source                                          | Target (`git mv`)                               |
| ----------------------------------------------- | ----------------------------------------------- |
| `specs/apps/ose-app/behavior/be/gherkin/`       | `specs/apps/ose/behavior/app-be/gherkin/`       |
| `specs/apps/ose-app/behavior/web/gherkin/`      | `specs/apps/ose/behavior/app-web/gherkin/`      |
| `specs/apps/ose-platform/behavior/api/gherkin/` | `specs/apps/ose/behavior/platform-be/gherkin/`  |
| `specs/apps/ose-platform/behavior/web/gherkin/` | `specs/apps/ose/behavior/platform-web/gherkin/` |
| `specs/apps/ose-platform/behavior/cli/gherkin/` | `specs/apps/ose/behavior/cli/gherkin/`          |
| `specs/apps/ose-app/containers/contracts/`      | `specs/apps/ose/containers/contracts/`          |

## Consumer Reference Impact

Every reference below was located via `Grep` for `specs/apps/ose-app` / `specs/apps/ose-platform`
(2026-06-11). Each must be rewritten to the corresponding target path.

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

| File                                                         | What changes                                                             |
| ------------------------------------------------------------ | ------------------------------------------------------------------------ |
| `apps/ose-web-fe-e2e/project.json`                           | feature glob (L43) `web/gherkin`→`platform-web/gherkin`                  |
| `apps/ose-web-fe-e2e/playwright.config.ts`                   | `features` (L9)                                                          |
| `apps/ose-web-be-e2e/playwright.config.ts`                   | `api/gherkin`→`platform-be/gherkin`                                      |
| `apps/ose-web-be-e2e/.features-gen/**`                       | **regenerate** (playwright-bdd output) — do not hand-edit                |
| `apps/ose-web/test/unit/be-steps/search.steps.ts`            | feature path (L11) `api/gherkin`→`platform-be/gherkin`                   |
| `apps/ose-cli/README.md`                                     | feature paths (L62, L102, L105) → `specs/apps/ose/behavior/cli/gherkin/` |
| `apps/ose-cli/**` (Go test fixtures, if any reference paths) | re-grep during execution and rewrite any spec path                       |

> **Open question (verify in execution)**: `apps/ose-cli/README.md:62` cites
> `specs/apps/ose-platform/cli/` (a top-level `cli/` dir), but `find` at authoring time shows only
> `specs/apps/ose-platform/behavior/cli/gherkin/`. Confirm whether a second `cli/` location exists;
> if so, fold it into `specs/apps/ose/behavior/cli/gherkin/` (the normalize decision). If the
> README is stale, fix the README. `[Unverified]`

### Phase C — Framing merge + index

| File                                                                                                 | What changes                                                            |
| ---------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------- |
| `specs/apps/ose/{product,system-context,containers,components,ddd}/`                                 | author unified docs merging both products' content as labelled sections |
| `specs/apps/ose/README.md`                                                                           | unified family index (merge `ose-app` + `ose-platform` READMEs)         |
| `specs/README.md`                                                                                    | index rows (L32–33) `ose-app` + `ose-platform` → single `ose`           |
| `specs/apps/ose-app/ddd/bounded-contexts.yaml`, `specs/apps/ose-platform/ddd/bounded-context-map.md` | merge into `specs/apps/ose/ddd/`                                        |

### Phase D — Standard + docs sweep

| File                                                                 | What changes                                                                                                                                          |
| -------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------ |
| `repo-governance/conventions/structure/specs-directory-structure.md` | add "Multi-Deployable Family Layout" subsection + `be` perspective rule + `specs/apps/ose` worked example                                             |
| `repo-governance/conventions/structure/app-readme-vs-specs.md`       | cross-check for OSE-specific path references; update if present                                                                                       |
| `.claude/agents/specs-checker.md`                                    | add checks: one tree per family; surface-prefixed multi-deployable behavior; reject `api` perspective. Re-sync bindings (`npm run generate:bindings`) |
| `.claude/agents/specs-maker.md`                                      | document the multi-deployable template (if it enumerates layout)                                                                                      |
| `AGENTS.md`                                                          | no app-name change needed; verify no `specs/apps/ose-app                                                                                              | ose-platform` path strings need updating (grep at execution) |
| `repo-governance/`, `docs/` cross-refs                               | grep for `specs/apps/ose-app` / `specs/apps/ose-platform` and rewrite                                                                                 |

## Design Decisions & Rationale

- **Flat merge over nested sub-products** — keeps OSE visually aligned with the flat
  single-tree families (`organiclever`, etc.); the only deviation is surface-prefixed `behavior/`
  subtrees, which is the minimum disambiguation needed. Trade-off: the unified C4/DDD docs must
  carry two products' framing, handled by labelled sections rather than separate folders.
- **`be` over `api`** — the app tree already uses `be`; standardizing on it gives the merged
  tree one vocabulary and lets `specs-checker` enforce a single perspective name.
- **`git mv` for every relocation** — preserves blame/history across the move.
- **Phased (app then platform)** — smaller reviewable change surface; each phase independently
  green, so a pause after Phase A leaves the repo coherent.
- **Convention amendment is additive** — the existing single-deployable rule stays; the
  multi-deployable layout is a new subsection, avoiding contradiction.

## Toolchain Notes

- `spec-coverage` is driven by `rhino-cli spec-coverage validate <gherkin-dir> <project>`; it
  takes paths as arguments, so **no `rhino-cli` code change** is required — only the
  `project.json` command/input path strings.
- `ose-web-be-e2e` uses `playwright-bdd`; its `.features-gen/` directory is generated from
  `playwright.config.ts`. Regenerate (do not hand-edit) after the path/`api`→`be` change.
- Contracts codegen: `ose-app-be` (Rust) and `ose-app-web` (`@hey-api/openapi-ts`) both read
  `containers/contracts/generated/openapi-bundled.yaml`; both input paths move with the project.

## Rollback

Each phase is a discrete set of `git mv` + ref rewrites. To roll back a phase before its commit
is pushed: `git restore --staged --worktree specs/apps apps/` for the touched paths, or revert
the phase commit. Because moves use `git mv`, a revert restores the original tree exactly. No data
is deleted — only relocated.

## Open Questions

1. Second `ose-cli` `cli/` location — verify existence (see Phase B note). `[Unverified]`
2. Whether `specs-maker.md` enumerates per-family layout explicitly (if not, Phase D updates only
   `specs-checker.md` + the convention). `[Unverified]` — confirm by reading the agent file at
   execution.
