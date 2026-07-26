"""Example 24: Estimate Max Concurrency (Simple)."""


def max_concurrency(total_gpu_bytes: int, weights_bytes: int, bytes_per_request: int) -> int:  # => co-07/co-18
    remaining = total_gpu_bytes - weights_bytes  # => co-18: whatever is left after weights, for cache
    if remaining <= 0:  # => weights alone don't even fit -- zero concurrency is possible
        return 0  # => a defensive floor -- a negative remainder would otherwise give a nonsense answer
    return remaining // bytes_per_request  # => co-07: how many requests' worth of cache fit


gpu_total = 24 * 1024**3  # => a common consumer/workstation GPU size
weights = 13 * 1024**3  # => a served model's fixed weight footprint
per_request = 500 * 1024**2  # => 500 MiB per request

concurrency = max_concurrency(gpu_total, weights, per_request)  # => the smaller GPU's ceiling
bigger_gpu_concurrency = max_concurrency(80 * 1024**3, weights, per_request)  # => same model, bigger GPU
print(concurrency, bigger_gpu_concurrency)  # => Output: 22 137

assert concurrency > 0  # => co-18: this GPU can serve SOME concurrent requests
assert bigger_gpu_concurrency > concurrency  # => co-18: more total memory, more concurrency headroom
# => this "simple" estimate ignores fragmentation and activations -- later examples refine it
print("ex-24 OK")  # => a self-check marker confirming both concurrency estimates held
