"""Example 44: Prefix Sharing Limits."""

import math  # => stdlib only -- same ceil() trick as Examples 41 and 43

BLOCK_TOKENS = 16  # => same fixed block size as every paging example so far


def blocks_needed(token_len: int) -> int:  # => co-09: the same rounding-up rule as Example 41
    return math.ceil(token_len / BLOCK_TOKENS)  # => blocks required to hold this many tokens


def shared_prefix_length(prompt_a: list[int], prompt_b: list[int]) -> int:  # => co-10: length of the SHARED prefix
    shared = 0  # => counts matching tokens from the START of both prompts
    for a, b in zip(prompt_a, prompt_b):  # => walks both prompts token-by-token, in lockstep
        if a != b:  # => co-10: sharing stops at the FIRST divergence -- no partial-token sharing
            break  # => stops counting immediately -- no credit for matches AFTER the first mismatch
        shared += 1  # => one more token confirmed identical in both prompts
    return shared  # => the length of the longest common PREFIX, not just any common tokens


prompt_a = [1, 2, 3, 4, 5]  # => a request-specific prompt with no common system prompt
# => two prompts that share NOTHING at the start -- the adversarial case for prefix sharing
prompt_b = [9, 8, 7, 6, 5]  # => an entirely different prompt -- diverges on token 0

shared_len = shared_prefix_length(prompt_a, prompt_b)  # => co-10: how much these two ACTUALLY share
print(shared_len)  # => Output: 0
# => co-10's limit made concrete: sharing only helps when a REAL common prefix exists

with_sharing_blocks = blocks_needed(shared_len) + blocks_needed(len(prompt_a) - shared_len) + blocks_needed(len(prompt_b) - shared_len)  # => shared blocks (zero here) plus each request's own remainder
without_sharing_blocks = blocks_needed(len(prompt_a)) + blocks_needed(len(prompt_b))  # => no sharing attempted at all
print(with_sharing_blocks, without_sharing_blocks)  # => Output: 2 2

assert shared_len == 0  # => co-10: no shared prefix exists between these two prompts
# => contrast with Example 43, where a genuinely shared prefix DID pay off
assert with_sharing_blocks == without_sharing_blocks  # => co-10: sharing buys NOTHING when there's nothing to share
print("ex-44 OK")  # => a self-check marker confirming sharing degrades gracefully to "no benefit," not a crash
