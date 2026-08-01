# Product Requirements: Cross-Repo Port Registry

## Persona

An engineer or AI agent scaffolding a new app in any of the four sibling repos under
`/Users/wkf/ose-projects/` — needs to know immediately, not at first concurrent run, whether a
chosen port collides with an app in a sibling repo.

## User Story

As an engineer or AI agent allocating a port for a new app, I want an automated check against a
shared, machine-readable port registry, so that I learn immediately whether my chosen port
collides with an app in any sibling repo.

## Product Scope

**Candidate scope, pending the Phase 1 scope confirmation** (see `tech-docs.md` and
`delivery.md`'s Phase 1): a registry covering every app across `ose-public`, `ose-primer`,
`ose-private`, and `beaver-nest`, plus a validator runnable from any one repo.

## Acceptance Criteria (Gherkin)

```gherkin
Feature: Cross-repo port registry

  Scenario: A new app's port collides with an existing sibling-repo allocation
    Given a shared port registry listing every app's allocated port across all four repos
    And a new app declares a port already claimed by an app in a sibling repo
    When the registry validator runs
    Then it fails with the colliding port and both apps' names

  Scenario: A new app's port is unclaimed
    Given a shared port registry listing every app's allocated port across all four repos
    And a new app declares a port not claimed by any other app
    When the registry validator runs
    Then it passes with no findings
```

## Non-Goals

- Does not re-litigate `beaver-nest-be`'s (19320) or `beaver-nest-fe`'s (19310) already-allocated ports.
- Does not change any app's runtime port configuration — only adds a check.
