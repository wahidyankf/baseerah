Feature: crane-be health endpoint

  @unit @e2e
  Scenario: crane-be reports healthy over HTTP
    Given the crane-be service is running on its configured port
    When a client sends GET to /health
    Then the response status is 200
    And the response body indicates the service is healthy
