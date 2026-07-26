"""Example 69: Interconnect Bottleneck Measurement. [GPU] illustrative reference numbers."""


def all_reduce_time_ms(traffic_bytes: int, interconnect_bandwidth_bytes_per_sec: float) -> float:
    # => co-20: converts Example 68's byte count into WALL-CLOCK time, given a link's real bandwidth
    return traffic_bytes / interconnect_bandwidth_bytes_per_sec * 1000  # => seconds converted to milliseconds


traffic_bytes = 384_000_000  # => co-20: tensor-parallel traffic per step, from Example 68
fast_interconnect = 300_000_000_000  # => 300 GB/s, illustrative -- e.g. a high-end multi-GPU link
slow_interconnect = 10_000_000_000  # => 10 GB/s, illustrative -- e.g. commodity networking between hosts

fast_time = all_reduce_time_ms(traffic_bytes, fast_interconnect)  # => co-20: the SAME traffic, fast link
slow_time = all_reduce_time_ms(traffic_bytes, slow_interconnect)  # => co-20: the SAME traffic, slow link
print(round(fast_time, 3), round(slow_time, 2))  # => Output: 1.28 38.4

decode_step_budget_ms = 20.0  # => co-15/co-16: this topic's own decode-step cost model, from Example 33
assert fast_time < decode_step_budget_ms  # => co-20: a fast interconnect adds a NEGLIGIBLE fraction of the step budget
assert slow_time > decode_step_budget_ms  # => co-20: a slow interconnect ALONE exceeds the entire step budget
# => choosing TP over PP without checking link bandwidth first can turn a compute win into a network loss
print("ex-69 OK")  # => a self-check marker confirming the interconnect-speed-vs-step-budget comparison held
