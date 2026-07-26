"""Example 59: Capacity Model."""


def kv_cache_bytes_per_request(num_layers: int, num_heads: int, head_dim: int, seq_len: int, bytes_per_value: int) -> int:
    # => co-06: the SAME cache-size formula from Example 11, reused here as ONE input to capacity planning
    return 2 * num_layers * num_heads * head_dim * seq_len * bytes_per_value  # => *2 accounts for BOTH K and V


def max_concurrency(cache_budget_bytes: int, bytes_per_request: int) -> int:
    # => co-07/co-21: cache budget, not raw compute, is what SETS the concurrency ceiling
    return cache_budget_bytes // bytes_per_request  # => floor division -- a partial extra request never counts


cache_budget_bytes = 20 * 1024**3  # => 20 GiB left over after weights and activations -- co-18's remainder
per_request_bytes = kv_cache_bytes_per_request(num_layers=32, num_heads=32, head_dim=128, seq_len=2048, bytes_per_value=2)
# => same formula, same shape as Example 11, applied to a larger 32-layer configuration
print(per_request_bytes)  # => Output: 1073741824 -- exactly 1 GiB per request, at this configuration

capacity = max_concurrency(cache_budget_bytes, per_request_bytes)  # => co-21: budget divided by per-request cost
print(capacity)  # => Output: 20 -- 20 GiB of budget divided by 1 GiB per request

assert capacity > 0  # => co-18: this GPU can serve SOME concurrent requests at this configuration
assert capacity == cache_budget_bytes // per_request_bytes  # => co-21: capacity planning IS this one division, end to end
# => Example 71 shows what happens when autoscaling reacts to this ceiling too aggressively
print("ex-59 OK")  # => a self-check marker confirming the capacity model reduced to one clean division
