# PRD — Standardize App Spec Trees

## Product Overview

Consolidate the two OSE spec trees into a single `specs/apps/ose/` tree and codify the resulting
"multi-deployable family" shape as the repo-wide standard. The product surface here is the
**developer experience of the `specs/` directory and its governing convention** — not an
end-user feature.

## Personas

- **Contributor (human)** — edits or adds Gherkin/specs and needs to know exactly where OSE
  specs live.
- **AI agent (`specs-maker` / `specs-checker` / `swe-*-dev`)** — resolves spec paths
  mechanically from the family name and validates layout conformance.
- **Maintainer** — relies on uniform tooling (spec-coverage, ose-primer sync) free of OSE
  special cases.

## User Stories

1. As a contributor, I want all `apps/ose-*` projects to reference one `specs/apps/ose/` tree so
   I never have to guess between `ose-app` and `ose-platform`.
2. As a spec author, I want the convention to show an explicit layout for a family that ships
   multiple deployables (app + marketing site + CLI) so I can place specs without inventing a
   structure.
3. As a spec validator, I want `specs-checker` to fail when an app family splits across multiple
   spec trees or uses a non-standard perspective name (`api` instead of `be`).
4. As a maintainer, I want every non-OSE family confirmed conformant so the standard reflects
   reality across the whole repo.

## Acceptance Criteria (Gherkin)

```gherkin
Feature: Single consolidated OSE spec tree

  Scenario: All OSE apps reference one spec tree
    Given the migration is complete
    When I grep the repository for "specs/apps/ose-app" or "specs/apps/ose-platform"
    Then zero references remain outside the plans/done archive
    And every apps/ose-* project resolves its specs under "specs/apps/ose/"

  Scenario: Surface-disambiguated behavior subtrees
    Given the consolidated tree exists
    When I list "specs/apps/ose/behavior/"
    Then it contains "app-be", "app-web", "platform-be", "platform-web", and "cli"
    And no two surfaces collide on the same directory name

  Scenario: Backend perspective uses the standard name
    Given the platform specs previously used an "api" perspective
    When the migration completes
    Then the platform backend behavior lives under "platform-be"
    And no "behavior/.../api/gherkin" path remains for OSE

  Scenario: CLI specs are normalized to one canonical location
    Given ose-cli specs previously had a legacy split layout
    When the migration completes
    Then all ose-cli Gherkin lives under "specs/apps/ose/behavior/cli/gherkin/"
    And ose-cli tests resolve features from that single location
```

```gherkin
Feature: Contracts project relocation

  Scenario: Contracts Nx project moves and is renamed
    Given the contracts project was "ose-app-contracts" rooted at the ose-app tree
    When the migration completes
    Then the Nx project is named "ose-contracts" rooted at "specs/apps/ose/containers/contracts"
    And "nx run ose-contracts:lint" succeeds
    And ose-app-be and ose-app-web codegen read the bundled spec from the new path
```

```gherkin
Feature: Quality gates stay green across phases

  Scenario: Each phase ends green
    Given a phase has moved specs and rewritten references
    When I run "nx affected -t spec-coverage test:quick" plus the affected e2e suites
    Then all targets pass before the next phase begins
```

```gherkin
Feature: Multi-deployable family standard

  Scenario: Convention codifies the multi-deployable layout
    Given the specs-directory-structure convention previously assumed one deployable per family
    When the amendment lands
    Then it documents the surface-prefixed behavior layout for multi-deployable families
    And it names "be" as the standard backend-HTTP perspective
    And it cites specs/apps/ose as the worked example

  Scenario: Checker enforces the standard
    Given the amended convention is active
    When specs-checker runs against specs/apps
    Then it flags any family split across multiple spec trees
    And it flags any "api"-named backend perspective as non-standard

  Scenario: All non-OSE families already conform
    Given the conformance audit runs
    When each apps/ family is checked against the standard
    Then organiclever, ayokoding, wahidyankf, crane, and rhino are confirmed conformant
    And ose is confirmed conformant after migration
```

## Product Scope

**In scope:** spec file relocation, reference rewrites, contracts project rename, perspective
rename, cli normalization, convention amendment, checker enforcement, conformance audit, docs
cross-ref sweep.

**Out of scope:** app renames, new specs/features, non-OSE structural changes, deployment changes.

## Product Risks

| Risk                                                         | Mitigation                                                               |
| ------------------------------------------------------------ | ------------------------------------------------------------------------ |
| Surface prefixes confuse readers (`app-be` vs `platform-be`) | Document the naming scheme in the convention with the OSE worked example |
| `specs-checker` amendment yields false positives on libs     | Scope the multi-deployable rule to `specs/apps/` families only           |
| Unified C4/DDD merge loses product-specific framing nuance   | Preserve both products' content as labelled sections within unified docs |
