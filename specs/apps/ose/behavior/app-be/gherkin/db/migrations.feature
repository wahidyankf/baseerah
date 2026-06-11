Feature: Database migrations on boot

  As a platform operator
  I want the backend to automatically apply pending schema migrations on startup
  So that the database schema is always in sync without manual intervention

  @integration
  Scenario: Backend applies pending migrations on startup
    Given the ose-app-be database has no applied migrations
    When the ose-app-be backend runs its migration routine
    Then the ose-app-be migrations table records at least one applied migration
