Feature: crane-be NATS core request/reply

  @e2e
  Scenario: crane-be answers a NATS core request/reply on crane.convert
    Given crane-be has subscribed to subject crane.convert on a backend NATS server
    When a backend publishes a request to crane.convert with sample PDF bytes
    Then crane-be replies on the auto _INBOX subject with markdown
    And the requesting backend receives the markdown reply

  @e2e
  Scenario: crane-be replies with an error envelope for an unparseable NATS payload
    Given crane-be has subscribed to subject crane.convert on a backend NATS server
    When a backend publishes a request to crane.convert with bytes that are not a PDF
    Then crane-be replies on the auto _INBOX subject with an error envelope
    And the error envelope names the parse failure
