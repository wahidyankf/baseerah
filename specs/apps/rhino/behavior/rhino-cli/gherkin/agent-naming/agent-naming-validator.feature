@agent-naming-validator
Feature: The agent-naming validator fires on invalid agent files

  As a maintainer relying on the naming gate to catch bad agent-file names
  I want the naming validator to fire on an invalid agent-file rename
  So that no invalid name slips through, and no trigger path uses the buggy singular .opencode/agent/ form

  Scenario: The agent-naming validator fires
    Given an agent file renamed to an invalid suffix
    When the naming validator runs (triggered on .opencode/agents/ changes)
    Then it detects the invalid name and fails
    And no trigger path references the singular .opencode/agent/
