Feature: organiclever-be messaging configuration

  @unit
  Scenario: organiclever-be fails fast when its NATS URL is missing
    Given ORGANICLEVER_BE_NATS_URL is unset
    When organiclever-be reads its messaging configuration
    Then startup aborts with a clear missing-variable error
