Feature: ose-app-be crane PDF conversion via NATS

  @e2e
  Scenario: ose-app-be converts a PDF via the crane NATS path over HTTP
    Given a running stack of ose-app-be, its NATS server, and crane-be
    When a client sends POST to the ose-app-be media-convert endpoint with a sample PDF
    Then the response status is 200
    And the response body contains markdown produced by crane-be
