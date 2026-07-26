"""Example 50: Cache Block Size Tradeoff."""

import math  # => stdlib only -- same ceil() trick as every paging example so far

BYTES_PER_TOKEN = 1000  # => same illustrative constant used throughout the paging examples
BOOKKEEPING_BYTES_PER_BLOCK = 64  # => co-09: each block needs its own table entry -- a small, fixed cost


def total_cost(actual_lengths: list[int], block_tokens: int) -> int:  # => co-09: waste PLUS bookkeeping
    total = 0  # => accumulates BOTH cost components across every request
    for length in actual_lengths:  # => processes each request's true length independently
        blocks = math.ceil(length / block_tokens)  # => co-09: rounds UP -- smaller blocks round up LESS
        data_bytes = blocks * block_tokens * BYTES_PER_TOKEN  # => the rounded-up allocation, in bytes
        bookkeeping_bytes = blocks * BOOKKEEPING_BYTES_PER_BLOCK  # => co-09: MORE blocks, MORE bookkeeping
        total += data_bytes + bookkeeping_bytes  # => both costs move in OPPOSITE directions as block size changes
    return total  # => the true total cost -- data waste AND bookkeeping overhead combined


lengths = [50, 2000, 30, 40]  # => the same mixed workload as Examples 40-41
small_blocks_cost = total_cost(lengths, block_tokens=16)  # => less rounding waste, more bookkeeping entries
large_blocks_cost = total_cost(lengths, block_tokens=256)  # => more rounding waste, fewer bookkeeping entries
print(small_blocks_cost, large_blocks_cost)  # => Output: 2152576 2816704

assert small_blocks_cost != large_blocks_cost  # => co-09: block size is a genuine tuning knob, not free
# => vLLM/TGI ship a default block size precisely because this tradeoff has no universal optimum
print("ex-50 OK")  # => a self-check marker confirming block size measurably changes total cost
