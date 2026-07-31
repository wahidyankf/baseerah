Feature: Backend hello world

  Background:
    Given the baseerah-be service is running on port 19320

  Scenario: The service reports liveness
    Given the service has finished starting
    When I send a GET request to "/api/v1/health"
    Then the response status is 200
    And the response body field "status" equals "ok"
