"""Example 30: Static Batching -- Idle Waste."""

from dataclasses import dataclass  # => stdlib only -- measuring waste needs no framework


@dataclass
class SimRequest:  # => one simulated in-flight generation
    id: str  # => a label, useful only for reading print output
    output_tokens: int  # => how many decode steps THIS request needs before it is done


def idle_slot_steps(requests: list[SimRequest]) -> int:  # => co-11: counts wasted (idle) slot-steps
    max_tokens = max(r.output_tokens for r in requests)  # => the batch runs this long, no matter what
    total_idle = 0  # => accumulates every step a slot sat occupied but did no useful work
    for r in requests:  # => a request idles for every step AFTER it finished, until the batch ends
        total_idle += max_tokens - r.output_tokens  # => the GAP between this request and the slowest one
    return total_idle  # => summed idle steps across the whole batch


def useful_slot_steps(requests: list[SimRequest]) -> int:  # => steps that actually emitted a token
    return sum(r.output_tokens for r in requests)  # => the useful-work counterpart to idle_slot_steps above


batch = [SimRequest("a", 3), SimRequest("b", 10), SimRequest("c", 2)]  # => the same batch as Example 29
idle = idle_slot_steps(batch)  # => wasted slot-steps across the whole batch
useful = useful_slot_steps(batch)  # => useful slot-steps across the whole batch
utilization = useful / (idle + useful)  # => the fraction of ALL slot-steps that did real work
# => idle + useful together account for EVERY slot-step the batch ever occupied
print(idle, useful)  # => Output: 15 15
print(round(utilization, 2))  # => Output: 0.5 -- exactly half the batch's slot-steps did nothing useful

assert idle == (10 - 3) + (10 - 10) + (10 - 2)  # => co-11: exactly the "finished early" gap, summed
assert utilization == 0.5  # => HALF of every slot-step in this batch was wasted, doing nothing
# => co-12 previews the fix this waste motivates: retire and refill slots continuously instead
print("ex-30 OK")  # => a self-check marker confirming the idle/useful split matched the arithmetic
