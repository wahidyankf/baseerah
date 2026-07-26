"""Example 47: Request Queue Wait Time."""


def simulate_queue_wait(arrival_times: list[float], admit_rate_per_sec: float) -> list[float]:
    # => co-13: requests wait if they arrive faster than the server can ADMIT them
    wait_times: list[float] = []  # => one entry per request, in arrival order
    next_free_slot_time = 0.0  # => the earliest moment the server can admit ANOTHER request
    for arrival in arrival_times:  # => processes each arrival in the order it actually showed up
        admit_time = max(arrival, next_free_slot_time)  # => co-13: can't be admitted before a slot is free
        wait_times.append(admit_time - arrival)  # => co-16: the QUEUEING component of total latency
        next_free_slot_time = admit_time + (1.0 / admit_rate_per_sec)  # => the next slot frees up after this
    return wait_times  # => the full per-request wait-time trace, for inspecting the burst's shape


arrivals = [0.0, 0.1, 0.2, 0.3, 0.4]  # => 5 requests arriving in a fast burst
waits = simulate_queue_wait(arrivals, admit_rate_per_sec=2.0)  # => server can only admit 2/sec
print([round(w, 2) for w in waits])  # => Output: [0.0, 0.4, 0.8, 1.2, 1.6]

assert waits[0] == 0.0  # => co-13: the FIRST request in a burst never waits
assert waits[-1] > waits[0]  # => co-16: queueing delay accumulates for requests later in the burst
# => this queueing component is invisible in per-request prefill/decode timing alone
print("ex-47 OK")  # => a self-check marker confirming queueing delay accumulated across the burst
