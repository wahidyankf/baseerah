Feature: IA navigation revamp

  As a reader visiting AyoKoding after the information architecture revamp
  I want content to be reachable under the /c namespace with proper navigation chrome
  So that I can browse and read content through the new URL structure

  Background:
    Given the app is running

  Scenario: English content resolves under the /c namespace
    When a visitor navigates to "/en/c/learn/software-engineering"
    Then the page should respond with HTTP 200
    And a breadcrumb nav should be present

  Scenario: The /c browse index lists all content sections
    When a visitor navigates to "/en/c"
    Then the page should load successfully
    And the browse index should show a section card for "learn"
    And the browse index should show a section card for "rants"
    And a breadcrumb nav should be present
    And the breadcrumb should start with a Home link
