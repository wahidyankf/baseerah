"""Example 57: Realistic Load Test."""

from dataclasses import dataclass  # => stdlib only -- driving the engine with a real mix needs no framework


@dataclass
class SimRequest:  # => one simulated in-flight generation
    id: str  # => a label, useful only for reading print output
    output_tokens: int  # => how many decode steps THIS request needs before it is done
    tokens_emitted: int = 0  # => mutated in place as the simulated batch runs

    @property
    def finished(self) -> bool:  # => co-12: the per-request check continuous batching relies on
        return self.tokens_emitted >= self.output_tokens  # => True once this request hits its own target


def run_continuous_batch(requests: list[SimRequest], max_batch_slots: int) -> int:
    # => co-12/co-22: the SAME continuous-batching engine from Example 31, now driven by a real length mix
    pending = list(requests)  # => the admission queue
    active: list[SimRequest] = []  # => the currently-running slots -- capped by max_batch_slots
    steps = 0  # => the simulated wall-clock -- one tick per decode step, for the WHOLE batch
    while pending or active:  # => keep going until EVERYTHING has both arrived and finished
        while len(active) < max_batch_slots and pending:  # => co-12: fill any FREE slot immediately
            active.append(pending.pop(0))  # => moves ONE request from pending to active, per free slot
        for r in active:  # => every active request takes exactly one step this tick
            r.tokens_emitted += 1  # => the ONLY mutation each active request undergoes per tick
        steps += 1  # => one wall-clock tick has elapsed, regardless of how many slots were full
        active = [r for r in active if not r.finished]  # => co-12: retire finished requests IMMEDIATELY
    return steps  # => total wall-clock steps this realistic-mix workload actually took


LENGTH_BUCKETS = [(50, 4), (200, 3), (500, 2)]  # => co-21: a small, deterministic stand-in for a real length mix


def expand_to_workload(buckets: list[tuple[int, int]]) -> list[int]:
    # => co-21: the SAME expansion logic as Example 56, applied to a different bucket table
    out: list[int] = []  # => accumulates one entry per simulated request
    for length, weight in buckets:  # => processes every bucket in the fixed table above
        out.extend([length] * weight)  # => `weight` copies of this length, deterministically
    return out  # => a flat list -- one length per simulated request


lengths = expand_to_workload(LENGTH_BUCKETS)  # => co-21: 9 requests, mixed lengths -- not one uniform number
requests = [SimRequest(f"r{i}", length) for i, length in enumerate(lengths)]  # => wraps each length as a SimRequest
total_tokens = sum(lengths)  # => the total real work this load test represents
steps = run_continuous_batch(requests, max_batch_slots=4)  # => the SAME engine, now under a realistic length mix
print(total_tokens, steps)  # => Output: 1800 750
# => 1800 total tokens took 750 wall-clock steps -- co-22: NOT one step per token, batching overlaps work

throughput = total_tokens / steps  # => co-22: real work delivered per wall-clock step, under this mix
print(round(throughput, 2))  # => Output: 2.4

assert steps < total_tokens  # => co-22: WALL-CLOCK steps are far fewer than total tokens -- batching is working
# => this IS the payoff of Example 31's continuous batching, now measured on a realistic length mix
# => Example 58 shows why testing with a SINGLE averaged length instead of this mix hides real behavior
print("ex-57 OK")  # => a self-check marker confirming the engine handled a realistic length mix correctly
