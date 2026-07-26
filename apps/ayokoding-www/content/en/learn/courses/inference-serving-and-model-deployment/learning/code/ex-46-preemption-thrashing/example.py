"""Example 46: Preemption Thrashing."""


def simulate_aggressive_preemption(rounds: int, tokens_per_round: int) -> int:
    # => co-14: preempts EVERY round -- progress is discarded before it ever completes
    total_wasted = 0  # => accumulates progress lost to preemption, round after round
    for _ in range(rounds):  # => this loop NEVER lets a request finish -- that IS the thrashing pattern
        total_wasted += tokens_per_round  # => co-14: all progress this round is thrown away, every time
    return total_wasted  # => zero real progress was ever kept -- pure thrashing


def simulate_fixed_min_progress_guard(rounds: int, tokens_per_round: int, min_progress_tokens: int) -> int:
    # => co-13: fix -- a request must reach a MINIMUM progress before it becomes preemptible again
    completed_tokens = 0  # => tracks progress toward the protection threshold
    wasted = 0  # => stays zero in this simplified model once the guard kicks in
    for _ in range(rounds):
        completed_tokens += tokens_per_round  # => the SAME per-round progress as the aggressive case
        if completed_tokens >= min_progress_tokens:  # => co-13: protected once it clears the guard
            break  # => once protected, it runs to completion in this simplified model -- no more waste
    return wasted  # => the guard means later rounds are never thrashed away


wasted_aggressive = simulate_aggressive_preemption(rounds=5, tokens_per_round=10)  # => the thrashing baseline
wasted_with_guard = simulate_fixed_min_progress_guard(rounds=5, tokens_per_round=10, min_progress_tokens=20)
# => same 5 rounds, same per-round progress -- ONLY the guard differs between the two simulations
print(wasted_aggressive, wasted_with_guard)  # => Output: 50 0
# => going from "thrashes forever" to "zero waste" needed nothing but a minimum-progress floor

assert wasted_aggressive == 50  # => co-14: every one of the 5 rounds' progress was fully discarded
assert wasted_with_guard < wasted_aggressive  # => co-13: the minimum-progress guard eliminates the thrash
# => real schedulers commonly call this a "grace period" or "protected window" for the same reason
print("ex-46 OK")  # => a self-check marker confirming the min-progress guard eliminated the thrashing
