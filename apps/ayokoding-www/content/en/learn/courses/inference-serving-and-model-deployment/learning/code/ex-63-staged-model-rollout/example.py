"""Example 63: Staged Model Rollout."""

ROLLOUT_STAGES = [5, 25, 100]  # => co-25: percentage of traffic sent to the NEW model version, staged


def next_stage(current_stage_index: int, error_rate: float, error_rate_guardrail: float) -> int | None:
    # => co-25: advance ONLY if the current stage is healthy; `None` means "halt right here"
    if error_rate > error_rate_guardrail:  # => co-25: the guardrail is checked BEFORE any advance decision
        return None  # => `None` is the explicit "stop, do not proceed" signal
    if current_stage_index + 1 < len(ROLLOUT_STAGES):  # => is there a wider stage still ahead?
        return current_stage_index + 1  # => co-25: advance to the NEXT wider stage
    return current_stage_index  # => already at 100% -- rollout complete, nothing further to advance to


healthy_progression: list[int | None] = []  # => records the stage index reached after each check
stage = 0  # => rollout always starts at the narrowest stage
for error_rate in [0.001, 0.002, 0.001]:  # => healthy at every one of the three stages
    stage_result = next_stage(stage, error_rate, error_rate_guardrail=0.01)  # => co-25: one gated check per stage
    healthy_progression.append(stage_result)  # => records whatever this stage's check produced
    stage = stage_result if stage_result is not None else stage  # => only advances the LIVE stage on success
print(healthy_progression)  # => Output: [1, 2, 2]

halted_stage = next_stage(0, error_rate=0.05, error_rate_guardrail=0.01)  # => a 5% error rate at 5% traffic
print(halted_stage)  # => Output: None

assert healthy_progression[-1] == 2  # => co-25: reached the final stage (index 2 => 100% traffic)
assert halted_stage is None  # => co-25: rollout HALTS at 5% traffic rather than propagating a bad model further
# => Example 73 pairs this same guardrail idea with an automatic rollback trigger
print("ex-63 OK")  # => a self-check marker confirming both the healthy progression and the halt held
