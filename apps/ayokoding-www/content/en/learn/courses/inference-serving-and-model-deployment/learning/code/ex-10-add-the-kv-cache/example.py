"""Example 10: Add the KV Cache."""


def attention_cost_with_cache(seq_len: int) -> int:  # => co-05: each NEW token's K/V projection is O(1) with a cache
    # => the cache already holds every PREVIOUS token's key/value -- only the new token's K/V is projected
    # => this measures projection work only: scoring the new query against every cached key/value
    # => (the attention step itself) still costs O(t) per step and is NOT eliminated by caching
    return seq_len  # => O(n) total: one constant-cost projection per token, not one O(t) re-projection per token


def attention_cost_without_cache(seq_len: int) -> int:  # => same formula as Example 9, for comparison
    return sum(range(1, seq_len + 1))  # => O(n^2) total -- every step re-projects everything from scratch


seq_len = 20  # => same sequence length fed to both cost functions, for an apples-to-apples comparison
cached_cost = attention_cost_with_cache(seq_len)  # => the cache-backed path
uncached_cost = attention_cost_without_cache(seq_len)  # => the no-cache path, same input
print(cached_cost, uncached_cost)  # => Output: 20 210

assert cached_cost == seq_len  # => co-05: K/V projection work is linear in sequence length, not quadratic
assert cached_cost < uncached_cost  # => the SAME final K/V projections, far cheaper to reach
print("ex-10 OK")  # => a self-check marker confirming the cache/no-cache comparison held
