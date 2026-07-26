"""Example 72: Canary Rollout With Metrics Guardrail."""


def evaluate_canary_stage(error_rate: float, p99_latency_ms: float, error_guardrail: float, latency_guardrail_ms: float) -> str:
    # => co-25: a stage is healthy ONLY if BOTH guardrails hold -- either one alone is not enough
    if error_rate > error_guardrail:  # => the FIRST gate -- checked before latency is even looked at
        return "halt: error_rate_guardrail_breached"
    if p99_latency_ms > latency_guardrail_ms:  # => co-25: a SECOND, independent gate -- errors alone are not enough
        return "halt: latency_guardrail_breached"
    return "advance"  # => both gates cleared -- the stage is genuinely healthy


stage_results = [  # => co-25: the SAME guardrails, three different simulated health states
    evaluate_canary_stage(0.002, 180.0, error_guardrail=0.01, latency_guardrail_ms=250.0),  # => healthy on both axes
    evaluate_canary_stage(0.002, 400.0, error_guardrail=0.01, latency_guardrail_ms=250.0),  # => latency regression only
    evaluate_canary_stage(0.05, 180.0, error_guardrail=0.01, latency_guardrail_ms=250.0),  # => error spike only
]
print(stage_results)
# => Output: ['advance', 'halt: latency_guardrail_breached', 'halt: error_rate_guardrail_breached']
# => co-25: this contrasts with Example 63, which only checked ONE signal (error rate alone)

assert stage_results[0] == "advance"  # => co-25: healthy on BOTH axes -- correctly allowed to proceed
# => "advance" here means the NEXT wider traffic stage becomes eligible, same idea as Example 63
assert stage_results[1] == "halt: latency_guardrail_breached"  # => co-25: a latency-ONLY regression is caught too
# => an error-rate-only rollout gate would have MISSED this regression entirely
assert stage_results[2] == "halt: error_rate_guardrail_breached"  # => co-25: an error-ONLY spike is caught too
print("ex-72 OK")  # => a self-check marker confirming both independent guardrails caught their own failure mode
# => co-25: a real canary would also gate on custom business metrics, not just error rate and latency
