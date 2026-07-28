# Product Requirements — ayokoding-www Cost Reduction

> **WHAT gets built.** The business reasoning behind it lives in [`brd.md`](./brd.md); the method
> and the cited research snapshot live in [`tech-docs.md`](./tech-docs.md).

## Product overview

Seven independent engineering units of work inside `apps/ayokoding-www/` — no new routes, no new
screens — that together cut hosting and runtime cost. Each unit ships a PR-able delivery boundary
under the `worktree-to-pr` default:

1. **Quick wins (config/docs drift)** — pin `oxlint`, reconcile coverage threshold mismatch
   (`vitest.config.ts:80` vs `project.json:82`), fix the README feature-table omission of the
   `cost-of-living-calculator`, dedupe the two prebuild generators between `project.json` and
   `vercel.json`, and wire the existing `validate-indexes` target into `test:quick`.
2. **Dependency modernization** — adopt TypeScript 7 side-by-side (`npm:@typescript/typescript6` +
   `npm:typescript@^7.0.2`), bump Next.js to 16.3+ as the TS-7 experimental floor, and apply Path A
   (LTS) / Path B (60-day soak) patch bumps to `react`, `react-dom`, `zod`, `shiki`, and the
   `@trpc/*` family, exact-pinning each.
3. **Pagefind migration** — drop `flexsearch@0.7.43` and its 3 MB client-shipped search index;
   rewrite `src/features/search/` to load Pagefind's prebuilt static index from `public/pagefind/`;
   remove the `generate-search-data` Nx target and its `vercel.json` duplicate; drop the
   `serverExternalPackages: ["flexsearch"]` carve-out from `next.config.ts`.
4. **Mermaid build-time migration** — drop client `mermaid@11`; add `rehype-mermaid` with
   `strategy: "inline-svg"` to the rehype pipeline; diagrams render at build time (one shared
   Playwright/Chromium browser per build).
5. **`html-react-parser` removal** — audit `src/` for runtime usages; replace each with a build-time
   rehype pipeline step; drop the `html-react-parser` dependency entirely (closes a documented XSS
   surface).
6. **Calculator data lazy-load** — split `cities.ts` (79 KB) and `roles.ts` (76 KB) into separate
   chunks loaded via dynamic `import()` on calculator route entry, moving ~155 KB off the initial
   bundle.
7. **Docker base + trace narrowing** — switch the Dockerfile base from `node:24-alpine` to
   `node:24-slim` (per the Next.js `with-docker` example); narrow `outputFileTracingIncludes` from
   the `"/**"` glob to per-route globs derived from the actual `fs.readFile` call sites; audit
   `generated/**` and Codemod to drop what is truly unused.

Each unit ships independently-PR-able and merged-before-dark (where applicable) under the repo's
feature-flag default. The plan declines to introduce a feature flag for any of these units because
no unit adds a user-reachable surface change beyond replacing an existing runtime path with its
build-time equivalent (search, mermaid, html-react-parser) or splitting an existing chunk
(calculator), so each unit's revert is `git revert` of one PR.

## Personas

Solo-maintainer repository; the first three are hats the maintainer wears; the last is the audience.

| Persona                             | Goal                                                                    | Frustration this plan removes                                                                               |
| ----------------------------------- | ----------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| **Build / DevOps** (maintainer)     | Spend fewer Vercel build minutes per weekly course ship                 | Generators duplicated across `project.json` and `vercel.json`; `npx oxlint@latest` flake                    |
| **Frontend developer** (maintainer) | Ship a smaller, faster initial bundle without a content re-architecture | The 3 MB FlexSearch index baked in; the ~700 KB client `mermaid`; the 155 KB calculator data on first paint |
| **Repo governance owner**           | Audit that every dep bump carries its Path classification               | Caret-pinned `@trpc/*` and the stale `flexsearch@0.7` block the audit                                       |
| **Public reader** (the audience)    | See the same content load fast on a cold visit, with no diagram flash   | Bundle weight and hydration flashes; the cold-start cost of bundles shipped against a slow first visit      |

## User stories

- **US-1** — As a **build / DevOps maintainer**, I want `oxlint` installed as an exact-pinned
  `devDependency` so my CI lint invocation has no network round-trip and is reproducible.
