"""Example 29: Static Batching."""

from dataclasses import dataclass  # => stdlib only -- the batching model needs no framework

# => "static" means the batch membership is FIXED at start -- nobody joins or leaves mid-batch


@dataclass
class SimRequest:  # => one simulated in-flight generation
    id: str  # => a label, useful only for reading print output
    output_tokens: int  # => how many decode steps THIS request needs before it is done
    tokens_emitted: int = 0  # => mutated in place as the simulated batch runs


def run_static_batch(requests: list[SimRequest]) -> int:  # => co-11: returns steps until ALL finish
    step = 0  # => the simulated clock -- one tick per decode step, for the WHOLE batch
    max_tokens = max(r.output_tokens for r in requests)  # => the batch runs as long as its LONGEST member
    while step < max_tokens:  # => co-11: the loop cannot stop early, no matter who finished already
        step += 1  # => advances for the WHOLE batch, whether or not any given member still needs it
        for r in requests:  # => every member gets touched on every step, finished or not
            if r.tokens_emitted < r.output_tokens:  # => still generating -- takes a real step
                r.tokens_emitted += 1  # => one more token toward this request's own target
            # => a FINISHED request still occupies its batch slot -- co-11's core problem
    return step  # => total steps == the SLOWEST member's output length, never less


batch = [SimRequest("a", 3), SimRequest("b", 10), SimRequest("c", 2)]  # => three very different lengths
steps = run_static_batch(batch)  # => run the whole batch to completion
print(steps)  # => Output: 10
print([r.tokens_emitted for r in batch])  # => Output: [3, 10, 2]

assert steps == 10  # => co-11: the batch takes as long as its SLOWEST member, always
assert all(r.tokens_emitted == r.output_tokens for r in batch)  # => every request DID finish correctly
print("ex-29 OK")  # => a self-check marker confirming the slowest-member-sets-the-pace behavior held
