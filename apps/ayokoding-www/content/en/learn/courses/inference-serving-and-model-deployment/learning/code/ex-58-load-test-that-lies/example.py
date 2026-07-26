"""Example 58: Load Test That Lies."""


def simulate_steps(lengths: list[int], max_batch_slots: int) -> int:
    # => co-22: the same continuous-batching mechanics as Example 57, generalized to a plain list of lengths
    pending = list(lengths)  # => the admission queue
    active: list[int] = []  # => remaining tokens for each currently-active request
    steps = 0  # => the simulated wall-clock -- one tick per decode step, for the WHOLE batch
    while pending or active:  # => keep going until EVERYTHING has both arrived and finished
        while len(active) < max_batch_slots and pending:  # => co-12: fill any FREE slot immediately
            active.append(pending.pop(0))  # => moves ONE request from pending to active, per free slot
        active = [r - 1 for r in active]  # => every active request takes one step
        steps += 1  # => one wall-clock tick has elapsed, regardless of how many slots were full
        active = [r for r in active if r > 0]  # => co-12: retire finished requests IMMEDIATELY
    return steps  # => total wall-clock steps this particular length list actually took


real_lengths = [50, 50, 50, 50, 200, 200, 200, 500, 500]  # => co-21's realistic mixed workload, 9 requests
mean_length = round(sum(real_lengths) / len(real_lengths))  # => co-22: the WRONG shortcut -- "just use the average"
uniform_lengths = [mean_length] * len(real_lengths)  # => a load test that assumes every request costs the SAME
print(mean_length)  # => Output: 200

real_steps = simulate_steps(real_lengths, max_batch_slots=4)  # => the TRUE mixed-length workload's cost
uniform_steps = simulate_steps(uniform_lengths, max_batch_slots=4)  # => the "averaged" workload's cost
print(real_steps, uniform_steps)  # => Output: 750 600

assert real_steps != uniform_steps  # => co-22: the uniform-length load test measures a DIFFERENT workload than production
# => a load test built on the mean alone can pass in staging and still mislead about production capacity
print("ex-58 OK")  # => a self-check marker confirming the averaged load test disagreed with the real one
