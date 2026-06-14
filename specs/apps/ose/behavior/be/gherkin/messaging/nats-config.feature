Feature: ose-be messaging configuration

  @unit
  Scenario: ose-be fails fast when its NATS URL is missing
    Given OSE_BE_NATS_URL is unset
    When ose-be reads its messaging configuration
    Then startup aborts with a clear missing-variable error
