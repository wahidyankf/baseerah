@env-validate-app-drift
Feature: `env validate` detects declared-but-unread and read-but-undeclared env-key drift

  As a maintainer keeping .env.example and application code in sync
  I want `env validate` to flag keys declared in .env.example but never read
  by the app, and keys read by the app but never declared in .env.example
  So that stale or missing environment variables are caught before they
  reach production

  Scenario: A key declared in .env.example but never read by the app fails validation
    Given an app surface whose .env.example declares a key the source code never reads
    When the developer runs env validate
    Then the command exits with a failure code
    And the output names the key as declared-but-unread

  Scenario: A key read by the app but never declared in .env.example fails validation
    Given an app surface whose source code reads a key absent from .env.example
    When the developer runs env validate
    Then the command exits with a failure code
    And the output names the key as read-but-undeclared
