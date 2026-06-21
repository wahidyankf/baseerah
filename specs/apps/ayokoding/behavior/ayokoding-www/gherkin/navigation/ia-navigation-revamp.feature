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

  Scenario: Header shows primary nav links on desktop
    Given the viewport is set to desktop width
    When a visitor navigates to "/en"
    Then the header primary nav should contain a link to "/en/c" labelled "Learn"
    And the header primary nav should contain a link to "/en/tools" labelled "Tools"

  Scenario: Mobile navigation mirrors the header links
    Given the viewport is set to mobile width
    When a visitor navigates to "/en"
    And the visitor opens the mobile navigation menu
    Then the mobile nav should contain a link to "/en/c" labelled "Learn"
    And the mobile nav should contain a link to "/en/tools" labelled "Tools"

  Scenario: Footer shows grouped navigation with localized labels
    When a visitor navigates to "/id"
    Then the footer should display a "Learn" column
    And the footer should display a "Tools" column
    And the footer should display an "About" column
    And the footer "About" column should link to "/id/tentang-ayokoding"
    And the footer "About" column should link to "/id/syarat-dan-ketentuan"

  Scenario: Landing homepage renders hero, sections, and tools teaser in English
    When a visitor navigates to "/en"
    Then the hero heading should be visible on the landing page
    And the hero intro should be visible on the landing page
    And the landing section grid should include a card linking to "/en/c/rants"
    And the tools teaser should link to "/en/tools/cost-of-living-calculator"

  Scenario: Landing homepage renders hero, sections, and tools teaser in Indonesian
    When a visitor navigates to "/id"
    Then the hero heading should be visible on the landing page
    And the hero intro should be visible on the landing page
    And the landing section grid should include a card linking to "/id/c/celoteh"
    And the tools teaser should link to "/id/tools/cost-of-living-calculator"
