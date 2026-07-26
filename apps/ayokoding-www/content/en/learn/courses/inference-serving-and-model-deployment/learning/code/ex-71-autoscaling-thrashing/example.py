"""Example 71: Autoscaling Thrashing."""


def naive_threshold_decision(queue_depth: int, threshold: int) -> str:
    # => co-23: a SINGLE threshold with no gap flips direction the instant queue depth crosses it, either way
    if queue_depth > threshold:  # => any tick above threshold triggers an immediate scale-out
        return "scale_out"
    if queue_depth < threshold:  # => any tick below threshold triggers an immediate scale-in
        return "scale_in"
    return "hold"  # => exactly at threshold -- the only tick that does nothing


def hysteresis_decision(queue_depth: int, threshold: int, cooldown_active: bool) -> str:
    # => co-23: fix -- ANY scaling action arms a cooldown that suppresses the very next decision
    if cooldown_active:  # => co-23: a cooldown from the PREVIOUS tick overrides whatever this tick would decide
        return "hold_cooldown"
    return naive_threshold_decision(queue_depth, threshold)  # => no active cooldown -- decide normally


queue_trace = [21, 19, 22, 18, 21]  # => co-23: queue depth oscillating right around threshold=20
# => this is a deliberately adversarial trace -- real traffic rarely bounces this tightly, but it CAN
naive_decisions = [naive_threshold_decision(q, threshold=20) for q in queue_trace]  # => co-23: no memory between ticks
print(naive_decisions)  # => Output: ['scale_out', 'scale_in', 'scale_out', 'scale_in', 'scale_out']

naive_scaling_events = sum(1 for d in naive_decisions if d != "hold")  # => co-23: count of REAL scaling actions
print(naive_scaling_events)  # => Output: 5 -- every single tick triggers a REVERSAL, this is thrashing

hysteresis_decisions: list[str] = []  # => records the decision made at each tick, cooldown-aware
cooldown = False  # => starts clear -- the first tick is never suppressed
for q in queue_trace:  # => processes the SAME queue trace as the naive policy, for a fair comparison
    decision = hysteresis_decision(q, threshold=20, cooldown_active=cooldown)  # => co-23: cooldown-aware decision
    hysteresis_decisions.append(decision)  # => records this tick's outcome before the cooldown updates
    cooldown = decision in ("scale_out", "scale_in")  # => co-23: any REAL action arms the cooldown for next tick
print(hysteresis_decisions)  # => Output: ['scale_out', 'hold_cooldown', 'scale_out', 'hold_cooldown', 'scale_out']

hysteresis_scaling_events = sum(1 for d in hysteresis_decisions if d in ("scale_out", "scale_in"))  # => co-23: real actions only
print(hysteresis_scaling_events)  # => Output: 3

assert naive_scaling_events > hysteresis_scaling_events  # => co-23: hysteresis cuts thrashing roughly in half here
# => co-23: the SAME idea that fixes preemption thrashing (Example 46) also fixes scaling thrashing
# => a cooldown window is a general pattern: any policy re-evaluated too fast can start oscillating
# => this trace also stresses cold-start cost (Example 60) -- every reversal pays that penalty again
print("ex-71 OK")  # => a self-check marker confirming hysteresis measurably reduced the thrashing
