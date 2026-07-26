"""Example 34: The Throughput/Latency Frontier."""

BASE_STEP_MS = 15.0  # => co-16: same fixed floor cost as Example 33
MS_PER_EXTRA_SLOT = 1.0  # => co-15: same per-slot penalty as Example 33


def frontier_point(batch_size: int) -> tuple[float, float]:  # => co-15: (throughput, latency) pair
    itl_ms = BASE_STEP_MS + MS_PER_EXTRA_SLOT * batch_size  # => the SAME formula from Example 33
    throughput = batch_size * 1000.0 / itl_ms  # => aggregate tokens/sec across the whole batch
    return throughput, itl_ms  # => one point on the frontier, for one batch size


points = [frontier_point(b) for b in (1, 2, 4, 8, 16, 32, 64)]  # => co-15: the frontier, batch size by batch size
print([(round(t, 1), round(lat, 1)) for t, lat in points])
# => Output: [(62.5, 16.0), (117.6, 17.0), (210.5, 19.0), (347.8, 23.0), (516.1, 31.0), (680.9, 47.0), (810.1, 79.0)]

# => co-15: EVERY step along the frontier trades higher throughput for higher latency -- never both improve
regressions = [i for i in range(1, len(points)) if points[i][0] <= points[i - 1][0] or points[i][1] <= points[i - 1][1]]
# => a regression would be a batch size that is WORSE on one axis without gaining on the other
print(regressions)  # => Output: [] -- no such regression exists anywhere on this frontier

assert regressions == []  # => no configuration beats an earlier one on BOTH axes simultaneously
print("ex-34 OK")  # => a self-check marker confirming the frontier has no dominated points
