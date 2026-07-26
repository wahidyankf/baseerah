"""Example 43: Prefix Sharing."""

import math  # => stdlib only -- same ceil() trick as Example 41

BLOCK_TOKENS = 16  # => same fixed block size as Example 41 and 42


def blocks_needed(token_len: int) -> int:  # => co-09: the same rounding-up rule as Example 41
    return math.ceil(token_len / BLOCK_TOKENS)  # => blocks required to hold this many tokens


shared_system_prompt_tokens = 200  # => co-10: identical prefix across BOTH requests below
# => imagine a shared system prompt or few-shot examples -- the SAME text, sent to multiple requests
request_a_unique_tokens = 30  # => request A's own tokens, AFTER the shared prefix
request_b_unique_tokens = 45  # => request B's own tokens, AFTER the shared prefix

without_sharing = (
    2 * blocks_needed(shared_system_prompt_tokens)  # => the prefix, counted TWICE -- once per request
    + blocks_needed(request_a_unique_tokens)  # => A's own unique portion
    + blocks_needed(request_b_unique_tokens)  # => B's own unique portion
)  # => co-10: prefix stored TWICE, once per request
with_sharing = (
    blocks_needed(shared_system_prompt_tokens)  # => the prefix, counted ONCE -- referenced, not duplicated
    + blocks_needed(request_a_unique_tokens)  # => A's own unique portion, same as above
    + blocks_needed(request_b_unique_tokens)  # => B's own unique portion, same as above
)  # => co-09/co-10: prefix's blocks are REFERENCED, not duplicated
print(without_sharing, with_sharing)  # => Output: 31 18 -- the shared prefix's blocks counted once, not twice

blocks_saved = without_sharing - with_sharing  # => the direct payoff of sharing this one prefix
# => this saving grows with EVERY additional request that shares the same system prompt
print(blocks_saved)  # => Output: 13

assert with_sharing < without_sharing  # => co-10: sharing strictly reduces total blocks held
# => and it does so WITHOUT changing a single byte of the actual generated output
assert blocks_saved == blocks_needed(shared_system_prompt_tokens)  # => co-09: saves EXACTLY the shared prefix's blocks
print("ex-43 OK")  # => a self-check marker confirming the sharing savings matched the prefix's own block count
