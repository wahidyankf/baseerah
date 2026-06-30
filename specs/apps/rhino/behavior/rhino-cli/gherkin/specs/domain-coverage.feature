@specs-domain-coverage
Feature: specs domain-coverage validate

  As a developer
  I want rhino-cli specs domain-coverage validate to enforce per-level @covers markers for domain
  scenarios
  So that every domain/** Gherkin scenario is explicitly covered at its declared test levels

  @unit
  Scenario: An uncovered domain scenario fails the gate
    Given a project listed in the specs.domain-areas allowlist
    And a domain scenario not covered at its required level by any @covers marker
    When rhino-cli specs domain-coverage validate runs
    Then it fails and names the uncovered domain scenario

  @unit
  Scenario: A project not in the domain-areas allowlist is skipped
    Given a project not listed in the specs.domain-areas allowlist
    And that project has domain/** feature files
    When rhino-cli specs domain-coverage validate runs
    Then the project is skipped and no violation is reported
