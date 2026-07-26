"""Example 5: Measure Decode."""

DECODE_MS_PER_TOKEN = 20.0  # => co-03: decode emits ONE token per step -- a fixed per-step cost
# => unlike prefill (Example 4), decode cost is NOT front-loaded -- it accrues one step at a time


def simulate_decode_ms(output_tokens: int) -> float:  # => total decode time for a full generation
    return output_tokens * DECODE_MS_PER_TOKEN  # => co-03: total cost scales with output length


def simulate_per_token_ms(output_tokens: int) -> float:  # => the PER-TOKEN cost, not the total
    return simulate_decode_ms(output_tokens) / output_tokens  # => normalizing out length reveals the constant


ten_tokens_total = simulate_decode_ms(10)  # => a short generation
thousand_tokens_total = simulate_decode_ms(1000)  # => a generation 100x longer
print(ten_tokens_total, thousand_tokens_total)  # => Output: 200.0 20000.0 -- scales with length

per_token_short = simulate_per_token_ms(10)  # => same formula, normalized
per_token_long = simulate_per_token_ms(1000)  # => same formula, normalized, different length
print(per_token_short, per_token_long)  # => Output: 20.0 20.0 -- NEAR-CONSTANT per-token cost

assert per_token_short == per_token_long  # => co-03: per-token cost stays flat regardless of length
assert thousand_tokens_total == ten_tokens_total * 100  # => but TOTAL cost scales with token count
print("ex-05 OK")  # => a self-check marker confirming both assertions held
