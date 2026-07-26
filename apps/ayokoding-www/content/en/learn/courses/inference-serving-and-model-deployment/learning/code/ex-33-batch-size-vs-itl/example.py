"""Example 33: Batch Size vs Inter-Token Latency."""

BASE_STEP_MS = 15.0  # => co-16: a fixed floor cost per decode step
MS_PER_EXTRA_SLOT = 1.0  # => co-15: cost added per additional concurrently-batched sequence
# => a fixed floor plus a per-slot penalty -- the whole batch-size-vs-latency tension in one formula


def step_latency_ms(batch_size: int) -> float:  # => co-15: ITL grows as the batch gets fuller
    return BASE_STEP_MS + MS_PER_EXTRA_SLOT * batch_size  # => linear in batch size, by construction


itl_by_batch = {b: step_latency_ms(b) for b in (1, 4, 16, 64)}  # => a small, a mid, and two large batch sizes
print(itl_by_batch)  # => Output: {1: 16.0, 4: 19.0, 16: 31.0, 64: 79.0}

aggregate_throughput = {b: b * 1000.0 / itl_by_batch[b] for b in itl_by_batch}  # => tokens/sec, one per slot
print({b: round(t, 1) for b, t in aggregate_throughput.items()})
# => Output: {1: 62.5, 4: 210.5, 16: 516.1, 64: 810.1}

assert itl_by_batch[64] > itl_by_batch[1]  # => co-15: bigger batch, WORSE per-user latency
assert aggregate_throughput[64] > aggregate_throughput[1]  # => co-15: but BETTER aggregate throughput
print("ex-33 OK")  # => a self-check marker confirming both sides of the latency/throughput trade held
