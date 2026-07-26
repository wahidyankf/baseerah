"""Example 45: Preemption Under Pressure."""

from dataclasses import dataclass  # => stdlib only -- preemption bookkeeping needs no framework


@dataclass
class ActiveRequest:  # => one currently-running request, eligible for eviction under pressure
    id: str  # => a label, useful only for reading print output
    tokens_emitted: int  # => co-14: also THIS request's cache footprint -- more tokens, more cache held
    preempted_count: int = 0  # => how many times this request has already been evicted and restarted


def evict_for_new_high_priority(active: list[ActiveRequest], budget_slots: int) -> str:
    # => co-14: under pressure, evict the request holding the MOST cache to free the most room at once
    if len(active) < budget_slots:  # => there's already a free slot -- no eviction needed at all
        return "admitted_without_preemption"
    victim = max(active, key=lambda r: r.tokens_emitted)  # => co-14: the biggest cache holder is evicted
    victim.preempted_count += 1  # => records that this request was interrupted, for later inspection
    saved_progress = victim.tokens_emitted  # => co-14: this progress is LOST -- recomputed later, from scratch
    victim.tokens_emitted = 0  # => co-14: the eviction is total -- no partial credit for work already done
    return f"preempted {victim.id}, lost {saved_progress} tokens of progress"  # => a human-readable outcome


active = [  # => three requests at very different points in their own generation
    ActiveRequest("a", tokens_emitted=5),  # => barely started
    ActiveRequest("b", tokens_emitted=40),  # => the furthest along -- and therefore the biggest cache holder
    ActiveRequest("c", tokens_emitted=10),  # => partway through
]
result = evict_for_new_high_priority(active, budget_slots=3)  # => all 3 slots full -- forces an eviction
print(result)  # => Output: preempted b, lost 40 tokens of progress
# => the request FURTHEST along paid the price, precisely because it held the most cache

b = next(r for r in active if r.id == "b")  # => pulls "b" back out of the list to inspect its post-state
print(b.preempted_count, b.tokens_emitted)  # => Output: 1 0
# => the object was mutated IN PLACE -- "b" inside `active` now reflects the eviction too

assert b.preempted_count == 1  # => co-14: "b" was the victim -- it held the most cache
# => a real scheduler would re-admit "b" later and repeat this same generation from scratch
assert b.tokens_emitted == 0  # => co-14: its progress was discarded, not paused
# => Example 46 shows what happens when this eviction rule runs unchecked, every single round
print("ex-45 OK")  # => a self-check marker confirming the biggest-holder-gets-evicted policy held
