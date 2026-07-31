Feature: Frontend hello world

  Background:
    Given the baseerah-fe app is running on port 19310 against a live baseerah-be

  Scenario: The landing page names the product and shows the backend greeting
    Given I have not visited the site before
    When I navigate to "/"
    Then the page shows a level-one heading containing "Baseerah"
    And the page shows the text "Hello from Baseerah" sourced from the backend

  Scenario: The landing page meets the baseline accessibility bar
    Given I am on "/"
    When an automated accessibility scan runs against the rendered page
    Then it reports zero serious violations
    And it reports zero critical violations
