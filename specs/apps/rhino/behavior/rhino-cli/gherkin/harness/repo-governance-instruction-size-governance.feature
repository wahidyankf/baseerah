@repo-governance-instruction-size-governance
Feature: Governance of the instruction-file size-budget rule

  As a repository maintainer
  I want the size-budget rule documented, checker-aware, and preflight-tracked
  So that the gate is discoverable, AI-checkable, and deterministically enforced

  Scenario: The rule is documented as a convention
    Given the plan is complete
    When I look under "repo-governance/conventions/structure/"
    Then "instruction-file-size-budget.md" exists
    And the file lists the monitored file class, per-file budgets, and enforcement points

  Scenario: repo-rules-checker validates the budget qualitatively
    Given the plan is complete
    When "repo-rules-checker" runs Step 6
    Then it reports qualitative bloat concerns across the whole instruction-file class
    And it annotates that the byte ceiling is enforced by the deterministic "instruction-size" gate

  Scenario: The quality-gate workflow lists the validator as a fourth preflight category
    Given the plan is complete
    When I read "repo-governance/workflows/repo/repo-rules-quality-gate.md"
    Then "instruction-size" is named among the Step 0.5 categories

  Scenario: The preflight envelope carries the instruction-size category
    Given a repo with instruction files within the configured budgets
    When the developer runs "rhino-cli repo-governance audit" with JSON output
    Then the envelope schema is "rhino-cli/repo-governance-audit/v1"
    And "result.categories" contains a category named "instruction-size"

  Scenario: The AI checker defers to the deterministic preflight finding
    Given a preflight JSON contains an "instruction-size" category with findings
    When "repo-rules-checker" runs Step 0.5
    Then it populates the deterministic skip set with "instruction-size"
    And it embeds the preflight findings verbatim under "Deterministic Findings"
    And it does not re-derive byte counts in Step 6
