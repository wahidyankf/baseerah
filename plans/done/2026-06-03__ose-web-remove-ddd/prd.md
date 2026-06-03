# PRD — ose-web-remove-ddd

## Product Overview

Remove the DDD scaffolding from `apps/ose-web/` (a Next.js 16 content/marketing site) while
keeping its hexagonal feature-module layout. The deliverable is a tidy: deleted DDD spec registry,
deleted empty `domain/` layers, de-DDD'd `project.json` pre-push commands, an updated `rhino-cli`
allowlist, and a rewritten README — with no change to runtime behavior.

## Personas

- **Maintainer-as-architect**: wants the app's architecture to match governance and to read the
  README without encountering DDD terms that do not apply to a web app.
- **Plan-execution agent**: reads `apps/ose-web/README.md` and runs `ose-web:test:quick`; needs
  both to be accurate and free of dead DDD validators.
- **`rhino-cli` maintainer**: needs `apps_with_ddd()` to list only genuine DDD apps so DDD
  validation runs only where it is meaningful.

## User Stories

- **US-1** — As the maintainer-as-architect, I want the DDD spec registry deleted so that the
  `ose-platform` spec tree contains only C4 and behavior specs, matching a web app's reality.
- **US-2** — As a plan-execution agent, I want `ose-web:test:quick` to stop running the two
  `rhino-cli ddd` validators so that the pre-push gate reflects the app's actual architecture.
- **US-3** — As the `rhino-cli` maintainer, I want `ose-platform` removed from `apps_with_ddd()`
  and the `membership` test kept green so that the allowlist lists only true DDD apps.
- **US-4** — As a developer reading the code, I want the seven empty `domain/` folders removed so
  that the layout shows only the layers that actually contain code.
- **US-5** — As any reader, I want `apps/ose-web/README.md` to describe hexagonal feature modules
  (not DDD bounded contexts) so that documentation matches governance and code.
- **US-6** — As the maintainer, I want the app's runtime output, routes, and tests unchanged so
  that this tidy is provably behavior-preserving.

## Acceptance Criteria (Gherkin)

### AC-1: DDD spec registry deleted (US-1)

```gherkin
Scenario: The ose-platform DDD spec directory no longer exists
  Given the repository at the plan worktree
  When I run "test -d specs/apps/ose-platform/ddd"
  Then the command exits non-zero
  And "git status" shows the 11 ddd files as deleted
  And "specs/apps/ose-platform/system-context" still exists
  And "specs/apps/ose-platform/behavior" still exists
```

### AC-2: ose-web pre-push gate drops the DDD validators (US-2)

```gherkin
Scenario: project.json no longer invokes the rhino-cli ddd validators
  Given "apps/ose-web/project.json"
  When I grep the file for "ddd bc ose-platform" and "ddd ul ose-platform"
  Then no matches are found
  And the test:quick "inputs" array contains no "specs/apps/ose-platform/ddd/" glob
  And the test:quick "inputs" array still contains the behavior/web and behavior/api gherkin globs
  And the vitest coverage command with threshold 86 is unchanged
```

### AC-3: rhino-cli allowlist drops ose-platform and stays green (US-3)

```gherkin
Scenario: apps_with_ddd no longer contains ose-platform and the membership test passes
  Given "apps/rhino-cli/src/internal/allowlist.rs"
  When I inspect the apps_with_ddd() slice
  Then it does not contain "ose-platform"
  And it still contains "organiclever", "ayokoding", and "ose-app"
  And the module "//!" doc block no longer lists an ose-platform bullet
  When I run "cargo test" for rhino-cli
  Then the membership test passes with the decremented expected length
```

### AC-4: Empty domain layers removed (US-4)

```gherkin
Scenario: No context retains an empty domain layer
  Given "apps/ose-web/src/contexts/"
  When I run "find apps/ose-web/src/contexts -type d -name domain"
  Then zero directories are returned
  And no "domain/index.ts" file remains under any context
  And "nx run ose-web:typecheck" exits 0
```

### AC-5: README describes hexagonal feature modules (US-5)

```gherkin
Scenario: README no longer uses DDD framing
  Given "apps/ose-web/README.md"
  When I grep for "DDD", "bounded context", "Per-BC", and "ddd/bounded-contexts.yaml"
  Then no matches are found
  And the Architecture section describes hexagonal feature modules with three layers
        application, infrastructure, presentation
  And the README links to repo-governance hexagonal-architecture-web.md
```

### AC-6: Behavior preserved (US-6)

```gherkin
Scenario: ose-web builds, tests, and renders unchanged
  Given all five change groups are applied
  When I run "nx affected -t typecheck lint test:quick spec-coverage" for ose-web and rhino-cli
  Then every target passes
  And "nx build ose-web" exits 0
  When I start the dev server and visit "/", "/updates", and "/about" via Playwright MCP
  Then each page renders
  And the browser console reports zero errors
```

## Product Scope

### In scope

- Delete `specs/apps/ose-platform/ddd/` (11 files).
- Edit `apps/ose-web/project.json` (remove 2 commands + 2 inputs globs).
- Edit `apps/rhino-cli/src/internal/allowlist.rs` (entry + relative test edit + doc block).
- Delete 7 empty `src/contexts/*/domain/` folders + barrels.
- Rewrite Architecture/Specs/Bounded-Contexts sections of `apps/ose-web/README.md`.

### Out of scope

- Any change to `application/`/`infrastructure/`/`presentation/` source or imports.
- C4 specs (`product`, `system-context`, `containers`, `components`).
- Gherkin behavior specs and the `spec-coverage` gate.
- The `rhino-cli ddd` subcommands (kept for other apps).

## Product Risks

| Risk                                                  | Mitigation                                                                                  |
| ----------------------------------------------------- | ------------------------------------------------------------------------------------------- |
| README rewrite drifts from governance terminology     | Mirror `hexagonal-architecture-web.md` layer names and link to it directly.                 |
| `test:quick` cache key change from editing `inputs`   | Remove only the two `ddd/...` globs; verify all other globs byte-for-byte unchanged.        |
| A future merge re-introduces the `ose-platform` entry | Relative test edit + Phase gates catch any re-introduction via a failing `membership` test. |
