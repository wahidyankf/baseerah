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

  Scenario: The homepage tells a first-time visitor what Baseerah is
    Given a first-time visitor with no prior context navigates to "/"
    When the page finishes loading
    Then a one-line description of what Baseerah does is visible without scrolling

  Scenario: The multilingual brand chip is understandable to a non-Arabic, non-Indonesian reader
    Given a first-time visitor viewing the homepage brand chip
    When they read or hover the "بصيرة" and "wawasan" terms
    Then a plain-language English gloss or tooltip explains what each term means

  Scenario: A visitor to a non-existent path can recover
    Given a visitor navigates to a non-existent path on baseerah-fe
    When the 404 page renders
    Then it shows Baseerah branding
    And it offers a link back to the homepage
