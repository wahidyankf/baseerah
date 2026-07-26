"""Example 39: Chunked Prefill."""


def naive_stall_ms(prefill_tokens: int, ms_per_token: float) -> float:  # => co-17: the WHOLE prefill blocks decode
    return prefill_tokens * ms_per_token  # => one giant stall, sized by the ENTIRE prompt


def chunked_max_stall_ms(ms_per_token: float, chunk_tokens: int) -> float:
    # => co-13: split the prefill into chunks; a decode step can run BETWEEN chunks, not just after all of it
    return chunk_tokens * ms_per_token  # => the WORST single stall is now just one chunk's cost


prefill_tokens = 2000  # => a long incoming prompt
ms_per_token = 0.5  # => co-02: the same prefill-cost-per-token constant used throughout this topic
naive = naive_stall_ms(prefill_tokens, ms_per_token)  # => the unchunked, worst-case stall
chunked = chunked_max_stall_ms(ms_per_token, chunk_tokens=200)  # => the SAME prompt, chunked into pieces
print(naive, chunked)  # => Output: 1000.0 100.0

assert chunked < naive  # => co-17: chunking sharply cuts the WORST-CASE stall any in-flight decode sees
assert chunked == naive / 10  # => co-13: splitting into 10 chunks cuts worst-case stall by exactly 10x
# => this is the co-17 fix Example 38 motivated -- interleave prefill work instead of blocking on it
print("ex-39 OK")  # => a self-check marker confirming the chunking-cuts-worst-case-stall relationship held
