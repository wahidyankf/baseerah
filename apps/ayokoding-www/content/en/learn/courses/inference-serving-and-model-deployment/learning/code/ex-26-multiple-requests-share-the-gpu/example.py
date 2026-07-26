"""Example 26: Multiple Requests Share the GPU."""

TOTAL_CACHE_BUDGET_BYTES = 10 * 1024**3  # => co-18: the SAME pool every concurrent request draws from
# => this budget is fixed -- more concurrent requests means a SMALLER share for each, not a bigger pool


def bytes_per_request_if_shared(total_budget: int, num_requests: int) -> int:  # => co-07/co-18
    return total_budget // num_requests  # => equal shares -- a simplified fair-share model


shares = {  # => a lookup table: request count -> per-request share, in MiB
    n: bytes_per_request_if_shared(TOTAL_CACHE_BUDGET_BYTES, n) // 1024**2  # => the SAME formula, four counts
    for n in (1, 2, 5, 10)  # => from serving one request alone up to ten sharing the pool
}  # => closes the dict comprehension
print(shares)  # => Output: {1: 10240, 2: 5120, 5: 2048, 10: 1024} -- share shrinks as concurrency grows

assert bytes_per_request_if_shared(TOTAL_CACHE_BUDGET_BYTES, 1) == TOTAL_CACHE_BUDGET_BYTES  # => alone: gets it all
assert bytes_per_request_if_shared(TOTAL_CACHE_BUDGET_BYTES, 10) == TOTAL_CACHE_BUDGET_BYTES // 10  # => co-18: shared
print("ex-26 OK")  # => a self-check marker confirming the fair-share arithmetic held at both extremes
