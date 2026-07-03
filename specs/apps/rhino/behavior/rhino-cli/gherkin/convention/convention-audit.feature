@convention-audit
Feature: `convention audit` aggregates every convention validator into one pass/fail report

  As a maintainer running the full repository-convention gate in one command
  I want `convention audit` to run every convention validator in sequence and
  report a single result
  So that a missing AGENTS.md is caught even when the emoji and license
  validators pass

  Scenario: A missing AGENTS.md fails the aggregate convention audit
    Given a repository with no AGENTS.md file
    When the developer runs "rhino-cli convention audit"
    Then the command exits with a failure code
    And the output names the failing "agents-md-size" validator
