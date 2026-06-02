# Product Requirements — Remove DDD and Hexagonal from wahidyankf-web

## Product Overview

This change is an internal refactor + governance/docs update for
`wahidyankf-web`. It removes the DDD bounded-context accretion and flattens the
hexagonal `src/contexts/` layout to a flat `src/features/` layout, with no
change to the deployed site's behavior or appearance.

## Personas

This is a solo-maintainer repo; "personas" are the hats the maintainer wears and
the agents that consume the files.

- **Maintainer-as-architect** — owns the decision to flatten and the governance
  opt-out clause.
- **Maintainer-as-developer** — executes the refactor.
- **AI consuming agents** — `swe-typescript-dev`, `swe-rust-dev`,
  `readme-maker`, governance-aware agents (all existing). [Repo-grounded]

## User Stories

- **As the maintainer-as-architect**, I want a documented opt-out from the
  hexagonal-web convention for trivially-small static sites, so that flattening
  `wahidyankf-web` is a sanctioned deviation rather than silent drift.
- **As the maintainer-as-developer**, I want the four-layer `contexts/` structure
  collapsed to a flat `features/` layout, so that I navigate fewer empty folders
  and less barrel indirection when editing portfolio pages.
- **As the maintainer-as-developer**, I want the DDD pre-push gates and spec tree
  removed, so that `test:quick` stops running `ddd` validation an empty domain
  cannot benefit from.
- **As an AI consuming agent**, I want READMEs and governance docs to accurately
  describe the flat layout, so that I do not reintroduce hexagonal/DDD scaffolding.

## Acceptance Criteria (Gherkin)

### Scenario: DDD spec tree removed

```gherkin
Given the plan has been executed
When I check the filesystem for the DDD spec directory
Then "specs/apps/wahidyankf/ddd/" should not exist
And "specs/apps/wahidyankf/behavior/" should still exist unchanged
```

### Scenario: project.json no longer runs DDD gates

```gherkin
Given the plan has been executed
When I inspect "apps/wahidyankf-web/project.json"
Then the "test:quick" commands should not contain "ddd bc wahidyankf"
And the "test:quick" commands should not contain "ddd ul wahidyankf"
And the "test:quick" inputs should not reference "ddd/bounded-contexts.yaml"
And the "test:quick" inputs should not reference "ddd/ubiquitous-language"
And the vitest run, coverage-80 validation, and spec-coverage target remain intact
And "dependsOn" still lists "rhino-cli:build"
```

### Scenario: rhino-cli allowlist drops wahidyankf

```gherkin
Given the plan has been executed
When I inspect "apps/rhino-cli/src/internal/allowlist.rs"
Then "apps_with_ddd()" should not contain "wahidyankf"
And the membership test "assert_eq!(v.len(), N)" should be decremented by one
And no "assert!(v.contains(&\"wahidyankf\"))" assertion should remain
And the top rustdoc comment should not list a wahidyankf DDD entry
And "cargo test" for rhino-cli should pass
```

### Scenario: contexts flattened to features

```gherkin
Given the plan has been executed
When I list "apps/wahidyankf-web/src/"
Then a "features/" directory should exist with one folder per context
And the five feature folders app-shell, home, cv, personal-projects, search should exist
And "apps/wahidyankf-web/src/contexts/" should not exist
And no source or test file should import from "@/contexts/"
```

### Scenario: imports resolve via the @/ alias

```gherkin
Given the contexts have been flattened to features
When I run "nx run wahidyankf-web:typecheck"
Then the command should exit 0
And all "@/features/<ctx>/X" imports should resolve via the "@/* -> ./src/*" path alias
```

### Scenario: behavior preserved across the refactor

```gherkin
Given the contexts have been flattened to features
When I run "nx run wahidyankf-web:test:quick"
Then all unit tests should pass
And coverage should remain at or above the 80% threshold
```

### Scenario: spec-coverage gate still resolves

```gherkin
Given the contexts have been flattened to features
When I run "nx run wahidyankf-web:spec-coverage"
Then the command should exit 0
And it should scan "apps/wahidyankf-web" and the behavior gherkin specs unchanged
```

### Scenario: governance opt-out clause present

```gherkin
Given the plan has been executed
When I read "repo-governance/development/pattern/hexagonal-architecture-web.md"
Then it should contain an exemptions/opt-out subsection
And that subsection should permit a flat "src/features/<name>/" layout for
  trivially-small static content sites with no IO ports and no business rules
And the text should be vendor-neutral
```

### Scenario: READMEs accurate

```gherkin
Given the plan has been executed
When I read "apps/wahidyankf-web/README.md"
Then it should describe the flat "src/features/" layout
And it should not contain "DDD", "bounded context", "hexagonal", "ddd bc", or "ddd ul"
And it should not reference "specs/apps/wahidyankf/ddd"
```

### Scenario: full quality gate green

```gherkin
Given the plan has been executed
When I run "nx affected -t typecheck lint test:quick spec-coverage"
Then it should pass for "wahidyankf-web" and "rhino-cli" with zero failures
And "nx build wahidyankf-web" should pass
```

### Scenario: visual smoke clean

```gherkin
Given a running dev server for wahidyankf-web
When I navigate to "/", "/cv", and "/personal-projects" and use the search box
Then each page should render correctly
And the browser console should report zero errors
```

## Product Scope

**In scope (product-visible to maintainers/agents)**:

- Flat `src/features/<ctx>/` layout replacing `src/contexts/<ctx>/<layer>/`.
- Updated `project.json`, rhino-cli allowlist, governance doc, and README.
- Removed DDD spec tree.

**Out of scope**:

- Any change to deployed site content, routes, styling, or UX.
- C4 specs and Gherkin behavior specs.
- `rhino-cli ddd` subcommand implementations.
- Sibling apps' architecture.

## Product Risks

| Risk                                                       | Mitigation                                                                 |
| ---------------------------------------------------------- | -------------------------------------------------------------------------- |
| A missed import leaves a dangling `@/contexts` reference   | Final grep-clean gate for `@/contexts`; per-phase typecheck catches breaks |
| Coverage drops below 80% after file moves                  | `test:quick` runs the coverage-80 validation each phase gate               |
| README or governance doc reintroduces hexagonal vocabulary | Phase 7 grep check for forbidden terms in the README                       |

See [tech-docs.md](./tech-docs.md) for the technical design and file-impact map.
