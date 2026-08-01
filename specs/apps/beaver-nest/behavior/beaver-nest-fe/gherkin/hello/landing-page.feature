Feature: Frontend hello world

  Background:
    Given the beaver-nest-fe app is running on port 19310 against a live beaver-nest-be

  Scenario: The landing page names the product and shows the backend greeting
    Given I have not visited the site before
    When I navigate to "/"
    Then the page shows a level-one heading containing "BeaverNest"
    And the page shows the text "Hello from BeaverNest" sourced from the backend

  Scenario: The landing page meets the baseline accessibility bar
    Given I am on "/"
    When an automated accessibility scan runs against the rendered page
    Then it reports zero serious violations
    And it reports zero critical violations

  Scenario: The homepage tells a first-time visitor what BeaverNest is
    Given a first-time visitor with no prior context navigates to "/"
    When the page finishes loading
    Then a one-line description of what BeaverNest does is visible without scrolling

  Scenario: The homepage no longer renders a brand-chip etymology gloss
    Given a first-time visitor viewing the rendered homepage
    When they inspect the page for a hoverable multilingual term chip
    Then no بصيرة/wawasan-style etymology chip is present
    And no automated test or Gherkin scenario asserts one exists

  Scenario: A visitor to a non-existent path can recover
    Given a visitor navigates to a non-existent path on beaver-nest-fe
    When the 404 page renders
    Then it shows BeaverNest branding
    And it offers a link back to the homepage

  Scenario: External GitHub link announces it opens in a new tab
    Given a first-time visitor viewing the rendered homepage
    When they encounter the "View on GitHub" link
    Then its accessible name indicates it opens in a new browser tab
