"""Example 14: Memory Budget Breakdown."""

GPU_TOTAL_BYTES = 80 * 1024**3  # => a stated total budget -- co-18: everything must fit inside this
WEIGHTS_BYTES = 14 * 1024**3  # => the served model's fixed weight footprint
ACTIVATIONS_BYTES = 4 * 1024**3  # => scratch space used DURING a forward pass
FRAMEWORK_OVERHEAD_BYTES = 2 * 1024**3  # => the serving framework's own fixed memory cost
BYTES_PER_REQUEST_CACHE = 2 * 1024**3  # => co-06: one request's KV cache footprint
# => four FIXED consumers plus one PER-REQUEST consumer -- this split is the whole capacity story


def remaining_for_cache(total: int, weights: int, activations: int, overhead: int) -> int:  # => co-18
    return total - weights - activations - overhead  # => whatever is left is the ONLY budget for cache


cache_budget = remaining_for_cache(  # => subtract every fixed consumer from the total, in order
    GPU_TOTAL_BYTES,  # => the starting budget, before anything is subtracted
    WEIGHTS_BYTES,  # => first fixed consumer -- the served model itself
    ACTIVATIONS_BYTES,  # => second fixed consumer -- forward-pass scratch space
    FRAMEWORK_OVERHEAD_BYTES,  # => third fixed consumer -- the serving framework's own cost
)  # => closes the call -- one subtraction chain, four fixed consumers
# => whatever survives all four subtractions is the ONLY pool cache can ever draw from
max_concurrency = cache_budget // BYTES_PER_REQUEST_CACHE  # => co-18: the remainder buys concurrency
print(cache_budget // 1024**3)  # => Output: 60 -- GiB left over for cache after the fixed consumers
print(max_concurrency)  # => Output: 30 -- floor division: partial capacity for one more request is wasted

assert cache_budget == 60 * 1024**3  # => 80 - 14 - 4 - 2 == 60 GiB left for cache
assert max_concurrency == 30  # => co-18: 60 GiB / 2 GiB-per-request == 30 concurrent requests, at most
print("ex-14 OK")  # => a self-check marker confirming the full budget breakdown held
