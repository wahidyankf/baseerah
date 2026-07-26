"""Example 12: Cache Growth Over a Generation."""

BYTES_PER_TOKEN = 98304  # => co-06: a fixed per-token cache cost for one worked model configuration
# => this is Example 11's kv_cache_bytes() formula collapsed to "bytes per ONE additional token"
# => holding num_layers, num_heads, head_dim, and precision fixed while step count varies


def cache_bytes_at_step(step: int) -> int:  # => co-07: cache is the scarce, GROWING resource
    return step * BYTES_PER_TOKEN  # => co-06: grows LINEARLY, one token's worth per decode step


occupancy = [cache_bytes_at_step(s) for s in (0, 100, 500, 1000, 2000)]  # => sampled generation steps
print(occupancy)  # => Output: [0, 9830400, 49152000, 98304000, 196608000]

deltas = [occupancy[i + 1] - occupancy[i] for i in range(len(occupancy) - 1)]  # => growth per interval
print(deltas[0] == 100 * BYTES_PER_TOKEN)  # => Output: True -- linear: same rate at every interval

assert occupancy[0] == 0  # => an empty generation has written nothing to the cache yet
assert occupancy[-1] == 2000 * BYTES_PER_TOKEN  # => co-06: cache size IS step count times per-token cost
print("ex-12 OK")  # => a self-check marker confirming the linear growth rate held at every interval
