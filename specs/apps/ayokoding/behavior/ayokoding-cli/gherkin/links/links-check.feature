@links-check-ayokoding
Feature: Internal Link Validation for ayokoding-www

  As a content author
  I want to validate internal links in ayokoding-www content
  So that readers always reach the intended pages

  @integration
  Scenario: A content directory with all valid Hugo-path links passes validation
    Given ayokoding-www content where all internal links resolve correctly
    When the developer runs links check
    Then the command exits successfully

  @integration
  Scenario: A broken internal link is detected and reported
    Given ayokoding-www content with a link pointing to a non-existent page
    When the developer runs links check
    Then the command exits with a failure code

  @integration
  Scenario: External URLs are not validated
    Given ayokoding-www content with only external HTTPS links
    When the developer runs links check
    Then the command exits successfully

  @integration
  Scenario: JSON output produces structured results
    Given ayokoding-www content where all internal links resolve correctly
    When the developer runs links check with JSON output
    Then the command exits successfully
    And the output is valid JSON
