"""Example 27: Cache Budget vs Sequence Length Table."""

BYTES_PER_TOKEN = 98304  # => co-06: the same per-token cost used in Examples 12 and 20
CACHE_BUDGET_BYTES = 12 * 1024**3  # => a fixed cache budget -- the denominator every concurrency count divides


def concurrent_requests_at_length(cache_budget: int, seq_len: int) -> int:  # => co-06/co-07
    bytes_per_request = seq_len * BYTES_PER_TOKEN  # => co-06: cost of ONE request at this sequence length
    return cache_budget // bytes_per_request  # => co-07: how many such requests the budget admits


table = {  # => a lookup table: sequence length -> max concurrent requests at that length
    length: concurrent_requests_at_length(CACHE_BUDGET_BYTES, length)  # => the SAME formula, four lengths
    for length in (256, 512, 1024, 2048)  # => a doubling sequence, chosen so the pattern is easy to see
}  # => closes the dict comprehension
print(table)  # => Output: {256: 512, 512: 256, 1024: 128, 2048: 64} -- concurrency halves as length doubles

assert table[512] == table[256] // 2  # => co-06: DOUBLING sequence length HALVES achievable concurrency
assert table[2048] == table[1024] // 2  # => the same halving relationship holds at every length
# => this is Example 20's context-window ceiling, viewed from the opposite direction
print("ex-27 OK")  # => a self-check marker confirming the halving relationship held at both scales
