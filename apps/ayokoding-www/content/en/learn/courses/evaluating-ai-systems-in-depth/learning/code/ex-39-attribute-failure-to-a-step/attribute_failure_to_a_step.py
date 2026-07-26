"""Worked Example 39: Locate the Causing Step Inside a Failed Trajectory."""  # => co-20: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-20: ToolCall is a typed record with an explicit correctness flag


class ToolCall(NamedTuple):  # => co-20: one step in a trajectory, PLUS whether that step was itself correct
    tool_name: str  # => co-20: which tool was invoked
    result: str  # => co-20: what the tool returned
    step_is_correct: bool  # => co-20: ground truth -- was THIS step, in isolation, the right thing to do?


FAILED_TRAJECTORY = (  # => co-20: a four-step trajectory that ends in the WRONG final action
    ToolCall("search_ticket", "found ticket #91", step_is_correct=True),  # => co-20: step 1 -- correct
    ToolCall("get_ticket", "status=closed, priority=low", step_is_correct=True),  # => co-20: step 2 -- correct, and it revealed the ticket is CLOSED
    ToolCall("update_priority", "priority updated to high", step_is_correct=False),  # => co-20: step 3 -- WRONG: raised priority on a closed ticket, ignoring step 2's own result
    ToolCall("close_ticket", "already closed, no-op", step_is_correct=True),  # => co-20: step 4 -- a harmless no-op, not itself wrong
)  # => co-20: closes FAILED_TRAJECTORY


def find_first_failing_step(trajectory: tuple[ToolCall, ...]) -> int | None:  # => co-20: attributes failure to ONE specific step index, not "the trajectory" as a whole
    """Return the 0-based index of the FIRST step where `step_is_correct` is False, or None if all steps were correct."""  # => co-20: documents find_first_failing_step's contract -- no runtime output, just sets its __doc__
    for index, step in enumerate(trajectory):  # => co-20: scans steps in order -- the FIRST failure is the root cause, later steps are downstream noise
        if not step.step_is_correct:  # => co-20: found the causing step
            return index  # => co-20: returns this computed value to the caller
    return None  # => co-20: no failing step found -- every step was individually correct


if __name__ == "__main__":  # => co-20: entry point -- runs only when this file executes directly, not on import
    failing_index = find_first_failing_step(FAILED_TRAJECTORY)  # => co-20: attribute the trajectory's failure to one specific step
    print(f"Trajectory has {len(FAILED_TRAJECTORY)} steps")  # => co-20: prints the trajectory length
    for i, step in enumerate(FAILED_TRAJECTORY):  # => co-20: prints every step with its own correctness verdict
        print(f"  Step {i}: {step.tool_name} -> {step.result} (correct: {step.step_is_correct})")  # => co-20
    print(f"First failing step index: {failing_index}")  # => co-20: prints the attributed index

    assert failing_index == 2, "the failure must be attributed to step 2 (update_priority) -- the FIRST step that acted against its own evidence"  # => co-20: the rule this example proves
    assert FAILED_TRAJECTORY[failing_index].tool_name == "update_priority", "the attributed step must be the one that ignored step 2's closed-ticket result"  # => co-20
    print(f"MATCH: the trajectory's overall failure is attributed to step {failing_index} ({FAILED_TRAJECTORY[failing_index].tool_name}), not to the trajectory as an undifferentiated whole")  # => co-20
    # => co-20: attributing failure to ONE step (not the whole run) is what makes ex-40's subagent-vs-orchestrator split possible next
