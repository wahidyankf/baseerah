"""Example 21: Single-Request Timeline."""

PREFILL_MS = 100.0  # => co-02: one compute-bound pass, modeled here as a fixed-cost block
DECODE_STEP_MS = 20.0  # => co-03: one memory-bandwidth-bound step per emitted token
# => the timeline below is what these two phases look like laid end-to-end, in wall-clock order


def build_timeline(output_tokens: int) -> list[tuple[str, float]]:  # => co-02/co-03/co-16: one request's timeline
    events: list[tuple[str, float]] = [("prefill_start", 0.0)]  # => the clock starts here, at t=0
    clock = PREFILL_MS  # => prefill happens as ONE block, so the clock jumps straight past it
    events.append(("first_token", clock))  # => co-16: this IS the time-to-first-token instant
    for i in range(1, output_tokens):  # => the remaining decode steps, one at a time
        clock += DECODE_STEP_MS  # => each step advances the clock by exactly one fixed increment
        events.append((f"token_{i + 1}", clock))  # => co-16: the gap between consecutive events IS the ITL
    return events  # => a full ordered event log, not just a single summary number


timeline = build_timeline(output_tokens=4)  # => 4 output tokens: 1 prefill event, 4 token events
print(timeline)
# => Output: [('prefill_start', 0.0), ('first_token', 100.0), ('token_2', 120.0), ('token_3', 140.0), ('token_4', 160.0)]

ttft = timeline[1][1]  # => co-16: reading TTFT straight off the timeline -- the SECOND event's timestamp
total_ms = timeline[-1][1]  # => the LAST event's timestamp is the request's total wall-clock time
print(ttft, total_ms)  # => Output: 100.0 160.0

assert ttft == PREFILL_MS  # => co-16: TTFT lands exactly at the end of the prefill phase
assert total_ms == PREFILL_MS + 3 * DECODE_STEP_MS  # => co-03: 3 MORE steps after the first token
print("ex-21 OK")  # => a self-check marker confirming both timeline-derived assertions held
