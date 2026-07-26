"""Example 9: No-Cache Recomputation."""


def attention_cost_without_cache(seq_len: int) -> int:  # => co-05: cost of recomputing attention from scratch
    # => at step t, attention scans ALL t previous tokens; summed over t=1..seq_len, that's O(n^2)
    return sum(range(1, seq_len + 1))  # => 1 + 2 + ... + seq_len -- the quadratic sum


cost_10 = attention_cost_without_cache(10)  # => 55
cost_20 = attention_cost_without_cache(20)  # => 210 (double the length)
print(cost_10, cost_20)  # => Output: 55 210

ratio = cost_20 / cost_10  # => the RATIO is the tell -- a linear cost would give exactly 2.0 here
print(round(ratio, 2))  # => Output: 3.82 -- doubling length nearly QUADRUPLES cost, not doubles it

assert cost_20 > cost_10 * 2  # => co-05: growth outpaces linear -- this is the quadratic blowup
assert ratio > 3.5  # => close to 4x, the signature of O(n^2) under a doubled input
print("ex-09 OK")  # => a self-check marker confirming the quadratic-growth assertions held
