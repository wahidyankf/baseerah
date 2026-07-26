"""Example 15: Memory Budget -- Scaling the Cache Portion."""

WEIGHTS_BYTES = 14 * 1024**3  # => co-18: FIXED regardless of how many requests are served
ACTIVATIONS_BYTES = 4 * 1024**3  # => FIXED -- scratch space sized by the framework, not by traffic
OVERHEAD_BYTES = 2 * 1024**3  # => FIXED -- the serving framework's own baseline cost
# => three fixed consumers subtracted from a VARYING total -- the cache share is what's left


def cache_share(total_bytes: int) -> int:  # => co-18/co-06: only the CACHE consumer scales with total
    return total_bytes - WEIGHTS_BYTES - ACTIVATIONS_BYTES - OVERHEAD_BYTES
    # => same three constants subtracted every time -- only the input total changes between calls


small_gpu = cache_share(40 * 1024**3)  # => a smaller GPU
large_gpu = cache_share(80 * 1024**3)  # => a GPU with double the total memory
# => the fixed 20 GiB overhead matters PROPORTIONALLY more on the smaller GPU
print(small_gpu // 1024**3, large_gpu // 1024**3)  # => Output: 20 60

assert large_gpu - small_gpu == (80 - 40) * 1024**3  # => co-06: every extra GiB of GPU goes STRAIGHT to cache
assert large_gpu > small_gpu * 2  # => cache share grows FASTER than total, since fixed costs don't scale
print("ex-15 OK")  # => a self-check marker confirming cache share scales super-linearly with GPU size
