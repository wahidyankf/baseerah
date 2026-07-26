"""Example 23: Why Request Count Is a Bad Metric."""

DECODE_TOKENS_PER_SEC_BUDGET = 1000.0  # => co-01: the GPU's real, physical throughput ceiling
# => two capacity models are compared below -- ONE ignores tokens, the OTHER doesn't


def naive_capacity_by_requests(requests_per_sec: float) -> bool:  # => WRONG model: ignores token cost
    return requests_per_sec <= 50  # => an arbitrary "50 requests/sec feels fine" guess


def true_capacity_by_tokens(requests_per_sec: float, avg_tokens_per_request: float) -> bool:  # => co-01: RIGHT model
    return requests_per_sec * avg_tokens_per_request <= DECODE_TOKENS_PER_SEC_BUDGET
    # => this IS Example 2's lesson applied to a capacity DECISION, not just a cost estimate


workload_a = (40.0, 10.0)  # => 40 requests/sec, short 10-token replies -- 400 tokens/sec
workload_b = (40.0, 100.0)  # => the SAME 40 requests/sec, but long 100-token replies -- 4000 tokens/sec

naive_a = naive_capacity_by_requests(workload_a[0])  # => only looks at the request rate
naive_b = naive_capacity_by_requests(workload_b[0])  # => same request rate as workload_a -- same verdict
true_a = true_capacity_by_tokens(*workload_a)  # => accounts for the actual reply length too
true_b = true_capacity_by_tokens(*workload_b)  # => the SAME rate, but a very different token load
print(naive_a, naive_b)  # => Output: True True -- the naive model says BOTH workloads are fine
print(true_a, true_b)  # => Output: True False -- the token-aware model correctly flags workload_b

assert naive_a == naive_b  # => co-01: request-count alone cannot tell these workloads apart
assert true_a and not true_b  # => but the token-aware model correctly distinguishes them
print("ex-23 OK")  # => a self-check marker confirming the naive/true-model disagreement held
