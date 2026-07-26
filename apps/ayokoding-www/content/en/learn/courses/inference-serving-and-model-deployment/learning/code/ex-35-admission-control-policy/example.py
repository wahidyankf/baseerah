"""Example 35: Admission Control Policy."""

from dataclasses import dataclass, field  # => stdlib only -- an admission gate needs no framework

# => "admission control" means EVERY arrival passes through one gate, admitted or queued, never dropped


@dataclass
class AdmissionQueue:  # => co-13: decides who gets to join the active batch next
    cache_budget_bytes: int  # => the hard ceiling this gate enforces
    bytes_per_request: int  # => assumed uniform cost per request, for simplicity
    active: list[str] = field(default_factory=list[str])  # => explicit generic keeps type-checking strict
    queued: list[str] = field(default_factory=list[str])  # => FIFO order preserved -- nobody jumps the line

    def submit(self, request_id: str) -> None:  # => co-13: every arrival goes through this ONE gate
        if len(self.active) * self.bytes_per_request < self.cache_budget_bytes:  # => room right now?
            self.active.append(request_id)  # => admit immediately
        else:  # => the budget check above failed -- there is genuinely no room
            self.queued.append(request_id)  # => co-13: wait for a slot to free up, not rejected outright


queue = AdmissionQueue(cache_budget_bytes=3 * 1000, bytes_per_request=1000)  # => room for exactly 3
for rid in ["r1", "r2", "r3", "r4", "r5"]:  # => 5 arrivals, only 3 seats
    queue.submit(rid)  # => the SAME gate decides every single arrival's fate
print(queue.active)  # => Output: ['r1', 'r2', 'r3']
print(queue.queued)  # => Output: ['r4', 'r5']
# => nobody was rejected outright -- r4 and r5 are waiting, not dropped

assert len(queue.active) == 3  # => co-07: exactly as many as the cache budget allows
assert queue.queued == ["r4", "r5"]  # => co-13: predictable FIFO order -- neither is silently dropped
print("ex-35 OK")  # => a self-check marker confirming the admit/queue split matched the budget exactly
