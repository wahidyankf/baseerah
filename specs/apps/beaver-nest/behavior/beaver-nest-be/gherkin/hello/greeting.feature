Feature: Backend hello world

  Background:
    Given the beaver-nest-be service is running on port 19320

  @unit
  Scenario: The service returns a greeting
    Given the service has finished starting
    When I send a GET request to "/api/v1/hello"
    Then the response status is 200
    And the response body field "message" equals "Hello from BeaverNest"

  @unit
  Scenario: An unknown route is refused
    Given the service has finished starting
    When I send a GET request to "/api/v1/does-not-exist"
    Then the response status is 404
    And the response body field "error" is a non-empty string

  @unit
  Scenario: An undeclared query string is ignored
    Given the service has finished starting
    When I send a GET request to "/api/v1/hello?extra=param"
    Then the response status is 200
    And the response body field "message" equals "Hello from BeaverNest"
