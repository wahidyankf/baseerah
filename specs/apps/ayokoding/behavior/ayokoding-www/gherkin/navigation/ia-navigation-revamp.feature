Feature: IA navigation revamp

  As a reader visiting AyoKoding after the information architecture revamp
  I want content to be reachable under the /c namespace with proper navigation chrome
  So that I can browse and read content through the new URL structure

  Background:
    Given the app is running

  @unit @e2e
  Scenario: English content resolves under the /c namespace
    When a visitor navigates to "/en/c/learn/software-engineering"
    Then the page should respond with HTTP 200
    And a breadcrumb nav should be present

  @unit @e2e
  Scenario: The /c browse index lists all content sections
    When a visitor navigates to "/en/c"
    Then the page should load successfully
    And the browse index should show a section card for "learn"
    And the browse index should show a section card for "rants"
    And a breadcrumb nav should be present
    And the breadcrumb should start with a Home link

  @unit @e2e
  Scenario: Header shows primary nav links on desktop
    Given the viewport is set to desktop width
    When a visitor navigates to "/en"
    Then the header primary nav should contain a link to "/en/c" labelled "Learn"
    And the header primary nav should contain a link to "/en/tools" labelled "Tools"

  @unit @e2e
  Scenario: Mobile navigation mirrors the header links
    Given the viewport is set to mobile width
    When a visitor navigates to "/en"
    And the visitor opens the mobile navigation menu
    Then the mobile nav should contain a link to "/en/c" labelled "Learn"
    And the mobile nav should contain a link to "/en/tools" labelled "Tools"

  @unit @e2e
  Scenario: Footer shows grouped navigation with localized labels
    When a visitor navigates to "/id"
    Then the footer should display a "Learn" column
    And the footer should display a "Tools" column
    And the footer should display an "About" column
    And the footer "About" column should link to "/id/tentang-ayokoding"
    And the footer "About" column should link to "/id/syarat-dan-ketentuan"

  @unit @e2e
  Scenario: Landing homepage renders hero, sections, and tools teaser in English
    When a visitor navigates to "/en"
    Then the hero heading should be visible on the landing page
    And the hero intro should be visible on the landing page
    And the landing section grid should include a card linking to "/en/c/rants"
    And the tools teaser should link to "/en/tools/cost-of-living-calculator"

  @unit @e2e
  Scenario: Landing homepage renders hero, sections, and tools teaser in Indonesian
    When a visitor navigates to "/id"
    Then the hero heading should be visible on the landing page
    And the hero intro should be visible on the landing page
    And the landing section grid should include a card linking to "/id/c/celoteh"
    And the tools teaser should link to "/id/tools/cost-of-living-calculator"

  @unit @e2e
  Scenario: Breadcrumb segments link to /c URLs
    Given a visitor is on "/en/c/learn/software-engineering/data"
    When the breadcrumb renders its ancestor segments
    Then each ancestor crumb links to a "/c/" prefixed URL

  @unit @e2e
  Scenario: Internal content links emit /c URLs directly without relying on redirects
    Given the sidebar tree, breadcrumb, prev-next, and search results render content links
    When their hrefs are computed via the central content URL helper
    Then every content link resolves directly to a "/c/" URL with status 200
    And no internal content link resolves through a 308 redirect

  @unit @e2e
  Scenario: Sitemap lists only the new /c content URLs
    Given the sitemap is generated from the content index
    When the sitemap entries are produced
    Then every moved-content entry uses a "/c/" prefixed URL
    But top-level pages (about, terms, tools) are not prefixed with "/c/"

  @unit @e2e
  Scenario: RSS feed item links use the new /c content URLs
    Given the feed is generated from the content index
    When the feed items are produced
    Then every content item link uses a "/c/" prefixed URL

  @unit @e2e
  Scenario: Canonical link for moved content points to the /c URL
    Given the content page at "/en/c/learn/software-engineering"
    When its metadata is generated
    Then the canonical alternate is "/en/c/learn/software-engineering"
    And the language alternates include en and x-default
