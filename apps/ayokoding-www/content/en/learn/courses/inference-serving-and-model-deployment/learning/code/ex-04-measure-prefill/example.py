"""Example 4: Measure Prefill."""

PREFILL_MS_PER_TOKEN = 0.5  # => co-02: prefill is compute-bound and processes the WHOLE prompt at once
# => this one constant is why long prompts front-load latency BEFORE the first token ever appears


def simulate_prefill_ms(prompt_tokens: int) -> float:  # => a deterministic stand-in for a real timer
    return prompt_tokens * PREFILL_MS_PER_TOKEN  # => co-02: cost scales with prompt length


short_prompt_ms = simulate_prefill_ms(50)  # => a 50-token prompt
long_prompt_ms = simulate_prefill_ms(2000)  # => a 2000-token prompt, 40x longer
print(short_prompt_ms, long_prompt_ms)  # => Output: 25.0 1000.0 -- same formula, 40x the input

assert long_prompt_ms == short_prompt_ms * 40  # => co-02: cost scales LINEARLY with prompt length
assert long_prompt_ms > short_prompt_ms  # => longer prompt, more prefill work, no exceptions
print("ex-04 OK")  # => a self-check marker confirming both assertions held
