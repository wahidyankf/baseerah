"""Example 13: Concurrency Limited by Cache."""

CACHE_BUDGET_BYTES = 2_000_000_000  # => co-18: what's LEFT after weights, for cache -- the whole budget here
BYTES_PER_REQUEST = 400_000_000  # => co-06: one request's steady-state cache footprint
# => same shape as Example 11's formula, collapsed to a single per-request constant


class AdmissionSimulator:  # => co-07: the cache is what actually gates admission, not compute
    def __init__(self, budget_bytes: int, bytes_per_request: int) -> None:  # => wires the two constants in
        self.budget_bytes = budget_bytes  # => the hard ceiling -- never grows during a run
        self.bytes_per_request = bytes_per_request  # => assumed uniform across requests, for simplicity
        self.admitted: int = 0  # => how many requests are CURRENTLY holding cache

    def try_admit(self) -> bool:  # => co-07: admission succeeds only while cache budget allows it
        if (self.admitted + 1) * self.bytes_per_request > self.budget_bytes:  # => would this exceed budget?
            return False  # => refused -- not enough cache left, regardless of spare compute
        self.admitted += 1  # => only increments on the SUCCESS path -- refusals leave state untouched
        return True  # => accepted -- the budget check above already proved this fits


sim = AdmissionSimulator(CACHE_BUDGET_BYTES, BYTES_PER_REQUEST)  # => one simulator, one fixed budget
results = [sim.try_admit() for _ in range(7)]  # => try to admit 7 requests in a row
print(results)  # => Output: [True, True, True, True, True, False, False]
print(sim.admitted)  # => Output: 5

assert sim.admitted == 5  # => co-18: 2_000_000_000 // 400_000_000 == 5 -- the arithmetic sets the ceiling
assert results[5] is False  # => co-07: the 6th request is refused -- cache, not compute, is the gate
print("ex-13 OK")  # => a self-check marker confirming the admission ceiling held exactly at 5
