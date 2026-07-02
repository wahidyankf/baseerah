@repo-governance-instruction-size
Feature: Instruction-file size budget

  As a repository maintainer
  I want all auto-loaded instruction surfaces to stay within configured byte thresholds
  So that coding-agent harnesses load instruction files completely without silent truncation

  Background:
    Given a committed "instruction-size-budget.yaml" mapping instruction-file globs to target, warn, and fail byte thresholds

  Scenario: A file within target passes silently
    Given "AGENTS.md" is 24000 bytes
    And its target is 24000 and its fail ceiling is 30000
    When the developer runs harness instruction-size validate
    Then the command exits successfully
    And the file is reported with severity "ok"

  Scenario: A file over target but under the ceiling warns without failing
    Given "AGENTS.md" is 28000 bytes
    And its target is 24000 and its fail ceiling is 30000
    When the developer runs harness instruction-size validate
    Then the command exits successfully
    And the file is reported with severity "warn"

  Scenario: A file over its hard ceiling fails the command
    Given "AGENTS.md" is 41108 bytes
    And its fail ceiling is 30000
    When the developer runs harness instruction-size validate
    Then the command exits with a failure code
    And the file is reported with severity "fail"

  Scenario: A configured glob matching no file is a no-op
    Given no file exists at ".github/copilot-instructions.md"
    When the developer runs harness instruction-size validate
    Then no finding is emitted for ".github/copilot-instructions.md"

  Scenario: The resolved tree is checked against the fail ceiling
    Given "CLAUDE.md" imports "AGENTS.md" via "@AGENTS.md"
    And the sum of "CLAUDE.md" plus the imported files exceeds the 38000-byte tree ceiling
    When the developer runs harness instruction-size validate
    Then a finding with key "resolved-tree" is reported with severity "fail"

  Scenario: The legacy alias still works
    When the developer runs convention agents-md-size
    Then only "AGENTS.md" is measured
    And the command behaves as a scoped instruction-size run
