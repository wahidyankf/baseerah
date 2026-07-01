@wip
Feature: Internal markdown link checking
  As a CLI author consuming rust-commons
  I want check_links to walk a content directory and report broken internal links
  So that ayokoding-cli and ose-cli can catch broken links in Next.js content before deploy

  Scenario: A broken internal link is reported
    Given a content directory with a markdown file linking to "/does-not-exist"
    When I run check_links on the content directory
    Then the result should contain 1 broken link

  Scenario: A valid internal link is not reported as broken
    Given a content directory with a markdown file linking to an existing page
    When I run check_links on the content directory
    Then the result should contain 0 broken links
