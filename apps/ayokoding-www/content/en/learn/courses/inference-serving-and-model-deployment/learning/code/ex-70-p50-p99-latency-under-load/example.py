"""Example 70: P50/P99 Latency Under Load."""


def itl_at_batch(batch_size: int) -> float:  # => co-15/co-16: the SAME cost model as Example 33
    return 15.0 + 1.0 * batch_size  # => the SAME fixed-floor-plus-per-slot formula, reused here


def simulate_latencies_under_load(batch_sizes_over_time: list[int]) -> list[float]:
    # => co-22: latency observed by requests DEPENDS on how full the batch was WHEN they were served
    return [itl_at_batch(b) for b in batch_sizes_over_time]  # => one latency sample per observed batch size


# => co-22: a realistic trace -- batch size fluctuates with arrivals, occasionally spiking
batch_trace = [4, 4, 5, 4, 32, 4, 4, 5, 4, 4]  # => one spike to batch=32 among mostly-light load
latencies = simulate_latencies_under_load(batch_trace)  # => co-22: the resulting per-request latency samples
sorted_latencies = sorted(latencies)  # => sorting is required to read off percentiles by index
p50 = sorted_latencies[len(sorted_latencies) // 2]  # => the median observed latency
p99_index = min(len(sorted_latencies) - 1, int(len(sorted_latencies) * 0.99))  # => clamped so it never overflows
p99 = sorted_latencies[p99_index]  # => co-22: the tail -- almost the WORST observed latency
print(sorted_latencies)  # => Output: [19.0, 19.0, 19.0, 19.0, 19.0, 19.0, 19.0, 20.0, 20.0, 47.0]
print(p50, p99)  # => Output: 19.0 47.0

assert p99 > p50 * 1.5  # => co-22: the tail is dominated by the rare spike -- far above the typical case
print("ex-70 OK")  # => a self-check marker confirming the tail-dominated-by-spike relationship held
