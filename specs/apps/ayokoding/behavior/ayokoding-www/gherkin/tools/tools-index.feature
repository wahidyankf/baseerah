Feature: Tools index

  # AC-13 (UWT-009) — The tools index calculator entry has a description distinct from its link text
  Scenario: The calculator entry shows a description distinct from its link text
    Given I am on the tools index page
    When the calculator entry renders
    Then the calculator entry shows a description distinct from its link text
