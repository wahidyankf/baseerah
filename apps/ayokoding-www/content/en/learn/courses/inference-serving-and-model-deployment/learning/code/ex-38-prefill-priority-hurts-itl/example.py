"""Example 38: Prefill Priority Hurts ITL."""


def simulate_stall(prefill_priority: bool, new_prefill_ms: float) -> float:
    # => co-17: if prefill is prioritized, in-flight decode steps must WAIT for it to finish first
    if prefill_priority:
        return new_prefill_ms  # => co-16: every in-flight decode step's ITL grows by the FULL prefill time
    return 0.0  # => decode is never interrupted -- prefill waits its turn instead


new_prefill_ms = 150.0  # => a moderately long incoming prompt's prefill cost
# => the SAME new_prefill_ms feeds both calls below -- only the priority FLAG differs
itl_stall_with_priority = simulate_stall(prefill_priority=True, new_prefill_ms=new_prefill_ms)  # => the tradeoff cost
itl_stall_without_priority = simulate_stall(prefill_priority=False, new_prefill_ms=new_prefill_ms)  # => the baseline
print(itl_stall_with_priority, itl_stall_without_priority)  # => Output: 150.0 0.0

new_request_ttft_with_priority = new_prefill_ms  # => co-16: the NEW request's TTFT improves -- it ran FIRST
new_request_ttft_without_priority = new_prefill_ms + 3 * 20.0  # => co-16: it waits behind 3 in-flight decode steps
print(new_request_ttft_with_priority, new_request_ttft_without_priority)  # => Output: 150.0 210.0

assert itl_stall_with_priority > itl_stall_without_priority  # => co-17: existing users' ITL got WORSE
# => this is the exact co-17 tension: helping the NEW request always costs existing users something
assert new_request_ttft_with_priority < new_request_ttft_without_priority  # => co-16: the new request's TTFT got BETTER
print("ex-38 OK")  # => a self-check marker confirming the co-17 ITL-vs-TTFT tradeoff held in both directions
