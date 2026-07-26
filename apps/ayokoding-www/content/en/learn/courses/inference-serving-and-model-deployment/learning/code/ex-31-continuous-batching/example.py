"""Example 31: Continuous Batching."""

from dataclasses import dataclass  # => stdlib only -- continuous batching needs no framework


@dataclass
class SimRequest:  # => one simulated in-flight generation
    id: str  # => a label, useful only for reading print output
    output_tokens: int  # => how many decode steps THIS request needs before it is done
    tokens_emitted: int = 0  # => mutated in place as the simulated batch runs

    @property
    def finished(self) -> bool:  # => co-12: the per-request check that drives immediate retirement
        return self.tokens_emitted >= self.output_tokens  # => True the instant a request hits its own target


def run_continuous_batch(requests: list[SimRequest], max_batch_slots: int) -> list[int]:
    # => co-12: admits/retires at TOKEN granularity, not once per whole batch
    # => contrast Example 29's run_static_batch(): that one checks membership ONCE, at the start
    pending = list(requests)  # => the admission queue
    active: list[SimRequest] = []  # => the currently-running slots -- capped by max_batch_slots
    occupancy_per_step: list[int] = []  # => tracked purely for the observation this example makes
    while pending or active:  # => keep going until EVERYTHING has both arrived and finished
        while len(active) < max_batch_slots and pending:  # => co-12: fill any FREE slot immediately
            active.append(pending.pop(0))  # => moves ONE request from pending to active, per free slot
        for r in active:  # => every active request takes exactly one step this tick
            r.tokens_emitted += 1  # => the ONLY mutation each active request undergoes per tick
        occupancy_per_step.append(len(active))  # => how full the batch was on THIS step
        active = [r for r in active if not r.finished]  # => co-12: retire finished requests IMMEDIATELY
    return occupancy_per_step  # => a step-by-step occupancy trace, for inspecting fill behavior


requests = [SimRequest("a", 3), SimRequest("b", 10), SimRequest("c", 2), SimRequest("d", 4)]  # => 4 mixed lengths
occupancy = run_continuous_batch(requests, max_batch_slots=2)  # => only 2 slots -- forces queueing
# => contrast with Example 29's static batch: here, "d" gets a slot the MOMENT one frees up, not later
print(occupancy)  # => Output: [2, 2, 2, 2, 2, 2, 2, 2, 2, 1]

assert min(occupancy[:-1]) == 2  # => co-12: the batch stays FULL right up until requests genuinely run out
# => only the FINAL step dips below full -- every earlier gap was refilled immediately
assert len(occupancy) == 10  # => bounded by the longest request ("b", 10 tokens) -- same as Example 29
print("ex-31 OK")  # => a self-check marker confirming the batch stayed full until requests ran out
