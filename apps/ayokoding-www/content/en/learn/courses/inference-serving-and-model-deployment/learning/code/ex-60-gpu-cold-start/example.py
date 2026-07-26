"""Example 60: GPU Cold Start."""


def weights_load_time_seconds(model_size_bytes: int, storage_bandwidth_bytes_per_sec: float) -> float:
    # => co-23: loading multi-GB weights from storage into GPU memory dominates a GPU service's cold start
    return model_size_bytes / storage_bandwidth_bytes_per_sec  # => bytes divided by bandwidth -- pure I/O time


model_size_bytes = 14_000_000_000  # => a 7B-parameter model stored in fp16
storage_bandwidth = 2_000_000_000  # => 2 GB/s, an illustrative, reasonably fast network-attached storage read
cold_start_seconds = weights_load_time_seconds(model_size_bytes, storage_bandwidth)  # => co-23: the I/O floor alone
print(round(cold_start_seconds, 1))  # => Output: 7.0

typical_cpu_service_cold_start_seconds = 2.0  # => co-23: a stateless CPU service is usually READY within seconds
ratio = cold_start_seconds / typical_cpu_service_cold_start_seconds  # => how many times slower the GPU service is
print(round(ratio, 2))  # => Output: 3.5

assert cold_start_seconds > typical_cpu_service_cold_start_seconds * 2  # => co-23: GPU cold start is FAR slower, not comparable
# => autoscaling policies (Example 71) that assume CPU-service cold-start speed will misbehave on GPU fleets
print("ex-60 OK")  # => a self-check marker confirming GPU cold start dwarfed the CPU-service baseline
