Feature: ose-app-be messaging configuration

  @unit
  Scenario: ose-app-be fails fast when its NATS URL is missing
    Given OSE_APP_BE_NATS_URL is unset
    When ose-app-be reads its messaging configuration
    Then startup aborts with a clear missing-variable error
