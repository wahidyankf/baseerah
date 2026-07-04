Feature: Search

  As a reader visiting AyoKoding
  I want to search for content across the site
  So that I can quickly find the articles and topics I am looking for

  Background:
    Given the app is running

  @unit @e2e
  Scenario: Cmd+K keyboard shortcut opens the search dialog
    When a visitor presses Cmd+K on the page
    Then the search dialog should open
    And the search input should have focus

  @unit @e2e
  Scenario: Typing in the search input shows debounced results
    Given the search dialog is open
    When the visitor types a query into the search input
    Then search results should appear after a debounce delay
    And results should update when the visitor changes the query

  @unit @e2e
  Scenario: Clicking a search result navigates to that page
    Given the search dialog is open
    And the visitor has typed a query that returns at least one result
    When the visitor clicks a search result
    Then the search dialog should close
    And the visitor should be navigated to the page for that result

  @unit @e2e
  Scenario: Escape key closes the search dialog
    Given the search dialog is open
    When the visitor presses Escape
    Then the search dialog should close
    And focus should return to the page behind the dialog

  @unit @e2e
  Scenario: Search results show title, section path, and excerpt
    Given the search dialog is open
    When the visitor types a query that returns results
    Then each result should display the page title
    And each result should display the section path indicating where the page lives
    And each result should display a text excerpt showing the matching content
