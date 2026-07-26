"""Example 73: Rollback on Latency Regression."""


def should_rollback(old_version_p99_ms: float, new_version_p99_ms: float, regression_tolerance: float) -> bool:
    # => co-25/co-26: rollback if the NEW version's tail latency regresses beyond a tolerated multiple of the OLD one
    return new_version_p99_ms > old_version_p99_ms * (1 + regression_tolerance)  # => a simple relative-threshold check


old_p99 = 200.0  # => the previous version's known-good tail latency
new_p99_ok = 220.0  # => 10% worse -- within a 15% tolerance
new_p99_bad = 300.0  # => 50% worse -- breaches a 15% tolerance

decision_ok = should_rollback(old_p99, new_p99_ok, regression_tolerance=0.15)  # => co-25: the SAME tolerance, modest regression
decision_bad = should_rollback(old_p99, new_p99_bad, regression_tolerance=0.15)  # => co-25: the SAME tolerance, large regression
print(decision_ok, decision_bad)  # => Output: False True

assert decision_ok is False  # => co-25: a modest regression stays within tolerance -- roll forward
assert decision_bad is True  # => co-26: a large regression is caught by the SAME observability signal and rolled back
# => pairing this check with the staged rollout in Example 63 halts a bad model before it reaches 100%
print("ex-73 OK")  # => a self-check marker confirming the tolerance correctly separated OK from bad regressions
