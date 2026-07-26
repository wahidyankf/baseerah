"""Example 32: Throughput Gain -- Static vs Continuous."""

from dataclasses import dataclass  # => stdlib only -- comparing the two policies needs no framework


@dataclass
class SimRequest:  # => one simulated in-flight generation
    id: str  # => a label, useful only for reading print output
    output_tokens: int  # => how many decode steps THIS request needs before it is done


def static_batch_slot_steps(requests: list[SimRequest]) -> int:  # => co-11: slot-steps INCLUDING idle waste
    max_tokens = max(r.output_tokens for r in requests)  # => static batching pays for the SLOWEST member
    return max_tokens * len(requests)  # => every slot occupied for the WHOLE batch duration


def continuous_batch_slot_steps(requests: list[SimRequest]) -> int:  # => co-12: slot-steps actually spent
    return sum(r.output_tokens for r in requests)  # => a slot is freed the instant its request finishes


requests = [SimRequest("a", 3), SimRequest("b", 10), SimRequest("c", 2), SimRequest("d", 4)]  # => same batch as Ex 31
static_steps = static_batch_slot_steps(requests)  # => slot-steps IF this batch ran statically
continuous_steps = continuous_batch_slot_steps(requests)  # => slot-steps the SAME batch actually needs
useful_tokens = sum(r.output_tokens for r in requests)  # => the SAME real work, either way
print(static_steps, continuous_steps)  # => Output: 40 19
# => same 19 real tokens of work either way -- static just SPENDS more slot-steps to deliver them

static_throughput = useful_tokens / static_steps  # => useful work divided by slot-steps SPENT (static)
continuous_throughput = useful_tokens / continuous_steps  # => same ratio, continuous policy's denominator
print(round(static_throughput, 2), round(continuous_throughput, 2))  # => Output: 0.47 1.0

assert continuous_steps < static_steps  # => co-12: fewer WASTED slot-steps for the identical real work
# => this gap IS Example 30's stranded-capacity number, expressed as a throughput multiplier instead
assert continuous_throughput > static_throughput  # => co-12: measurably higher effective throughput
print("ex-32 OK")  # => a self-check marker confirming continuous batching's throughput edge held
