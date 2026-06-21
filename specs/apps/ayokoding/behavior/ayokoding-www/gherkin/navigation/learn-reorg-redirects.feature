Feature: Learn-tree reorganization redirects

  As a content reader
  I want old learn-tree URLs to redirect to their new canonical locations
  So that inbound links from past blog posts and external references continue to resolve

  Background:
    Given the app is running

  Scenario: platform-web redirects to platforms/web under the /c namespace
    When a visitor navigates to "/en/learn/software-engineering/platform-web"
    Then the current URL should contain "/en/c/learn/software-engineering/platforms/web"
