"""Example 48: Batch Occupancy Over Time -- Static vs Continuous."""

from dataclasses import dataclass  # => stdlib only -- comparing occupancy needs no framework


@dataclass
class SimRequest:  # => one simulated in-flight generation
    id: str  # => a label, useful only for reading print output
    output_tokens: int  # => how many decode steps THIS request needs before it is done
    tokens_emitted: int = 0  # => mutated in place as the simulated batch runs

    @property
    def finished(self) -> bool:  # => co-12: the per-request check continuous batching relies on
        return self.tokens_emitted >= self.output_tokens  # => True once this request hits its own target


def static_occupancy(requests: list[SimRequest]) -> list[int]:  # => co-11: ALL admitted slots held all along
    max_tokens = max(r.output_tokens for r in requests)  # => the batch runs this long, no matter what
    return [len(requests)] * max_tokens  # => every step, every admitted slot stays occupied, finished or not


def continuous_occupancy(requests: list[SimRequest], slots: int) -> list[int]:  # => co-12: real occupancy per step
    pending = list(requests)  # => the admission queue
    active: list[SimRequest] = []  # => the currently-running slots -- capped by `slots`
    occupancy: list[int] = []  # => a step-by-step record of how many slots were TRULY in use
    while pending or active:  # => keep going until EVERYTHING has both arrived and finished
        while len(active) < slots and pending:  # => co-12: fill any FREE slot immediately
            active.append(pending.pop(0))  # => moves ONE request from pending to active, per free slot
        for r in active:  # => every active request takes exactly one step this tick
            r.tokens_emitted += 1  # => the ONLY mutation each active request undergoes per tick
        occupancy.append(len(active))  # => how full the batch REALLY was on this step
        active = [r for r in active if not r.finished]  # => co-12: retire finished requests IMMEDIATELY
    return occupancy  # => the true occupancy trace, for direct comparison against static_occupancy


requests_for_static = [SimRequest("a", 3), SimRequest("b", 10), SimRequest("c", 2)]  # => same batch as Example 29
requests_for_continuous = [SimRequest("a", 3), SimRequest("b", 10), SimRequest("c", 2)]  # => a FRESH copy of the same batch

static_occ = static_occupancy(requests_for_static)  # => the reported-full trace, static policy
continuous_occ = continuous_occupancy(requests_for_continuous, slots=3)  # => the true-demand trace, continuous policy
static_avg = sum(static_occ) / len(static_occ)  # => mean occupancy under the static policy
continuous_avg = sum(continuous_occ) / len(continuous_occ)  # => mean occupancy under the continuous policy
print(static_avg)  # => Output: 3.0 -- always "full," including idle slots
print(round(continuous_avg, 2))  # => Output: 1.5 -- HALF the reported occupancy, the SAME real workload
# => the static number is a comforting fiction; the continuous number is what actually happened

assert static_avg == 3.0  # => co-11: static batching NEVER reports below full
# => a monitoring dashboard reading "3.0 avg occupancy" here would be actively MISLEADING
assert continuous_avg < static_avg  # => co-12: continuous occupancy reflects REAL, lower demand
# => always measure occupancy against a policy that can actually report less-than-full
print("ex-48 OK")  # => a self-check marker confirming the reported-vs-real occupancy gap held