- **US-2** — As a **build / DevOps maintainer**, I want a single source of truth for the prebuild
  generator commands so I stop tracking two divergent copies of the same build pipeline in
  `project.json` and `vercel.json`.
- **US-3** — As a **build / DevOps maintainer**, I want the `vitest.config.ts` coverage threshold
  to match the `project.json` flag so a green local run stays green on CI.
- **US-4** — As a **repo governance owner**, I want every dep bump in Phase 2 to carry a written
  Path A / Path B / Path C classification with the LTS-line or 60-day-soak evidence so I can audit
  each bump against the Dependency Bump Stability & Safety Policy.
- **US-5** — As a **repo governance owner**, I want `typescript` to resolve to TS 6 (for Next.js's
  JS-API build path) and a separate `typescript-7` package to give me the Go-native `tsc` for
  faster type-check builds.
- **US-6** — As a **frontend developer**, I want search powered by a prebuilt Pagefind index served
  from `public/pagefind/` so my client bundle no longer ships 3 MB on the search route.
- **US-7** — As a **frontend developer**, I want Mermaid diagrams rendered as static SVG at build
  time so the client stops shipping the ~700 KB `mermaid@11` library.
- **US-8** — As a **frontend developer**, I want runtime HTML parsing of content removed — content
  should be parsed at build time only under the rehype pipeline — so I close a documented XSS
  surface and drop a runtime dep.
- **US-9** — As a **frontend developer**, I want the 155 KB of hand-curated cost-of-living data
  split into lazy chunks so the route's initial bundle is ~155 KB lighter.
- **US-10** — As a **build / DevOps maintainer**, I want the Docker image on `node:24-slim` with a
  narrowed `outputFileTracingIncludes` so I lose the ~100 MB that the broad `"/**"` glob currently
  pulls into the trace.
- **US-11** — As a **public reader**, I want search to behave the same way it did before, just
  faster; diagrams to render before hydration instead of after; my calculator to still work; and
  the same content at the same URLs.
- **US-12** — As a **repo governance owner**, I want the README's source-layout feature table to
  list `cost-of-living-calculator` and to state `app-shell`'s zones correctly so newcomers stop
  paying the onboarding cost of a stale doc.

## UI design funnel

