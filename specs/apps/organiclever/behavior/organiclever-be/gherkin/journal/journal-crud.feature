Feature: Backend journal CRUD

  As a future server-of-record migration
  I want the organiclever-be backend to persist journal entries over HTTP
  So that the PGlite client journal can one day be backed by PostgreSQL

  Background:
    Given the journal API is running

  Scenario: Create a journal entry
    When a client posts a valid journal entry
    Then the journal response status code should be 201
    And the journal response body should include an id

  Scenario: Reject a journal entry with a blank name
    When a client posts a journal entry with a blank name
    Then the journal response status code should be 400

  Scenario: List journal entries
    Given a journal entry has been created
    When a client lists the journal entries
    Then the journal response status code should be 200
    And the journal list should include the created entry

  Scenario: Fetch a missing journal entry
    When a client fetches a journal entry that does not exist
    Then the journal response status code should be 404

  Scenario: Update a journal entry
    Given a journal entry has been created
    When a client updates the journal entry name
    Then the journal response status code should be 200
    And the updated journal entry should reflect the new name

  Scenario: Delete a journal entry
    Given a journal entry has been created
    When a client deletes the journal entry
    Then the journal response status code should be 204
    And fetching the deleted journal entry should return 404
