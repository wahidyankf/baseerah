Feature: ose-be JetStream durable demo

  @e2e
  Scenario: ose-be publishes and durably consumes its demo subject with ack
    Given ose-be has a JetStream durable stream and consumer for its demo subject
    When ose-be publishes a demo message to that subject
    Then the durable consumer receives the message
    And the message is acknowledged
    And the messaging status surface reports the demo delivered and acked
