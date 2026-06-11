Feature: crane-be dual NATS connection isolation

  @e2e
  Scenario: the single crane-be serves both backends over independent NATS connections
    Given crane-be has opened one NATS connection to each backend's NATS server
    And each subscription uses the same queue group crane.workers
    When each backend independently issues a crane.convert request
    Then each backend receives a markdown reply from crane-be
    And neither backend's request is delivered to the other backend's NATS server
