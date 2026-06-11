# PRD — Standardize App Spec Trees

## Product Overview

Adopt one **flat product-surface** naming scheme for behavior dirs across every ose-public app
family, consolidate the two OSE spec trees into a single `specs/apps/ose/` tree, rename every `api`
surface to `be`, and codify the scheme as the repo-wide standard. The product surface here is the
**developer experience of the `specs/` directory and its governing convention** — not an end-user
feature.

## Personas

- **Contributor (human)** — edits or adds Gherkin/specs and needs to know exactly where each
  family's specs live and what the behavior dirs are named.
- **AI agent (`specs-maker` / `specs-checker` / `swe-*-dev`)** — resolves spec paths mechanically
  from the family + surface and validates layout conformance.
- **Maintainer** — relies on uniform tooling (spec-coverage, ose-primer sync) free of OSE special
  cases and naming drift.

## User Stories

1. As a contributor, I want all `apps/ose-*` projects to reference one `specs/apps/ose/` tree so I
   never have to guess between `ose-app` and `ose-platform`.
2. As a contributor, I want every family's behavior dirs named by the same flat product-surface
   rule so I can find any surface's Gherkin without memorising per-family exceptions.
3. As a spec author, I want the convention to show the flat product-surface layout for both a
   multi-product family (OSE) and a single-product family (organiclever) so I can place specs
   without inventing a structure.
4. As a spec validator, I want `specs-checker` to fail when a family splits across multiple trees,
   uses a bare-surface behavior dir, or uses the `api` perspective instead of `be`.
5. As a maintainer, I want every ose-public family migrated and confirmed conformant so the standard
   reflects reality across the whole repo.

## Acceptance Criteria (Gherkin)

```gherkin
Feature: Single consolidated OSE spec tree

  Scenario: All OSE apps reference one spec tree
    Given the OSE migration is complete
    When I grep the repository for "specs/apps/ose-app" or "specs/apps/ose-platform"
    Then zero references remain outside the plans archive
    And every apps/ose-* project resolves its specs under "specs/apps/ose/"

  Scenario: OSE surfaces use flat product-surface behavior dirs
    Given the consolidated OSE tree exists
    When I list "specs/apps/ose/behavior/"
    Then it contains "app-be", "app-web", "platform-be", "platform-web", and "cli"
    And no two surfaces collide on the same directory name

  Scenario: OSE backend perspective uses the standard name
    Given the platform specs previously used an "api" perspective
    When the OSE migration completes
    Then the platform backend behavior lives under "platform-be"
    And no "behavior/.../api/gherkin" path remains for OSE

  Scenario: OSE CLI specs are normalized to one canonical location
    Given ose-cli specs and a stale README path previously diverged
    When the OSE migration completes
    Then all ose-cli Gherkin lives under "specs/apps/ose/behavior/cli/gherkin/"
    And the ose-cli README cites only that single location
```

```gherkin
Feature: Contracts project relocation

  Scenario: Contracts Nx project moves and is renamed
    Given the contracts project was "ose-app-contracts" rooted at the ose-app tree
    When the OSE migration completes
    Then the Nx project is named "ose-contracts" rooted at "specs/apps/ose/containers/contracts"
    And "nx run ose-contracts:lint" succeeds
    And ose-app-be and ose-app-web codegen read the bundled spec from the new path
```

```gherkin
Feature: Single-product families adopt flat product-surface dirs

  Scenario Outline: A single-product family renames its behavior dirs
    Given family "<family>" previously used bare-surface behavior dirs
    When its migration completes
    Then its behavior dirs are named "<expected-dirs>"
    And "nx run <coverage-project>:spec-coverage" exits zero

    Examples:
      | family       | expected-dirs                                                            | coverage-project  |
      | organiclever | organiclever-be, organiclever-web                                        | organiclever-be   |
      | ayokoding    | ayokoding-be, ayokoding-web, ayokoding-cli, ayokoding-build-tools        | ayokoding-web     |
      | crane        | crane-cli                                                                | crane-cli         |
      | rhino        | rhino-cli                                                                | rhino-cli         |
      | wahidyankf   | wahidyankf-web                                                           | wahidyankf-web    |
```

```gherkin
Feature: ayokoding backend perspective is renamed

  Scenario: ayokoding api surface becomes ayokoding-be
    Given ayokoding previously used a "behavior/api/gherkin" surface
    When the ayokoding migration completes
    Then its backend behavior lives under "behavior/ayokoding-be/gherkin"
    And no "specs/apps/ayokoding/behavior/api" path remains
```

```gherkin
Feature: rhino source defaults follow the renamed surface

  Scenario: rhino-cli source default paths point at the new behavior dir
    Given rhino-cli Rust source hardcodes "specs/apps/rhino/behavior/cli/gherkin" defaults
    When the rhino migration completes
    Then those defaults reference "specs/apps/rhino/behavior/rhino-cli/gherkin"
    And "nx run rhino-cli:test:quick" passes
```

```gherkin
Feature: Quality gates stay green across phases

  Scenario: Each phase ends green
    Given a phase has moved specs and rewritten references
    When I run "nx affected -t spec-coverage test:quick" plus the affected e2e suites
    Then all targets pass before the next phase begins
```

```gherkin
Feature: Flat product-surface standard codified and enforced

  Scenario: Convention codifies the flat product-surface layout
    Given the convention previously named behavior dirs by bare perspective
    When the amendment lands
    Then it documents the flat product-surface behavior layout for multi-product and single-product families
    And it names "be" as the standard backend-HTTP perspective deprecating "api"
    And it shows worked examples for "specs/apps/ose" and "specs/apps/organiclever"

  Scenario: Checker enforces the standard
    Given the amended convention is active
    When specs-checker runs against specs/apps
    Then it flags any family split across multiple spec trees
    And it flags any bare-surface or "api"-named behavior dir as non-standard

  Scenario: All ose-public families conform after migration
    Given the conformance audit runs
    When each apps/ family is checked against the standard
    Then ose, organiclever, ayokoding, crane, rhino, and wahidyankf are confirmed conformant
```

## Product Scope

**In scope:** spec relocation + behavior-dir renames for every ose-public family, reference
rewrites (project.json, playwright config, step files, READMEs, governance/docs cross-refs,
regenerated playwright-bdd artifacts), contracts project rename, `api`→`be` perspective renames,
rhino source-default updates, convention amendment, `specs-checker`/`specs-maker` updates + bindings
re-sync, rationale doc, conformance audit.

**Out of scope:** `apps/*` project renames (except the `ose-contracts` spec project), new
specs/features, sibling-repo restructuring, deployment changes.

## Product Risks

| Risk                                                           | Mitigation                                                                     |
| -------------------------------------------------------------- | ------------------------------------------------------------------------------ |
| Product-surface prefixes confuse readers (`app-be` vs `be`)    | Document the scheme in the convention with both OSE and organiclever examples  |
| `specs-checker` amendment yields false positives on libs       | Scope the flat product-surface rule to `specs/apps/` families only             |
| rhino Rust source-default change breaks unit tests             | TDD-shaped (RED first) so the test names the expected new path before the swap |
| Unified OSE C4/DDD merge loses product-specific framing nuance | Preserve both products' content as labelled sections within unified docs       |
