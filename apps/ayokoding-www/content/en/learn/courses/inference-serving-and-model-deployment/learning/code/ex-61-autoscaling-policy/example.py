"""Example 61: Autoscaling Policy."""


def naive_scale_out_decision(queue_depth: int, threshold: int) -> bool:
    # => co-23: naive CPU-style policy -- scale out only AFTER the queue is already deep
    return queue_depth > threshold  # => a purely reactive check -- no notion of how long relief will take


def proactive_scale_out_decision(queue_depth: int, threshold: int, cold_start_seconds: float, arrival_rate_per_sec: float) -> bool:
    # => co-23: GPU-aware policy -- scale out EARLY enough the new replica is ready before the queue overflows
    projected_queue_depth_after_cold_start = queue_depth + arrival_rate_per_sec * cold_start_seconds
    # => co-23: projects forward by exactly the cold-start delay -- the reactive check's blind spot
    return projected_queue_depth_after_cold_start > threshold  # => decides on the PROJECTED depth, not today's


queue_depth = 8  # => today's queue depth -- looks fine on its own
threshold = 20  # => the same threshold both policies are checked against
naive = naive_scale_out_decision(queue_depth, threshold)  # => only looks at right-now
proactive = proactive_scale_out_decision(queue_depth, threshold, cold_start_seconds=7.0, arrival_rate_per_sec=2.0)
# => co-23: same queue_depth, same threshold -- only the PROJECTION differs between the two calls
print(naive, proactive)  # => Output: False True
# => a naive autoscaler here would start scaling only AFTER the queue has already overflowed

assert naive is False  # => co-23: naive policy says "not yet" -- queue is under threshold RIGHT NOW
assert proactive is True  # => co-23: proactive policy already sees the cold start will let the queue overflow
print("ex-61 OK")  # => a self-check marker confirming the reactive-vs-proactive disagreement held
