# Product Requirements Document — ayokoding-web Remove DDD

## Product overview

Remove the DDD scaffolding from `ayokoding-web` while preserving its hexagonal feature-module
structure and all runtime behavior. The "product" here is the developer-facing codebase and its
documentation/tooling surface — there is no end-user-visible change.

## Personas

- **Maintainer-as-architect** — wants the app's structure and docs to match the repo's
  hexagonal-web governance, with no DDD contradiction.
- **Maintainer-as-web-developer** — wants `test:quick` to validate the app's real behavior
  without unrelated DDD checks, and a README that describes the actual three-layer structure.
- **`rhino-cli` consumer apps** (`organiclever-be`, `organiclever-web`, `ose-app-be`) — need the
  `ddd` subcommands and `apps_with_ddd()` to keep functioning after the `ayokoding` entry is
  removed.
- **Planning/content agents** — read `apps/ayokoding-web/README.md` and expect consistent,
  non-DDD architecture language.

## User stories

- **US-1**: As the maintainer-architect, I want the DDD spec subtree for ayokoding deleted, so
  that the repo no longer maintains a registry for an app where DDD does not apply.
- **US-2**: As the maintainer-web-developer, I want the two `rhino-cli ddd` pre-push commands and
  their `inputs` globs removed from `ayokoding-web`'s `test:quick`, so that pre-push validates
  only the app's real behavior.
- **US-3**: As a `rhino-cli` consumer, I want `ayokoding` removed from `apps_with_ddd()` and the
  membership test updated, so that DDD validation no longer targets ayokoding while other apps
  keep working.
- **US-4**: As the maintainer-web-developer, I want the six empty `domain/` folders and barrels
  deleted, so that the source tree reflects only the layers that actually contain code.
- **US-5**: As a planning/content agent, I want the README to describe hexagonal feature modules
  (three layers) instead of DDD bounded contexts, so that docs match governance.

## Acceptance criteria (Gherkin)

```gherkin
Feature: Remove DDD scaffolding from ayokoding-web while keeping hexagonal structure

  Scenario: DDD spec subtree is deleted (US-1)
    Given the directory "specs/apps/ayokoding/ddd/" exists with its 10 files
    When the plan deletes the DDD spec subtree
    Then "specs/apps/ayokoding/ddd/" does not exist
    And a repo-wide search for "specs/apps/ayokoding/ddd" outside "plans/" returns no matches

  Scenario: test:quick no longer runs DDD validation for ayokoding (US-2)
    Given "apps/ayokoding-web/project.json" test:quick command array contains
          "ddd bc ayokoding" and "ddd ul ayokoding"
    And its inputs list contains the two "specs/apps/ayokoding/ddd/..." globs
    When the plan removes those two commands and those two input globs
    Then the test:quick command array contains no "ddd bc" or "ddd ul" invocation
    And the inputs list references no "specs/apps/ayokoding/ddd" path
    And "nx run ayokoding-web:test:quick" passes

  Scenario: ayokoding is removed from the rhino-cli DDD allowlist (US-3)
    Given "apps_with_ddd()" in allowlist.rs currently includes "ayokoding"
    And the membership test asserts the slice length and that it contains "ayokoding"
    When the plan removes the "ayokoding" entry, decrements the asserted length by one,
         and removes the "ayokoding" assertion and doc line
    Then "apps_with_ddd()" no longer contains "ayokoding"
    And the membership test still asserts presence of the remaining required apps
    And "cargo test --manifest-path apps/rhino-cli/Cargo.toml --lib" passes
    And the edit remains correct regardless of sibling-plan execution order

  Scenario: empty domain layers are deleted (US-4)
    Given each "src/contexts/<ctx>/domain/" folder is empty except for a blank index.ts barrel
    And no file outside those barrels imports from a contexts "domain/" path
    When the plan deletes all six "domain/" folders and their barrels
    Then no "src/contexts/*/domain/" folder exists in ayokoding-web
    And "nx run ayokoding-web:typecheck" exits 0

  Scenario: README describes hexagonal feature modules, not DDD (US-5)
    Given "apps/ayokoding-web/README.md" uses "bounded context" and "DDD registry" language
    And it references "specs/apps/ayokoding/ddd/..." and "rhino-cli ddd bc/ul"
    When the plan rewrites the architecture-related sections
    Then the README describes the structure as hexagonal feature modules with three layers
         (application, infrastructure, presentation)
    And the README links the hexagonal-architecture-web governance doc
    And the README contains no "DDD registry" or DDD-enforcement language
    And the README contains no "specs/apps/ayokoding/ddd" reference

  Scenario: full quality gate stays green (definition of done)
    Given all five change groups are applied
    When "nx affected -t typecheck lint test:quick spec-coverage" runs for ayokoding-web and rhino-cli
    Then every target passes with zero failures
    And "nx build ayokoding-web" succeeds

  Scenario: dev server renders with no console errors (manual smoke)
    Given the ayokoding-web dev server is running on port 3101
    When a reviewer opens the home page and one content page via Playwright MCP
    Then both pages render their expected content
    And the browser console reports zero errors
```

## Product scope

### In scope

- Deletion of the DDD spec subtree, allowlist entry, pre-push commands + input globs, empty
  `domain/` layers, and DDD README language (the five change groups in
  [`README.md`](./README.md#scope)).

### Out of scope

- Any change to `contexts/` runtime code, imports, or dependency directions.
- C4 spec trees, Gherkin `behavior/**`, and the `spec-coverage` gate.
- The `rhino-cli ddd` subcommands (retained for other apps).
- DDD removal from other apps.

## Product risks

| Risk                                                       | Mitigation                                                                                                              |
| ---------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------- |
| README rewrite drifts from the governing hexagonal-web doc | Cite and link `hexagonal-architecture-web.md`; describe only the three layers that exist.                               |
| Removing `domain/` implies the governance layout is wrong  | README notes ayokoding currently ships no `domain/` code; governance layout still lists `domain/` as an optional layer. |
| Stale Nx cache masks the `inputs` change                   | Phase gate re-runs `test:quick` so Nx recomputes the cache from new inputs.                                             |
