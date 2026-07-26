"""Example 20: Context Window and the Cache Ceiling."""

BYTES_PER_TOKEN = 98304  # => co-06: same per-token cache cost used in Example 12
# => this example runs the SAME formula in reverse: bytes available -> tokens affordable


def max_context_length(cache_budget_bytes: int) -> int:  # => co-06: inverts the cache formula
    return cache_budget_bytes // BYTES_PER_TOKEN  # => the LONGEST single sequence the budget can hold


small_budget = 20_000 * BYTES_PER_TOKEN  # => sized to yield an exact 20,000-token ceiling (~1.83 GiB)
large_budget = 80_000 * BYTES_PER_TOKEN  # => 4x the budget (~7.32 GiB)

small_ceiling = max_context_length(small_budget)  # => same inversion, smaller budget
large_ceiling = max_context_length(large_budget)  # => same inversion, larger budget
print(small_ceiling, large_ceiling)  # => Output: 20000 80000

assert large_ceiling == small_ceiling * 4  # => co-06: 4x the cache budget buys 4x the context ceiling
assert small_ceiling < 32_000  # => co-06: even this budget does not stretch to a 32K-token context
print("ex-20 OK")  # => a self-check marker confirming the budget-to-context-length inversion held
