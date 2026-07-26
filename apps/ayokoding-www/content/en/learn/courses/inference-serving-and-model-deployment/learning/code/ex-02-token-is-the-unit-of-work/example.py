"""Example 2: Token Is the Unit of Work."""

DECODE_COST_PER_TOKEN_MS = 20.0  # => co-01: a fixed per-token cost, independent of "which" request
# => this constant models real decode latency: bigger models cost more per token, but the SHAPE
# => of the cost curve (linear in tokens) holds regardless of the specific per-token constant


def estimate_cost_ms(tokens_per_request: list[int]) -> float:  # => co-01: cost as a function of tokens
    total_tokens = sum(tokens_per_request)  # => the TRUE unit of work -- not the request count
    return total_tokens * DECODE_COST_PER_TOKEN_MS  # => cost scales with TOKENS, never request count


# => Two workloads with the IDENTICAL request count (5 each) but wildly different token counts
short_workload = [10, 10, 10, 10, 10]  # => 5 requests, 50 tokens total
long_workload = [500, 500, 500, 500, 500]  # => 5 requests, 2500 tokens total

short_cost = estimate_cost_ms(short_workload)  # => same function, same formula, different input size
long_cost = estimate_cost_ms(long_workload)  # => the ONLY difference between these two calls is tokens
print(len(short_workload), len(long_workload))  # => Output: 5 5 -- SAME request count
print(short_cost, long_cost)  # => Output: 1000.0 50000.0 -- WILDLY different real cost

assert len(short_workload) == len(long_workload)  # => request count alone says "identical load"
assert long_cost == short_cost * 50  # => but token-measured cost says otherwise, by 50x
print("ex-02 OK")  # => a self-check marker, confirming the assertions above held
