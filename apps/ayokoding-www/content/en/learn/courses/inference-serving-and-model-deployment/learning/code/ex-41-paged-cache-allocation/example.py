"""Example 41: Paged Cache Allocation."""

import math  # => stdlib only -- ceil() is the entire "round up to nearest block" trick

BLOCK_TOKENS = 16  # => co-09: a FIXED block size, the same principle as OS virtual-memory paging
BYTES_PER_TOKEN = 1000  # => same illustrative constant used in Example 40, for a direct comparison


def paged_alloc_bytes(actual_len: int, block_tokens: int, bytes_per_token: int) -> int:
    # => co-09: round UP to the nearest whole block -- waste is bounded by ONE block, not the max length
    blocks_needed = math.ceil(actual_len / block_tokens)  # => the number of FULL blocks this length needs
    return blocks_needed * block_tokens * bytes_per_token  # => allocated bytes -- always a multiple of one block


requests_actual_lengths = [50, 2000, 30, 40]  # => the SAME mixed workload as Example 40
paged_total = sum(paged_alloc_bytes(length, BLOCK_TOKENS, BYTES_PER_TOKEN) for length in requests_actual_lengths)
# => co-09: each request now reserves only up to ONE block's worth of slack, not the whole MAX_SEQ_LEN
contiguous_total = 4 * 2000 * BYTES_PER_TOKEN  # => Example 40's contiguous reservation, for comparison
print(paged_total, contiguous_total)  # => Output: 2144000 8000000

recovered = contiguous_total - paged_total  # => co-08/co-09: exactly what paging recovers vs contiguous
print(recovered)  # => Output: 5856000

assert paged_total < contiguous_total  # => co-08/co-09: paging strands FAR less memory
assert recovered > 5_000_000  # => co-09: almost all of Example 40's stranded capacity is recovered
# => Example 42 maps this same mechanism onto the OS virtual-memory vocabulary it's borrowed from
print("ex-41 OK")  # => a self-check marker confirming paging's memory-recovery advantage held
