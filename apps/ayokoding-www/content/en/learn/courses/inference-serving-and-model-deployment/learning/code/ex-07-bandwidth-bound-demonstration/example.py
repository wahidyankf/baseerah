"""Example 7: Bandwidth-Bound Demonstration."""

BASE_TOKENS_PER_SEC_PER_SLOT = 50.0  # => co-04: each concurrent sequence adds this many tokens/sec...
BANDWIDTH_CEILING_TOKENS_PER_SEC = 400.0  # => ...until the GPU's memory bandwidth is fully saturated
# => two constants, one min() -- this IS the entire bandwidth-bound-decode model in miniature


def aggregate_throughput(batch_size: int) -> float:  # => co-04: models the bandwidth-bound ceiling
    naive = batch_size * BASE_TOKENS_PER_SEC_PER_SLOT  # => what throughput WOULD be with no ceiling
    return min(naive, BANDWIDTH_CEILING_TOKENS_PER_SEC)  # => co-04: bandwidth caps it, compute does not


throughputs = [aggregate_throughput(b) for b in (1, 2, 4, 8, 16)]  # => co-03: batch sizes tried
print(throughputs)  # => Output: [50.0, 100.0, 200.0, 400.0, 400.0] -- flattens out, does not keep climbing

assert throughputs[3] == BANDWIDTH_CEILING_TOKENS_PER_SEC  # => batch=8 already hits the ceiling
assert throughputs[4] == throughputs[3]  # => co-04: doubling batch size AGAIN buys nothing more
print("ex-07 OK")  # => a self-check marker confirming the ceiling behavior held
