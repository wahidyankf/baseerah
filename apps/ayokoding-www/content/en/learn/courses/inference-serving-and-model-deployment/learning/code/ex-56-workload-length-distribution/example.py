"""Example 56: Workload Length Distribution."""

# => co-21: a realistic workload is a DISTRIBUTION of lengths, never one fixed number
LENGTH_BUCKETS = [  # => (output_tokens, relative weight) -- weight is a count, not a random draw
    (50, 40),  # => short replies dominate
    (200, 35),  # => the second-most-common bucket
    (500, 15),  # => a mid-length minority
    (1500, 8),  # => a genuinely long minority
    (4000, 2),  # => rare, very long generations -- the tail that matters most for capacity
]


def expand_to_workload(buckets: list[tuple[int, int]]) -> list[int]:
    # => co-21: materializes the distribution as an explicit, deterministic list -- no random sampling anywhere
    workload: list[int] = []  # => accumulates one entry per simulated request
    for length, weight in buckets:  # => processes every bucket in the fixed table above
        workload.extend([length] * weight)  # => `weight` copies of this length, deterministically
    return workload  # => a flat list -- 100 request lengths, drawn from a fixed, known distribution


workload = expand_to_workload(LENGTH_BUCKETS)  # => co-21: the whole distribution, expanded into one list
print(len(workload))  # => Output: 100 -- 100 requests total across all buckets

mean_length = sum(workload) / len(workload)  # => the single-number summary this example will show is misleading
print(round(mean_length, 1))  # => Output: 365.0

sorted_workload = sorted(workload)  # => sorting is required to read off percentiles by index
p50 = sorted_workload[49]  # => the median request length
p99 = sorted_workload[98]  # => co-21: the tail -- almost the WORST request in this sample
print(p50, p99)  # => Output: 200 4000

assert p99 > mean_length  # => co-21: the tail sits FAR above the mean -- a single "average length" hides this
# => capacity planning off the mean alone under-provisions for exactly the requests that hurt most
print("ex-56 OK")  # => a self-check marker confirming the tail-above-mean relationship held