> **Exemption declaration** — this plan is **not UI-bearing**: it adds no user-facing screens under
> `apps/`. The Surface-Conditional Tester Gates
> ([plan-planning §3 surface-conditional tester gates](../../../repo-governance/workflows/plan/plan-planning.md#surface-conditional-tester-gates))
> still apply because Phases 3 / 4 / 5 / 6 / 7 each change behavior a user can reach (search,
> diagrams, content rendering, calculator initial paint, cold start). The plan therefore runs the
> [`ui/ui-quality-gate.md`](../../../repo-governance/workflows/ui/ui-quality-gate.md) **static** check
> against changed component source AND the
> [`web/web-ux-test-fixing-planning.md`](../../../repo-governance/workflows/web/web-ux-test-fixing-planning.md)
> **live-site** triad (EWT/UWT/DWT) before archival in Phase 8 — no UI mockup embeds are required
> under the [UI Mockups in Plan Docs convention](../../../repo-governance/conventions/formatting/diagrams.md#ui-mockups-in-plan-docs)
> because no new screens are added.

## Product scope

### In scope

| #    | Feature                                                                                                                                                                                                                                 |
| ---- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| F-1  | Pin `oxlint` as an exact-pinned `devDependency` in `apps/ayokoding-www/package.json`; rewrite the `lint` Nx target to invoke the local binary; remove `npx oxlint@latest` from `project.json`                                           |
| F-2  | Reconcile coverage threshold mismatch: align `vitest.config.ts` `lines: 80` and `project.json` `--coverage.thresholds.lines=82` to the same value                                                                                       |
| F-3  | Fix `apps/ayokoding-www/README.md:71-79` feature table — add `cost-of-living-calculator` row and correct the `app-shell` zone list                                                                                                      |
| F-4  | Dedupe the two prebuild generators — drive them only from `project.json` `build.dependsOn`; `vercel.json` falls back to `nx build` or an equivalent single command                                                                      |
| F-5  | Wire the existing `validate-indexes` Nx target into `test:quick` so silent index drift can no longer ship                                                                                                                               |
| F-6  | Adopt TypeScript 7 side-by-side: `devDependencies.typescript` = `"npm:@typescript/typescript6@^6.0.2"`, `devDependencies.typescript-7` = `"npm:typescript@^7.0.2"`; bump `next` to 16.3+                                                |
| F-7  | Re-point the `ayokoding-www:typecheck` Nx target to invoke the Go-native `tsc` (via the `typescript-7` alias or `tsgo`), keeping `next build`'s JS-API path on `@typescript/typescript6`                                                |
| F-8  | Apply every other Path A / Path B patch bump with a written classification: `react`, `react-dom`, `zod`, `shiki`, exact-pin all `@trpc/*` minors                                                                                        |
| F-9  | Pagefind migration — drop `flexsearch`; add `pagefind` devDep; rewrite `src/features/search/` to load Pagefind's prebuilt static index from `public/pagefind/`                                                                          |
| F-10 | Remove the `generate-search-data` Nx target from `project.json` and the duplicate invocation from `vercel.json`                                                                                                                         |
| F-11 | Drop `serverExternalPackages: ["flexsearch"]` from `next.config.ts`                                                                                                                                                                     |
| F-12 | Mermaid build-time migration — drop client `mermaid`; add `rehype-mermaid`; one shared Playwright/Chromium per build; emit static inline SVG                                                                                            |
| F-13 | `html-react-parser` removal — audit `src/` for runtime usages; replace with a build-time rehype pipeline step; drop the dependency                                                                                                      |
| F-14 | Calculator lazy-load — split `cities.ts` and `roles.ts` into dynamic-import chunks loaded on calculator route entry                                                                                                                     |
| F-15 | Docker base swap — `node:24-alpine` → `node:24-slim` per the Next.js with-docker example                                                                                                                                                |
| F-16 | Narrow `outputFileTracingIncludes` from `"/**"` to per-route globs derived from the actual `fs.readFile` call sites; audit `generated/**` for what's truly needed at runtime                                                            |
| F-17 | Companion Gherkin scenarios under `specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/cost-reduction/` for the behavior-changing units (search, mermaid, calculator), bound by vitest-cucumber unit + `playwright-bdd` e2e steps |

### Out of scope

| #      | Excluded                                                         | Why                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| ------ | ---------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| OOS-1  | Bilingual parity work for the `id` locale                        | 124-file stub vs 1,884 — warrants its own plan; not a runtime cost line                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| OOS-2  | ISR migration of the catch-all content route                     | Plan keeps **full SSG for every route whose render is compatible** — the catch-all `generateStaticParams` keeps enumerating all 1,884 content slugs; `dynamicParams = true` already covers only the manifest-driven `learn/paths/**` on-demand namespace. Preserves 0 ms first-visit and 0 ISR-metering. The ~3–12 min weekly build-minute saving an ISR migration would remove is deferred until a future plan measures actual build pressure. Note: the Vercel plan is **Pro ($20/mo)**, which buys 12-way build parallelism (the lever that matters for the weekly course-ship programme) but the same 45-min single-build cap as Hobby; build-minute headroom is ample today |
| OOS-3  | Build-time content ingestion (eliminate runtime `fs.readFile`)   | Bigger content-pipeline re-architecture than this scope tier permits                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| OOS-4  | Manual workspace hoisting fix in the Dockerfile                  | Proper `npm ls`-visible link rivals the trace-narrowing surface area; defer                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| OOS-5  | A11y overhaul of the calculator route's `jsx-a11y` suppressions  | Known debt; a focused A11y plan owns it, not this cost-reduction plan                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| OOS-6  | Replacing the misfiled `next-config-security.unit.test.ts`       | Unrelated smaller debt; not a cost-bundle line                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| OOS-7  | Lazy-loading `role-lookup.ts`'s lookup logic itself              | The lookup logic stays in the bundle; only the static data (`cities.ts`, `roles.ts`) is split into chunks                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| OOS-8  | Replacing the entire `oxlint` invocation with `biome` or similar | Out of scope; the plan keeps `oxlint` but pins it                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| OOS-9  | Adding runtime APM telemetry or a build-time budget mining step  | Visible at maintenance cost; deferred — the plan lands the cuts first, a future plan may measure them                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| OOS-10 | A `flexsearch 0.7 → 0.8` migration                               | Declined in favor of the Pagefind migration; the 0.7→0.8 step is breaking and would not retire the 3 MB client-index lever                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |

## Acceptance criteria

> Feature file: `specs/apps/ayokoding/behavior/ayokoding-www/gherkin/tools/cost-reduction/cost-reduction.feature`
>
> Tag routing — `@unit` binds a vitest-cucumber step in
> `apps/ayokoding-www/test/unit/fe-steps/cost-reduction.steps.tsx`; `@e2e` binds a `playwright-bdd`
> step in `apps/ayokoding-www-fe-e2e/src/steps/cost-reduction.steps.ts`. Every scenario needs a
> `@covers` annotation in its step implementation or `specs:behavior:coverage` fails.
>
> Scenarios are authored **incrementally, phase by phase**, alongside the step definitions that
> satisfy them — never all at once — because `specs:behavior:coverage` fails on any scenario without
> a step implementation, which would red every intervening phase gate. The phase that owns each
> scenario is named in [`delivery.md`](./delivery.md).

```gherkin
Feature: Cost reduction for ayokoding-www
```

### Quick wins (Phase 1)

```gherkin
  # AC-1 [@unit]
  Scenario: The lint target invokes an exact-pinned oxlint binary
    Given apps/ayokoding-www/package.json declares an exact-pinned oxlint devDependency
    When the ayokoding-www:lint target is invoked
    Then the lint invocation uses node_modules/.bin/oxlint
    But the lint invocation issues no network request

  # AC-2 [@unit]
  Scenario: Coverage thresholds agree across config files
    Given apps/ayokoding-www/vitest.config.ts declares the coverage threshold
    When the same threshold is read from apps/ayokoding-www/project.json
    Then the two values match exactly

  # AC-3 [@unit]
  Scenario: The README lists every feature module present under src/features
    Given the directory apps/ayokoding-www/src/features enumerates its subdirectories
    When apps/ayokoding-www/README.md is parsed
    Then every subdirectory appears in the README's feature table

  # AC-4 [@unit]
  Scenario: The prebuild generator commands have one source of truth
    Given apps/ayokoding-www/project.json declares its build dependsOn list
    When apps/ayokoding-www/vercel.json's buildCommand is parsed
    Then the vercel buildCommand does not re-declare commands already in project.json dependsOn

  # AC-5 [@unit]
  Scenario: The validate-indexes target runs under test:quick
    Given the test:quick target's command list is read
    When the list is inspected
    Then it contains validate-indexes
```

### Dependency modernization (Phase 2)

```gherkin
  # AC-6 [@unit]
  Scenario: TypeScript resolves to the TS 6 alias for next build
    Given apps/ayokoding-www/package.json devDependencies carry typescript as npm:@typescript/typescript6
    When the next build's JS API resolves the typescript module
    Then it resolves the typescript6 alias entry

  # AC-7 [@unit]
  Scenario: The typecheck target invokes the Go-native tsc
    Given apps/ayokoding-www/project.json declares a typecheck target
    When the typecheck target's command is parsed
    Then it invokes tsgo or the typescript-7 alias binary
    But it does not invoke the typescript6 alias for the project typecheck

  # AC-8 [@unit]
  Scenario: Every dep bump in the modernization phase carries a Path classification
    Given apps/ayokoding-www/package.json declares its dependency versions
    When each modernized dep is matched against the plan's Path A / Path B / Path C table in tech-docs.md
    Then every modernized dep has a classification entry

  # AC-9 [@unit @e2e]
  Scenario: The app still builds after the dep modernization
    Given the modernized package.json is installed
    When nx build ayokoding-www runs
    Then it exits 0
```

### Pagefind migration (Phase 3)

```gherkin
  # AC-10 [@unit @e2e]
  Scenario: The search route no longer imports flexsearch
    Given apps/ayokoding-www/src is scanned
    When every module under src is parsed
    Then no module imports from flexsearch

  # AC-11 [@unit @e2e]
  Scenario: The search route returns results from the Pagefind static index
    Given the Pagefind index exists at public/pagefind/
    When the search dialog receives a query
    Then it returns matching results from the Pagefind index
    And it does not load a generated/search-data.json artifact

  # AC-12 [@unit]
  Scenario: The generate-search-data target is removed
    Given apps/ayokoding-www/project.json declares its targets
    When the targets are enumerated
    Then generate-search-data is not present

  # AC-13 [@unit]
  Scenario: The flexsearch serverExternalPackages entry is removed
    Given apps/ayokoding-www/next.config.ts declares serverExternalPackages
    When the entry list is inspected
    Then flexsearch is not in the list

  # AC-14 [@unit]
  Scenario: The generated search-data.json file is no longer produced
    Given a clean working tree
    When nx build ayokoding-www runs
    Then the file apps/ayokoding-www/generated/search-data.json is not produced
```

### Mermaid build-time (Phase 4)

```gherkin
  # AC-15 [@unit @e2e]
  Scenario: No client module imports mermaid
    Given apps/ayokoding-www/src is scanned
    When every module under src is parsed
    Then no client module imports from mermaid

  # AC-16 [@unit @e2e]
  Scenario: Mermaid diagrams render as inline SVG without hydration
    Given a content page carries a mermaid diagram code block
    When the page is server-rendered
    Then the rendered HTML contains an inline SVG
    And the rendered HTML does not contain a mermaid client-render script marker

  # AC-17 [@unit]
  Scenario: The rehype pipeline carries the rehype-mermaid plugin with inline-svg strategy
    Given the content rendering rehype pipeline is enumerated
    When the pipeline list is inspected
    Then it contains rehype-mermaid with strategy inline-svg
```

### html-react-parser removal (Phase 5)

```gherkin
  # AC-18 [@unit @e2e]
  Scenario: No client module imports html-react-parser
    Given apps/ayokoding-www/src is scanned
    When every module under src is parsed
    Then no module imports from html-react-parser

  # AC-19 [@unit]
  Scenario: The html-react-parser dependency is removed from package.json
    Given apps/ayokoding-www/package.json declares its dependencies
    When the dependency list is inspected
    Then html-react-parser is not present

  # AC-20 [@unit @e2e]
  Scenario: Content pages still render their HTML body
    Given a content page carries rendered markdown
    When the page is server-rendered
    Then the rendered HTML contains the expected content body
    And no untrusted-HTML parsing runs at request time

  # AC-21 [@unit]
  Scenario: A rehype-React build-time step renders content HTML to React elements
    Given the content rendering rehype pipeline is enumerated
    When the pipeline list is inspected
    Then it contains a rehype-react or equivalent build-time renderer
```

### Calculator lazy-load (Phase 6)

```gherkin
  # AC-22 [@unit @e2e]
  Scenario: The calculator route initial bundle does not ship the cities dataset
    Given the Next build output is read for the calculator route
    When the route's initial chunk list is enumerated
    Then cities.ts is not present in the initial chunk

  # AC-23 [@unit @e2e]
  Scenario: The calculator route loads the cities dataset on demand
    Given the calculator route is rendered
    When the user interacts with a control that needs the cities dataset
    Then a lazy import() loads the cities dataset chunk
    And the calculator's interactive behaviour is preserved

  # AC-24 [@unit]
  Scenario: Cities and roles data live in their own dynamic-import chunks
    Given the Next build output is read
    When the chunk list is enumerated
    Then cities and roles data live in separate chunks from the route's initial chunk
```

### Docker base and trace narrowing (Phase 7)

```gherkin
  # AC-25 [@unit]
  Scenario: The Dockerfile uses node:24-slim as its base
    Given apps/ayokoding-www/Dockerfile is parsed
    When its FROM lines are enumerated
    Then every FROM line names node:24-slim

  # AC-26 [@unit]
  Scenario: The outputFileTracingIncludes patterns are narrow per-route globs
    Given apps/ayokoding-www/next.config.ts declares outputFileTracingIncludes
    When the include patterns are inspected
    Then no include pattern uses the "/**" route glob
    And every pattern is scoped to a specific route prefix

  # AC-27 [@unit]
  Scenario: The narrowed trace still covers every fs.readFile content path
    Given apps/ayokoding-www/src is scanned for fs.readFile call sites
    When every call site's argument is resolved
    Then every resolved path is matched by some pattern in the narrowed outputFileTracingIncludes

  # AC-28 [@unit]
  Scenario: Generated search-data is no longer in the trace include set
    Given apps/ayokoding-www/next.config.ts declares outputFileTracingIncludes
    When the include patterns are inspected
    Then no pattern includes generated/search-data.json
```

### Bilingual completeness (cross-phase)

```gherkin
  # AC-29 [@unit @e2e]
  Scenario Outline: No raw translation key leaks on either locale after any phase
    Given the locale is "<locale>"
    When the AI benchmark page renders after the cost-reduction work
    Then no rendered text matches a raw translation key

    Examples:
      | locale |
      | en     |
      | id     |
```

### Rule-15 retest gate (Phase 8, recorded but not authored now)

> Per the plan-planning workflow's
> [Three UI Gates Are Complementary](../../../repo-governance/workflows/plan/plan-planning.md#the-three-ui-gates-are-complementary-never-substitutes)
> rule, the live-site EWT/UWT/DWT triad is invoked in Phase 8 against the running target across
> both locales and three breakpoints. Findings and SG-### spec gaps are appended under
> **Rule-15 retest follow-ups** in `delivery.md` Phase 8.

## Product risks

| Risk                                                                                                            | Likelihood | Impact | Mitigation                                                                                                                                                                                 |
| --------------------------------------------------------------------------------------------------------------- | ---------- | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| TS 7 side-by-side breaks `next build` on the existing 253 production source                                     | High       | High   | Phase 2 binds the side-by-side path to the documented recipe; the JS-API alias is the fallback; CI greentime stays green before the PR merges                                              |
| Pagefind's UI semantics differ from the FlexSearch dialog enough to regress search UX                           | Medium     | High   | Phase 3 ships the new search behind the existing dialog wrapper; the Phase 8 Rule-15 retest verifies before archival                                                                       |
| `rehype-mermaid` Playwright warmup adds cache-miss build-minute cost greater than the client bundle win         | Low        | Medium | The 5 s warm-up is shared across all diagrams in a build (one browser per build); a Phase 4 evidence file records the marginal build-time delta                                            |
| `html-react-parser` removal regresses content that previously rendered via runtime parsing                      | Low        | High   | Phase 5 ships a snapshot assertion comparing the rendered HTML AST before and after the swap; any divergence is a test failure                                                             |
| Calculator lazy-load regression in the route's UX (flash of unstyled/empty state before data loads)             | Low        | Medium | The calculator route already has a Suspense boundary; the data chunks load synchronously on the first interaction                                                                          |
| Docker `node:24-slim` regression on `npm ci` workspaces (any musl-vs-glibc surprise from a transitive dep)      | Low        | High   | Slim is glibc; the workspace's transitive deps are already glibc-validated via Vercel's Node 24 runtime; a Phase 7 e2e verified                                                            |
| `outputFileTracingIncludes` narrowing drops a file needed at runtime → 500 on a content route                   | Medium     | High   | Narrowing is derived from static analysis of `fs.readFile` call sites; a Phase 7 unit assertion confirms coverage of every call site path                                                  |
| Concurrent execution of this plan with the `ayokoding-www-tools-ai-benchmark` backlog plan touches shared files | Medium     | Medium | One-worktree-per-unit HARD RULE + Step 0 sync against `origin/main`; shared file surfaces are limited to `libs/web-ui-token/src/ayokoding.css` only                                        |
| TS 7 strict-mode type errors begin surfacing pre-existing latent issues in maintenance grains                   | Medium     | Low    | Phase 2 explicitly keeps the existing `tsconfig.json` strictness; any new error is a pre-existing issue and is fixed as a preexisting-error-of-this-phase per the Local Quality Gates rule |
| The `@trpc/*` exact-pin lands a regression the caret pin was tolerating silently                                | Low        | Medium | Every bump carries a Path classification; the bumps stay within minor and on the LTS line                                                                                                  |

## Cross-references

- Business reasoning and risk ownership: [`brd.md`](./brd.md).
- The scoring method, the honesty surface, and the cited snapshot: [`tech-docs.md`](./tech-docs.md).
- Phase-by-phase scenario ownership: [`delivery.md`](./delivery.md).
