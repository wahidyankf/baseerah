Feature: Content namespace redirects

  As a reader following an old content URL
  I want the URL to permanently redirect to its new /c/ namespace location
  So that bookmarks and external links continue to resolve after the IA revamp

  Background:
    Given the app is running

  @unit @e2e
  Scenario: Old English learn URL permanently redirects to the /c namespace
    When a raw HTTP GET is made to "/en/learn/software-engineering" with redirects disabled
    Then the response status should be 308
    And the response Location header should equal "/en/c/learn/software-engineering"

  @unit @e2e
  Scenario: Old Indonesian belajar URL permanently redirects to the /c namespace
    When a raw HTTP GET is made to "/id/belajar/ikhtisar" with redirects disabled
    Then the response status should be 308
    And the response Location header should equal "/id/c/belajar/ikhtisar"

  @unit @e2e
  Scenario: About page keeps its top-level URL and is not redirected
    When a visitor navigates to "/en/about-ayokoding"
    Then the page should load successfully
    And the current URL should not contain "/c/"

  @unit @e2e
  Scenario: Indonesian terms page keeps its top-level URL and is not redirected
    When a visitor navigates to "/id/syarat-dan-ketentuan"
    Then the page should load successfully
    And the current URL should not contain "/c/"

  @unit @e2e
  Scenario: Tools index keeps its top-level URL and is not redirected
    When a visitor navigates to "/en/tools"
    Then the page should load successfully
    And the current URL should not contain "/c/"
