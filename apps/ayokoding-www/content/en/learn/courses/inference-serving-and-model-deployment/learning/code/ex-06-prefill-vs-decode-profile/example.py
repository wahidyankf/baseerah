"""Example 6: Prefill vs Decode Profile."""

PREFILL_MS_PER_TOKEN = 0.5  # => co-02: one compute-bound pass over the whole prompt
DECODE_MS_PER_TOKEN = 20.0  # => co-03: one memory-bandwidth-bound step per output token
# => same request, two DIFFERENT cost regimes -- neither number alone tells the whole story


def profile_request(prompt_tokens: int, output_tokens: int) -> dict[str, float]:
    prefill_ms = prompt_tokens * PREFILL_MS_PER_TOKEN  # => co-02: scales with INPUT length
    decode_ms = output_tokens * DECODE_MS_PER_TOKEN  # => co-03: scales with OUTPUT length
    return {"prefill_ms": prefill_ms, "decode_ms": decode_ms, "total_ms": prefill_ms + decode_ms}
    # => total is a simple sum, but the two summands have very different sensitivities


profile = profile_request(prompt_tokens=200, output_tokens=50)  # => a 200-token prompt, 50-token reply
print(profile["prefill_ms"], profile["decode_ms"])  # => Output: 100.0 1000.0 -- decode dominates here

assert profile["prefill_ms"] == 100.0  # => 200 tokens * 0.5 ms/token -- a SINGLE parallel pass
assert profile["decode_ms"] == 1000.0  # => 50 tokens * 20 ms/token -- 50 SEQUENTIAL steps
assert profile["decode_ms"] > profile["prefill_ms"]  # => co-02/co-03: the two phases behave differently
# => this same split -- one parallel pass, then many sequential steps -- recurs throughout this topic
print("ex-06 OK")  # => a self-check marker confirming all three assertions held
