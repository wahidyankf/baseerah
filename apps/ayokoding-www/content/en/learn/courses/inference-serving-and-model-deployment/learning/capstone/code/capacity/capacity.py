"""Capstone step 4 -- capacity/: characterize the workload's prompt and output length distributions,
run a load test reproducing them, derive a capacity model, and write an autoscaling policy sized for
weight-load cold starts.

Reuses the distribution shape from Example 56, the capacity formula from Example 59, and the
proactive-autoscaling policy from Example 61.
"""

LENGTH_BUCKETS = [  # => co-21: a realistic, deterministic length distribution -- short-dominated, long-tailed
    (50, 40),
    (200, 35),
    (500, 15),
    (1500, 8),
    (4000, 2),
]


def expand_to_workload(buckets: list[tuple[int, int]]) -> list[int]:  # => co-21: materialize as an explicit list
    workload: list[int] = []
    for length, weight in buckets:
        workload.extend([length] * weight)
    return workload


def capacity_model(cache_budget_bytes: int, bytes_per_request_at_typical_length: int) -> int:
    # => co-07/co-21: the SAME division as Example 59, now driven by a typical (not worst-case) request size
    return cache_budget_bytes // bytes_per_request_at_typical_length


def proactive_scale_out_decision(queue_depth: int, threshold: int, cold_start_seconds: float, arrival_rate_per_sec: float) -> bool:
    # => co-23: the SAME proactive policy as Example 61 -- project queue growth across the cold-start window
    projected = queue_depth + arrival_rate_per_sec * cold_start_seconds
    return projected > threshold


def run_load_test(workload_lengths: list[int], max_batch_slots: int) -> int:
    # => co-22: a load test that reproduces the REAL length distribution, not a uniform-average shortcut
    pending = list(workload_lengths)
    active: list[int] = []
    steps = 0
    while pending or active:
        while len(active) < max_batch_slots and pending:
            active.append(pending.pop(0))
        active = [r - 1 for r in active]
        steps += 1
        active = [r for r in active if r > 0]
    return steps
