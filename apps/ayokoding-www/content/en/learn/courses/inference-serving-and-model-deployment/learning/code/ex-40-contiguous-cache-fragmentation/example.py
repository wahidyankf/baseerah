"""Example 40: Contiguous Cache Fragmentation."""


def contiguous_alloc_bytes(reserved_len: int, bytes_per_token: int) -> int:  # => co-08: RESERVED, not USED
    return reserved_len * bytes_per_token  # => a contiguous allocator must reserve the WORST-CASE length up front


BYTES_PER_TOKEN = 1000  # => an illustrative per-token cost, chosen to keep the arithmetic readable
MAX_SEQ_LEN = 2000  # => the longest length the allocator must be ready for, reserved for EVERY request

requests_actual_lengths = [50, 2000, 30, 40]  # => most requests are short; one genuinely needs the max
reserved_total = sum(contiguous_alloc_bytes(MAX_SEQ_LEN, BYTES_PER_TOKEN) for _ in requests_actual_lengths)
# => co-08: EVERY request reserves the full MAX_SEQ_LEN, regardless of its own actual length
used_total = sum(contiguous_alloc_bytes(length, BYTES_PER_TOKEN) for length in requests_actual_lengths)
# => the SAME formula, but fed each request's TRUE length instead of the worst case
stranded = reserved_total - used_total  # => co-08: reserved-but-never-touched bytes -- pure waste
print(reserved_total, used_total)  # => Output: 8000000 2120000
print(stranded)  # => Output: 5880000

utilization = used_total / reserved_total  # => the fraction of reserved cache that was actually used
assert utilization < 0.3  # => co-08: contiguous reservation strands the MAJORITY of allocated cache
assert stranded > used_total  # => more memory is WASTED than actually used, under this mixed workload
# => Example 41 fixes exactly this waste with paged, block-based allocation
print("ex-40 OK")  # => a self-check marker confirming the fragmentation-driven waste held
